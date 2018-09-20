struct TriangleNumbers {
    i: u64,
    curr: u64,
}

impl TriangleNumbers {
    fn new() -> TriangleNumbers {
        TriangleNumbers {
            i: 1,
            curr: 0,
        }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.curr += self.i;
        self.i += 1;
        Some(self.curr)
    }
}

fn num_factors(i: u64) -> u64 {
    (1..i).filter(|n| i % n == 0).count() as u64 + 1
}

pub fn solve(num_factors_needed: u64) -> u64 {
    TriangleNumbers::new().find(|&t| num_factors(t) > num_factors_needed).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let res: Vec<u64> = TriangleNumbers::new().take(10).collect();
        assert_eq!(res, vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55])
    }

    #[test]
    fn factor_func() {
        assert_eq!(num_factors(2), 2);
        assert_eq!(num_factors(3), 2);
        assert_eq!(num_factors(4), 3);
        assert_eq!(num_factors(10), 4);
        assert_eq!(num_factors(28), 6);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(5), 28);
    }
}
