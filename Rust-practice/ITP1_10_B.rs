fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let nums = line
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let a = nums[0];
    let b = nums[1];
    let theta = nums[2] / 360.0 * (2.0 * std::f64::consts::PI);
    println!("{}", a * b * theta.sin() * 0.5);
    println!(
        "{}",
        a + b + (a.powf(2.0) + b.powf(2.0) - 2.0 * a * b * theta.cos()).sqrt()
    );
    println!("{}", b * theta.sin());
}
