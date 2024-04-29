use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Graph = HashMap<usize, Vec<usize>>;

pub fn print_graph(graph: &Graph) {
    for (node, neighbors) in graph {
        println!("{} -> {:?}", node, neighbors);
    }
}

pub fn read_graph(filename: &str) -> (usize, Graph) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut edges = vec![];
    let mut max_index = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<usize> = line.split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap())
                                    .collect();
        if parts.len() >= 2 {
            let from = parts[0];
            let to = parts[1];
            edges.push((from, to));
            max_index = max_index.max(from).max(to);
        }
    }

    let mut graph = HashMap::new();
    for (from, to) in edges {
        graph.entry(from).or_insert_with(Vec::new).push(to);
    }

    (max_index + 1, graph)
}

pub fn bfs(start: usize, num_vertices: usize, graph: &Graph) {
    let mut distances = vec![-1; num_vertices]; // Ensure it covers all vertices
    distances[start] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if neighbor < distances.len() && distances[neighbor] == -1 { // Check bounds and unvisited
                    distances[neighbor] = distances[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
    }

    println!("Distances from node {}:", start);
    for (index, &distance) in distances.iter().enumerate() {
        if distance != -1 {
            println!("{} -> {}", index, distance);
        }
    }
}

fn main() {
    let (num_vertices, graph) = read_graph("facebook_combined.txt");

    println!("Number of vertices: {}", num_vertices);
    println!("Graph representation:");
    print_graph(&graph);

    for node in 0..num_vertices {
        if graph.contains_key(&node) {
            bfs(node, num_vertices, &graph);
        }
    }
}














