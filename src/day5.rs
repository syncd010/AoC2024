use aoc2024::AoCResult;
use itertools::Itertools;
use std::collections::HashMap;

pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let (rules_in, orders_in) = input.trim().split_once("\n\n").unwrap();

    let mut rules = HashMap::<i32, Vec<i32>>::new();
    rules_in.lines().for_each(|line| {
        let rule = line.split('|').map(parse_int).collect_vec();
        rules.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
    });

    let orders = orders_in
        .lines()
        .map(|line| line.split(',').map(parse_int).collect_vec())
        .collect_vec();

    (rules, orders)
}

fn is_valid_order(order: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    order
        .windows(2)
        .all(|v| rules.contains_key(&v[0]) && rules.get(&v[0]).unwrap().contains(&v[1]))
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (rules, orders) = parse_input(input);

    let res: i32 = orders
        .iter()
        .filter(|order| is_valid_order(order, &rules))
        .map(|order| order.get(order.len() / 2).copied().unwrap())
        .sum();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (rules, mut orders) = parse_input(input);

    let res: i32 = orders
        .iter_mut()
        .filter(|order| !is_valid_order(order, &rules))
        .map(|v| {
            v.sort_by(|a, b| {
                if !rules.contains_key(a) {
                    std::cmp::Ordering::Greater
                } else if rules.get(a).unwrap().contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            v
        })
        .map(|order| order.get(order.len() / 2).copied().unwrap())
        .sum();

    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input5Test"),
        include_str!("../data/input5"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [143, 5955];
    const EXPECTED_PART_TWO: [i64; 2] = [123, 4030];

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
