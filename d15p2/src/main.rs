use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
struct Cell {
    own_risk: u32,
    best_path: u32,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.own_risk.to_string())
    }
}

impl Cell {
    fn new() -> Self {
        Cell {
            own_risk: 0,
            best_path: 0,
        }
    }
}

const WIDTH: usize = 100;

fn main() {
    let small_mx: Vec<Vec<_>> = include_str!("../input").lines().map(|line| {
        line.as_bytes().iter().map(|b| Cell {
            own_risk: (b - '0' as u8) as u32,
            best_path: 0,
        }).collect()
    }).collect();

    let mut mx = [[Cell::new(); 5*WIDTH]; 5*WIDTH];
    for i in 0..5 {
        for j in 0..5 {
            for x in 0..small_mx.len() {
                for y in 0..small_mx[0].len() {
                    mx[i*WIDTH+x][j*WIDTH+y] = Cell {
                        own_risk: (small_mx[x][y].own_risk + i as u32 + j as u32 - 1) % 9 + 1,
                        best_path: 0,
                    };
                }
            }
        }
    }

    for x in 0..mx.len() {
        for y in 0..mx[0].len() {
            print!("{}", mx[x][y])
        }
        println!()
    }

    let (w, h) = (mx.len(), mx[0].len());
    let rounds: Vec<Vec<_>> = (1..w+h-1).map(|i| {
        (0..w).flat_map(|x| {
            (0..h).filter(|y| x + y == i).map(|y| (x, y)).collect::<Vec<_>>()
        }).collect()
    }).collect();

    for round in rounds {
        for (x, y) in round {
            let own_risk =  mx[x][y].own_risk;
            let left = if x > 0 {
                own_risk as u32 + mx[x-1][y].best_path
            } else {
                u32::MAX
            };
            let top = if y > 0 {
                own_risk as u32 + mx[x][y-1].best_path
            } else {
                u32::MAX
            };
            mx[x][y].best_path = left.min(top)
        }
    }
    println!("{}", mx[mx.len()-1][mx[0].len()-1].best_path)

    // 3165 too high
    // 2926 too high
}
