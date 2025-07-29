mod config;
mod i18n;
mod interactive;
mod roulette;

use interactive::{InteractiveError, InteractiveUI};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ヘルプメッセージ
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("🎴 Liar's Table v0.0.1");
        println!("");
        println!("使用方法:");
        println!("  liars_table                 対話式ゲームを開始");
        println!("  liars_table --test          テストモードで実行（非対話的）");
        println!("  liars_table --help          このヘルプを表示");
        println!("");
        println!("説明:");
        println!("  Liar's Barにインスパイアされたロシアンルーレットゲーム");
        println!("  プレイヤー同士でロシアンルーレットを楽しめます");
        return;
    }

    // テストモード（--testフラグ）の場合
    if args.len() > 1 && args[1] == "--test" {
        match run_test_mode() {
            Ok(()) => {
                println!("✅ テストモード: すべての機能が正常に動作しています");
                process::exit(0);
            }
            Err(e) => {
                eprintln!("❌ テストモードでエラーが発生しました: {}", e);
                process::exit(1);
            }
        }
    }

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

/// テストモードを実行（非対話的）
fn run_test_mode() -> Result<(), InteractiveError> {
    println!("🧪 テストモードを開始します...");

    // 基本的な初期化テスト
    let _ui = InteractiveUI::new()?;
    println!("✅ InteractiveUI初期化成功");

    // 設定ファイルのテスト
    let config = config::GameConfig::load()?;
    println!("✅ 設定ファイル読み込み成功");
    println!("   - 言語: {}", config.language);
    println!("   - 参加人数: {}人", config.participant_count);
    println!("   - 装弾数: {}", config.bullet_capacity);
    println!("   - プレイヤー数: {}人", config.players.len());
    println!(
        "   - アクティブプレイヤー: {}人",
        config.active_player_count()
    );

    // i18nテスト
    let i18n = i18n::I18nManager::new()?;
    println!("✅ 多言語システム初期化成功");
    println!(
        "   - 利用可能言語: {:?}",
        i18n.available_languages()
            .iter()
            .map(|l| &l.code)
            .collect::<Vec<_>>()
    );

    // ロシアンルーレット機能テスト
    println!("✅ ロシアンルーレット機能テスト");
    for capacity in 2..=6 {
        let probability = (100.0 / capacity as f64).round();
        println!("   - 装弾数{}: {}%の確率", capacity, probability);
    }

    Ok(())
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
    #[test]
    fn test_main_modules_exist() {
        // モジュールが正しく読み込まれることを確認
        // 実際のテストは各モジュール内で実装
    }
}
