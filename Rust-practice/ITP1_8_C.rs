use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_to_string(&mut line).unwrap();

    let chars = line.chars().collect::<Vec<char>>();

    let mut dict = HashMap::new();
    for c in chars {
        let count = dict.entry(c.to_lowercase().to_string()).or_insert(0);
        *count += 1;
    }

    let alphabet = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    for c in alphabet {
        let count = dict.entry(c.to_lowercase().to_string()).or_insert(0);
        println!("{} : {}", c, *count);
    }
}
