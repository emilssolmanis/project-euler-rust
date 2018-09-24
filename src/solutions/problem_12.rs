use super::common::Primes;
use std::collections::BTreeMap;

struct TriangleNumbers {
    i: u64
}

impl TriangleNumbers {
    fn new() -> TriangleNumbers {
        TriangleNumbers {
            i: 0
        }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.i += 1;
        Some((self.i * (self.i + 1)) / 2)
    }
}

fn factorize(i: u64, primes: &[u64]) -> BTreeMap<u64, u64> {
    let mut factors = BTreeMap::new();
    let mut remainder = i;

    for &p in primes {
        let mut num_p = 0;

        while remainder % p == 0 && remainder > 1 {
            num_p += 1;
            remainder /= p;
        }

        if num_p != 0 {
            factors.insert(p, num_p);
        }

        if remainder <= 1 {
            break;
        }
    }

    factors
}

fn num_factors(i: u64, primes: &[u64]) -> u64 {
    let factors = factorize(i, primes);

    let mut num_divisors = 0;

    for &power in factors.values() {
        num_divisors += power + num_divisors * power;
    }

    num_divisors + 1
}

pub fn solve(num_factors_needed: u64) -> u64 {
    let primes: Vec<u64> = Primes::new().take(100_000).collect();
    TriangleNumbers::new()
        .find(|&t| num_factors(t, &primes) > num_factors_needed)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let res: Vec<u64> = TriangleNumbers::new().take(10).collect();
        assert_eq!(res, vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55])
    }

    #[test]
    fn test_factorize_24() {
        let primes: Vec<u64> = Primes::new().take(50 as usize).collect();

        let mut expected: BTreeMap<u64, u64> = BTreeMap::new();
        expected.insert(2, 3);
        expected.insert(3, 1);
        let expected: BTreeMap<u64, u64> = expected;

        assert_eq!(factorize(24, &primes), expected);

        let mut expected: BTreeMap<u64, u64> = BTreeMap::new();
        expected.insert(3, 1);
        let expected: BTreeMap<u64, u64> = expected;
        assert_eq!(factorize(3, &primes), expected);
    }

    #[test]
    fn factor_func() {
        let primes: Vec<u64> = Primes::new().take(50 as usize).collect();

        assert_eq!(num_factors(2, &primes), 2);
        assert_eq!(num_factors(3, &primes), 2);
        assert_eq!(num_factors(4, &primes), 3);
        assert_eq!(num_factors(10, &primes), 4);
        assert_eq!(num_factors(28, &primes), 6);
        assert_eq!(num_factors(29, &primes), 2);
        assert_eq!(num_factors(30, &primes), 8);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(5), 28);
    }
}
