use crate::minesweeper::error::MinesweeperError;
use crate::minesweeper::provider::provider::{ApiData, PlayerData, Provider};

pub struct McPlayHdProvider;

impl Provider for McPlayHdProvider {
    fn id(&self) -> &str {
        "mcplayhd"
    }

    fn name(&self) -> &str {
        "McPlayHD"
    }

    fn fetch_data(&self, gameid: u64) -> Result<ApiData, MinesweeperError> {
        let api_key = get_api_key();
        if String::is_empty(&api_key) {
            return Err(MinesweeperError::ApiKeyNotFound);
        }

        let request_data = ureq::get(
            format!("https://mcplayhd.net/api/v1/minesweeper/stats/game/{gameid}").as_ref(),
        )
        .set("Authorization", format!("Bearer {api_key}").as_str())
        .call()
        .map_err(|_| MinesweeperError::GameDataNotFound)?
        .into_string()
        .map_err(|_| MinesweeperError::GameDataNotFound)?;

        serde_json::from_str(request_data.as_ref()).map_err(|_| MinesweeperError::ApiDataParse)
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
