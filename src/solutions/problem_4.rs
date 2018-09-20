struct Product {
    i: u32,
    j: u32,
}

impl Product {
    fn new() -> Product {
        Product { i: 999, j: 1000 }
    }
}

impl Iterator for Product {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.j = self.j - 1;

        if self.j < 100 {
            self.j = 999;
            self.i = self.i - 1;

            if self.i < 100 {
                return None;
            }
        }

        Some(self.i * self.j)
    }
}

fn is_palindrome(i: u32) -> bool {
    let s = i.to_string();
    let s = s.as_bytes();

    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }

    true
}

pub fn solve() -> u32 {
    Product::new().filter(|&p| is_palindrome(p)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let res: Vec<u32> = Product::new().take(4).collect();
        assert_eq!(res, vec![999 * 999, 999 * 998, 999 * 997, 999 * 996]);
    }

    #[test]
    fn test_more() {
        let mut res: Vec<u32> = Product::new().collect();
        res.reverse();
        let res = res;

        assert_eq!(res[..4].to_vec(), vec![100 * 100, 100 * 101, 100 * 102, 100 * 103]);
    }

    #[test]
    fn palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(1234321));

        assert!(!is_palindrome(9001));
    }
}