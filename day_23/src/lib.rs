use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

type Graph = HashMap<String, HashSet<String>>;
type ConnectedComponent = BTreeSet<String>;

pub fn read_input_from_file(file_path: &str) -> (Graph, HashSet<String>) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read lines");
    let re = Regex::new(r"(\w*)-(\w*)").unwrap();
    let mut graph: Graph = Graph::new();
    lines.iter().for_each(|line| {
        if let Some(re_caps) = re.captures(&line) {
            let n1: String = re_caps.get(1).unwrap().as_str().parse().unwrap();
            let n2: String = re_caps.get(2).unwrap().as_str().parse().unwrap();
            graph.entry(n1.clone()).or_insert(HashSet::new()).insert(n2.clone());
            graph.entry(n2).or_insert(HashSet::new()).insert(n1);
        }
    });
    let t_keys = graph.keys().filter(|k| k.starts_with("t")).map(|s| s.clone()).collect::<HashSet<String>>();
    (graph, t_keys)
}

fn get_connected_components(graph: &Graph, key: &String) -> BTreeSet<String> {
    let mut output: BTreeSet<String> = BTreeSet::new();
    let key_neighbours = graph.get(key).unwrap();
    key_neighbours.iter().for_each(|n| {
        let neighbour_neighbours = graph.get(n).unwrap();
        let common_neighbours = key_neighbours.intersection(neighbour_neighbours).collect::<Vec<&String>>();
        common_neighbours.iter().for_each(|cn| {
            let mut cc: ConnectedComponent = ConnectedComponent::new();
            cc.insert(key.clone());
            cc.insert(n.clone());
            cc.insert(cn.to_string());
            let cc_str: String = cc.iter().map(|s| s.clone()).collect();
            output.insert(cc_str);
        });
    });
    output
}

pub fn part_1(file_path: &str) -> usize {
    let (graph, t_keys) = read_input_from_file(file_path);
    let mut all_ccs: HashSet<String> = HashSet::new();
    t_keys.iter().for_each(|tk| {
        all_ccs.extend(get_connected_components(&graph, tk));
    });
    // println!("{:?}", all_ccs);
    all_ccs.len()
}

fn get_key_weights(graph: &Graph) -> HashMap<String, usize> {
    graph.iter().fold(HashMap::new(), |mut acc, n| {
        acc.insert(n.0.clone(), n.1.len());
        acc
    })
}

fn clique(candidates: &BTreeSet<String>, current_clique: HashSet<String>, graph: &Graph) -> HashSet<String> {
    if candidates.is_empty() {
        return current_clique.clone();
    }

    let mut candidates = candidates.clone();
    let mut current_clique = current_clique.clone();

    let a = candidates.pop_first().unwrap();
    let a_neighbours = match graph.get(&a) {
        None => return clique(&candidates, current_clique, graph),
        Some(neighbours) => neighbours,
    };

    // First branch: try without 'a'
    let max_clique_without_a = clique(&candidates, current_clique.clone(), graph);

    // Check if 'a' can be added to the current clique
    let can_a_be_in_clique = current_clique.iter().all(|c| a_neighbours.contains(c));
    
    if !can_a_be_in_clique {
        return max_clique_without_a;
    }

    // Second branch: try with 'a'
    current_clique.insert(a.clone());
    let max_clique_with_a = clique(&candidates, current_clique, graph);

    // Return the larger of the two cliques
    if max_clique_with_a.len() > max_clique_without_a.len() {
        max_clique_with_a
    } else {
        max_clique_without_a
    }
}

pub fn part_2(file_path: &str) -> String {
    let (graph, _) = read_input_from_file(file_path);
    let mut visited: HashSet<String> = HashSet::new();
    let mut largest_clique: HashSet<String> = HashSet::new();
    graph.iter().for_each(|node| {
        if !visited.contains(node.0) {
            visited.insert(node.0.clone());
            let node_neighbours = node.1;
            visited.extend(node_neighbours.clone().into_iter().collect::<HashSet<String>>());
            let mut current_clique = HashSet::new();
            current_clique.insert(node.0.clone());
            let candidates: BTreeSet<String> = node_neighbours.clone().into_iter().collect();
            let clique_originated_from_node = clique(&candidates, current_clique, &graph);
            if clique_originated_from_node.len() > largest_clique.len() {
                largest_clique = clique_originated_from_node;
            }
        }
    });
    let mut largest_clique = largest_clique.into_iter().collect::<Vec<String>>();
    largest_clique.sort();
    largest_clique.join(",")
}

pub fn get_largest_cc(graph: &Graph) {
    let a = graph.iter().map(|g| g.1.len()).max().unwrap();
    println!("a: {a}.");
}

pub fn pg() {
    let a = read_input_from_file("data/input.txt");
    let graph = a.0;
    let s = graph.iter().map(|kv| kv.1.len()).max();
    println!("max size {}", s.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("data/example.txt"), 7);
        assert_eq!(part_1("data/input.txt"), 1308);
        assert_eq!(part_2("data/input.txt"), "bu,fq,fz,pn,rr,st,sv,tr,un,uy,zf,zi,zy");
    }
}
