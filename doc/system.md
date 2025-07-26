# Liar's Table - システム設計書（第 2 版）

## アーキテクチャ概要（新設計）

```
┌─────────────────────┐
│ Interactive UI      │
├─────────────────────┤
│ I18n System         │
├─────────────────────┤
│ Player Manager      │
├─────────────────────┤
│ Roulette Engine     │
├─────────────────────┤
│ Config Manager      │
├─────────────────────┤
│ Secure Random       │
└─────────────────────┘
```

## 設計変更の概要

### 従来設計の問題点

- カード管理の不要性（実際のトランプを使用するため）
- コマンドライン引数の煩雑さ
- 国際化対応の欠如
- プレイヤー識別の困難さ

### 新しい設計方針

1. **対話式 UI**: 自然な会話フローでの操作
2. **多言語対応**: TOML 設定による言語管理
3. **プレイヤー中心**: カスタム名前と ID 管理
4. **シンプル化**: ロシアンルーレットの仕様簡素化

## モジュール構成（新設計）

### 1. メインモジュール (`main.rs`)

- 対話式 UI のエントリーポイント
- 言語選択とゲーム開始
- 全体フローの制御

### 2. 国際化システム (`i18n.rs`)

```rust
pub struct I18n {
    current_language: String,
    messages: HashMap<String, HashMap<String, String>>,
}

impl I18n {
    pub fn new(language: &str) -> Result<Self, I18nError>;
    pub fn get(&self, key: &str) -> &str;
    pub fn set_language(&mut self, language: &str) -> Result<(), I18nError>;
}
```

### 3. プレイヤー管理 (`player_manager.rs`)

```rust
pub struct PlayerManager {
    players: Vec<Player>,
    current_player: usize,
}

pub struct Player {
    id: u8,
    name: String,
    is_active: bool,
}

impl PlayerManager {
    pub fn setup_players(count: u8) -> Self;
    pub fn set_player_name(&mut self, id: u8, name: String);
    pub fn get_current_player(&self) -> &Player;
    pub fn eliminate_player(&mut self, id: u8);
}
```

### 4. 対話式 UI (`interactive.rs`)

```rust
pub struct InteractiveUI {
    i18n: I18n,
    player_manager: PlayerManager,
}

impl InteractiveUI {
    pub fn run(&mut self) -> Result<(), UIError>;
    pub fn select_language(&mut self) -> Result<String, UIError>;
    pub fn setup_players(&mut self) -> Result<(), UIError>;
    pub fn main_menu(&mut self) -> Result<MenuChoice, UIError>;
    pub fn roulette_challenge(&mut self, player_id: u8) -> Result<(), UIError>;
}
```

### 5. 設定管理 (`config.rs`)

```rust
#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    language: String,
    bullet_capacity: u8,    // 装弾数
    player_names: Vec<String>,
}

impl GameConfig {
    pub fn load() -> Result<Self, ConfigError>;
    pub fn save(&self) -> Result<(), ConfigError>;
    pub fn default() -> Self;
}
```

### 6. ロシアンルーレット（簡素化）(`roulette.rs`)

```rust
pub struct RouletteConfig {
    bullet_capacity: u8,    // 装弾数（6が標準）
    // 実弾数は常に1で固定
}

pub struct RouletteEngine {
    config: RouletteConfig,
}

impl RouletteEngine {
    pub fn spin(&self) -> RouletteResult;
    pub fn get_probability(&self) -> f64 {
        1.0 / self.config.bullet_capacity as f64
    }
}
    // 実弾数は常に1で固定
}

pub struct RouletteEngine {
    config: RouletteConfig,
}

impl RouletteEngine {
    pub fn spin(&self) -> RouletteResult;
    pub fn get_probability(&self) -> f64 {
        1.0 / self.config.chamber_capacity as f64
    }
}
```

## データ構造（新設計）

### 設定ファイル構造

```toml
# config.toml
[game]
language = "ja"
bullet_capacity = 6
player_count = 3

[players]
names = ["太郎", "花子", "次郎"]
```

### 言語ファイル構造（分離型）

```toml
# languages/ja.toml
[language]
code = "ja"
name = "日本語"

[messages]
welcome_msg = "🎴 Liar's Tableへようこそ！"
select_language = "言語を選択してください："
setup_players = "プレイヤーを設定してください"
player_name_prompt = "プレイヤー{id}の名前："
bullet_capacity_prompt = "装弾数を設定してください（標準: 6）："
roulette_result_safe = "{name}さんはセーフです！"
roulette_result_out = "{name}さんはアウトです..."
```

