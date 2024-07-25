use macrometa_sdk::api::collections::CollectionsClient;
use macrometa_sdk::api::document::DocumentClient;
use macrometa_sdk::api::key_value::KeyValueClient;
use macrometa_sdk::api::SdkResponse;
use macrometa_sdk::models::collections::{CollectionType, CreateCollectionBody};
use macrometa_sdk::models::key_value::CreateKeyValueCollectionBody;
use macrometa_sdk::models::key_value::GetKeyValuePairResponse;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub async fn create_key_value_collection(
    client: &KeyValueClient,
    collection_name: &str,
    collection_config: &str,
) -> Result<(), Box<dyn Error>> {
    let post_request_body: CreateKeyValueCollectionBody = serde_json::from_str(collection_config)?;
    client
        .create_key_value_collection(&collection_name.to_string(), &post_request_body, &None)
        .await?;
    println!("Collection '{}' created successfully", collection_name);

    Ok(())
}

pub async fn add_data_to_collection(
    client: &KeyValueClient,
    collection_name: &str,
    data: Value,
) -> Result<(), Box<dyn Error>> {
    client
        .set_key_value_pairs(&collection_name.to_string(), &data)
        .await?;
    println!("Data added to collection '{}'", collection_name);

    Ok(())
}

pub async fn get_data_from_collection(
    client: &KeyValueClient,
    collection_name: &str,
    key: &str,
) -> Result<Value, Box<dyn Error>> {
    let response: SdkResponse<GetKeyValuePairResponse<Value>> = client
        .get_key_value_pair(&collection_name.to_string(), &key.to_string())
        .await?;
    let value = serde_json::to_value(response.body.value)?;
    println!(
        "Data retrieved from collection '{}': {:?}",
        collection_name, value
    );

    Ok(value)
}

pub async fn create_document_collection(
    client: &CollectionsClient,
    collection_name: &str,
) -> Result<(), Box<dyn Error>> {
    let request_body = CreateCollectionBody {
        name: collection_name.to_string(),
        wait_for_sync: true,
        enable_shards: false,
        key_options: None,
        is_local: false,
        stream: true,
        collection_type: CollectionType::Document,
    };

    client.create_collection(&request_body).await?;
    println!(
        "Document collection '{}' created successfully",
        collection_name
    );

    Ok(())
}

pub async fn add_document_to_collection(
    client: &DocumentClient,
    collection_name: &str,
    document: Value,
) -> Result<(), Box<dyn Error>> {
    client
        .insert(&collection_name.to_string(), &document)
        .await?;
    println!("Document added to collection '{}'", collection_name);

    Ok(())
}
