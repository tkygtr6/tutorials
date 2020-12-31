fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let chars = line.trim().chars().collect::<Vec<char>>();

    let mut output_line = String::new();
    for c in chars {
        if c.is_lowercase() {
            output_line += &c.to_uppercase().collect::<String>();
        } else if c.is_uppercase() {
            output_line += &c.to_lowercase().collect::<String>();
        } else {
            output_line.push(c);
        }
    }

    println!("{}", output_line);
}
