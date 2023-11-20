use std::fmt::{write, Display};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Cell {
    number: u8,
    is_crossed: bool,
}
struct BingoBoard {
    cells: [[Cell; 5]; 5],
}

impl BingoBoard {
    fn from_rows(rows: Vec<Vec<u8>>) -> Result<BingoBoard, &'static str> {
        if rows.len() != 5 || rows.iter().any(|row| row.len() != 5) {
            return Err("Invalid board, must be 5x5!");
        }

        let mut cells = [[Cell {
            number: 0,
            is_crossed: false,
        }; 5]; 5];

        for (i, row) in rows.iter().enumerate() {
            for (j, &number) in row.iter().enumerate() {
                cells[i][j].number = number;
            }
        }

        Ok(BingoBoard { cells })
    }

    fn cross_out(&mut self, n: u8) {
        for row in &mut self.cells {
            for cell in row {
                if cell.number == n {
                    cell.is_crossed = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        if self
            .cells
            .iter()
            .any(|row| row.iter().all(|c| c.is_crossed))
        {
            return true;
        }
        for col in 0..5 {
            if self.cells.iter().all(|row| row[col].is_crossed) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell| !cell.is_crossed)
            .map(|cell| cell.number as u32)
            .sum()
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for c in row {
                if c.is_crossed {
                    write!(f, "[{:2}] ", c.number)?;
                } else {
                    write!(f, " {:2}  ", c.number)?;
                }
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut lines = read_lines("./input")?;

    let first_line: Vec<u8> = lines
        .next()
        .unwrap()
        .expect("must have a first line")
        .split(",")
        .map(|x| x.parse().expect("should be a number"))
        .collect();

    let mut rows: Vec<Vec<u8>> = Vec::new();
    let mut boards: Vec<BingoBoard> = Vec::new();
    for line in lines {
        let line = line?;
        if line == "" {
            continue;
        }
        let row: Vec<u8> = line
            .split_whitespace()
            .map(|x| x.parse().expect("not an int"))
            .collect();
        rows.push(row);
        if rows.len() == 5 {
            boards.push(BingoBoard::from_rows(rows).expect("should be able to parse"));
            rows = Vec::new();
        }
    }

    for x in first_line {
        for b in boards.iter_mut() {
            if b.is_bingo() {
                continue;
            }
            b.cross_out(x);
            if b.is_bingo() {
                println!(
                    "{}\nunmarked is: {}, score is: {}",
                    b,
                    b.unmarked_sum(),
                    b.unmarked_sum() * x as u32,
                );
            }
        }
    }

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
