use std::collections::HashMap;
use num_integer::lcm;

fn main() {
    let (instructions, map) = include_str!("../input").split_once("\n\n").unwrap();
    let map: HashMap<_,_> = map.lines().map(|line| {
        let node = &line[0..3];
        let neighbors = (&line[7..10], &line[12..15]);
        (node, neighbors)
    }).collect();

    let mut steps = 0;
    let mut tiles: Vec<_> = map.keys().filter(|&k| k.as_bytes()[2] == b'A').map(|k| *k).collect();
    let mut nums = Vec::new();
    loop {
        let next: Vec<_> = tiles.into_iter().filter(|&tile| {
            if tile.as_bytes()[2] == b'Z' {
                nums.push(steps);
                return false;
            }
            true
        }).map(|tile| {
            let tile = map.get(tile).unwrap();
            match instructions.as_bytes()[steps % instructions.len()] {
                b'R' => tile.1,
                b'L' => tile.0,
                _ => panic!("invalid input"),
            }
        }).collect();

        if next.len() == 0 {
            let res = nums.iter().fold(1, |a, n| {
                lcm(a, *n)
            });
            println!("res: {res}");
            return;
        }

        steps += 1;
        tiles = next;
    }
}
