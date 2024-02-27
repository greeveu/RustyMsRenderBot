use serde::{Deserialize, Serialize};

use crate::minesweeper::error::MinesweeperError;

pub trait Provider: Sync + Send {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn fetch_data(&self, gameid: &str) -> Result<ApiData, MinesweeperError>;
    fn fetch_name(&self, uuid: &str) -> Result<PlayerData, MinesweeperError>;
}

#[derive(Serialize, Deserialize)]
pub struct ApiData {
    #[serde(rename = "gameData")]
    pub game_data: Option<String>,
    #[serde(rename = "type")]
    pub tiepe: Option<String>,
    pub time: u64,
    pub generator: Option<String>,
    pub uuid: String,
    #[serde(rename = "correctFlags")]
    pub correct_flags: Option<u32>,
    #[serde(rename = "incorrectFlags")]
    pub incorrect_flags: Option<u32>,
    pub won: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
}
