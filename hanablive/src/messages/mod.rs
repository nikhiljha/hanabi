use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use thiserror::Error;
use tracing::Span;
use commands::{CommandData, CommandHistoryGetData, CommandHistoryGetSeedData, CommandInactiveData, CommandNoteData, CommandReplayCreateData, CommandRestartData, CommandSettingData, CommandTableCreateData, CommandTableSpectateData, CommandTableStartData, CommandVotesData};
use notifications::{ChatData, ChatListData, ChatPMData, GameHistory, GameMessage, SpectatorsMessage, TableData, UserData, UserId, WelcomeMessage};
use crate::messages::notifications::{Action, ActionWithTableID, BootMessage, CardIdentitiesMessage, ClockMessage, ConnectedMessage, FinishOngoingGameMessage, GameActionListMessage, GameActionMessage, InitMessage, NoteListMessage, NoteListPlayerMessage, PauseMessage, ReplayLeaderMessage, ReplaySegmentMessage, SuggestionMessage, TableProgressMessage, UserInactiveMessage, VoteMessage};

pub mod commands;
pub mod notifications;

#[derive(Error, Debug)]
pub enum WebsocketParseError {
    #[error("message type not found")]
    NoMessageType,
    #[error("json error")]
    JSONError(#[from] serde_json::Error),
    #[error("unknown message type `{0}`")]
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "tag", content = "data", rename_all = "camelCase")]
pub enum Message {
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
    Game(GameMessage),
    Joined(CommandData),
    PregameSpectators(SpectatorsMessage),
    Spectators(SpectatorsMessage),
    Init(InitMessage),
    GameActionList(GameActionListMessage),
    Connected(ConnectedMessage),
    Clock(ClockMessage),
    NoteList(NoteListMessage),
    Suggestion(SuggestionMessage),
    ReplayLeader(ReplayLeaderMessage),
    CardIdentities(CardIdentitiesMessage),
    VoteChange(VoteMessage),
    Boot(BootMessage),
    NoteListPlayer(NoteListPlayerMessage),
    GameAction(GameActionMessage),
    TableProgress(TableProgressMessage),
    TableGone(CommandData),
    FinishOngoingGame(FinishOngoingGameMessage),
    ReplaySegment(ReplaySegmentMessage),
    UserInactive(UserInactiveMessage),

    // Table commands
    TableCreate(CommandTableCreateData),
    TableJoin(CommandData),
    TableLeave(CommandData),
    TableUnattend(CommandData),
    TableReattend(CommandData),
    TableSetVariant(CommandSettingData),
    TableSetLeader(CommandData),
    TableStart(CommandTableStartData),
    TableTerminate(CommandData),
    TableVoteForTermination(CommandVotesData),
    TableSpectate(CommandTableSpectateData),
    TableRestart(CommandRestartData),
    TableUpdate(CommandData),
    TableSuggest(CommandData),

    // Other lobby commands
    Setting(CommandSettingData),
    // Chat(CommandChatData),
    ChatRead(CommandData),
    ChatTyping(CommandData),
    ChatFriend(CommandData),
    ChatUnfriend(CommandData),
    ChatLink(CommandData),
    ChatUnlink(CommandData),
    ChatLinked(CommandData),
    ChatPlayerInfo(CommandData),
    GetName(CommandData),
    Inactive(CommandInactiveData),
    HistoryGet(CommandHistoryGetData),
    HistoryGetSeed(CommandHistoryGetSeedData),
    HistoryFriendsGet(CommandHistoryGetData),
    ReplayCreate(CommandReplayCreateData),
    TagSearch(CommandData),

    // Game and replay commands
    GetGameInfo1(CommandData),
    GetGameInfo2(CommandData),
    Loaded(CommandData),
    Tag(CommandData),
    TagDelete(CommandData),
    TagsDeleteAll(CommandData),

    // Game commands
    Action(ActionWithTableID),
    Note(CommandNoteData),
    Pause(PauseMessage),

    // Replay commands
    ReplayAction(CommandData),
}

fn to_json_error(e: serde_json::Error) -> WebsocketParseError {
    WebsocketParseError::JSONError(e)
}

impl Message {
    #[tracing::instrument(err)]
    pub fn from_websocket_message(
        message: &str,
    ) -> Result<Message, WebsocketParseError> {
        // Parse the message
        let parts: Vec<&str> = message.splitn(2, ' ').collect();
        let message_type = *parts.first().ok_or(WebsocketParseError::NoMessageType)?;
        Span::current().record("message_type", &tracing::field::display(message_type));
        let message_data = parts
            .get(1)
            // TODO: This silently loses information about whether the message_data was present
            .and_then(|s| serde_json::from_str::<Value>(s).ok());
        Span::current().record("message_data", &tracing::field::debug(&message_data));

        let serde_tagged_message = match message_data {
            Some(data) => json!({
                "tag": message_type,
                "data": data,
            }),
            None => json!({
                "tag": message_type,
                "data": Value::Object(Map::new()),
            }),
        };

        serde_json::from_value(serde_tagged_message).map_err(to_json_error)
    }

    pub fn to_websocket_message(&self) -> Result<String, WebsocketParseError> {
        let serde_tagged_value = serde_json::to_value(self).map_err(to_json_error)?;
        let tag = serde_tagged_value
            .as_object()
            .unwrap()
            .get("tag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let data = serde_tagged_value
            .as_object()
            .unwrap()
            .get("data")
            .map(|v| v.to_string());

        match data {
            Some(data) => Ok(format!("{} {}", tag, data)),
            None => Ok(tag),
        }
    }
}
