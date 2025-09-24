mod handlers;
mod models;
mod routes;
mod state;

use crate::routes::create_router;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Initialise app state
    let state = AppState::new();

    // Build router with state
    let app = create_router(state);

    // Bind a TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "✅ Server running at http://{}",
        listener.local_addr().unwrap()
    );

    // Run the server (new style in axum 0.8.x)
    axum::serve(listener, app).await.unwrap();
}
