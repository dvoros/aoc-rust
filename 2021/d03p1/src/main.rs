use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut lines = read_lines("./input")?;
    let mut counts: Vec<i32> = Vec::new();

    let first_line = lines.next();
    match first_line {
        Some(line) => {
            let line = line?;
            for ch in line.chars() {
                if ch == '1' {
                    counts.push(1)
                } else {
                    counts.push(0)
                }
            }
        }
        None => panic!("no lines"),
    }

    let mut number_of_lines = 1;
    for line in lines {
        number_of_lines += 1;
        let line = line?;
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                counts[i] += 1;
            }
        }
    }

    counts.reverse();

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut i = 1;
    for cnt in counts {
        if cnt == number_of_lines / 2 {
            panic!("half and half");
        }
        if cnt > number_of_lines / 2 {
            gamma += i;
        } else {
            epsilon += i;
        }
        i *= 2;
    }

    println!("{gamma} * {epsilon} = {}", gamma * epsilon);

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
