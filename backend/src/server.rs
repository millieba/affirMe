use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use std::env;

mod api; // Import the "api" module
mod models; // Import the "models" module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // Load environment variables from .env file
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_uri = env::var("DATABASE_URI").expect("DATABASE_URI not set");

    let client_options = ClientOptions::parse(&database_uri)
        .await
        .expect("Failed to parse MongoDB client options");

    let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

    println!("🎉 Connected to the database successfully");

    // Create a new instance of the Actix Web HttpServer:
    let server = HttpServer::new(move || {
        App::new() // Create a new instance of the Actix Web App
            .wrap(
                // Add the CORS middleware to the app
                Cors::default().allowed_origin("http://127.0.0.1:5173"), // Allow requests from http://localhost:5173 (our frontend)
            )
            // Store a reference to the MongoDB client in the App data for sharing across handlers.
            // This is to reuse the database connection for several requests, rather than creating a new connection for each request:
            .app_data(web::Data::new(client.clone()))
            // Register the random_affirmation handler function as a service to handle requests to the /affirmations/random route:
            .service(api::get_affirmations)
            .service(api::get_random)
            .service(api::get_tags)
    })
    .bind(("127.0.0.1", 8080))?; // Bind the server to the specified IP address and port

    println!("🚀 Server ready at: http://localhost:8080/affirmations/random");
    server.run().await?;
    // We need to return an Ok(()) value here because the main() function is expected to return a Result
    return Ok(());
}
