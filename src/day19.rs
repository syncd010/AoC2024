use aoc2024::AoCResult;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();
    (
        patterns.split(", ").collect_vec(),
        designs.split("\n").collect_vec(),
    )
}

// fn possible_combinations<'a>(design: &'a str, patterns: &Vec<&str>, memo: &mut HashMap<&'a str, u64>) -> u64 {
//     if design.len() == 0 {
//         1
//     } else if let Some(count) = memo.get(design) {
//         *count
//     } else {
//         let res = patterns
//             .iter()
//             .filter(|pat| design.starts_with(*pat))
//             .map(|pat| possible_combinations(&design[pat.len()..], patterns, memo))
//             .sum();
//         memo.entry(design).and_modify(|c| *c = res).or_insert(res);
//         res
//     }
// }

pub fn solve_part_one(input: &str) -> AoCResult {
    let (patterns, designs) = parse_input(input);

    let res = designs
        .iter()
        .map(|&design| {
            let mut reached_idx = Vec::from([0usize]);
            while let Some(idx) = reached_idx.pop() {
                for pat in &patterns {
                    if design[idx..].starts_with(pat) {
                        let new_idx = idx + pat.len();
                        if new_idx == design.len() {
                            return 1;
                        } else {
                            reached_idx.push(new_idx);
                        }
                    }
                }
            }
            0
        })
        .sum::<i64>();

    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (patterns, designs) = parse_input(input);

    let res = designs
        .iter()
        .map(|design| {
            let mut paths_to_idx = vec![0u64; design.len() + 1];
            paths_to_idx[0] = 1;
            for idx in 0..design.len() {
                for pat in &patterns {
                    if design[idx..].starts_with(pat) {
                        let new_idx = idx + pat.len();
                        paths_to_idx[new_idx] += paths_to_idx[idx];
                    }
                }
            }
            *paths_to_idx.last().unwrap()
        })
        .sum::<u64>();
    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input19Test"),
        include_str!("../data/input19"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [6, 228];
    const EXPECTED_PART_TWO: [i64; 2] = [16, 584553405070389];

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
