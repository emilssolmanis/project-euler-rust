use rug::Integer;

pub fn solve(pow: u32) -> u32 {
    let res = Integer::from(Integer::u_pow_u(2, pow));
    res.to_string()
        .as_bytes()
        .iter()
        .map(|&b| b as u32 - 48)
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