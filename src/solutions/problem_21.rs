use solutions::common::Factors;
use std::collections::HashSet;

fn get_amicable(x: u64) -> Option<u64> {
    let pair: u64 = x.factors().iter().sum();
    let inverse: u64 = pair.factors().iter().sum();

    if x == inverse && x != pair {
        Some(pair)
    } else {
        None
    }
}

fn get_amicables_under(under: u64) -> HashSet<u64> {
    let mut found_amicables: HashSet<u64> = HashSet::new();

    for i in 1..under {
        if !found_amicables.contains(&i) {
            if let Some(x) = get_amicable(i) {
                found_amicables.insert(i);
                found_amicables.insert(x);
            }
        }
    }

    found_amicables
}

pub fn solve() -> u64 {
    get_amicables_under(10_000).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amicable() {
        assert_eq!(get_amicable(220), Some(284));
        assert_eq!(get_amicable(113), None);
        assert_eq!(get_amicable(6), None);
        assert_eq!(get_amicable(28), None);
        assert_eq!(get_amicable(496), None);
    }

    #[test]
    fn test_get_amicables_1000() {
        let mut expected: HashSet<u64> = HashSet::new();
        expected.extend(vec![220, 284]);

        assert_eq!(get_amicables_under(1000), expected);
    }

    #[test]
    fn get_amicables_10000() {
        let mut expected: HashSet<u64> = HashSet::new();
        expected.extend(vec![220, 284, 1184, 1210, 2620, 2924, 5020, 5564, 6232, 6368]);

        assert_eq!(get_amicables_under(10_000), expected);
    }
}