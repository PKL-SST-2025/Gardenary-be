use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: Uuid,
    pub name: String,
    pub plant_type: String,
    pub image: Option<String>,
    pub planted_date: DateTime<Utc>,
    pub age: i32,
    pub user_id: Uuid,
    pub status: serde_json::Value, // JSON object for daily status
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantStatus {
    pub watered: bool,
    pub fertilized: bool,
    pub harvested: bool,
}

impl Default for PlantStatus {
    fn default() -> Self {
        Self {
            watered: false,
            fertilized: false,
            harvested: false,
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,      
    pub message: String,      
    pub data: Option<T>,      
}