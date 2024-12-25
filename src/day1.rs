use std::fs;

fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day1.txt";
    let contents: String = fs::read_to_string(file_path)
        .expect("Could not read file");

    let lines: std::str::Split<'_, &str> = contents.split("\n");
    let mut col_1: Vec<i32> = Vec::new();
    let mut col_2: Vec<i32> = Vec::new();
    for line in lines {
        let mut values_iter: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let col_1_val: i32 = values_iter.next().unwrap().parse().unwrap();
        let col_2_val: i32 = values_iter.next().unwrap().parse().unwrap();
        col_1.push(col_1_val);
        col_2.push(col_2_val);
    }

    // Sort the lists then gather the differences
    col_1.sort();
    col_2.sort();
    let mut sum: i32 = 0;
    for (val_1, val_2) in col_1.iter().zip(col_2.iter()) {
        sum += (val_1 - val_2).abs();
    }
    println!("Total difference is {}", sum);

    let mut sim_score = 0;
    for val in col_1.iter() {
        let occurrences = col_2.iter().filter(|v| *v == val).count();
        sim_score += val * occurrences as i32;
    }
    println!("Total similarity score is {}", sim_score);

}
