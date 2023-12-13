fn main() {
    let res = include_str!("../input").trim().split("\n\n").map(|m| {
        m.lines().map(|line| line.as_bytes().to_vec()).collect()
    }).map(|mx: Vec<Vec<u8>>| {
        if let Some(row) = row_reflect(&mx) {
            return row * 100
        }
        row_reflect(&transpose(&mx)).unwrap()
    }).sum::<usize>();

    println!("res: {res}");
}

fn row_reflect(mx: &Vec<Vec<u8>>) -> Option<usize> {
    for r in 1..mx.len() {
        let mut rr = 0;
        let mut all_match = true;
        loop {
            let r1 = r+rr;
            let r2 = r as isize + (-1*(rr+1) as isize);
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
            return Some(r);
        }
    }
    None
}

fn transpose(mx: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..mx[0].len())
        .map(|col| (0..mx.len()).map(|row| mx[row][col]).collect())
        .collect()
}