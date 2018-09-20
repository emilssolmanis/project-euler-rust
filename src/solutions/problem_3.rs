struct Primes {
    previous: Vec<u64>
}

impl Primes {
    pub fn new() -> Primes {
        Primes { previous: vec![2] }
    }

    fn is_prime(&self, i: u64) -> bool {
        let test_up_to: u64 = (i as f64).sqrt().ceil() as u64;

        for prime in self.previous.iter().take_while(|i| **i <= test_up_to) {
            if i % *prime == 0 {
                return false;
            };
        };

        true
    }

    fn compute_new_prime(&mut self, prev_prime: u64) {
        for number_under_test in (prev_prime + 1).. {
            if self.is_prime(number_under_test) {
                self.previous.push(number_under_test);
                break;
            };
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut default = 2;
        let prev_prime = *self.previous.last_mut().unwrap_or(&mut default);

        self.compute_new_prime(prev_prime);

        Some(prev_prime)
    }
}

pub fn solve(num: u64) -> u64 {
    Primes::new()
        .take_while(|p| *p <= (num as f64).sqrt().ceil() as u64)
        .filter(|p| num % p == 0)
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let primes: Vec<u64> = Primes::new().take(7).collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17]);
    }

    #[test]
    fn test_example() {
        let num = 13195;

        assert_eq!(29, solve(num));
    }
}
