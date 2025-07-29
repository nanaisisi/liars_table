use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum I18nError {
    #[error("Language file not found: {0}")]
    LanguageNotFound(String),
    #[error("Failed to parse language file: {0}")]
    ParseError(String),
    #[error("Message key not found: {0}")]
    MessageNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageData {
    pub language: LanguageInfo,
    pub messages: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct I18nManager {
    current_language: String,
    messages: HashMap<String, String>,
    available_languages: Vec<LanguageInfo>,
}

impl I18nManager {
    /// 新しいI18nManagerを作成し、利用可能な言語を読み込む
    pub fn new() -> Result<Self, I18nError> {
        let mut manager = I18nManager {
            current_language: "ja".to_string(), // デフォルトは日本語
            messages: HashMap::new(),
            available_languages: Vec::new(),
        };

        // 利用可能な言語を読み込む
        manager.load_available_languages()?;

        // デフォルト言語を読み込む
        manager.load_language("ja")?;

        Ok(manager)
    }

    /// 利用可能な言語一覧を取得
    fn load_available_languages(&mut self) -> Result<(), I18nError> {
        let languages_dir = Path::new("languages");

        if !languages_dir.exists() {
            return Err(I18nError::LanguageNotFound(
                "languages directory not found".to_string(),
            ));
        }

        for entry in fs::read_dir(languages_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let lang_data = self.load_language_file(stem)?;
                    self.available_languages.push(lang_data.language);
                }
            }
        }

        Ok(())
    }

    /// 指定された言語ファイルを読み込む（内部用）
    fn load_language_file(&self, language_code: &str) -> Result<LanguageData, I18nError> {
        let file_path = format!("languages/{}.toml", language_code);

        if !Path::new(&file_path).exists() {
            return Err(I18nError::LanguageNotFound(language_code.to_string()));
        }

        let content = fs::read_to_string(&file_path)?;
        let lang_data: LanguageData = toml::from_str(&content)
            .map_err(|e| I18nError::ParseError(format!("{}: {}", file_path, e)))?;

        Ok(lang_data)
    }

    /// 指定された言語に切り替える
    pub fn load_language(&mut self, language_code: &str) -> Result<(), I18nError> {
        let lang_data = self.load_language_file(language_code)?;

        self.current_language = language_code.to_string();
        self.messages = lang_data.messages;

        Ok(())
    }

    /// メッセージを取得する
    pub fn get_message(&self, key: &str) -> Result<String, I18nError> {
        self.messages
            .get(key)
            .cloned()
            .ok_or_else(|| I18nError::MessageNotFound(key.to_string()))
    }

    /// プレースホルダーを置換してメッセージを取得する
    pub fn get_message_with_args(
        &self,
        key: &str,
        args: &HashMap<String, String>,
    ) -> Result<String, I18nError> {
        let mut message = self.get_message(key)?;

        for (placeholder, value) in args {
            message = message.replace(&format!("{{{}}}", placeholder), value);
        }

        Ok(message)
    }

    /// 現在の言語コードを取得
    #[allow(dead_code)] // 将来のバージョンで使用予定
    pub fn current_language(&self) -> &str {
        &self.current_language
    }

    /// 利用可能な言語一覧を取得
    pub fn available_languages(&self) -> &[LanguageInfo] {
        &self.available_languages
    }

    /// 指定された言語コードが利用可能かチェック
    pub fn is_language_available(&self, language_code: &str) -> bool {
        self.available_languages
            .iter()
            .any(|lang| lang.code == language_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_manager_creation() {
        // テスト用の言語ファイルが存在する場合のみテスト
        if Path::new("languages/ja.toml").exists() {
            let manager = I18nManager::new();
            assert!(manager.is_ok());
        }
    }

    #[test]
    fn test_message_placeholder_replacement() {
        let mut manager = I18nManager {
            current_language: "test".to_string(),
            messages: HashMap::new(),
            available_languages: Vec::new(),
        };

        manager.messages.insert(
            "test_message".to_string(),
            "Hello {name}, you have {count} items".to_string(),
        );

        let mut args = HashMap::new();
        args.insert("name".to_string(), "Alice".to_string());
        args.insert("count".to_string(), "5".to_string());

        let result = manager.get_message_with_args("test_message", &args);
        assert_eq!(result.unwrap(), "Hello Alice, you have 5 items");
    }
}
