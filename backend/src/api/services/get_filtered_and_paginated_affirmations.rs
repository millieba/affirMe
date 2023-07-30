use crate::models::Affirmation;
use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

// This struct is used for the pagination service functions.
// It returns both the affirmations and the total number of documents in the collection which match the query.
#[derive(Debug, serde::Serialize)]
pub struct PaginatedAffirmations {
    pub affirmations: Vec<Affirmation>,
    pub total_documents: usize,
}

pub async fn get_filtered_and_paginated_affirmations(
    collection: &Collection<Document>,
    search: String,
    tags: String,
    page_number: usize,
    items_per_page: usize,
) -> Result<PaginatedAffirmations, HttpResponse> {
    let mut filter = doc! {
        "text": { // Create a regex filter for the "text" field
            "$regex": search.split_whitespace() // Split the search string into individual words
                // For each word, create a case insensitive, "(?i)", regex pattern that matches the word anywhere in the "text" field
                // Escape any special characters to prevent unexpected regex behaviour
                .map(|word| format!(r"(?i).*{}.*", regex::escape(word)))
                .collect::<Vec<String>>() // Collect the regex patterns into a vector of strings to be joined together
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

    // Count the total number of documents that match the filter
    // This is important for the frontend, so it knows how many pages can be displayed
    let total_documents = match collection.count_documents(filter.clone(), None).await {
        Ok(count) => count,
        Err(err) => {
            log::error!("Error counting documents: {}", err);
            return Err(HttpResponse::InternalServerError()
                .body(format!("Error counting documents: {}", err)));
        }
    };

    let skip = (page_number - 1) * items_per_page; // To find the correct documents, we calculate the number of documents to skip

    let cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
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
                Err(err) => {
                    log::error!("Error: {}", err);
                    return None;
                }
            }
        })
        .collect::<Vec<Affirmation>>() // Collect the results into a vector of Affirmation structs
        .await;

    if affirmations.is_empty() {
        return Err(HttpResponse::NotFound().body("No affirmations found in the database"));
    }

    let affirmation_results = PaginatedAffirmations {
        affirmations,
        total_documents: total_documents as usize,
    };

    Ok(affirmation_results)
}
