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
const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

    let res: u32 = pathfinding::directed::dijkstra::dijkstra(
        &(0, 0), 
            |&(x, y)| {
                NEIGHBORS.iter().map(|(xx, yy)| {
                    mx.get((x + xx) as usize)
                        .and_then(|row| row.get((y + yy) as usize))
                        .map(|cell| ((x+xx, y+yy), cell.own_risk))
                })
                .flatten().collect::<Vec<_>>()
            },
    |&p| p == (mx.len() as i32 -1, mx.len() as i32 -1)
    ).unwrap().1;
    
    println!("{res}")
}
