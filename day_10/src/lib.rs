use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{BTreeSet, HashSet};

fn read_input_from_file(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");
    lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect()
}

fn get_trailheads(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    input.iter().enumerate()
    .flat_map(|(i, v)| {
        v.iter().enumerate()
        .filter(|(_, c)| **c == '0')
        .map(move |(j, _)| (i, j))
    }).collect()
}

fn get_trailends(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    input.iter().enumerate()
    .flat_map(|(i, v)| {
        v.iter().enumerate()
        .filter(|(_, c)| **c == '9')
        .map(move |(j, _)| (i, j))
    }).collect()
}

fn get_down_neighbours(map: &Vec<Vec<char>>, home: (usize, usize)) -> Vec<(usize, usize)> {
    if map[home.0][home.1] == '0' {
        return vec![];
    }
    let m = map.len() as isize;
    let n = map[0].len() as isize;
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter(|(i, j)| {
        let ii = i + home.0 as isize;
        let jj = j + home.1 as isize;
        ii >= 0 && ii < m && jj >= 0 && jj < n &&
        map[ii as usize][jj as usize] != '.' && 
        map[ii as usize][jj as usize].to_digit(10).unwrap() + 1 == map[home.0][home.1].to_digit(10).unwrap()
    }).map(|(i, j)| (i + home.0 as isize, j + home.1 as isize)).map(|(i, j)| (i as usize, j as usize)).collect()
}

fn get_up_neighbours(map: &Vec<Vec<char>>, home: (usize, usize)) -> Vec<(usize, usize)> {
    if map[home.0][home.1] == '9' {
        return vec![];
    }
    let m = map.len() as isize;
    let n = map[0].len() as isize;
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter(|(i, j)| {
        let ii = i + home.0 as isize;
        let jj = j + home.1 as isize;
        ii >= 0 && ii < m && jj >= 0 && jj < n &&
        map[ii as usize][jj as usize] != '.' && 
        map[ii as usize][jj as usize].to_digit(10).unwrap() == map[home.0][home.1].to_digit(10).unwrap() + 1
    }).map(|(i, j)| (i + home.0 as isize, j + home.1 as isize)).map(|(i, j)| (i as usize, j as usize)).collect()
}

fn print_2d_vector<T: std::fmt::Debug>(vector: &Vec<Vec<T>>) {
    for row in vector {
        println!("{:?}", row);
    }
    println!();
}

pub fn travel(map: &Vec<Vec<char>>) -> usize {
    let ends = get_trailends(map);
    let m = map.len();
    let n = map[0].len();
    let mut dp_table: Vec<Vec<HashSet<(usize, usize)>>> = vec![vec![HashSet::new(); n]; m];
    let mut q1 = BTreeSet::<(usize, usize)>::new();
    let mut q2 = BTreeSet::<(usize, usize)>::new();
    
    
    ends.iter().for_each(|(i, j)| {
        dp_table[*i][*j].insert((*i, *j)); 
        get_down_neighbours(map, (*i, *j)).iter().for_each(|(ni, nj)| { q2.insert((*ni, *nj)); }); });
    let mut sum_ranks: usize = 0;
    loop {
        if q1.is_empty() && q2.is_empty() { break sum_ranks; }
        if q1.is_empty() { q1 = q2; q2 = BTreeSet::<(usize, usize)>::new(); }
        let node = q1.pop_first().unwrap();
        get_up_neighbours(map, node).iter().for_each(|(ni, nj)| {
            let tmp = dp_table[*ni][*nj].clone();
            dp_table[node.0][node.1].extend(tmp.iter());
        });
        if map[node.0][node.1] == '0' {
            sum_ranks += dp_table[node.0][node.1].len();
        }
        get_down_neighbours(map, node).iter().for_each(|(ni, nj)| { 
            q2.insert((*ni, *nj));
        });
    }
}

pub fn travel_rating(map: &Vec<Vec<char>>) -> usize {
    let ends = get_trailends(map);
    let m = map.len();
    let n = map[0].len();
    let mut dp_table: Vec<Vec<usize>> = vec![vec![0; n]; m];
    let mut q1 = BTreeSet::<(usize, usize)>::new();
    let mut q2 = BTreeSet::<(usize, usize)>::new();
    
    
    ends.iter().for_each(|(i, j)| {
        dp_table[*i][*j] = 1; 
        get_down_neighbours(map, (*i, *j)).iter().for_each(|(ni, nj)| { q2.insert((*ni, *nj)); });
    });
    let mut sum_ranks: usize = 0;
    loop {
        if q1.is_empty() && q2.is_empty() { break sum_ranks; }
        if q1.is_empty() { q1 = q2; q2 = BTreeSet::<(usize, usize)>::new(); }
        let node = q1.pop_first().unwrap();
        get_up_neighbours(map, node).iter().for_each(|(ni, nj)| {
            dp_table[node.0][node.1] += dp_table[*ni][*nj];
        });
        if map[node.0][node.1] == '0' {
            sum_ranks += dp_table[node.0][node.1];
        }
        get_down_neighbours(map, node).iter().for_each(|(ni, nj)| { 
            q2.insert((*ni, *nj));
        });
    }
}


pub fn part_1(file_path: &str) -> usize {
    let map = read_input_from_file(file_path);
    travel(&map)
}

pub fn part_2(file_path: &str) -> usize {
    let map = read_input_from_file(file_path);
    travel_rating(&map)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/example_4.txt"), 36);
        assert_eq!(part_1("data/input.txt"), 538);
        assert_eq!(part_2("data/example_4.txt"), 81);
        assert_eq!(part_2("data/input.txt"), 1110);
    }
}
