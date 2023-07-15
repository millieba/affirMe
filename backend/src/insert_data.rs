use mongodb::{bson::doc, options::ClientOptions, Client}; // Import required dependencies from the `mongodb` crate
use serde::Serialize; // Import `Serialize` trait from the `serde` crate
use std::error::Error; // Import `Error` trait from the `std` module

#[derive(Debug, Serialize)] // Implement the `Serialize` trait for the `Affirmation` struct
struct Affirmation {
    text: String,
    tags: Vec<String>,
}

#[tokio::main] // Use the `tokio` runtime for asynchronous execution
async fn main() -> Result<(), Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?; // Parse client options for connecting to the MongoDB server

    let client = Client::with_options(client_options)?; // Create a new MongoDB client with the specified options

    let db = client.database("affirMe"); // Access the "affirMe" database

    let collection = db.collection::<mongodb::bson::Document>("affirmations"); // Access the "affirmations" collection within the database

    collection.delete_many(doc! {}, None).await?; // Delete any existing documents in the "affirmations" collection

    let file = std::fs::File::open("affirmations.csv")?; // Open the "affirmations.csv" file
    let mut reader = csv::Reader::from_reader(file); // Create a CSV reader from the file

    for result in reader.records() {
        let record = result?; // Unwrap the CSV record
        let affirmation = Affirmation {
            text: record.get(0).unwrap().to_owned(), // Extract the text field from the CSV record
            tags: record
                .get(1)
                .unwrap()
                .split(',')
                .map(|tag| tag.trim().to_owned()) // Extract tags from the CSV record by splitting on commas and trimming whitespace
                .collect(), // Collect the extracted tags into a vector
        };

        let document = mongodb::bson::to_document(&affirmation)?; // Convert the `Affirmation` struct to a BSON document

        collection.insert_one(document, None).await?; // Insert the document into the "affirmations" collection
    }

    println!("Data insertion completed successfully!"); // Print a success message

    return Ok(()); // Return `Ok` to indicate that the program completed successfully
}
