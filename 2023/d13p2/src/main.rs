fn main() {
    let res = include_str!("../input")
        .trim()
        .split("\n\n")
        .map(|m| m.lines().map(|line| line.as_bytes().to_vec()).collect())
        .map(|mx: Vec<Vec<u8>>| value(mx))
        .sum::<usize>();

    println!("res: {res}");
}

fn value(mut mx: Vec<Vec<u8>>) -> usize {
    let original_value;
    let v = row_reflect(&mx);
    if v.len() > 0 {
        original_value = v[0] * 100
    } else {
        original_value = row_reflect(&transpose(&mx))[0];
    }

    for r in 0..mx.len() {
        for c in 0..mx[0].len() {
            let orig = mx[r][c];
            match orig {
                b'.' => mx[r][c] = b'#',
                _ => mx[r][c] = b'.',
            }
            for v in row_reflect(&mx) {
                if v * 100 != original_value {
                    return v * 100;
                }
            }
            for v in row_reflect(&transpose(&mx)) {
                if v != original_value {
                    return v;
                }
            }
            mx[r][c] = orig;
        }
    }

    panic!("no value")
}

fn row_reflect(mx: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut res = Vec::new();
    for r in 1..mx.len() {
        let mut rr = 0;
        let mut all_match = true;
        loop {
            let r1 = r + rr;
            let r2 = r as isize + (-1 * (rr + 1) as isize);
            if r1 >= mx.len() || r2 < 0 {
                break;
            }
            if mx[r1] != mx[r2 as usize] {
                all_match = false;
                break;
            }
            rr += 1;
        }
        if all_match {
            res.push(r);
        }
    }
    res
}

fn transpose(mx: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..mx[0].len())
        .map(|col| (0..mx.len()).map(|row| mx[row][col]).collect())
        .collect()
}
