fn main() {
    // let (t_x, t_y) = ((20, 30), (-10, -5));
    let (t_x, t_y) = ((241, 275), (-75, -49));

    let mut count = 0;
    for x in 0..=t_x.1 {
        for y in t_y.0..=(-1 * t_y.0) {
            let hit = simulate(x, y, (t_x, t_y));
            if hit {
                count += 1;
                // println!("({x}, {y})");
            }
        }
    }
    println!("count: {count}")
}

fn simulate(mut dx: isize, mut dy: isize, target: ((isize, isize), (isize, isize))) -> bool {
    let mut p = (0, 0);
    loop {
        p = (p.0 + dx, p.1 + dy);
        if p.0 >= target.0.0 && p.0 <= target.0.1 && p.1 >= target.1.0 && p.1 <= target.1.1 {
            return true
        }
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
    false
}