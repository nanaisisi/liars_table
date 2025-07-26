use rand::thread_rng;
use rand::Rng;

/// ロシアンルーレットの結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouletteResult {
    /// セーフ - 実弾に当たらなかった
    Safe,
    /// アウト - 実弾に当たった
    Out,
}
    
    pub fn spin(&self) -> RouletteResult {
        // Generate cryptographically secure random numbers
        let mut rng = OsRng;
        
        // Determine which chambers are loaded
        let mut loaded_chambers = Vec::new();
        let mut chambers: Vec<u8> = (1..=self.config.chambers).collect();
        
        // Randomly select chambers to load
        for _ in 0..self.config.loaded_bullets {
            if chambers.is_empty() {
                break;
            }
            
            let index = (rng.next_u32() as usize) % chambers.len();
            loaded_chambers.push(chambers.remove(index));
        }
        
        // Spin the cylinder - select a random chamber
        let chamber_hit = ((rng.next_u32() as u8) % self.config.chambers) + 1;
        
        let outcome = if loaded_chambers.contains(&chamber_hit) {
            RouletteOutcome::Out
        } else {
            RouletteOutcome::Safe
        };
        
        RouletteResult {
            outcome,
            chamber_hit,
            loaded_chambers,
        }
    }
    
    pub fn config(&self) -> &RouletteConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roulette_config() {
        let config = RouletteConfig::new(2);
        assert_eq!(config.chambers, 6);
        assert_eq!(config.loaded_bullets, 2);
        assert_eq!(config.probability(), 2.0 / 6.0);
    }
    
    #[test]
    fn test_default_config() {
        let config = RouletteConfig::default();
        assert_eq!(config.chambers, 6);
        assert_eq!(config.loaded_bullets, 1);
    }
    
    #[test]
    fn test_roulette_engine() {
        let config = RouletteConfig::new(1);
        let engine = RouletteEngine::new(config);
        
        // Test multiple spins to ensure randomness
        let mut safe_count = 0;
        let mut out_count = 0;
        let total_spins = 1000;
        
        for _ in 0..total_spins {
            let result = engine.spin();
            
            // Verify chamber_hit is in valid range
            assert!(result.chamber_hit >= 1 && result.chamber_hit <= 6);
            
            // Verify loaded_chambers count
            assert_eq!(result.loaded_chambers.len(), 1);
            
            // Count outcomes
            match result.outcome {
                RouletteOutcome::Safe => safe_count += 1,
                RouletteOutcome::Out => out_count += 1,
            }
            
            // Verify outcome consistency
            let is_loaded = result.loaded_chambers.contains(&result.chamber_hit);
            match result.outcome {
                RouletteOutcome::Out => assert!(is_loaded),
                RouletteOutcome::Safe => assert!(!is_loaded),
            }
        }
        
        // Check that we get both outcomes (very high probability)
        assert!(safe_count > 0);
        assert!(out_count > 0);
        assert_eq!(safe_count + out_count, total_spins);
        
        // Check approximate distribution (with large tolerance for randomness)
        let expected_out_ratio = 1.0 / 6.0;
        let actual_out_ratio = out_count as f64 / total_spins as f64;
        let tolerance = 0.05; // 5% tolerance
        
        assert!(
            (actual_out_ratio - expected_out_ratio).abs() < tolerance,
            "Expected ~{:.2}, got {:.2}",
            expected_out_ratio,
            actual_out_ratio
        );
    }
    
    #[test]
    fn test_all_chambers_loaded() {
        let config = RouletteConfig {
            chambers: 6,
            loaded_bullets: 6,
        };
        let engine = RouletteEngine::new(config);
        
        // All spins should result in Out
        for _ in 0..100 {
            let result = engine.spin();
            assert_eq!(result.outcome, RouletteOutcome::Out);
            assert_eq!(result.loaded_chambers.len(), 6);
        }
    }
    
    #[test]
    fn test_no_bullets() {
        let config = RouletteConfig {
            chambers: 6,
            loaded_bullets: 0,
        };
        let engine = RouletteEngine::new(config);
        
        // All spins should result in Safe
        for _ in 0..100 {
            let result = engine.spin();
            assert_eq!(result.outcome, RouletteOutcome::Safe);
            assert!(result.loaded_chambers.is_empty());
        }
    }
}
