use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

fn main() {
    let (coords, folds) = include_str!("../input").split_once("\n\n").unwrap();
    let coords: HashSet<_> = coords.lines().map(|line| {
        let (x, y) = line.split_once(",").unwrap();
        (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
    }).collect();


    let folds: Vec<_> = folds.lines().map(|fold| {
        let parts = fold.split_once(" ").unwrap().1.
            split_once("=").unwrap();
        match parts.0 {
            "along x" => Fold::X(parts.1.parse().unwrap()),
            _ => Fold::Y(parts.1.parse().unwrap()),
        }
    }).collect();

    let res = folds.iter().fold(coords, |prev, f| {
        do_fold(&prev, f)
    });

    print(&res);
}

fn print(coords: &HashSet<(u32, u32)>) {
    let (xmax, ymax) = coords.iter().fold((0, 0), |(xmax, ymax), (x, y)| {
        (xmax.max(*x), ymax.max(*y))
    });
    for y in 0..=ymax {
        for x in 0..=xmax {
            if coords.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn do_fold(coords: &HashSet<(u32, u32)>, fold: &Fold) -> HashSet<(u32, u32)> {
    coords.iter().map( |c| {
        match *fold {
            Fold::X(x) => (
                if c.0 > x {
                    x - (c.0 - x)
                } else {
                    c.0
                },
                c.1
            ),
            Fold::Y(y) => (
                c. 0,
                if c.1 > y {
                    y - (c.1 - y)
                } else {
                    c.1
                }
            ),
        }
    }).collect()
}