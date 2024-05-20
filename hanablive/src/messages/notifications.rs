use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::{EndCondition};

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
    pub num_players: Option<u64>,
    pub starting_player: Option<u64>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BestScore {
    pub num_players: i64,
    pub score: i64,
    pub modifier: i64,
    pub deck_plays: bool,
    pub empty_clues: bool,
    pub one_extra_card: bool,
    pub one_less_card: bool,
    pub all_or_nothing: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsRow {
    pub num_games: i64,
    pub best_scores: Vec<BestScore>,
    pub average_score: f64,
    pub num_strikeouts: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PregameStats {
    #[serde(rename = "numGames")]
    pub num_games: i64,
    pub variant: UserStatsRow,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayerMessage {
    pub index: i64,
    pub name: String,
    pub you: bool,
    pub present: bool,
    pub stats: Option<PregameStats>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub name: String,
    pub owner: i64,
    pub players: Vec<GamePlayerMessage>,
    pub options: GameOptions,
    #[serde(rename = "passwordProtected")]
    pub password_protected: bool,
    #[serde(rename = "maxPlayers")]
    pub max_players: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorsMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub spectators: Vec<Spectator>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitMessage {
    // Game settings
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub player_names: Vec<String>,
    pub our_player_index: i64,
    pub spectating: bool,
    pub shadowing: bool,
    pub replay: bool,
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    pub has_custom_seed: bool,
    pub seed: String,
    pub datetime_started: chrono::DateTime<chrono::Utc>,
    pub datetime_finished: chrono::DateTime<chrono::Utc>,
    pub options: GameOptions,

    // Character settings
    pub character_assignments: Vec<i64>,
    pub character_metadata: Vec<i64>,

    // Shared replay settings
    pub shared_replay: bool,
    pub shared_replay_leader: String,
    pub shared_replay_segment: i64,
    pub shared_replay_eff_mod: i64,

    // Other features
    pub paused: bool,
    pub pause_player_index: i64,
    pub pause_queued: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameActionListMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub list: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub list: Vec<bool>,
}

// 	type ClockMessage struct {
// 		TableID           uint64  `json:"tableID"`
// 		Times             []int64 `json:"times"`
// 		ActivePlayerIndex int     `json:"activePlayerIndex"`
// 		TimeTaken         int64   `json:"timeTaken"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClockMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub times: Vec<i64>,
    pub active_player_index: i64,
    pub time_taken: i64,
}

// 	type NoteList struct {
// 		Name        string   `json:"name"`
// 		Notes       []string `json:"notes"`
// 		IsSpectator bool     `json:"isSpectator"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteList {
    pub name: String,
    pub notes: Vec<String>,
    pub is_spectator: bool,
}

// 	type NoteListMessage struct {
// 		TableID uint64     `json:"tableID"`
// 		Notes   []NoteList `json:"notes"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteListMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub notes: Vec<NoteList>,
}

// 	type SuggestionMessage struct {
// 		TableID  uint64 `json:"tableID"`
// 		UserName string `json:"userName"`
// 		Segment  int    `json:"segment"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub user_name: String,
    pub segment: i64,
}

// 	type ReplayLeaderMessage struct {
// 		TableID uint64 `json:"tableID"`
// 		Name    string `json:"name"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLeaderMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub name: String,
}

// 	type CardIdentity struct {
// 	SuitIndex int `json:"suitIndex"`
// 	Rank      int `json:"rank"`
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardIdentity {
    pub suit_index: i64,
    pub rank: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardIdentitiesMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub card_identities: Vec<CardIdentity>,
}

// 	type VoteMessage struct {
// 		Vote bool `json:"vote"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VoteMessage {
    pub vote: bool,
}

// 	type BootMessage struct {
// 		TableID uint64
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BootMessage {
    // Yes, the T is capitalized in the JSON
    // This is inconsistent with the other messages
    #[serde(rename = "TableID")]
    pub table_id: u64,
}

// 	type PauseMessage struct {
// 		TableID     uint64 `json:"tableID"`
// 		Active      bool   `json:"active"`
// 		PlayerIndex int    `json:"playerIndex"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PauseMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub active: bool,
    pub player_index: i64,
}

// 			type NoteListPlayerMessage struct {
// 				TableID uint64   `json:"tableID"`
// 				Notes   []string `json:"notes"`
// 			}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteListPlayerMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub notes: Vec<String>,
}

// 	type GameActionMessage struct {
// 		TableID uint64      `json:"tableID"`
// 		Action  interface{} `json:"action"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameActionMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub action: Action,
}

