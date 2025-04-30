use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mongodb::Database;
use bson::oid::ObjectId;
use serde_json::Value;

use crate::{
    db::mongodb::MongoRepo,
    models::restaurant::Restaurant,
    error::AppError,
};

pub async fn start(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    let repo = web::Data::new(MongoRepo::new(&db));
    
    println!("Starting Actix Web server at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .service(
                web::scope("/api")
                    .route("/restaurants", web::post().to(create_restaurant))
                    .route("/restaurants", web::get().to(list_restaurants))
                    .route("/restaurants/{id}", web::get().to(get_restaurant))
                    .route("/restaurants/{id}", web::put().to(update_restaurant))
                    .route("/restaurants/{id}", web::delete().to(delete_restaurant))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}

async fn create_restaurant(
    repo: web::Data<MongoRepo>,
    restaurant: web::Json<Restaurant>,
) -> impl Responder {
    match repo.create_restaurant(restaurant.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn list_restaurants(repo: web::Data<MongoRepo>) -> impl Responder {
    match repo.get_restaurants(10).await {
        Ok(restaurants) => HttpResponse::Ok().json(restaurants),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_restaurant(
    repo: web::Data<MongoRepo>,
    id: web::Path<String>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&*id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    match repo.get_restaurant_by_id(object_id).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(AppError::NotFound) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_restaurant(
    repo: web::Data<MongoRepo>,
    id: web::Path<String>,
    update: web::Json<Value>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&*id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let update_doc = match bson::to_document(&update) {
        Ok(doc) => doc,
        Err(_) => return HttpResponse::BadRequest().body("Invalid update document"),
    };

    match repo.update_restaurant(object_id, update_doc).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(AppError::NotFound) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_restaurant(
    repo: web::Data<MongoRepo>,
    id: web::Path<String>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&*id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    match repo.delete_restaurant(object_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(AppError::NotFound) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}