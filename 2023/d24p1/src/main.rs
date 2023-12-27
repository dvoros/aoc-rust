type Stone = ((isize, isize, isize), (isize, isize, isize));

fn main() {
    let stones: Vec<Stone> = include_str!("../input")
        .trim()
        .lines()
        .map(|line| {
            let (pos, v) = line.split_once(" @ ").unwrap();
            let mut pos = pos.split(",").map(|x| x.trim().parse::<isize>().unwrap());
            let pos = (
                pos.next().unwrap(),
                pos.next().unwrap(),
                pos.next().unwrap(),
            );
            let mut v = v.split(",").map(|x| x.trim().parse::<isize>().unwrap());
            let v = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
            (pos, v)
        })
        .collect();

    // let bounds = (7.0, 27.0);
    let bounds = (200000000000000.0, 400000000000000.0);

    let mut sum = 0;
    for i in 0..stones.len() - 1 {
        for j in i + 1..stones.len() {
            match intersection(&stones[i], &stones[j]) {
                None => {}
                Some(((x, y), future)) => {
                    if future && x > bounds.0 && x < bounds.1 && y > bounds.0 && y < bounds.1 {
                        // println!("{} and {} intersect at ({}, {}), future:{}", i, j, x, y, future);
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("res: {sum}");
}

fn intersection(s1: &Stone, s2: &Stone) -> Option<((f64, f64), bool)> {
    let (p1, v1) = s1;
    let (p2, v2) = s2;
    if v1.1 * v2.0 == v1.0 * v2.1 {
        return None;
    }
    let p1x = p1.0 as f64;
    let p1y = p1.1 as f64;
    let v1x = v1.0 as f64;
    let v1y = v1.1 as f64;
    let p2x = p2.0 as f64;
    let p2y = p2.1 as f64;
    let v2x = v2.0 as f64;
    let v2y = v2.1 as f64;

    let x = (-v2y / v2x * p2x + p2y + v1y / v1x * p1x - p1y) / (v1y / v1x - v2y / v2x);
    let y = (v1y / v1x) * (x - p1x) + p1y;

    let future1 = x > p1x && v1x > 0.0
        || x < p1x && v1x < 0.0
        || y > p1y && v1y > 0.0
        || y < p1y && v1y < 0.0;
    let future2 = x > p2x && v2x > 0.0
        || x < p2x && v2x < 0.0
        || y > p2y && v2y > 0.0
        || y < p2y && v2y < 0.0;

    Some(((x, y), future1 && future2))
}
