fn main() {
    let res: isize = include_str!("../input")
        .lines()
        .map(|line| {
            let series = line
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect();
            predict(series)
        })
        .sum();
    println!("res: {res}");
}

fn predict(mut series: Vec<isize>) -> isize {
    let mut firsts = Vec::new();
    loop {
        firsts.push(series[0]);
        if series.iter().all(|n| *n == 0) {
            break;
        }
        series = derive(&series);
    }
    firsts.iter().rev().fold(0, |a, n| n - a)
}

fn derive(series: &Vec<isize>) -> Vec<isize> {
    series.windows(2).map(|w| w[1] - w[0]).collect()
}
