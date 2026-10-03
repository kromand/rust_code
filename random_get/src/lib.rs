use rand::Rng;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidRange {
    pub min: i64,
    pub max: i64,
}

impl fmt::Display for InvalidRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "minimum ({}) must not exceed maximum ({})",
            self.min, self.max
        )
    }
}

impl Error for InvalidRange {}

/// Generates `count` random integers in the inclusive range `min..=max`.
pub fn generate_numbers(count: usize, min: i64, max: i64) -> Result<Vec<i64>, InvalidRange> {
    if min > max {
        return Err(InvalidRange { min, max });
    }

    let mut random = rand::thread_rng();
    Ok((0..count).map(|_| random.gen_range(min..=max)).collect())
}

#[cfg(test)]
mod tests {
    use super::generate_numbers;

    #[test]
    fn generates_requested_count_inclusive_range() {
        let numbers = generate_numbers(100, -2, 2).expect("range should be valid");

        assert_eq!(numbers.len(), 100);
        assert!(numbers.iter().all(|number| (-2..=2).contains(number)));
    }

    #[test]
    fn supports_single_value_ranges() {
        assert_eq!(generate_numbers(4, 7, 7).unwrap(), vec![7, 7, 7, 7]);
    }

    #[test]
    fn rejects_reversed_ranges() {
        assert!(generate_numbers(1, 4, 3).is_err());
    }
}
