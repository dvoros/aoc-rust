use std::collections::HashMap;

#[derive(Debug)]
enum Cell {
    Digit(u8),
    Symbol(u8),
    Empty,
}

#[derive(Debug)]
struct NumSym {
    num: usize,
    row: usize,
    col_from: usize,
    col_to: usize,
    syms: Vec<u8>,
}

pub fn main() {
    let mx = include_str!("../input").trim().lines().map(
        |line| {
            line.bytes().map(
                |ch| {
                    if ch == '.' as u8 {
                        Cell::Empty
                    } else if ch >= '0' as u8 && ch <= '9' as u8 {
                        Cell::Digit(ch - '0' as u8)
                    } else {
                        Cell::Symbol(ch)
                    }
                }
            ).collect::<Vec<_>>()
        }
    ).collect::<Vec<_>>();

    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    let mut number: usize = 0;
    let mut number_from = 0;
    for r in 0..mx.len() {
        for c in 0..mx[0].len() {
            let cell = &mx[r][c];
            if let Cell::Digit(d) = cell {
                if number == 0 {
                    number_from = c;
                }
                number = number * 10 + *d as usize;
            } else {
                if number > 0 {
                    let num_sym = NumSym{
                        num: number,
                        row: r,
                        col_from: number_from,
                        col_to: c-1,
                        syms: Vec::new(),
                    };
                    numbers.push(num_sym);
                    number = 0;
                }
                match cell {
                    Cell::Empty => {},
                    Cell::Symbol(s) => {
                        symbols.insert((r,c), *s);
                    }
                    _ => panic!("shouldn't happen"),
                }
            }
        }
        if number > 0 {
            let c = mx[0].len() - 1;
            let num_sym = NumSym{
                num: number,
                row: r,
                col_from: number_from,
                col_to: c,
                syms: Vec::new(),
            };
            numbers.push(num_sym);
            number = 0;
        }
    }

    let deltas: Vec<_> = [-1, 0, 1].iter().flat_map(|x| {
        [-1, 0, 1].iter().filter(|&y| *x != 0 || *y != 0).map(|&y| (*x, y))
    }).collect();

    for (k, v) in symbols {
        for d in deltas.iter() {
            let (r, c) = (k.0 as isize + d.0, k.1 as isize + d.1);
            if r < 0 || r >= mx.len() as isize || c < 0 || c >= mx[0].len() as isize {
                continue;
            }
            for num_sym in numbers.iter_mut() {
                if num_sym.row == r as usize && c as usize >= num_sym.col_from && c as usize <= num_sym.col_to {
                    num_sym.syms.push(v);
                }
            }
        }
    }

    let res: usize = numbers.iter().filter(|ns| ns.syms.len() > 0).map(|ns| ns.num).sum();
    println!("{res}");
}
