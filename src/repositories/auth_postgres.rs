use crate::models::user::{User, SafeUser};
use crate::dtos::auth_dto::RegisterDTO;
use deadpool_postgres::Pool;
use uuid::Uuid;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct AuthPostgresRepo {
    pub pool: Pool,
}

fn user_from_row(row: Row) -> User {
    User {
        id: row.get::<_, Uuid>("id"),
        name: row.get("name"),
        email: row.get("email"),
        password: row.get("password"),
        city: row.get("city"),
        birth_date: row.get("birth_date"),
        created_at: row.get("created_at"),
    }
}

impl AuthPostgresRepo {
    pub async fn register(&self, dto: RegisterDTO) -> Result<SafeUser, String> {
        // Validate password confirmation
        if dto.password != dto.confirm_password {
            return Err("Password and confirm password do not match".to_string());
        }

        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        
        // Check if email already exists
        let check_stmt = client.prepare("SELECT id FROM users WHERE email = $1").await.map_err(|e| e.to_string())?;
        if client.query_opt(&check_stmt, &[&dto.email]).await.map_err(|e| e.to_string())?.is_some() {
            return Err("Email already exists".to_string());
        }
        
        let id = Uuid::new_v4();
        let hashed_password = hash(&dto.password, DEFAULT_COST).map_err(|e| e.to_string())?;
        
        let stmt = client.prepare(
            "INSERT INTO users (id, name, email, password, city, birth_date, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, NOW()) 
             RETURNING id, name, email, password, city, birth_date, created_at::text"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_one(&stmt, &[
            &id,
            &dto.name,
            &dto.email,
            &hashed_password,
            &dto.city,
            &dto.birth_date
        ]).await.map_err(|e| e.to_string())?;
        
        let user = user_from_row(row);
        Ok(SafeUser::from(user))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<SafeUser, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        
        let stmt = client.prepare(
            "SELECT id, name, email, password, city, birth_date, created_at::text 
             FROM users WHERE email = $1"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_opt(&stmt, &[&email])
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Invalid email or password".to_string())?;
            
        let user = user_from_row(row);
        
        // Verify password
        if verify(password, &user.password).map_err(|e| e.to_string())? {
            Ok(SafeUser::from(user))
        } else {
            Err("Invalid email or password".to_string())
        }
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<SafeUser, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        
        let stmt = client.prepare(
            "SELECT id, name, email, password, city, birth_date, created_at::text 
             FROM users WHERE id = $1"
        ).await.map_err(|e| e.to_string())?;
        
        let row = client.query_opt(&stmt, &[&id])
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "User not found".to_string())?;
            
        let user = user_from_row(row);
        Ok(SafeUser::from(user))
    }
}