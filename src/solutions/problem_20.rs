use solutions::common::factorial;
use solutions::common::ToDigits;

pub fn solve(x: u64) -> u32 {
    let res = factorial(x);
    res.to_individual_digits().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(10), 27);
    }
}
