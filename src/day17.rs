use aoc2024::AoCResult;
use itertools::Itertools;

type Regs = [u64; 3];

fn parse_input(input: &str) -> (Regs, Vec<u8>) {
    let mut regs = [0, 0, 0];
    let mut instuct = Vec::new();
    let reg_map = [
        ("Register A:", 0usize),
        ("Register B:", 1usize),
        ("Register C:", 2usize),
    ];

    for line in input.lines() {
        for (pat, idx) in reg_map {
            if line.starts_with(pat) {
                regs[idx] = line[line.find(":").unwrap() + 2..].parse().unwrap();
                continue;
            }
        }
        if line.starts_with("Program:") {
            line[line.find(":").unwrap() + 2..]
                .split(",")
                .for_each(|v| {
                    instuct.push(v.parse().unwrap());
                });
        }
    }

    (regs, instuct)
}

fn combo_op(op: u8, regs: &Regs) -> u64 {
    match op {
        0 | 1 | 2 | 3 => op as u64,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!("Uninplemented"),
    }
}

fn run(regs: &mut Regs, instruct: &[u8]) -> Vec<u64> {
    let mut ip = 0usize;
    let mut out = Vec::new();

    while ip < instruct.len() {
        let inst = instruct[ip];
        let operand = instruct[ip + 1];
        match inst {
            0 => {
                regs[0] = regs[0] / 2u64.pow(combo_op(operand, &regs) as u32);
            }
            1 => {
                regs[1] = regs[1] ^ operand as u64;
            }
            2 => {
                regs[1] = combo_op(operand, &regs) % 8;
            }
            3 => {
                if regs[0] != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => {
                regs[1] = regs[1] ^ regs[2];
            }
            5 => {
                out.push(combo_op(operand, &regs) % 8);
            }
            6 => {
                regs[1] = regs[0] / 2u64.pow(combo_op(operand, &regs) as u32);
            }
            7 => {
                regs[2] = regs[0] / 2u64.pow(combo_op(operand, &regs) as u32);
            }
            _ => {
                panic!("Unknow instruction!");
            }
        }
        ip += 2;
    }
    out
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (mut regs, instruct) = parse_input(input);

    let out = run(&mut regs, &instruct);
    let res = out.iter().map(|v| v.to_string()).join(",");
    AoCResult::Str(res)
}

fn run_part_two(a: u64, _instruct: &[u8]) -> u64 {
    // Decoded instructions
    // let mut b = (a % 8) ^ 2;
    // let c = a >> b;
    // b = b ^ 7 ^ c;
    // let out = b % 8;
    // // let out = a % 8;
    // out

    // Directly interpreting the program
    // Remove the last JNZ instruction, so that it only executes once
    let mut regs = [a, 0, 0];
    let out = run(&mut regs, &_instruct[0.._instruct.len() - 2]);
    out[0]
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (mut _regs, instruct) = parse_input(input);
    let mut valid = vec![0u64];
    // Generate each digit in turn
    for &v in instruct.iter().rev() {
        // There can be several solutions, try all and save new valid ones
        let mut next_valid = Vec::new();
        while let Some(curr) = valid.pop() {
            for d in 0..8 {
                // Next digit can only be at curr..curr+8, try each one
                let a = curr + d;
                let out = run_part_two(a, &instruct);
                if out as u8 == v {
                    next_valid.push(a * 8);
                }
            }
        }
        valid = next_valid;
        if valid.len() == 0 {
            panic!("Couldn't find match for {v}");
        }
    }

    let res = *valid.iter().min().unwrap() / 8;
    AoCResult::Int(res as i64)
}
