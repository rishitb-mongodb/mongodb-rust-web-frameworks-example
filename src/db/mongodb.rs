use mongodb::{Database, Collection, bson::{doc, Document, oid::ObjectId}};
use futures::stream::TryStreamExt;
use crate::{models::restaurant::Restaurant, error::AppError};

pub struct MongoRepo {
    collection: Collection<Restaurant>,
}

impl MongoRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("restaurants"),
        }
    }

    pub async fn create_restaurant(&self, restaurant: Restaurant) -> Result<Restaurant, AppError> {
        let result = self.collection.insert_one(restaurant, None).await?;
        let filter = doc! { "_id": result.inserted_id };
        let created_restaurant = self.collection.find_one(filter, None).await?
            .ok_or(AppError::NotFound)?;
        Ok(created_restaurant)
    }

    pub async fn get_restaurants(&self, limit: i64) -> Result<Vec<Restaurant>, AppError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut restaurants = Vec::new();
        while let Some(restaurant) = cursor.try_next().await? {
            restaurants.push(restaurant);
            if restaurants.len() >= limit as usize {
                break;
            }
        }
        Ok(restaurants)
    }

    pub async fn get_restaurant_by_id(&self, id: ObjectId) -> Result<Restaurant, AppError> {
        let filter = doc! { "_id": id };
        let restaurant = self.collection.find_one(filter, None).await?
            .ok_or(AppError::NotFound)?;
        Ok(restaurant)
    }

    pub async fn update_restaurant(&self, id: ObjectId, update: Document) -> Result<Restaurant, AppError> {
        let filter = doc! { "_id": id };
        let update_doc = doc! { "$set": update };
        
        let result = self.collection.update_one(filter.clone(), update_doc, None).await?;
        if result.modified_count == 0 {
            return Err(AppError::NotFound);
        }

        self.get_restaurant_by_id(id).await
    }

    pub async fn delete_restaurant(&self, id: ObjectId) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter, None).await?;
        if result.deleted_count == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}