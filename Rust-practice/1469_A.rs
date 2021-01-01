fn body() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let chars = line.trim().chars().collect::<Vec<_>>();

    let mut num_open: i32 = 0;
    let mut num_close: i32 = 0;

    for c in &chars {
        match c {
            '(' => {
                num_open += 1;
            }
            ')' => {
                num_close += 1;
            }
            _ => {}
        }
    }

    if chars.len() % 2 != 0 {
        println!("NO");
        return;
    }

    let mut num_open_for_question = (chars.len() as i32) / 2 - num_open;
    let mut num_close_for_question = (chars.len() as i32) / 2 - num_close;
    let mut count = 0;

    for c in &chars {
        match c {
            '(' => {
                count += 1;
            }
            ')' => {
                count -= 1;
            }
            '?' => {
                if 0 < num_open_for_question {
                    num_open_for_question -= 1;
                    count += 1;
                } else {
                    num_close_for_question -= 1;
                    count -= 1;
                }
            }
            _ => {}
        }

        if count < 0 {
            println!("NO");
            return;
        }
    }

    if count == 0 {
        println!("YES");
    } else {
        println!("NO");
    }

    return;
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        body();
    }
}
