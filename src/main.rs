mod game;

use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // start game
        .route("/start", post(game::handle_game_start))
        // end game
        .route("/end", post(game::handle_game_over))
        // move in a direction on the board
        .route("/move", post(game::handle_move));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
