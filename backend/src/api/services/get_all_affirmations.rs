use crate::models::Affirmation;
use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{bson::Document, Collection};

pub async fn get_all_affirmations(
    collection: &Collection<Document>,
) -> Result<Vec<Affirmation>, HttpResponse> {
    let cursor = match collection.find(None, None).await {
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
        return Err(
            HttpResponse::InternalServerError().body("No affirmations found in the database")
        );
    }

    return Ok(affirmations);
}
