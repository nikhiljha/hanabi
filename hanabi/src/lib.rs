use std::collections::HashMap;
use std::fmt::Display;
use enumflags2::BitFlag;
use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use cards::{AnnotatedCard, Card};
use crate::cards::{Rank, Suit};

#[cfg(test)]
mod tests;
pub mod constants;
pub mod cards;

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
    Suit(Suit),
    Rank(Rank),
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum GameState {
    InProgress,
    Ended,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HanabiGame {
    deck: Vec<AnnotatedCard>,
    players: Vec<Player>,
    discard_pile: Vec<Card>,
    clues: usize,
    bombs_left: usize,
    current_player: usize,
    game_actions: Vec<AnnotatedAction>,
    stacks: HashMap<Suit, Vec<Card>>,
    config: GameConfig,
    state: GameState,
    turns_remaining: Option<usize>,
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
    #[error("You can't act if the game is over.")]
    GameOver,
}

impl HanabiGame {
    /// Create a new game of Hanabi with the given players.
    pub fn new(mut players: Vec<Player>) -> Self {
        let mut deck = Vec::new();
        for suit in [Suit::Red, Suit::Yellow, Suit::Green, Suit::Blue, Suit::Purple] {
            for rank in [Rank::One, Rank::One, Rank::One, Rank::Two, Rank::Two, Rank::Three, Rank::Three, Rank::Four, Rank::Four, Rank::Five] {
                deck.push(AnnotatedCard::new(Card::new(suit, rank)));
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
            bombs_left: config.max_bombs,
            current_player: 0,
            game_actions: vec![],
            stacks: HashMap::new(),
            config,
            state: GameState::InProgress,
            turns_remaining: None,
        }
    }

    /// Take a turn in the game, depending on the player's chosen action.
    pub fn act(&mut self, action: AnnotatedAction) -> Result<(), ActError> {
        if self.state == GameState::Ended {
            return Err(ActError::GameOver);
        }

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
                self.draw_card(self.current_player);
            },
            Action::Discard(card) => {
                self.discard_card(card);
                self.draw_card(self.current_player);
            },
        }

        self.game_actions.push(action);
        self.increment_player();

        if let Some(turns_remaining) = self.turns_remaining {
            if turns_remaining == 0 {
                self.state = GameState::Ended;
            } else {
                self.turns_remaining = Some(turns_remaining - 1);
            }
        }

        Ok(())
    }

    /// Internal function to play a card from the current player's hand.
    /// Should only be called from the `act` function.
    fn play_card(&mut self, card: usize) {
        let player = &mut self.players[self.current_player];
        let annotated_card = player.hand.remove(card);
        let card = annotated_card.card;
        let stack = self.stacks.entry(card.suit()).or_insert(Vec::new());

        let top = stack.last().map(|card| card.rank() as usize).unwrap_or(0);
        if top + 1 == card.rank() as usize {
            stack.push(card);
            if card.rank() == Rank::Five {
                self.clues += 1;
            }
        } else {
            self.bomb();
            self.discard_pile.push(card);
        }
    }

    /// Internal function to discard a card from the current player's hand.
    /// Should only be called from the [HanabiGame::act] function.
    fn discard_card(&mut self, card: usize) {
        let player = &mut self.players[self.current_player];
        let annotated_card = player.hand.remove(card);
        self.clues += 1;
        self.discard_pile.push(annotated_card.card);
    }

    /// Internal function to draw a new card from the deck.
    /// Should only be called from the [HanabiGame::act] function.
    fn draw_card(&mut self, player: usize) {
        if self.deck.is_empty() {
            if let Some(turns_remaining) = self.turns_remaining {
                self.turns_remaining = Some(turns_remaining - 1);
            } else {
                self.turns_remaining = Some(self.players.len());
            }
            return;
        }

        let card = self.deck.pop().unwrap();
        self.players[player].hand.insert(0, card);
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

    /// Reduce the number of bombs remaining.
    fn bomb(&mut self) {
        self.bombs_left -= 1;
        if self.bombs_left == 0 {
            self.state = GameState::Ended;
        }
    }

    /// Get the number of bombs remaining.
    pub fn bombs_left(&self) -> usize {
        self.bombs_left
    }

    /// Get a player's hand.
    pub fn hand(&self, player: usize) -> &[AnnotatedCard] {
        &self.players[player].hand
    }

    /// Get the current score.
    pub fn score(&self) -> usize {
        if self.bombs_left == 0 {
            return 0;
        }
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

    /// Get the stacks
    pub fn stacks(&self) -> &HashMap<Suit, Vec<Card>> {
        &self.stacks
    }
}
