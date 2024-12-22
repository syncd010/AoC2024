#![allow(unused)]
use std::collections::{HashMap, HashSet};

use aoc2024::*;


fn parse_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
}

fn pos_diff(p1: &Pos, p2: &Pos) -> Dir {
    Dir {
        x: p1.x as isize - p2.x as isize,
        y: p1.y as isize - p2.y as isize,
    }
}

fn dir_to_keypresses(dir: &Dir) -> String {
    let horiz = match dir.x.signum() {
        -1 => "<".repeat(dir.x.abs() as usize),
        1 => ">".repeat(dir.x.abs() as usize),
        _ => String::new(),
    };
    let vert = match dir.y.signum() {
        -1 => "^".repeat(dir.y.abs() as usize),
        1 => "v".repeat(dir.y.abs() as usize),
        _ => String::new(),
    };
    horiz + &vert
}

fn iter_combs(v: String) -> HashSet<String> {
    let len = v.len();
    let mut res = HashSet::new();
    if len == 0 {
        res.insert(v);
    } else {
        for i_start in 0..len {
            res.insert(format!("{}{}", &v[i_start..], &v[0..i_start]));
        }
    }
    res
}

fn keypresses(keypad: &HashMap<char, Pos>, start_pos: Pos, code: &str) -> Vec<String> {
    if let Some(c) = code.chars().next() {
        let keypad_pos = keypad.get(&c).expect("Couldn't get keypad position");
        let moves = pos_diff(keypad_pos, &start_pos);

        let keypad_inverted: HashMap<&Pos, &char> = keypad.iter().map(|(k, v)| (v, k)).collect();
        let possible_keypresses = iter_combs(dir_to_keypresses(&moves));

        let a = possible_keypresses.iter().filter(|&v| {
            // Filter for keypresses that go through the black hole in the keypad
            let mut p = start_pos;
            for c in v.chars() {
                match c {
                    '<' => p.x -= 1,
                    '>' => p.x += 1,
                    '^' => p.y -= 1,
                    'v' => p.y += 1,
                    _ => panic!("Unexpected direction"),
                }
                if !keypad_inverted.contains_key(&p) {
                    return false;
                }
            }
            true
        });
        let b = a.flat_map(|head| {
            // Recurse to get keypresses for the following chars
            keypresses(keypad, *keypad_pos, &code[1..])
                .iter()
                .map(|tail| head.to_owned() + "A" + tail)
                .collect::<Vec<_>>()
        });
        b.collect()
    } else {
        vec![String::new()]
    }
}

// fn keypresses_rec(single_step: &str, depth: u8, n_map: &HashMap<&str, Vec<&str>>) -> usize {
//     if depth == 0 {
//         single_step.len() - 1
//     } else {
//         let next_step = n_map.get(single_step).expect("Couldn't find step in map");
//         let mut min_len = usize::MAX;
//         for &step_branch in next_step {
//             let mut total_len = 0;
//             for i in 0..step_branch.len() - 1 {
//                 total_len += keypresses_rec(&step_branch[i..i + 2], depth - 1, n_map);
//             }
//             min_len = min_len.min(total_len + 1);
//         }
//         min_len
//     }
// }

fn keypresses_rec(single_step: &str, depth: u8, n_map: &HashMap<&str, Vec<&str>>) -> String {
    if depth == 0 {
        single_step[0..single_step.len() - 1].to_owned()
    } else {
        let next_step = n_map.get(single_step).expect("Couldn't find step in map");
        let mut min_len = usize::MAX;
        let mut res_str = String::new();
        for &step_branch in next_step {
            let mut total_len = String::new();
            for i in 0..step_branch.len() - 1 {
                total_len = format!("{}{}", total_len, keypresses_rec(&step_branch[i..i + 2], depth - 1, n_map));
            }
            if min_len >= total_len.len() {
                min_len = total_len.len();
                res_str = total_len + "A";
            }
        }
        res_str
    }
}

fn keypresses_2(code: &str, depth: u8) -> usize {
    let numeric_map = HashMap::from([
        ("A^", vec!["<A"]),
        ("A>", vec!["vA"]),
        ("Av", vec!["v<A", "<vA"]),
        ("A<", vec!["v<<A", "<v<A"]),
        ("AA", vec!["A"]),
        ("^A", vec![">A"]),
        ("^v", vec!["vA"]),
        ("^<", vec!["v<A"]),
        ("^>", vec!["v>A", ">vA"]),
        ("^^", vec!["A"]),
        (">A", vec!["^A"]),
        (">v", vec!["<A"]),
        ("><", vec!["<<A"]),
        (">^", vec!["<^A", "^<A"]),
        (">>", vec!["A"]),
        ("v<", vec!["<A"]),
        ("v>", vec![">A"]),
        ("v^", vec!["^A"]),
        ("vA", vec!["^>A", ">^A"]),
        ("vv", vec!["A"]),
        ("<v", vec![">A"]),
        ("<>", vec![">>A"]),
        ("<^", vec![">^A"]),
        ("<A", vec![">>^A", ">^>A"]),
        ("<<", vec!["A"]),
    ]);

    let mut res = 0;
    let mut res_str = String::new();
    for i in 0..code.len() - 1 {
        let s = keypresses_rec(&code[i..i + 2], depth, &numeric_map);
        println!("{}", s);
        res_str = format!("{}{}", res_str, s);
        // res += keypresses_rec(&code[i..i + 2], depth, &numeric_map);
    }
    println!("End {}", res_str);
    res
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let codes = parse_input(input);

    let num_keypad = HashMap::from([
        ('7', Pos { y: 0, x: 0 }),
        ('8', Pos { y: 0, x: 1 }),
        ('9', Pos { y: 0, x: 2 }),
        ('4', Pos { y: 1, x: 0 }),
        ('5', Pos { y: 1, x: 1 }),
        ('6', Pos { y: 1, x: 2 }),
        ('1', Pos { y: 2, x: 0 }),
        ('2', Pos { y: 2, x: 1 }),
        ('3', Pos { y: 2, x: 2 }),
        ('0', Pos { y: 3, x: 1 }),
        ('A', Pos { y: 3, x: 2 }),
    ]);

    let dir_keypad = HashMap::from([
        ('^', Pos { y: 0, x: 1 }),
        ('A', Pos { y: 0, x: 2 }),
        ('<', Pos { y: 1, x: 0 }),
        ('v', Pos { y: 1, x: 1 }),
        ('>', Pos { y: 1, x: 2 }),
    ]);

    let mut res = 0;
    for code in codes {
        let mut min_len = usize::MAX;

        for robot_1 in keypresses(&num_keypad, Pos { y: 3, x: 2 }, code) {
            let m = keypresses_2(&format!("{}{}", "A", robot_1), 1);
            println!("{} {}", robot_1, m);
            break;

            // for robot_2 in keypresses(&dir_keypad, Pos { y: 0, x: 2 }, &robot_1) {
            //     for robot_3 in keypresses(&dir_keypad, Pos { y: 0, x: 2 }, &robot_2) {
            //         min_len = min_len.min(robot_3.len());
            //     }
            // }
        }

       
        min_len = 0;
        let n = code
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        res += min_len * n;

        break;
    }

    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let codes = parse_input(input);

    let res = 0;
    AoCResult::Int(res as i64)
}
