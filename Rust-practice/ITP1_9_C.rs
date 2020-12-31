fn compare_letter(s: &Vec<char>, t: &Vec<char>) -> i32 {
    let min_length: i32 = std::cmp::min(s.len() as i32, t.len() as i32);
    for idx in 0..min_length {
        if s[idx as usize] < t[idx as usize] {
            return 1;
        } else if s[idx as usize] > t[idx as usize] {
            return -1;
        } else {
            continue;
        }
    }

    if s.len() < t.len() {
        return 1;
    } else if s.len() > t.len() {
        return -1;
    }

    return 0;
}

fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();

    let mut points = [0, 0];

    for _ in 0..num {
        let mut lines = String::new();
        std::io::stdin().read_line(&mut lines).unwrap();
        let lines: Vec<_> = lines.trim().split_whitespace().collect();

        let mut cards: Vec<Vec<char>> = [].to_vec();
        for line in lines {
            let card: Vec<char> = line.chars().collect();
            cards.push(card);
        }

        match compare_letter(&cards[0], &cards[1]) {
            -1 => {
                points[0] += 3;
            }
            0 => {
                points[0] += 1;
                points[1] += 1;
            }
            1 => {
                points[1] += 3;
            }
            _ => {}
        }
    }

    println!("{} {}", points[0], points[1]);
}
