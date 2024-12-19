use aoc2024::AoCResult;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
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

    let mut res = 0;
    for design in designs {
        let mut reached_idxs = Vec::from([0usize]);
        let mut design_is_possible = false;
        while let Some(idx) = reached_idxs.pop() {
            for pat in &patterns {
                if design[idx..].starts_with(pat) {
                    let new_idx = idx + pat.len();
                    if new_idx == design.len() {
                        design_is_possible = true;
                        break;
                    } else {
                        reached_idxs.push(new_idx);
                    }
                }
            }
            if design_is_possible {
                res += 1;
                break;
            }
        }
    }

    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (patterns, designs) = parse_input(input);

    let mut res = 0;
    for design in &designs {
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
        res += paths_to_idx.last().unwrap();
    }
    AoCResult::Int(res as i64)
}
