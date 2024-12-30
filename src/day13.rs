use aoc2024::*;

#[derive(Debug, Copy, Clone)]
struct Machine {
    prize: [f64; 2],
    buttons: [[f64; 2]; 2],
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut next = Machine {
        prize: [0.0, 0.0],
        buttons: [[0.0, 0.0], [0.0, 0.0]],
    };

    for line in input.lines() {
        if line.starts_with("Button A:") {
            next.buttons[0][0] = line[line.find("X+").unwrap() + 2..line.find(",").unwrap()]
                .parse()
                .unwrap();
            next.buttons[0][1] = line[line.find("Y+").unwrap() + 2..].parse().unwrap();
        } else if line.starts_with("Button B:") {
            next.buttons[1][0] = line[line.find("X+").unwrap() + 2..line.find(",").unwrap()]
                .parse()
                .unwrap();
            next.buttons[1][1] = line[line.find("Y+").unwrap() + 2..].parse().unwrap();
        } else if line.starts_with("Prize:") {
            next.prize[0] = line[line.find("X=").unwrap() + 2..line.find(",").unwrap()]
                .parse()
                .unwrap();
            next.prize[1] = line[line.find("Y=").unwrap() + 2..].parse().unwrap();

            machines.push(next.clone());
        }
    }
    machines
}

// 2x2 matrix determinant
fn determinant(matrix: &[[f64; 2]; 2]) -> f64 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

// 2x2 matrix invert
fn invert(matrix: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let det = determinant(matrix) as f64;

    [
        [matrix[1][1] as f64 / det, -matrix[0][1] as f64 / det],
        [-matrix[1][0] as f64 / det, matrix[0][0] as f64 / det],
    ]
}

// 2x2 matrix dot product
fn dot(b: &[f64; 2], matrix: &[[f64; 2]; 2]) -> [f64; 2] {
    [
        b[0] as f64 * matrix[0][0] + b[1] as f64 * matrix[1][0],
        b[0] as f64 * matrix[0][1] + b[1] as f64 * matrix[1][1],
    ]
}

fn solve_linear_eq(b: &[f64; 2], matrix: &[[f64; 2]; 2]) -> i64 {
    // Check for singular matrix
    if determinant(&matrix) as i64 == 0 {
        return 0;
    }

    // aX = b <=> a = dot(b, inv(X))
    let a = dot(&b, &invert(&matrix));
    // Only consider integer solutions for the number of steps
    let a_rounded = [a[0].round(), a[1].round()];
    if (a_rounded[0] - a[0]).abs() < 1e-3 && (a_rounded[1] - a[1]).abs() < 1e-3 {
        a_rounded[0] as i64 * 3 + a_rounded[1] as i64
    } else {
        0
    }
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let res: i64 = parse_input(input)
        .iter()
        .map(|v| solve_linear_eq(&v.prize, &v.buttons))
        .sum();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let res: i64 = parse_input(input)
        .iter()
        .map(|&(mut v)| {
            v.prize[0] += 10000000000000.0;
            v.prize[1] += 10000000000000.0;
            solve_linear_eq(&v.prize, &v.buttons)
        })
        .sum();
    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input13Test"),
        include_str!("../data/input13"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [480, 34787];
    const EXPECTED_PART_TWO: [i64; 2] = [875318608908, 85644161121698];

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
