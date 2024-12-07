use std::fs::File;
use std::io::{self, BufRead};

fn read_input_from_file(file_path: &str) -> Vec<(i64, Vec<i64>)> {
    // Open the file
    let file = File::open(file_path).expect("");
    let reader = io::BufReader::new(file);

    // Create the output vector
    reader.lines().map(|line| {
        let line = line.expect("");
        let mut first_split = line.split(':');
        let calibration: i64 = first_split.next().expect("").trim().parse().expect("");
        let nums = first_split.next().expect("").trim().split(" ").map(|c| {
            c.parse().expect("")
    }).collect::<Vec<i64>>();
        (calibration, nums)
    }).collect::<Vec<(i64, Vec<i64>)>>()
}

fn get_mutations(nums: &[i64], do_concat: bool) -> Vec<i64> {
    if nums.len() == 1 {
        return vec![*nums.last().expect("")];
    }
    let rest = get_mutations(&nums[0.. nums.len() - 1], do_concat);
    let plus_rest = rest.iter().map(|r| r + nums.last().expect("")).collect::<Vec<i64>>();
    let mul_rest = rest.iter().map(|r| r * nums.last().expect("")).collect::<Vec<i64>>();
    let concat_res = if do_concat {
        rest.iter().map(|r| (r.to_string() + nums.last().expect("").to_string().as_str()).parse::<i64>().expect("")).collect::<Vec<i64>>()
    } else {vec![]};
    
    let mut result = plus_rest;
    result.extend(mul_rest);
    result.extend(concat_res);
    result
}

pub fn get_matches_sum(file_path: &str, do_concat: bool) -> i64 {
    let input = read_input_from_file(file_path);
    input.iter().filter(|(calibration, nums)| {
        get_mutations(nums, do_concat).contains(calibration)}).map(|(c, _)| c).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_matches_sum("data/example.txt", false), 3749);
        assert_eq!(get_matches_sum("data/input.txt", false), 3);
        assert_eq!(get_matches_sum("data/example.txt", true), 11387);
        assert_eq!(get_matches_sum("data/input.txt", true), 11387);
    }
}
