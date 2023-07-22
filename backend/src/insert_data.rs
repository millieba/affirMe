use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use std::error::Error;
// Import the Affirmation struct from models.rs
mod models;
use crate::models::Affirmation;

#[tokio::main] // Use the "tokio" runtime for asynchronous execution
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok(); // Load environment variables from .env file

    let uri = env::var("DATABASE_URI").expect("DATABASE_URI not set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");
    let collection_name = env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set");

    let client_options = ClientOptions::parse(&uri).await?; // Parse client options for connecting to the MongoDB server
    let client = Client::with_options(client_options)?; // Create a new MongoDB client with the specified options

    let db = client.database(&db_name); // Access the specified database
    let collection = db.collection::<mongodb::bson::Document>(&collection_name); // Access the specified collection within the database

    collection.delete_many(doc! {}, None).await?; // Delete any existing documents in the collection

    let file = std::fs::File::open("affirmations.csv")?; // Open the "affirmations.csv" file
    let mut reader = csv::Reader::from_reader(file); // Create a CSV reader from the file

    for result in reader.records() {
        let record = result?; // Unwrap the CSV record
        let affirmation = Affirmation {
            text: record.get(0).unwrap().to_owned(), // Extract the text field from the CSV record
            tags: record
                .get(1)
                .unwrap()
                .split(',') // Extract tags from the CSV record by splitting on commas and trimming whitespace
                .map(|tag| tag.trim().to_owned())
                .collect(), // Collect the extracted tags into a vector
        };

        let document = mongodb::bson::to_document(&affirmation)?; // Convert the "Affirmation" struct to a BSON document

        collection.insert_one(document, None).await?; // Insert the document into the collection
    }

    println!("😌🙌 Mission accomplished! Your affirmations have journeyed through the stars and landed safely in the database with cosmic precision!"); // Print a success message

    Ok(()) // Return "Ok" to indicate that the program completed successfully
}
