use crate::models::user::{User, SafeUser};
use crate::dtos::auth_dto::RegisterDTO;
use reqwest::Client;
use uuid::Uuid;
use serde_json::json;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct AuthSupabaseRepo {
    pub project_url: String,
    pub api_key: String,
}

impl AuthSupabaseRepo {
    fn base_url(&self) -> String {
        let mut url = self.project_url.clone();
        if !url.ends_with("/rest/v1") {
            url = format!("{}/rest/v1", url.trim_end_matches('/'));
        }
        format!("{}/users", url)
    }

    pub async fn register(&self, dto: RegisterDTO) -> Result<SafeUser, String> {
        // Validate password confirmation
        if dto.password != dto.confirm_password {
            return Err("Password and confirm password do not match".to_string());
        }

        println!("=== REGISTER DEBUG START ===");
        println!("Base URL: {}", self.base_url());
        println!("DTO: {:?}", dto);
        
        let client = Client::new();
        
        // Check if email already exists
        let check_url = format!("{}?email=eq.{}", self.base_url(), dto.email);
        println!("Check URL: {}", check_url);
        
        let check_res = client
            .get(&check_url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to check existing email: {}", e))?;
        
        println!("Check response status: {}", check_res.status());
        let check_text = check_res.text().await
            .map_err(|e| format!("Failed to get check response text: {}", e))?;
        println!("Check response body: {}", check_text);
            
        let existing_users: Vec<User> = serde_json::from_str(&check_text)
            .map_err(|e| format!("Failed to parse existing users response: {} | Response was: {}", e, check_text))?;
        
        if !existing_users.is_empty() {
            return Err("Email already exists".to_string());
        }
        
        let id = Uuid::new_v4();
        let hashed_password = hash(&dto.password, DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;
        
        let payload = json!({
            "id": id,
            "name": dto.name,
            "email": dto.email,
            "password": hashed_password,
            "city": dto.city,
            "birth_date": dto.birth_date
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
            .map_err(|e| format!("Failed to send request to Supabase: {}", e))?;

        let status = res.status();
        let text = res.text().await
            .map_err(|e| format!("Failed to get response text: {}", e))?;
        
        if !status.is_success() {
            return Err(format!("Supabase error ({}): {}", status, text));
        }
        
        // Debug: Print the actual response
        println!("Supabase response: {}", text);
        
        // Try to parse as array first (standard Supabase response)
        if let Ok(users) = serde_json::from_str::<Vec<User>>(&text) {
            if let Some(user) = users.into_iter().next() {
                return Ok(SafeUser::from(user));
            } else {
                return Err("No user returned in array response".to_string());
            }
        }
        
        // If array parsing fails, try parsing as single object
        if let Ok(user) = serde_json::from_str::<User>(&text) {
            return Ok(SafeUser::from(user));
        }
        
        // If both fail, return detailed error
        Err(format!("Failed to parse Supabase response. Response was: {}", text))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<SafeUser, String> {
        let client = Client::new();
        let url = format!("{}?email=eq.{}", self.base_url(), email);
        
        let res = client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to send login request: {}", e))?;
            
        let users: Vec<User> = res.json().await
            .map_err(|e| format!("Failed to parse login response: {}", e))?;
        
        let user = users.into_iter().next()
            .ok_or_else(|| "Invalid email or password".to_string())?;
        
        // Verify password
        if verify(password, &user.password).map_err(|e| format!("Password verification error: {}", e))? {
            Ok(SafeUser::from(user))
        } else {
            Err("Invalid email or password".to_string())
        }
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<SafeUser, String> {
        let client = Client::new();
        let url = format!("{}?id=eq.{}", self.base_url(), id);
        
        let res = client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to get user by id: {}", e))?;
            
        let users: Vec<User> = res.json().await
            .map_err(|e| format!("Failed to parse get_user response: {}", e))?;
        
        let user = users.into_iter().next()
            .ok_or_else(|| "User not found".to_string())?;
        
        Ok(SafeUser::from(user))
    }
}