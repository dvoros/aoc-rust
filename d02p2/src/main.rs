use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;

    let lines = read_lines("./input")?;
    for line in lines {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.split(" ").collect();
            let dir = parts[0];
            let num: i32 = parts[1].trim().parse().expect("should be a number");
            match dir {
                "down" => aim += num,
                "up" => aim -= num,
                "forward" => {
                    pos += num;
                    depth += aim * num;
                }
                d => panic!("unexpected instruction {d}"),
            }
        }
    }
    println!("{}", pos * depth);

    Ok(())
}

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
