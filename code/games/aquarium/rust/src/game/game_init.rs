use crate::game::Game;

use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Sums {
    cols: Vec<i32>,
    rows: Vec<i32>,
}

#[derive(Deserialize)]
struct JsonGame {
    sums: Sums,
    matrix: Vec<Vec<usize>>,
}

impl Game {
    /// Pull a game from the actual aquarium source using a game id.
    pub async fn from_id(id: &str) -> Option<Game> {
        let url = "https://aquarium2.vercel.app/api/get?id=";
        let res = reqwest::get(format!("{url}{id}")).await.ok()?;
        let j = res.json::<JsonGame>().await.ok()?;
        Some(Self::new(j.sums.rows, j.sums.cols, j.matrix))
    }

    /// Pull a game from the actual aquarium source using a game id.
    pub fn from_filename<P: AsRef<Path>>(filename: P) -> Option<Game> {
        let data = std::fs::read_to_string(filename).ok()?;
        Game::from_json(&data)
    }

    fn from_json(json: &str) -> Option<Game> {
        let j = serde_json::from_str::<JsonGame>(&json).ok()?;
        Some(Self::new(j.sums.rows, j.sums.cols, j.matrix))
    }
}
