use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};

mod card;
mod game;
mod player;
mod roulette;

use card::CardType;
use game::{Game, GameError};

#[derive(Parser)]
#[command(name = "liars_table")]
#[command(about = "A CLI tool for Liar's Table card game with Russian Roulette")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new game
    Init {
        /// Number of players (2-3)
        #[arg(short, long, default_value_t = 3)]
        players: u8,
        /// Number of bullets in roulette (1-3)
        #[arg(short, long, default_value_t = 1)]
        bullets: u8,
    },
    /// Deal cards to all players
    Deal,
    /// Play cards
    Play {
        /// Player ID (1-3)
        #[arg(short, long)]
        player: u8,
        /// Card positions to play (comma separated, 1-based)
        #[arg(short, long)]
        cards: String,
        /// Declared card type
        #[arg(short, long)]
        declare: String,
    },
    /// Challenge the previous player
    Challenge {
        /// ID of the challenging player
        #[arg(short, long)]
        challenger: u8,
    },
    /// Execute Russian Roulette
    Roulette {
        /// Target player ID
        #[arg(short, long)]
        target: u8,
    },
    /// Show current game status
    Status,
}

const STATE_FILE: &str = ".liars_table_state.json";

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { players, bullets } => handle_init(players, bullets),
        Commands::Deal => handle_deal(),
        Commands::Play {
            player,
            cards,
            declare,
        } => handle_play(player, cards, declare),
        Commands::Challenge { challenger } => handle_challenge(challenger),
        Commands::Roulette { target } => handle_roulette(target),
        Commands::Status => handle_status(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_init(players: u8, bullets: u8) -> Result<(), GameError> {
    eprintln!(
        "DEBUG: Initializing game with {} players and {} bullets",
        players, bullets
    );

    if players < 2 || players > 3 {
        return Err(GameError::InvalidPlayerCount);
    }
    if bullets < 1 || bullets > 3 {
        return Err(GameError::InvalidBulletCount);
    }

    let game = Game::new(players, bullets);
    eprintln!("DEBUG: Game created");

    save_game(&game)?;
    eprintln!("DEBUG: Game saved");

    println!(
        "Game initialized with {} players and {} bullets",
        players, bullets
    );
    println!("Use 'liars_table deal' to start the game");
    Ok(())
}

fn handle_deal() -> Result<(), GameError> {
    eprintln!("DEBUG: Attempting to load game...");
    let mut game = load_game()?;
    eprintln!("DEBUG: Game loaded successfully");

    game.deal_cards();
    eprintln!("DEBUG: Cards dealt");

    save_game(&game)?;
    eprintln!("DEBUG: Game saved");

    println!("=== Cards Dealt ===");
    for player in &game.players {
        if player.is_active {
            println!("Player {}: {}", player.id, format_hand(&player.hand));
        }
    }
    println!("Game started! Player 1's turn.");
    Ok(())
}

fn handle_play(player_id: u8, cards_str: String, declare_str: String) -> Result<(), GameError> {
    let mut game = load_game()?;

    let card_positions: Vec<usize> = cards_str
        .split(',')
        .map(|s| s.trim().parse::<usize>().map(|n| n - 1)) // Convert to 0-based
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| GameError::InvalidCardPosition)?;

    let declared_type = match declare_str.to_lowercase().as_str() {
        "ace" => CardType::Ace,
        "queen" => CardType::Queen,
        "king" => CardType::King,
        _ => return Err(GameError::InvalidCardType),
    };

    game.play_cards(player_id, card_positions.clone(), declared_type)?;
    save_game(&game)?;

    println!(
        "Player {} played {} cards and declared: {}",
        player_id,
        card_positions.len(),
        declare_str.to_uppercase()
    );
    println!("Next player can 'challenge' or continue the game.");
    Ok(())
}

fn handle_challenge(challenger_id: u8) -> Result<(), GameError> {
    let mut game = load_game()?;
    let result = game.challenge(challenger_id)?;
    save_game(&game)?;

    match result.is_liar {
        true => {
            println!("Challenge Result: CORRECT");
            println!(
                "Player {} was lying! (Played: {})",
                result.target_player,
                format_cards(&result.actual_cards)
            );
            println!(
                "Player {} must face Russian Roulette.",
                result.target_player
            );
        }
        false => {
            println!("Challenge Result: INCORRECT");
            println!(
                "Player {} was telling the truth! (Played: {})",
                result.target_player,
                format_cards(&result.actual_cards)
            );
            println!("Player {} must face Russian Roulette.", challenger_id);
        }
    }
    Ok(())
}

fn handle_roulette(target_id: u8) -> Result<(), GameError> {
    let mut game = load_game()?;
    let result = game.execute_roulette(target_id)?;
    save_game(&game)?;

    match result.outcome {
        roulette::RouletteOutcome::Safe => {
            println!("Russian Roulette Result: SAFE");
            println!("Player {} survives this round.", target_id);
        }
        roulette::RouletteOutcome::Out => {
            println!("Russian Roulette Result: OUT");
            println!("Player {} is eliminated from the game.", target_id);

            // Check for winner
            let active_players: Vec<_> = game.players.iter().filter(|p| p.is_active).collect();

            if active_players.len() == 1 {
                println!("🎉 Player {} wins the game!", active_players[0].id);
            }
        }
    }
    Ok(())
}

fn handle_status() -> Result<(), GameError> {
    let game = load_game()?;

    println!("=== Game Status ===");
    println!("Current Turn: Player {}", game.current_player);
    println!(
        "Active Players: {}",
        game.players.iter().filter(|p| p.is_active).count()
    );
    println!();

    for player in &game.players {
        let status = if player.is_active {
            if player.id == game.current_player {
                "[Active] ← Current"
            } else {
                "[Active]"
            }
        } else {
            "[Eliminated]"
        };
        println!(
            "Player {}: {} cards {}",
            player.id,
            player.hand.len(),
            status
        );
    }

    println!();
    println!(
        "Roulette Config: {} chambers, {} bullets",
        game.roulette_config.chambers, game.roulette_config.loaded_bullets
    );

    Ok(())
}

fn load_game() -> Result<Game, GameError> {
    let content = fs::read_to_string(STATE_FILE).map_err(|_| GameError::GameNotInitialized)?;
    let game: Game = serde_json::from_str(&content)
        .map_err(|e| GameError::IoError(format!("Failed to parse game state: {}", e)))?;
    Ok(game)
}

fn save_game(game: &Game) -> Result<(), GameError> {
    let content = serde_json::to_string_pretty(game)
        .map_err(|e| GameError::IoError(format!("Failed to serialize game state: {}", e)))?;
    fs::write(STATE_FILE, content)
        .map_err(|e| GameError::IoError(format!("Failed to save game state: {}", e)))?;
    Ok(())
}

fn format_hand(hand: &[card::Card]) -> String {
    hand.iter()
        .map(|card| format!("{}", card))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_cards(cards: &[card::Card]) -> String {
    cards
        .iter()
        .map(|card| format!("{}", card))
        .collect::<Vec<_>>()
        .join(", ")
}
