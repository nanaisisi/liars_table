> **About AI Involvement**: This document was created with assistance from GitHub Copilot. Specifications were determined through discussions between the owner user and AI.

# Liar's Table

**A psychological warfare game assistant tool to be used with actual playing cards**

[🇯🇵 日本語 README](README.md)

## Overview

Liar's Table is a **digital assistant tool** for a 2-4 player psychological warfare game using actual playing cards (6 each of ACE, QUEEN, KING, and 2 JOKERs, totaling 20 cards).

## Installation

### Prerequisites

- Rust 1.70 or higher

### Build and Run

```bash
git clone https://github.com/nanaisisi/liars_table
cd liars_table
cargo build --release
cargo run
```

## Basic Usage

### 🃏 Use with Actual Card Game

1. **Physical Preparation**

   - Prepare 6 each of ACE, QUEEN, KING, and 2 JOKERs (20 cards total)
   - Distribute appropriately to each player (10 each for 2 players, 6-7 each for 3 players, 5 each for 4 players)

2. **Launch Program**

   ```bash
   cargo run
   ```

3. **Game Progress Example**

   ```
   === Liar's Table v0.3 ===

   Taro's turn
   ✔ Card type to declare: › ACE
   ✔ Number of cards to play: › 2
   → Taro declared 2 ACEs

   Hanako's turn
   Taro's declaration: 2 ACEs
   ✔ What do you do?
     › Accept (believe)
       Challenge (think it's a lie)
   ```

## Game Rules (Simplified)

### Basic Rules

1. **Card Play**: Current player places actual cards face down and declares the type
2. **Challenge Decision**: Next player chooses "Accept (believe)" or "Challenge (doubt)"
3. **Result Processing**: If Challenge, cards are checked. If it's a lie, the player who played faces Russian roulette; if honest, the challenger faces Russian roulette

### Declaration Rules

- **Declarable**: ACE, QUEEN, KING only
- **Valid Cards**: Declared type or JOKER (wild)
- **Card Composition**: 6 each of ACE, QUEEN, KING, 2 JOKERs
- **JOKER**: Valid for any declaration

### Russian Roulette

- **Probability**: 1/6 ≈ 16.7% (bullet count is configurable)
- **Result**: OUT (defeat) or SAFE (continue game)

## Main Features

### 🎮 Game Progress Support

- Turn-based turn management
- Card play recording
- Accept/Challenge selection
- Challenge result determination

### ⚙️ Settings & Management

- Player name customization
- Bullet count adjustment (1-12 bullets)
- Language switching (Japanese/English)
- Game history recording

### 🎯 Planned for v0.3

- [ ] Card play recording feature
- [ ] Challenge determination system
- [ ] Card verification & validation feature
- [ ] Basic statistics display

## Development Status

### ✅ v0.2.0 (Current)

- Interactive UI
- Multi-language support (Japanese/English)
- Player management & name setting
- Russian roulette feature
- Settings persistence

### 🚧 v0.3.0 (In Development)

Planning to complete the actual card game experience:

- Card play recording feature
- Turn-based challenge system
- Card verification & determination feature
- Basic statistics & history feature

## Project Structure

```
liars_table/
├── src/
│   ├── config.rs         # Configuration management
│   ├── i18n.rs          # Multi-language support
│   ├── interactive.rs    # Interactive UI
│   ├── roulette.rs      # Russian roulette
│   └── main.rs          # Entry point
├── languages/
│   ├── ja.toml          # Japanese messages
│   └── en.toml          # English messages
└── doc/
    ├── concept.md       # Game specification
    ├── history.md       # Development history
    └── v0.3_plan.md     # v0.3 plan
```

## Troubleshooting

### Configuration File Location

- Windows: `C:\Users\[username]\.liars_table\config.toml`
- Reset settings: Delete the above file and restart

### Common Issues

- **Language doesn't switch**: Select "3. Change Language" from the main menu
- **Player names aren't saved**: After changing player settings, return to main menu for auto-save

## Contributing & Development

### Developer Information

- **Language**: Rust 2024 Edition
- **Main Dependencies**: dialoguer, serde, toml, dirs, thiserror
- **Testing**: `cargo test`
- **Build**: `cargo build --release`

### How to Contribute

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

### Related Documentation

- [Specification](doc/concept.md) - Detailed game rules
- [Development History](doc/history.md) - Design philosophy and evolution
- [v0.3 Plan](doc/v0.3_plan.md) - Next version plan

## License

MIT License or Apache License 2.0 - See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files.

---

**Liar's Table is a practical game assistant tool that provides the convenience of digital tools without compromising the charm of physical card games.**
