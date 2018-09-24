use rug::Integer;

pub struct Fibo {
    prev: Integer,
    curr: Integer,
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo {
            prev: Integer::from(0),
            curr: Integer::from(1),
        }
    }
}

impl Iterator for Fibo {
    type Item = Integer;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let existing_prev = self.prev.clone();
        self.prev = self.curr.clone();
        self.curr += existing_prev.clone();
        Some(existing_prev)
    }
}

pub fn factorial(x: u64) -> Integer {
    let mut res = Integer::from(1);
    for i in 1..=x {
        res *= Integer::from(i);
    }
    res
}

pub trait ToDigits {
    fn to_individual_digits(&self) -> Vec<u32>;
}

impl ToDigits for str {
    fn to_individual_digits(&self) -> Vec<u32> {
        self.as_bytes().iter().map(|&b| u32::from(b) - 48).collect()
    }
}

impl ToDigits for Integer {
    fn to_individual_digits(&self) -> Vec<u32> {
        self.to_string().to_individual_digits()
    }
}

pub trait Factors {
    type Item;

    fn factors(&self) -> Vec<Self::Item>;
}

impl Factors for u64 {
    type Item = u64;

    fn factors(&self) -> Vec<Self::Item> {
        (1..*self).filter(|&i| self % i == 0).collect()
    }
}

pub struct Primes {
    previous: Vec<u64>
}

impl Primes {
    pub fn new() -> Primes {
        Primes { previous: vec![2] }
    }

    fn is_prime(&self, i: u64) -> bool {
        let test_up_to: u64 = (i as f64).sqrt().ceil() as u64;

        for &prime in self.previous.iter().take_while(|&&i| i <= test_up_to) {
            if i % prime == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let primes: Vec<u64> = Primes::new().take(7).collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17]);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(Integer::from(120), factorial(5));
    }

    #[test]
    fn test_str_digits() {
        assert_eq!("1234321".to_individual_digits(), vec![1, 2, 3, 4, 3, 2, 1]);
    }

    #[test]
    fn test_factors() {
        assert_eq!(220.factors(), vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);
        assert_eq!(284.factors(), vec![1, 2, 4, 71, 142]);
    }

    #[test]
    fn test_fibo() {
        let res: Vec<Integer> = Fibo::new().take_while(|i| i < &10).collect();

        assert_eq!(
            res,
            vec![0, 1, 1, 2, 3, 5, 8]);
    }
}
