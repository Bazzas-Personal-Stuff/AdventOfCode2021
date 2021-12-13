use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));


    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    for line in reader.lines(){
        let line_str = line.unwrap();
        let mut split = line_str.split("-");
        let node_a = split.next().unwrap().to_string();
        let node_b = split.next().unwrap().to_string();
        // node_vec.push(node_a);
        // let a_ref= node_vec.last().unwrap();
        // node_vec.push(node_b);
        // let b_ref = node_vec.last().unwrap();

        if !nodes.contains_key(&node_a) {
            nodes.insert(node_a.clone(), Vec::new());
        }
        if !nodes.contains_key(&node_b) {
            nodes.insert(node_b.clone(), Vec::new());
        }

        nodes.get_mut(&node_a).unwrap().push(node_b.clone());
        nodes.get_mut(&node_b).unwrap().push(node_a.clone());
    }

    let path_count = dfs("start".to_string(), &nodes, &mut visited, false);

    println!("Number of paths that only visit small caves once: {}", path_count);
}

fn dfs(cur_node: String, nodes: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, visit_2: bool) -> u64 {
    let mut valid_paths = 0;
    let mut is_second_visit = false;
    if cur_node == "end" {
        return 1;
    }
    if visited.contains(&cur_node) {
        if !visit_2 {
            is_second_visit = true;
        }
        else {
            return 0;
        }
    }

    if cur_node.chars().next().unwrap().is_ascii_lowercase() {
        visited.insert(cur_node.clone());
    }

    for node in &nodes[&cur_node] {
        if node != &"start" {
            valid_paths += dfs(node.clone(), nodes, visited, visit_2 || is_second_visit);
        }
    }
    if !is_second_visit {
        visited.remove(&cur_node);
    }

    return valid_paths;
}
