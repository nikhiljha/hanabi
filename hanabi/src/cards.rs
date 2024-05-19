use std::fmt::Display;
use serde::{Deserialize, Serialize};
use enumflags2::{BitFlag, BitFlags, bitflags};
use crate::Clue;

#[bitflags(default = Red | Yellow | Green | Blue | Purple)]
#[repr(u16)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Suit {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
    Rainbow,
    Black,
    Pink,
}

impl TryFrom<&str> for Suit {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value.to_ascii_lowercase().as_str() {
            "r" => Ok(Suit::Red),
            "y" => Ok(Suit::Yellow),
            "g" => Ok(Suit::Green),
            "b" => Ok(Suit::Blue),
            "p" => Ok(Suit::Purple),
            "m" => Ok(Suit::Rainbow),
            "k" => Ok(Suit::Black),
            "i" => Ok(Suit::Pink),
            _ => Err(()),
        }
    }
}

impl Into<String> for Suit {
    fn into(self) -> String {
        match self {
            Suit::Red => "r".to_string(),
            Suit::Yellow => "y".to_string(),
            Suit::Green => "g".to_string(),
            Suit::Blue => "b".to_string(),
            Suit::Purple => "p".to_string(),
            Suit::Rainbow => "m".to_string(),
            Suit::Black => "k".to_string(),
            Suit::Pink => "i".to_string(),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

#[bitflags(default = One | Two | Three | Four | Five)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl TryFrom<&str> for Rank {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "1" => Ok(Rank::One),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            _ => Err(()),
        }
    }
}

impl Into<usize> for Rank {
    fn into(self) -> usize {
        match self {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        usize::fmt(&(*self).into(), f)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl Card {
    pub fn new(color: Suit, value: Rank) -> Self {
        Self { suit: color, rank: value }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnotatedCard {
    pub(crate) card: Card,
    clues: Vec<Clue>,
    possible_colors: BitFlags<Suit>,
    possible_values: BitFlags<Rank>,
}

impl AnnotatedCard {
    pub fn new(card: Card) -> Self {
        Self {
            card,
            clues: Vec::new(),
            possible_colors: Suit::all(),
            possible_values: Rank::all(),
        }
    }

    pub fn card(&self) -> Card {
        self.card
    }

    pub fn suit(&self) -> Suit {
        self.card.suit
    }

    pub fn rank(&self) -> Rank {
        self.card.rank
    }

    pub fn clues(&self) -> &[Clue] {
        &self.clues
    }

    fn clue_matches(&self, clue: Clue) -> bool {
        match clue {
            Clue::Suit(color) => match color {
                Suit::Rainbow => true,
                Suit::Black => false,
                _ => self.suit().eq(&color)
            }
            Clue::Rank(value) => match self.card.suit {
                Suit::Pink => true,
                _ => self.rank().eq(&value)
            }
        }
    }

    pub fn add_clue(&mut self, clue: Clue) {
        self.clues.push(clue);
        self.update_from_clue(clue);
    }

    fn update_from_clue(&mut self, clue: Clue) {
        match (clue, self.clue_matches(clue)) {
            (Clue::Suit(color), true) => {
                self.possible_colors &= BitFlags::from(color) | BitFlags::from(Suit::Rainbow);
            },
            (Clue::Suit(color), false) => {
                self.possible_colors &= !(BitFlags::from(color) | BitFlags::from(Suit::Rainbow));
            },
            (Clue::Rank(value), true) => {
                self.possible_values = BitFlags::from(value);
            },
            (Clue::Rank(value), false) => {
                self.possible_values.remove(value);
            },
        }
    }
}
