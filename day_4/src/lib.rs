use std::fs::File;
use std::io::{self, BufRead};

type Matrix = Vec<Vec<char>>;

fn read_input_from_file(file_path: &str) -> io::Result<Matrix> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create the output vector
    let output: Matrix = reader.lines().map(|line| {
        line.expect("").chars().collect::<Vec<char>>()
    }).collect();
    Ok(output)
}


fn match_in_direction(matrix: &Matrix, i: usize, j: usize, di: i32, dj: i32) -> i32 {
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;
    let ii = i as i32;
    let jj = j as i32;
    if  ii + 3 * di >= 0 &&
        ii + 3 * di < m &&
        jj + 3 * dj >= 0 &&
        jj + 3 * dj < n &&
        matrix[i][j] == 'X' &&
        matrix[(ii + di) as usize][(jj + dj) as usize] == 'M' &&
        matrix[(ii + di + di) as usize][(jj + dj + dj) as usize] == 'A' &&
        matrix[(ii + di + di + di) as usize][(jj + dj + dj + dj) as usize] == 'S' { 1 } else { 0 }
}


fn xmas_from_index(matrix: &Matrix, i: usize, j: usize) -> i32 {
    match_in_direction(matrix, i, j, 0, 1) + 
    match_in_direction(matrix, i, j, 0, -1) +
    match_in_direction(matrix, i, j, 1, 0) +
    match_in_direction(matrix, i, j, -1, 0) +
    match_in_direction(matrix, i, j, 1, 1) +
    match_in_direction(matrix, i, j, 1, -1) +
    match_in_direction(matrix, i, j, -1, 1) +
    match_in_direction(matrix, i, j, -1, -1)
}

fn central_xmas_from_index(matrix: &Matrix, i: i32, j: i32) -> i32 {
    if matrix[i as usize][j as usize] != 'A' { return 0; }

    if matrix[(i - 1) as usize][(j - 1) as usize] == 'M' && matrix[(i - 1) as usize][(j + 1) as usize] == 'M' && 
       matrix[(i + 1) as usize][(j - 1) as usize] == 'S' && matrix[(i + 1) as usize][(j + 1) as usize] == 'S' {return 1;} 
    if matrix[(i - 1) as usize][(j - 1) as usize] == 'M' && matrix[(i - 1) as usize][(j + 1) as usize] == 'S' && 
       matrix[(i + 1) as usize][(j - 1) as usize] == 'M' && matrix[(i + 1) as usize][(j + 1) as usize] == 'S' {return 1;} 

    if matrix[(i - 1) as usize][(j - 1) as usize] == 'S' && matrix[(i - 1) as usize][(j + 1) as usize] == 'S' && 
       matrix[(i + 1) as usize][(j - 1) as usize] == 'M' && matrix[(i + 1) as usize][(j + 1) as usize] == 'M' {return 1;} 
    if matrix[(i - 1) as usize][(j - 1) as usize] == 'S' && matrix[(i - 1) as usize][(j + 1) as usize] == 'M' && 
       matrix[(i + 1) as usize][(j - 1) as usize] == 'S' && matrix[(i + 1) as usize][(j + 1) as usize] == 'M' {return 1;} 

    0
}

pub fn xmas_from_file(file_path: &str) -> i32 {
    let matrix: Matrix = read_input_from_file(file_path).expect("");
    let m = matrix.len();
    let n = matrix[0].len();
    (0..m)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .map(|(i, j)| xmas_from_index(&matrix, i, j))
        .sum()
}

pub fn central_xmas_from_file(file_path: &str) -> i32 {
    let matrix: Matrix = read_input_from_file(file_path).expect("");
    let m = matrix.len();
    let n = matrix[0].len();
    (1..m - 1)
        .flat_map(|i| (1..n - 1).map(move |j| (i, j)))
        .map(|(i, j)| central_xmas_from_index(&matrix, i as i32, j as i32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(xmas_from_file("data/example.txt"), 18);
        assert_eq!(xmas_from_file("data/input.txt"), 2639);
        
        assert_eq!(central_xmas_from_file("data/example.txt"), 9);
        assert_eq!(central_xmas_from_file("data/input.txt"), 2005);
    }
}
