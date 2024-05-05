mod graph;
mod bfs;
use std::collections::HashMap;
type Graph = HashMap<usize, Vec<usize>>;

fn main() {
   
    let graph = graph::read_graph("facebook_combined.txt");
    //this the print the nodes that are connected to each node in the graph and the number of the connections the node its connected to has
    bfs::bfs_from_every_node(&graph);

    //Counts the number of distances for each distance for every node
    bfs::distance_statistics_for_each_node(&graph);
    

    // Counts the total number of distances for each distance 
    //Calculates the frequency that the distance occurs throughout the graph
    bfs::max_distance_counts(&graph);


    //total number of vertices
    let num_vertices = bfs::get_num_vertices(&graph);
    println!("Number of vertices: {}", num_vertices);

    //Calculates the average distance throughout the graph
    bfs::calculate_average_distance(&graph);
    



    // Calculate and print the average number of nodes at each distance from every node
    bfs::average_distances_nodes_count(&graph);

    
        
}
mod tests {
    use super::*;
    //use std::collections::HashMap;

    #[test]
    fn test_calculate_average_distance() {
        // Create a sample graph for testing
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 4]);
        graph.insert(3, vec![1, 5]);
        graph.insert(4, vec![2, 6]);
        graph.insert(5, vec![3, 6]);
        graph.insert(6, vec![4, 5]);

        // Call the function to calculate average distance
        bfs::calculate_average_distance(&graph);

        
    }
        // Helper function to create a simple graph
        fn create_graph(edges: &[(usize, usize)]) -> Graph {
            let mut graph = Graph::new();
            for &(u, v) in edges {
                graph.entry(u).or_insert(Vec::new()).push(v);
                graph.entry(v).or_insert(Vec::new()).push(u); // Assuming undirected graph
            }
            graph
        }
    
        #[test]
        fn test_bfs_distance_single_node() {
            let mut graph = Graph::new();
            graph.insert(1, Vec::new());  // Explicitly add a single node with no edges
        
            let distances = bfs::bfs_distance(&graph, 1);
            assert_eq!(distances.len(), 1, "Should contain exactly one entry for the single node.");
            assert_eq!(distances.get(&1), Some(&0), "Distance to itself should be 0.");
        }
    
        #[test]
        fn test_bfs_distance_chain() {
            let graph = create_graph(&[(1, 2), (2, 3), (3, 4)]);
            let distances = bfs::bfs_distance(&graph, 1);
            assert_eq!(distances[&2], 1, "Distance to node 2 should be 1");
            assert_eq!(distances[&3], 2, "Distance to node 3 should be 2");
            assert_eq!(distances[&4], 3, "Distance to node 4 should be 3");
        }

}



    














