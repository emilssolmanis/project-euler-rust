use rug::Integer;

fn factorial(x: u64) -> Integer {
    let mut res = Integer::from(1);
    for i in 1..=x {
        res *= Integer::from(i);
    }
    res
}

pub fn solve(rows: u64, cols: u64) -> Integer {
    factorial(rows + cols) / (factorial(rows) * factorial(cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20),
                   Integer::from(Integer::parse("2432902008176640000").unwrap()));
    }

    #[test]
    fn test_it() {
        assert_eq!(solve(1, 1), 2);
        assert_eq!(solve(2, 2), 6);
    }
}
