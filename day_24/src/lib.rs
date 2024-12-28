
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ord;


use regex::Regex;
use strum_macros::{EnumString, Display};

type WireTable = HashMap<String, u8>;
type LogicTable = HashMap<(String, String), HashMap<Operation, HashSet<String>>>;
type SignalLogicTable = HashMap<(String, Operation), (String, String)>;

fn read_input_from_file(file_path: &str) -> (WireTable, LogicTable, SignalLogicTable) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");

    let mut wire_value_table: WireTable = HashMap::new();
    let mut signal_logic_table = SignalLogicTable::new();
    let mut logic_table: LogicTable = HashMap::new();
    let wire_re = Regex::new(r"(\w*): (\d)").unwrap();
    let logic_re = Regex::new(r"(\w*) (\w*) (\w*) -> (\w*)").unwrap();
    lines.iter().for_each(|line| {
        if let Some(caps) = wire_re.captures(&line) {
            let wire: String = caps.get(1).unwrap().as_str().parse().unwrap();
            let value: u8 = caps.get(2).unwrap().as_str().parse().unwrap();
            wire_value_table.insert(wire, value);
        } else if let Some(caps) = logic_re.captures(&line) {
            // println!("line: {line}");
            let wire_1: String = caps.get(1).unwrap().as_str().parse().unwrap();
            let operation: String = caps.get(2).unwrap().as_str().parse().unwrap();
            let wire_2: String = caps.get(3).unwrap().as_str().parse().unwrap();
            let result: String = caps.get(4).unwrap().as_str().parse().unwrap();
            let operation: Operation = operation.parse::<Operation>().unwrap();
            signal_logic_table.insert((wire_1.clone(), operation.clone()), (wire_2.clone(), result.clone()));
            // println!("w1: {wire_1}, w2: {wire_2}, result: {result}");
            let e = logic_table.entry((wire_1.clone(), wire_2.clone())).or_default();
            let ee = e.entry(operation.clone()).or_default();
            ee.insert(result.clone());
            let e = logic_table.entry((wire_2.clone(), wire_1.clone())).or_default();
            let ee = e.entry(operation.clone()).or_default();
            ee.insert(result.clone());
        }
    });
    return (wire_value_table, logic_table, signal_logic_table);
}

fn fill_outputs(mut wire_table: WireTable, logic_table: LogicTable) -> u64 {
    let mut new_revealed_wires: WireTable = WireTable::new();
    let mut visited: HashSet<(String, String)> = HashSet::new();
    // println!("wire_table: {:?}", wire_table);
    // println!("logic_table: {:?}", logic_table);
    loop {
        wire_table.keys()
        .flat_map(|k1| wire_table.keys().map(move |k2| (k1, k2)))
        .for_each(|(k1, k2) | {
            if !visited.contains(&(k1.clone(), k2.clone())) { 
                if let Some(entry_map) = logic_table.get(&(k1.clone(), k2.clone())) {
                    entry_map.iter().for_each(|(o, v)| {
                        v.iter().for_each(|wire_o| {
                            let result = operate(k1, k2, o.clone(), &wire_table);
                            new_revealed_wires.insert(wire_o.clone(), result);
                            visited.insert((k1.clone(), k2.clone()));
                        });
                    });
                }
            }
        });
        // println!("new_revealed: {:?}", new_revealed_wires);
        new_revealed_wires.iter().for_each(|(k, v)| { 
            wire_table.insert(k.clone(), v.clone());
        });
        if new_revealed_wires.is_empty() {break;}
        new_revealed_wires.clear();
    }
    let mut z_keys = wire_table.keys().filter(|k| k.starts_with("z")).map(|k| k.clone()).collect::<Vec<String>>();
    z_keys.sort();
    z_keys.reverse();
    // println!("z_keys: {:?}", z_keys);
    let output = z_keys.iter().map(|z_key| wire_table.get(z_key).unwrap().to_string()).collect::<String>();
    // println!("output: {output}");
    u64::from_str_radix(&output, 2).unwrap()
}


