use crate::api::get_random_affirmation;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::Document, Client};
use std::env;

// This handler function will be called when a GET request is made to the /affirmations/random route.
#[get("/affirmations/random")]
pub async fn get_random(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    // Call the get_random_affirmation service function to retrieve a random affirmation from the database:
    match get_random_affirmation(&collection).await {
        Ok(affirmations) => HttpResponse::Ok().json(affirmations),
        Err(err) => err, // Propagate the error directly as a return value
    }
}
