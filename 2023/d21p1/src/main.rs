use std::collections::HashSet;

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut walls = HashSet::new();
    let mut reached = HashSet::new();
    include_str!("../input").trim().lines().enumerate()
        .for_each(|(r, row)| {
            row.bytes().enumerate().for_each(|(c, col)| {
                match col {
                    b'#' => {
                        walls.insert((r as isize, c as isize));
                    },
                    b'S' => {
                        reached.insert((r as isize, c as isize));
                    },
                    _ => {},
                }
            })
        });

    let target = 64;
    (1..=target).for_each(|step| {
        let mut next_reached = HashSet::new();
        for p in &reached {
            add_neighbors(&walls, p, &mut next_reached);
        }
        reached = next_reached;
        println!("after step {step} reached {} gardens", reached.len());
    });
}

fn add_neighbors(walls: &HashSet<(isize, isize)>, p: &(isize, isize), add_to: &mut HashSet<(isize, isize)>) {
    for n in NEIGHBORS {
        if !walls.contains(&(p.0 + n.0, p.1 + n.1)) {
            add_to.insert((p.0 + n.0, p.1 + n.1));
        }
    }
}