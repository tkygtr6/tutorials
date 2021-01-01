fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<u32>().unwrap();

    println!("{}:{}:{}", num / 3600, (num % 3600) / 60, num % 60);
}
