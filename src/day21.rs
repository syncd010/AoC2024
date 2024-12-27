use aoc2024::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect_vec()
}

fn dir_to_moves(dir: &Dir) -> String {
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

fn char_combinations(v: String) -> HashSet<String> {
    if v.len() == 0 {
        HashSet::from([v])
    } else {
        (0..v.len())
            .map(|i| format!("{}{}", &v[i..], &v[0..i]))
            .collect()
    }
}

// Expands the code by simulating moving through the given keypad
fn keypad_sim(
    keypad: &HashMap<char, Dir>,
    keypad_inv: &HashMap<Dir, char>,
    start_pos: Dir,
    code: &str,
) -> Vec<String> {
    if let Some(c) = code.chars().next() {
        // Expand going from start_pos to the first char position on the keypad
        let &keypad_pos = keypad.get(&c).expect("Couldn't get keypad position");
        let moves = keypad_pos - start_pos;
        // possible_moves contains all combinations of moves to the destination
        let possible_moves = char_combinations(dir_to_moves(&moves));
        possible_moves
            .iter()
            .filter(|&v| {
                // Filter for moves that go through the black hole in the keypad
                let mut p = start_pos;
                v.chars().all(|c| {
                    match c {
                        '<' => p.x -= 1,
                        '>' => p.x += 1,
                        '^' => p.y -= 1,
                        _ => p.y += 1,
                    }
                    keypad_inv.contains_key(&p)
                })
            })
            .flat_map(|head| {
                // Recurse to get moves for the following chars
                keypad_sim(keypad, keypad_inv, keypad_pos, &code[1..])
                    .iter()
                    .map(|tail| head.to_owned() + "A" + tail)
                    .collect_vec()
            })
            .collect()
    } else {
        vec![String::new()]
    }
}

// Expands a single move recursively, up to the specified depth
// Returns the length of the minimum expansion found
fn expand_rec<'a>(
    single_move: &'a str,
    depth: u8,
    expansion_map: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, u8), usize>,
) -> usize {
    // Already memoized?
    let memo_key = (single_move, depth);
    if memo.contains_key(&memo_key) {
        return memo[&memo_key];
    }

    // Expand
    let expansions = expansion_map
        .get(single_move)
        .expect("Couldn't find step in map");
    let mut min_expansion = usize::MAX;

    if depth == 1 {
        // Reached the bottom, directly return the minimum expansion length found
        min_expansion = expansions
            .iter()
            .reduce(|acc, s| if acc.len() < s.len() { acc } else { s })
            .unwrap()
            .len()
            - 1;
    } else {
        // Expand each of the possibilities, keeping the shortest one
        for &expansion in expansions {
            let mut expanded = 0;
            // Loop through the characters of the expansion on this level, recursively expanding a single one
            for i in 0..expansion.len() - 1 {
                expanded += expand_rec(&expansion[i..i + 2], depth - 1, expansion_map, memo);
            }
            min_expansion = min_expansion.min(expanded);
        }
    }
    // memoize
    memo.insert(memo_key, min_expansion);
    min_expansion
}

// Returns the minimum length of the expansion of moves to the specified depth
fn expand_moves(moves: &str, depth: u8) -> usize {
    // The expansion of each possible move through the keypad, including the initial "A"
    let expansion_map: HashMap<&str, Vec<&str>> = HashMap::from([
        ("A^", vec!["A<A"]),
        ("A>", vec!["AvA"]),
        ("Av", vec!["Av<A", "A<vA"]),
        ("A<", vec!["Av<<A", "A<v<A"]),
        ("AA", vec!["AA"]),
        ("^A", vec!["A>A"]),
        ("^v", vec!["AvA"]),
        ("^<", vec!["Av<A"]),
        ("^>", vec!["Av>A", "A>vA"]),
        ("^^", vec!["AA"]),
        (">A", vec!["A^A"]),
        (">v", vec!["A<A"]),
        ("><", vec!["A<<A"]),
        (">^", vec!["A<^A", "A^<A"]),
        (">>", vec!["AA"]),
        ("v<", vec!["A<A"]),
        ("v>", vec!["A>A"]),
        ("v^", vec!["A^A"]),
        ("vA", vec!["A^>A", "A>^A"]),
        ("vv", vec!["AA"]),
        ("<v", vec!["A>A"]),
        ("<>", vec!["A>>A"]),
        ("<^", vec!["A>^A"]),
        ("<A", vec!["A>>^A", "A>^>A"]),
        ("<<", vec!["AA"]),
    ]);

    let mut memo = HashMap::new();
    let moves = &format!("A{}", moves);
    let mut expanded_len = 0;
    for i in 0..moves.len() - 1 {
        expanded_len += expand_rec(&moves[i..i + 2], depth, &expansion_map, &mut memo);
    }
    expanded_len
}

// Returns the numeric keypad and its inversion
fn num_keypads() -> (HashMap<char, Dir>, HashMap<Dir, char>) {
    let keypad = HashMap::from([
        ('7', Dir { y: 0, x: 0 }),
        ('8', Dir { y: 0, x: 1 }),
        ('9', Dir { y: 0, x: 2 }),
        ('4', Dir { y: 1, x: 0 }),
        ('5', Dir { y: 1, x: 1 }),
        ('6', Dir { y: 1, x: 2 }),
        ('1', Dir { y: 2, x: 0 }),
        ('2', Dir { y: 2, x: 1 }),
        ('3', Dir { y: 2, x: 2 }),
        ('0', Dir { y: 3, x: 1 }),
        ('A', Dir { y: 3, x: 2 }),
    ]);
    let keypad_inverted = keypad.iter().map(|(&k, &v)| (v, k)).collect();
    (keypad, keypad_inverted)
}

// fn dir_keypads() -> (HashMap<char, Dir>, HashMap<Dir, char>) {
//     let keypad = HashMap::from([
//         ('^', Dir { y: 0, x: 1 }),
//         ('A', Dir { y: 0, x: 2 }),
//         ('<', Dir { y: 1, x: 0 }),
//         ('v', Dir { y: 1, x: 1 }),
//         ('>', Dir { y: 1, x: 2 }),
//     ]);
//     let keypad_inverted = keypad.iter().map(|(&k, &v)| (v, k)).collect();
//     (keypad, keypad_inverted)
// }

pub fn solve(input: &str, depth: u8) -> AoCResult {
    let codes = parse_input(input);
    let (num_keypad, num_keypad_inv) = num_keypads();

    let res = codes.iter().fold(0, |acc, code| {
        let mut min_len = usize::MAX;

        for robot_num in keypad_sim(&num_keypad, &num_keypad_inv, Dir { y: 3, x: 2 }, code) {
            let robot_dir = expand_moves(&robot_num, depth);
            min_len = min_len.min(robot_dir);
        }
        let n = code
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        acc + min_len * n
    });
    AoCResult::Int(res as i64)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    solve(input, 2)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    solve(input, 25)
}
