use std::fs::File;
use std::io::Read;

fn compute_score(name: &str) -> u32 {
    name.chars().map(|b| u32::from(b) - u32::from('A') + 1).sum()
}

fn read_name_list(filename: &str) -> Vec<String> {
    let mut contents = String::new();

    File::open(filename)
        .expect("file should exist")
        .read_to_string(&mut contents)
        .expect("failed to read file");

    contents.split(',').map(|s: &str| s.replace('"', "")).collect()
}

fn score_list(names: Vec<String>) -> u32 {
    let mut res: u32 = 0;

    for (idx, name) in names.iter().enumerate() {
        res += (idx + 1) as u32 * compute_score(name);
    }

    res
}

pub fn solve(filename: &str) -> u32 {
    let mut name_list = read_name_list(filename);
    name_list.sort();

    score_list(name_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_score() {
        assert_eq!(compute_score("COLIN"), 53);
        assert_eq!(compute_score("BOB"), 19);
    }

    #[test]
    fn test_score_list() {
        assert_eq!(score_list(vec![String::from("ALICE"),
                                   String::from("BOB"),
                                   String::from("COLIN")]),
                   1 * compute_score("ALICE")
                       + 2 * compute_score("BOB")
                       + 3 * compute_score("COLIN"));
    }
}
