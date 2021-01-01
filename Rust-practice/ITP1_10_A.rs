fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let nums = line
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|c| c.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "{}",
        ((nums[0] - nums[2]).powf(2.0) + (nums[1] - nums[3]).powf(2.0)).sqrt()
    )
}
