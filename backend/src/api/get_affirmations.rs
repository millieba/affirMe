use crate::api::{get_all_affirmations, get_filtered_affirmations, get_paginated_affirmations};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::Document, Client};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct QueryParams {
    page_number: Option<usize>,
    items_per_page: Option<usize>,
    search: Option<String>,
    tags: Option<String>,
}

// This handler function will be called when a GET request is made to the /affirmations route.
// It uses match expressions to determine which service function to call based on the query parameters provided.
#[get("/affirmations")]
pub async fn get_affirmations(
    client: web::Data<Client>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    match (
        query.page_number,
        query.items_per_page,
        query.search.clone(),
        query.tags.clone(),
    ) {
        // If only page number and items per page are provided, call get_paginated_affirmations handler
        (Some(page), Some(items), None, None) => {
            match get_paginated_affirmations(&collection, page, items).await {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If both search term and tags are provided, call get_filtered_affirmations handler with both parameters
        (None, None, Some(term), Some(tags)) => {
            match get_filtered_affirmations(&collection, term, tags).await {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If only search term is provided, call get_filtered_affirmations handler with search term and empty tags
        (None, None, Some(term), None) => {
            match get_filtered_affirmations(&collection, term, String::new()).await {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If no query parameters are provided, call get_all_affirmations handler
        (None, None, None, None) => match get_all_affirmations(&collection).await {
            Ok(affirmations) => HttpResponse::Ok().json(affirmations),
            Err(err) => err, // Propagate the error directly as a return value
        },
        // Handle invalid request
        _ => return HttpResponse::BadRequest().body("Invalid request"),
    }
}
