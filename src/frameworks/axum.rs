use axum::{
    routing::{get, post, put, delete},
    Router, Json, extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
};
use mongodb::Database;
use bson::oid::ObjectId;
use serde_json::Value;
use std::sync::Arc;
use std::net::SocketAddr;

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
    error::AppError,
};

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Arc::new(MongoRepo::new(&db));
    
    let app = Router::new()
        .route("/api/restaurants", post(create_restaurant))
        .route("/api/restaurants", get(list_restaurants))
        .route("/api/restaurants/:id", get(get_restaurant))
        .route("/api/restaurants/:id", put(update_restaurant))
        .route("/api/restaurants/:id", delete(delete_restaurant))
        .with_state(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Starting Axum server at http://{}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn create_restaurant(
    State(repo): State<Arc<MongoRepo>>,
    Json(restaurant): Json<Restaurant>,
) -> impl IntoResponse {
    match repo.create_restaurant(restaurant).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn list_restaurants(
    State(repo): State<Arc<MongoRepo>>,
) -> impl IntoResponse {
    match repo.get_restaurants(10).await {
        Ok(restaurants) => (StatusCode::OK, Json(restaurants)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_restaurant(
    State(repo): State<Arc<MongoRepo>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response(),
    };

    match repo.get_restaurant_by_id(object_id).await {
        Ok(restaurant) => (StatusCode::OK, Json(restaurant)).into_response(),
        Err(AppError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_restaurant(
    State(repo): State<Arc<MongoRepo>>,
    Path(id): Path<String>,
    Json(update): Json<Value>,
) -> impl IntoResponse {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response(),
    };

    let update_doc = match bson::to_document(&update) {
        Ok(doc) => doc,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid update document").into_response(),
    };

    match repo.update_restaurant(object_id, update_doc).await {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(AppError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_restaurant(
    State(repo): State<Arc<MongoRepo>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response(),
    };

    match repo.delete_restaurant(object_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(AppError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}