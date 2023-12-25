use std::collections::{HashSet, HashMap};

fn main() {
    let mx: Vec<Vec<_>> = include_str!("../input")
        .trim()
        .lines()
        .map(|row| row.as_bytes().into_iter().map(|c| *c).collect())
        .collect();


    let mut nodes = HashMap::new();
    nodes.insert((0, 1), Vec::new());
    graph(&mx, (1, 1), (0, 1), (0, 1), 1, &mut HashSet::new(), &mut nodes);
    // println!("nodes: {:?}", nodes);
    // dot(&nodes);

    let target = (mx.len()-1, mx[0].len()-2);
    let mut solutions = HashSet::new();
    longest(&nodes, (0, 1), target, &mut HashSet::new(), 0, &mut solutions);
    println!("found {} solutions", solutions.len());
    println!("res: {}", solutions.iter().max().unwrap());
    // 4722 too low
}

fn dot_node_name(node: &(usize, usize)) -> String {
    format!("{}_{}", node.0, node.1)
}

fn dot(nodes: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>) {
    println!("graph g {{");
    let mut seen = HashSet::new();
    for n1 in nodes {
        for n2 in n1.1 {
            if !seen.contains(&n2.0) {
                println!("  \"{}\" -- \"{}\" [label={}]", dot_node_name(&n1.0), dot_node_name(&n2.0), n2.1);
            }
        }
        seen.insert(n1.0);
    }
    println!("}}");
}

fn graph(
    mx: &Vec<Vec<u8>>,
    at: (usize, usize),
    last_pos: (usize, usize),
    mut last_node: (usize, usize),
    mut since_last_node: usize,
    visited: &mut HashSet<(usize, usize)>,
    nodes: &mut HashMap<(usize, usize), Vec<((usize, usize), usize)>>
) {
    visited.insert(at);
    if is_node(mx, at) {
        if !nodes.contains_key(&at) {
            // println!("new node: {:?}", at);
            nodes.insert(at, vec![(last_node, since_last_node)]);
        } else {
            // println!("old node: {:?}", at);
            nodes.entry(at).and_modify(|l| l.push((last_node, since_last_node)));
        }
        nodes.entry(last_node).and_modify(|l| l.push((at, since_last_node)));
        last_node = at;
        since_last_node = 0;
    } else if at == (mx.len() - 1, mx[0].len() - 2) {
        nodes.insert(at, vec![(last_node, since_last_node)]);
        nodes.entry(last_node).and_modify(|l| l.push((at, since_last_node)));
        return;
    }
    let new_neighbors: Vec<_> = neighbors(mx, at).into_iter().filter(|&p| p != last_pos && (!visited.contains(&p) || is_node(mx, p))).collect();
    // println!("{:?} has {} neigbors: {:?}", at, new_neighbors.len(), new_neighbors);
    // if new_neighbors.len() > 1 {
    //     println!("continuing on {} paths", new_neighbors.len());
    // }
    new_neighbors.iter().for_each(|&p| {
        graph(mx, p, at, last_node, since_last_node + 1, visited, nodes);
    })
}

fn is_node(mx: &Vec<Vec<u8>>, at: (usize, usize)) -> bool {
    neighbors(mx, at).len() > 2
}

fn neighbors(mx: &Vec<Vec<u8>>, at: (usize, usize)) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter_map(|dir| {
        let p = (at.0 as isize + dir.0, at.1 as isize + dir.1);
        if p.0 >= 0 && p.0 < mx.len() as isize && p.1 >= 0 && p.1 < mx[0].len() as isize {
            let p = (p.0 as usize, p.1 as usize);
            match mx[p.0][p.1] {
                b'#' => return None,
                _ => return Some(p),
            }
        }
        None
    }).collect()
}

fn longest(
    nodes: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    at: (usize, usize),
    target: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    len: usize,
    solutions: &mut HashSet<usize>,
) {
    if at == target {
        solutions.insert(len);
    }
    if visited.contains(&at) {
        return;
    }
    visited.insert(at);

    for n in &nodes[&at] {
        longest(nodes, n.0, target, &mut visited.clone(), len+n.1, solutions);
    }
}
