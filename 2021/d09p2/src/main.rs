use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

struct Point {
    found: bool,
    value: u8,
}

impl Point {
    fn new(value: u8) -> Point {
        Point {
            found: false,
            value,
        }
    }
}

pub fn main() {
    let mut mx = parse_lines(include_str!("../input")).unwrap().1;

    let mut sizes = Vec::new();
    for i in 0..mx.len() {
        for j in 0..mx[0].len() {
            if !mx[i][j].found && mx[i][j].value != 9 {
                let basin_size = basin(&mut mx, i, j);
                // println!("basin of {basin_size}");
                sizes.push(basin_size);
            }
        }
    }
    sizes.sort_unstable();
    sizes.reverse();
    let res = sizes[0..3].iter().fold(1, |c, x| c * x);
    println!("result is {res}");
}

fn basin(mx: &mut Vec<Vec<Point>>, i: usize, j: usize) -> u32 {
    mx[i][j].found = true;
    if mx[i][j].value == 9 {
        return 0;
    }
    let mut from_neighbors = 0;
    let deltas: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for delta in deltas {
        let r = i as isize + delta.0;
        if r >= 0 && r < mx.len() as isize {
            let row = &mx[r as usize];
            let c = j as isize + delta.1;
            if c >= 0 && c < row.len() as isize {
                if !mx[r as usize][c as usize].found {
                    from_neighbors += basin(mx, r as usize, c as usize);
                }
            }
        }
    }
    1 + from_neighbors
}

fn from_str(input: &str) -> Result<Point, std::num::ParseIntError> {
    let value = input.parse::<u8>()?;
    Ok(Point::new(value))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    many1(map_res(take(1usize), from_str))(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(tag("\n"), parse_line)(input)
}
