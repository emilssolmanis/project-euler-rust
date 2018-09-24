use super::common::Fibo;
use rug::Integer;

pub fn solve(up_to: u32) -> u32 {
    Fibo::new()
        .take_while(|i| i < &up_to)
        .filter(|i| i.mod_u(2) == 0)
        .sum::<Integer>()
        .to_u32()
        .unwrap()
}
