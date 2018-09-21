use std::fs::File;
use std::io::Read;
use std::cmp::max;

struct Triangle {
    rows: Vec<Vec<u64>>
}

impl Triangle {
    fn collapse_row(&mut self) {
        let bottom_row = self.rows.pop().unwrap();
        let penultimate_row = self.rows.last_mut().unwrap();

        for (idx, num) in penultimate_row.iter_mut().enumerate() {
            *num = *num + max(bottom_row[idx], bottom_row[idx + 1]);
        };
    }

    pub fn max_path(&mut self) -> u64 {
        while self.rows.len() > 1 {
            self.collapse_row();
        }

        return self.rows[0][0];
    }
}

fn parse_row(row: &str) -> Vec<u64> {
    row.split(' ')
        .map(|s: &str| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn parse(filename: &str) -> Triangle {
    let mut contents = String::new();
    File::open(filename)
        .expect("failed to open file")
        .read_to_string(&mut contents)
        .expect("failed to read stuff");

    Triangle {
        rows: contents.trim()
            .split('\n')
            .map(parse_row)
            .collect::<Vec<Vec<u64>>>()
    }
}

pub fn solve(filename: &str) -> u64 {
    let mut t = parse(filename);
    t.max_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let mut t = Triangle {
            rows: vec![
                vec![3],
                vec![7, 4],
                vec![2, 4, 6],
                vec![8, 5, 9, 3],
            ]
        };
        assert_eq!(t.max_path(), 23);
    }
}
