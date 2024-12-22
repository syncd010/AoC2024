#![allow(unused)]
use std::collections::HashMap;

use aoc2024::{AoCResult, Dir};

fn parse_input(input: &str) -> Vec<u64> {
    let vals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse()
                .expect("Unexpected string in input, not a number")
        })
        .collect::<Vec<_>>();
    vals
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let secrets = parse_input(input);
    let limit = 2000;

    let res = secrets
        .iter()
        .map(|&(mut secret)| {
            for i in 0..limit {
                // n = ((n * 64) ^ n) % 16777216;
                // n = ((n / 32) ^ n) % 16777216;
                // n = ((n * 2048) ^ n) % 16777216;
                secret = ((secret << 6) ^ secret) & 0xffffff;
                secret = ((secret >> 5) ^ secret) & 0xffffff;
                secret = ((secret << 11) ^ secret) & 0xffffff;
            }
            secret
        })
        .sum::<u64>();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let secrets = parse_input(input);
    let limit = 2000 - 1;

    let mut prices = vec![vec![0u8; limit + 1]; secrets.len()];
    let mut diff = vec![vec![0i8; limit]; secrets.len()];

    // let mut sequence_gains = HashMap::<(i8, i8, i8, i8), u64>::new();
    let mut sequence_gains = HashMap::<u64, u64>::new();
    for (monkey, &(mut secret)) in secrets.iter().enumerate() {
        prices[monkey][0] = (secret % 10) as u8;
        // let mut monkey_sequence_prices = HashMap::<(i8, i8, i8, i8), u8>::new();
        let mut monkey_sequence_prices = HashMap::<u64, u8>::new();
        for i in 0..limit {
            secret = ((secret << 6) ^ secret) & 0xffffff;
            secret = ((secret >> 5) ^ secret) & 0xffffff;
            secret = ((secret << 11) ^ secret) & 0xffffff;
            prices[monkey][i + 1] = (secret % 10) as u8;
            diff[monkey][i] = prices[monkey][i + 1] as i8 - prices[monkey][i] as i8;
        }

        for i in 3..limit {
            // let k = (diff[monkey][i - 3], diff[monkey][i - 2], diff[monkey][i - 1], diff[monkey][i]);
            let k = ((diff[monkey][i - 3] as u8) as u64) << 24
                | ((diff[monkey][i - 2] as u8) as u64) << 16
                | ((diff[monkey][i - 1] as u8) as u64) << 8
                | ((diff[monkey][i] as u8) as u64);
            let price = prices[monkey][i + 1];
            monkey_sequence_prices.entry(k).or_insert(price);
        }
        for (k, price) in monkey_sequence_prices {
            sequence_gains
                .entry(k)
                .and_modify(|v| *v += price as u64)
                .or_insert(price as u64);
        }
    }

    let res = *sequence_gains.values().max().unwrap_or(&0);
    AoCResult::Int(res as i64)
}
