use rocket::{
    self,
    serde::json::Json,
    State,
    response::status::Created,
    http::Status,
    routes, // Import the `routes` macro
};
use mongodb::Database;
use bson::oid::ObjectId;
use serde_json::Value;

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
    error::AppError,
};

#[rocket::get("/restaurants")]
async fn list_restaurants(repo: &State<MongoRepo>) -> Result<Json<Vec<Restaurant>>, Status> {
    match repo.get_restaurants(10).await {
        Ok(restaurants) => Ok(Json(restaurants)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::get("/restaurants/<id>")]
async fn get_restaurant(repo: &State<MongoRepo>, id: &str) -> Result<Json<Restaurant>, Status> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match repo.get_restaurant_by_id(object_id).await {
        Ok(restaurant) => Ok(Json(restaurant)),
        Err(AppError::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::post("/restaurants", data = "<restaurant>")]
async fn create_restaurant(
    repo: &State<MongoRepo>,
    restaurant: Json<Restaurant>,
) -> Result<Created<Json<Restaurant>>, Status> {
    match repo.create_restaurant(restaurant.into_inner()).await {
        Ok(created) => Ok(Created::new("/").body(Json(created))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::put("/restaurants/<id>", data = "<update>")]
async fn update_restaurant(
    repo: &State<MongoRepo>,
    id: &str,
    update: Json<bson::Document>,
) -> Result<Json<Restaurant>, Status> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match repo.update_restaurant(object_id, update.into_inner()).await {
        Ok(updated) => Ok(Json(updated)),
        Err(AppError::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::delete("/restaurants/<id>")]
async fn delete_restaurant(repo: &State<MongoRepo>, id: &str) -> Status {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Status::BadRequest,
    };

    match repo.delete_restaurant(object_id).await {
        Ok(_) => Status::NoContent,
        Err(AppError::NotFound) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    let repo = MongoRepo::new(&db);
    
    println!("Starting Rocket server at http://127.0.0.1:8082");
    
    let config = rocket::Config::figment()
        .merge(("port", 8082));
    
    rocket::custom(config)
        .manage(repo)
        .mount("/api", routes![
            list_restaurants,
            get_restaurant,
            create_restaurant,
            update_restaurant,
            delete_restaurant,
        ])
        .launch()
        .await?;
    
    Ok(())
}