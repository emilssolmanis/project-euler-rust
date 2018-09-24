use rug::Integer;
use super::common::ToDigits;

pub fn solve(pow: u32) -> u32 {
    let res = Integer::from(Integer::u_pow_u(2, pow));
    res.to_individual_digits()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(15), 26);
    }
}