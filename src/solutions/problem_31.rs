const COINS: &[u64; 8] = &[1, 2, 5, 10, 20, 50, 100, 200];

fn num_partitions(sum: u64, nominals: &[u64]) -> u64 {
    if sum == 0 || nominals.is_empty() {
        return 0;
    }

    let mut res = 0;
    for (i, &coin) in nominals.iter().enumerate() {
        if coin == sum {
            res += 1;
        }
        let new_sum = if coin > sum { 0 } else { sum - coin };
        res += num_partitions(new_sum, &nominals[i..nominals.len()]);
    }

    res
}

pub fn solve(sum: u64) -> u64 {
    num_partitions(sum, COINS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(num_partitions(1, &[1]), 1);
        assert_eq!(num_partitions(2, &[1]), 1);
        assert_eq!(num_partitions(2, &[1, 2]), 2);
        assert_eq!(num_partitions(3, &[1, 2]), 2);
        assert_eq!(num_partitions(4, &[1, 2]), 3);
        assert_eq!(num_partitions(5, COINS), 4);
    }
}