```toml
# languages/en.toml
[language]
code = "en"
name = "English"

[messages]
welcome_msg = "🎴 Welcome to Liar's Table!"
select_language = "Please select language:"
setup_players = "Please setup players"
player_name_prompt = "Player {id} name:"
bullet_capacity_prompt = "Set bullet capacity (default: 6):"
roulette_result_safe = "{name} is SAFE!"
roulette_result_out = "{name} is OUT..."
```

[languages.en]
name = "English"
welcome = "Welcome to Liar's Table!"
select_language = "Please select language:"
setup_players = "Please setup players"
player_name_prompt = "Player {id} name:"
roulette_result_safe = "{name} is SAFE!"
roulette_result_out = "{name} is OUT..."

````

### ゲーム状態ファイル

```json
{
  "config": {
    "language": "ja",
    "chamber_capacity": 6
  },
  "players": [
    { "id": 1, "name": "太郎", "is_active": true },
    { "id": 2, "name": "花子", "is_active": true },
    { "id": 3, "name": "次郎", "is_active": false }
  ],
  "current_player": 1,
  "game_log": []
}
````

## 対話式 UI フロー

### 起動時フロー

```
1. 言語選択
   ┌─────────────────────┐
   │ Select Language:    │
   │ 1. 日本語           │
   │ 2. English          │
   │ Choice: _           │
   └─────────────────────┘

2. プレイヤー設定
   ┌─────────────────────┐
   │ プレイヤー設定      │
   │ プレイヤー1の名前: _ │
   │ プレイヤー2の名前: _ │
   │ プレイヤー3の名前: _ │
   └─────────────────────┘

3. ゲーム設定
   ┌─────────────────────┐
   │ ロシアンルーレット設定 │
   │ 装填上限 (標準6): _  │
   └─────────────────────┘
```

### メインゲームフロー

```
1. メインメニュー
   ┌─────────────────────┐
   │ = Liar's Table =    │
   │ 現在: 太郎のターン   │
   │                     │
   │ 1. ロシアンルーレット │
   │ 2. プレイヤー設定変更 │
   │ 3. 言語変更         │
   │ 4. 終了             │
   │ Choice: _           │
   └─────────────────────┘

2. ロシアンルーレット実行
   ┌─────────────────────┐
   │ ロシアンルーレット   │
   │ 対象: 太郎          │
   │ 確率: 1/6 (16.7%)   │
   │                     │
   │ 実行しますか？(y/N): _ │
   └─────────────────────┘

3. 結果表示
   ┌─────────────────────┐
   │ 結果: セーフ！       │
   │ 太郎さんは生き残りました │
   │                     │
   │ 続行するには Enter... │
   └─────────────────────┘
```

## 依存関係の追加

### 新しい Crate

```toml
[dependencies]
# 既存
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"

# 新規追加
dialoguer = "0.11"          # 対話式UI
toml = "0.8"                # TOML設定ファイル
dirs = "5.0"                # 設定ディレクトリ管理
thiserror = "1.0"           # エラーハンドリング
```

## 実装優先順位

### Phase 2A: 基盤整備（1-2 日）

1. ✅ **多言語システム**の実装
2. ✅ **設定管理機能**の実装
3. ✅ **対話式 UI 基盤**の構築

### Phase 2B: 機能実装（2-3 日）

1. 🔄 **プレイヤー管理**の刷新
2. 🔄 **ロシアンルーレット**の簡素化
3. 🔄 **メインゲームフロー**の再実装

### Phase 2C: 統合・テスト（1 日）

1. ⏳ **全機能の統合**
2. ⏳ **多言語テスト**
3. ⏳ **ユーザビリティテスト**
   Ace,
   Queen,
   King,
   Joker,
   }

pub struct Card {
card_type: CardType,
id: u8, // 1-4 (同じ種類の 4 枚を区別)
}

pub struct Deck {
cards: Vec<Card>,
}

````

### 3. プレイヤー管理 (`player.rs`)

```rust
pub struct Player {
    id: u8,
    name: String,
    hand: Vec<Card>,
    is_active: bool,
}

pub struct GameState {
    players: Vec<Player>,
    current_player: usize,
    table_cards: Vec<Card>,
    declared_type: Option<CardType>,
}
````

### 4. ロシアンルーレット (`roulette.rs`)

