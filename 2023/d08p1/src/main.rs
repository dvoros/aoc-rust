use std::collections::HashMap;

fn main() {
    let (instructions, map) = include_str!("../input").split_once("\n\n").unwrap();
    let map: HashMap<_,_> = map.lines().map(|line| {
        let node = &line[0..3];
        let neighbors = (&line[7..10], &line[12..15]);
        (node, neighbors)
    }).collect();

    let mut steps = 0;
    let mut tile = map.get("AAA").unwrap();
    loop {
        let next;
        match instructions.as_bytes()[steps % instructions.len()] {
            b'R' => {next = tile.1},
            b'L' => {next = tile.0},
            _ => panic!("invalid input"),
        }
        steps += 1;
        if next == "ZZZ" {
            println!("res: {steps}");
            return;
        }
        tile = map.get(next).unwrap();
    }
}
