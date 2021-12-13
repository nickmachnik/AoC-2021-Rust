use std::collections::HashSet;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    solve2(include_str!("../data/input.txt"));
}

fn parse_input(file: &str) -> (Vec<(usize, usize)>, Vec<(char, usize)>) {
    let mut dots: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(char, usize)> = Vec::new();
    let mut parse_folds = false;
    for line in file.lines() {
        if line == "" {
            parse_folds = true;
            continue;
        }
        if parse_folds {
            let mut spl = line.split_ascii_whitespace().last().unwrap().split('=');
            let dir = spl.next().unwrap().chars().next().unwrap();
            let loc = spl.next().unwrap().parse::<usize>().unwrap();
            folds.push((dir, loc));
        } else {
            let mut spl = line.split(',').map(|e| e.parse::<usize>().unwrap());
            let x = spl.next().unwrap();
            let y = spl.next().unwrap();
            dots.push((x, y));
        }
    }
    (dots, folds)
}

fn fold(mut dot: (usize, usize), folds: &[(char, usize)]) -> (usize, usize) {
    for (dir, loc) in folds {
        match dir {
            'x' => {
                if dot.0 < *loc {
                    continue;
                }
                dot = (loc - (dot.0 - loc), dot.1)
            }
            'y' => {
                if dot.1 < *loc {
                    continue;
                }
                dot = (dot.0, loc - (dot.1 - loc))
            }
            _ => unimplemented!(),
        }
    }
    dot
}

fn solve1(file: &str) -> usize {
    let (dots, folds) = parse_input(file);
    dots.iter()
        .map(|d| fold(*d, &folds[0..1]))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

fn solve2(file: &str) {
    let (dots, folds) = parse_input(file);
    let folded_dots = dots
        .iter()
        .map(|d| fold(*d, &folds))
        .collect::<HashSet<(usize, usize)>>();
    let mut nrows = 0;
    let mut ncols = 0;
    for (x, y) in folded_dots.iter() {
        if *x > ncols {
            ncols = *x;
        }
        if *y > nrows {
            nrows = *y;
        }
    }
    let mut paper = vec![vec![','; ncols + 1]; nrows + 1];
    for (x, y) in folded_dots.iter() {
        paper[*y][*x] = '#';
    }
    for row in paper {
        println!("{:?}", row.into_iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 17);
    }
}
