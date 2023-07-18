use crate::models::Affirmation; // Import the Affirmation struct from the models module
use actix_web::{get, web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{bson::Document, Client};
use rand::seq::SliceRandom;
use std::env;

#[get("/affirmations/random")]
pub async fn random_affirmation(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    // affirmations is a vector of Affirmation structs, which we will populate with the documents from the database
    let affirmations = match collection.find(None, None).await {
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

                            return Some(Affirmation {
                                // "to_owned()" converts borrowed references (`&str`) to owned `String` values,
                                // ensuring that the extracted values are independent and not tied to the lifetime of the original document.
                                text: document.get_str("text").ok()?.to_owned(),
                                tags,
                            });
                        }
                        // Handle failed document retrieval:
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return None; // return None if an error occurs
                        }
                    }
                })
                // Collect the stream of documents into a vector of Affirmation structs:
                .collect::<Vec<Affirmation>>()
                .await
        }
        // Handle failed find operation:
        Err(err) => {
            eprintln!("Error: {}", err);
            return HttpResponse::InternalServerError().body(format!("Error: {}", err));
        }
    };

    // Handle empty collection:
    if affirmations.is_empty() {
        return HttpResponse::InternalServerError().body("No affirmations found in the database");
    }

    // Select a random element from the vector using the rand::thread_rng() random number generator
    // unwrap() is safe to use here because we know that the vector is not empty (we checked for that above)
    let random_affirmation = affirmations.choose(&mut rand::thread_rng()).unwrap();
    // Return the randomly selected affirmation as JSON
    HttpResponse::Ok().json(random_affirmation)
}
