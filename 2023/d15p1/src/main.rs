fn main() {
    let res = include_str!("../input").trim().split(",")
        .map(|s| hash(s) as usize)
        .sum::<usize>();

    println!("res: {res}");
}
            
fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |a, b| {
        let s = a + *b as usize;
        (s * 17) % 256
    })
}
