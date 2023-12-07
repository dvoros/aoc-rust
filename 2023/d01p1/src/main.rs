pub fn main() {
    let sum: u32 = include_str!("../input").trim().lines().map(
        |line| {
            let nums = line.as_bytes().iter()
                .filter(|&b| *b >= b'0' && *b <= b'9')
                .map(|b| b - b'0')
                .collect::<Vec<_>>();
            (nums[0] * 10 + nums[nums.len()-1]) as u32
        }
    ).sum();

    println!("{:?}", sum)
}
