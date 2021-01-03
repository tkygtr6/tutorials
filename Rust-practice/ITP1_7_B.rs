fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let nums = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let n = nums[0];
        let sum = nums[1];

        if n == 0 && sum == 0 {
            break;
        }

        let mut count = 0;
        for a in 1..=n {
            for b in a + 1..=n {
                for c in b + 1..=n {
                    let calc = a + b + c;
                    if calc == sum {
                        count += 1;
                    }
                }
            }
        }

        println!("{}", count);
    }
}
