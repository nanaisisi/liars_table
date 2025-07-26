use crate::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub hand: Vec<Card>,
    pub is_active: bool,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            name: format!("Player {}", id),
            hand: Vec::new(),
            is_active: true,
        }
    }
    
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.hand.extend(cards);
    }
    
    pub fn remove_cards(&mut self, positions: &[usize]) -> Result<Vec<Card>, String> {
        // Validate positions
        for &pos in positions {
            if pos >= self.hand.len() {
                return Err(format!("Invalid card position: {}. Player has {} cards.", pos + 1, self.hand.len()));
            }
        }
        
        // Sort positions in reverse order to remove from back to front
        let mut sorted_positions = positions.to_vec();
        sorted_positions.sort_unstable();
        sorted_positions.reverse();
        
        let mut removed_cards = Vec::new();
        for &pos in &sorted_positions {
            removed_cards.push(self.hand.remove(pos));
        }
        
        // Reverse to maintain original order
        removed_cards.reverse();
        Ok(removed_cards)
    }
    
    pub fn eliminate(&mut self) {
        self.is_active = false;
    }
    
    pub fn cards_count(&self) -> usize {
        self.hand.len()
    }
    
    pub fn has_won(&self) -> bool {
        self.is_active && self.hand.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, CardType};

    #[test]
    fn test_player_creation() {
        let player = Player::new(1);
        assert_eq!(player.id, 1);
        assert_eq!(player.name, "Player 1");
        assert!(player.hand.is_empty());
        assert!(player.is_active);
    }
    
    #[test]
    fn test_add_cards() {
        let mut player = Player::new(1);
        let cards = vec![
            Card { card_type: CardType::Ace, id: 1 },
            Card { card_type: CardType::Queen, id: 2 },
        ];
        
        player.add_cards(cards);
        assert_eq!(player.hand.len(), 2);
    }
    
    #[test]
    fn test_remove_cards() {
        let mut player = Player::new(1);
        let cards = vec![
            Card { card_type: CardType::Ace, id: 1 },
            Card { card_type: CardType::Queen, id: 2 },
            Card { card_type: CardType::King, id: 3 },
        ];
        player.add_cards(cards);
        
        let removed = player.remove_cards(&[0, 2]).unwrap();
        assert_eq!(removed.len(), 2);
        assert_eq!(removed[0].card_type, CardType::Ace);
        assert_eq!(removed[1].card_type, CardType::King);
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.hand[0].card_type, CardType::Queen);
    }
    
    #[test]
    fn test_remove_invalid_position() {
        let mut player = Player::new(1);
        let cards = vec![Card { card_type: CardType::Ace, id: 1 }];
        player.add_cards(cards);
        
        let result = player.remove_cards(&[1]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_eliminate() {
        let mut player = Player::new(1);
        assert!(player.is_active);
        
        player.eliminate();
        assert!(!player.is_active);
    }
    
    #[test]
    fn test_has_won() {
        let mut player = Player::new(1);
        assert!(!player.has_won()); // Has no cards but this means not dealt yet
        
        let cards = vec![Card { card_type: CardType::Ace, id: 1 }];
        player.add_cards(cards);
        assert!(!player.has_won()); // Has cards
        
        player.remove_cards(&[0]).unwrap();
        assert!(player.has_won()); // No cards and active
        
        player.eliminate();
        assert!(!player.has_won()); // No cards but eliminated
    }
}
