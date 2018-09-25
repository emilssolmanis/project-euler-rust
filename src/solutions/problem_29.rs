use std::collections::HashSet;
use rug::Integer;

pub fn solve() -> usize {
    let mut s: HashSet<Integer> = HashSet::new();

    for a in 2..=100 {
        for b in 2..=100 {
            s.insert(Integer::from(Integer::u_pow_u(a, b)));
        }
    }

    s.len()
}
