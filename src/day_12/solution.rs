use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let nodes = get_nodes();
    let visited: HashSet<String> = HashSet::from([String::from("start")]);
    let solution = count_paths(&nodes, String::from("start"), visited);
    println!("{}", solution);
}

pub fn part_2() {
    let nodes = get_nodes();
    let visited: Vec<String> = Vec::new();
    let solution = count_paths_2(&nodes, String::from("start"), visited);
    println!("{}", solution);
}

fn count_paths(nodes: &Graph, current: String, visited: HashSet<String>) -> u64 {
    if current == "end" {
        return 1;
    }

    let current_str: &str = current.as_ref();
    let empty_hash_set: HashSet<String> = HashSet::new();
    let neighbours: &HashSet<String> = nodes.get(current_str).unwrap_or(&empty_hash_set);

    let available_nodes =
        neighbours.iter()
                  .filter(|&n| is_big(n) || !visited.contains(n));

    return available_nodes.map(|next| {
        let mut next_visited = visited.clone();
        next_visited.insert(current.clone());
        return count_paths(nodes, next.clone(), next_visited);
    }).sum();
}

fn count_paths_2(nodes: &Graph, current: String, visited: Vec<String>) -> u64 {
    if current == "end" {
        return 1;
    }

    let current_str: &str = current.as_ref();
    let empty_hash_set: HashSet<String> = HashSet::new();
    let neighbours: &HashSet<String> = nodes.get(current_str).unwrap_or(&empty_hash_set);

    let mut next_visited = visited.clone();
    next_visited.push(current.clone());

    return
        neighbours.iter()
            .filter(|&n| can_visit(&next_visited,n))
            .map(|next| count_paths_2(nodes, next.clone(), next_visited.clone()))
            .sum();
}

fn can_visit(visited: &Vec<String>, id: &String) -> bool {
    if id == "start" {
        return false;
    }

    if is_big(id) {
        return true;
    }

    if !visited.contains(id) {
        return true;
    }

    let mut visit_counts: HashMap<String, u64> = HashMap::new();

    for cave_id in visited.iter().filter(|&v| !is_big(v)) {
        let current = visit_counts.entry(cave_id.clone()).or_insert(0);
        *current += 1;
        if *current > 1 {
            return false;
        }
    }

    return true;
}

type Graph = HashMap<String, HashSet<String>>;

// Returns a map of Node ID -> Navigable Node IDs
fn get_nodes() -> Graph {
    let mut node_map: Graph = HashMap::new();

    for line in read_file_as_lines("day_12/input.txt") {
        let (a, b) = parse_line(&line);

        let current_a = node_map.entry(a.clone()).or_insert(HashSet::new());
        (*current_a).insert(b.clone());

        let current_b = node_map.entry(b.clone()).or_insert(HashSet::new());
        (*current_b).insert(a.clone());
    }

    return node_map;
}

fn parse_line(line: &String) -> (String, String) {
    let parts = line.split("-").collect_vec();
    let from = String::from(parts[0]);
    let to = String::from(parts[1]);
    return (from, to);
}

fn is_big(id: &String) -> bool {
    return id.to_uppercase() == *id;
}
