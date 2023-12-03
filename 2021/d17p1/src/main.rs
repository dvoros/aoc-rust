use std::collections::HashSet;

fn main() {
    let (t_x, t_y) = ((20, 30), (-10, -5));


    let ps = simulate(6, 9, (t_x, t_y));
    println!("{:?}", ps);
    draw(&ps, (t_x, t_y));

    let (t_x, t_y) = ((241, 275), (-75, -49));
    let highest_y = t_y.0 * (t_y.0+1) / 2;
    println!("highest: {highest_y}");
}

fn draw(steps: &Vec<(isize, isize)>, target: ((isize, isize), (isize, isize))) {
    
        for y in (target.1.0..=steps.iter().map(|(x,y)| *y).max().unwrap()).rev() {
            for x in 0..target.0.1 {
                let mut ch = ".";
                if x >= target.0.0 && x <= target.0.1 && y >= target.1.0 && y <= target.1.1 {
                    ch = "T";
                }
                if steps.contains(&(x, y)) {
                    ch = "#";
                }
                print!("{ch}");
            }
            println!();
        }
}

fn simulate(mut dx: isize, mut dy: isize, target: ((isize, isize), (isize, isize))) -> Vec<(isize, isize)> {
    let mut res = vec![(0, 0)];
    let mut p = (0, 0);
    let mut steps = 0;
    loop {
        steps += 1;
        p = (p.0 + dx, p.1 + dy);
        res.push(p);
        if p.1 < target.1.0 || p.0 > target.0.1 {
            break;
        }
        dy -= 1;
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
    }

    res
}