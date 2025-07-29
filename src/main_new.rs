mod config;
mod i18n;
mod interactive;
mod roulette;

// 既存モジュール（後方互換性のため保持）
mod card;
mod game;
mod player;

use interactive::{InteractiveError, InteractiveUI};
use std::process;

fn main() {
    // v0.0.1 対話式UIを起動
    match run_interactive_mode() {
        Ok(()) => {
            println!("\n👋 ゲームを終了します。ありがとうございました！");
        }
        Err(e) => {
            eprintln!("❌ エラーが発生しました: {}", e);

            // デバッグ情報を表示（開発中のみ）
            #[cfg(debug_assertions)]
            {
                eprintln!("\nデバッグ情報:");
                eprintln!("{:?}", e);
            }

            process::exit(1);
        }
    }
}

/// 対話式モードを実行
fn run_interactive_mode() -> Result<(), InteractiveError> {
    let mut ui = InteractiveUI::new()?;
    ui.run()
}

// 後方互換性のための関数（将来的に削除予定）
#[allow(dead_code)]
fn run_legacy_cli() {
    eprintln!("⚠️  CLI モードは廃止予定です。対話式モードをご利用ください。");

    // 既存のCLI実装があればここに配置
    // 現在はプレースホルダーとして空実装
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_modules_exist() {
        // モジュールが正しく読み込まれることを確認
        // 実際のテストは各モジュール内で実装
    }
}
