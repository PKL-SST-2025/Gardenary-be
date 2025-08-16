// plant_supabase.rs - FIXED VERSION
use crate::models::plant::Plant;
use crate::dtos::plant_dto::{CreatePlantDTO, UpdatePlantDTO, UpdatePlantStatusDTO};
use reqwest::Client;
use uuid::Uuid;
use serde_json::json;
use chrono::Utc;

pub struct PlantSupabaseRepo {
    pub project_url: String,
    pub api_key: String,
}

impl PlantSupabaseRepo {
    fn base_url(&self) -> String {
        let mut url = self.project_url.clone();
        if !url.ends_with("/rest/v1") {
            url = format!("{}/rest/v1", url.trim_end_matches('/'));
        }
        format!("{}/plants", url)
    }

    pub async fn add(&self, dto: CreatePlantDTO) -> Result<Plant, String> {
        let client = Client::new();
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let payload = json!({
            "id": id,
            "name": dto.name,
            "plant_type": dto.plant_type,
            "image": dto.image,
            "planted_date": now.to_rfc3339(),
            "age": 0,
            "user_id": dto.user_id,
            "status": json!({}),
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339(),
        });
        
        let res = client
            .post(&self.base_url())
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let text = res.text().await.map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err(format!("Supabase error: {}", text));
        }
        
        let mut arr: Vec<Plant> = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        arr.pop().ok_or_else(|| "Failed to create plant".to_string())
    }

    pub async fn get_all_by_user(&self, user_id: Uuid) -> Result<Vec<Plant>, String> {
        let client = Client::new();
        let url = format!("{}?user_id=eq.{}&order=created_at.desc", self.base_url(), user_id);
        
        let res = client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        let arr: Vec<Plant> = res.json().await.map_err(|e| e.to_string())?;
        Ok(arr)
    }

    pub async fn get_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Plant, String> {
        let client = Client::new();
        let url = format!("{}?id=eq.{}&user_id=eq.{}", self.base_url(), id, user_id);
        
        let res = client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        let mut arr: Vec<Plant> = res.json().await.map_err(|e| e.to_string())?;
        arr.pop().ok_or_else(|| "Plant not found".to_string())
    }

    pub async fn update(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantDTO) -> Result<Plant, String> {
        let client = Client::new();
        let url = format!("{}?id=eq.{}&user_id=eq.{}", self.base_url(), id, user_id);
        
        let mut payload = serde_json::Map::new();
        if let Some(name) = dto.name { 
            payload.insert("name".to_string(), json!(name)); 
        }
        if let Some(plant_type) = dto.plant_type { 
            payload.insert("plant_type".to_string(), json!(plant_type)); 
        }
        if let Some(image) = dto.image { 
            payload.insert("image".to_string(), json!(image)); 
        }
        if let Some(status) = dto.status { 
            payload.insert("status".to_string(), status); 
        }
        payload.insert("updated_at".to_string(), json!(Utc::now().to_rfc3339()));

        let res = client
            .patch(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let mut arr: Vec<Plant> = res.json().await.map_err(|e| e.to_string())?;
        arr.pop().ok_or_else(|| "Plant not found/updated".to_string())
    }

    pub async fn update_status(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantStatusDTO) -> Result<Plant, String> {
        // First get the current plant
        let current = self.get_by_id(id, user_id).await?;
        
        let mut status = current.status.clone();
        // FIX: Simplified pattern matching for Rust 2021
        if let serde_json::Value::Object(mut map) = status {
            let date_status = map.entry(&dto.date).or_insert_with(|| json!({
                "watered": false,
                "fertilized": false,
                "harvested": false
            }));
            
            if let serde_json::Value::Object(date_map) = date_status {
                date_map.insert(dto.status_type, json!(dto.value));
            }
            status = serde_json::Value::Object(map);
        } else {
            status = json!({
                dto.date: {
                    "watered": dto.status_type == "watered" && dto.value,
                    "fertilized": dto.status_type == "fertilized" && dto.value,
                    "harvested": dto.status_type == "harvested" && dto.value,
                }
            });
        }
        
        let update_dto = UpdatePlantDTO {
            name: None,
            plant_type: None,
            image: None,
            status: Some(status),
        };
        
        self.update(id, user_id, update_dto).await
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<u64, String> {
        let client = Client::new();
        let url = format!("{}?id=eq.{}&user_id=eq.{}", self.base_url(), id, user_id);
        
        let res = client
            .delete(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .header("Prefer", "return=representation")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(1)
        } else {
            Err("Failed to delete plant".to_string())
        }
    }

    pub async fn get_dashboard_stats(&self, user_id: Uuid, date: &str) -> Result<serde_json::Value, String> {
        let plants = self.get_all_by_user(user_id).await?;
        
        let mut watered_count = 0;
        let mut fertilized_count = 0;
        let mut harvested_count = 0;
        let total_plants = plants.len();
        
        for plant in &plants {
            if let Some(date_status) = plant.status.get(date) {
                if let Some(watered) = date_status.get("watered") {
                    if watered.as_bool().unwrap_or(false) {
                        watered_count += 1;
                    }
                }
                if let Some(fertilized) = date_status.get("fertilized") {
                    if fertilized.as_bool().unwrap_or(false) {
                        fertilized_count += 1;
                    }
                }
                if let Some(harvested) = date_status.get("harvested") {
                    if harvested.as_bool().unwrap_or(false) {
                        harvested_count += 1;
                    }
                }
            }
        }
        
        Ok(json!({
            "total_plants": total_plants,
            "watered_today": watered_count,
            "fertilized_today": fertilized_count,
            "harvested_today": harvested_count,
            "need_watering": total_plants - watered_count,
            "need_fertilizing": total_plants - fertilized_count,
            "ready_to_harvest": plants.iter().filter(|p| {
                if let Some(date_status) = p.status.get(date) {
                    let watered = date_status.get("watered").and_then(|v| v.as_bool()).unwrap_or(false);
                    let fertilized = date_status.get("fertilized").and_then(|v| v.as_bool()).unwrap_or(false);
                    let harvested = date_status.get("harvested").and_then(|v| v.as_bool()).unwrap_or(false);
                    watered && fertilized && !harvested
                } else {
                    false
                }
            }).count()
        }))
    }
}