use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    if c <= a + b {
        println!("Yes");
    } else {
        println!("No");
    }
}
