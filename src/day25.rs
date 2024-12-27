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

pub fn solve_part_two(input: &str) -> AoCResult {
    let _vals = parse_input(input);
    let res = 0;
    AoCResult::Int(res as i64)
}
