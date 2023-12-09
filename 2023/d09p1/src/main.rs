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
    let mut sum = 0;
    loop {
        sum += *series.iter().last().unwrap();
        if series.iter().all(|n| *n == 0) {
            return sum;
        }
        series = derive(&series);
    }
}

fn derive(series: &Vec<isize>) -> Vec<isize> {
    series.windows(2).map(|w| w[1] - w[0]).collect()
}
