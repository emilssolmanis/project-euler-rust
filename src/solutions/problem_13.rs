use std::fs::File;
use std::io::Read;
use rug::Integer;
use rug::Assign;

pub fn solve(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("failed to read");

    contents.split_whitespace()
        .map(|s| {
            let mut i = Integer::new();
            i.assign(Integer::parse(s).expect("failed to parse int"));
            i
        })
        .sum::<Integer>()
        .to_string()[..10]
        .to_string()
}
