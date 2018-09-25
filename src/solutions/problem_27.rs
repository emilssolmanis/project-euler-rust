use super::common::PrimeChecker;

fn compoot(a: i32, b: i32, n: i32) -> i32 {
    n.pow(2) + a * n + b
}

fn length_of_prime_seq(a: i32, b: i32, pc: &PrimeChecker) -> i32 {
    for i in 0.. {
        let res = compoot(a, b, i);
        if !pc.prime(res as u64) {
            return i;
        }
    }

    0
}

pub fn solve() -> i32 {
    let mut pc = PrimeChecker::new(100_000_000);
    let mut max_len = 0;
    let mut res = 0;

    for a in -999..1000 {
        for b in -1000..=1000 {
            let l = length_of_prime_seq(a, b, &mut pc);
            if l > max_len {
                max_len = l;
                res = a * b;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let pc = PrimeChecker::new(10_000_000);

        assert_eq!(length_of_prime_seq(1, 41, &pc), 40);
        assert_eq!(length_of_prime_seq(-79, 1601, &pc), 80);
    }
}