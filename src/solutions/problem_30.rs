use rug::Integer;
use solutions::common::ToDigits;

pub fn solve() -> u32 {
    // Because the number we're getting is a sum, the max sum we can get is 9^5 + 9^5 + 9^5 ... = n * 9^5 = 59049n
    // 10 = 10^1, 100 = 10^2. Obviously, 10^n grows faster than 9^5n. We can get the equality point (and therefore, the
    // sum's upper bound) by solving 10^n = 59049n for n.
    // n ~ 0.0000169357; n ~ 5.51257
    // Therefore, ONLY numbers smaller than 10^5.51257 can be written as the sum of their digits. The precise number is
    // k * 9^5, but we can get k as the number of digits in 10^5.51257, which is obviously 6.

    let mut res = 0;

    for i in 7..=(6 * 9u32.pow(5)) {
        let s: u32 = Integer::from(i).to_individual_digits().iter().map(|d| d.pow(5)).sum();
        if i == s {
            res += i;
        }
    }

    res
}
