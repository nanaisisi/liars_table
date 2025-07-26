use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Ace,
    Queen,
    King,
    Joker,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardType::Ace => write!(f, "ACE"),
            CardType::Queen => write!(f, "QUEEN"),
            CardType::King => write!(f, "KING"),
            CardType::Joker => write!(f, "JOKER"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub card_type: CardType,
    pub id: u8, // 1-4 for each type
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self.id {
            1 => "♠",
            2 => "♥", 
            3 => "♦",
            4 => "♣",
            _ => "?",
        };
        
        match self.card_type {
            CardType::Ace => write!(f, "A{}", suit),
            CardType::Queen => write!(f, "Q{}", suit),
            CardType::King => write!(f, "K{}", suit),
            CardType::Joker => write!(f, "Joker"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        
        // Add 4 cards of each type
        for card_type in [CardType::Ace, CardType::Queen, CardType::King, CardType::Joker] {
            for id in 1..=4 {
                cards.push(Card { card_type, id });
            }
        }
        
        Self { cards }
    }
    
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::rngs::OsRng;
        
        self.cards.shuffle(&mut OsRng);
    }
    
    pub fn deal(&mut self, count: usize) -> Vec<Card> {
        self.cards.drain(0..count.min(self.cards.len())).collect()
    }
    
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 16); // 4 types × 4 cards each
        
        // Count each type
        let ace_count = deck.cards.iter().filter(|c| c.card_type == CardType::Ace).count();
        let queen_count = deck.cards.iter().filter(|c| c.card_type == CardType::Queen).count();
        let king_count = deck.cards.iter().filter(|c| c.card_type == CardType::King).count();
        let joker_count = deck.cards.iter().filter(|c| c.card_type == CardType::Joker).count();
        
        assert_eq!(ace_count, 4);
        assert_eq!(queen_count, 4);
        assert_eq!(king_count, 4);
        assert_eq!(joker_count, 4);
    }
    
    #[test]
    fn test_card_display() {
        let ace = Card { card_type: CardType::Ace, id: 1 };
        assert_eq!(format!("{}", ace), "A♠");
        
        let joker = Card { card_type: CardType::Joker, id: 1 };
        assert_eq!(format!("{}", joker), "Joker");
    }
    
    #[test]
    fn test_deal() {
        let mut deck = Deck::new();
        let original_count = deck.cards.len();
        
        let dealt = deck.deal(5);
        assert_eq!(dealt.len(), 5);
        assert_eq!(deck.cards.len(), original_count - 5);
    }
}
