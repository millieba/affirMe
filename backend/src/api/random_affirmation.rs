use crate::api::get_random_affirmation;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::Document, Client};
use std::env;

#[get("/affirmations/random")]
pub async fn random_affirmation(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    match get_random_affirmation(&collection).await {
        Ok(affirmations) => return HttpResponse::Ok().json(affirmations),
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}
