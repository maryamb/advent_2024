use std::fs::File;
use std::io::{self, BufRead};


pub fn read_input_from_file(file_path: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    lines.iter().for_each(|line| {
        if line.starts_with("#") {
            warehouse.push(line.chars().collect());
        } else {
            moves.extend(line.chars().collect::<Vec<char>>());
        }
    });
    (warehouse, moves)
}

fn expand_warehouse(warehouse: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_w: Vec<Vec<char>> = Vec::new();
    warehouse.iter().for_each(|row| {
        let mut new_r: Vec<char> = Vec::new();
        row.iter().for_each(|c| {
            match c {
                '#' => { new_r.push('#'); new_r.push('#'); },
                '.' => { new_r.push('.'); new_r.push('.'); },
                '@' => { new_r.push('@'); new_r.push('.'); },
                'O' => { new_r.push('['); new_r.push(']'); },
                _ => panic!("Should not happen"),
            }
        });
        new_w.push(new_r);
    });
    new_w
}

fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    warehouse.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn get_robot_location(warehouse: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    warehouse.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, &ch)| {
            if ch == '@' {
                Some((i, j))  // Return the first (i, j) where '@' is found
            } else {
                None
            }
        })
    })
}

fn move_left(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize))  {
    let row = &mut warehouse[home.0];
    
    // Find the first empty space or obstacle in the row before 'home.1'
    let first_empty = row[0..home.1].iter_mut().rev().enumerate().find(|(j, ch)| {
        **ch == '.' || **ch == '#'
    }).map(|(j, ch)| (home.1 - 1 - j, ch));

    let (first_empty_index, first_empty_char) = first_empty.unwrap();
    if *first_empty_char == '#' {
        return;
    }

    // Move the '@' to the left and place 'O' where the previous '@' was
    row[home.1] = '.';         // Clear current position
    row[home.1 - 1] = '@';     // Move '@' left
    if first_empty_index != home.1 - 1 {row[first_empty_index] = 'O';} // Place 'O' at the found empty position
    home.1 -= 1;
}

fn move_right(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize)) {
    let row = &mut warehouse[home.0];
    
    // Find the first empty space or obstacle in the row before 'home.1'
    let first_empty = row[home.1 + 1..].iter_mut().enumerate().find(|(j, ch)| {
        **ch == '.' || **ch == '#'
    }).map(|(j, ch)| (home.1 + 1 + j, ch));

    let (first_empty_index, first_empty_char) = first_empty.unwrap();
    if *first_empty_char == '#' {
        return ;
    }

    // Move the '@' to the left and place 'O' where the previous '@' was
    row[home.1] = '.';         // Clear current position
    row[home.1 + 1] = '@';     // Move '@' left
    if first_empty_index != home.1 + 1 {row[first_empty_index] = 'O';} // Place 'O' at the found empty position
    home.1 += 1;
}

fn move_up(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize))  {
    
    // Find the first empty space or obstacle in the row before 'home.1'
    let first_empty = (0..home.0).into_iter().rev().find(|i| {
        warehouse[*i][home.1] == '#' || warehouse[*i][home.1] == '.'
    }).map(|i| (i, warehouse[i][home.1]));

    let (first_empty_index, first_empty_char) = first_empty.unwrap();
    if first_empty_char == '#' {
        return;
    }

    // Move the '@' to the left and place 'O' where the previous '@' was
    warehouse[home.0][home.1] = '.';         // Clear current position
    warehouse[home.0 - 1][home.1] = '@';     // Move '@' left
    if first_empty_index != home.0 - 1 {warehouse[first_empty_index][home.1] = 'O';} // Place 'O' at the found empty position
    home.0 -= 1;
}

fn move_down(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize))  {
    
    // Find the first empty space or obstacle in the row before 'home.1'
    let first_empty = (home.0 + 1..).into_iter().find(|i| {
        warehouse[*i][home.1] == '#' || warehouse[*i][home.1] == '.'
    }).map(|i| (i, warehouse[i][home.1]));

    let (first_empty_index, first_empty_char) = first_empty.unwrap();
    if first_empty_char == '#' {
        return;
    }

    // Move the '@' to the left and place 'O' where the previous '@' was
    warehouse[home.0][home.1] = '.';         // Clear current position
    warehouse[home.0 + 1][home.1] = '@';     // Move '@' left
    if first_empty_index != home.0 + 1 {warehouse[first_empty_index][home.1] = 'O';} // Place 'O' at the found empty position
    home.0 += 1;
}

