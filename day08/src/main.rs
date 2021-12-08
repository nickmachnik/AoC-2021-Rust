use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

fn vec_from_str(pattern: &str) -> Vec<char> {
    let mut c = pattern.chars().collect::<Vec<char>>();
    c.sort_unstable();
    c
}

fn vec_from_hs(pattern: HashSet<char>) -> Vec<char> {
    let mut c = pattern.into_iter().collect::<Vec<char>>();
    c.sort_unstable();
    c
}

fn hs_from_str(pattern: &str) -> HashSet<char> {
    pattern.chars().collect::<HashSet<char>>()
}

fn find_pattern_by_intersection(
    v: &Vec<HashSet<char>>,
    other: &HashSet<char>,
    exp_count: usize,
) -> Option<usize> {
    for (ix, pattern) in v.iter().enumerate() {
        if pattern.intersection(other).count() == exp_count {
            return Some(ix);
        }
    }
    None
}

fn find_pattern_by_element(v: &Vec<HashSet<char>>, elem: &char) -> Option<usize> {
    for (ix, pattern) in v.iter().enumerate() {
        if pattern.contains(elem) {
            return Some(ix);
        }
    }
    None
}

fn decode_display_output(usp: &[&str], output: &[&str]) -> u64 {
    let mut two_three_five: Vec<HashSet<char>> = Vec::new();
    let mut zero_six_nine: Vec<HashSet<char>> = Vec::new();
    let mut code: HashMap<Vec<char>, u8> = HashMap::new();
    let mut decoded: HashMap<u8, HashSet<char>> = HashMap::new();
    for pattern in usp {
        match pattern.len() {
            2 => {
                code.insert(vec_from_str(pattern), 1);
                decoded.insert(1, hs_from_str(pattern));
            }
            3 => {
                code.insert(vec_from_str(pattern), 7);
                decoded.insert(7, hs_from_str(pattern));
            }
            4 => {
                code.insert(vec_from_str(pattern), 4);
                decoded.insert(4, hs_from_str(pattern));
            }
            7 => {
                code.insert(vec_from_str(pattern), 8);
                decoded.insert(8, hs_from_str(pattern));
            }
            5 => {
                two_three_five.push(hs_from_str(pattern));
            }
            6 => {
                zero_six_nine.push(hs_from_str(pattern));
            }
            _ => unimplemented!(),
        }
    }

    // find 6
    let ix = find_pattern_by_intersection(&zero_six_nine, decoded.get(&1).unwrap(), 1).unwrap();
    code.insert(vec_from_hs(zero_six_nine.swap_remove(ix)), 6);

    // find 3
    let ix = find_pattern_by_intersection(&two_three_five, decoded.get(&1).unwrap(), 2).unwrap();
    let three_pattern = two_three_five.swap_remove(ix);
    let lower_left_panel: char = decoded
        .get(&8)
        .unwrap()
        .difference(&three_pattern)
        .cloned()
        .collect::<HashSet<char>>()
        .difference(decoded.get(&4).unwrap())
        .cloned()
        .next()
        .unwrap();
    code.insert(vec_from_hs(three_pattern), 3);

    // find 2 and 5
    let ix = find_pattern_by_element(&two_three_five, &lower_left_panel).unwrap();
    code.insert(vec_from_hs(two_three_five.swap_remove(ix)), 2);
    code.insert(vec_from_hs(two_three_five.swap_remove(0)), 5);

    // find 9 and 0
    let ix = find_pattern_by_element(&zero_six_nine, &lower_left_panel).unwrap();
    code.insert(vec_from_hs(zero_six_nine.swap_remove(ix)), 0);
    code.insert(vec_from_hs(zero_six_nine.swap_remove(0)), 9);

    // decode output
    output
        .iter()
        .map(|s| code.get(&vec_from_str(s)).unwrap())
        .rev()
        .enumerate()
        .map(|(e, d)| *d as u64 * 10u64.pow(e as u32))
        .sum::<u64>()
}

fn solve1(file: &str) -> u64 {
    let displays = file.lines().map(|l| {
        l.split(" | ")
            .map(|sl| sl.split_ascii_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
    });
    let mut unique_count: u64 = 0;
    displays.for_each(|v| {
        v[1].iter().for_each(|s| match s.len() {
            2 | 3 | 4 | 7 => unique_count += 1,
            _ => (),
        })
    });
    unique_count
}

fn solve2(file: &str) -> u64 {
    let displays = file.lines().map(|l| {
        l.split(" | ")
            .map(|sl| sl.split_ascii_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
    });
    displays.map(|v| decode_display_output(&v[0], &v[1])).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 61229);
    }
}
