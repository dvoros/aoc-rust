use std::collections::HashSet;

const SIZE: isize = 131;
const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut walls = HashSet::new();
    let mut start = (0, 0);
    include_str!("../input").trim().lines().enumerate()
        .for_each(|(r, row)| {
            row.bytes().enumerate().for_each(|(c, col)| {
                match col {
                    b'#' => {
                        walls.insert((r as isize, c as isize));
                    },
                    b'S' => {
                        start = (r as isize, c as isize);
                    },
                    _ => {},
                }
            })
        });

    let reached_big = reached_in(2*SIZE as usize + (SIZE/2) as usize, &walls, &start);
    
    let o = reached_big.iter().filter(|x| within_bounds(x, 0, 0)).count();
    let e = reached_big.iter().filter(|x| within_bounds(x, 0, 1)).count();
    let a = 
        reached_big.iter().filter(|x| within_bounds(x, -1, 1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 1, 1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, -1, -1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 1, -1)).count();
    let b = 
        reached_big.iter().filter(|x| within_bounds(x, -2, 1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 2, 1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, -2, -1)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 2, -1)).count();
    let t = 
        reached_big.iter().filter(|x| within_bounds(x, 0, 2)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 0, -2)).count() +
        reached_big.iter().filter(|x| within_bounds(x, -2, 0)).count() +
        reached_big.iter().filter(|x| within_bounds(x, 2, 0)).count();

    let steps = 26501365;
    let n = (steps - SIZE as usize /2) / SIZE as usize;

    let res = (n-1) * a + n * b + t + n*n*e + (n-1)*(n-1)*o;

    println!("res: {res}");

}

fn within_bounds(x: &(isize, isize), r: isize, c: isize) -> bool {
    x.0 >= SIZE*r && x.0 < SIZE*(r+1) && x.1 >= SIZE*c && x.1 < SIZE*(c+1)
}

fn reached_in(steps: usize, walls: &HashSet<(isize, isize)>, from: &(isize, isize)) -> HashSet<(isize, isize)> {
    let mut reached = HashSet::new();
    reached.insert(from.clone());
    for step in 1..=steps {
        let mut next_reached = HashSet::new();
        for p in &reached {
            add_neighbors(&walls, p, &mut next_reached);
        }
        reached = next_reached;
    }
    reached
}

fn add_neighbors(walls: &HashSet<(isize, isize)>, p: &(isize, isize), add_to: &mut HashSet<(isize, isize)>) {
    for n in NEIGHBORS {
        let check_this = (pos_mod(p.0 + n.0, SIZE), pos_mod(p.1 + n.1, SIZE));
        let add_this = (p.0 + n.0, p.1 + n.1);
        if !walls.contains(&check_this) {
            add_to.insert(add_this);
        }
    }
}

fn pos_mod(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

fn print_mx(walls: &HashSet<(isize, isize)>, reached: &HashSet<(isize, isize)>) {
    let (mut x, mut y) = ((0, 0), (0, 0));
    for r in reached {
        x.0 = x.0.min(r.0);
        x.1 = x.1.max(r.0);
        y.0 = y.0.min(r.1);
        y.1 = y.1.max(r.1);
    }
    for r in x.0-2..x.1+2 {
        for c in y.0-2..y.1+2 {
            if walls.contains(&(pos_mod(r, SIZE), pos_mod(c, SIZE))) {
                print!("#");
            } else if reached.contains(&(r, c)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}