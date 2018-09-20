extern crate rust_fuckery;

use rust_fuckery::solutions;

fn main() {
    println!("problem 1: {}", solutions::problem_1::solve(1000));
    println!("problem 2: {}", solutions::problem_2::solve(4_000_000));
    println!("problem 3: {}", solutions::problem_3::solve(600851475143));
    println!("problem 4: {}", solutions::problem_4::solve());
    println!("problem 5: {}", solutions::problem_5::solve(20));
}