```rust
pub struct RouletteConfig {
    chambers: u8,      // 通常6
    loaded_bullets: u8, // 装弾数
}

pub fn spin_roulette(config: &RouletteConfig) -> RouletteResult;
```

### 5. ゲームエンジン (`game.rs`)

```rust
pub struct Game {
    state: GameState,
    roulette_config: RouletteConfig,
}

impl Game {
    pub fn new(player_count: u8) -> Self;
    pub fn deal_cards(&mut self);
    pub fn play_cards(&mut self, player_id: u8, cards: Vec<Card>, declared: CardType);
    pub fn challenge(&mut self, challenger_id: u8) -> ChallengeResult;
    pub fn execute_roulette(&mut self, target_id: u8) -> RouletteResult;
}
```

## コマンドライン仕様

### メインコマンド

```bash
liars_table [SUBCOMMAND]
```

### サブコマンド

#### 1. ゲーム初期化

```bash
liars_table init --players <COUNT> --bullets <COUNT>
```

- `--players`: プレイヤー数（2-3）
- `--bullets`: 装弾数（1-3、デフォルト 1）

#### 2. カード配布

```bash
liars_table deal
```

- 各プレイヤーに 5 枚ずつランダムに配布
- 結果をプレイヤー別に表示

#### 3. カードプレイ

```bash
liars_table play --player <ID> --cards <POSITIONS> --declare <TYPE>
```

- `--player`: プレイヤー ID
- `--cards`: 手札の位置（1,2,3 など）
- `--declare`: 宣言する種類（ace/queen/king）

#### 4. チャレンジ

```bash
liars_table challenge --challenger <ID>
```

- `--challenger`: チャレンジするプレイヤー ID
- 結果と次のアクションを表示

#### 5. ロシアンルーレット

```bash
liars_table roulette --target <ID>
```

- `--target`: 対象プレイヤー ID
- 結果（セーフ/アウト）を表示

#### 6. ゲーム状態表示

```bash
liars_table status
```

- 現在のゲーム状態を表示
- 各プレイヤーの手札数
- 現在のターン

## データ構造

### ゲーム状態ファイル

```json
{
  "game_id": "uuid",
  "created_at": "2025-07-26T10:00:00Z",
  "players": [
    {
      "id": 1,
      "name": "Player1",
      "hand": [
        { "type": "Ace", "id": 1 },
        { "type": "Joker", "id": 3 }
      ],
      "is_active": true
    }
  ],
  "roulette_config": {
    "chambers": 6,
    "loaded_bullets": 1
  },
  "current_turn": 1,
  "game_log": []
}
```

## 乱数生成戦略

### 要件

- 暗号学的に安全
- 予測不可能
- 再現不可能

### 実装

```rust
use rand::rngs::OsRng;
use rand::RngCore;

pub struct SecureRandom {
    rng: OsRng,
}

impl SecureRandom {
    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    pub fn gen_range(&mut self, range: std::ops::Range<u8>) -> u8 {
        // OS提供の暗号学的乱数を使用
    }
}
```

## エラーハンドリング

### エラー種別

```rust
pub enum GameError {
    InvalidPlayerCount,
    InvalidCardPosition,
    PlayerNotFound,
    GameNotInitialized,
    InvalidCommand,
    IoError(std::io::Error),
}
```

### エラー処理方針

- 全てのエラーは適切にキャッチ
- ユーザーフレンドリーなエラーメッセージ
- ゲーム状態の整合性を保証

## テスト戦略

### 単体テスト

- 各モジュールの基本機能
- エッジケースの処理
- エラー条件のテスト

### 統合テスト

- コマンドライン引数の処理
- ゲーム全体のフロー
- ファイル I/O 操作

### 確率テスト

- ロシアンルーレットの確率分布
- カード配布のランダム性
- 長期間実行での偏りチェック

## パフォーマンス考慮事項

### メモリ使用量

- 最小限のカード情報のみ保持
- 不要なデータの早期開放

### 実行速度

- O(1)での状態アクセス
- 効率的なカード検索

### スケーラビリティ

- プレイヤー数の増加に対応
- 将来的なネットワーク機能への拡張性

## セキュリティ考慮事項

### 乱数の安全性

- OS 提供の暗号学的乱数生成器使用
- 予測不可能な結果保証

### データ保護

- 機密情報の適切な管理
- 一時ファイルの安全な削除

### 入力検証

- 全ての外部入力の検証
- SQL インジェクション等の防止
