use crate::minesweeper::error::MinesweeperError;
use crate::minesweeper::provider::provider::{ApiData, PlayerData, Provider};

pub struct GreevProvider;

impl Provider for GreevProvider {
    fn id(&self) -> &str {
        "greev"
    }

    fn name(&self) -> &str {
        "Greev"
    }

    fn fetch_data(&self, gameid: &str) -> Result<ApiData, MinesweeperError> {
        let request_data =
            ureq::get(format!("http://api.greev.eu/v2/stats/minesweeper/game/{gameid}").as_ref())
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
