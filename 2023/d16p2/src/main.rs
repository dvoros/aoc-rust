use std::{cell::RefCell, collections::HashMap};

type Position = (isize, isize);
type Direction = (isize, isize);

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

fn main() {
    let mx: Vec<Vec<u8>> = include_str!("../input")
        .trim()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let res = (0..mx.len())
        .flat_map(|r| [((r, 0), RIGHT), ((r, mx[0].len() - 1), LEFT)])
        .chain((0..mx[0].len()).flat_map(|c| [((0, c), DOWN), ((mx[0].len(), c), UP)]))
        .map(|(pos, dir)| beam(&mx, (pos.0 as isize, pos.1 as isize), dir))
        .max()
        .unwrap();
    println!("res: {res}");
}

fn beam(mx: &Vec<Vec<u8>>, pos: Position, dir: Direction) -> usize {
    let past = RefCell::new(HashMap::new());
    _beam(mx, pos, dir, &past);
    let res = past.borrow().keys().count();
    res
}

fn _beam(
    mx: &Vec<Vec<u8>>,
    pos: Position,
    dir: Direction,
    past: &RefCell<HashMap<Position, Vec<Direction>>>,
) {
    if pos.0 < 0 || pos.0 >= mx.len() as isize || pos.1 < 0 || pos.1 >= mx[0].len() as isize {
        return;
    }
    {
        let mut past = past.borrow_mut();
        let e = past.entry(pos).or_insert(Vec::new());
        if e.contains(&dir) {
            return;
        }
        e.push(dir);
    }
    match mx[pos.0 as usize][pos.1 as usize] {
        b'.' => _beam(mx, mv(pos, dir), dir, past),
        b'|' => match dir {
            UP | DOWN => _beam(mx, mv(pos, dir), dir, past),
            _ => {
                _beam(mx, mv(pos, UP), UP, past);
                _beam(mx, mv(pos, DOWN), DOWN, past);
            }
        },
        b'-' => match dir {
            LEFT | RIGHT => _beam(mx, mv(pos, dir), dir, past),
            _ => {
                _beam(mx, mv(pos, LEFT), LEFT, past);
                _beam(mx, mv(pos, RIGHT), RIGHT, past);
            }
        },
        b'/' => match dir {
            RIGHT => _beam(mx, mv(pos, UP), UP, past),
            LEFT => _beam(mx, mv(pos, DOWN), DOWN, past),
            UP => _beam(mx, mv(pos, RIGHT), RIGHT, past),
            _ => _beam(mx, mv(pos, LEFT), LEFT, past),
        },
        b'\\' => match dir {
            RIGHT => _beam(mx, mv(pos, DOWN), DOWN, past),
            LEFT => _beam(mx, mv(pos, UP), UP, past),
            UP => _beam(mx, mv(pos, LEFT), LEFT, past),
            _ => _beam(mx, mv(pos, RIGHT), RIGHT, past),
        },
        _ => {}
    }
}

fn mv(p: Position, d: Direction) -> Position {
    (p.0 + d.0, p.1 + d.1)
}
