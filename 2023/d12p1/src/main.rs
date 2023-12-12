fn main() {
    let res: usize = include_str!("../input")
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .map(|(s, g)| combinations(s, &g))
        .sum();

    println!("res: {res}");
}

fn parse_line(line: &str) -> (&[u8], Vec<usize>) {
    let (springs, groups) = line.split_once(" ").unwrap();
    let springs = springs.as_bytes();
    let groups = groups.split(",").map(|g| g.parse().unwrap()).collect();
    (springs, groups)
}

// , i_s: usize, i_g: usize
fn combinations(springs: &[u8], groups: &[usize]) -> usize {
    if groups.len() == 0 {
        if springs.iter().any(|s| *s == b'#') {
            return 0;
        } else {
            return 1;
        }
    }
    if springs.len() < groups[0] {
        return 0;
    }
    let mut remaining_combinations = 0;
    if springs[0..groups[0]]
        .iter()
        .all(|s| matches!(s, b'#' | b'?'))
        && (springs.len() == groups[0] || springs[groups[0]] != b'#')
    {
        let mut continue_at = groups[0];
        if springs.len() > groups[0] {
            continue_at += 1;
        }
        remaining_combinations += combinations(&springs[continue_at..], &groups[1..]);
    } else if springs[0] == b'#' {
        return 0;
    }
    if springs[0] != b'#' {
        remaining_combinations += combinations(&springs[1..], groups);
    }
    remaining_combinations
}
