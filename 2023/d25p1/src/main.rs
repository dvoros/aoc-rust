use pathfinding::directed::dijkstra::build_path;
use pathfinding::prelude::dijkstra_all;
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {
    let mut graph = HashMap::new();
    include_str!("../input").trim().lines().for_each(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        for r in right.split(" ") {
            graph
                .entry(left.to_string())
                .or_insert(Vec::new())
                .push((r.to_string(), 1));
            graph
                .entry(r.to_string())
                .or_insert(Vec::new())
                .push((left.to_string(), 1));
        }
    });

    let N = 1000;

    let nodes: Vec<_> = graph.keys().cloned().collect();
    let mut rng = rand::thread_rng();

    let mut counts = HashMap::new();

    for _ in 1..=N {
        let from = nodes.choose(&mut rng).unwrap();
        let reachable = dijkstra_all(from, |n: &String| graph[n].clone());
        for _ in 1..=N {
            let to = nodes.choose(&mut rng).unwrap();
            let path = build_path(to, &reachable);
            for i in 0..path.len() - 1 {
                let edge = if path[i] < path[i + 1] {
                    format!("{}-{}", path[i], path[i + 1])
                } else {
                    format!("{}-{}", path[i + 1], path[i])
                };
                *counts.entry(edge).or_insert(0) += 1;
            }
        }
    }

    let mut top = counts.iter().collect::<Vec<_>>();
    top.sort_by(|a, b| b.1.cmp(&a.1));

    top.iter().take(3).for_each(|edge| {
        let (l, r) = edge.0.split_once("-").unwrap();
        graph.get_mut(l).unwrap().retain(|v| v.0 != r.to_string());
        graph.get_mut(r).unwrap().retain(|v| v.0 != l.to_string());
    });

    let size = graph.len();
    let one_side = dijkstra_all(graph.iter().next().unwrap().0, |n: &String| {
        graph[n].clone()
    })
    .len()
        + 1;
    let other_side = size - one_side;

    let res = one_side * other_side;
    println!("res: {res} ({one_side} * {other_side})");
}
