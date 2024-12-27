use aoc2024::AoCResult;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn valid_levels(levels: &Vec<i32>) -> bool {
    let diff = levels.windows(2).map(|v| v[1] - v[0]).collect_vec();
    diff.iter().all(|&v| v >= 1 && v <= 3) || diff.iter().all(|&v| v <= -1 && v >= -3)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let res = parse_input(input)
        .iter()
        .map(|levels| valid_levels(levels) as i32)
        .sum::<i32>();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let res = parse_input(input)
        .iter()
        .map(|levels| {
            if valid_levels(levels) {
                return 1;
            }

            let diff = levels.windows(2).map(|v| v[1] - v[0]).collect_vec();
            // Index of first ascending sequence break
            let asc_break_idx = diff
                .iter()
                .map(|&v| (v >= 1 && v <= 3) as u8)
                .position(|v| v == 0)
                .unwrap_or(0);
            // Index of first descending sequence break
            let dsc_break_idx = diff
                .iter()
                .map(|&v| (v <= -1 && v >= -3) as u8)
                .position(|v| v == 0)
                .unwrap_or(0);

            // Remove the first asc/desc sequence break or its successor and check if we get a valid levels
            for idx in [
                asc_break_idx,
                asc_break_idx + 1,
                dsc_break_idx,
                dsc_break_idx + 1,
            ] {
                let mut new_levels = levels.clone();
                new_levels.remove(idx);
                if valid_levels(&new_levels) {
                    return 1;
                }
            }
            return 0;
        })
        .sum();

    AoCResult::Int(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input2Test"),
        include_str!("../data/input2"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [2, 663];
    const EXPECTED_PART_TWO: [i64; 2] = [4, 692];

    #[test]
    fn test_part_one() {
        for i in 0..2 {
            let res = solve_part_one(INPUT[i]);
            match res {
                AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_ONE[i]),
                _ => panic!("Wrong result type returned"),
            }
        }
    }

    #[test]
    fn test_part_two() {
        for i in 0..2 {
            let res = solve_part_two(INPUT[i]);
            match res {
                AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_TWO[i]),
                _ => panic!("Wrong result type returned"),
            }
        }
    }
}
