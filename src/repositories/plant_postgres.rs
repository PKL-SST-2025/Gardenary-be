// plant_postgres.rs - FIXED VERSION
use crate::models::plant::Plant;
use crate::dtos::plant_dto::{CreatePlantDTO, UpdatePlantDTO, UpdatePlantStatusDTO};
use deadpool_postgres::Pool;
use uuid::Uuid;
use tokio_postgres::Row;
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct PlantPostgresRepo {
    pub pool: Pool,
}

fn from_row(row: Row) -> Result<Plant, String> {
    let status_json: serde_json::Value = row.try_get("status")
        .unwrap_or_else(|_| json!({}));
    
    Ok(Plant {
        id: row.get("id"),
        name: row.get("name"),
        plant_type: row.get("plant_type"),
        image: row.get("image"),
        planted_date: row.get("planted_date"),
        age: row.get("age"),
        user_id: row.get("user_id"),
        status: status_json,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

impl PlantPostgresRepo {
    pub async fn add(&self, dto: CreatePlantDTO) -> Result<Plant, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let id = Uuid::new_v4();
        let now = Utc::now();
        let default_status = json!({});
        
        let stmt = client.prepare(
            "INSERT INTO plants (id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
             RETURNING id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_one(&stmt, &[
            &id, &dto.name, &dto.plant_type, &dto.image, &now, &0i32, 
            &dto.user_id, &default_status, &now, &now
        ]).await.map_err(|e| e.to_string())?;
        
        from_row(row)
    }

    pub async fn get_all_by_user(&self, user_id: Uuid) -> Result<Vec<Plant>, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare(
            "SELECT id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at 
             FROM plants WHERE user_id = $1 ORDER BY created_at DESC"
        ).await.map_err(|e| e.to_string())?;
        
        let rows = client.query(&stmt, &[&user_id]).await.map_err(|e| e.to_string())?;
        let mut plants = Vec::new();
        
        for row in rows {
            plants.push(from_row(row)?);
        }
        
        Ok(plants)
    }

    pub async fn get_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Plant, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare(
            "SELECT id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at 
             FROM plants WHERE id = $1 AND user_id = $2"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_one(&stmt, &[&id, &user_id]).await.map_err(|e| e.to_string())?;
        from_row(row)
    }

    pub async fn update(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantDTO) -> Result<Plant, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let current = self.get_by_id(id, user_id).await?;
        
        let name = dto.name.unwrap_or(current.name);
        let plant_type = dto.plant_type.unwrap_or(current.plant_type);
        let image = dto.image.or(current.image);
        let status = dto.status.unwrap_or(current.status);
        let now = Utc::now();

        let stmt = client.prepare(
            "UPDATE plants SET name = $1, plant_type = $2, image = $3, status = $4, updated_at = $5 
             WHERE id = $6 AND user_id = $7 
             RETURNING id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_one(&stmt, &[
            &name, &plant_type, &image, &status, &now, &id, &user_id
        ]).await.map_err(|e| e.to_string())?;
        
        from_row(row)
    }

    pub async fn update_status(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantStatusDTO) -> Result<Plant, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let current = self.get_by_id(id, user_id).await?;
        
        let mut status = current.status.clone();
        if let serde_json::Value::Object(mut map) = status {
            let date_status = map.entry(&dto.date).or_insert_with(|| json!({
                "watered": false,
                "fertilized": false,
                "harvested": false
            }));
            
            // FIX: Remove ref mut - works in edition 2021
            if let serde_json::Value::Object(date_map) = date_status {
                date_map.insert(dto.status_type, json!(dto.value));
            }
            status = serde_json::Value::Object(map);
        } else {
            // Initialize status if it's not an object
            status = json!({
                dto.date: {
                    "watered": dto.status_type == "watered" && dto.value,
                    "fertilized": dto.status_type == "fertilized" && dto.value,
                    "harvested": dto.status_type == "harvested" && dto.value,
                }
            });
        }
        
        let now = Utc::now();
        let stmt = client.prepare(
            "UPDATE plants SET status = $1, updated_at = $2 
             WHERE id = $3 AND user_id = $4 
             RETURNING id, name, plant_type, image, planted_date, age, user_id, status, created_at, updated_at"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_one(&stmt, &[&status, &now, &id, &user_id]).await.map_err(|e| e.to_string())?;
        from_row(row)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<u64, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare("DELETE FROM plants WHERE id = $1 AND user_id = $2").await.map_err(|e| e.to_string())?;
        let res = client.execute(&stmt, &[&id, &user_id]).await.map_err(|e| e.to_string())?;
        Ok(res)
    }

    // Dashboard specific queries
    pub async fn get_dashboard_stats(&self, user_id: Uuid, date: &str) -> Result<serde_json::Value, String> {
        let _client = self.pool.get().await.map_err(|e| e.to_string())?; // FIX: Add underscore
        let plants = self.get_all_by_user(user_id).await?;
        
        let mut watered_count = 0;
        let mut fertilized_count = 0;
        let mut harvested_count = 0;
        let total_plants = plants.len(); // FIX: Remove mut
        
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