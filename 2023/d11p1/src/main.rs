use std::collections::HashSet;

#[derive(Debug, PartialEq, PartialOrd)]
enum Space {
    Galaxy,
    Empty,
}

fn main() {
    let mut galaxies = HashSet::new();
    let mx: Vec<Vec<_>> = include_str!("../input")
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, b)| {
                    let s = match b {
                        b'.' => Space::Empty,
                        b'#' => Space::Galaxy,
                        _ => unreachable!(),
                    };
                    if s == Space::Galaxy {
                        galaxies.insert((row, col));
                    }
                    s
                })
                .collect()
        })
        .collect();

    let empty_rows: Vec<usize> = (0..mx.len())
        .filter(|row| (0..mx[0].len()).all(|col| mx[*row][col] == Space::Empty))
        .collect();
    let empty_cols: Vec<usize> = (0..mx[0].len())
        .filter(|col| (0..mx.len()).all(|row| mx[row][*col] == Space::Empty))
        .collect();

    let galaxies: HashSet<_> = galaxies
        .iter()
        .map(|(r, c)| {
            (
                r + empty_rows.iter().filter(|&er| er < r).count(),
                c + empty_cols.iter().filter(|&ec| ec < c).count(),
            )
        })
        .collect();

    let sum: usize = galaxies
        .iter()
        .map(|g1| {
            galaxies
                .iter()
                .filter(|g2| *g2 != g1)
                .map(|g2| g2.0.abs_diff(g1.0) + g2.1.abs_diff(g1.1))
                .sum::<usize>()
        })
        .sum();
    println!("{}", sum / 2);
}
