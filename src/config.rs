use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found")]
    #[allow(dead_code)] // 将来のバージョンで使用予定
    ConfigNotFound,
    #[error("Failed to parse configuration: {0}")]
    ParseError(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameConfig {
    pub language: String,
    pub bullet_capacity: u8, // 装弾数（シリンダー容量）
    pub players: Vec<Player>,
    pub current_turn: u8, // 現在のターンのプレイヤーID
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            language: "ja".to_string(),
            bullet_capacity: 6,
            players: vec![
                Player {
                    id: 1,
                    name: "プレイヤー1".to_string(),
                    is_active: true,
                },
                Player {
                    id: 2,
                    name: "プレイヤー2".to_string(),
                    is_active: true,
                },
                Player {
                    id: 3,
                    name: "プレイヤー3".to_string(),
                    is_active: true,
                },
                Player {
                    id: 4,
                    name: "プレイヤー4".to_string(),
                    is_active: true,
                },
            ],
            current_turn: 1,
        }
    }
}

impl GameConfig {
    /// 設定ファイルから読み込む
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path();

        if !config_path.exists() {
            // 設定ファイルが存在しない場合、デフォルト設定を作成して保存
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: GameConfig =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(format!("{}", e)))?;

        // 設定値の検証
        config.validate()?;

