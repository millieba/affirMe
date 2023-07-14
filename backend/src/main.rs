use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};
use rand::seq::SliceRandom;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Affirmation {
    text: String,
    tag: String,
}

#[get("/affirmations/random")]
async fn random_affirmation(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database("affirMe")
        .collection::<Document>("affirmations");

    let affirmations = match collection.find(None, None).await {
        Ok(cursor) => {
            cursor
                .filter_map(|result| async {
                    match result {
                        Ok(document) => Some(Affirmation {
                            text: document.get_str("Affirmation").ok()?.to_owned(),
                            tag: document.get_str("Tag").ok()?.to_owned(),
                        }),
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            None
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

    let random_affirmation = affirmations.choose(&mut rand::thread_rng()).unwrap();
    HttpResponse::Ok().json(random_affirmation)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Failed to parse MongoDB client options");

    let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(random_affirmation)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
