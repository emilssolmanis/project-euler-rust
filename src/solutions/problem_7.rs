use super::common::Primes;

pub fn solve(n: usize) -> u64 {
    Primes::new().skip(n).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(0), 2);
        assert_eq!(solve(5), 13);
    }
}
