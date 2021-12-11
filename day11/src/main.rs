use std::collections::HashSet;

const RADIX: u32 = 10;
const NROUNDS1: usize = 100;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

struct OctoMap {
    map: Vec<Vec<u8>>,
    nrows: usize,
    ncols: usize,
    flashing: HashSet<(usize, usize)>,
}

impl OctoMap {
    fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(RADIX).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Self {
            nrows: map.len(),
            ncols: map[0].len(),
            map: map,
            flashing: HashSet::new(),
        }
    }

    fn in_bounds(&self, ix: (usize, usize)) -> bool {
        if ix.0 < self.nrows && ix.1 < self.ncols {
            return true;
        }
        false
    }

    fn get_neighbor_ixs(&self, ix: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        let (rix, cix) = ix;
        // left
        if cix > 0 {
            res.push((rix, cix - 1));
        }
        // above
        if rix > 0 {
            res.push((rix - 1, cix));
        }
        // right
        if cix < self.ncols - 1 {
            res.push((rix, cix + 1))
        }
        // below
        if rix < self.nrows - 1 {
            res.push((rix + 1, cix))
        }
        // upper left
        if rix > 0 && cix > 0 {
            res.push((rix - 1, cix - 1))
        }
        // upper right
        if rix > 0 && cix < self.ncols - 1 {
            res.push((rix - 1, cix + 1))
        }
        // lower left
        if rix < self.nrows - 1 && cix > 0 {
            res.push((rix + 1, cix - 1))
        }
        // lower right
        if rix < self.nrows - 1 && cix < self.ncols - 1 {
            res.push((rix + 1, cix + 1))
        }
        res
    }

    fn increment_all(&mut self) {
        let mut rix = 0;
        for row in &mut self.map {
            let mut cix = 0;
            for val in row {
                if *val == 9 {
                    self.flashing.insert((rix, cix));
                }
                *val += 1;
                cix += 1
            }
            rix += 1;
        }
    }

    fn increment_at(&mut self, ix: (usize, usize)) -> Option<u8> {
        if self.in_bounds(ix) {
            self.map[ix.0][ix.1] += 1;
            return Some(self.map[ix.0][ix.1]);
        }
        return None;
    }

    fn flash(&mut self) -> u64 {
        let mut to_flash: Vec<(usize, usize)> = self.flashing.iter().cloned().collect();
        while let Some(curr) = to_flash.pop() {
            for neighbor in self.get_neighbor_ixs(curr) {
                if self.flashing.contains(&neighbor) {
                    continue;
                }
                if self.increment_at(neighbor).unwrap() > 9 {
                    self.flashing.insert(neighbor);
                    to_flash.push(neighbor);
                };
            }
        }
        let n_flashing = self.flashing.len() as u64;
        let m = &mut self.map;
        for ix in self.flashing.drain() {
            m[ix.0][ix.1] = 0;
        }
        n_flashing
    }
}

fn solve1(file: &str) -> u64 {
    let mut octos = OctoMap::from_str(file);
    let mut n_flashes: u64 = 0;
    for _ in 0..NROUNDS1 {
        octos.increment_all();
        n_flashes += octos.flash();
    }
    n_flashes
}

fn solve2(file: &str) -> u64 {
    let mut octos = OctoMap::from_str(file);
    let max_flash: u64 = (octos.ncols * octos.nrows) as u64;
    let mut round = 1;
    octos.increment_all();
    while max_flash != octos.flash() {
        round += 1;
        octos.increment_all();
    }
    round
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 195);
    }
}
