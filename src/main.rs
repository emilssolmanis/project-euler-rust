extern crate rust_fuckery;

use rust_fuckery::solutions;

fn main() {
    println!("problem 1: {}", solutions::problem_1::solve(1000));
    println!("problem 2: {}", solutions::problem_2::solve(4_000_000));
    println!("problem 3: {}", solutions::problem_3::solve(600851475143));
    println!("problem 4: {}", solutions::problem_4::solve());
    println!("problem 5: {}", solutions::problem_5::solve(20));
    println!("problem 6: {}", solutions::problem_6::solve(100));
    println!("problem 7: {}", solutions::problem_7::solve(10_000));
    println!("problem 8: {}", solutions::problem_8::solve(13));
    println!("problem 9: {}", solutions::problem_9::solve());
    println!("problem 10: {}", solutions::problem_10::solve(2_000_000));
    println!("problem 11: {}", solutions::problem_11::solve());
}
