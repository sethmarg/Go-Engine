use super::*;
use axum::{extract::State, routing::post, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

// The current state of the app
#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<GTP>>,
}

// Given the current API state and a GTP command, returns the output of the GTP command
// and updates the current API state appropriately
async fn read_command(State(state): State<AppState>, command: String) -> String {
    let response = state.engine.lock().await.accept_command(command);

    if response == "quit" {
        std::process::exit(0);
    }

    response
}

// Begins a new listener for HTTP requests on port 3000 for GTP commands
#[tokio::main]
pub async fn start_api() {
    let shared_state = AppState {
        engine: Arc::new(Mutex::new(GTP::new())),
    };
    let app = Router::new().route("/", post(read_command).with_state(shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}