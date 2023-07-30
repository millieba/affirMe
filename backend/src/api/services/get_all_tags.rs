use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{bson::Document, Collection};
use std::collections::HashSet;

pub async fn get_all_tags(collection: &Collection<Document>) -> Result<Vec<String>, HttpResponse> {
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(HttpResponse::InternalServerError().body(format!("Error: {}", err)));
        }
    };

    // Use a HashSet to only store distinct tags (no duplicates):
    let mut tag_set: HashSet<String> = HashSet::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(tags) = document.get_array("tags").ok() {
                    // Iterate over the tags array and add each tag to the HashSet
                    for tag in tags {
                        if let Some(tag_str) = tag.as_str() {
                            tag_set.insert(tag_str.to_owned());
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Error: {}", err);
                return Err(HttpResponse::InternalServerError().body(format!("Error: {}", err)));
            }
        }
    }

    // Convert the HashSet back to a Vec<String> and return it if it's not empty:
    let mut tags: Vec<String> = tag_set.into_iter().collect();
    tags.sort(); // Sort the tags alphabetically

    if tags.is_empty() {
        return Err(HttpResponse::NotFound().body("No tags found in the database"));
    }

    Ok(tags)
}
