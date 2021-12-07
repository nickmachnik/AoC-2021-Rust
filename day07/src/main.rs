fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

fn get_median(v: &[i64]) -> f64 {
    let size = v.len();
    match size {
        even if even % 2 == 0 => (v[even / 2 - 1] + v[even / 2 - 1]) as f64 / 2.,
        odd => v[odd / 2] as f64,
    }
}

fn solve1(file: &str) -> u64 {
    let mut positions = file
        .split(',')
        .map(|e| e.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    positions.sort_unstable();
    let m = get_median(&positions);
    positions.iter().map(|e| (*e as f64 - m).abs() as u64).sum()
}

fn solve2(file: &str) -> i64 {
    let positions = file
        .split(',')
        .map(|e| e.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mean: i64 = positions.iter().sum::<i64>() / positions.len() as i64;
    let a: i64 = positions
        .iter()
        .map(|e| sum_first_n_ints((*e as i64 - mean as i64).abs()))
        .sum();
    let b = positions
        .iter()
        .map(|e| sum_first_n_ints((*e as i64 - (mean + 1) as i64).abs()))
        .sum();
    a.min(b)
}

fn sum_first_n_ints(n: i64) -> i64 {
    ((n * n) + n) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 168);
    }
}
