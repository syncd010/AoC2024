use std::collections::HashMap;

use aoc2024::AoCResult;

pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut in_rules_section = true;
    let mut rules = HashMap::<i32, Vec<i32>>::new();
    let mut orders = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            in_rules_section = false;
            continue;
        }

        if in_rules_section {
            let rule: Vec<_> = line.split('|').map(parse_int).collect();
            let v = rules.entry(rule[0]).or_insert(Vec::new());
            v.push(rule[1]);
        } else {
            orders.push(line.split(',').map(parse_int).collect::<Vec<_>>());
        }
    }
    (rules, orders)
}

fn is_valid_order(order: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    order
        .windows(2)
        .all(|v| rules.contains_key(&v[0]) && rules.get(&v[0]).unwrap().contains(&v[1]))
    // for (i, val) in order.iter().enumerate() {
    //     if !rules.contains_key(val) {
    //         continue;
    //     }
    //     let successors = rules.get(val).unwrap();
    //     if order[0..i].iter().any(|v| successors.contains(v)) {
    //         return false;
    //     }
    // }
    // return true;
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
