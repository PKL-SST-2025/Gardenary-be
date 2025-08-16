use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub city: Option<String>,
    pub birth_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub city: Option<String>,
    pub birth_date: Option<String>,
}