// type ActionCardIdentity struct {
// 	Type        string `json:"type"`
// 	PlayerIndex int    `json:"playerIndex"` // Needed so that we can validate who holds the card
// 	Order       int    `json:"order"`
// 	SuitIndex   int    `json:"suitIndex"`
// 	Rank        int    `json:"rank"`
// }
//
// type ActionClue struct {
// 	Type   string `json:"type"`
// 	Clue   Clue   `json:"clue"`
// 	Giver  int    `json:"giver"`
// 	List   []int  `json:"list"` // The list of cards that the clue "touches"
// 	Target int    `json:"target"`
// 	// The client records the turn that each clue is given (for the clue log)
// 	Turn int `json:"turn"`
// }
//
// type ActionDiscard struct {
// 	Type        string `json:"type"`
// 	PlayerIndex int    `json:"playerIndex"`
// 	Order       int    `json:"order"` // The ID of the card (based on its order in the deck)
// 	SuitIndex   int    `json:"suitIndex"`
// 	Rank        int    `json:"rank"`
// 	Failed      bool   `json:"failed"`
// }
//
// type ActionDraw struct {
// 	Type        string `json:"type"`
// 	PlayerIndex int    `json:"playerIndex"`
// 	Order       int    `json:"order"` // The ID of the card, based on its ordering in the deck
// 	SuitIndex   int    `json:"suitIndex"`
// 	Rank        int    `json:"rank"`
// }
//
// type ActionGameOver struct {
// 	Type         string `json:"type"`
// 	EndCondition int    `json:"endCondition"`
// 	PlayerIndex  int    `json:"playerIndex"`
// 	Votes        []int  `json:"votes"`
// }
//
// type ActionPlay struct {
// 	Type        string `json:"type"`
// 	PlayerIndex int    `json:"playerIndex"`
// 	Order       int    `json:"order"` // The ID of the card (based on its order in the deck)
// 	SuitIndex   int    `json:"suitIndex"`
// 	Rank        int    `json:"rank"`
// }
//
// type ActionPlayerTimes struct {
// 	Type        string  `json:"type"`
// 	PlayerTimes []int64 `json:"playerTimes"`
// 	Duration    int64   `json:"duration"`
// }
//
// type ActionStrike struct {
// 	Type  string `json:"type"`
// 	Num   int    `json:"num"`   // Whether it was the first strike, the second strike, etc.
// 	Turn  int    `json:"turn"`  // The turn that the strike happened
// 	Order int    `json:"order"` // The order of the card that was played
// }
//
// type ActionStatus struct {
// 	Type     string `json:"type"`
// 	Clues    int    `json:"clues"`
// 	Score    int    `json:"score"`
// 	MaxScore int    `json:"maxScore"`
// }
//
// type ActionTurn struct {
// 	Type               string `json:"type"`
// 	Num                int    `json:"num"`
// 	CurrentPlayerIndex int    `json:"currentPlayerIndex"`
// }
//
// type Clue struct {
// 	Type  int `json:"type"`
// 	Value int `json:"value"`
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clue {
    #[serde(rename = "type")]
    clue_type: i64,
    value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    #[serde(rename_all = "camelCase")]
    CardIdentity {
        player_index: i64,
        order: i64,
        suit_index: i64,
        rank: i64,
    },
    #[serde(rename_all = "camelCase")]
    Clue {
        clue: Clue,
        giver: i64,
        list: Vec<i64>,
        target: i64,
        turn: i64,
    },
    #[serde(rename_all = "camelCase")]
    Discard {
        player_index: i64,
        order: i64,
        suit_index: i64,
        rank: i64,
        failed: bool,
    },
    #[serde(rename_all = "camelCase")]
    Draw {
        player_index: i64,
        order: i64,
        suit_index: i64,
        rank: i64,
    },
    #[serde(rename_all = "camelCase")]
    GameOver {
        end_condition: i64,
        player_index: i64,
        // This is null if nobody voted, hence the Option
        votes: Option<Vec<i64>>,
    },
    #[serde(rename_all = "camelCase")]
    Play {
        player_index: i64,
        order: i64,
        suit_index: i64,
        rank: i64,
    },
    #[serde(rename_all = "camelCase")]
    PlayerTimes {
        player_times: Vec<i64>,
        duration: i64,
    },
    #[serde(rename_all = "camelCase")]
    Strike {
        num: i64,
        turn: i64,
        order: i64,
    },
    #[serde(rename_all = "camelCase")]
    Status {
        clues: i64,
        score: i64,
        max_score: i64,
    },
    #[serde(rename_all = "camelCase")]
    Turn {
        num: i64,
        current_player_index: i64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionWithTableID {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    #[serde(flatten)]
    pub action: crate::types::Action,
}

// 	type TableProgressMessage struct {
// 		TableID  uint64 `json:"tableID"`
// 		Progress int    `json:"progress"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableProgressMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub progress: i64,
}

// 	type TableGoneMessage struct {
// 		TableID uint64 `json:"tableID"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableGoneMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
}

// 	type FinishOngoingGameMessage struct {
// 		TableID            uint64 `json:"tableID"`
// 		DatabaseID         int    `json:"databaseID"`
// 		SharedReplayLeader string `json:"sharedReplayLeader"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FinishOngoingGameMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    pub shared_replay_leader: String,
}

// 	type ReplaySegmentMessage struct {
// 		TableID uint64 `json:"tableID"`
// 		Segment int    `json:"segment"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySegmentMessage {
    #[serde(rename = "tableID")]
    pub table_id: u64,
    pub segment: i64,
}

// 	type UserInactiveMessage struct {
// 		UserID   int  `json:"userID"`
// 		Inactive bool `json:"inactive"`
// 	}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInactiveMessage {
    #[serde(rename = "userID")]
    pub user_id: i64,
    pub inactive: bool,
}
