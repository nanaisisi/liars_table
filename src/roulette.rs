use rand::Rng;
use rand::thread_rng;

/// ロシアンルーレットの結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouletteResult {
    /// セーフ - 実弾に当たらなかった
    Safe,
    /// アウト - 実弾に当たった
    Out,
}

/// ロシアンルーレットを実行する
///
/// # 引数
///
/// * `bullet_capacity` - 装弾数（シリンダー容量）
///
/// # 戻り値
///
/// ロシアンルーレットの結果（Safe または Out）
///
/// # 実装詳細
///
/// - 実弾数は常に1発固定
/// - 確率は 1/bullet_capacity
/// - 標準設定では 1/6 = 約16.7%の確率でOut
pub fn execute_roulette(bullet_capacity: u8) -> RouletteResult {
    let mut rng = thread_rng();

    // 1 から bullet_capacity までの数値をランダムに選択
    // 1が実弾の位置とする（任意の1つの位置でも同じ確率）
    let chamber = rng.gen_range(1..=bullet_capacity);

    if chamber == 1 {
        RouletteResult::Out
    } else {
        RouletteResult::Safe
    }
}

/// ロシアンルーレットの確率を計算（0.0 - 1.0）
#[allow(dead_code)] // 将来のバージョンで使用予定
pub fn calculate_probability(bullet_capacity: u8) -> f64 {
    if bullet_capacity == 0 {
        0.0
    } else {
        1.0 / bullet_capacity as f64
    }
}

/// ロシアンルーレットの確率をパーセンテージで計算
#[allow(dead_code)] // 将来のバージョンで使用予定
pub fn calculate_probability_percentage(bullet_capacity: u8) -> f64 {
    (calculate_probability(bullet_capacity) * 100.0).round()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_roulette() {
        // 複数回実行して両方の結果が出ることを確認
        let mut safe_count = 0;
        let mut out_count = 0;

        for _ in 0..1000 {
            match execute_roulette(6) {
                RouletteResult::Safe => safe_count += 1,
                RouletteResult::Out => out_count += 1,
            }
        }

        // 両方の結果が出ることを確認
        assert!(safe_count > 0);
        assert!(out_count > 0);

        // 大体の比率をチェック（6分の1なので、outは大体100-200回程度）
        assert!(out_count > 50); // 極端に少なくない
        assert!(out_count < 300); // 極端に多くない
    }

    #[test]
    fn test_calculate_probability() {
        assert_eq!(calculate_probability(6), 1.0 / 6.0);
        assert_eq!(calculate_probability(1), 1.0);
        assert_eq!(calculate_probability(0), 0.0);
    }

    #[test]
    fn test_calculate_probability_percentage() {
        assert_eq!(calculate_probability_percentage(6), 17.0); // 16.67% -> 17%（四捨五入）
        assert_eq!(calculate_probability_percentage(1), 100.0);
        assert_eq!(calculate_probability_percentage(0), 0.0);
    }

    #[test]
    fn test_extreme_cases() {
        // 装弾数1の場合は必ずOut
        for _ in 0..10 {
            assert_eq!(execute_roulette(1), RouletteResult::Out);
        }
    }
}
