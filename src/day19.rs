use aoc2024::AoCResult;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut patterns = None;
    let mut designs = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if patterns.is_none() {
            patterns = Some(
                line.split(", ")
                    .map(|p| p.chars().collect())
                    .collect::<Vec<_>>(),
            );
        } else {
            designs.push(line.chars().collect());
        }
    }

    (patterns.unwrap(), designs)
}

fn pattern_fits(pattern: &[char], design: &[char]) -> bool {
    pattern
        .iter()
        .enumerate()
        .all(|(i, c)| i < design.len() && *c == design[i])
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (patterns, designs) = parse_input(input);

    let mut res = 0;
    for design in designs {
        let mut reached_idxs = Vec::from([0usize]);
        let mut design_is_possible = false;
        while let Some(idx) = reached_idxs.pop() {
            for pat in &patterns {
                if pattern_fits(pat, &design[idx..]) {
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
                if pattern_fits(pat, &design[idx..]) {
                    let new_idx = idx + pat.len();
                    paths_to_idx[new_idx] += paths_to_idx[idx];
                }
            }
        }
        res += paths_to_idx.last().unwrap();
    }
    AoCResult::Int(res as i64)
}
