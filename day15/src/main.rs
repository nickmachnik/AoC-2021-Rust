use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const RADIX: u32 = 10;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    cum_risk: usize,
    coords: (usize, usize),
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cum_risk
            .cmp(&self.cum_risk)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    risks: Vec<Vec<usize>>,
    nrows: usize,
    ncols: usize,
    cum_risk: Vec<Vec<usize>>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let risks = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(RADIX).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Self {
            nrows: risks.len(),
            ncols: risks[0].len(),
            risks: risks,
            cum_risk: Vec::new(),
        }
    }

    fn incr_row_counts(&self, row: &[usize]) -> Vec<usize> {
        let mut res = Vec::with_capacity(row.len());
        for v in row {
            if *v == 9 {
                res.push(1);
            } else {
                res.push(v + 1);
            }
        }
        res
    }

    fn fill_row(&mut self, rix: usize) {
        let segment_len = self.ncols;
        let mut prev_segment = &self.risks[rix][0..segment_len];
        for i in 1..5 {
            let new_segment = self.incr_row_counts(prev_segment);
            let _ = &self.risks[rix].extend(new_segment);
            prev_segment = &self.risks[rix][segment_len * i..segment_len * (i + 1)];
        }
    }

    fn from_str_extended(input: &str) -> Self {
        let mut res = Self::from_str(input);
        // fill first column
        for rix in res.nrows..res.nrows * 5 {
            res.risks
                .push(res.incr_row_counts(&res.risks[rix - res.nrows]));
        }
        // fill rows
        for rix in 0..res.nrows * 5 {
            res.fill_row(rix);
        }
        res.nrows *= 5;
        res.ncols *= 5;
        res
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let x = pos.0 as isize;
        let y = pos.1 as isize;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter(|(a, b)| {
                *a >= 0 && *a <= self.ncols as isize - 1 && *b >= 0 && *b <= self.nrows as isize - 1
            })
            .map(|(a, b)| (*a as usize, *b as usize))
            .collect()
    }

    fn risk_at(&self, pos: (usize, usize)) -> usize {
        self.risks[pos.0][pos.1]
    }

    fn get_cum_risk_at(&self, pos: (usize, usize)) -> usize {
        self.cum_risk[pos.0][pos.1]
    }
    fn set_cum_risk_at(&mut self, pos: (usize, usize), val: usize) {
        self.cum_risk[pos.0][pos.1] = val;
    }

    fn calc_cum_risk(&mut self) {
        let mut to_visit = HashSet::new();
        let mut queue = BinaryHeap::new();
        self.cum_risk = vec![vec![0usize; self.ncols]; self.nrows];
        for rix in 0..self.nrows {
            for cix in 0..self.ncols {
                self.cum_risk[rix][cix] = 9 * (rix + cix);
                to_visit.insert((rix, cix));
            }
        }
        queue.push(Pos {
            coords: (0, 0),
            cum_risk: 0,
        });
        while let Some(Pos { coords, cum_risk }) = queue.pop() {
            if cum_risk > self.get_cum_risk_at(coords) {
                continue;
            }
            for neighbor in self.neighbors(coords) {
                if !(to_visit.contains(&neighbor)) {
                    continue;
                }
                let enter_risk = cum_risk + self.risk_at(neighbor);
                if self.get_cum_risk_at(neighbor) > enter_risk {
                    self.set_cum_risk_at(neighbor, enter_risk);
                    queue.push(Pos {
                        coords: neighbor,
                        cum_risk: enter_risk,
                    });
                }
            }
            to_visit.remove(&coords);
        }
    }

    fn lowest_risk_path_sum(&mut self) -> usize {
        self.calc_cum_risk();
        self.get_cum_risk_at((self.nrows - 1, self.ncols - 1))
    }
}

fn solve1(file: &str) -> usize {
    let mut map = Map::from_str(file);
    map.lowest_risk_path_sum()
}

fn solve2(file: &str) -> usize {
    let mut map = Map::from_str_extended(file);
    map.lowest_risk_path_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 315);
    }
}
