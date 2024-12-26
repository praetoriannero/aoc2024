use std::fs;


fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day2.txt";
    let contents: String = fs::read_to_string(file_path).unwrap();
    let lines: std::str::Split<'_, &str> = contents.split("\n");
    let max_diff: i64 = 3;
    let min_diff: i64 = 1;
    let mut total: i64 = 0;
    let mut is_incrementing: Option<bool>;
    for line in lines {
        is_incrementing = None;
        let values = line
            .split_whitespace()
            .map(|s: &str| s.parse::<i64>().unwrap());
        let mut last_val: i64 = i64::MAX;
        for val in values {
            if last_val == i64::MAX {
                last_val = val;
            } else {
                let mut diff: i64 = val - last_val;
                let sign: bool = diff > 0;
                diff = diff.abs();
                if sign {
                    if is_incrementing.is_none() {

                    }
                }
                if diff > 0 && diff >= min_diff && diff <= max_diff {
                    total += 1;
                } else {
                    continue;
                }
            }
        }
    }

}