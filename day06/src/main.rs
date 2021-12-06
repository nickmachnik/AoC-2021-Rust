const DAYS1: usize = 80;
const DAYS2: usize = 256;

fn main() {
    println!("{}", solve(include_str!("../data/input.txt"), DAYS1));
    println!("{}", solve(include_str!("../data/input.txt"), DAYS2));
}

fn solve(file: &str, n_days: usize) -> u64 {
    let mut counts = [0u64; 9];
    let mut tmp = [0u64; 9];
    file.split(',')
        .for_each(|e| counts[e.parse::<usize>().unwrap()] += 1);
    for _ in 0..n_days {
        for tmp_ix in 0..8 {
            tmp[tmp_ix] = counts[tmp_ix + 1];
        }
        tmp[8] = counts[0];
        tmp[6] += counts[0];
        counts = tmp;
        tmp = counts;
    }
    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve(include_str!("../data/test1.txt"), DAYS1), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(include_str!("../data/test1.txt"), DAYS2), 26984457539);
    }
}
