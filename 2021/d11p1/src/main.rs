use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let mut mx = parse_lines(include_str!("../input")).unwrap().1;
    let mut total_flashes = 0;
    for step in 1..=100 {
        let flashes = do_step(&mut mx);
        total_flashes += flashes;
        // println!("after step {step} ({flashes} flash(es) this step)");
        // print_mx(& mx)
    }
    println!("there were {total_flashes} flashes");

}

fn print_mx(mx: &Vec<Vec<u8>>) {
    for row in mx {
        println!("{:?}", row);
    }
    println!();
}

fn do_step(mx: &mut Vec<Vec<u8>>) -> u32{
    for i in 0..mx.len() {
        for j in 0..mx[0].len() {
            mx[i][j] += 1;
        }
    }

    let mut flashes = 0;
    for i in 0..mx.len() {
        for j in 0..mx[0].len() {
            if mx[i][j] >= 10 {
                flashes += flash(mx, i, j);
            }
        }
    }
    flashes
}

fn flash(mx: &mut Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    mx[i][j] = 0;
    let deltas: Vec<(i32, i32)> = [-1, 0, 1].iter().flat_map(|&x| {
        [-1, 0, 1].map(|y| (x, y))
    }).filter(|&x| x != (0, 0)).collect();
    let mut flashes = 1;
    for d in &deltas {
        let ii = i as i32 + d.0;
        let jj = j as i32 + d.1;
        if ii >= 0 && (ii as usize) < mx.len() && jj >= 0 && (jj as usize) < mx[0].len() {
            let ii = ii as usize;
            let jj = jj as usize;
            if mx[ii][jj] > 0 {
                mx[ii][jj] += 1;
                if mx[ii][jj] >= 10 {
                    flashes += flash(mx, ii, jj);
                }
            }
        }
    }
    flashes
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