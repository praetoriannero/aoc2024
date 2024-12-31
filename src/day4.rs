use std::fs;


fn main() {
    let file_path: &str = "D:\\source\\aoc2024\\aoc\\src\\day4.txt";
    let contents: String = fs::read_to_string(file_path).unwrap();
    let xmas_vec: Vec<char> = "XMAS".chars().collect();
    let xmas_vec_rev: Vec<char> = "SAMX".chars().collect();
    let char_count: usize = 140;
    let mut char_matrix: Vec<char> = vec![];
    for char in contents.chars() {
        if xmas_vec.contains(&char) {
            char_matrix.push(char);
        }
    }
    let mut xmas_lines = [[[0_i32; 2]; 4]; 4];
    for i in 0..=3 {
        xmas_lines[0][i] = [-(i as i32), -(i as i32)];
        xmas_lines[1][i] = [0, -(i as i32)];
        xmas_lines[2][i] = [i as i32, -(i as i32)];
        xmas_lines[3][i] = [i as i32, 0];
    }
    let mut total = 0;
    let mut char_vec: Vec<char> = Vec::new();
    println!("{:?}", xmas_lines);
    for i in 0..char_count {
        for j in 0..char_count {
            'line_loop: for line in xmas_lines {
                char_vec.clear();
                for char_pos in line {
                    if i as i32 + char_pos[0] >= char_count as i32 || j as i32 + char_pos[1] >= char_count as i32 {
                        continue 'line_loop;
                    }
                    if i as i32 + char_pos[0] < 0 || j as i32 + char_pos[1] < 0 {
                        continue 'line_loop;
                    }
                    let row: i32 = (i as i32 + char_pos[0]) * char_count as i32;
                    let col: i32 = j as i32 + char_pos[1];
                    let pos: Result<usize, std::num::TryFromIntError> = usize::try_from(row + col);
                    if pos.is_err() {
                        continue 'line_loop;
                    }
                    char_matrix.get(pos.unwrap()).map(|c| char_vec.push(*c));
                }
                total += (char_vec == xmas_vec || char_vec == xmas_vec_rev) as i32;
            }
        }
    }
    println!("{}", total);

    total = 0;
    let mut xmas_lines = [[[0; 2]; 3]; 2];
    for i in [-1, 1] {
        xmas_lines[0] = [[-i, -i], [0, 0], [i, i]];
        xmas_lines[1] = [[-i, i], [0, 0], [i, -i]];
    }
    let mas_vec: Vec<char> = vec!['S', 'A', 'M'];
    let mas_vec_rev: Vec<char> = vec!['M', 'A', 'S'];
    for i in 0..char_count {
        for j in 0..char_count {
            let mut mas_count = 0;
            'line_loop: for line in xmas_lines {
                char_vec.clear();
                for char_pos in line {
                    if i as i32 + char_pos[0] >= char_count as i32 || j as i32 + char_pos[1] >= char_count as i32 {
                        continue 'line_loop;
                    }
                    if i as i32 + char_pos[0] < 0 || j as i32 + char_pos[1] < 0 {
                        continue 'line_loop;
                    }
                    let row: i32 = (i as i32 + char_pos[0]) * char_count as i32;
                    let col: i32 = j as i32 + char_pos[1];
                    let pos: Result<usize, std::num::TryFromIntError> = usize::try_from(row + col);
                    if pos.is_err() {
                        continue 'line_loop;
                    }
                    char_matrix.get(pos.unwrap()).map(|c| char_vec.push(*c));
                }
                mas_count += (char_vec == mas_vec || char_vec == mas_vec_rev) as i32;
            }
            if mas_count == 2 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
 