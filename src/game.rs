use crate::card::{Card, CardType, Deck};
use crate::player::Player;
use crate::roulette::{RouletteResult, execute_roulette};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InvalidPlayerCount,
    InvalidBulletCount,
    InvalidCardPosition,
    InvalidCardType,
    PlayerNotFound,
    GameNotInitialized,
    InvalidCommand,
    IoError(String),
    GameAlreadyStarted,
    NotEnoughCards,
    NoLastPlay,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidPlayerCount => write!(f, "Invalid player count. Must be 2-3."),
            GameError::InvalidBulletCount => write!(f, "Invalid bullet count. Must be 1-3."),
            GameError::InvalidCardPosition => write!(f, "Invalid card position."),
            GameError::InvalidCardType => write!(f, "Invalid card type. Use 'ace', 'queen', or 'king'."),
            GameError::PlayerNotFound => write!(f, "Player not found."),
            GameError::GameNotInitialized => write!(f, "Game not initialized. Use 'liars_table init' first."),
            GameError::InvalidCommand => write!(f, "Invalid command."),
            GameError::IoError(msg) => write!(f, "IO Error: {}", msg),
            GameError::GameAlreadyStarted => write!(f, "Game already started."),
            GameError::NotEnoughCards => write!(f, "Not enough cards in deck."),
            GameError::NoLastPlay => write!(f, "No previous play to challenge."),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastPlay {
    pub player_id: u8,
    pub cards: Vec<Card>,
    pub declared_type: CardType,
}

#[derive(Debug, Clone)]
pub struct ChallengeResult {
    pub target_player: u8,
    pub actual_cards: Vec<Card>,
    pub declared_type: CardType,
    pub is_liar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub players: Vec<Player>,
    pub current_player: u8,
    pub roulette_config: RouletteConfig,
    pub deck: Deck,
    pub last_play: Option<LastPlay>,
    pub is_started: bool,
}

impl Game {
    pub fn new(player_count: u8, bullet_count: u8) -> Self {
        let mut players = Vec::new();
        for i in 1..=player_count {
            players.push(Player::new(i));
        }
        
        Self {
            players,
            current_player: 1,
            roulette_config: RouletteConfig::new(bullet_count),
            deck: Deck::new(),
            last_play: None,
            is_started: false,
        }
    }
    
    pub fn deal_cards(&mut self) {
        if self.is_started {
            return; // Already dealt
        }
        
        self.deck.shuffle();
        
        // Deal 5 cards to each player
        for player in &mut self.players {
            if player.is_active {
                let cards = self.deck.deal(5);
                player.add_cards(cards);
            }
        }
        
        self.is_started = true;
    }
    
    pub fn play_cards(&mut self, player_id: u8, card_positions: Vec<usize>, declared_type: CardType) -> Result<(), GameError> {
        if player_id != self.current_player {
            return Err(GameError::InvalidCommand);
        }
        
        let player = self.find_player_mut(player_id)?;
        
        if !player.is_active {
            return Err(GameError::PlayerNotFound);
        }

        let cards = player.remove_cards(&card_positions)
            .map_err(|_| GameError::InvalidCardPosition)?;
        
        // Validate that cards match declaration or are jokers
        let _is_valid = cards.iter().all(|card| {
            card.card_type == declared_type || card.card_type == CardType::Joker
        });
        
        self.last_play = Some(LastPlay {
            player_id,
            cards: cards.clone(),
            declared_type,
        });
        
        // Move to next active player
        self.advance_turn();
        
        Ok(())
    }
    
    pub fn challenge(&mut self, challenger_id: u8) -> Result<ChallengeResult, GameError> {
        let last_play = self.last_play.as_ref()
            .ok_or(GameError::NoLastPlay)?
            .clone();
        
        let challenger = self.find_player(challenger_id)?;
        if !challenger.is_active {
            return Err(GameError::PlayerNotFound);
        }
        
        // Check if the last play was a lie
        let is_liar = !last_play.cards.iter().all(|card| {
            card.card_type == last_play.declared_type || card.card_type == CardType::Joker
        });
        
        let result = ChallengeResult {
            target_player: last_play.player_id,
            actual_cards: last_play.cards,
            declared_type: last_play.declared_type,
            is_liar,
        };
        
        // Clear last play after challenge
        self.last_play = None;
        
        Ok(result)
    }
    
