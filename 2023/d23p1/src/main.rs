use std::collections::HashSet;

fn main() {
    let mx: Vec<Vec<_>> = include_str!("../input")
        .trim()
        .lines()
        .map(|row| row.as_bytes().into_iter().map(|c| *c).collect())
        .collect();

    let mut solutions = HashSet::new();
    longest(&mx, (0, 1), &mut HashSet::new(), 0, &mut solutions);
    println!("res: {}", solutions.iter().max().unwrap());
}

fn longest(
    mx: &Vec<Vec<u8>>,
    at: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    len: usize,
    solutions: &mut HashSet<usize>,
) {
    if at == (mx.len() - 1, mx[0].len() - 2) {
        solutions.insert(len);
    }
    if visited.contains(&at) {
        return;
    }
    visited.insert(at);

    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let p = (at.0 as isize + dir.0, at.1 as isize + dir.1);
        if p.0 >= 0 && p.0 < mx.len() as isize && p.1 >= 0 && p.1 < mx[0].len() as isize {
            let p = (p.0 as usize, p.1 as usize);
            match mx[p.0][p.1] {
                b'.' => longest(mx, p, &mut visited.clone(), len + 1, solutions),
                b'<' => {
                    if *dir == (0, -1) {
                        longest(mx, p, &mut visited.clone(), len + 1, solutions);
                    }
                }
                b'>' => {
                    if *dir == (0, 1) {
                        longest(mx, p, &mut visited.clone(), len + 1, solutions);
                    }
                }
                b'v' => {
                    if *dir == (1, 0) {
                        longest(mx, p, &mut visited.clone(), len + 1, solutions);
                    }
                }
                b'^' => {
                    if *dir == (-1, 0) {
                        longest(mx, p, &mut visited.clone(), len + 1, solutions);
                    }
                }
                _ => {}
            }
        }
    }
}
