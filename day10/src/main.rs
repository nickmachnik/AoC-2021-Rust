const PAREN_ERR_SCORE: u64 = 3;
const BRACK_ERR_SCORE: u64 = 57;
const BRACE_ERR_SCORE: u64 = 1197;
const ANGLE_ERR_SCORE: u64 = 25137;

const PAREN_COMPL_SCORE: u64 = 1;
const BRACK_COMPL_SCORE: u64 = 2;
const BRACE_COMPL_SCORE: u64 = 3;
const ANGLE_COMPL_SCORE: u64 = 4;

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

fn get_partner(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        _ => unimplemented!(),
    }
}

fn get_err_score(c: char) -> u64 {
    match c {
        ')' => PAREN_ERR_SCORE,
        '}' => BRACE_ERR_SCORE,
        ']' => BRACK_ERR_SCORE,
        '>' => ANGLE_ERR_SCORE,
        _ => unimplemented!(),
    }
}

fn get_compl_score(c: char) -> u64 {
    match c {
        '(' => PAREN_COMPL_SCORE,
        '{' => BRACE_COMPL_SCORE,
        '[' => BRACK_COMPL_SCORE,
        '<' => ANGLE_COMPL_SCORE,
        _ => unimplemented!(),
    }
}

fn solve1(file: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    let mut total_score: u64 = 0;
    for line in file.lines() {
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                _ => {
                    let other = stack.pop();
                    if other.is_none() || other.unwrap() != get_partner(c) {
                        total_score += get_err_score(c);
                        break;
                    }
                }
            }
        }
        stack.clear();
    }
    total_score
}

fn solve2(file: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();
    for line in file.lines() {
        let mut is_corrupted = false;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                _ => {
                    let other = stack.pop();
                    if other.is_none() || other.unwrap() != get_partner(c) {
                        is_corrupted = true;
                        break;
                    }
                }
            }
        }
        if is_corrupted {
            stack.clear();
        } else {
            let mut score: u64 = 0;
            while let Some(c) = stack.pop() {
                score *= 5;
                score += get_compl_score(c);
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 288957);
    }
}
