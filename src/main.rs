use dotenv::dotenv;
use macrometa_sdk::api::collections::CollectionsClient;
use macrometa_sdk::api::document::DocumentClient;
use macrometa_sdk::api::key_value::KeyValueClient;
use macrometa_sdk::configuration::{ApiKey, Configuration};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::io::{self, Write};

mod functions;
use functions::{
    add_data_to_collection, add_document_to_collection, create_document_collection,
    create_key_value_collection, get_data_from_collection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load environment variables
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let fabric = env::var("FABRIC").expect("FABRIC must be set");

    // Initialize the clients
    let key_value_client = KeyValueClient::with_configuration(Configuration::with_api_key(
        base_url.to_string(),
        ApiKey::new(api_key.to_string()),
        fabric.to_string(),
    ));
    let collections_client = CollectionsClient::with_configuration(Configuration::with_api_key(
        base_url.to_string(),
        ApiKey::new(api_key.to_string()),
        fabric.to_string(),
    ));
    let document_client = DocumentClient::with_configuration(Configuration::with_api_key(
        base_url.to_string(),
        ApiKey::new(api_key.to_string()),
        fabric.to_string(),
    ));

    let mut input = String::new();

    loop {
        println!("\nMenu:");
        println!("1. Create Key-Value Collection");
        println!("2. Add Data to Key-Value Collection");
        println!("3. Retrieve Data from Key-Value Collection");
        println!("4. Create Document Collection");
        println!("5. Add Data to Document Collection");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter the collection name:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let collection_name = input.trim().to_string();
                input.clear();

                // Example configuration for the collection
                let collection_config = r#"
                {
                    "stream": true,
                    "enableShards": false,
                    "waitForSync": true,
                    "shardKeys": [
                        "key1"
                    ],
                    "blobs" : false,
                    "expiration" : false
                }
                "#;

                create_key_value_collection(&key_value_client, &collection_name, collection_config)
                    .await?;
            }
            "2" => {
                println!("Enter the collection name:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let collection_name = input.trim().to_string();
                input.clear();

                let mut data = Vec::new();

                loop {
                    println!("Enter key (or 'done' to finish):");
                    input.clear();
                    io::stdin().read_line(&mut input)?;
                    let key = input.trim().to_string();
                    if key == "done" {
                        break;
                    }

                    println!("Enter value:");
                    input.clear();
                    io::stdin().read_line(&mut input)?;
                    let value = input.trim().to_string();

                    println!("Enter expiration time (-1 for no expiration):");
                    input.clear();
                    io::stdin().read_line(&mut input)?;
                    let expire_at: i64 = input.trim().parse().unwrap_or(-1);

                    let record = json!({
                        "_key": key,
                        "value": value,
                        "expireAt": expire_at
                    });

                    data.push(record);
                }

                let data = serde_json::Value::Array(data);
                add_data_to_collection(&key_value_client, &collection_name, data).await?;
            }
            "3" => {
                println!("Enter the collection name:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let collection_name = input.trim().to_string();
                input.clear();

                println!("Enter the key:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let key = input.trim().to_string();
                input.clear();

                let value =
                    get_data_from_collection(&key_value_client, &collection_name, &key).await?;
                println!("Retrieved value: {:?}", value);
            }
            "4" => {
                println!("Enter the collection name:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let collection_name = input.trim().to_string();
                input.clear();

                create_document_collection(&collections_client, &collection_name).await?;
            }
            "5" => {
                println!("Enter the collection name:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let collection_name = input.trim().to_string();
                input.clear();

                println!("Enter the JSON document to add:");
                input.clear();
                io::stdin().read_line(&mut input)?;
                let document: serde_json::Value = serde_json::from_str(input.trim())?;

                add_document_to_collection(&document_client, &collection_name, document).await?;
            }
            "6" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }

    Ok(())
}
