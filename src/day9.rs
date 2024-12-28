use aoc2024::AoCResult;
use itertools::Itertools;

// Returns the input values and a memory representation
fn parse_input(input: &str) -> (Vec<u32>, Vec<i32>) {
    let vals = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();

    let sz = vals.iter().sum::<u32>() as usize;
    let mut mem = vec![-1; sz];
    let mut mem_ptr = 0usize;
    for (i, &v) in vals.iter().enumerate() {
        let mem_contents = if (i % 2) == 0 { i as i32 / 2 } else { -1 };
        for j in 0..v {
            mem[mem_ptr + j as usize] = mem_contents;
        }
        mem_ptr += v as usize;
    }

    (vals, mem)
}

fn checksum(mem: &[i32]) -> i64 {
    mem.iter()
        .enumerate()
        .map(|(i, &v)| if v != -1 { i as i32 * v } else { 0 } as i64)
        .sum()
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (_vals, mut mem) = parse_input(input);

    let mut start = 0usize;
    let mut end = mem.len() - 1;
    while start <= end {
        while mem[start] != -1 {
            start += 1;
        }
        while mem[start] == -1 && start <= end {
            while mem[end] == -1 {
                end -= 1;
            }
            mem[start] = mem[end];
            mem[end] = -1;
            end -= 1;
            start += 1;
        }
    }

    AoCResult::Int(checksum(&mem) as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (vals, mut mem) = parse_input(input);
    let mut moved = vals.clone();
    let initial_offsets = moved
        .iter()
        .scan(0, |state, &x| {
            let res = *state;
            *state += x;
            Some(res)
        })
        .collect::<Vec<_>>();

    // Move each value starting from the end
    for from in (0..vals.len()).step_by(2).rev() {
        let need = vals[from];
        let offset_from = initial_offsets[from];
        let mut offset_to = 0;
        // Find space for it starting from the start
        for to in (1..from).step_by(2) {
            offset_to += moved[to - 1];
            if moved[to] >= need {
                // Found space, move. Update from, to and free space around each one
                moved[from] -= need;
                moved[from - 1] += need;
                moved[to] -= need;
                moved[to - 1] += need;

                // Relocate mem
                for idx in 0..need {
                    mem[(offset_to + idx) as usize] = mem[(offset_from + idx) as usize];
                    mem[(offset_from + idx) as usize] = -1;
                }
                break;
            }
            offset_to += moved[to];
        }
    }

    AoCResult::Int(checksum(&mem) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input9Test"),
        include_str!("../data/input9"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [1928, 6399153661894];
    const EXPECTED_PART_TWO: [i64; 2] = [2858, 6421724645083];

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
