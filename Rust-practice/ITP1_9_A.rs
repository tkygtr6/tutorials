// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_A

fn main() {
    let mut target_word = String::new();
    std::io::stdin().read_line(&mut target_word).unwrap();
    let target_word = String::from(target_word.trim().to_lowercase());

    let mut count = 0;

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let words: Vec<_> = buf.trim().split_whitespace().collect();

        if words[0] == "END_OF_TEXT" {
            break;
        }

        for word in words {
            if word.to_lowercase() == target_word {
                count += 1;
            } 
        }
    }

    println!("{}", count);
}

