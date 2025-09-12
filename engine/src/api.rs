use super::*;
use axum::{extract::State, routing::post, Json, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::Deserialize;

// The current state of the app
#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<GTP>>,
}

#[derive (Deserialize)]
struct CommandInput {
    board_size: u16,
    move_list: Vec<String>,
    next_command: String,
}

async fn read_json(Json(payload): Json<CommandInput>) -> String {
    let mut gtp = GTP::new();
    if BoardSize::from_u16(payload.board_size).is_some() {
        gtp.accept_command(format!("boardsize {}", payload.board_size));
        for mov in payload.move_list {
            gtp.accept_command(format!("play {}", mov));
        }

        gtp.accept_command(payload.next_command)
    } else {
        format!("Invalid board size {} given", payload.board_size)
    }
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
    // let shared_state = AppState {
    //     engine: Arc::new(Mutex::new(GTP::new())),
    // };
    let app = Router::new()
        //.route("/", post(read_command).with_state(shared_state))
        .route("/", post(read_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}