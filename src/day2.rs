use aoc2024::AoCResult;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let vals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    vals
}

pub fn valid_levels(levels: &Vec<i32>) -> bool {
    let diff: Vec<_> = levels.windows(2).map(|v| v[1] - v[0]).collect();
    return diff.iter().all(|v| *v >= 1 && *v <= 3) || diff.iter().all(|v| *v <= -1 && *v >= -3);
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let vals = parse_input(input);
    let res = vals
        .iter()
        .map(|levels| valid_levels(levels) as i32)
        .sum::<i32>();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let vals = parse_input(input);

    let res = vals
        .iter()
        .map(|levels| {
            if valid_levels(levels) {
                return 1;
            }

            let diff: Vec<_> = levels.windows(2).map(|v| v[1] - v[0]).collect();
            // Index of first ascending sequence break
            let asc_break_idx = diff
                .iter()
                .map(|v| (*v >= 1 && *v <= 3) as i32)
                .position(|v| v == 0)
                .unwrap_or(0);
            // Index of first descending sequence break
            let dsc_break_idx = diff
                .iter()
                .map(|v| (*v <= -1 && *v >= -3) as i32)
                .position(|v| v == 0)
                .unwrap_or(0);

            // Remove the first asc/desc sequence break or its successor and check if we get a valid levels
            for idx in [asc_break_idx, asc_break_idx + 1, dsc_break_idx, dsc_break_idx + 1] {
                let mut new_levels = levels.clone();
                new_levels.remove(idx);
                if valid_levels(&new_levels) {
                    return 1;
                }    
            }
            return 0;
        })
        .sum::<i32>();

    AoCResult::Int(res as i64)
}
