use enumflags2::{BitFlag, BitFlags};
use crate::cards::{AnnotatedCard, Card, Rank, Suit};

pub trait Variant {
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn starting_deck(&self) -> Vec<AnnotatedCard>;
    fn legal_suit_clues() -> BitFlags<Suit>;
    fn legal_rank_clues() -> BitFlags<Rank>;
}

pub struct NoVariant;
impl Variant for NoVariant {
    fn name() -> &'static str {
        "No Variant (5 Suits)"
    }

    fn description() -> &'static str {
        "The original Hanabi rules."
    }

    fn starting_deck(&self) -> Vec<AnnotatedCard> {
        let mut deck = Vec::new();
        for suit in [Suit::Red, Suit::Yellow, Suit::Green, Suit::Blue, Suit::Purple] {
            for rank in [Rank::One, Rank::One, Rank::One, Rank::Two, Rank::Two, Rank::Three, Rank::Three, Rank::Four, Rank::Four, Rank::Five] {
                deck.push(AnnotatedCard::new(Card::new(suit, rank)));
            }
        }
        deck
    }

    fn legal_suit_clues() -> BitFlags<Suit> {
        BitFlags::from(Suit::Red) | BitFlags::from(Suit::Yellow) | BitFlags::from(Suit::Green) | BitFlags::from(Suit::Blue) | BitFlags::from(Suit::Purple)
    }

    fn legal_rank_clues() -> BitFlags<Rank> {
        Rank::all()
    }
}
impl Default for NoVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl NoVariant {
    pub fn new() -> Self {
        Self
    }
}

pub struct Rainbow6Variant;
impl Variant for Rainbow6Variant {
    fn name() -> &'static str {
        "Rainbow (6 Suits)"
    }

    fn description() -> &'static str {
        "A 'rainbow' suit is added, which is clued by all color clues."
    }

    fn starting_deck(&self) -> Vec<AnnotatedCard> {
        let mut deck = Vec::new();
        for suit in [Suit::Red, Suit::Yellow, Suit::Green, Suit::Blue, Suit::Purple, Suit::Rainbow] {
            for rank in [Rank::One, Rank::One, Rank::One, Rank::Two, Rank::Two, Rank::Three, Rank::Three, Rank::Four, Rank::Four, Rank::Five] {
                deck.push(AnnotatedCard::new(Card::new(suit, rank)));
            }
        }
        deck
    }

    fn legal_suit_clues() -> BitFlags<Suit> {
        BitFlags::from(Suit::Red) | BitFlags::from(Suit::Yellow) | BitFlags::from(Suit::Green) | BitFlags::from(Suit::Blue) | BitFlags::from(Suit::Purple)
    }

    fn legal_rank_clues() -> BitFlags<Rank> {
        Rank::all()
    }
}
impl Default for Rainbow6Variant {
    fn default() -> Self {
        Self::new()
    }
}

impl Rainbow6Variant {
    pub fn new() -> Self {
        Self
    }
}
