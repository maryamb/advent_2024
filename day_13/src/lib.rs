use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

use num_traits::{cast, Num, NumCast, Zero};

pub fn read_input_from_file<T>(file_path: &str) -> Vec<Machine<T>> 
where 
    T: std::str::FromStr + Clone,
    T: Num + Zero,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    let mut result: Vec<Machine<T>> = Vec::new();
    let a_re = Regex::new(r"Button A: X([+-]?[0-9]+), Y([+-]?[0-9]+)").unwrap();
    let b_re = Regex::new(r"Button B: X([+-]?[0-9]+), Y([+-]?[0-9]+)").unwrap();

    let p_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut a: Button<T> = Button::<T>{x: T::zero(), y: T::zero()};
    let mut b: Button<T> = Button::<T>{x: T::zero(), y: T::zero()};

    lines.iter().for_each(|line| {
        if let Some(a_caps) = a_re.captures(&line) {
            a = Button{x: a_caps.get(1).unwrap().as_str().parse::<T>().unwrap(), y: a_caps.get(2).unwrap().as_str().parse().unwrap()};
        }
        if let Some(b_caps) = b_re.captures(&line) {
            b = Button{x: b_caps.get(1).unwrap().as_str().parse().unwrap(), y: b_caps.get(2).unwrap().as_str().parse().unwrap()};
        }
        if let Some(p_caps) = p_re.captures(&line) {
            let p = Prize{x: p_caps.get(1).unwrap().as_str().parse().unwrap(), y: p_caps.get(2).unwrap().as_str().parse().unwrap()};
            result.push(Machine{a_button: a.clone(), b_button: b.clone(), prize: p});
        }
    });
    result
}

#[derive(Clone, Debug)]
struct Button<T> {
    x: T,
    y: T,
}

#[derive(Clone, Debug)]
struct Prize<T> {
    x: T,
    y: T,
}

#[derive(Clone, Debug)]
pub struct Machine<T> {
    a_button: Button<T>,
    b_button: Button<T>,
    prize: Prize<T>,
}

impl<T: std::ops::Mul> Machine<T> {
    pub fn is_feasible(&self) -> T 
    where 
        T: Num + NumCast + Copy + std::str::FromStr,
    {
        let a1 = &self.a_button;
        let a2 = &self.b_button;
        let det = a1.x * a2.y - a1.y * a2.x;
        if det == T::zero() {return T::zero();}
        let p = &self.prize;
        let n1_num = p.x * a2.y - a2.x * p.y;
        let n2_num = a1.x * p.y - p.x * a1.y;
        let n1 = n1_num / det;
        let n2 = n2_num / det;
        if n1 * a1.x + n2 * a2.x == p.x && n1 * a1.y + n2 * a2.y == p.y {
            return n1 * cast(3).unwrap() + n2;
        }
        T::zero()
    }
}

pub fn part_1(file_path: &str) -> i32 {
    read_input_from_file::<i32>(file_path).iter().map(|machine| machine.is_feasible()).sum()
}

pub fn part_2(file_path: &str) -> i64 {
    read_input_from_file::<i64>(file_path).iter_mut().map(|machine| {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        machine.is_feasible()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/example.txt"), 480);
        assert_eq!(part_1("data/input.txt"), 35574);
        assert_eq!(part_2("data/example.txt"), 875318608908);
        assert_eq!(part_2("data/input.txt"), 80882098756071);
    }
}