#[derive(Display, Debug, EnumString, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    AND,
    OR,
    XOR,
}

fn operate(operand_1: &String, operand_2: &String, operator: Operation, wire_table: &WireTable) -> u8 {
    let operand_1 = wire_table.get(operand_1).unwrap();
    let operand_2 = wire_table.get(operand_2).unwrap();
    match (operand_1, operand_2) {
        (0, 1) => match operator {
            Operation::AND => 0,
            _ => 1,
        },
        (1, 0) => match operator {
            Operation::AND => 0,
            _ => 1,
        },
        (1, 1) => match operator {
            Operation::XOR => 0,
            _ => 1,
        },
        (0, 0) => 0,
        _ => panic!(""),
    }
}

pub fn part_1(file_path: &str) -> u64 {
    let (wire_table, logic_table, _) = read_input_from_file(file_path);
    fill_outputs(wire_table, logic_table)
}

// Assumed and confirmed this architecture:
// https://www.101computing.net/binary-additions-using-logic-gates/
// Solution is mostly reverse engineered and cheated from here:
// https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3llouk/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
// Very ugly solution.
pub fn part_2(file_path: &str) -> String {
    let (_, _, signal_logic_table) = read_input_from_file(file_path);
    let mut originated_from: HashMap<String, Operation> = HashMap::new();
    let mut c0: String = Default::default();
    signal_logic_table.iter().for_each(|signal| {
        originated_from.insert(signal.1.1.clone(), signal.0.1.clone());
        if ((signal.0.0 == "x00" && signal.1.0 == "y00")
         || (signal.0.0 == "y00" && signal.1.0 == "x00")) && signal.0.1 != Operation::XOR {
            c0 = signal.1.1.clone();
        }
    });
    // println!("c0: {c0}");

    let mut suspects: HashSet<String> = HashSet::new();
    signal_logic_table.iter().for_each(|signal| {
        let (w1, operation, w2, result) = (signal.0.0.clone(), signal.0.1.clone(), signal.1.0.clone(), signal.1.1.clone());

        let starts_with_x_or_y = |signal: &String| -> bool {
            signal.starts_with("x") || signal.starts_with("y")
        };

        if operation == Operation::XOR {
            if starts_with_x_or_y(&w1) && starts_with_x_or_y(&w2) {
                if result.starts_with("z") && result != "z00" {
                    suspects.insert(result.clone());
                }
            } else if !starts_with_x_or_y(&w1) && !starts_with_x_or_y(&w2) {
                if !result.starts_with("z") {
                    suspects.insert(result.clone());
                }
            }
        // The operation is not XOR
        } else {
            // The operation is not XOR and result starts with z
            // All z's should be a result of an XOR operation (except for z45)
            if result.starts_with("z") && result != "z45" {
                suspects.insert(result.clone());
            }
            // input of OR should be output of AND, except for LSB.
            if operation == Operation::OR {
                if *originated_from.get(&w1).unwrap() != Operation::AND {
                    suspects.insert(w1.clone());
                }
                if *originated_from.get(&w2).unwrap() != Operation::AND {
                    suspects.insert(w2.clone());
                }
                // Two AND gates don't follow unless for c0 (carry out in the LSB)
            } else if operation == Operation::AND {
                if !starts_with_x_or_y(&w1) && w1 != c0 && *originated_from.get(&w1).unwrap() == Operation::AND {
                    suspects.insert(w1.clone());
                }
                if !starts_with_x_or_y(&w2) && w2 != c0 && *originated_from.get(&w2).unwrap() == Operation::AND {
                    suspects.insert(w2.clone());
                }
                
            }
        }
    }); 
    // println!("suspects: {:?}", suspects);  
    let mut suspects = suspects.into_iter().collect::<Vec<String>>();
    suspects.sort();
    suspects.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/input.txt"), 43942008931358);
        assert_eq!("dvb,fhg,fsq,tnc,vcf,z10,z17,z39", part_2("data/input.txt"));
    }
}
