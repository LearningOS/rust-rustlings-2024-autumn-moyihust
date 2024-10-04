/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>, // Node -> Vec of (Neighbor, Weight)
}

// Trait definition for common graph functionality
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        let table = self.adjacency_table_mutable();
        if table.contains_key(node) {
            false // Node already exists
        } else {
            table.insert(node.to_string(), Vec::new());
            true // New node added
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;

        // Add the nodes if they don't already exist
        self.add_node(node1);
        self.add_node(node2);

        let adj_table = self.adjacency_table_mutable();

        // Add the edge node1 -> node2
        if let Some(neighbors) = adj_table.get_mut(node1) {
            if !neighbors.iter().any(|(n, _)| n == node2) {
                neighbors.push((node2.to_string(), weight));
            }
        }

        // Add the edge node2 -> node1 (undirected graph)
        if let Some(neighbors) = adj_table.get_mut(node2) {
            if !neighbors.iter().any(|(n, _)| n == node1) {
                neighbors.push((node1.to_string(), weight));
            }
        }
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        let table = self.adjacency_table();

        for (from_node, neighbors) in table {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }

        edges
    }
}

// Implement the Graph trait for the UndirectedGraph
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

// Unit tests to verify the implementation
#[cfg(test)]
mod test_undirected_graph {
    use super::{Graph, UndirectedGraph};

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }
}