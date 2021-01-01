fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut nums = line.trim().split_whitespace().collect::<Vec<_>>();
    nums.reverse();
    let nums = nums.join(" ");
    println!("{}", nums);
}
