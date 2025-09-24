use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub city: Option<String>,
    pub birth_date: Option<String>,
    pub created_at: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafeUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub city: Option<String>,
    pub birth_date: Option<String>,
    pub created_at: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            id: user.id,
            name: user.name,
            email: user.email,
            city: user.city,
            birth_date: user.birth_date,
            created_at: user.created_at,
            avatar: user.avatar,
            bio: user.bio,
        }
    }
}