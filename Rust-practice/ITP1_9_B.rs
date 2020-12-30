fn main() {
    loop {
        let mut dataset = String::new();
        std::io::stdin().read_line(&mut dataset).unwrap();
        let dataset = dataset.trim();

        if dataset == "-" {
            return;
        }

        let mut m = String::new();

        std::io::stdin().read_line(&mut m).unwrap();
        let m: i32 = m.trim().parse().unwrap();

        let mut sum = 0;
        for _i in 0..m {
            let mut num = String::new();
            std::io::stdin().read_line(&mut num).unwrap();
            let num: i32 = num.trim().parse().unwrap();
            sum += num;
        }
        let sum = sum as usize;

        let dataset_len = dataset.chars().count();
        println!(
            "{}{}",
            &dataset[sum % dataset_len..],
            &dataset[0..sum % dataset_len]
        );
    }
}