use std::collections::{BTreeSet, VecDeque};

fn move_bbox_left(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut elements_on_the_way: Vec<(usize, usize)> = vec![];
    q.push_back(*home);
    let mut is_blocked = false;
    while !q.is_empty() {
        let head = q.pop_back().unwrap();
        let new_pos = (head.0, head.1 - 1);
        match warehouse[new_pos.0][new_pos.1] {
            '#' => {
                is_blocked = true;
                break;
            },
            '[' => {
                q.push_back(new_pos);
                elements_on_the_way.push(new_pos);
            },
            ']' => {
                q.push_back(new_pos);
                elements_on_the_way.push(new_pos);
            },
            _ => {},
        }
    }
    if !is_blocked {
        elements_on_the_way.iter().rev().for_each(|(x, y)| {
            warehouse[*x][y - 1] = warehouse[*x][*y];
        });
        warehouse[home.0][home.1 - 1] = '@';
        warehouse[home.0][home.1] = '.';
        home.1 -= 1;
    }
}

fn move_bbox_right(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut elements_on_the_way: Vec<(usize, usize)> = vec![];
    q.push_back(*home);
    let mut is_blocked = false;
    while !q.is_empty() {
        let head = q.pop_back().unwrap();
        let new_pos = (head.0, head.1 + 1);
        match warehouse[new_pos.0][new_pos.1] {
            '#' => {
                is_blocked = true;
                break;
            },
            '[' => {
                q.push_back(new_pos);
                elements_on_the_way.push(new_pos);
            },
            ']' => {
                q.push_back(new_pos);
                elements_on_the_way.push(new_pos);
            },
            _ => {},
        }
    }
    if !is_blocked {
        elements_on_the_way.iter().rev().for_each(|(x, y)| {
            warehouse[*x][y + 1] = warehouse[*x][*y];
        });
        warehouse[home.0][home.1 + 1] = '@';
        warehouse[home.0][home.1] = '.';
        home.1 += 1;
    }
}

fn move_bbox_down(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut elements_on_the_way: BTreeSet<(usize, usize)> = BTreeSet::new();
    q.push_back(*home);
    let mut is_blocked = false;
    while !q.is_empty() {
        let head = q.pop_back().unwrap();
        let new_pos = (head.0 + 1, head.1);
        match warehouse[new_pos.0][new_pos.1] {
            '#' => {
                is_blocked = true;
                break;
            },
            '[' => {
                q.push_back(new_pos);
                let matching_bbox = (new_pos.0, new_pos.1 + 1);
                q.push_back(matching_bbox);
                elements_on_the_way.insert(new_pos);
                elements_on_the_way.insert(matching_bbox);
            },
            ']' => {
                q.push_back(new_pos);
                let matching_bbox = (new_pos.0, new_pos.1 - 1);
                q.push_back(matching_bbox);
                elements_on_the_way.insert(new_pos);
                elements_on_the_way.insert(matching_bbox);
            },
            _ => {},
        }
    }
    if !is_blocked {
        elements_on_the_way.iter().rev().for_each(|(x, y)| {
            warehouse[x + 1][*y] = warehouse[*x][*y];
            warehouse[*x][*y] = '.';
        });
        warehouse[home.0 + 1][home.1] = '@';
        warehouse[home.0][home.1] = '.';
        home.0 += 1;
    }
}

