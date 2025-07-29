> **AI 関与について**: このドキュメントは GitHub
> Copilot の支援により作成されました。仕様はオーナーユーザーと AI の協議により決定されています。

# Liar's Table

**実際のトランプカードと併用する心理戦ゲーム補助ツール**

[🇺🇸 English README](README_EN.md)

## 概要

Liar's Table は、ビデオゲーム「[Liar's Bar](https://store.steampowered.com/app/3097560/Liars_Bar/)」にインスパイアされた、実際のトランプカード（ACE・QUEEN・KING 各 6 枚、JOKER 2 枚の計 20 枚）を使った 2-4 人用心理戦ゲームの**デジタル補助ツール**です。

## インストール

### 前提条件

- Rust 1.70 以上

### ビルドと実行

```bash
git clone https://github.com/nanaisisi/liars_table
cd liars_table
cargo build --release
cargo run
```

## 基本的な使い方

### 🃏 実際のカードゲームと併用

1. **物理的な準備**

   - ACE・QUEEN・KING 各 6 枚、JOKER 2 枚を用意（計 20 枚）
   - 各プレイヤーに適切に配布（2 人なら 10 枚ずつ、3 人なら 6-7 枚ずつ、4 人なら 5 枚ずつ）

2. **プログラム起動**

   ```bash
   cargo run
   ```

3. **ゲーム進行の例**

   ```
   === Liar's Table ===

   太郎のターンです
   ✔ 宣言する種類: › ACE
   ✔ 出すカード枚数: › 2
   → 太郎がACE 2枚を宣言しました

   花子のターンです
   太郎の宣言: ACE 2枚
   ✔ どうしますか？
     › Accept (信じる)
       Challenge (Liarだと思う)
   ```

## ゲームルール（簡単版）

### 基本ルール

1. **カード出し**: 現在のプレイヤーが実際のカードを伏せて出し、種類を宣言
2. **チャレンジ判断**: 次のプレイヤーが「Accept（信じる）」か「Challenge（疑う）」を選択
3. **結果処理**: Challenge の場合、カードを確認し、嘘なら出した人が、正直なら疑った人がロシアンルーレット

### 宣言ルール

- **宣言可能**: ACE、QUEEN、KING のみ
- **有効カード**: 宣言した種類 または JOKER（万能）
- **カード構成**: ACE・QUEEN・KING 各 6 枚、JOKER 2 枚
- **JOKER**: どの宣言に対しても有効

### ロシアンルーレット

- **確率**: 1/6 ≈ 16.7%（装弾数は設定可能）
- **結果**: OUT（敗北）または SAFE（ゲーム続行）

## 主な機能

### 🎮 ゲーム進行サポート

- 順番制ターン管理
- カードプレイの記録
- Accept/Challenge 選択
- チャレンジ結果の判定

### ⚙️ 設定・管理

- プレイヤー名のカスタマイズ
- 装弾数の調整（1-12 発）
- 言語切り替え（日本語/英語）
- ゲーム履歴の記録

### 🚧 実装予定機能

- [ ] カードプレイ記録機能
- [ ] チャレンジ判定システム
- [ ] カード確認・検証機能
- [ ] 基本統計表示

## 現在の機能

### ✅ 実装済み機能

- 対話式 UI
- 多言語対応（日本語/英語）
- プレイヤー管理・名前設定
- ロシアンルーレット機能
- 設定の永続化

### 🚧 開発中の機能

実際のカードゲーム体験を完成させる予定：

- カードプレイ記録機能
- 順番制チャレンジシステム
- カード検証・判定機能
- 基本統計・履歴機能

## プロジェクト構造

```
liars_table/
├── src/
│   ├── config.rs         # 設定管理
│   ├── i18n.rs          # 多言語対応
│   ├── interactive.rs    # 対話式UI
│   ├── roulette.rs      # ロシアンルーレット
│   └── main.rs          # エントリーポイント
├── languages/
│   ├── ja.toml          # 日本語メッセージ
│   └── en.toml          # 英語メッセージ
├── doc/
│   ├── concept.md       # ゲーム仕様書
│   ├── history.md       # 開発経緯
│   └── v0.0.1_plan.md   # 実装計画書
├── config.toml          # 高速コンパイル設定
└── Cargo.toml           # パッケージ設定
```

## トラブルシューティング

### 設定ファイルの場所

- Windows: `C:\Users\[username]\.liars_table\config.toml`
- 設定リセット: 上記ファイルを削除して再起動

### よくある問題

- **言語が切り替わらない**: メインメニューから「3. 言語変更」を選択
- **プレイヤー名が保存されない**: プレイヤー設定変更後、メインメニューに戻ることで自動保存

```bash
liars_table init [OPTIONS]
```

**オプション:**

## 貢献・開発

### 開発者向け情報

- **言語**: Rust 2024 Edition
- **主要依存関係**: dialoguer, serde, toml, dirs, thiserror
- **テスト**: `cargo test`
- **ビルド**: `cargo build --release`

#### Rust 環境構築（推奨）

```bash
# Windows - 公式インストーラー経由でのRust導入（推奨）
# https://rustup.rs/ からrustup-init.exeをダウンロードして実行
rustup default stable

# 開発用の追加ツール
# scoopでgitを導入（任意）
scoop install git
rustup component add rustfmt clippy
```

#### コンパイル手法

```bash
# リリースビルド（最適化済み）
cargo build --release

# プロファイル別ビルド（開発用）
cargo build --profile wasm-dev    # WASM向け開発用
cargo build --profile server-dev  # サーバー向け開発用
cargo build --profile android-dev # Android向け開発用
```

**コンパイル設定について**

プロジェクトには `config.toml`（プロジェクト直下）で高速コンパイル設定がデフォルトで含まれています。
この設定により高速なコンパイルが可能ですが、以下のツールが必要です：

```bash
# 必要なツールのインストール（必須）
rustup toolchain install nightly
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly

# Windows環境
scoop install sccache  # ビルドキャッシュ

# Linux環境
sudo apt-get install mold clang  # 高速リンカー
```

**シンプルなコンパイルを行いたい場合**

上記ツールをインストールしたくない場合は、`config.toml` を削除してから：

```bash
rm config.toml  # または手動で削除
cargo build --release
```

#### 依存関係の管理

- **軽量化**: 不要な依存関係は削除済み（clap, uuid, chrono 等）
- **安定性**: 実績のある crate のみ使用
- **メンテナンス性**: 依存関係は最小限に抑制

### 貢献方法

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

### 関連ドキュメント

- [仕様書](doc/concept.md) - ゲームルールの詳細
- [開発経緯](doc/history.md) - 設計思想と変遷
- [実装計画](doc/v0.0.1_plan.md) - 実装計画書

## ライセンス

MIT License または Apache License 2.0 - [LICENSE-MIT](LICENSE-MIT) および [LICENSE-APACHE](LICENSE-APACHE) ファイルをご覧ください。
