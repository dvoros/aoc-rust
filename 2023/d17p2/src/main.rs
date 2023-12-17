const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let mx: Vec<Vec<usize>> = include_str!("../input")
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| (b - b'0') as usize).collect())
        .collect();

    let res = pathfinding::prelude::dijkstra(
        &((0 as isize, 0 as isize), (0 as isize, 1 as isize), 0),
        |&(pos, dir, straight)| {
            let mut successors = Vec::new();
            if straight < 10 {
                let np = (pos.0 + dir.0, pos.1 + dir.1);
                if np.0 >= 0 && np.0 < mx.len() as isize && np.1 >= 0 && np.1 < mx[0].len() as isize
                {
                    successors.push(((np, dir, straight + 1), mx[np.0 as usize][np.1 as usize]));
                }
            }
            if straight >= 4 {
                for n in vec![-1 as isize, 1] {
                    let new_dir = DIRECTIONS[(((DIRECTIONS.iter().position(|d| *d == dir).unwrap()
                        as isize
                        + n)
                        + 4)
                        % 4) as usize];
                    let np = (pos.0 + new_dir.0, pos.1 + new_dir.1);
                    if np.0 >= 0
                        && np.0 < mx.len() as isize
                        && np.1 >= 0
                        && np.1 < mx[0].len() as isize
                    {
                        successors.push(((np, new_dir, 1), mx[np.0 as usize][np.1 as usize]));
                    }
                }
            }
            successors
        },
        |&(pos, _, straight)| {
            pos.0 == mx.len() as isize - 1 && pos.1 == mx[0].len() as isize - 1 && straight >= 4
        },
    );

    println!("res: {:?}", res.unwrap().1);
}
