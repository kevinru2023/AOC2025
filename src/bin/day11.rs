use aoc2025::utils;
use std::collections::{HashMap, HashSet, VecDeque};

fn build_graph(input: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input {
        let node_end = line.find(':').unwrap();
        let node = line[..node_end].to_string();
        let mut outgoing_connections: HashSet<String> = HashSet::new();
        for outgoing in line[node_end + 1..].split_ascii_whitespace() {
            outgoing_connections.insert(outgoing.to_string());
        }
        graph.insert(node, outgoing_connections);
    }
    graph
}

// really brute force solution but basically check check count paths
// from src to target
fn part_1(input: &Vec<String>) {
    let graph = build_graph(input);
    let start: String = "you".to_string(); // bro.
    let mut paths = 0;

    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(start.clone());

    while let Some(node) = q.pop_front() {
        if node == "out" {
            paths += 1;
            continue;
        }
        for neighbor in graph.get(&node).unwrap() {
            q.push_back(neighbor.clone());
        }
    }
    println!("Part 1: {}", paths);
}


// super similar approach to the one in https://leetcode.com/problems/all-paths-from-source-to-target/editorial/
fn part_2(input: &Vec<String>) {
    let graph = build_graph(input);
    let start: String = "svr".to_string();
    let target: String = "out".to_string();

    // these have to be in the path or else it's an invalid path
    let req1: String = "dac".to_string();
    let req2: String = "fft".to_string();

    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();

    fn dfs(
        node: &str,
        target: &str,
        has_req1: bool,
        has_req2: bool,
        req1: &str,
        req2: &str,
        graph: &HashMap<String, HashSet<String>>,
        memo: &mut HashMap<(String, bool, bool), usize>,
    ) -> usize {
        if node == target {
            return if has_req1 && has_req2 { 1 } else { 0 };
        }

        // check memo
        let state = (node.to_string(), has_req1, has_req2);
        if let Some(&cached) = memo.get(&state) {
            return cached;
        }

        let new_has_req1 = has_req1 || node == req1;
        let new_has_req2 = has_req2 || node == req2;

        let mut total_paths = 0;
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                total_paths += dfs(
                    neighbor,
                    target,
                    new_has_req1,
                    new_has_req2,
                    req1,
                    req2,
                    graph,
                    memo,
                );
            }
        }

        memo.insert(state, total_paths);
        total_paths
    }

    let paths = dfs(
        &start, &target, false, false, &req1, &req2, &graph, &mut memo,
    );
    println!("Part 2: {}", paths);
}

fn main() {
    let input: Vec<String> = utils::read_lines(11);

    part_1(&input);
    part_2(&input);
}
