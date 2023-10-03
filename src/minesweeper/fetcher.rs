use serde::{Deserialize, Serialize};

use crate::minesweeper::error::MinesweeperError;

#[derive(Serialize, Deserialize)]
pub struct ApiData {
    #[serde(rename = "gameData")]
    pub game_data: Option<String>,
    #[serde(rename = "type")]
    pub tiepe: String,
    pub time: u64,
    pub generator: String,
    pub uuid: String,
    #[serde(rename = "correctFlags")]
    pub correct_flags: u32,
    #[serde(rename = "incorrectFlags")]
    pub incorrect_flags: u32,
    pub won: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
}

pub(crate) fn fetch_data(gameid: u64) -> Result<ApiData, MinesweeperError> {
    let request_data =
        ureq::get(format!("http://api.greev.eu/v2/stats/minesweeper/game/{gameid}").as_ref())
            .call()
            .map_err(|_| MinesweeperError::GameDataNotFound)?
            .into_string()
            .map_err(|_| MinesweeperError::GameDataNotFound)?;

    serde_json::from_str(request_data.as_ref()).map_err(|_| MinesweeperError::ApiDataParse)
}

pub(crate) fn fetch_name(uuid: &str) -> Result<PlayerData, MinesweeperError> {
    let request_data = ureq::get(format!("http://api.greev.eu/v2/player/name/{uuid}").as_ref())
        .call()
        .map_err(|_| MinesweeperError::GameDataNotFound)?
        .into_string()
        .map_err(|_| MinesweeperError::GameDataNotFound)?;

    serde_json::from_str(request_data.as_ref()).map_err(|_| MinesweeperError::ApiDataParse)
}
