pub fn solve(up_to: u32) -> u32 {
    (0..up_to).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_1() {
        assert_eq!(super::solve(10), 23);
    }
}