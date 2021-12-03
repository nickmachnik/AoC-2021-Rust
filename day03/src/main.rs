const RADIX: u32 = 10;

fn main() {
    println!(
        "output part 1: {}",
        solve_part1(include_str!("../data/input.txt"))
    );
    println!(
        "output part 2: {}",
        solve_part2(include_str!("../data/input.txt"))
    );
}

fn parse_into_u8_vecs(file: &str) -> Vec<Vec<u8>> {
    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(RADIX).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn find_most_common_bits(nums: &Vec<Vec<u8>>) -> Vec<u8> {
    let n_half: f64 = nums.len() as f64 / 2.;
    let mut counts: Vec<u64> = vec![0; nums[0].len()];
    nums.iter().for_each(|n| {
        n.iter()
            .enumerate()
            .for_each(|(i, d)| counts[i] += *d as u64)
    });
    counts
        .iter()
        .map(|c| if (*c as f64) >= n_half { 1u8 } else { 0u8 })
        .collect()
}

fn find_most_common_bit_at_pos(nums: &Vec<Vec<u8>>, pos: usize) -> u8 {
    let n_half: f64 = nums.len() as f64 / 2.;
    let mut count: u64 = 0;
    nums.iter().for_each(|n| {
        if n[pos] == 1 {
            count += 1
        }
    });
    if count as f64 >= n_half {
        1
    } else {
        0
    }
}

fn find_least_common_bit_at_pos(nums: &Vec<Vec<u8>>, pos: usize) -> u8 {
    let n_half: f64 = nums.len() as f64 / 2.;
    let mut count: u64 = 0;
    nums.iter().for_each(|n| {
        if n[pos] == 1 {
            count += 1
        }
    });
    if count as f64 >= n_half {
        0
    } else {
        1
    }
}

fn binary_to_decimal<'a, I>(b: I) -> u64
where
    I: Iterator<Item = &'a u8>,
    I: DoubleEndedIterator<Item = &'a u8>,
{
    b.rev()
        .enumerate()
        .map(|(e, d)| (*d as u64) * 2u64.pow(e as u32))
        .sum()
}

fn filter_nums(
    nums: &Vec<Vec<u8>>,
    pos: usize,
    bit_criteria_fn: fn(&Vec<Vec<u8>>, usize) -> u8,
) -> u64 {
    if nums.len() == 1 {
        binary_to_decimal(nums[0].iter())
    } else {
        let filter_val = bit_criteria_fn(&nums, pos);
        filter_nums(
            &nums
                .iter()
                .filter(|v| v[pos] == filter_val)
                .cloned()
                .collect(),
            pos + 1,
            bit_criteria_fn,
        )
    }
}

fn solve_part1(file: &str) -> u64 {
    let most_common_bits = find_most_common_bits(&parse_into_u8_vecs(file));
    let least_common_bits = most_common_bits
        .iter()
        .map(|e| if (*e) == 1 { 0 } else { 1 })
        .collect::<Vec<u8>>();
    let gamma = binary_to_decimal(most_common_bits.iter());
    let epsilon = binary_to_decimal(least_common_bits.iter());
    gamma * epsilon
}

fn solve_part2(file: &str) -> u64 {
    let nums = parse_into_u8_vecs(file);
    filter_nums(&nums, 0, find_most_common_bit_at_pos)
        * filter_nums(&nums, 0, find_least_common_bit_at_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_part1() {
        assert_eq!(solve_part1(include_str!("../data/test1.txt")), 198);
    }

    #[test]
    fn test_cases_part2() {
        assert_eq!(solve_part2(include_str!("../data/test1.txt")), 230);
    }
}
