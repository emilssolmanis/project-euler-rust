pub fn solve() -> i32 {
    for i in 1..1000_i32 {
        for j in i..1000 {
            let k = 1000 - i - j;
            if i.pow(2) + j.pow(2) == k.pow(2) {
                return i * j * k;
            }
        }
    }
    panic!("not found");
}
