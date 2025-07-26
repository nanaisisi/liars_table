# Liar's Table

トランプを使ったロシアンルーレット要素付き心理戦カードゲーム「Liar's Table」の CLI ツールです。

## 概要

Liar's Table は、ACE・QUEEN・KING・JOKER 各 4 枚を使った 2-3 人用のカードゲームです。プレイヤーは手札から複数枚を伏せて出し、その内容について真偽を競います。嘘がバレた場合、または間違った指摘をした場合、ロシアンルーレットが発動します。

## インストール

### 前提条件

- Rust 1.70 以上

### ビルド

```bash
git clone <repository-url>
cd liars_table
cargo build --release
```

実行ファイルは `target/release/liars_table` に生成されます。

## 基本的な使い方

### 1. ゲームの初期化

```bash
liars_table init --players 3 --bullets 1
```

- `--players`: プレイヤー数（2-3 人）
- `--bullets`: ロシアンルーレットの装弾数（1-3 発、デフォルト 1 発）

### 2. カードの配布

```bash
liars_table deal
```

実行すると、各プレイヤーに 5 枚ずつカードが配られ、以下のように表示されます：

```
Player 1: [A♠, Q♥, K♣, Joker, A♦]
Player 2: [Q♠, K♥, Joker, Q♣, K♠]
Player 3: [A♥, K♦, Q♦, Joker, A♣]
```

### 3. カードをプレイ

```bash
liars_table play --player 1 --cards 1,3 --declare ace
```

- `--player`: プレイヤー ID（1-3）
- `--cards`: 出すカードの位置（カンマ区切り、1 から始まる）
- `--declare`: 宣言する種類（`ace`, `queen`, `king`）

### 4. チャレンジ（Liar 指摘）

```bash
liars_table challenge --challenger 2
```

前のプレイヤーが嘘をついていると思う場合に実行します。

結果例：

```
Challenge Result: CORRECT
Player 1 was lying! (Played: Q♥, K♣)
Player 1 must face Russian Roulette.
```

### 5. ロシアンルーレット

```bash
liars_table roulette --target 1
```

指定されたプレイヤーがロシアンルーレットを実行します。

結果例：

```
Russian Roulette Result: SAFE
Player 1 survives this round.
```

または

```
Russian Roulette Result: OUT
Player 1 is eliminated from the game.
```

### 6. ゲーム状態の確認

```bash
liars_table status
```

現在のゲーム状態を表示：

```
=== Game Status ===
Current Turn: Player 2
Active Players: 3

Player 1: 3 cards [Active]
Player 2: 4 cards [Active] ← Current
Player 3: 5 cards [Active]

Roulette Config: 6 chambers, 1 bullet
```

## ゲームの流れ

1. **ゲーム初期化**: プレイヤー数と装弾数を設定
2. **カード配布**: 各プレイヤーに 5 枚ずつ配布
3. **ゲーム開始**: プレイヤー 1 から順番に開始

### ターンの流れ

1. 現在のプレイヤーがカードを出して種類を宣言
2. 次のプレイヤーが`challenge`するかどうかを決定
   - Challenge しない場合：ゲーム続行
   - Challenge する場合：カードが公開され、結果に応じてロシアンルーレット
3. 次のプレイヤーのターン

## コマンドリファレンス

### `init`

ゲームを初期化します。

```bash
liars_table init [OPTIONS]
```

**オプション:**

- `--players <COUNT>`: プレイヤー数（2-3、デフォルト: 3）
- `--bullets <COUNT>`: 装弾数（1-3、デフォルト: 1）

### `deal`

カードを配布します。

```bash
liars_table deal
```

### `play`

カードをプレイします。

```bash
liars_table play --player <ID> --cards <POSITIONS> --declare <TYPE>
```

**引数:**

- `--player <ID>`: プレイヤー ID（1-3）
- `--cards <POSITIONS>`: カードの位置（例: `1,2,3`）
- `--declare <TYPE>`: 宣言する種類（`ace`, `queen`, `king`）

### `challenge`

前のプレイヤーにチャレンジします。

```bash
liars_table challenge --challenger <ID>
```

**引数:**

- `--challenger <ID>`: チャレンジするプレイヤー ID

### `roulette`

ロシアンルーレットを実行します。

```bash
liars_table roulette --target <ID>
```

**引数:**

- `--target <ID>`: 対象プレイヤー ID

### `status`

現在のゲーム状態を表示します。

```bash
liars_table status
```

## ゲームルール詳細

### カードの種類

- **ACE**: 4 枚（A♠, A♥, A♦, A♣）
- **QUEEN**: 4 枚（Q♠, Q♥, Q♦, Q♣）
- **KING**: 4 枚（K♠, K♥, K♦, K♣）
- **JOKER**: 4 枚（万能カード）

### 宣言ルール

- 宣言できるのは`ACE`, `QUEEN`, `KING`のみ
- 実際に出すカードは宣言した種類または JOKER でなければならない
- JOKER は任意の宣言に対して正当なカード

### チャレンジルール

- チャレンジが**正解**（相手が嘘）: 嘘をついたプレイヤーがロシアンルーレット
- チャレンジが**間違い**（相手が正直）: チャレンジしたプレイヤーがロシアンルーレット

### ロシアンルーレット

- 6 つの部屋のうち、指定された数の部屋に弾が装填
- ランダムに 1 つの部屋が選ばれる
- 弾が入っていた場合「OUT」、入っていなかった場合「SAFE」

### 勝利条件

- 最後まで残ったプレイヤーの勝利
- 全員生存の場合、手札を最初に使い切ったプレイヤーの勝利

## トラブルシューティング

### よくあるエラー

**"Game not initialized"**

```bash
# 解決方法：まずゲームを初期化
liars_table init
```

**"Invalid card position"**

```bash
# 解決方法：正しいカード位置を指定（1から始まる）
liars_table play --player 1 --cards 1,2 --declare ace
```

**"Player not found"**

```bash
# 解決方法：有効なプレイヤーIDを指定（1-3）
liars_table challenge --challenger 2
```

### ゲームリセット

問題が発生した場合、以下でゲームをリセットできます：

```bash
rm .liars_table_state.json  # ゲーム状態ファイルを削除
liars_table init             # 新しいゲームを開始
```

## 開発・貢献

プロジェクトへの貢献を歓迎します！詳細は以下をご覧ください：

- [仕様書](doc/concept.md)
- [システム設計](doc/system.md)
- [実装経緯](doc/history.md)

### テスト実行

```bash
cargo test
```

### リリースビルド

```bash
cargo build --release
```

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルをご覧ください。
