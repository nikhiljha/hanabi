use serde::{Deserialize, Serialize};
use crate::messages::notifications::GameOptions;
use crate::types::Game;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandData {
    #[serde(rename = "tableID")]
    pub table_id: Option<u64>,
    pub database_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandSettingData {
    pub setting: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandChatData {
    pub msg: String,
    pub room: String,
    pub recipient: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandTableCreateData {
    pub name: String,
    pub options: GameOptions,
    pub password: String,
    pub max_players: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandTableStartData {
    #[serde(default)]
    pub intended_players: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandVotesData {
    #[serde(default)]
    pub votes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandNoteData {
    pub note: String,
    pub order: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandTableSpectateData {
    pub shadowing_player_index: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandReplayCreateData {
    pub source: String,
    #[serde(rename = "gameJSON")]
    pub game_json: Game,
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandSharedReplayData {
    pub segment: i64,
    pub rank: i64,
    pub suit: i64,
    pub sound: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandHistoryGetData {
    pub offset: i64,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandHistoryGetSeedData {
    pub seed: String,
    pub friends: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandHypoActionData {
    #[serde(rename = "actionJSON")]
    pub action_json: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandInactiveData {
    pub inactive: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandRestartData {
    pub hide_pregame: bool,
}
