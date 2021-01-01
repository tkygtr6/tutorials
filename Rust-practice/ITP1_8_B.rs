fn main() {
    loop {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        let num = num.trim().chars().collect::<Vec<char>>();

        if num.len() == 1 && num[0] == '0' {
            break;
        }

        let mut sum: u32 = 0;
        for digit in num {
            sum += digit.to_digit(10).unwrap();
        }

        println!("{}", sum);
    }
}
