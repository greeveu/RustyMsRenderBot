use crate::minesweeper::base36;
use crate::minesweeper::error::MinesweeperError;
use crate::minesweeper::provider::provider::{ApiData, PlayerData, Provider};
use serde::{Deserialize, Serialize};

pub struct McPlayHdProvider;

impl Provider for McPlayHdProvider {
    fn id(&self) -> &str {
        "mcplayhd"
    }

    fn name(&self) -> &str {
        "McPlayHD"
    }

    fn fetch_data(&self, gameid: &str) -> Result<ApiData, MinesweeperError> {
        let api_key = get_api_key();
        if String::is_empty(&api_key) {
            return Err(MinesweeperError::ApiKeyNotFound);
        }

        let id = base36::decode(gameid);

        let request_data =
            ureq::get(format!("https://mcplayhd.net/api/v1/minesweeper/game/{id}").as_ref())
                .set("Authorization", format!("Bearer {api_key}").as_str())
                .call()
                .map_err(|_| MinesweeperError::GameDataNotFound)?
                .into_string()
                .map_err(|_| MinesweeperError::GameDataNotFound)?;

        let ms_data: Response = serde_json::from_str(request_data.as_ref())
            .map_err(|_| MinesweeperError::ApiDataParse)?;

        Ok(ApiData {
            game_data: Some(ms_data.data.game_info.algebraic_notation.clone()),
            tiepe: None,
            time: ms_data.data.game_info.time_taken,
            generator: None,
            uuid: ms_data.data.game_info.uuid.clone(),
            correct_flags: Some(ms_data.data.game_info.flags_correct),
            incorrect_flags: Some(ms_data.data.game_info.flags_incorrect),
            won: ms_data.data.game_info.won,
        })
    }

    fn fetch_name(&self, uuid: &str) -> Result<PlayerData, MinesweeperError> {
        let request_data = ureq::get(format!("http://api.greev.eu/v2/player/name/{uuid}").as_ref())
            .call()
            .map_err(|_| MinesweeperError::GameDataNotFound)?
            .into_string()
            .map_err(|_| MinesweeperError::GameDataNotFound)?;

        serde_json::from_str(request_data.as_ref()).map_err(|_| MinesweeperError::ApiDataParse)
    }
}

fn get_api_key() -> String {
    std::env::var("MCPLAYHD_API_KEY").unwrap_or_default()
}

#[derive(Debug, Serialize, Deserialize)]
struct GameInfo {
    id: u32,
    uuid: String,
    won: bool,
    #[serde(rename = "flagsCorrect")]
    flags_correct: u32,
    #[serde(rename = "flagsIncorrect")]
    flags_incorrect: u32,
    #[serde(rename = "timeStart")]
    time_start: u64,
    #[serde(rename = "timeEnd")]
    time_end: u64,
    #[serde(rename = "timeTaken")]
    time_taken: u64,
    mines: u32,
    #[serde(rename = "sizeX")]
    size_x: u32,
    #[serde(rename = "sizeZ")]
    size_z: u32,
    #[serde(rename = "algebraicNotation")]
    algebraic_notation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    uuid: String,
    name: String,
    group: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    #[serde(rename = "gameInfo")]
    game_info: GameInfo,
    players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status: u32,
    data: Data,
}
