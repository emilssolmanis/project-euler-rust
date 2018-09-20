struct Primes {
    previous: Vec<u32>
}

impl Primes {
    pub fn new() -> Primes {
        Primes { previous: vec }
    }

    fn is_prime(&self, i: u32) -> bool {
        let test_up_to: u32 = (i as f64).sqrt().ceil() as u32;
        for prime in self.previous.iter().take_while(|i| **i <= test_up_to) {
            if i % *prime == 0 {
                return false;
            }
        }
        true
    }

    fn compute_new_prime(&mut self, prev_prime: u32) {
        for number_under_test in (prev_prime + 1).. {
            if self.is_prime(number_under_test) {
                self.previous.push(number_under_test);
                break;
            };
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let prev_prime = self.previous.last().or(Some(&2)).unwrap();

        self.compute_new_prime(*prev_prime);

        Some(*prev_prime)
    }
}

pub fn solve() {

}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn test_it() {
//        let primes: Vec<u32> = Primes::new().take(5).collect();
//        assert_eq!(primes, vec![2, 3, 5, 7, 11]);
//    }
    #[test]
    fn test_more_it() {
        Primes::new();
    }
}