use crate::models::Affirmation;
use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

pub async fn get_filtered_affirmations(
    collection: &Collection<Document>,
    search: String,
    tags: String,
) -> Result<Vec<Affirmation>, HttpResponse> {
    let mut filter = doc! {
        "text": { // Create a regex filter for the "text" field
            "$regex": search.split_whitespace() // Split the search string into individual words
                // For each word, create a case insensitive, "(?i)", regex pattern that matches the word anywhere in the "text" field
                // Escape any special characters to prevent unexpected regex behaviour
                .map(|word| format!(r"(?i).*{}.*", regex::escape(word)))
                .collect::<Vec<String>>() // Collect the regex pattterns into a vector of strings to be joined together
                .join(""), // Join the regex patterns together to create the final regex pattern for the search term
        },
    };

    // If the tags parameter is not empty, create a filter for the "tags" field too
    if !tags.is_empty() {
        // Split the tags into individual tags
        let tag_list: Vec<String> = tags.split(',').map(|tag| tag.trim().to_owned()).collect();

        // Combine the search term and tags filters, so that the affirmations must match both filters
        filter = doc! {
            "$and": [
                filter,
                doc! {
                    "tags": {
                        "$in": tag_list
                    }
                },
            ]
        };
    }

    let cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(HttpResponse::InternalServerError().body(format!("Error: {}", err)));
        }
    };

    let affirmations = cursor
        .filter_map(|result| async {
            match result {
                Ok(document) => {
                    let tags = document
                        .get_array("tags")
                        .ok()?
                        .iter()
                        .filter_map(|tag| tag.as_str().map(String::from))
                        .collect::<Vec<String>>();

                    return Some(Affirmation {
                        text: document.get_str("text").ok()?.to_owned(),
                        tags,
                    });
                }
                Err(err) => {
                    log::error!("Error: {}", err);
                    return None;
                }
            }
        })
        .collect::<Vec<Affirmation>>()
        .await;

    if affirmations.is_empty() {
        return Err(HttpResponse::NotFound().body("No affirmations found in the database"));
    }

    Ok(affirmations)
}
