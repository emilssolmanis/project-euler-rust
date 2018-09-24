use super::common::Fibo;

pub fn solve(num_digits: usize) -> usize {
    Fibo::new()
        .enumerate()
        .find(|(_, number)| number.to_string().len() >= num_digits)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(3), 12);
    }
}