use std::fs;


fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day2.txt";
    let contents: String = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");
    for line in lines {
        let values_iter: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let values = values_iter.map(|s| s.parse::<i64>().unwrap());

    }
}