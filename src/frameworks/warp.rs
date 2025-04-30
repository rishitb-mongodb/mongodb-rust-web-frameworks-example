use warp::{
    self,
    Filter,
    Reply,
    Rejection,
    reply::{json, with_status},
    http::StatusCode,
};
use mongodb::Database;
use bson::oid::ObjectId;
use serde_json::Value;
use std::sync::Arc;

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
    error::AppError,
};

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Arc::new(MongoRepo::new(&db));
    
    let repo_filter = warp::any().map(move || repo.clone());
    
    // CRUD Routes
    let create_restaurant = warp::post()
        .and(warp::path("api"))
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(repo_filter.clone())
        .and(warp::body::json())
        .and_then(create_restaurant_handler);

    let list_restaurants = warp::get()
        .and(warp::path("api"))
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(repo_filter.clone())
        .and_then(list_restaurants_handler);

    let get_restaurant = warp::get()
        .and(warp::path("api"))
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(repo_filter.clone())
        .and_then(get_restaurant_handler);

    let update_restaurant = warp::put()
        .and(warp::path("api"))
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(repo_filter.clone())
        .and(warp::body::json())
        .and_then(update_restaurant_handler);

    let delete_restaurant = warp::delete()
        .and(warp::path("api"))
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(repo_filter.clone())
        .and_then(delete_restaurant_handler);

    let routes = create_restaurant
        .or(list_restaurants)
        .or(get_restaurant)
        .or(update_restaurant)
        .or(delete_restaurant);

    println!("Starting Warp server at http://127.0.0.1:8083");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8083))
        .await;
    
    Ok(())
}

async fn create_restaurant_handler(
    repo: Arc<MongoRepo>,
    restaurant: Restaurant,
) -> Result<impl Reply, Rejection> {
    match repo.create_restaurant(restaurant).await {
        Ok(created) => Ok(with_status(json(&created), StatusCode::CREATED)),
        Err(e) => Ok(with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn list_restaurants_handler(repo: Arc<MongoRepo>) -> Result<impl Reply, Rejection> {
    match repo.get_restaurants(10).await {
        Ok(restaurants) => Ok(with_status(json(&restaurants), StatusCode::OK)),
        Err(e) => Ok(with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_restaurant_handler(
    id: String,
    repo: Arc<MongoRepo>,
) -> Result<impl Reply, Rejection> {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(with_status(json(&"Invalid ID format"), StatusCode::BAD_REQUEST)),
    };

    match repo.get_restaurant_by_id(object_id).await {
        Ok(restaurant) => Ok(with_status(json(&restaurant), StatusCode::OK)),
        Err(AppError::NotFound) => Ok(with_status(json(&"Not found"), StatusCode::NOT_FOUND)),
        Err(e) => Ok(with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn update_restaurant_handler(
    id: String,
    repo: Arc<MongoRepo>,
    update: Value,
) -> Result<impl Reply, Rejection> {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(with_status(json(&"Invalid ID format"), StatusCode::BAD_REQUEST)),
    };

    let update_doc = match bson::to_document(&update) {
        Ok(doc) => doc,
        Err(_) => return Ok(with_status(json(&"Invalid update document"), StatusCode::BAD_REQUEST)),
    };

    match repo.update_restaurant(object_id, update_doc).await {
        Ok(updated) => Ok(with_status(json(&updated), StatusCode::OK)),
        Err(AppError::NotFound) => Ok(with_status(json(&"Not found"), StatusCode::NOT_FOUND)),
        Err(e) => Ok(with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn delete_restaurant_handler(
    id: String,
    repo: Arc<MongoRepo>,
) -> Result<impl Reply, Rejection> {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(with_status(json(&"Invalid ID format"), StatusCode::BAD_REQUEST)),
    };

    match repo.delete_restaurant(object_id).await {
        Ok(_) => Ok(with_status(json(&""), StatusCode::NO_CONTENT)),
        Err(AppError::NotFound) => Ok(with_status(json(&"Not found"), StatusCode::NOT_FOUND)),
        Err(e) => Ok(with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}