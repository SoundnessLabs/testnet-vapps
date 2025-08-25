use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn value(&self) -> u8 {
        self.rank as u8
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Jack => "J",
            Rank::Queen => "Q", 
            Rank::King => "K",
            Rank::Ace => "A",
            _ => return write!(f, "{}{}", self.rank as u8, self.suit_symbol()),
        };
        write!(f, "{}{}", rank_str, self.suit_symbol())
    }
}

impl Suit {
    fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦", 
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

impl Card {
    fn suit_symbol(&self) -> &'static str {
        self.suit.symbol()
    }
}

/// Create a full 52-card deck
pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            deck.push(Card::new(suit, rank));
        }
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = create_deck();
        assert_eq!(deck.len(), 52);
        
        // Test uniqueness
        let mut unique_cards = std::collections::HashSet::new();
        for card in &deck {
            assert!(unique_cards.insert(*card));
        }
        assert_eq!(unique_cards.len(), 52);
    }

    #[test]
    fn test_card_display() {
        let ace_spades = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(ace_spades.to_string(), "A♠");
        
        let two_hearts = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(two_hearts.to_string(), "2♥");
    }
}
