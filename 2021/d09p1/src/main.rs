use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

pub fn main() {
    let mx = parse_lines(include_str!("../input")).unwrap().1;

    let mut risk_level: u32 = 0;
    for (i, row) in mx.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            let ns = neighbors(&mx, i, j);
            if ns.iter().all(|n| n > x) {
                risk_level += *x as u32 + 1;
                // println!("{x} (at [{i}, {j}]) is a low point");
            }
        }
    }
    println!("risk level is {risk_level}");
}

fn neighbors(mx: &Vec<Vec<u8>>, i: usize, j: usize) -> Vec<u8> {
    let mut res = Vec::new();
    let deltas: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for delta in deltas {
        let r = i as isize + delta.0;
        if r >= 0 && r < mx.len() as isize {
            let row = &mx[r as usize];
            let c = j as isize + delta.1;
            if c >= 0 && c < row.len() as isize {
                res.push(row[c as usize]);
            }
        }
    }
    res
}

fn from_str(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

fn parse_line(input: &str) -> IResult<&str, Vec<u8>> {
    many1(map_res(take(1usize), from_str))(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(tag("\n"), parse_line)(input)
}
