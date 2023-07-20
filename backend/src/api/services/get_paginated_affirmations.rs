use crate::models::Affirmation;
use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{bson::Document, Collection};

pub async fn get_paginated_affirmations(
    collection: &Collection<Document>,
    page_number: usize,
    items_per_page: usize,
) -> Result<Vec<Affirmation>, HttpResponse> {
    let skip = (page_number - 1) * items_per_page; // To find the correct documents, we calculate the number of documents to skip

    let cursor = match collection.find(None, None).await {
        // If the cursor is valid, continue
        Ok(cursor) => cursor,
        // If the cursor is invalid, return an error
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(HttpResponse::InternalServerError().body(format!("Error: {}", err)));
        }
    };

    let affirmations = cursor
        .skip(skip)
        .take(items_per_page)
        .filter_map(|result| async {
            match result {
                // If the document is valid, extract the tags array, convert it to a vector of strings
                // and return it together with the text as an Affirmation
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
                // If the document is invalid, return None, and log the error
                Err(err) => {
                    log::error!("Error: {}", err);
                    return None;
                }
            }
        })
        .collect::<Vec<Affirmation>>() // Collect the results into a vector of Affirmation structs
        .await;

    // If the vector is empty, return an error
    if affirmations.is_empty() {
        return Err(HttpResponse::NotFound().body("No affirmations found in the database"));
    }

    return Ok(affirmations);
}
