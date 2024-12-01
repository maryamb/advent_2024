use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_file_and_store_columns(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create two vectors to store the columns
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    reader.lines().for_each(|line| {
        let line: String = line.expect("");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if numbers.len() == 2 {
            col1.push(numbers[0]);
            col2.push(numbers[1]);
        }
    });

    Ok((col1, col2)) // Return the two vectors
}

fn sum_all_diffs(mut col1: Vec<i32>, mut col2: Vec<i32>) -> i32 {
    col1.sort();
    col2.sort();
    col1.iter().zip(col2.iter())
    .map(|(a, b)| (a - b).abs())
    .sum()
}

fn sum_of_diffs_from_file(file_path: &str) -> i32{
    let (col1, col2) = read_file_and_store_columns(file_path).expect("");
    sum_all_diffs(col1, col2)
}

fn get_similarity_measure(col1: Vec<i32>, col2: Vec<i32>) -> i32 {
    let mut frequency_map:  HashMap<i32, i32> = HashMap::new();
    col2.iter().for_each(|&key| {
        *frequency_map.entry(key).or_insert(0) += 1;
    });
    col1.iter()
    .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
    .sum()
}

fn get_similarity_measure_from_file(file_path: &str) -> i32 {
    let (col1, col2) = read_file_and_store_columns(file_path).expect("");
    get_similarity_measure(col1, col2)
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_example_1() {
        assert_eq!(sum_of_diffs_from_file("data/example_1.txt"), 11);
        assert_eq!(sum_of_diffs_from_file("data/input_1.txt"), 2113135);
        assert_eq!(get_similarity_measure_from_file("data/example_1.txt"), 31);
        assert_eq!(get_similarity_measure_from_file("data/input_1.txt"), 19097157);
    }
}
