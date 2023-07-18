use crate::models::Affirmation; // Import the Affirmation struct from the models module
use actix_web::{get, web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{bson::Document, Client};
use std::env;

#[get("/affirmations")]
pub async fn all_affirmations(client: web::Data<Client>) -> impl Responder {
    println!("Crate root: {:?}", env!("CARGO_MANIFEST_DIR")); // Print the crate root
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    let affirmations = match collection.find(None, None).await {
        Ok(cursor) => {
            cursor
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
                            eprintln!("Error: {}", err);
                            return None;
                        }
                    }
                })
                .collect::<Vec<Affirmation>>()
                .await
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return HttpResponse::InternalServerError().body(format!("Error: {}", err));
        }
    };

    if affirmations.is_empty() {
        return HttpResponse::InternalServerError().body("No affirmations found in the database");
    }

    HttpResponse::Ok().json(affirmations)
}