fn move_bbox_up(warehouse: &mut Vec<Vec<char>>, home: &mut (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut elements_on_the_way: BTreeSet<(usize, usize)> = BTreeSet::new();
    q.push_back(*home);
    let mut is_blocked = false;
    while !q.is_empty() {
        let head = q.pop_back().unwrap();
        let new_pos = (head.0 - 1, head.1);
        match warehouse[new_pos.0][new_pos.1] {
            '#' => {
                is_blocked = true;
                break;
            },
            '[' => {
                q.push_back(new_pos);
                let matching_bbox = (new_pos.0, new_pos.1 + 1);
                q.push_back(matching_bbox);
                elements_on_the_way.insert(new_pos);
                elements_on_the_way.insert(matching_bbox);
            },
            ']' => {
                q.push_back(new_pos);
                let matching_bbox = (new_pos.0, new_pos.1 - 1);
                q.push_back(matching_bbox);
                elements_on_the_way.insert(new_pos);
                elements_on_the_way.insert(matching_bbox);
            },
            _ => {},
        }
    }
    if !is_blocked {
        // println!("elements_on_the_way: {:?}", elements_on_the_way);
        elements_on_the_way.iter().for_each(|(x, y)| {
            warehouse[x - 1][*y] = warehouse[*x][*y];
            warehouse[*x][*y] = '.';
        });
        warehouse[home.0 - 1][home.1] = '@';
        warehouse[home.0][home.1] = '.';
        home.0 -= 1;
    }
}

pub fn part_1(file_path: &str) -> usize {
    let (mut warehouse, moves) = read_input_from_file(file_path);
    let mut home = get_robot_location(&warehouse).unwrap();
    moves.iter().for_each(|m| {
        // println!("move: {}", m);
        match *m {
            '<' => move_left(&mut warehouse, &mut home),
            '>' => move_right(&mut warehouse, &mut home),
            '^' => move_up(&mut warehouse, &mut home),
            'v' => move_down(&mut warehouse, &mut home),
            _ => panic!("Should not happen"),
        }
    });
    print_warehouse(&warehouse);
    warehouse.iter().enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, ch)| {
        match ch {
            'O' => i * 100 + j,  
            _ => 0,
        }
    })).sum()
}

pub fn part_2(file_path: &str) -> usize {
    let (w, moves) = read_input_from_file(file_path);
    let mut warehouse = expand_warehouse(&w);
    let mut home = get_robot_location(&warehouse).unwrap();
    let num_boxes = warehouse.iter().map(|r| {
        r.iter().filter(|c| **c == '[').count()
    }).sum::<usize>();
    print_warehouse(&warehouse);
    println!("home: {:?}, num_boxes: {}", home, num_boxes);
    println!("------------------------------------------");
    moves.iter().for_each(|m| {
        let w_clone = warehouse.clone();
        match *m {
            '<' => move_bbox_left(&mut warehouse, &mut home),
            '>' => move_bbox_right(&mut warehouse, &mut home),
            '^' => move_bbox_up(&mut warehouse, &mut home),
            'v' => move_bbox_down(&mut warehouse, &mut home),
            _ => panic!("Should not happen"),
        }
        let nb = warehouse.iter().map(|r| {
            r.iter().filter(|c| **c == '[').count()
        }).sum::<usize>();
        if nb != num_boxes {
            print_warehouse(&w_clone);
            println!("move: {}", m);
            print_warehouse(&warehouse);
            println!("------------------------------------------");
            panic!("");
        }

    });
    print_warehouse(&warehouse);
    warehouse.iter().enumerate().map(|(i, r)| {
        r.iter().enumerate().filter(|(_, c)| {
            **c == '['
        }).map(|(j, _)| {
            i * 100 + j
        }).sum::<usize>()
    }).sum()
}

fn pg() {
    let (mut warehouse, moves) = read_input_from_file("data/example_1.txt");
    let mut home = get_robot_location(&warehouse).unwrap();
    print_warehouse(&warehouse);
    move_down(&mut warehouse, &mut home);
    print_warehouse(&warehouse);
    move_down(&mut warehouse, &mut home);
    print_warehouse(&warehouse);
    println!("moves: {:?}", moves);
}

fn pg_2() {
    let (mut w, moves) = read_input_from_file("data/example_2.txt");
    let mut home = get_robot_location(&w).unwrap();
    let warehouse = expand_warehouse(&w);
    print_warehouse(&w);
    println!("-------------------------------------------------------");
    print_warehouse(&warehouse);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/input.txt"), 1509863);
        assert_eq!(part_2("data/input.txt"), 1548815);
    }
}
