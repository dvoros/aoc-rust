fn main() {
    let mut mx: Vec<Vec<_>> = include_str!("../input")
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut prev = vec![mx.clone()];
    let target = 1000000000;
    for i in 1..target {
        for d in dirs {
            mx = tilt(&mx, d);
        }
        if let Some(p) = prev.iter().position(|x| *x == mx) {
            println!("res: {}", load(&prev[p + ((target - i) % (i - p))]));
            return;
        }
        prev.push(mx.clone());
    }
}

fn load(mx: &Vec<Vec<u8>>) -> usize {
    let mut res = 0;
    for r in 0..mx.len() {
        for c in 0..mx[0].len() {
            if mx[r][c] == b'O' {
                res += mx.len() - r
            }
        }
    }
    res
}

fn tilt(mx: &Vec<Vec<u8>>, dir: (isize, isize)) -> Vec<Vec<u8>> {
    let mut res = vec![vec![b'.'; mx[0].len()]; mx.len()];
    for r in 0..mx.len() {
        for c in 0..mx[0].len() {
            match mx[r][c] {
                b'#' => res[r][c] = b'#',
                b'O' => {
                    let mut rr = r as isize;
                    let mut cc = c as isize;
                    let mut count_d = 0;
                    let mut count_o = 0;
                    loop {
                        rr += dir.0;
                        cc += dir.1;
                        if rr < 0
                            || rr >= mx.len() as isize
                            || cc < 0
                            || cc >= mx.len() as isize
                            || mx[rr as usize][cc as usize] == b'#'
                        {
                            break;
                        }
                        count_d += 1;
                        if mx[rr as usize][cc as usize] == b'O' {
                            count_o += 1;
                        }
                    }
                    count_d -= count_o;
                    res[(r as isize + dir.0 * count_d) as usize]
                        [(c as isize + dir.1 * count_d) as usize] = b'O';
                }
                _ => (),
            }
        }
    }

    res
}
