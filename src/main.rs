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
    println!("problem 12: {}", solutions::problem_12::solve(500));
    println!("problem 13: {}", solutions::problem_13::solve("resources/problem_13.dat"));
    println!("problem 14: {}", solutions::problem_14::solve());
    println!("problem 15: {}", solutions::problem_15::solve(20, 20));
    println!("problem 16: {}", solutions::problem_16::solve(1000));
}
