use crate::{models::player::PlayerPublic, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
// … rest of code

#[derive(serde::Serialize)]
pub struct PlayerLookupResponse {
    found: bool,
    player: Option<PlayerPublic>,
}

pub async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>, // 👈 Axum parses path param directly into Uuid
) -> Json<PlayerLookupResponse> {
    let players = state.players.lock().unwrap();

    let player = players.iter().find(|p| p.id == id).map(|p| PlayerPublic {
        id: p.id,
        username: p.username.clone(),
        email: p.email.clone(),
    });

    Json(PlayerLookupResponse {
        found: player.is_some(),
        player,
    })
}
