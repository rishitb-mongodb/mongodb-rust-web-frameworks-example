use tide::{Request, Response, StatusCode};
use mongodb::Database;
use bson::oid::ObjectId;
use serde_json::Value;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
    error::AppError,
};

#[derive(Clone)]
struct State {
    repo: Arc<MongoRepo>,
    runtime: Arc<Runtime>,
}

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    // Create a Tokio runtime for MongoDB operations
    let runtime = Arc::new(Runtime::new()?);
    
    let state = State {
        repo: Arc::new(MongoRepo::new(&db)),
        runtime,
    };
    
    let mut app = tide::with_state(state);
    
    app.at("/api/restaurants")
        .post(create_restaurant)
        .get(list_restaurants);
    
    app.at("/api/restaurants/:id")
        .get(get_restaurant)
        .put(update_restaurant)
        .delete(delete_restaurant);

    println!("Starting Tide server at http://127.0.0.1:8084");
    
    app.listen("127.0.0.1:8084").await?;
    
    Ok(())
}

async fn create_restaurant(mut req: Request<State>) -> tide::Result {
    let restaurant: Restaurant = req.body_json().await?;
    let repo = req.state().repo.clone();
    let runtime = req.state().runtime.clone();
    
    let result = runtime.block_on(async move {
        repo.create_restaurant(restaurant).await
    });
    
    match result {
        Ok(created) => Ok(Response::builder(StatusCode::Created)
            .body(tide::Body::from_json(&created)?)
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(e.to_string())
            .build()),
    }
}

async fn list_restaurants(req: Request<State>) -> tide::Result {
    let repo = req.state().repo.clone();
    let runtime = req.state().runtime.clone();
    
    let result = runtime.block_on(async move {
        repo.get_restaurants(10).await
    });
    
    match result {
        Ok(restaurants) => Ok(Response::builder(StatusCode::Ok)
            .body(tide::Body::from_json(&restaurants)?)
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(e.to_string())
            .build()),
    }
}

async fn get_restaurant(req: Request<State>) -> tide::Result {
    let id = req.param("id")?;
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Ok(Response::builder(StatusCode::BadRequest)
            .body("Invalid ID format")
            .build()),
    };

    let repo = req.state().repo.clone();
    let runtime = req.state().runtime.clone();
    
    let result = runtime.block_on(async move {
        repo.get_restaurant_by_id(object_id).await
    });

    match result {
        Ok(restaurant) => Ok(Response::builder(StatusCode::Ok)
            .body(tide::Body::from_json(&restaurant)?)
            .build()),
        Err(AppError::NotFound) => Ok(Response::builder(StatusCode::NotFound)
            .body("Restaurant not found")
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(e.to_string())
            .build()),
    }
}

async fn update_restaurant(mut req: Request<State>) -> tide::Result {
    let id = req.param("id")?;
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Ok(Response::builder(StatusCode::BadRequest)
            .body("Invalid ID format")
            .build()),
    };

    let update: Value = req.body_json().await?;
    let update_doc = match bson::to_document(&update) {
        Ok(doc) => doc,
        Err(_) => return Ok(Response::builder(StatusCode::BadRequest)
            .body("Invalid update document")
            .build()),
    };

    let repo = req.state().repo.clone();
    let runtime = req.state().runtime.clone();
    
    let result = runtime.block_on(async move {
        repo.update_restaurant(object_id, update_doc).await
    });

    match result {
        Ok(updated) => Ok(Response::builder(StatusCode::Ok)
            .body(tide::Body::from_json(&updated)?)
            .build()),
        Err(AppError::NotFound) => Ok(Response::builder(StatusCode::NotFound)
            .body("Restaurant not found")
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(e.to_string())
            .build()),
    }
}

async fn delete_restaurant(req: Request<State>) -> tide::Result {
    let id = req.param("id")?;
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Ok(Response::builder(StatusCode::BadRequest)
            .body("Invalid ID format")
            .build()),
    };

    let repo = req.state().repo.clone();
    let runtime = req.state().runtime.clone();
    
    let result = runtime.block_on(async move {
        repo.delete_restaurant(object_id).await
    });

    match result {
        Ok(_) => Ok(Response::builder(StatusCode::NoContent).build()),
        Err(AppError::NotFound) => Ok(Response::builder(StatusCode::NotFound)
            .body("Restaurant not found")
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(e.to_string())
            .build()),
    }
}