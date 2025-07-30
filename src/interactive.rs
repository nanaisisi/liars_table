use crate::config::{ConfigError, GameConfig};
use crate::i18n::{I18nError, I18nManager};
use crate::roulette::RouletteResult;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InteractiveError {
    #[error("I18n error: {0}")]
    I18nError(#[from] I18nError),
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Dialog error: {0}")]
    DialogError(String),
}

pub struct InteractiveUI {
    i18n: I18nManager,
    config: GameConfig,
    theme: ColorfulTheme,
}

/// メインメニューの選択肢
#[derive(Debug)]
enum MainMenuChoice {
    ExecuteRoulette,
    PlayerSettings,
    ParticipantCountSettings,
    PlayerRecovery,
    PlayerRevive,
    LanguageSettings,
    RouletteSettings,
    Exit,
}

impl InteractiveUI {
    /// 新しいInteractiveUIを作成
    pub fn new() -> Result<Self, InteractiveError> {
        let mut i18n = I18nManager::new()?;
        let config = GameConfig::load()?;

        // 設定された言語に切り替え
        if i18n.is_language_available(&config.language) {
            i18n.load_language(&config.language)?;
        }

        Ok(InteractiveUI {
            i18n,
            config,
            theme: ColorfulTheme::default(),
        })
    }

    /// メインループを開始
    pub fn run(&mut self) -> Result<(), InteractiveError> {
        // ウェルカムメッセージ
        self.show_welcome()?;

        // 初期言語選択（必要に応じて）
        if self.should_select_language()? {
            self.select_language()?;
        }

        // ゲーム完了状態チェック（前回のゲーム完了後の場合）
        if self.config.game_completed {
            self.handle_game_reset()?;
        }

        // メインゲームループ
        loop {
            // 勝者チェック
            if let Some(winner) = self.config.check_winner() {
                self.show_winner(winner)?;
                self.config.mark_game_completed();
                self.config.save()?;

                // 新しいゲームを開始するか確認
                if self.ask_new_game()? {
                    self.config.reset_game();
                    continue; // ゲームを続行
                } else {
                    break; // ゲーム終了
                }
            }

            // メインメニュー表示
            match self.show_main_menu()? {
                MainMenuChoice::ExecuteRoulette => {
                    self.execute_roulette()?;
                }
                MainMenuChoice::PlayerSettings => {
                    self.player_settings_menu()?;
                }
                MainMenuChoice::ParticipantCountSettings => {
                    self.participant_count_settings_menu()?;
                }
                MainMenuChoice::PlayerRecovery => {
                    self.player_recovery_menu()?;
                }
                MainMenuChoice::PlayerRevive => {
                    self.player_revive_menu()?;
                }
                MainMenuChoice::LanguageSettings => {
                    self.select_language()?;
                }
                MainMenuChoice::RouletteSettings => {
                    self.roulette_settings_menu()?;
                }
                MainMenuChoice::Exit => {
                    break;
                }
            }

            // 設定を保存
            self.config.save()?;
        }

        Ok(())
    }

    /// ウェルカムメッセージを表示
    fn show_welcome(&self) -> Result<(), InteractiveError> {
        println!();
        println!("{}", self.i18n.get_message("welcome_msg")?);
        println!();
        Ok(())
    }

    /// 言語選択が必要かチェック
    fn should_select_language(&self) -> Result<bool, InteractiveError> {
        // 初回起動時や設定された言語が利用不可能な場合
        Ok(!self.i18n.is_language_available(&self.config.language))
    }

    /// 言語選択メニュー
    fn select_language(&mut self) -> Result<(), InteractiveError> {
        let available_languages = self.i18n.available_languages().to_vec(); // cloneして所有権を取得
        let language_names: Vec<&str> = available_languages
            .iter()
            .map(|lang| lang.name.as_str())
            .collect();

        if language_names.is_empty() {
            return Err(InteractiveError::DialogError(
                "No languages available".to_string(),
            ));
        }

        let prompt_text = self
            .i18n
            .get_message("select_language")
            .unwrap_or_else(|_| "Please select language:".to_string());

        let selection = Select::with_theme(&self.theme)
            .with_prompt(prompt_text)
            .items(&language_names)
            .default(0)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Language selection failed: {}", e))
            })?;

        let selected_lang = &available_languages[selection];
        self.i18n.load_language(&selected_lang.code)?;
        self.config.language = selected_lang.code.clone();

        println!("\n✓ {}", self.i18n.get_message("game_start")?);
        Ok(())
    }

    /// メインメニューを表示
    fn show_main_menu(&self) -> Result<MainMenuChoice, InteractiveError> {
        println!("\n{}", self.i18n.get_message("main_menu")?);

        // 現在のターン表示
        if let Some(current_player) = self.config.current_player() {
            let mut args = HashMap::new();
            args.insert("name".to_string(), current_player.name.clone());
            println!(
                "{}",
                self.i18n.get_message_with_args("current_turn", &args)?
            );
        }

        // アクティブプレイヤー数表示
        let active_count = self.config.active_player_count();
        let total_players = self.config.players.len();
        println!(
            "DEBUG: アクティブプレイヤー: {}/{}",
            active_count, total_players
        );

        // プレイヤー詳細表示
        for player in &self.config.players {
            println!(
                "DEBUG: プレイヤー{}: {} (アクティブ: {})",
                player.id, player.name, player.is_active
            );
        }

        let mut args = HashMap::new();
        args.insert("count".to_string(), active_count.to_string());
        println!(
            "{}",
            self.i18n.get_message_with_args("active_players", &args)?
        );

        // 参加人数表示
        args.clear();
        args.insert(
            "count".to_string(),
            self.config.participant_count.to_string(),
        );
        println!(
            "{}",
            self.i18n
                .get_message_with_args("current_participant_count", &args)?
        );
        println!();

        // メニュー選択肢
        let menu_items = vec![
            self.i18n.get_message("menu_roulette")?,
            self.i18n.get_message("menu_player_settings")?,
            self.i18n.get_message("menu_participant_count")?,
            self.i18n.get_message("menu_player_recovery")?,
            self.i18n.get_message("menu_player_revive")?,
            self.i18n.get_message("menu_language")?,
            self.i18n.get_message("menu_roulette_settings")?,
            self.i18n.get_message("menu_exit")?,
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("choose_option")?)
            .items(&menu_items)
            .default(0)
            .interact()
            .map_err(|e| InteractiveError::DialogError(format!("Menu selection failed: {}", e)))?;

        match selection {
            0 => Ok(MainMenuChoice::ExecuteRoulette),
            1 => Ok(MainMenuChoice::PlayerSettings),
            2 => Ok(MainMenuChoice::ParticipantCountSettings),
            3 => Ok(MainMenuChoice::PlayerRecovery),
            4 => Ok(MainMenuChoice::PlayerRevive),
            5 => Ok(MainMenuChoice::LanguageSettings),
            6 => Ok(MainMenuChoice::RouletteSettings),
            7 => Ok(MainMenuChoice::Exit),
            _ => unreachable!(),
        }
    }

    /// ロシアンルーレット実行
    fn execute_roulette(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("roulette_execution")?);

        // アクティブなプレイヤーを取得
        let active_players: Vec<&crate::config::Player> =
            self.config.players.iter().filter(|p| p.is_active).collect();

        if active_players.is_empty() {
            return Err(InteractiveError::DialogError(
                "No active players found".to_string(),
            ));
        }

        // プレイヤー選択
        let player_names: Vec<String> = active_players
            .iter()
            .map(|p| format!("{}: {}", p.id, p.name))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("select_target_player")?)
            .items(&player_names)
            .default(0)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Player selection failed: {}", e))
            })?;

        let target_player = active_players[selection];

        // 対象プレイヤー表示
        let mut args = HashMap::new();
        args.insert("name".to_string(), target_player.name.clone());
        println!(
            "\n{}",
            self.i18n.get_message_with_args("target_player", &args)?
        );

        args.clear();
        args.insert(
            "capacity".to_string(),
            self.config.bullet_capacity.to_string(),
        );
        args.insert(
            "percentage".to_string(),
            format!("{:.0}", self.config.roulette_probability_percentage()),
        );
        println!(
            "{}",
            self.i18n
                .get_message_with_args("roulette_probability", &args)?
        );

        // 実行確認
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("confirm_execution")?)
            .default(false)
            .interact()
            .map_err(|e| InteractiveError::DialogError(format!("Confirmation failed: {}", e)))?;

        if !confirmed {
            // 実行しない場合はそのまま戻る
            return Ok(());
        }

        // ロシアンルーレット実行
        println!("\n{}", self.i18n.get_message("roulette_spinning")?);
        std::thread::sleep(std::time::Duration::from_millis(1500)); // 演出

        let result = crate::roulette::execute_roulette(self.config.bullet_capacity);

        let mut args = HashMap::new();
        args.insert("name".to_string(), target_player.name.clone());

        match result {
            RouletteResult::Safe => {
                println!(
                    "{}",
                    self.i18n
                        .get_message_with_args("roulette_result_safe", &args)?
                );
            }
            RouletteResult::Out => {
                println!(
                    "{}",
                    self.i18n
                        .get_message_with_args("roulette_result_out", &args)?
                );
                println!(
                    "{}",
                    self.i18n
                        .get_message_with_args("player_eliminated", &args)?
                );
                // プレイヤーを除外
                self.config.eliminate_player(target_player.id)?;
            }
        }

        self.wait_for_continue()?;
        Ok(())
    }

    /// プレイヤー設定メニュー
    fn player_settings_menu(&mut self) -> Result<(), InteractiveError> {
        loop {
            println!("\n{}", self.i18n.get_message("current_players")?);
            for player in &self.config.players {
                let status = if player.is_active { "🟢" } else { "🔴" };
                println!("  {}: {} {}", player.id, player.name, status);
            }

            let confirmed = Confirm::with_theme(&self.theme)
                .with_prompt(self.i18n.get_message("change_player_name")?)
                .default(false)
                .interact()
                .map_err(|e| {
                    InteractiveError::DialogError(format!("Confirmation failed: {}", e))
                })?;

            if !confirmed {
                break;
            }

            // プレイヤー番号入力
            let player_id: u8 = Input::with_theme(&self.theme)
                .with_prompt(self.i18n.get_message("which_player")?)
                .validate_with(|input: &u8| -> Result<(), &str> {
                    if *input >= 1 && *input <= self.config.players.len() as u8 {
                        Ok(())
                    } else {
                        Err("Invalid player ID")
                    }
                })
                .interact()
                .map_err(|e| {
                    InteractiveError::DialogError(format!("Player ID input failed: {}", e))
                })?;

            // 新しい名前入力
            let new_name: String = Input::with_theme(&self.theme)
                .with_prompt(self.i18n.get_message("new_name")?)
                .validate_with(|input: &String| -> Result<(), &str> {
                    let trimmed = input.trim();
                    if trimmed.is_empty() {
                        Err("Name cannot be empty")
                    } else if trimmed.len() > 20 {
                        Err("Name too long")
                    } else {
                        Ok(())
                    }
                })
                .interact()
                .map_err(|e| InteractiveError::DialogError(format!("Name input failed: {}", e)))?;

            // 名前変更
            self.config.change_player_name(player_id, new_name)?;
            println!("✓ 名前を変更しました");
        }

        Ok(())
    }

    /// ロシアンルーレット設定メニュー
    fn roulette_settings_menu(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("roulette_config")?);

        let mut args = HashMap::new();
        args.insert(
            "capacity".to_string(),
            self.config.bullet_capacity.to_string(),
        );
        args.insert(
            "percentage".to_string(),
            format!("{:.0}", self.config.roulette_probability_percentage()),
        );
        println!(
            "{}",
            self.i18n
                .get_message_with_args("current_probability", &args)?
        );

        let new_capacity: u8 = Input::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("bullet_capacity_prompt")?)
            .default(self.config.bullet_capacity)
            .validate_with(|input: &u8| -> Result<(), &str> {
                if *input >= 1 && *input <= 12 {
                    Ok(())
                } else {
                    Err("Capacity must be between 1 and 12")
                }
            })
            .interact()
            .map_err(|e| InteractiveError::DialogError(format!("Capacity input failed: {}", e)))?;

        self.config.bullet_capacity = new_capacity;

        println!("✓ 装弾数を{}に設定しました", new_capacity);
        Ok(())
    }

    /// 勝者表示
    fn show_winner(&self, winner: &crate::config::Player) -> Result<(), InteractiveError> {
        println!("\n🎉 ゲーム終了！ 🎉");

        let mut args = HashMap::new();
        args.insert("name".to_string(), winner.name.clone());
        println!("{}", self.i18n.get_message_with_args("game_winner", &args)?);

        self.wait_for_continue()?;
        Ok(())
    }

    /// 続行待ち
    fn wait_for_continue(&self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("continue_prompt")?);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(())
    }

    /// 参加人数設定メニュー
    fn participant_count_settings_menu(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("participant_count_setting")?);

        let mut args = HashMap::new();
        args.insert(
            "count".to_string(),
            self.config.participant_count.to_string(),
        );
        println!(
            "{}",
            self.i18n
                .get_message_with_args("current_participant_count", &args)?
        );

        let new_count: u8 = Input::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("set_participant_count")?)
            .default(self.config.participant_count)
            .validate_with(|input: &u8| -> Result<(), &str> {
                if *input >= 2 && *input <= 4 {
                    Ok(())
                } else {
                    Err("Participant count must be between 2 and 4")
                }
            })
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Participant count input failed: {}", e))
            })?;

        self.config.set_participant_count(new_count)?;

        let mut args = HashMap::new();
        args.insert("count".to_string(), new_count.to_string());
        println!(
            "✓ {}",
            self.i18n
                .get_message_with_args("participant_count_updated", &args)?
        );

        Ok(())
    }

    /// ゲームリセット処理
    fn handle_game_reset(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("game_reset")?);
        self.config.reset_game();
        Ok(())
    }

    /// 新しいゲームを開始するか確認
    fn ask_new_game(&self) -> Result<bool, InteractiveError> {
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("start_new_game")?)
            .default(true)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("New game confirmation failed: {}", e))
            })?;

        Ok(confirmed)
    }

    /// プレイヤー回復メニュー
    fn player_recovery_menu(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("player_recovery")?);

        // 回復可能なプレイヤー（非アクティブで参加人数範囲内）を取得
        let recoverable_players: Vec<(u8, String)> = self
            .config
            .players
            .iter()
            .filter(|p| !p.is_active && p.id <= self.config.participant_count)
            .map(|p| (p.id, p.name.clone()))
            .collect();

        if recoverable_players.is_empty() {
            println!("{}", self.i18n.get_message("no_players_to_recover")?);
            self.wait_for_continue()?;
            return Ok(());
        }

        let player_names: Vec<String> = recoverable_players
            .iter()
            .map(|(id, name)| format!("{}: {}", id, name))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("select_recovery_target")?)
            .items(&player_names)
            .default(0)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Recovery target selection failed: {}", e))
            })?;

        let (target_id, target_name) = &recoverable_players[selection];

        // 確認
        let mut args = HashMap::new();
        args.insert("name".to_string(), target_name.clone());
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message_with_args("confirm_recovery", &args)?)
            .default(true)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Recovery confirmation failed: {}", e))
            })?;

        if confirmed {
            // プレイヤーを回復（アクティブ化）
            if let Some(player) = self.config.get_player_mut(*target_id) {
                player.is_active = true;
            }

            println!(
                "{}",
                self.i18n.get_message_with_args("player_recovered", &args)?
            );
        }

        Ok(())
    }

    /// プレイヤー蘇生メニュー
    fn player_revive_menu(&mut self) -> Result<(), InteractiveError> {
        println!("\n{}", self.i18n.get_message("player_revive")?);

        // 蘇生可能なプレイヤー（非アクティブで参加人数範囲外）を取得
        let revivable_players: Vec<(u8, String)> = self
            .config
            .players
            .iter()
            .filter(|p| !p.is_active && p.id > self.config.participant_count)
            .map(|p| (p.id, p.name.clone()))
            .collect();

        if revivable_players.is_empty() {
            println!("{}", self.i18n.get_message("no_players_to_revive")?);
            self.wait_for_continue()?;
            return Ok(());
        }

        let player_names: Vec<String> = revivable_players
            .iter()
            .map(|(id, name)| format!("{}: {}", id, name))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message("select_revive_target")?)
            .items(&player_names)
            .default(0)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Revive target selection failed: {}", e))
            })?;

        let (target_id, target_name) = &revivable_players[selection];

        // 確認
        let mut args = HashMap::new();
        args.insert("name".to_string(), target_name.clone());
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(self.i18n.get_message_with_args("confirm_revive", &args)?)
            .default(true)
            .interact()
            .map_err(|e| {
                InteractiveError::DialogError(format!("Revive confirmation failed: {}", e))
            })?;

        if confirmed {
            let target_id = *target_id;
            // プレイヤーを蘇生（アクティブ化し、参加人数を増やす）
            if let Some(player) = self.config.get_player_mut(target_id) {
                player.is_active = true;
                // 参加人数を増やして蘇生したプレイヤーを含める
                if self.config.participant_count < target_id {
                    self.config.participant_count = target_id;
                }
            }

            println!(
                "{}",
                self.i18n.get_message_with_args("player_revived", &args)?
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_ui_creation() {
        // 言語ファイルと設定が存在する場合のみテスト
        if std::path::Path::new("languages/ja.toml").exists() {
            let ui = InteractiveUI::new();
            assert!(ui.is_ok());
        }
    }
}
