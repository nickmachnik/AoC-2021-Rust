use std::collections::HashMap;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

#[derive(Debug)]
struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Line {
    fn from_input_line(line: &str) -> Self {
        let coords: Vec<isize> = line
            .split(" -> ")
            .map(|s| s.split(','))
            .flatten()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        Self {
            x1: coords[0],
            y1: coords[1],
            x2: coords[2],
            y2: coords[3],
        }
    }

    fn ord_x(&self) -> (isize, isize) {
        if self.x1 < self.x2 {
            (self.x1, self.x2)
        } else {
            (self.x2, self.x1)
        }
    }

    fn ord_y(&self) -> (isize, isize) {
        if self.y1 < self.y2 {
            (self.y1, self.y2)
        } else {
            (self.y2, self.y1)
        }
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn slope(&self) -> (isize, isize) {
        // we know that there are only diagonals,
        // so no need to be accurate here
        (
            if self.y1 > self.y2 { -1 } else { 1 },
            if self.x1 > self.x2 { -1 } else { 1 },
        )
    }

    fn coordinates(&self) -> Vec<(isize, isize)> {
        if self.is_horizontal() {
            let (x1, x2) = self.ord_x();
            (x1..x2 + 1).map(|x| (x, self.y1)).collect()
        } else if self.is_vertical() {
            let (y1, y2) = self.ord_y();
            (y1..y2 + 1).map(|y| (self.x1, y)).collect()
        } else {
            let (y_change, x_change) = self.slope();
            let mut res: Vec<(isize, isize)> = Vec::new();
            res.push((self.x1, self.y1));
            let mut curr_x = self.x1;
            let mut curr_y = self.y1;
            while (curr_x, curr_y) != (self.x2, self.y2) {
                curr_x += x_change;
                curr_y += y_change;
                res.push((curr_x, curr_y));
            }
            res
        }
    }
}

fn solve1(file: &str) -> usize {
    let mut line_coords: HashMap<(isize, isize), usize> = HashMap::new();
    file.lines()
        .map(|l| Line::from_input_line(l))
        .for_each(|line| {
            if line.is_horizontal() || line.is_vertical() {
                line.coordinates().iter().for_each(|coord| {
                    *line_coords.entry(*coord).or_insert(0) += 1;
                })
            }
        });
    line_coords.values().filter(|v| **v >= 2).count()
}

fn solve2(file: &str) -> usize {
    let mut line_coords: HashMap<(isize, isize), usize> = HashMap::new();
    file.lines()
        .map(|l| Line::from_input_line(l))
        .for_each(|line| {
            line.coordinates().iter().for_each(|coord| {
                *line_coords.entry(*coord).or_insert(0) += 1;
            })
        });
    line_coords.values().filter(|v| **v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 12);
    }
}
