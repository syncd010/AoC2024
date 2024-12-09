use aoc2024::AoCResult;

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    // Each line has the total and the vector of numbers to check
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let idx = line.find(':').unwrap();
            let n = line[0..idx].parse::<i64>().unwrap();
            let parsed = line[idx + 1..]
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (n, parsed)
        })
        .collect::<Vec<_>>();
    lines
}

fn count_combinations(values: &[(i64, Vec<i64>)], num_ops: i32) -> i64 {
    let mut res = 0;

    for (total, numbers) in values.iter() {
        // Current and next stack of results
        let mut curr_stack = vec![numbers[0]];
        let mut next_stack = Vec::<i64>::new();
        // Loop through each position, placing each of the operations
        for pos in 0..numbers.len() - 1 {
            while let Some(curr) = curr_stack.pop() {
                if curr > *total {
                    continue;
                }
                next_stack.push(curr + numbers[pos + 1]);
                next_stack.push(curr * numbers[pos + 1]);
                if num_ops == 3 {
                    next_stack.push(curr * 10_i64.pow(1 + numbers[pos + 1].ilog10()) + numbers[pos + 1]);
                }
            }
            std::mem::swap(&mut curr_stack, &mut next_stack);
        }

        if curr_stack.iter().any(|r| *r == *total) {
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
