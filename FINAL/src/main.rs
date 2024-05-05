mod graph;
mod bfs;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;
type Graph = HashMap<usize, Vec<usize>>;



fn main() {
   
    let graph = graph::read_graph("facebook_combined.txt");
    
    //bfs::print_bfs_from_every_node(&graph);
    
    //total number of vertices
    let num_vertices = bfs::get_num_vertices(&graph);
    println!("Number of vertices: {}", num_vertices);

    bfs::calculate_average_distance(&graph);
    //bfs::print_distance_statistics_from_each_node(&graph);

    // Counts the total number of distances for each distance 
    //Calculates the frequency that the distance occurs throughout the graph
    bfs::print_max_distance_counts(&graph);

    // Calculate and print the average number of nodes at each distance from every node
    bfs::print_average_distances_nodes_count(&graph);

    
        
}
    














