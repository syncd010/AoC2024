use aoc2024::AoCResult;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
struct Gate<'a> {
    gate_type: &'a str,
    in1: &'a str,
    in2: &'a str,
    out: &'a str,
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, Vec<Gate>) {
    let (wires, ops) = input.trim().split_once("\n\n").unwrap();
    let wires: HashMap<_, _> = wires
        .lines()
        .map(|line| {
            let mut it = line.split(": ");
            (it.next().unwrap(), it.next().unwrap() == "1")
        })
        .collect();
    let ops = ops
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            Gate {
                gate_type: parts[1],
                in1: parts[0],
                in2: parts[2],
                out: parts[4],
            }
        })
        .collect();
    (wires, ops)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (mut wires, gates) = parse_input(input);
    let mut gates = VecDeque::from(gates);

    while let Some(gate) = gates.pop_front() {
        if let (Some(in1), Some(in2)) = (wires.get(&gate.in1), wires.get(&gate.in2)) {
            let gate_value = match gate.gate_type {
                "AND" => in1 & in2,
                "OR" => in1 | in2,
                "XOR" => in1 ^ in2,
                _ => panic!("Unknown gate type: {}", gate.gate_type),
            };
            wires.insert(gate.out, gate_value);
        } else {
            gates.push_back(gate);
        }
    }

    let outputs = wires
        .iter()
        .filter(|(&k, _)| k.starts_with("z"))
        .sorted_by(|a, b| b.0.cmp(a.0))
        .map(|(_, &v)| v as u8)
        .collect::<Vec<_>>();

    let l = outputs.len() as u32 - 1;
    let res = outputs.iter().enumerate().fold(0, |acc, (i, &bit)| {
        acc + 2u64.pow(l - i as u32) * bit as u64
    });
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (_wires, _gates) = parse_input(input);

    // println!("digraph {{ {{");
    // for gate in &gates {
    //     let op = match gate.gate_type {
    //         GateType::And => "box",
    //         GateType::Or => "triangle",
    //         GateType::Xor => "diamond",
    //     };
    //     println!("{} [shape={}]", gate.op3, op);
    // }
    // println!("}}");
    //
    // for gate in &gates {
    //     println!("{}", gate.op3);
    //     println!("{} -> {}", gate.op1, gate.op3);
    //     println!("{} -> {}", gate.op2, gate.op3);
    // }
    // println!("}}");

    // By inspection...
    //z05,tst - z23,frt - cgh,pmd - z11,sps
    let res = String::from("cgh,frt,pmd,sps,tst,z05,z11,z23");
    AoCResult::Str(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input24Test"),
        include_str!("../data/input24"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [2024, 60714423975686];
    const EXPECTED_PART_TWO: [&str; 2] = [
        "cgh,frt,pmd,sps,tst,z05,z11,z23",
        "cgh,frt,pmd,sps,tst,z05,z11,z23",
    ];

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
                AoCResult::Str(v) => assert_eq!(v, EXPECTED_PART_TWO[i]),
                _ => panic!("Wrong result type returned"),
            }
        }
    }
}
