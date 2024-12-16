use aoc2024::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;
use std::vec;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let grid = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    grid
}

fn find_in_grid(grid: &[Vec<char>], sentinel: char) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == sentinel {
                return Pos { x, y };
            }
        }
    }
    panic!("Couldn't find {sentinel}");
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pos: Pos,
    dir: Dir,
    prev: Option<Rc<State>>,
}

// The priority queue depends on `Ord`. Explicitly implement a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the ordering on costs and compare positions in case of tie
        other.cost.cmp(&self.cost).then_with(|| {
            self.pos
                .cmp(&other.pos)
                .then_with(|| self.dir.cmp(&other.dir))
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn successors(state: Rc<State>, grid: &[Vec<char>]) -> Vec<State> {
    let mut succ = Vec::new();
    let dirs = [
        (state.dir, 1),
        (state.dir.rotate(Rotation::Clockwise), 1000),
        (state.dir.rotate(Rotation::CounterClockwise), 1000),
    ];
    for (dir, cost) in dirs {
        let next_pos = state.pos + dir;
        if grid[next_pos.y][next_pos.x] != '#' {
            succ.push(State {
                cost: state.cost + cost,
                pos: if state.dir == dir {
                    next_pos
                } else {
                    state.pos
                },
                dir,
                prev: Some(state.clone()),
            });
        }
    }

    succ
}

fn dir_idx(d: Dir) -> usize {
    (d.x.abs() * 2 + (d.x + 1) / 2 + (d.y + 1) / 2) as usize
}

fn dijkstra(start: State, grid: &[Vec<char>]) -> Vec<Rc<State>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut frontier = BinaryHeap::from([Rc::new(start)]);
    let mut reached = vec![vec![vec![u64::MAX; 4]; width]; height];
    let mut min_goal_cost = u64::MAX;
    let mut goals_found = Vec::new();

    // Uniform cost search
    while let Some(current) = frontier.pop() {
        if current.cost > min_goal_cost {
            // Already found all goals with minimum cost
            break;
        }
        if current.cost > reached[current.pos.y][current.pos.x][dir_idx(current.dir)] {
            // Already visited this node with less cost
            continue;
        }
        if grid[current.pos.y][current.pos.x] == 'E' {
            // Node is goal, save
            min_goal_cost = current.cost;
            goals_found.push(current);
            continue;
        }
        for next in successors(current, &grid) {
            // Process successors
            let prev_cost = &mut reached[next.pos.y][next.pos.x][dir_idx(next.dir)];
            if next.cost > *prev_cost {
                // Already visited this node with less cost
                continue;
            }
            *prev_cost = next.cost;
            frontier.push(Rc::new(next));
        }
    }
    // Filter goals with the minimum cost found.
    goals_found
        .iter()
        .filter(|&rc| rc.cost == min_goal_cost)
        .map(|rc| rc.clone())
        .collect::<Vec<_>>()
}

fn solve(input: &str) -> (u64, u64) {
    let grid = parse_input(input);
    let start = State {
        pos: find_in_grid(&grid, 'S'),
        dir: Dir { y: 0, x: 1 },
        cost: 0,
        prev: None,
    };

    let paths = &dijkstra(start, &grid);
    let mut unique_pos = HashSet::new();
    // Reconstruct path
    for mut path in paths {
        unique_pos.insert(path.pos);

        while let Some(p) = path.prev.as_ref() {
            unique_pos.insert(p.pos);
            path = path.prev.as_ref().unwrap();
        }
    }

    (paths[0].cost, unique_pos.len() as u64)
}

use std::cell::Cell;
thread_local! { static SOLUTION: Cell<Option<(u64, u64)>> = Cell::new(None); }

pub fn solve_part_one(input: &str) -> AoCResult {
    // Solve both parts and store
    SOLUTION.with(|sol_cell| {
        let mut sol = sol_cell.get();
        if sol.is_none() {
            sol = Some(solve(input));
            sol_cell.set(sol);
        }
        AoCResult::Int(sol.unwrap().0 as i64)
    })
}

pub fn solve_part_two(input: &str) -> AoCResult {
    // Solve both parts and store
    SOLUTION.with(|sol_cell| {
        let mut sol = sol_cell.get();
        if sol.is_none() {
            sol = Some(solve(input));
            sol_cell.set(sol);
        }
        AoCResult::Int(sol.unwrap().1 as i64)
    })
}
