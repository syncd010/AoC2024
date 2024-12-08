use aoc2024::AoCResult;

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let vals = input
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
    vals
}

fn count_combinations(values: &[(i64, Vec<i64>)], num_ops: i32) -> i64 {
    let mut res = 0;

    for (total, numbers) in values.iter() {
        let mut count = 0;
        let places = numbers.len() - 1;

        // let combinations = (0..places).map(|i| 0..num_ops).multi_cartesian_product();
        // for comb in combinations {
        //     let mut result = numbers[0];
        //     for (i, op) in comb.iter().enumerate() {
        //         match op {
        //             0 => result += numbers[i + 1],
        //             1 => result *= numbers[i + 1],
        //             2 => result = result * 10_i64.pow(1 + numbers[i + 1].ilog10()) + numbers[i + 1],
        //             _ => (),
        //         }
        //         if result > *total {
        //             // Short circuit
        //             break;
        //         }
        //     }
        //     if result == *total {
        //         count += 1;
        //     }
        // }

        // Place the operations in each of the available places
        for n in 0..num_ops.pow(places as u32) {
            let mut remain = n;
            let mut result = numbers[0];
            for i in 0..places {
                let pow = num_ops.pow((places - i - 1) as u32);
                let div = remain / pow;
                remain = remain % pow;

                match div {
                    0 => result += numbers[i + 1],
                    1 => result *= numbers[i + 1],
                    2 => result = result * 10_i64.pow(1 + numbers[i + 1].ilog10()) + numbers[i + 1],
                    // 2 => result = format!("{}{}", result, numbers[i+1]).parse::<i64>().unwrap(),
                    _ => (),
                }
                if result > *total {
                    // Short circuit
                    break;
                }
            }
            if result == *total {
                count += 1;
            }
        }
        if count > 0 {
            res += total;
        }
    }

    return res;
}

use itertools::Itertools;

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
