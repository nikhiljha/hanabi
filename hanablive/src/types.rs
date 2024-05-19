use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// This is a JSON object that represents a game of Hanabi. This also matches how games are stored
/// in the database for the "Hanab Live" website.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Database ID of the game on the "Hanab Live" website.
    pub id: Option<u64>,

    /// String corresponding to the seed of the game on the "Hanab Live" website.
    pub seed: Option<String>,

    /// Contains the names of the players. The 0th player will always go first.
    pub players: Vec<String>,

    /// Contains all the cards in the deck. Lists the cards from top to bottom. Cards are NOT dealt
    /// to the players like they conventionally would be in real life. Instead, cards are dealt to
    /// the first player until they reach the maximum number of cards, then cards are dealt to the
    /// second player until they reach the maximum number of cards, and so forth.
    pub deck: Vec<Card>,

    /// Contains the actions that the players performed in the game.
    pub actions: Vec<Action>,

    /// The options for the game.
    pub options: Option<Options>,

    /// Contains the notes for each player. Players can write arbitrary notes on individual cards.
    /// The 0th array corresponds to the notes for the first player, and so on. The 0th note in a
    /// note array corresponds to the 1st card dealt, and so on.
    pub notes: Option<Vec<Vec<String>>>,

    /// Array of character objects, containing the "Detrimental Character" specification for each
    /// player. "Detrimental Characters" is an optional setting used on the website that is based on
    /// a post from Sean McCarthy: <https://boardgamegeek.com/thread/1688194/hanabi-characters-variant>.
    pub characters: Option<Vec<Character>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The suit number corresponds to the suits for the particular variant, from 0 to N. For
    /// example, 0 corresponds to red, 1 corresponds to yellow, and so on in a "No Variant" game.
    suit_index: u64,
    rank: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Play {
        /// The index of the card that is being played (e.g. 0 corresponds to the 1st card dealt).
        target: u64,
    },
    Discard {
        /// The index of the card that is being discarded (e.g. 0 corresponds to the 1st card dealt).
        target: u64,
    },
    ColorClue {
        /// The index of the player that is being clued (e.g. 0 corresponds to the 1st player).
        target: u64,

        /// The value for a color clue is the index of the possible colors from 1 to N, from left to
        /// right (e.g. 0 corresponds to red, 1 corresponds to yellow, and so on in a "No Variant"
        /// game).
        value: u64,
    },
    RankClue {
        /// The index of the player that is being clued (e.g. 0 corresponds to the 1st player).
        target: u64,

        /// The value for a rank clue is equal to the rank chosen for the clue (e.g. 1 corresponds
        /// to rank 1, 2 corresponds to rank 2, etc.).
        value: u64,
    },
    EndGame {
        /// The index of the player that is ending the game (e.g. 0 corresponds to the 1st player).
        target: u64,

        /// The reason for ending the game.
        value: EndCondition,
    },
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        #[derive(Serialize)]
        #[serde(rename = "Block")]
        struct AdjTagged {
            r#type: u64,
            target: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<u64>,
        }

        match *self {
            Action::Play { target } => AdjTagged { r#type: 0, target, value: None }.serialize(serializer),
            Action::Discard { target } => AdjTagged { r#type: 1, target, value: None }.serialize(serializer),
            Action::ColorClue { target, value } => AdjTagged { r#type: 2, target, value: Some(value) }.serialize(serializer),
            Action::RankClue { target, value } => AdjTagged { r#type: 3, target, value: Some(value) }.serialize(serializer),
            Action::EndGame { target, value } => AdjTagged { r#type: 4, target, value: Some(value as u64) }.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(rename = "Block")]
        struct AdjTagged {
            r#type: u64,
            target: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<u64>,
        }

        let adj_tagged = AdjTagged::deserialize(deserializer)?;
        match adj_tagged.r#type {
            0 => Ok(Action::Play { target: adj_tagged.target }),
            1 => Ok(Action::Discard { target: adj_tagged.target }),
            2 => Ok(Action::ColorClue { target: adj_tagged.target, value: adj_tagged.value.unwrap() }),
            3 => Ok(Action::RankClue { target: adj_tagged.target, value: adj_tagged.value.unwrap() }),
            4 => Ok(Action::EndGame { target: adj_tagged.target, value: EndCondition::from_u64(adj_tagged.value.unwrap()).unwrap() }),
            _ => Err(serde::de::Error::custom("invalid type")),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
/// <https://github.com/Hanabi-Live/hanabi-live/blob/main/packages/game/src/enums/EndCondition.ts>
pub enum EndCondition {
    InProgress = 0,
    Normal = 1,
    Strikeout = 2,
    Timeout = 3,
    TerminatedByPlayer = 4,
    SpeedrunFail = 5,
    IdleTimeout = 6,
    CharacterSoftlock = 7,
    AllOrNothingFail = 8,
    AllOrNothingSoftlock = 9,
    TerminatedByVote = 10,
}

impl EndCondition {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(EndCondition::InProgress),
            1 => Some(EndCondition::Normal),
            2 => Some(EndCondition::Strikeout),
            3 => Some(EndCondition::Timeout),
            4 => Some(EndCondition::TerminatedByPlayer),
            5 => Some(EndCondition::SpeedrunFail),
            6 => Some(EndCondition::IdleTimeout),
            7 => Some(EndCondition::CharacterSoftlock),
            8 => Some(EndCondition::AllOrNothingFail),
            9 => Some(EndCondition::AllOrNothingSoftlock),
            10 => Some(EndCondition::TerminatedByVote),
            _ => None,
        }
    }
}

fn default_variant() -> String {
    "No Variant".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(default = "default_variant")]
    pub variant: String,

    #[serde(default)]
    pub speedrun: bool,
    #[serde(default)]
    pub card_cycle: bool,
    #[serde(default)]
    pub deck_plays: bool,
    #[serde(default)]
    pub empty_clues: bool,
    #[serde(default)]
    pub one_extra_card: bool,
    #[serde(default)]
    pub one_less_card: bool,
    #[serde(default)]
    pub all_or_nothing: bool,
    #[serde(default)]
    pub detrimental_characters: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
    pub metadata: serde_json::Value,
}
