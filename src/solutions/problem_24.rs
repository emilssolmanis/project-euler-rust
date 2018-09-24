use solutions::common::factorial;
use rug::Integer;

pub fn solve() -> String {
    let mut perm = String::from("");
    let mut elems = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut n = Integer::from(999_999);

    for i in (0..10).rev() {
        let f = factorial(i);
        let idx = (n.clone() / f.clone()).to_usize().unwrap();
        perm.push(elems[idx]);
        elems.remove(idx);
        n %= f;
    }

    perm
}
