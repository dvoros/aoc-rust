use std::collections::HashMap;

fn main() {
    let mut mx = HashMap::new();
    let mut p = (0, 0);
    let (mut min, mut max) = ((0, 0), (0, 0));
    let mut prev_dir = b'X';
    let mut first_dir = b'X';
    include_str!("../input").trim().lines().for_each(|line| {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap().as_bytes().first().unwrap();
        if first_dir == b'X' {
            first_dir = *dir;
        }
        let num = parts.next().unwrap().parse::<i32>().unwrap();
        let delta;
        let c = corner(prev_dir, *dir);
        match dir {
            b'R' => delta = (0, 1),
            b'L' => delta = (0, -1),
            b'D' => delta = (1, 0),
            b'U' => delta = (-1, 0),
            _ => panic!("unknown dir"),
        }
        prev_dir = *dir;
        for i in 0..num {
            if i == 0 {
                mx.insert(p, c);
            } else {
                mx.insert(p, b'#');
            }
            p = (p.0 + delta.0, p.1 + delta.1);
            min.0 = min.0.min(p.0);
            min.1 = min.1.min(p.1);
            max.0 = max.0.max(p.0);
            max.1 = max.1.max(p.1);
        }
    });
    mx.insert((0, 0), corner(prev_dir, first_dir));

    // for r in min.0..=max.0 {
    //     for c in min.1..=max.1 {
    //         if let Some(ch) = mx.get(&(r,c)) {
    //             print!("{}", char::from(*ch));
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut sum = 0;
    for r in min.0..=max.0 {
        let mut inside = false;
        let mut in_wall = false;
        let mut wall_start = b'X';
        for c in min.1..=max.1 {
            match mx.get(&(r, c)) {
                None => {
                    if inside {
                        sum += 1;
                        // print!("o");
                    } else {
                        // print!(".");
                    }
                }
                Some(&v) => {
                    // print!("{}", char::from(v));
                    sum += 1;
                    match v {
                        b'#' => {
                            if in_wall {
                                continue;
                            }
                            inside = !inside
                        },
                        b'U' => {
                            if !in_wall {
                                wall_start = b'U'
                            } else {
                                if wall_start != b'U' {
                                    inside = !inside
                                }
                            }
                            in_wall = !in_wall
                        },
                        b'D' => {
                            if !in_wall {
                                wall_start = b'D'
                            } else {
                                if wall_start != b'D' {
                                    inside = !inside
                                }
                            }
                            in_wall = !in_wall
                        }
                        _ => panic!("unexpected value"),
                    }
                }
            }
        }
        // println!();
    }

    println!("res: {sum}");
}

fn corner(prev_dir: u8, curr_dir: u8) -> u8 {
    match curr_dir {
        b'R' => {
            if prev_dir == b'U' {
                return b'D'
            } else if prev_dir == b'D'{
                return b'U'
            }
        },
        b'L' => {
            if prev_dir == b'U' {
                return b'D'
            } else if prev_dir == b'D' {
                return b'U'
            }
        },
        b'D' => {
            return b'D'
        },
        b'U' => {
            return b'U'
        },
        _ => panic!("unknown dir"),
    }
    b'#'
}