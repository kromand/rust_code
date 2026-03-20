pub mod random_nums {
    use rand::{Rng, rngs::ThreadRng};
    pub struct prob_gen {
        rng: ThreadRng,
        range: usize,
    }
    impl prob_gen {
        pub fn new(r: usize) -> prob_gen {
            prob_gen {
                rng: rand::rng(),
                range: r,
            }
        }
        pub fn ProbabilityRoll(self: &mut prob_gen, prob: usize) -> bool {
            let roll = self.rng.random_range(0..=self.range);
            roll <= prob
        }
        pub fn ProbabilityRollVect(self: &mut prob_gen, count: usize) -> Vec<usize> {
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
