use uuid::Uuid;

use crate::{
    models::player::{Player, PlayerPublic, RegisterRequest},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand::rngs::OsRng;

/// Handler for POST /register
/// - hashes password with Argon2
/// - stores Player { password_hash }
/// - returns PlayerPublic (no password)
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<PlayerPublic>, (StatusCode, String)> {
    // Build password hash
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("hash error: {}", e),
            )
        })?
        .to_string();

    // Create player
    let new_player = Player {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash,
    };

    // Persist in-memory
    let mut players = state.players.lock().unwrap();
    players.push(new_player.clone());

    // Build public response (omit password_hash)
    let public = PlayerPublic {
        id: new_player.id,
        username: new_player.username,
        email: new_player.email,
    };

    Ok(Json(public))
}
