fn main() {
    println!("{}", solve(include_str!("../data/input.txt")));
}

fn solve(file: &str) -> usize {
    let mut incr: usize = 0;
    let mut last_num: usize = 0;
    for (lc, line) in file.lines().enumerate() {
        let curr_num = line.parse::<usize>().unwrap();
        if (lc > 0) && (curr_num > last_num) {
            incr += 1;
        }
        last_num = curr_num;
    }
    incr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(solve(include_str!("../data/test1.txt")), 7);
    }
}
