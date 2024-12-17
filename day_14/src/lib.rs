use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{Ord, Ordering};
use std::usize;

use regex::Regex;
use rayon::prelude::*;

pub fn read_input_from_file(file_path: &str) -> Vec<Robot> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    let re = Regex::new(r"p=([-]?[0-9]+),([-]?[0-9]+) v=([-]?[0-9]+),([-]?[0-9]+)").unwrap();

    let result = lines.iter().fold(Vec::<Robot>::new(), |mut robots, line| {
        if let Some(caps) = re.captures(&line) {
            robots.push(Robot{
                p: Point{
                    x: caps.get(1).unwrap().as_str().parse().unwrap(), 
                    y: caps.get(2).unwrap().as_str().parse().unwrap()},
                v: Velocity {
                    dx: caps.get(3).unwrap().as_str().parse().unwrap(),
                    dy: caps.get(4).unwrap().as_str().parse().unwrap(),
                },
                cache: HashMap::new(),
            });
        }
        robots
    });

    result
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Clone, Debug)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Debug)]
struct Robot {
    p: Point,
    v: Velocity,
    cache: HashMap<(i32, i32), (i32, i32)>,
}

impl Robot {
    fn step(&mut self, max_x: i32, max_y: i32) {
        let entry = self.cache.get(&(self.p.x, self.p.y));
        match entry {
            None => {
                let x = (self.p.x + self.v.dx + max_x) % max_x;
                let y = (self.p.y + self.v.dy + max_y) % max_y;
                self.cache.insert((self.p.x, self.p.y), (x, y));
                self.p.x = x;
                self.p.y = y;
            }
            Some(n) => {
                self.p.x = n.0;
                self.p.y = n.1;
            }
        }
    }
    fn step_n(&mut self, max_x: i32, max_y: i32, n: i32) {
        (0..n).into_iter().for_each(|_| self.step(max_x, max_y));
    }
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by x first
        if self.p.x != other.p.x {
            self.p.x.cmp(&other.p.x)
        } else {
            // If x is the same, compare by y
            self.p.y.cmp(&other.p.y)
        }
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.p.x == other.p.x && self.p.y == other.p.y
    }
}

impl Eq for Robot {}


pub fn part_1(file_path: &str, max_x: i32, max_y: i32, count: i32) -> usize {
    let mut robots = read_input_from_file(file_path);
    // robots.iter().for_each(|r| {println!("{:?}", r.p)});

    robots.iter_mut().for_each(|r| r.step_n(max_x, max_y, count));
    
    let q1 = robots.iter().filter(|&r| r.p.x < max_x / 2 && r.p.y < max_y / 2).count();
    let q2 = robots.iter().filter(|r| r.p.x < max_x / 2 && r.p.y > max_y / 2).count();
    let q3 = robots.iter().filter(|r| r.p.x > max_x / 2 && r.p.y < max_y / 2).count();
    let q4 = robots.iter().filter(|r| r.p.x > max_x / 2 && r.p.y > max_y / 2).count();
    q1 * q2 * q3 * q4
}

fn print_scene(robots: &Vec<Vec<usize>>) {
    robots.iter().for_each(|rr| {
        rr.iter().for_each(|r| {
            match r {
                0 => print!(" "),
                x => print!("{}", x),
            }
        });
        println!();
    });
    println!();
    println!();
    println!();
}

fn convert_to_map(input: &Vec<Robot>, max_x: i32, max_y: i32) -> Vec<Vec<usize>> {
    // Note we are saving for each y, a vector of xs.
    let mut maze: Vec<Vec<usize>> = vec![vec![0; max_x as usize]; max_y as usize];
    input.into_iter().for_each(|r| {
        let p = &r.p.clone();
        maze[p.y as usize][p.x as usize] += 1; });
    maze
}

fn get_std(robots: &Vec<Robot>) -> f32 {
    let mean: f32 = robots.iter().map(|r| {r.p.y as f32}).sum();
    let std = robots.iter().map(|r| (r.p.y as f32 - mean).abs()).sum();
    std
}

fn is_symmetric(maze: &Vec<Vec<usize>>) -> bool {
    // Note: This approach didn't work since it checks for exact symmetry. 
    // Instead, I should have checked for number of symmetries.
    let m = maze[0].len();  
    maze.par_iter().all(|row| {
        (0.. 1 + m/2).into_par_iter().all(|i| {
            (row[i] == 0 && row[m - 1 - i] == 0) || 
            (row[i] != 0 && row[m - 1 - i] != 0)
            // maze[i] == maze[m - 1 - i]
        })
    })    
}

fn measure_of_continuity(maze: &Vec<Vec<usize>>) -> usize {
    let m = maze[0].len();
    maze.iter().map(|row| {
        row.windows(2).into_iter().filter(|w| {
            w[1] != 0 && w[0] != 0
        }).count()
    }).sum::<usize>() +
    (0..m).into_iter().map(|r_ind| {
        maze.windows(2).into_iter().filter(move |w| {
            w[1][r_ind] != 0 && w[0][r_ind] != 0
        }).count()
    }).sum::<usize>()
}

pub fn part_2(mut robots: Vec<Robot>, max_x: i32, max_y: i32) -> Option<usize> {
    // let mut min_std_so_far: f32 = 1e10;
    let mut max_continuity: usize = 0;
    return (0..).position( |i| {
        if i % 1000 == 0 {
            print!("{}\t", i);
        }
        robots.par_iter_mut().for_each(|r| {
            r.step(max_x, max_y);
        });
        let maze = convert_to_map(&robots, max_x, max_y);
        let new_continuity = measure_of_continuity(&maze);
        if new_continuity > max_continuity {
            max_continuity = new_continuity;
            println!("\n-----------------------------------------------------------------------------------\n");
            println!("iteration: {}", i);
            print_scene(&maze);
            println!("\n-----------------------------------------------------------------------------------\n");
        }
        false
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut r = Robot{p: Point{x: 2, y: 4}, v: Velocity{dx: 2, dy: -3}};
        // r.step_n(11, 7, 5);
        // println!("{:?}", r.p);
        // let robots_vec = read_input_from_file("data/example.txt");
        // let maze = convert_to_map(&robots_vec, 11, 7);
        // print_scene(&maze);
        let robots_vec = read_input_from_file("data/input.txt");
        part_2(robots_vec, 101, 103);
        // assert_eq!(part_1("data/example.txt", 11, 7, 100), 12);
        // assert_eq!(part_1("data/input.txt", 101, 103, 100), 221616000);
    }
}
