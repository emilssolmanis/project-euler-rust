pub fn solve(n: u32) -> i32 {
//    top_left = 1
//    bottom_right = 1
//    # init to -1 because in the first iteration both diagonals get added
//    res = -1
//    for _ in range(1001):
//        if top_left == bottom_right:
//            res += top_left**2 - (top_left - 1)
//            res += top_left**2
//            top_left += 2
//        else:
//            res += bottom_right**2 + bottom_right + 1
//            res += (bottom_right + 1)**2 + 1
//            bottom_right += 2
//     return res

    let mut top_left: i32 = 1;
    let mut bottom_right: i32 = 1;
    let mut res: i32 = -1;

    for _ in 0..n {
        if top_left == bottom_right {
            res += top_left.pow(2) - (top_left - 1);
            res += top_left.pow(2);
            top_left += 2;
        } else {
            res += bottom_right.pow(2) + bottom_right + 1;
            res += (bottom_right + 1).pow(2) + 1;
            bottom_right += 2
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(solve(5), 101);
    }
}