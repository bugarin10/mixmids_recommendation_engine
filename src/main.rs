use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue};
use std::collections::HashMap;
use actix_files as fs;

async fn save_user_data(user_data: web::Form<UserData>) -> impl Responder {
    // Create DynamoDB client
    let client = DynamoDbClient::new(Region::UsEast1);

    // Prepare item for insertion into DynamoDB table
    let mut item = HashMap::new();
    item.insert("Username".to_string(), AttributeValue { s: Some(user_data.username.clone()), ..Default::default() });
    item.insert("Password".to_string(), AttributeValue { s: Some(user_data.password.clone()), ..Default::default() });
    item.insert("UserType".to_string(), AttributeValue { s: Some(user_data.user_type.clone()), ..Default::default() });

    // Prepare input for PutItem operation
    let input = PutItemInput {
        table_name: "Mixology_Butler_Login".to_string(), // Replace with your DynamoDB table name
        item,
        ..Default::default()
    };
    
    // Call PutItem operation to save user data to DynamoDB
    match client.put_item(input).await {
        Ok(_) => HttpResponse::Ok().body("User data saved successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving user data: {}", e)),
    }
}

#[derive(serde::Deserialize)]
struct UserData {
    username: String,
    password: String,
    user_type: String, // e.g., "Customer" or "BarOwner"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "static").index_file("user_interface.html"))
            .route("/submit", web::post().to(save_user_data)) // Endpoint for form submission
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

