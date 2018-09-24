use super::common::Factors;
use std::collections::HashMap;

struct AbundanceChecker {
    cache: HashMap<u64, bool>,
}

impl AbundanceChecker {
    fn new() -> AbundanceChecker {
        AbundanceChecker { cache: HashMap::new() }
    }

    fn abundant(&mut self, x: u64) -> bool {
        match self.cache.get(&x) {
            Some(&res) => res,
            None => {
                let res = x < x.factors().iter().sum::<u64>();
                self.cache.insert(x, res);
                res
            }
        }
    }
}

fn split_into_abundant(x: u64, checker: &mut AbundanceChecker) -> Option<(u64, u64)> {
    for i in 1..=(x / 2) {
        if checker.abundant(i) {
            let counterpart = x - i;
            if checker.abundant(counterpart) {
                return Some((i, counterpart));
            }
        }
    }
    None
}

pub fn solve() -> u64 {
    let mut a = AbundanceChecker::new();

    (1..=28123).filter(|&i| split_into_abundant(i, &mut a).is_none()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abundant() {
        let mut a = AbundanceChecker::new();
        assert!(!a.abundant(28));
        assert!(!a.abundant(11));
        assert!(a.abundant(12));
    }

    #[test]
    fn test_split_abundant() {
        let mut a = AbundanceChecker::new();

        assert_eq!(split_into_abundant(24, &mut a), Some((12, 12)));
        assert_eq!(split_into_abundant(36, &mut a), Some((12, 24)));
        assert_eq!(split_into_abundant(37, &mut a), None);
    }
}