    pub fn execute_roulette(&mut self, target_id: u8) -> Result<RouletteResult, GameError> {
        let roulette_config = self.roulette_config.clone();
        
        let player = self.find_player_mut(target_id)?;
        
        if !player.is_active {
            return Err(GameError::PlayerNotFound);
        }
        
        let engine = RouletteEngine::new(roulette_config);
        let result = engine.spin();
        
        if result.outcome == crate::roulette::RouletteOutcome::Out {
            player.eliminate();
        }
        
        Ok(result)
    }
    
    pub fn get_winner(&self) -> Option<u8> {
        let active_players: Vec<_> = self.players.iter()
            .filter(|p| p.is_active)
            .collect();
        
        if active_players.len() == 1 {
            Some(active_players[0].id)
        } else {
            // Check if any player has won by emptying their hand
            active_players.iter()
                .find(|p| p.has_won())
                .map(|p| p.id)
        }
    }
    
    fn find_player(&self, player_id: u8) -> Result<&Player, GameError> {
        self.players.iter()
            .find(|p| p.id == player_id)
            .ok_or(GameError::PlayerNotFound)
    }
    
    fn find_player_mut(&mut self, player_id: u8) -> Result<&mut Player, GameError> {
        self.players.iter_mut()
            .find(|p| p.id == player_id)
            .ok_or(GameError::PlayerNotFound)
    }
    
    fn advance_turn(&mut self) {
        let active_players: Vec<u8> = self.players.iter()
            .filter(|p| p.is_active)
            .map(|p| p.id)
            .collect();
        
        if active_players.is_empty() {
            return;
        }
        
        let current_index = active_players.iter()
            .position(|&id| id == self.current_player)
            .unwrap_or(0);
        
        let next_index = (current_index + 1) % active_players.len();
        self.current_player = active_players[next_index];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(3, 1);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.current_player, 1);
        assert!(!game.is_started);
    }
    
    #[test]
    fn test_deal_cards() {
        let mut game = Game::new(3, 1);
        game.deal_cards();
        
        assert!(game.is_started);
        for player in &game.players {
            assert_eq!(player.hand.len(), 5);
        }
        assert_eq!(game.deck.remaining(), 1); // 16 - (3 * 5) = 1
    }
    
    #[test]
    fn test_play_cards() {
        let mut game = Game::new(2, 1);
        game.deal_cards();
        
        let result = game.play_cards(1, vec![0, 1], CardType::Ace);
        assert!(result.is_ok());
        assert!(game.last_play.is_some());
        
        let player1 = game.find_player(1).unwrap();
        assert_eq!(player1.hand.len(), 3); // 5 - 2 = 3
    }
    
    #[test]
    fn test_challenge() {
        let mut game = Game::new(2, 1);
        game.deal_cards();
        
        // Manually set up a scenario
        let cards = vec![
            Card { card_type: CardType::Queen, id: 1 },
            Card { card_type: CardType::King, id: 2 },
        ];
        game.last_play = Some(LastPlay {
            player_id: 1,
            cards: cards.clone(),
            declared_type: CardType::Ace, // Lying!
        });
        
        let result = game.challenge(2).unwrap();
        assert!(result.is_liar);
        assert_eq!(result.actual_cards, cards);
        assert!(game.last_play.is_none()); // Should be cleared
    }
    
    #[test]
    fn test_roulette() {
        let mut game = Game::new(2, 6); // All bullets loaded for guaranteed result
        game.deal_cards();
        
        let result = game.execute_roulette(1);
        assert!(result.is_ok());
        
        // Player should be eliminated with 6 bullets
        let player1 = game.find_player(1).unwrap();
        assert!(!player1.is_active);
    }
    
    #[test]
    fn test_winner_detection() {
        let mut game = Game::new(2, 1);
        game.deal_cards();
        
        // No winner initially
        assert!(game.get_winner().is_none());
        
        // Eliminate player 1
        game.find_player_mut(1).unwrap().eliminate();
        
        // Player 2 should be winner
        assert_eq!(game.get_winner(), Some(2));
    }
    
    #[test]
    fn test_advance_turn() {
        let mut game = Game::new(3, 1);
        assert_eq!(game.current_player, 1);
        
        game.advance_turn();
        assert_eq!(game.current_player, 2);
        
        game.advance_turn();
        assert_eq!(game.current_player, 3);
        
        game.advance_turn();
        assert_eq!(game.current_player, 1); // Wrap around
    }
}
