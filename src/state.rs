use crate::models::player::Player;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut players = Vec::new();

        // Helper: hash password
        let argon2 = Argon2::default();

        let make_player = |username: &str, email: &str, password: &str| {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .expect("hashing failed")
                .to_string();

            let player = Player {
                id: Uuid::new_v4(),
                username: username.to_string(),
                email: email.to_string(),
                password_hash: password_hash.clone(),
            };

            println!(
                "Seeded player: {} ({}) -> {} ",
                player.id, player.email, player.password_hash
            );
            player
        };

        // Seed some demo users
        players.push(make_player("alice", "alice@example.com", "password1"));
        players.push(make_player("bob", "bob@example.com", "hunter2"));
        players.push(make_player("charlie", "charlie@example.com", "letmein"));

        Self {
            players: Arc::new(Mutex::new(players)),
        }
    }
}
