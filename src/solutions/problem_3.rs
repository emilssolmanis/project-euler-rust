use super::common::Primes;

pub fn solve(num: u64) -> u64 {
    Primes::new()
        .take_while(|&p| p <= (num as f64).sqrt().ceil() as u64)
        .filter(|p| num % p == 0)
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let num = 13195;

        assert_eq!(29, solve(num));
    }
}
