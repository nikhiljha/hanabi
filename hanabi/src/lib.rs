use std::collections::HashMap;
use enumflags2::{BitFlag, bitflags, BitFlags};
use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
mod tests;
mod constants;

#[bitflags(default = Red | Yellow | Green | Blue | Purple)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value.to_ascii_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "yellow" => Ok(Color::Yellow),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            "purple" => Ok(Color::Purple),
            _ => Err(()),
        }
    }
}

#[bitflags(default = One | Two | Three | Four | Five)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl TryFrom<&str> for Value {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "1" => Ok(Value::One),
            "2" => Ok(Value::Two),
            "3" => Ok(Value::Three),
            "4" => Ok(Value::Four),
            "5" => Ok(Value::Five),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Card {
    color: Color,
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnotatedCard {
    card: Card,
    clues: Vec<Clue>,
    possible_colors: BitFlags<Color>,
    possible_values: BitFlags<Value>,
}

impl AnnotatedCard {
    pub fn new(card: Card) -> Self {
        Self {
            card,
            clues: Vec::new(),
            possible_colors: Color::all(),
            possible_values: Value::all(),
        }
    }

    pub fn color(&self) -> Color {
        self.card.color
    }

    pub fn value(&self) -> Value {
        self.card.value
    }

    pub fn clues(&self) -> &[Clue] {
        &self.clues
    }

    fn clue_matches(&self, clue: Clue) -> bool {
        match clue {
            Clue::Color(color) => self.color().eq(&color),
            Clue::Value(value) => self.value().eq(&value),
        }
    }

    pub fn add_clue(&mut self, clue: Clue) {
        self.clues.push(clue);
        self.update_from_clue(clue);
    }

    fn update_from_clue(&mut self, clue: Clue) {
        match (clue, self.clue_matches(clue)) {
            (Clue::Color(color), true) => {
                self.possible_colors = BitFlags::from(color);
            },
            (Clue::Color(color), false) => {
                self.possible_colors.remove(color);
            },
            (Clue::Value(value), true) => {
                self.possible_values = BitFlags::from(value);
            },
            (Clue::Value(value), false) => {
                self.possible_values.remove(value);
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub hand: Vec<AnnotatedCard>,
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            hand: Vec::new(),
            name,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Clue {
    Color(Color),
    Value(Value),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Action {
    Clue {
        clue: Clue,
        target: usize,
    },
    Play(usize),
    Discard(usize),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct AnnotatedAction {
    pub player: usize,
    pub action: Action,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct GameConfig {
    pub max_clues: usize,
    pub max_bombs: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            max_clues: 8,
            max_bombs: 3,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HanabiGame {
    deck: Vec<AnnotatedCard>,
    players: Vec<Player>,
    discard_pile: Vec<Card>,
    clues: usize,
    bombs: usize,
    current_player: usize,
    game_actions: Vec<AnnotatedAction>,
    stacks: HashMap<Color, Vec<Card>>,
    config: GameConfig,
}

#[derive(Error, Debug, PartialEq)]
pub enum ActError {
    #[error("It is not your turn.")]
    NotYourTurn,
    #[error("You do not have enough clues.")]
    NotEnoughClues,
    #[error("You can't clue yourself.")]
    CantClueYourself,
    #[error("You can't clue if it doesn't match any cards.")]
    ClueDoesNotMatch,
}

impl HanabiGame {
    /// Create a new game of Hanabi with the given players.
    pub fn new(mut players: Vec<Player>) -> Self {
        let mut deck = Vec::new();
        for color in [Color::Red, Color::Yellow, Color::Green, Color::Blue, Color::Purple] {
            for value in [Value::One, Value::One, Value::One, Value::Two, Value::Two, Value::Three, Value::Three, Value::Four, Value::Four, Value::Five] {
                deck.push(AnnotatedCard::new(Card {
                    color,
                    value,
                }));
            }
        }

        deck.shuffle(&mut thread_rng());

        let num_players = players.len();
        for player in players.iter_mut() {
            while player.hand.len() < constants::CARDS_PER_HAND[num_players] {
                if let Some(card) = deck.pop() {
                    player.hand.push(card);
                }
            }
        }

        let config = GameConfig::default();

        Self {
            deck,
            players,
            discard_pile: Vec::new(),
            clues: config.max_clues,
            bombs: config.max_bombs,
            current_player: 0,
            game_actions: vec![],
            stacks: HashMap::new(),
            config,
        }
    }

    /// Take a turn in the game, depending on the player's chosen action.
    pub fn act(&mut self, action: AnnotatedAction) -> Result<(), ActError> {
        if self.current_player != action.player {
            return Err(ActError::NotYourTurn);
        }

        match action.action {
            Action::Clue { clue, target } => {
                if self.clues == 0 {
                    return Err(ActError::NotEnoughClues);
                }
                if target == action.player {
                    return Err(ActError::CantClueYourself);
                }
                self.give_clue(clue, target);
            },
            Action::Play(card) => {
                self.play_card(card);
            },
            Action::Discard(card) => {
                self.discard_card(card);
            },
        }

        self.game_actions.push(action);
        self.increment_player();
        Ok(())
    }

    /// Internal function to play a card from the current player's hand.
    /// Should only be called from the `act` function.
    fn play_card(&mut self, card: usize) {
        let player = &mut self.players[self.current_player];
        let annotated_card = player.hand.remove(card);
        let card = annotated_card.card;
        let stack = self.stacks.entry(card.color).or_insert(Vec::new());
        if stack.is_empty() {
            if card.value == Value::One {
                self.clues += 1;
            } else {
                self.bombs += 1;
            }
        } else if stack.last().unwrap().value as i32 + 1 != card.value as i32 {
            self.bombs += 1;
        }
        stack.push(card);
    }

    /// Internal function to discard a card from the current player's hand.
    /// Should only be called from the [HanabiGame::act] function.
    fn discard_card(&mut self, card: usize) {
        let player = &mut self.players[self.current_player];
        let annotated_card = player.hand.remove(card);
        self.clues += 1;
        self.discard_pile.push(annotated_card.card);
    }

    /// Internal function to give a clue to another player.
    /// Should only be called from the [HanabiGame::act] function.
    fn give_clue(&mut self, clue: Clue, target: usize) {
        self.clues -= 1;
        let player = &mut self.players[target];
        for annotated_card in player.hand.iter_mut() {
            annotated_card.add_clue(clue);
        }
    }

    /// Internal function to increment the current player.
    /// Should only be called from the [HanabiGame::act] function.
    fn increment_player(&mut self) {
        if self.current_player == self.players.len() - 1 {
            self.current_player = 0;
        } else {
            self.current_player += 1;
        }
    }

    /// Get the number of clues remaining.
    pub fn clues(&self) -> usize {
        self.clues
    }

    /// Get the number of bombs remaining.
    pub fn bombs(&self) -> usize {
        self.bombs
    }

    /// Get a player's hand.
    pub fn hand(&self, player: usize) -> &[AnnotatedCard] {
        &self.players[player].hand
    }

    /// Get the current score.
    pub fn score(&self) -> usize {
        self.stacks.iter().map(|(_, stack)| stack.len()).sum()
    }

    /// Get the current player.
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    /// Get the players.
    pub fn players(&self) -> &[Player] {
        &self.players
    }
}
