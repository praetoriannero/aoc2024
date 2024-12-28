use regex::{Regex, Match};
use std::{fs, usize};


enum RegexCase {
    DEFAULT,
    DO,
    DONT,
    MULT
}


fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day3.txt";
    let contents: String = fs::read_to_string(file_path).unwrap();
    let re_mul = Regex::new(r"mul\((?<num1>\d+),\s*(?<num2>\d+)\)").unwrap();
    let mut total_sum = 0;
    for (_, [num1, num2]) in re_mul.captures_iter(&contents).map(|c| c.extract()) {
        total_sum += num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap();
    }
    println!("{}", total_sum);

    total_sum = 0;
    let re_do = Regex::new(r"do()").unwrap();
    let re_dont: Regex = Regex::new(r"don't()").unwrap();
    let regex_arr: [&Regex; 3] = [&re_mul, &re_do, &re_dont];
    let case_arr: [&RegexCase; 3] = [&RegexCase::MULT, &RegexCase::DO, &RegexCase::DONT];
    let mut contents_slice: &str = &contents[..];
    let mut active: bool = true;
    loop {
        let mut temp_case: &RegexCase = &RegexCase::DEFAULT;
        let mut start_pos: usize = usize::MAX;
        let mut end_pos: usize = usize::MIN;
        for (re, case) in regex_arr.into_iter().zip(case_arr) {
            let re_found: Option<Match<'_>> = re.find(contents_slice);
            re_found.map(|m: Match<'_>| if m.start() <= start_pos {
                start_pos = m.start();
                end_pos = m.end();
                temp_case = case;
            });
        }
        match temp_case {
            RegexCase::DO => active = true,
            RegexCase::DONT => active = false,
            RegexCase::MULT => {
                if active {
                    let re_mul_capture: Option<regex::Captures<'_>> = re_mul.captures(&contents_slice);
                    total_sum += re_mul_capture
                        .as_ref()
                        .unwrap()
                        .name("num1")
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap()
                        * re_mul_capture
                            .as_ref()
                            .unwrap()
                            .name("num2")
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap();
                    }
                }
            RegexCase::DEFAULT => break
        };
        contents_slice = &contents_slice[end_pos..];
    }
    println!("{}", total_sum);
}
