use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    if k * 2 <= b - a + 1 {
        for i in a..=a + k - 1 {
            println!("{}", i);
        }
        for i in b - k + 1..=b {
            println!("{}", i);
        }
    } else {
        for i in a..=b {
            println!("{}", i);
        }
    }
}
