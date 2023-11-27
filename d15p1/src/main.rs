#[derive(Debug)]
struct Cell {
    own_risk: u8,
    best_path: u32,
}

fn main() {
    let mut mx: Vec<Vec<_>> = include_str!("../input").lines().map(|line| {
        line.as_bytes().iter().map(|b| Cell {
            own_risk: b - '0' as u8,
            best_path: 0,
        }).collect()
    }).collect();

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
            mx[x][y].best_path = left.min(top);
        }
    }
    println!("{}", mx[mx.len()-1][mx[0].len()-1].best_path)
}
