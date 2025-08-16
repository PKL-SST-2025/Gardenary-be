use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePlantDTO {
    pub name: String,
    pub plant_type: String, // "Vegetable", "Fruit", "Herb", "Flower"
    pub image: Option<String>, // base64 or URL
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePlantDTO {
    pub name: Option<String>,
    pub plant_type: Option<String>,
    pub image: Option<String>,
    pub status: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePlantStatusDTO {
    pub date: String, // format: "2025-07-15"
    pub status_type: String, // "watered", "fertilized", "harvested"
    pub value: bool,
}
