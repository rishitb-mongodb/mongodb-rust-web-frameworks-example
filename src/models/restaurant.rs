use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use mongodb::bson::DateTime;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Restaurant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default)]
    pub borough: String,
    #[serde(default)]
    pub cuisine: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grades: Vec<Grade>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub restaurant_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Address {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub building: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coord: Vec<f64>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub street: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub zipcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade {    
    pub date: DateTime,
    pub grade: String,
    pub score: i32,
}