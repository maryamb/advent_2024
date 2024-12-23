use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


pub fn read_input_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    lines.iter().map(|line| {
        line.parse::<i64>().unwrap()
    }).collect()
}

fn mix(s: &mut i64, input: &i64) {
    *s = *s ^ *input;
}

fn prune(s: &mut i64) {
    *s = *s % 16777216;
}

fn step(s: &mut i64) {
    let tmp = *s << 6;
    mix(s, &tmp);
    prune(s);

    let tmp = *s >> 5;
    mix(s, &tmp);
    prune(s);

    let tmp = *s << 11;
    mix(s, &tmp);
    prune(s);
}

fn step_n(s: &mut i64, n: i32) {
    (0..n).into_iter().for_each(|_| {
        step(s);
    });
}

fn get_max_price(secrets: &Vec<i64>, num_steps: i64) -> i64 {
    let all_seq_maps: Vec<HashMap<(i64, i64, i64, i64), i64>> = secrets.iter()
        .map(|s| get_seq_map(s, num_steps))
        .collect();
    let mut integrated_seq_maps: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    all_seq_maps.iter().for_each(|seq_map| {
        seq_map.iter().for_each(|(k, v)| {
            *integrated_seq_maps.entry(*k).or_insert(0) += v;
        });
    });
    *integrated_seq_maps.values().max().unwrap()
}

fn get_seq_map(secret: &i64, num_steps: i64) -> HashMap<(i64, i64, i64, i64), i64> {
    let secrets = (0..num_steps - 1).fold(vec![*secret], |mut acc, _| {
        let mut next_secret = *acc.last().unwrap();
        step(&mut next_secret);
        acc.push(next_secret);
        acc
    });
    let prices: Vec<i64> = secrets.iter()
        .map(|s| {
            *s % 10
        })
        .collect();
    let diffs: Vec<i64> = prices
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    
    let mut seq_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    diffs.windows(4)
        .enumerate()
        .for_each(|(i, w)| {
            seq_map.entry((w[0], w[1], w[2], w[3])).or_insert(prices[i + 4]);
    });
    seq_map
}

pub fn part_1() -> i64 {
    read_input_from_file("data/input.txt")
        .iter_mut()
        .map(|s| {
            step_n(s, 2000);
            *s
        })
        .sum()
}

pub fn part_2() -> i64 {
    let secrets = read_input_from_file("data/input.txt");
    get_max_price(&secrets, 2000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s: i64 = 42;
        mix(&mut s, &15);
        assert_eq!(s, 37);
        let mut s: i64 = 100000000;
        prune(&mut s);
        assert_eq!(s, 16113920);
        let mut s: i64 = 2024;
        step_n(&mut s, 2000);
        assert_eq!(s, 8667524);
        let mut s: i64 = 100;
        step_n(&mut s, 2000);
        assert_eq!(s, 15273692);
        let mut s: i64 = 10;
        step_n(&mut s, 2000);
        assert_eq!(s, 4700978);
        let mut s: i64 = 1;
        step_n(&mut s, 2000);
        assert_eq!(s, 8685429);
        assert_eq!(part_1(), 18317943467);
        assert_eq!(part_2(), 2018);
    }
}
