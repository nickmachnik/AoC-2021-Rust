use std::collections::{HashMap, HashSet};

struct Board {
    fields: HashMap<u64, (usize, usize)>,
    row_counts: HashMap<usize, usize>,
    col_counts: HashMap<usize, usize>,
}

impl Board {
    fn from_lines<'a, I>(lines: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut res = Board {
            fields: HashMap::new(),
            row_counts: HashMap::new(),
            col_counts: HashMap::new(),
        };
        let mut nrows = 0;
        for (row, line) in lines.enumerate() {
            if line == "" {
                break;
            }
            nrows += 1;
            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(col, e)| {
                    res.fields.insert(e.parse::<u64>().unwrap(), (row, col));
                });
        }
        if nrows == 0 {
            None
        } else {
            let ncols = res.fields.len() / nrows;
            (0..nrows).for_each(|r| {
                res.row_counts.insert(r, ncols);
            });
            (0..ncols).for_each(|c| {
                res.col_counts.insert(c, nrows);
            });
            Some(res)
        }
    }

    fn is_bingo(&mut self, new_val: u64) -> bool {
        if let Some((row, col)) = self.fields.remove(&new_val) {
            let rc = self.row_counts.get_mut(&row).unwrap();
            if *rc == 1 {
                return true;
            } else {
                *rc -= 1
            }
            let cc = self.col_counts.get_mut(&col).unwrap();
            if *cc == 1 {
                return true;
            } else {
                *cc -= 1
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u64 {
        self.fields.keys().sum()
    }
}

fn load_bingo_data(file: &str) -> (Vec<u64>, Vec<Board>) {
    let mut lines = file.lines();
    let bingo_nums: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    lines.next();
    let mut boards = Vec::new();
    while let Some(board) = Board::from_lines(&mut lines) {
        boards.push(board);
    }
    (bingo_nums, boards)
}

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

fn solve1(file: &str) -> u64 {
    let (bingo_nums, mut boards) = load_bingo_data(file);
    for num in bingo_nums {
        for board in &mut boards {
            if board.is_bingo(num) {
                return num * board.sum_unmarked();
            }
        }
    }
    0
}

fn solve2(file: &str) -> u64 {
    let (bingo_nums, mut boards) = load_bingo_data(file);
    let mut remaining_indices: HashSet<usize> = (0..boards.len()).collect();
    for num in bingo_nums {
        let mut ix = 0;
        for board in &mut boards {
            if remaining_indices.contains(&ix) && board.is_bingo(num) {
                if remaining_indices.len() == 1 {
                    return num * board.sum_unmarked();
                } else {
                    remaining_indices.remove(&ix);
                }
            }
            ix += 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 1924);
    }
}
