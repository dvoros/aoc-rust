use std::ops::Mul;

use nalgebra::{SMatrix, SquareMatrix, Vector2, Vector4};

type Stone = ((f64, f64, f64), (f64, f64, f64));

fn main() {
    let s: Vec<Stone> = include_str!("../input")
        .trim()
        .lines()
        .map(|line| {
            let (pos, v) = line.split_once(" @ ").unwrap();
            let mut pos = pos.split(",").map(|x| x.trim().parse::<f64>().unwrap());
            let pos = (
                pos.next().unwrap(),
                pos.next().unwrap(),
                pos.next().unwrap(),
            );
            let mut v = v.split(",").map(|x| x.trim().parse::<f64>().unwrap());
            let v = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
            (pos, v)
        })
        .collect();

    let a: SquareMatrix<_, _, _> = SMatrix::<f64, 4, 4>::from_row_slice(
        &[
            s[1].1.1-s[2].1.1, s[2].1.0-s[1].1.0, s[2].0.1-s[1].0.1, s[1].0.0-s[2].0.0,
            s[1].1.1-s[3].1.1, s[3].1.0-s[1].1.0, s[3].0.1-s[1].0.1, s[1].0.0-s[3].0.0,
            s[1].1.1-s[4].1.1, s[4].1.0-s[1].1.0, s[4].0.1-s[1].0.1, s[1].0.0-s[4].0.0,
            s[1].1.1-s[5].1.1, s[5].1.0-s[1].1.0, s[5].0.1-s[1].0.1, s[1].0.0-s[5].0.0,
    ]);
    let b = Vector4::from_row_slice(&[
        -s[1].0.1*s[1].1.0 + s[2].0.1*s[2].1.0 + s[1].0.0*s[1].1.1 - s[2].0.0*s[2].1.1,
        -s[1].0.1*s[1].1.0 + s[3].0.1*s[3].1.0 + s[1].0.0*s[1].1.1 - s[3].0.0*s[3].1.1,
        -s[1].0.1*s[1].1.0 + s[4].0.1*s[4].1.0 + s[1].0.0*s[1].1.1 - s[4].0.0*s[4].1.1,
        -s[1].0.1*s[1].1.0 + s[5].0.1*s[5].1.0 + s[1].0.0*s[1].1.1 - s[5].0.0*s[5].1.1,
    ]);
    let a1 = a.try_inverse().unwrap();
    let x1 = a1.mul(b);

    let a: SquareMatrix<_, _, _> = SMatrix::<f64, 4, 4>::from_row_slice(
        &[
            s[1].1.2-s[2].1.2, s[2].1.0-s[1].1.0, s[2].0.2-s[1].0.2, s[1].0.0-s[2].0.0,
            s[1].1.2-s[3].1.2, s[3].1.0-s[1].1.0, s[3].0.2-s[1].0.2, s[1].0.0-s[3].0.0,
            s[1].1.2-s[4].1.2, s[4].1.0-s[1].1.0, s[4].0.2-s[1].0.2, s[1].0.0-s[4].0.0,
            s[1].1.2-s[5].1.2, s[5].1.0-s[1].1.0, s[5].0.2-s[1].0.2, s[1].0.0-s[5].0.0,
    ]);
    let b = Vector4::from_row_slice(&[
        -s[1].0.2*s[1].1.0 + s[2].0.2*s[2].1.0 + s[1].0.0*s[1].1.2 - s[2].0.0*s[2].1.2,
        -s[1].0.2*s[1].1.0 + s[3].0.2*s[3].1.0 + s[1].0.0*s[1].1.2 - s[3].0.0*s[3].1.2,
        -s[1].0.2*s[1].1.0 + s[4].0.2*s[4].1.0 + s[1].0.0*s[1].1.2 - s[4].0.0*s[4].1.2,
        -s[1].0.2*s[1].1.0 + s[5].0.2*s[5].1.0 + s[1].0.0*s[1].1.2 - s[5].0.0*s[5].1.2,
    ]);
    let a1 = a.try_inverse().unwrap();
    let x2 = a1.mul(b);

    let res = x1[0].round() + x1[1].round() + x2[1].round();
    println!("res: {res}");

}