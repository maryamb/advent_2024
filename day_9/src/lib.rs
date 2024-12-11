use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn read_input_from_file(file_path: &str) -> Vec<u32> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    lines[0].chars()
    .map(|c| c.to_digit(10).expect("input is safe")).collect::<Vec<u32>>()
}

fn unzip_map(input_str: Vec<u32>) -> u64 {
    let mut id: u32 = 0;
    let mut result: String = String::new();
    input_str.iter().enumerate().for_each(|(i, count)| {
        let id_str = id.to_string();
        match i % 2 {
            0 => result.extend(std::iter::repeat(id_str).take(*count as usize)) ,  // file
            1 => {
                result.extend(std::iter::repeat('.').take(*count as usize));  // empty
                id += 1;
            },
            _ => println!("this should not happen: i = {}", i),
        }
    });
    // println!("result 1: {:?}", result);
    let mut i: usize = 0;
    let mut j: usize = result.len() - 1;
    let mut result_chars: Vec<char> = result.chars().collect(); // Convert to vector of chars

    loop {
        if i >= j {
            break;
        }
        
        // If result[i] is '.' and result[j] is not '.'
        if result_chars[i] == '.' && result_chars[j] != '.' {
            result_chars[i] = result_chars[j];
            result_chars[j] = '.';
            i += 1;
            j -= 1;
            continue;
        }
        
        // If result[i] is not '.' and result[j] is '.', continue
        if result_chars[i] != '.' && result_chars[j] == '.' {
            i += 1;
            j -= 1;
            continue;
        }
        
        // If both are '.', just decrement j
        if result_chars[i] == '.' && result_chars[j] == '.' {
            j -= 1;
            continue;
        }
        
        // If neither is '.', increment i
        if result_chars[i] != '.' && result_chars[j] != '.' {
            i += 1;
            continue;
        }
    }
    // println!("result_char: {:?}", result_chars);
    result_chars.iter().take_while(|c| **c != '.').enumerate().map(|(i, c)| i as u64 * c.to_digit(10).expect("") as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(unzip_map(vec![2,3,3,3,1,3,3,1,2,1,4,1,4,1,3,1,4,0,2]), 1928);
        assert_eq!(unzip_map(read_input_from_file("data/input.txt")), 1928);
    }
}
