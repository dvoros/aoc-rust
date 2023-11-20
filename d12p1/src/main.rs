use std::collections::{HashMap, HashSet};

use nom::{IResult, multi::{separated_list1, count}, bytes::complete::tag, sequence::separated_pair, character::complete::alpha1};

struct state {
    at: String,
    visited: HashSet<String>,
}

type Nodes<'a> = HashMap<String, HashSet<String>>;

fn main() {
    let edges = parse_lines(include_str!("../input")).unwrap().1;
    let mut nodes: Nodes = HashMap::new();
    for e in edges {
        let node_a : &mut HashSet<String> = nodes.entry(String::from(e.0)).or_insert(HashSet::new());
        node_a.insert(String::from(e.1));
        let node_b : &mut HashSet<String> = nodes.entry(String::from(e.1)).or_insert(HashSet::new());
        node_b.insert(String::from(e.0));
    }

    let res = count_paths(state{
        at: String::from("start"),
        visited: HashSet::new(),
    }, &nodes);

    println!("there are {res} paths")
}

fn count_paths(current: state, nodes: &Nodes) -> u32 {
    if current.at == "end" {
        return 1;
    }
    let mut new_visited = current.visited.clone();
    new_visited.insert(current.at.clone());
    let mut res = 0;
    for n in nodes[&current.at].iter() {
        if !current.visited.contains(n) || n.chars().next().unwrap().is_ascii_uppercase() {
            res += count_paths(
                state { 
                    at: String::from(n),
                    visited: new_visited.clone()
                },
                nodes,
            );
        }
    }
    res
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(tag("\n"), parse_line)(input)
}
