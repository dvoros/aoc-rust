use std::{collections::HashMap, cell::RefCell};

fn main() {
    let res: usize = include_str!("../input")
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .map(|(s, g)| {
            let mut five_s = Vec::from(s);
            for _ in 0..4 {
                five_s.push(b'?');
                five_s.extend(s);
            }
            let g = g.iter().cycle().take(g.len() * 5).cloned().collect::<Vec<_>>();
            let res = combinations(&five_s, &g);
            res
        })
        .sum();

    println!("res: {res}");
}

fn parse_line(line: &str) -> (&[u8], Vec<usize>) {
    let (springs, groups) = line.split_once(" ").unwrap();
    let springs = springs.as_bytes();
    let groups = groups.split(",").map(|g| g.parse().unwrap()).collect();
    (springs, groups)
}

fn combinations(
    springs: &[u8],
    groups: &[usize]) 
    -> usize {
    let memo: RefCell<HashMap<(usize, usize), usize>> = RefCell::new(HashMap::new());
    _combinations(springs, groups, 0, 0, &memo)
}

fn _combinations(
    springs: &[u8],
    groups: &[usize],
    si: usize,
    gi: usize,
    memo: &RefCell<HashMap<(usize, usize), usize>>)
    -> usize {
    if let Some(x) = memo.borrow().get(&(si, gi)) {
        return *x
    }
    if si >= springs.len() {
        if gi == groups.len() {
            return 1
        } else {
            return 0
        }
    }
    if gi >= groups.len() {
        if springs[si..].iter().all(|s| matches!(s, b'.' | b'?')) {
            return 1
        } else {
            return 0
        }
    }
    if groups[gi] > springs.len() - si {
        return 0
    }
    if springs[si] == b'.' {
        let res = _combinations(springs, groups, si+1, gi, memo);
        memo.borrow_mut().insert((si, gi), res);
        return res;
    } else if springs[si] == b'#' {
        if fits(springs, si, groups[gi]) {
            let res = _combinations(springs, groups, si+groups[gi]+1, gi+1, memo);
            memo.borrow_mut().insert((si, gi), res);
            return res;
        } else {
            return 0;
        }
    } else {
        let mut res = 0;
        res += _combinations(springs, groups, si+1, gi, memo);
        if fits(springs, si, groups[gi]) {
            res += _combinations(springs, groups, si+groups[gi]+1, gi+1, memo);
        }
        memo.borrow_mut().insert((si, gi), res);
        return res
    }
}

fn fits(
    springs: &[u8],
    si: usize,
    group: usize
) -> bool {
    if !springs[si..si+group].iter().all(|s| matches!(s, b'#' | b'?')) {
        return false
    }
    if springs.len() > si+group && springs[si+group] == b'#' {
        return false
    }
    true
}