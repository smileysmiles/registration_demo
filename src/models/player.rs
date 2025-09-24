use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain model for internal storage
#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String, // store hashed password
}

/// Public-safe player payload (what we return in APIs)
#[derive(Debug, Clone, Serialize)]
pub struct PlayerPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

/// DTO for registration input
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
