fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let nums = line
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let a = nums[0];
    let b = nums[1];
    println!("{} {}", a * b, (a + b) * 2);
}
