extern crate rust_fuckery;

use rust_fuckery::solutions;

fn main() {
    println!("problem 1: {}", solutions::problem_1::solve(1000));
    println!("problem 2: {}", solutions::problem_2::solve(4_000_000));
}
