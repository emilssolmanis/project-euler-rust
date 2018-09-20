use super::common::Primes;

fn highest_pow_under(i: u64, under: u64) -> u64 {
    let power = (under as f64).log(i as f64).floor();
    i.pow(power as u32)
}

pub fn solve(up_to: u64) -> u64 {
    Primes::new()
        .take_while(|&p| p <= up_to)
        .map(|i| highest_pow_under(i, up_to))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(2520, solve(10));
    }

    #[test]
    fn powers() {
        assert_eq!(highest_pow_under(2, 10), 8);
        assert_eq!(highest_pow_under(3, 10), 9);
    }
}
