use std::fs;

fn validate(values: &Vec<i64>) -> bool {
    let min_diff = 1;
    let max_diff = 3;
    let mut is_incrementing: Option<bool> = None;
    let mut last_val: i64 = i64::MAX;
    for val in values {
        if last_val == i64::MAX {
            last_val = *val;
        } else {
            let mut diff: i64 = val - last_val;
            let sign: bool = diff > 0;
            if is_incrementing.is_none() {
                is_incrementing = Some(sign);
            } else {
                if is_incrementing.unwrap() != sign {
                    return false;
                }
            }
            diff = diff.abs();
            if diff < min_diff || diff > max_diff {
                return false;
            }
            last_val = *val;
        }
    }
    true
}

fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day2.txt";
    let contents: String = fs::read_to_string(file_path).unwrap();
    let lines: std::str::Split<'_, &str> = contents.split("\n");
    let mut total: i64 = 0;
    for line in lines {
        let values = line
            .split_whitespace()
            .map(|s: &str| s.parse::<i64>().unwrap());
        let values_arr: Vec<i64> = values.into_iter().collect();
        if validate(&values_arr) {
            total += 1;
        } else {
            for idx in 0..values_arr.len() {
                let mut temp_arr = values_arr.clone();
                temp_arr.remove(idx);
                if validate(&temp_arr) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{}\n", total);
}
