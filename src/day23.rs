use aoc2024::AoCResult;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let input = input.lines().filter(|line| !line.is_empty()).map(|line| {
        let mut it = line.split("-");
        (it.next().unwrap(), it.next().unwrap())
    });

    let mut graph = HashMap::new();
    for (k, v) in input {
        graph.entry(k).or_insert_with(HashSet::new).insert(v);
        graph.entry(v).or_insert_with(HashSet::new).insert(k);
    }
    graph
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let graph = parse_input(input);

    let mut visited = HashSet::new();
    let mut res = 0u32;
    for (&a, a_edges) in &graph {
        let mut marks = a_edges.clone();
        for &b in a_edges.difference(&visited) {
            let b_edges = graph.get(b).unwrap();
            for &c in marks.intersection(&b_edges) {
                if !visited.contains(&c) {
                    if [a, b, c].iter().any(|node| node.starts_with("t")) {
                        res += 1;
                    }
                }
            }
            marks.remove(b);
        }
        visited.insert(a);
    }

    AoCResult::Int(res as i64)
}

fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &HashSet<&str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
) -> Rc<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        Rc::new(r.clone())
    } else if p.is_empty() {
        Rc::new(HashSet::new())
    } else {
        let mut max_clique: Rc<HashSet<&str>> = Rc::new(HashSet::new());
        let mut p_loop = p.clone();
        let mut x_loop = x.clone();
        let pivot = p.iter().next().copied().unwrap();
        let pivot_neighbors = graph.get(pivot).unwrap();
        for &v in p.difference(pivot_neighbors) {
            let mut r_rec = r.clone();
            r_rec.insert(v);
            let p_rec = p_loop
                .clone()
                .intersection(graph.get(v).unwrap())
                .copied()
                .collect();
            let x_rec = x_loop
                .clone()
                .intersection(graph.get(v).unwrap())
                .copied()
                .collect();
            let clique = bron_kerbosch(&r_rec, &p_rec, &x_rec, &graph);
            if max_clique.len() < clique.len() {
                max_clique = Rc::clone(&clique);
            }
            p_loop.remove(v);
            x_loop.insert(v);
        }

        max_clique
    }
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let graph = parse_input(input);

    let r = HashSet::new();
    let p = graph.keys().copied().collect::<HashSet<_>>();
    let x = HashSet::new();
    let max_clique = bron_kerbosch(&r, &p, &x, &graph);

    let res = max_clique.iter().sorted().copied().join(",");
    AoCResult::Str(res)
}
