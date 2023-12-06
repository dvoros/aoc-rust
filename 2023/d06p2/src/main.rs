fn main() {
    let mut lines = include_str!("../test_input").trim().lines().map(|line| {
        line.replace(" ", "")
            .split_once(":")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap()
    });
    let (t, d) = (lines.next().unwrap(), lines.next().unwrap());

    // -x^2 + t - d > 0
    // x_1 = (-t + sqrt(t*t - 4*d)) / -2
    // x_2 = (-t - sqrt(t*t - 4*d)) / -2
    let sq = f64::sqrt((t * t - 4 * d) as f64);
    let x_1 = ((-(t as f64) + sq.ceil()) / -2 as f64).ceil() as u64;
    let x_2 = ((-(t as f64) - sq.floor()) / -2 as f64).floor() as u64;
    let res = x_2 - x_1;
    println!("res: {res}");
}
