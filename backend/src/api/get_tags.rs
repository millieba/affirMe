use crate::api::get_all_tags;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::Document, Client};
use std::env;

// This handler function will be called when a GET request is made to the affirmations/tags route.
#[get("affirmations/tags")]
pub async fn get_tags(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    // Call the get_all_tags service function to retrieve all distinct tags on affirmations from the database:
    match get_all_tags(&collection).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(err) => err, // Propagate the error directly as a return value
    }
}
