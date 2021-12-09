use std::collections::HashSet;

const RADIX: u32 = 10;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

struct TopoMap {
    map: Vec<Vec<u32>>,
    nrows: usize,
    ncols: usize,
    low_points: Option<Vec<(usize, usize)>>,
    basin_sizes: Option<Vec<u32>>,
}

impl TopoMap {
    fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(RADIX).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        Self {
            nrows: map.len(),
            ncols: map[0].len(),
            map: map,
            low_points: None,
            basin_sizes: None,
        }
    }

    fn in_bounds(&self, rix: usize, cix: usize) -> bool {
        if rix < self.nrows && cix < self.ncols {
            return true;
        }
        false
    }

    fn get_height_at(&self, rix: usize, cix: usize) -> Option<u32> {
        if self.in_bounds(rix, cix) {
            return Some(self.map[rix][cix]);
        }
        None
    }

    fn get_neighbor_ixs(&self, rix: usize, cix: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        if cix > 0 {
            res.push((rix, cix - 1));
        }
        if rix > 0 {
            res.push((rix - 1, cix));
        }
        if cix < self.ncols - 1 {
            res.push((rix, cix + 1))
        }
        if rix < self.nrows - 1 {
            res.push((rix + 1, cix))
        }
        res
    }

    fn find_low_points(&mut self) {
        let mut res: Vec<(usize, usize)> = Vec::new();
        for rix in 0..self.nrows {
            for cix in 0..self.ncols {
                let curr_height = self.get_height_at(rix, cix).unwrap();
                let n_leq_neighbors = self
                    .get_neighbor_ixs(rix, cix)
                    .iter()
                    .filter(|(r, c)| self.get_height_at(*r, *c).unwrap() <= curr_height)
                    .count();
                if n_leq_neighbors == 0 {
                    res.push((rix, cix));
                }
            }
        }
        self.low_points = Some(res);
    }

    fn get_low_points(&self) -> &Option<Vec<(usize, usize)>> {
        &self.low_points
    }

    fn find_basin_sizes(&mut self) {
        let mut res: Vec<u32> = Vec::new();
        if self.get_low_points().is_none() {
            self.find_low_points();
        }
        for lp in self.get_low_points().as_ref().unwrap() {
            res.push(self.measure_basin_size(lp));
        }
        self.basin_sizes = Some(res);
    }

    fn measure_basin_size(&self, low_point: &(usize, usize)) -> u32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: Vec<(usize, usize)> = vec![*low_point];
        while let Some(p) = to_visit.pop() {
            visited.insert(p);
            for neighbor in self.get_neighbor_ixs(p.0, p.1) {
                if self.get_height_at(neighbor.0, neighbor.1).unwrap() < 9
                    && !visited.contains(&neighbor)
                {
                    to_visit.push(neighbor);
                }
            }
        }
        visited.len() as u32
    }
}

fn solve1(file: &str) -> u32 {
    let mut risk_sum: u32 = 0;
    let mut map = TopoMap::from_str(file);
    map.find_low_points();
    for (rix, cix) in map.get_low_points().as_ref().unwrap() {
        risk_sum += map.get_height_at(*rix, *cix).unwrap() + 1
    }
    risk_sum
}

fn solve2(file: &str) -> u64 {
    let mut map = TopoMap::from_str(file);
    map.find_basin_sizes();
    let mut basin_sizes = map.basin_sizes.unwrap();
    basin_sizes.sort_unstable();
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .fold(1, |a, b| a as u64 * *b as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 1134);
    }
}
