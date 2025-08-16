use crate::dtos::plant_dto::{CreatePlantDTO, UpdatePlantDTO, UpdatePlantStatusDTO};
use crate::models::plant::Plant;
use crate::repositories::plant_postgres::PlantPostgresRepo;
use crate::repositories::plant_supabase::PlantSupabaseRepo;
use uuid::Uuid;

pub struct PlantService {
    pub pg_repo: PlantPostgresRepo,
    pub sb_repo: PlantSupabaseRepo,
}

impl PlantService {
    // PostgreSQL methods
    pub async fn add_pg(&self, dto: CreatePlantDTO) -> Result<Plant, String> {
        self.pg_repo.add(dto).await
    }
    
    pub async fn get_all_pg(&self, user_id: Uuid) -> Result<Vec<Plant>, String> {
        self.pg_repo.get_all_by_user(user_id).await
    }
    
    pub async fn get_by_id_pg(&self, id: Uuid, user_id: Uuid) -> Result<Plant, String> {
        self.pg_repo.get_by_id(id, user_id).await
    }
    
    pub async fn update_pg(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantDTO) -> Result<Plant, String> {
        self.pg_repo.update(id, user_id, dto).await
    }
    
    pub async fn update_status_pg(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantStatusDTO) -> Result<Plant, String> {
        self.pg_repo.update_status(id, user_id, dto).await
    }
    
    pub async fn delete_pg(&self, id: Uuid, user_id: Uuid) -> Result<u64, String> {
        self.pg_repo.delete(id, user_id).await
    }
    
    pub async fn get_dashboard_stats_pg(&self, user_id: Uuid, date: &str) -> Result<serde_json::Value, String> {
        self.pg_repo.get_dashboard_stats(user_id, date).await
    }

    // Supabase methods
    pub async fn add_sb(&self, dto: CreatePlantDTO) -> Result<Plant, String> {
        self.sb_repo.add(dto).await
    }
    
    pub async fn get_all_sb(&self, user_id: Uuid) -> Result<Vec<Plant>, String> {
        self.sb_repo.get_all_by_user(user_id).await
    }
    
    pub async fn get_by_id_sb(&self, id: Uuid, user_id: Uuid) -> Result<Plant, String> {
        self.sb_repo.get_by_id(id, user_id).await
    }
    
    pub async fn update_sb(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantDTO) -> Result<Plant, String> {
        self.sb_repo.update(id, user_id, dto).await
    }
    
    pub async fn update_status_sb(&self, id: Uuid, user_id: Uuid, dto: UpdatePlantStatusDTO) -> Result<Plant, String> {
        self.sb_repo.update_status(id, user_id, dto).await
    }
    
    pub async fn delete_sb(&self, id: Uuid, user_id: Uuid) -> Result<u64, String> {
        self.sb_repo.delete(id, user_id).await
    }
    
    pub async fn get_dashboard_stats_sb(&self, user_id: Uuid, date: &str) -> Result<serde_json::Value, String> {
        self.sb_repo.get_dashboard_stats(user_id, date).await
    }
}