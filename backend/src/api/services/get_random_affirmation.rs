use crate::models::Affirmation;
use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{bson::Document, Collection};
use rand::seq::SliceRandom;

pub async fn get_random_affirmation(
    collection: &Collection<Document>,
) -> Result<Affirmation, HttpResponse> {
    // affirmations is a vector of Affirmation structs, which we will populate with the documents from the database
    let mut affirmations = match collection.find(None, None).await {
        // Handle successful find operation. The find operation has no filter, so it returns all documents in the collection
        // in a cursor, so we can iterate over it to access individual documents
        Ok(cursor) => {
            cursor
                .filter_map(|result| async {
                    match result {
                        // Handle successful document retrieval:
                        Ok(document) => {
                            // Extract the tags array from the document and convert it to a vector of strings
                            let tags = document
                                .get_array("tags") // Get the "tags" field as an array
                                .ok()? // Return None if "tags" field doesn't exist or is not an array
                                .iter() // Iterate over the array elements
                                .filter_map(|tag| tag.as_str().map(String::from)) // Filter out non-string elements and convert each element to String
                                .collect::<Vec<String>>(); // Collect the filtered elements into a vector of strings

                            let affirmation = Affirmation {
                                // "to_owned()" converts borrowed references (`&str`) to owned `String` values,
                                // ensuring that the extracted values are independent and not tied to the lifetime of the original document.
                                text: document.get_str("text").ok()?.to_owned(),
                                tags,
                            };
                            return Some(affirmation);
                        }
                        // Handle failed document retrieval:
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return None; // return None if an error occurs
                        }
                    }
                })
                .collect::<Vec<_>>()
                .await
        }
        // Handle failed find operation:
        Err(err) => {
            // TODO: Fix this error handling, looks goofy
            log::error!("Error: {}", err);
            return Err(HttpResponse::InternalServerError().body(format!("Error: {}", err)));
        }
    };

    if affirmations.is_empty() {
        return Err(HttpResponse::NotFound().body("No affirmations found in the database"));
    }

    // Select a random element from the vector using the rand::thread_rng() random number generator
    // unwrap() is safe to use here because we know that the vector is not empty (we checked for that above)
    let random_affirmation = affirmations
        .choose_mut(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
    // Return the randomly selected affirmation
    return Ok(random_affirmation);
}
