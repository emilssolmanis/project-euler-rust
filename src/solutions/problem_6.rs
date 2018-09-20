pub fn solve(up_to: u64) -> u64 {
    let sum_of_squares: u64 = (1..(up_to + 1)).map(|i| i.pow(2)).sum();
    let square_of_sums: u64 = (1..(up_to + 1)).sum::<u64>().pow(2);
    square_of_sums - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(10), 2640);
    }
}