        Ok(config)
    }

    /// 設定ファイルに保存する
    pub fn save(&self) -> Result<(), ConfigError> {
        // 保存前に検証
        self.validate()?;

        let config_path = Self::get_config_path();

        // 親ディレクトリが存在しない場合は作成
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    /// 設定ファイルのパスを取得
    fn get_config_path() -> PathBuf {
        // ホームディレクトリまたは現在のディレクトリに設定ファイルを配置
        if let Some(home_dir) = dirs::home_dir() {
            home_dir.join(".liars_table").join("config.toml")
        } else {
            PathBuf::from(".liars_table_config.toml")
        }
    }

    /// 設定値の検証
    fn validate(&self) -> Result<(), ConfigError> {
        // 装弾数の検証
        if self.bullet_capacity == 0 {
            return Err(ConfigError::InvalidValue(
                "Bullet capacity cannot be zero".to_string(),
            ));
        }
        if self.bullet_capacity > 12 {
            return Err(ConfigError::InvalidValue(
                "Bullet capacity cannot exceed 12".to_string(),
            ));
        }

        // プレイヤーの検証
        if self.players.is_empty() {
            return Err(ConfigError::InvalidValue(
                "At least one player is required".to_string(),
            ));
        }
        if self.players.len() > 6 {
            return Err(ConfigError::InvalidValue(
                "Maximum 6 players allowed".to_string(),
            ));
        }

        // プレイヤーIDの重複チェック
        let mut ids = std::collections::HashSet::new();
        for player in &self.players {
            if !ids.insert(player.id) {
                return Err(ConfigError::InvalidValue(format!(
                    "Duplicate player ID: {}",
                    player.id
                )));
            }

            // 名前の検証
            if player.name.trim().is_empty() {
                return Err(ConfigError::InvalidValue(format!(
                    "Player {} name cannot be empty",
                    player.id
                )));
            }
            if player.name.len() > 20 {
                return Err(ConfigError::InvalidValue(format!(
                    "Player {} name too long",
                    player.id
                )));
            }
        }

        // 現在のターンの検証
        if !self
            .players
            .iter()
            .any(|p| p.id == self.current_turn && p.is_active)
        {
            return Err(ConfigError::InvalidValue(
                "Current turn player is not active".to_string(),
            ));
        }

        Ok(())
    }

    /// アクティブなプレイヤー数を取得
    pub fn active_player_count(&self) -> usize {
        self.players.iter().filter(|p| p.is_active).count()
    }

    /// アクティブなプレイヤーのリストを取得
    pub fn active_players(&self) -> Vec<&Player> {
        self.players.iter().filter(|p| p.is_active).collect()
    }

    /// プレイヤーをIDで検索
    pub fn get_player(&self, id: u8) -> Option<&Player> {
        self.players.iter().find(|p| p.id == id)
    }

    /// プレイヤーをIDで検索（mutable）
    pub fn get_player_mut(&mut self, id: u8) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == id)
    }

    /// プレイヤー名を変更
    pub fn change_player_name(&mut self, id: u8, new_name: String) -> Result<(), ConfigError> {
        // 名前の検証
        let trimmed_name = new_name.trim();
        if trimmed_name.is_empty() {
            return Err(ConfigError::InvalidValue(
                "Player name cannot be empty".to_string(),
            ));
        }
        if trimmed_name.len() > 20 {
            return Err(ConfigError::InvalidValue(
                "Player name too long".to_string(),
            ));
        }

        if let Some(player) = self.get_player_mut(id) {
            player.name = trimmed_name.to_string();
            Ok(())
        } else {
            Err(ConfigError::InvalidValue(format!(
                "Player {} not found",
                id
            )))
        }
    }

    /// プレイヤーを除外（非アクティブ化）
    pub fn eliminate_player(&mut self, id: u8) -> Result<(), ConfigError> {
        if let Some(player) = self.get_player_mut(id) {
            player.is_active = false;

            // 除外されたプレイヤーが現在のターンの場合、次のプレイヤーに移す
            if self.current_turn == id {
                self.next_turn();
            }

            Ok(())
        } else {
            Err(ConfigError::InvalidValue(format!(
                "Player {} not found",
                id
            )))
        }
    }

    /// 次のプレイヤーのターンに移す
    pub fn next_turn(&mut self) {
        let active_players: Vec<_> = self.active_players().into_iter().map(|p| p.id).collect();

        if active_players.is_empty() {
            return;
        }

        // 現在のプレイヤーの次のアクティブプレイヤーを見つける
        if let Some(current_index) = active_players
            .iter()
            .position(|&id| id == self.current_turn)
        {
            let next_index = (current_index + 1) % active_players.len();
            self.current_turn = active_players[next_index];
        } else {
            // 現在のプレイヤーがアクティブでない場合、最初のアクティブプレイヤーに設定
            self.current_turn = active_players[0];
        }
    }

    /// 現在のターンのプレイヤーを取得
    pub fn current_player(&self) -> Option<&Player> {
        self.get_player(self.current_turn)
    }

    /// 勝者をチェック（アクティブプレイヤーが1人の場合）
    pub fn check_winner(&self) -> Option<&Player> {
        let active_players = self.active_players();
        if active_players.len() == 1 {
            Some(active_players[0])
        } else {
            None
        }
    }

    /// ロシアンルーレットの確率を計算（パーセンテージ）
    pub fn roulette_probability_percentage(&self) -> f64 {
        (100.0 / self.bullet_capacity as f64).round()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GameConfig::default();
        assert_eq!(config.language, "ja");
        assert_eq!(config.bullet_capacity, 6);
        assert_eq!(config.players.len(), 3);
        assert_eq!(config.current_turn, 1);
    }

    #[test]
    fn test_config_validation() {
        let mut config = GameConfig::default();

        // 正常な設定
        assert!(config.validate().is_ok());

        // 装弾数が0
        config.bullet_capacity = 0;
        assert!(config.validate().is_err());

        // 装弾数が大きすぎる
        config.bullet_capacity = 15;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_player_management() {
        let mut config = GameConfig::default();

        // プレイヤー名変更
        assert!(
            config
                .change_player_name(1, "テスト太郎".to_string())
                .is_ok()
        );
        assert_eq!(config.get_player(1).unwrap().name, "テスト太郎");

        // プレイヤー除外
        assert!(config.eliminate_player(1).is_ok());
        assert!(!config.get_player(1).unwrap().is_active);

        // アクティブプレイヤー数
        assert_eq!(config.active_player_count(), 2);
    }

    #[test]
    fn test_turn_management() {
        let mut config = GameConfig::default();

        // 初期状態
        assert_eq!(config.current_turn, 1);

        // ターン進行
        config.next_turn();
        assert_eq!(config.current_turn, 2);

        config.next_turn();
        assert_eq!(config.current_turn, 3);

        config.next_turn();
        assert_eq!(config.current_turn, 1); // 最初に戻る
    }
}
