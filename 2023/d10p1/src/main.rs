fn main() {
    let mx: Vec<_> = include_str!("../input")
        .trim()
        .lines()
        .map(|l| l.as_bytes())
        .collect();

    let s = find_s(&mx);

    let next_two: Vec<(usize, usize)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .map(|d| (s.0 as isize + d.0, s.1 as isize + d.1))
        .iter()
        .filter(|p| p.0 >= 0 && p.0 < mx.len() as isize && p.1 >= 0 && p.1 < mx[0].len() as isize)
        .map(|p| (p.0 as usize, p.1 as usize))
        .filter(|p| next_neighbor(&mx, s, *p).is_some())
        .collect();
    let mut next_two = (next_two[0], next_two[1]);
    let mut prev_two = (s, s);

    let mut distance = 1;
    loop {
        if next_two.0 == next_two.1 || next_two.0 == prev_two.1 || next_two.1 == prev_two.0 {
            println!("res: {distance}");
            return;
        }
        let tmp = next_two.clone();
        next_two = (
            next_neighbor(&mx, prev_two.0, next_two.0).unwrap(),
            next_neighbor(&mx, prev_two.1, next_two.1).unwrap(),
        );
        prev_two = tmp;
        distance += 1;
    }
}

fn next_neighbor(
    mx: &Vec<&[u8]>,
    prev: (usize, usize),
    curr: (usize, usize),
) -> Option<(usize, usize)> {
    match mx[curr.0][curr.1] {
        b'F' => {
            if prev.1 == curr.1 + 1 {
                Some((curr.0 + 1, curr.1))
            } else if prev.0 == curr.0 + 1 {
                Some((curr.0, curr.1 + 1))
            } else {
                None
            }
        }
        b'7' => {
            if prev.1 == curr.1 - 1 {
                Some((curr.0 + 1, curr.1))
            } else if prev.0 == curr.0 + 1 {
                Some((curr.0, curr.1 - 1))
            } else {
                None
            }
        }
        b'J' => {
            if prev.1 == curr.1 - 1 {
                Some((curr.0 - 1, curr.1))
            } else if prev.0 == curr.0 - 1 {
                Some((curr.0, curr.1 - 1))
            } else {
                None
            }
        }
        b'L' => {
            if prev.1 == curr.1 + 1 {
                Some((curr.0 - 1, curr.1))
            } else if prev.0 == curr.0 - 1 {
                Some((curr.0, curr.1 + 1))
            } else {
                None
            }
        }
        b'-' => {
            if prev.1 == curr.1 + 1 {
                Some((curr.0, curr.1 - 1))
            } else if prev.1 == curr.1 - 1 {
                Some((curr.0, curr.1 + 1))
            } else {
                None
            }
        }
        b'|' => {
            if prev.0 == curr.0 + 1 {
                Some((curr.0 - 1, curr.1))
            } else if prev.0 == curr.0 - 1 {
                Some((curr.0 + 1, curr.1))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_s(mx: &Vec<&[u8]>) -> (usize, usize) {
    for (r, &row) in mx.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == b'S' {
                return (r, c);
            }
        }
    }
    panic!("no S")
}
