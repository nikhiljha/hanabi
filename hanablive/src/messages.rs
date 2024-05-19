use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use crate::types::EndCondition;

#[derive(Error, Debug)]
pub enum WebsocketParseError {
    #[error("message type not found")]
    NoMessageType,
    #[error("json error")]
    JSONError(#[from] serde_json::Error),
    #[error("unknown message type `{0}`")]
    Unknown(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeMessage {
    #[serde(rename = "userID")]
    pub user_id: u64,
    pub username: String,
    pub total_games: u64,
    pub muted: bool,
    pub first_time_user: bool,
    pub settings: Settings,
    pub friends: Vec<String>,
    pub playing_at_tables: Vec<u64>,
    pub discon_spectating_table: Option<u64>,
    pub discon_shadowing_seat: Option<u64>,
    pub random_table_name: String,
    pub shutting_down: bool,
    pub datetime_shutdown_init: Option<String>,
    pub maintenance_mode: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserData {
    #[serde(rename = "userID")]
    pub user_id: u64,
    pub name: String,
    pub status: u64,
    #[serde(rename = "tableID")]
    pub table_id: Option<u64>,
    pub hyphenated: bool,
    pub inactive: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spectator {
    pub name: String,
    pub shadowing_player_index: Option<i64>,
    pub shadowing_player_username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub desktop_notification: bool,
    pub sound_move: bool,
    pub sound_timer: bool,
    pub keldon_mode: bool,
    pub colorblind_mode: bool,
    pub real_life_mode: bool,
    pub reverse_hands: bool,
    pub style_numbers: bool,
    pub show_timer_in_untimed: bool,
    pub volume: u64,
    pub speedrun_preplay: bool,
    pub speedrun_mode: bool,
    pub hyphenated_conventions: bool,
    pub create_table_variant: String,
    pub create_table_timed: bool,
    pub create_table_time_base_minutes: u64,
    pub create_table_time_per_turn_seconds: u64,
    pub create_table_speedrun: bool,
    pub create_table_card_cycle: bool,
    pub create_table_deck_plays: bool,
    pub create_table_empty_clues: bool,
    pub create_table_one_extra_card: bool,
    pub create_table_one_less_card: bool,
    pub create_table_all_or_nothing: bool,
    pub create_table_detrimental_characters: bool,
    pub create_table_max_players: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableData {
    pub id: u64,
    pub joined: bool,
    pub max_players: u64,
    pub name: String,
    pub num_players: u64,
    pub options: GameOptions,
    pub owned: bool,
    pub password_protected: bool,
    pub players: Vec<String>,
    pub progress: u64,
    pub running: bool,
    pub shared_replay: bool,
    pub spectators: Vec<Spectator>,
    pub time_base: u64,
    pub time_per_turn: u64,
    pub timed: bool,
    pub variant: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatData {
    pub msg: String,
    pub who: Option<String>,
    pub discord: bool,
    pub server: bool,
    pub datetime: String,
    pub room: Option<String>,
    pub recipient: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatListData {
    pub list: Vec<ChatData>,
    pub unread: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameHistory {
    pub id: u64,
    pub options: GameOptions,
    pub seed: String,
    pub score: u64,
    pub num_turns: u64,
    pub end_condition: EndCondition,
    pub datetime_started: chrono::DateTime<chrono::Utc>,
    pub datetime_finished: chrono::DateTime<chrono::Utc>,
    pub num_games_on_this_seed: u64,
    pub player_names: Vec<String>,
    pub increment_num_games: bool,
    pub tags: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserId {
    #[serde(rename = "userID")]
    pub user_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    pub num_players: u64,
    pub starting_player: u64,
    pub variant_name: String,
    pub timed: bool,
    pub time_base: u64,
    pub time_per_turn: u64,
    pub speedrun: bool,
    pub card_cycle: bool,
    pub deck_plays: bool,
    pub empty_clues: bool,
    pub one_extra_card: bool,
    pub one_less_card: bool,
    pub all_or_nothing: bool,
    pub detrimental_characters: bool,
    pub table_name: Option<String>,
    pub max_players: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatPMData {
    pub msg: String,
    pub recipient: String,
    pub room: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "tag", content = "data", rename_all = "camelCase")]
pub enum Message {
    // Server -> Client
    Welcome(WelcomeMessage),
    User(UserData),
    UserLeft(UserId),
    UserList(Vec<UserData>),
    Table(TableData),
    TableList(Vec<TableData>),
    Chat(ChatData),
    #[serde(rename = "chatPM")]
    ChatPM(ChatPMData),
    ChatList(ChatListData),
    GameHistory(Vec<GameHistory>),
    GameHistoryFriends(Vec<GameHistory>),
}

fn to_json_error(e: serde_json::Error) -> WebsocketParseError {
    WebsocketParseError::JSONError(e)
}

impl Message {
    pub fn from_websocket_message(message: &str) -> Result<Message, WebsocketParseError> {
        // Parse the message
        let parts: Vec<&str> = message.splitn(2, ' ').collect();
        let message_type = *parts.get(0)
            .ok_or(WebsocketParseError::NoMessageType)?;
        let message_data = parts.get(1)
            // TODO: This silently loses information about whether the message_data was present
            .map(|s| serde_json::from_str::<Value>(s).ok())
            .flatten();

        let serde_tagged_message = match message_data {
            Some(data) => json!({
                "tag": message_type,
                "data": data,
            }),
            None => json!({
                "tag": message_type,
            }),
        };

        serde_json::from_value(serde_tagged_message)
            .map_err(to_json_error)
    }

    pub fn to_websocket_message(&self) -> Result<String, WebsocketParseError> {
        let serde_tagged_value = serde_json::to_value(self).map_err(to_json_error)?;
        let tag = serde_tagged_value.
            as_object().unwrap()
            .get("tag").unwrap()
            .as_str().unwrap()
            .to_string();
        let data = serde_tagged_value
            .as_object().unwrap()
            .get("data")
            .map(|v| v.to_string());

        match data {
            Some(data) => Ok(format!("{} {}", tag, data)),
            None => Ok(tag),
        }
    }
}
