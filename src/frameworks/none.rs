use mongodb::Database;
use bson::{oid::ObjectId, Document};
use serde_json::Value;
use std::io::{self, Write};

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
};

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    let repo = MongoRepo::new(&db);
    
    loop {
        println!("\nAvailable operations:");
        println!("1. Create restaurant");
        println!("2. List first 10 restaurants");
        println!("3. Get restaurant by ID");
        println!("4. Update restaurant");
        println!("5. Delete restaurant");
        println!("6. Exit");
        
        print!("Enter your choice (1-6): ");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim().parse::<u8>()? {
            1 => create_restaurant(&repo).await?,
            2 => list_restaurants(&repo).await?,
            3 => get_restaurant_by_id(&repo).await?,
            4 => update_restaurant(&repo).await?,
            5 => delete_restaurant(&repo).await?,
            6 => break,
            _ => println!("Invalid choice!"),
        }
    }
    
    Ok(())
}

async fn create_restaurant(repo: &MongoRepo) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter restaurant JSON data:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let restaurant: Restaurant = serde_json::from_str(&input)?;
    match repo.create_restaurant(restaurant).await {
        Ok(created) => println!("Created restaurant: {:?}", created),
        Err(e) => println!("Error creating restaurant: {}", e),
    }
    
    Ok(())
}

async fn list_restaurants(repo: &MongoRepo) -> Result<(), Box<dyn std::error::Error>> {
    match repo.get_restaurants(10).await {
        Ok(restaurants) => {
            for restaurant in restaurants {
                println!("{:?}", restaurant);
            }
        }
        Err(e) => println!("Error fetching restaurants: {}", e),
    }
    
    Ok(())
}

async fn get_restaurant_by_id(repo: &MongoRepo) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter restaurant ID: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let id = ObjectId::parse_str(input.trim())?;
    match repo.get_restaurant_by_id(id).await {
        Ok(restaurant) => println!("{:?}", restaurant),
        Err(e) => println!("Error fetching restaurant: {}", e),
    }
    
    Ok(())
}

async fn update_restaurant(repo: &MongoRepo) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter restaurant ID: ");
    io::stdout().flush()?;
    
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input)?;
    
    println!("Enter update JSON data:");
    let mut update_input = String::new();
    io::stdin().read_line(&mut update_input)?;
    
    let id = ObjectId::parse_str(id_input.trim())?;
    let update: Value = serde_json::from_str(&update_input)?;
    let update_doc: Document = bson::to_document(&update)?;
    
    match repo.update_restaurant(id, update_doc).await {
        Ok(updated) => println!("Updated restaurant: {:?}", updated),
        Err(e) => println!("Error updating restaurant: {}", e),
    }
    
    Ok(())
}

async fn delete_restaurant(repo: &MongoRepo) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter restaurant ID: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let id = ObjectId::parse_str(input.trim())?;
    match repo.delete_restaurant(id).await {
        Ok(_) => println!("Restaurant deleted successfully"),
        Err(e) => println!("Error deleting restaurant: {}", e),
    }
    
    Ok(())
}