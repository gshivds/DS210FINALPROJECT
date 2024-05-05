use std::collections::{HashMap, VecDeque};
use rand::prelude::*;
type Graph = HashMap<usize, Vec<usize>>;

pub fn bfs_distance(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((node, dist)) = queue.pop_front() {
        if !distances.contains_key(&node) {
            distances.insert(node, dist);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !distances.contains_key(&neighbor) {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }
    }
    distances
}

pub fn bfs_from_every_node(graph: &Graph) {
    for &node in graph.keys() {
        let distances = bfs_distance(graph, node);
        println!("Distances from node {} -> {:?}", node, distances);
        
    }
}

pub fn calculate_average_distance(graph: &Graph) {
    let mut rng = thread_rng();
    let node_keys: Vec<&usize> = graph.keys().collect();
    let mut total_distance = 0.0;
    let mut count = 0.0;

    for _ in 0..3000 {
        let &start = node_keys.choose(&mut rng).unwrap();
        let distances = bfs_distance(graph, *start);
        for &distance in distances.values() {
            total_distance += distance as f64;
            count += 1.0;
        }
    }
    println!("Average distance: {}", total_distance / count);
}


pub fn get_num_vertices(graph: &Graph) -> usize {
    graph.len()
}

pub fn distance_statistics_for_each_node(graph: &Graph) {
    for &node in graph.keys() {
        let distances = bfs_distance(graph, node);
        let mut distance_counts = [0; 9]; // Indices 0-8 for distances 0-8
        for &dist in distances.values() {
            if dist <= 8 {
                distance_counts[dist] += 1;
            }
        }

        println!("Node {}: Distance counts up to 8:", node);
        for dist in 1..=8 {
            println!("  Distance {} -> {}", dist, distance_counts[dist]);
        }
    }
}

pub fn max_distance_counts(graph: &Graph) {
    // Store max count and corresponding nodes for each distance 1 to 8
    let mut max_counts = vec![(0, Vec::new()); 8]; // (count, nodes)
    let mut total_counts = vec![0; 9]; // Index 0-8 for distances 0-8
   
   
    for &node in graph.keys() {
        let distances = bfs_distance(graph, node);
        let mut distance_counts = vec![0; 9]; // Index 0-8 for distances 0-8

        // Count distances
        for &dist in distances.values() {
            if dist <= 8 {
                distance_counts[dist] += 1;
                total_counts[dist] += 1;
            }
        }

        // Update max counts for each distance
        for i in 1..=8 {
            if distance_counts[i] > max_counts[i-1].0 {
                max_counts[i-1] = (distance_counts[i], vec![node]);
            } else if distance_counts[i] == max_counts[i-1].0 {
                max_counts[i-1].1.push(node);
            }
        }
    }

    // Print results
    for i in 0..8 {
        let max_count = max_counts[i].0;
        let total_count = total_counts[i + 1]; // distance 1 corresponds to index 1 in total_counts
        let frequency = if total_count != 0 { max_count as f64 / total_count as f64 } else { 0.0 };
        println!("Distance {}: Max count = {}, Frequency = {}", i + 1, total_count, frequency);
    }
}

pub fn average_distances_nodes_count(graph: &Graph) {
    let max_distance = 8; // Calculate up to distance 8
    let mut total_counts = vec![0; max_distance + 1]; // Store total counts for each distance
    let mut node_counts = vec![0; max_distance + 1]; // Store number of nodes contributing to each distance

    for &node in graph.keys() {
        let distances = bfs_distance(graph, node);

        let mut local_counts = vec![0; max_distance + 1];
        for &dist in distances.values() {
            if dist <= max_distance {
                local_counts[dist] += 1;
            }
        }

        for dist in 1..=max_distance {
            if local_counts[dist] > 0 {
                total_counts[dist] += local_counts[dist];
                node_counts[dist] += 1;
            }
        }
    }
    // Print average number of nodes at each distance
    println!("Average number of nodes at each distance:");
    for dist in 1..=max_distance {
        if node_counts[dist] > 0 {
            let average = total_counts[dist] as f64 / node_counts[dist] as f64;
            println!("Distance {}: {:.2}", dist, average);
        } else {
            println!("Distance {}: No data", dist);
        }
    }
}










