struct Fibo {
    prev: u32,
    curr: u32,
}

impl Fibo {
    fn new() -> Fibo {
        Fibo {
            prev: 0,
            curr: 1,
        }
    }
}

impl Iterator for Fibo {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let existing_prev = self.prev;
        self.prev = self.curr;
        self.curr += existing_prev;
        Some(existing_prev)
    }
}

pub fn solve(up_to: u32) -> u32 {
    Fibo::new().take_while(|&i| i < up_to).filter(|i| i % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_2() {
        let res: Vec<u32> = super::Fibo::new().take_while(|&i| i < 10u32).collect();

        assert_eq!(
            res,
            vec![0, 1, 1, 2, 3, 5, 8]);
    }
}
