use rug::Integer;

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn order(a: u64, n: u64) -> u64 {
    if gcd(a, n) != 1 {
        panic!("gcd is not 1");
    }

    let n_int = Integer::from(n);
    let mut i = Integer::from(1);
    while Integer::from(a).pow_mod(&i, &n_int).unwrap() != 1 {
        i += 1
    }

    i.to_u64().unwrap()
}

fn reciprocal_length(denominator: u64) -> u64 {
    let mut d = denominator;

    while d % 2 == 0 {
        d /= 2;
    }

    while d % 5 == 0 {
        d /= 5;
    }

    if d == 1 { 0 } else { order(10, d) }
}

pub fn solve(up_to: u64) -> u64 {
    (1..up_to)
        .map(|d| (d, reciprocal_length(d)))
        .max_by(|&(_, l1), &(_, l2)| l1.cmp(&l2))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        assert_eq!(order(1, 7), 1);
        assert_eq!(order(2, 7), 3);
        assert_eq!(order(3, 7), 6);
        assert_eq!(order(4, 7), 3);
        assert_eq!(order(5, 7), 6);
        assert_eq!(order(6, 7), 2);
        assert_eq!(order(8, 7), 1);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 7);
    }
}
