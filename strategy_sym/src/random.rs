pub mod random_nums {
    use rand::{Rng, rngs::ThreadRng};
    pub struct ProgGen {
        rng: ThreadRng,
        range: usize,
    }
    impl ProgGen {
        pub fn new(r: usize) -> ProgGen {
            ProgGen {
                rng: rand::rng(),
                range: r,
            }
        }
        pub fn probability_roll(self: &mut ProgGen, prob: usize) -> bool {
            let roll = self.rng.random_range(0..=self.range);
            roll <= prob
        }
        pub fn roll_vec(self: &mut ProgGen, count: usize) -> Vec<usize> {
            let mut result = Vec::<usize>::new();
            for _ in 0..count {
                result.push(self.rng.random_range(0..=self.range));
            }
            result
        }
    }
    pub fn generate(upper: usize) -> usize {
        let mut rng = rand::rng();
        rng.random_range(0..=upper) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::random_nums::*;
    #[test]
    pub fn test_prob_gen() {
        let mut pg = ProgGen::new(100);
        let mut true_count = 0;
        let trials = 1000;
        for _ in 0..trials {
            if pg.probability_roll(30) {
                true_count += 1;
            }
        }
        let probability = true_count as f64 / trials as f64;
        assert!(
            (probability - 0.3).abs() < 0.05,
            "Expected around 0.3, got {}",
            probability
        );
    }
    #[test]
    pub fn test_prob_gen_vect() {
        let mut pg = ProgGen::new(100);
        let count = 5;
        let rolls = pg.roll_vec(count);
        assert_eq!(
            rolls.len(),
            count,
            "Expected {} rolls, got {}",
            count,
            rolls.len()
        );
        for roll in rolls {
            assert!(roll <= 100, "Roll {} exceeds upper bound", roll);
        }
    }
}
