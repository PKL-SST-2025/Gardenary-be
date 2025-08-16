use crate::dtos::auth_dto::{RegisterDTO, LoginResponse, UserInfo};
use crate::models::user::SafeUser;
use crate::repositories::auth_postgres::AuthPostgresRepo;
use crate::repositories::auth_supabase::AuthSupabaseRepo;
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject (user id)
    exp: usize,  // expiration time
    iat: usize,  // issued at
}

pub struct AuthService {
    pub pg_repo: AuthPostgresRepo,
    pub sb_repo: AuthSupabaseRepo,
    pub jwt_secret: String,
}

impl AuthService {
    fn generate_token(&self, user_id: Uuid) -> Result<String, String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token valid for 24 hours
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        ).map_err(|e| e.to_string())
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid, String> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|e| e.to_string())?;

        Uuid::parse_str(&token_data.claims.sub).map_err(|e| e.to_string())
    }

    // PostgreSQL methods
    pub async fn register_pg(&self, dto: RegisterDTO) -> Result<LoginResponse, String> {
        let user = self.pg_repo.register(dto).await?;
        let token = self.generate_token(user.id)?;
        
        Ok(LoginResponse {
            user: UserInfo {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
                city: user.city,
                birth_date: user.birth_date,
            },
            token,
        })
    }

    pub async fn login_pg(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let user = self.pg_repo.login(email, password).await?;
        let token = self.generate_token(user.id)?;
        
        Ok(LoginResponse {
            user: UserInfo {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
                city: user.city,
                birth_date: user.birth_date,
            },
            token,
        })
    }

    pub async fn get_user_by_id_pg(&self, id: Uuid) -> Result<SafeUser, String> {
        self.pg_repo.get_user_by_id(id).await
    }

    // Supabase methods
    pub async fn register_sb(&self, dto: RegisterDTO) -> Result<LoginResponse, String> {
        let user = self.sb_repo.register(dto).await?;
        let token = self.generate_token(user.id)?;
        
        Ok(LoginResponse {
            user: UserInfo {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
                city: user.city,
                birth_date: user.birth_date,
            },
            token,
        })
    }

    pub async fn login_sb(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let user = self.sb_repo.login(email, password).await?;
        let token = self.generate_token(user.id)?;
        
        Ok(LoginResponse {
            user: UserInfo {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
                city: user.city,
                birth_date: user.birth_date,
            },
            token,
        })
    }

    pub async fn get_user_by_id_sb(&self, id: Uuid) -> Result<SafeUser, String> {
        self.sb_repo.get_user_by_id(id).await
    }
}