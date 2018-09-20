use super::common::Primes;

pub fn solve(under: u64) -> u64 {
    Primes::new().take_while(|&p| p < under).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(10), 17);
    }
}