use aoc2024::AoCResult;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    // Each line has the total and the vector of numbers to check
    input
        .trim()
        .lines()
        .map(|line| {
            let (n, ns) = line.split_once(": ").unwrap();
            (
                n.parse().unwrap(),
                ns.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn count_combinations(values: &[(i64, Vec<i64>)], num_ops: i32) -> i64 {
    let mut res = 0;

    for (total, ns) in values.iter() {
        // Current and next stack of results
        let mut curr_stack = vec![ns[0]];
        let mut next_stack = Vec::new();
        // Loop through each position, placing each of the operations
        for pos in 0..ns.len() - 1 {
            while let Some(curr) = curr_stack.pop() {
                if curr > *total {
                    continue;
                }
                next_stack.push(curr + ns[pos + 1]);
                next_stack.push(curr * ns[pos + 1]);
                if num_ops == 3 {
                    next_stack.push(curr * 10_i64.pow(1 + ns[pos + 1].ilog10()) + ns[pos + 1]);
                }
            }
            std::mem::swap(&mut curr_stack, &mut next_stack);
        }

        if curr_stack.iter().any(|&r| r == *total) {
            res += total;
        }
    }

    return res;
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let values = parse_input(input);
    let res = count_combinations(&values, 2);
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let values = parse_input(input);
    let res = count_combinations(&values, 3);
    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input7Test"),
        include_str!("../data/input7"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [3749, 1399219271639];
    const EXPECTED_PART_TWO: [i64; 2] = [11387, 275791737999003];

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
