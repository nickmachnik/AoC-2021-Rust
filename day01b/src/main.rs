fn main() {
    println!("{}", solve(include_str!("../data/input.txt")));
}

fn solve(file: &str) -> usize {
    let depths: Vec<usize> = file.lines().map(|e| e.parse::<usize>().unwrap()).collect();
    let mut incr: usize = 0;
    for pos in 0..(depths.len() - 3) {
        if depths[pos] < depths[pos + 3] {
            incr += 1;
        }
    }
    incr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(solve(include_str!("../data/test1.txt")), 5);
    }
}
