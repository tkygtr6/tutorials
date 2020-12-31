fn main() {
    let mut given_str = String::new();
    std::io::stdin().read_line(&mut given_str).unwrap();
    let mut given_str = String::from(given_str.trim());

    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();

    for _ in 0..num {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let commands: Vec<_> = line.trim().split_whitespace().collect();

        match commands[0] {
            "replace" => {
                let start_idx: usize = commands[1].parse().unwrap();
                let end_idx: usize = commands[2].parse::<usize>().unwrap() + 1;
                let replace_str: &str = commands[3];

                given_str = format!(
                    "{}{}{}",
                    &given_str[..start_idx],
                    replace_str,
                    &given_str[end_idx..]
                );
            }
            "reverse" => {
                let start_idx: usize = commands[1].parse().unwrap();
                let end_idx: usize = commands[2].parse::<usize>().unwrap() + 1;

                given_str = format!(
                    "{}{}{}",
                    &given_str[..start_idx],
                    &given_str[start_idx..end_idx]
                        .chars()
                        .rev()
                        .collect::<String>(),
                    &given_str[end_idx..]
                );
            }
            "print" => {
                let start_idx: usize = commands[1].parse().unwrap();
                let end_idx: usize = commands[2].parse::<usize>().unwrap() + 1;
                println!("{}", &given_str[start_idx..end_idx]);
            }
            _ => {}
        }
    }
}
