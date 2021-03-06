#![feature(test)]
extern crate test;

fn main() {
    println!(
        "output part 1: {}",
        solve_part1(include_str!("../data/input.txt"))
    );
    println!(
        "output part 2: {}",
        solve_part2(include_str!("../data/input.txt"))
    );
}

fn solve_part1(file: &str) -> u64 {
    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;
    file.lines().for_each(|l| {
        let mut lsplit = l.split(' ');
        match lsplit.next().unwrap() {
            "up" => depth -= lsplit.next().unwrap().parse::<u64>().unwrap(),
            "down" => depth += lsplit.next().unwrap().parse::<u64>().unwrap(),
            "forward" => horizontal += lsplit.next().unwrap().parse::<u64>().unwrap(),
            _ => panic!("Unexpected movement code!"),
        }
    });
    horizontal * depth
}

fn solve_part2(file: &str) -> u64 {
    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;
    let mut aim: u64 = 0;
    file.lines().for_each(|l| {
        let mut lsplit = l.split(' ');
        let instr = lsplit.next().unwrap();
        let val = lsplit.next().unwrap().parse::<u64>().unwrap();
        match instr {
            "up" => aim -= val,
            "down" => aim += val,
            "forward" => {
                horizontal += val;
                depth += aim * val;
            }
            _ => panic!("Unexpected movement code!"),
        }
    });
    horizontal * depth
}

fn solve_part2_fold(file: &str) -> u64 {
    let (h, d, _) = file.lines().map(|l| l.split_once(" ").unwrap()).fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (instr, val)| {
            let pval = val.parse::<u64>().unwrap();
            match instr {
                "up" => (horizontal, depth, aim - pval),
                "down" => (horizontal, depth, aim + pval),
                "forward" => (horizontal + pval, depth + aim * pval, aim),
                _ => panic!("Unexpected movement code!"),
            }
        },
    );
    h * d
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_cases_part1() {
        assert_eq!(solve_part1(include_str!("../data/test1.txt")), 150);
    }

    #[test]
    fn test_cases_part2() {
        assert_eq!(solve_part2(include_str!("../data/test1.txt")), 900);
        assert_eq!(solve_part2_fold(include_str!("../data/test1.txt")), 900);
    }

    #[bench]
    fn bench_fold(b: &mut Bencher) {
        b.iter(|| solve_part2_fold(include_str!("../data/test1.txt")));
    }

    #[bench]
    fn bench_for_each(b: &mut Bencher) {
        b.iter(|| solve_part2(include_str!("../data/test1.txt")));
    }
}
