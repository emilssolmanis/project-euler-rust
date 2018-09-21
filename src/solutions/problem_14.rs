pub struct Collatz {
    i: u64
}

impl Collatz {
    fn new(start: u64) -> Collatz {
        Collatz { i: start }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let res = self.i;

        if res == 0 {
            return None;
        } else if res == 1 {
            self.i = 0
        } else if self.i % 2 == 0 {
            self.i /= 2;
        } else {
            self.i = self.i * 3 + 1;
        }

        Some(res)
    }
}

pub fn solve() -> u64 {
    let mut start = 0;
    let mut max_length = 0;

    for i in 1..1_000_000 {
        let len = Collatz::new(i).count();
        if len > max_length {
            start = i;
            max_length = len;
        }
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let res: Vec<u64> = Collatz::new(13).collect();
        assert_eq!(res, vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
    }
}
