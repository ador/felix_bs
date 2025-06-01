use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use std::fmt;
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Movements {
    Up,
    //#[serde(rename = "down")]
    Down,
    //#[serde(rename = "left")]
    Left,
    //#[serde(rename = "right")]
    Right,
}

impl Movements {
    const ALL: [Movements; 4] = [
        Movements::Up,
        Movements::Down,
        Movements::Left,
        Movements::Right,
    ];

    fn coords(&self) -> Coord {
        match self {
            Movements::Up => Coord::new(0, 1),
            Movements::Down => Coord::new(0, -1),
            Movements::Left => Coord::new(-1, 0),
            Movements::Right => Coord::new(1, 0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customizations {
    color: String,
    head: String,
    tail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Coord>,
    latency: String,
    head: Coord,
    length: i32,
    shout: String,
    squad: Option<String>,
    customizations: Customizations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    name: String,
    version: String,
    settings: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
    pub map: String,
    pub timeout: i32,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineInput {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, Clone)]
struct MoveResponse {
    pub r#move: Movements,
    //pub rmove: Movements,
    pub shout: &'static str,
}

impl MoveResponse {
    const _SHOUT_MAX_LENGTH: usize = 256;
}

/// Handles POST /start ... used to start a new game
/// response is irrelevant
pub async fn handle_game_start(Json(input): Json<EngineInput>) -> impl IntoResponse {
    //TODO: impl
    println!("Game start with Game id: {}", input.game.id);
    StatusCode::OK
}

/// Handles POST /end ... used to end the current game
/// response is irrelevant as
pub async fn handle_game_over(Json(input): Json<EngineInput>) -> impl IntoResponse {
    println!("Game ended with Game id: {}", input.game.id);
    StatusCode::OK
}

/// Handles POST /move ... used to move the snake
pub async fn handle_move(Json(input): Json<EngineInput>) -> impl IntoResponse {
    println!("Move?");
    let response = MoveResponse {
        r#move: Movements::Up,
        shout: "hi",
    };
    // (StatusCode::OK, Json(response))
    StatusCode::OK
}
