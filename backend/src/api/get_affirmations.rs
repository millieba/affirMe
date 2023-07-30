use crate::api::{get_filtered_affirmations, get_filtered_and_paginated_affirmations};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use mongodb::{bson::Document, Client};
use serde::Deserialize;
use std::{collections::HashSet, env};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    page_number: Option<usize>,
    items_per_page: Option<usize>,
    search: Option<String>,
    tags: Option<String>,
}

// A function to parse the raw query string and return a HashSet of parameter names
fn parse_query_string(raw_query: &str) -> HashSet<String> {
    url::form_urlencoded::parse(raw_query.as_bytes())
        .map(|(key, _)| key.to_string())
        .collect()
}

// This handler function will be called when a GET request is made to the /affirmations route.
// It uses match expressions to determine which service function to call based on the query parameters provided.
// Rust doesn't support function overloading, so this is the best way to achieve the same result.
#[get("/affirmations")]
pub async fn get_affirmations(
    client: web::Data<Client>,
    req: HttpRequest,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let collection = client
        .database(&env::var("DATABASE_NAME").expect("DATABASE_NAME not set"))
        .collection::<Document>(
            &env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set"),
        );

    // Parse the raw query string and get a HashSet of parameter names
    let parsed_params = parse_query_string(req.query_string());

    // Make a HashSet of valid query parameters
    let valid_params: HashSet<String> = ["page_number", "items_per_page", "search", "tags"]
        .iter()
        .map(|&param| param.to_string())
        .collect();

    // Check if any parameters in the parsed query string are not valid
    if !parsed_params.is_subset(&valid_params) {
        return HttpResponse::BadRequest().body("Invalid query parameters");
    }

    match (
        query.page_number,
        query.items_per_page,
        query.search.clone(),
        query.tags.clone(),
    ) {
        // If all query parameters are provided, call get_filtered_and_paginated_affirmations handler with all parameters
        (Some(page), Some(items), Some(term), Some(tags)) => {
            match get_filtered_and_paginated_affirmations(&collection, term, tags, page, items)
                .await
            {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If tags are missing, call get_filtered_and_paginated_affirmations handler with empty tags
        (Some(page), Some(items), Some(term), None) => {
            match get_filtered_and_paginated_affirmations(
                &collection,
                term,
                String::new(),
                page,
                items,
            )
            .await
            {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If search term is missing, call get_filtered_and_paginated_affirmations handler with empty search term
        (Some(page), Some(items), None, Some(tags)) => {
            match get_filtered_and_paginated_affirmations(
                &collection,
                String::new(),
                tags,
                page,
                items,
            )
            .await
            {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If search term and tags are missing, call get_filtered_and_paginated_affirmations handler with empty search term and tags
        (Some(page), Some(items), None, None) => {
            match get_filtered_and_paginated_affirmations(
                &collection,
                String::new(), // empty search term
                String::new(), // empty tags
                page,
                items,
            )
            .await
            {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If only search term and tags are provided, call get_filtered_affirmations handler with search term and tags
        // get_filtered_affirmations will return all affirmations that match the search term and tags (no pagination)
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
        // If only tags are provided, call get_filtered_affirmations handler with tags and empty search term
        (None, None, None, Some(tags)) => {
            match get_filtered_affirmations(&collection, String::new(), tags).await {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // If no query parameters are provided, call get_filtered_affirmations handler with empty search term and tags
        (None, None, None, None) => {
            match get_filtered_affirmations(&collection, String::new(), String::new()).await {
                Ok(affirmations) => HttpResponse::Ok().json(affirmations),
                Err(err) => err, // Propagate the error directly as a return value
            }
        }
        // Handle invalid request
        _ => return HttpResponse::BadRequest().body("Invalid request"),
    }
}
