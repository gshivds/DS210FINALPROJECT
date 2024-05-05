use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
type Graph = HashMap<usize, Vec<usize>>;

pub fn read_graph(filename: &str) -> Graph {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<usize> = line.split_whitespace()
                                    .map(|s| s.parse().expect("Failed to parse usize"))
                                    .collect();
        if parts.len() == 2 {
            let (from, to) = (parts[0], parts[1]);
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default().push(from); // Assuming undirected graph
        }
    }
    graph
}
















