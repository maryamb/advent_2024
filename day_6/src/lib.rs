use std::fs::File;
use std::io::{self, BufRead};

pub type Maze = Vec<Vec<char>>;

pub fn print_maze(maze: &Vec<Vec<char>>) {
    for row in maze {
        // Print each row as a string
        println!("{:?}", row.iter().collect::<String>());
    }
    println!("");
}

fn read_input_from_file(file_path: &str) -> io::Result<Maze> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut graph: Maze = Maze::new();

    // Create the output vector
    reader.lines().for_each(|line| {
        let line = line.expect("");
        graph.push(line.chars().collect::<Vec<char>>());
    });
    Ok(graph)
}
#[derive(Clone, PartialEq)]
struct Dir {
    x: i32,
    y: i32
}

impl Dir {
    fn next(self) -> Result<Dir, String>{
        match (self.x, self.y) {
            (-1, 0) => Ok(Dir{x: 0, y: 1}), 
            (0, 1) => Ok(Dir{x: 1, y: 0}), 
            (1, 0) => Ok(Dir{x: 0, y: -1}), 
            (0, -1) => Ok(Dir{x: -1, y: 0}),
            _ => Err("Should not happen".to_string()),
        }
    }
}

fn find_home(maze: &Maze) -> (usize, usize) {
    maze.iter().enumerate()
    .find_map(|(i, row)| {
        row.iter().enumerate()
            .find(|(_, element)| **element == '^')
            .map(|(j, _)| (i, j))
    }).unwrap()
}

fn turn(x: usize, y: usize, dir: Dir, maze: &Maze) -> (usize, usize, Dir) {
    let i = (x as i32 + dir.x) as usize;
    let j = (y as i32 + dir.y) as usize;
    if maze[i][j] == '#' || maze[i][j] == 'o' {
        return turn(x, y, dir.next().expect(""), maze);
    }
    return (i, j, dir);
}

pub fn part_1(file_path: &str) -> i32 {
    let mut maze = read_input_from_file(file_path).expect("");
    let m = maze.len() as i32;
    let n = maze[0].len() as i32;
    let (mut x, mut y) = find_home(&maze);
    let mut dir = Dir{x: -1, y: 0};  // Up
    let mut count = 0;
    loop {
        if maze[x][y] != 'x' {
            maze[x][y] = 'x';
            count += 1;
        };
        let i = x as i32 + dir.x;
        let j = y as i32 + dir.y;
        if i < 0 || j < 0 || i >= m || j >= n { 
            // print_maze(&maze);
            return count;
        }
        (x, y, dir) = turn(x, y, dir, &maze);
    }
}

pub fn is_current_obstacle_loopable(maze: &Maze, ox: usize, oy: usize, hx: usize, hy: usize) -> bool {
    let mut maze = maze.clone();
    maze[ox][oy] = 'o';
    let m = maze.len() as i32;
    let n = maze[0].len() as i32;
    let mut dir_history: Vec<Vec<Option<Dir>>> = vec![vec![None; maze.len()]; maze[0].len()];
    let (mut x, mut y) = (hx, hy);
    let mut dir = Dir{x: -1, y: 0};  // Up
    loop {
        if let Some(history) = &dir_history[x][y] {
            if *history == dir {
                return true;
            }
        }
        if maze[x][y] != 'x' {
            maze[x][y] = 'x';
            dir_history[x][y] = Some(dir.clone());
        };
        let i = x as i32 + dir.x;
        let j = y as i32 + dir.y;
        if i < 0 || j < 0 || i >= m || j >= n { 
            // print_maze(&maze);
            return false;
        }
        (x, y, dir) = turn(x, y, dir, &maze);
    }
}

pub fn part_2(file_path: &str) -> usize {
    let maze = read_input_from_file(file_path).expect("");
    let (hx, hy) = find_home(&maze);
    maze.iter().enumerate().map(|(i, v)| {
        v.iter().enumerate().filter(|(j, &e)| {
             e == '.' && is_current_obstacle_loopable(&maze, i, *j, hx, hy) }).count()  }).sum::<usize>().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/example.txt"), 41);
        assert_eq!(part_1("data/input.txt"), 4819);
        assert_eq!(part_2("data/example.txt"), 6);
        assert_eq!(part_2("data/input.txt"), 1796);
    }
}