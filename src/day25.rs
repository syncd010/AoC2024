use aoc2024::AoCResult;

fn count_in_columns(grid: &[&str], mark: char) -> Vec<usize> {
    let mut counts = vec![0; grid[0].len()];
    for &row in grid {
        for (x, c) in row.chars().enumerate() {
            if c == mark {
                counts[x] += 1;
            }
        }
    }
    counts
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let locks_and_keys = input
        .trim()
        .split("\n\n")
        .map(|grid| grid.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let lock_heights = locks_and_keys
        .iter()
        .filter(|&grid| grid[0].chars().all(|c| c == '#'))
        .map(|grid| count_in_columns(grid, '#'))
        .collect::<Vec<_>>();

    let key_heights = locks_and_keys
        .iter()
        .filter(|&grid| grid[0].chars().all(|c| c == '.'))
        .map(|grid| count_in_columns(grid, '.'))
        .collect::<Vec<_>>();

    (lock_heights, key_heights)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (locks, keys) = parse_input(input);
    // println!("Locks: {:?}\nKeys: {:?}", locks, keys);

    let res = locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(|key| lock.iter().enumerate().all(|(i, &v)| v <= key[i]))
        })
        .count();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(_input: &str) -> AoCResult {
    let res = 0;
    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input25Test"),
        include_str!("../data/input25"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [3, 2933];
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
}
