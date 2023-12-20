fn main() {
    let steps: Vec<_> = include_str!("../input").trim().lines().map(|line| {
        let hex = line.split(" ").nth(2).unwrap();
        let dir_code = (hex.as_bytes()[hex.len()-2] - b'0') as i32;
        let num = i64::from_str_radix(&hex[2..hex.len()-2], 16).unwrap();
        (dir_code, num)
    }).collect();

    let mut height = 0;
    let mut sum = 0;
    for i in 0..steps.len() {
        let prev = steps[(i as isize -1+steps.len() as isize) as usize %steps.len()];
        let curr = steps[i];
        let next = steps[(i+1)%steps.len()];

        let num = curr.1 + mod_length(prev.0, curr.0, next.0);
        match curr.0 {
            0 => sum += num * height,
            1 => height -= num,
            2 => sum -= num * height,
            3 => height += num,
            _ => panic!("unexpected dir code"),
        }
    }

    println!("res: {sum}");
}

fn mod_length(prev: i32, curr: i32, next: i32) -> i64 {
    if prev == next {
        return 0;
    }
    if prev == (curr + 1) % 4 {
        return -1;
    }
    1
}