mod models;
mod db;
mod frameworks;
mod error;

use std::env;
use dotenv::dotenv;
use mongodb::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = client.database("sample_restaurants");
    
    println!("Connected to MongoDB!");
    println!("Available web frameworks:");
    println!("1. None (MongoDB driver only)");
    println!("2. Actix Web");
    println!("3. Axum");
    println!("4. Rocket");
    println!("5. Warp");
    println!("6. Tide");
    
    println!("\nEnter your choice (1-6):");
    
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    
    match choice.trim().parse::<u8>()? {
        1 => frameworks::none::start(db).await?,
        2 => frameworks::actix::start(db).await?,
        3 => frameworks::axum::start(db).await?,
        4 => frameworks::rocket::start(db).await?,
        5 => frameworks::warp::start(db).await?,
        6 => frameworks::tide::start(db).await?,
        _ => println!("Invalid choice!")
    }

    Ok(())
}
