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
    #[serde(rename = "Affirmation")]
    text: String,
    #[serde(rename = "Tag")]
    tag: String,
}

#[get("/affirmations/random")]
async fn random_affirmation(client: web::Data<Client>) -> impl Responder {
    let collection = client
        .database("affirMe")
        .collection::<Document>("affirmations");
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut affirmations = Vec::<Affirmation>::new();
    while let Some(Ok(document)) = cursor.next().await {
        let affirmation = Affirmation {
            text: document.get_str("Affirmation").unwrap().to_owned(),
            tag: document.get_str("Tag").unwrap().to_owned(),
        };
        affirmations.push(affirmation);
    }
    let random_affirmation = affirmations.choose(&mut rand::thread_rng()).unwrap();
    HttpResponse::Ok().json(random_affirmation)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(random_affirmation)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
