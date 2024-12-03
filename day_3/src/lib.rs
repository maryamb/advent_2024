
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn read_input_from_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create the output vector
    let output: String = reader.lines().map(|line| {
        line.expect("")
    }).collect();
    Ok(output)
}

pub fn get_sum_of_multiplies(file_path: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let input = read_input_from_file(file_path).expect("");
    re.find_iter(input.as_str()).map(|capture| {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let two_nums = re.captures(capture.as_str()).unwrap();
        let num_1: i32 = two_nums.get(1).unwrap().as_str().parse().unwrap();
        let num_2: i32 = two_nums.get(2).unwrap().as_str().parse().unwrap();
        num_1 * num_2
    }).sum()
}

pub fn get_sum_of_multiplies_with_disablers(file_path: &str) -> i32 {
    let re = Regex::new(
        r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let digits_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = read_input_from_file(file_path).expect("");
    let mut enable: bool = true;
    let mut sum_all = 0;
    re.find_iter(input.as_str()).for_each(|expression| {
        match expression.as_str() {
            "do()" => enable = true,
            "don't()" => enable = false,
            _ => if enable {
                let two_nums = digits_re.captures(expression.as_str()).unwrap();
                let num_1: i32 = two_nums.get(1).unwrap().as_str().parse().unwrap();
                let num_2: i32 = two_nums.get(2).unwrap().as_str().parse().unwrap();
                sum_all += num_1 * num_2;
            }
        }
    });
    sum_all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_sum_of_multiplies("data/input_1.txt"), 173731097);
        assert_eq!(get_sum_of_multiplies_with_disablers("data/input_1.txt"), 93729253);
    }
}
