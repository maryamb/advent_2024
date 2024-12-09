use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

type Matrix = HashMap<char, Vec<(usize, usize)>>;

fn read_input_from_file(file_path: &str) -> (Matrix, usize, usize) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    let matrix = lines.iter().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .map(move |(j, c)| (c.clone(), (i, j)))
    })
    .filter(|c| c.0 != '.')
    .fold(HashMap::new(), |mut acc, (c, pos)| {
        acc.entry(c)
            .or_insert_with(Vec::new)
            .push(pos);
        acc
    });
    (matrix, lines.len(), lines[0].len())
}

fn is_within_bounds(p: &(i32, i32), bounds: &(i32, i32)) -> bool {
    p.0 >= 0 && p.1 >= 0 &&
    p.0 < bounds.0 && p.1 < bounds.1
}

fn get_pair_antinodes(loc_1: &(usize, usize), loc_2: &(usize, usize), bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let loc_1 = (loc_1.0 as i32, loc_1.1 as i32);
    let loc_2 = (loc_2.0 as i32, loc_2.1 as i32);
    let bounds = (bounds.0 as i32, bounds.1 as i32);
    let delta = (loc_1.0 - loc_2.0, loc_1.1 - loc_2.1);
    let p1 = (loc_1.0 + delta.0, loc_1.1 + delta.1);
    let p2 = (loc_2.0 - delta.0, loc_2.1 - delta.1);
    [p1, p2].iter().filter(|p| is_within_bounds(p, &bounds)).map(|p| (p.0 as usize, p.1 as usize)).collect()
}

fn get_antinodes(loc_1: &(usize, usize), loc_2: &(usize, usize), bounds: &(usize, usize), resonant_harmonics: bool) -> Vec<(usize, usize)> {
    let loc_1 = (loc_1.0 as i32, loc_1.1 as i32);
    let loc_2 = (loc_2.0 as i32, loc_2.1 as i32);
    let bounds = (bounds.0 as i32, bounds.1 as i32);
    let delta = (loc_1.0 - loc_2.0, loc_1.1 - loc_2.1);
    let i_upper = match resonant_harmonics {
        true => 1 + bounds.0,
        _ => 2
    };
    let j_upper = match resonant_harmonics {
        true => 1 + bounds.1,
        _ => 2
    };
    let mut r1: Vec<(usize, usize)> = (1..i_upper).take_while(|&i| {
        is_within_bounds(&(loc_1.0 + i * delta.0, loc_1.1 + i * delta.1), &bounds)
    }).map(|i| ((loc_1.0 + i * delta.0) as usize, (loc_1.1 + i * delta.1) as usize)).collect::<Vec<(usize, usize)>>();
    let r2 = (1..j_upper).take_while(|&j| {
        is_within_bounds(&(loc_2.0 - j * delta.0, loc_2.1 - j * delta.1), &bounds)
    }).map(|j| ((loc_2.0 - j * delta.0) as usize, (loc_2.1 - j * delta.1) as usize)).collect::<Vec<(usize, usize)>>();
    r1.extend(r2);

    r1
}

pub fn get_antinode_locations(file_path: &str, resonant_harmonics: bool) -> usize {
    let (matrix, m, n) = read_input_from_file(file_path);
    let bounds = (m, n);
    let mut result = HashSet::<(usize, usize)>::new();
    // Flattening the nested iterators using flat_map
    let locations = matrix.iter().flat_map(|(_, v)| {
        v.iter().enumerate().flat_map(|(i, p1)| {
            v[i + 1..].iter().enumerate().map(move |(_, p2)| {
                get_antinodes(p1, p2, &bounds, resonant_harmonics)
            })
        })
    })
    .fold(HashSet::<(usize, usize)>::new(), |mut acc, v| {
        // Flatten the result from get_pair_antinodes and extend the HashSet
        acc.extend(v);
        acc
    });
    if resonant_harmonics {
        matrix.iter().for_each(|v| {
            v.1.iter().for_each(|p| {result.insert(*p);});
        });
    }
    result.extend(locations);
    result.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_antinode_locations("data/example.txt", false), 14);
        assert_eq!(get_antinode_locations("data/input.txt", false), 423);
        assert_eq!(get_antinode_locations("data/input.txt", true), 1287);
        assert_eq!(get_antinode_locations("data/example_2.txt", true), 9);
        assert_eq!(get_antinode_locations("data/example.txt", true), 34);
    }
}
