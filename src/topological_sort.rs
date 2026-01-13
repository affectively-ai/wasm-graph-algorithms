use super::{Graph, TopologicalSortResult};
use std::collections::{HashMap, VecDeque};

/// Compute topological sort using Kahn's algorithm
pub fn compute_topological_sort(graph: &Graph) -> TopologicalSortResult {
    // Build adjacency list and in-degree map
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    // Initialize all nodes
    for node in &graph.nodes {
        adjacency.insert(node.clone(), Vec::new());
        in_degree.insert(node.clone(), 0);
    }

    // Build graph and calculate in-degrees
    for edge in &graph.edges {
        adjacency
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.clone());
        *in_degree.entry(edge.to.clone()).or_insert(0) += 1;
    }

    // Kahn's algorithm
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut sorted: Vec<String> = Vec::new();

    // Find all nodes with in-degree 0
    for (node, degree) in &in_degree {
        if *degree == 0 {
            queue.push_back(node.clone());
        }
    }

    while let Some(node) = queue.pop_front() {
        sorted.push(node.clone());

        if let Some(neighbors) = adjacency.get(&node) {
            for neighbor in neighbors {
                let degree = in_degree.entry(neighbor.clone()).or_insert(0);
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    // Check for cycle: if sorted length < total nodes, there's a cycle
    let has_cycle = sorted.len() < graph.nodes.len();

    TopologicalSortResult {
        sorted,
        has_cycle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_topological_sort_linear() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            edges: vec![
                super::super::Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: None,
                },
                super::super::Edge {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    weight: None,
                },
            ],
        };

        let result = compute_topological_sort(&graph);
        assert!(!result.has_cycle);
        assert_eq!(result.sorted.len(), 3);
        // A should come before B, B before C
        let a_pos = result.sorted.iter().position(|x| x == "A").unwrap();
        let b_pos = result.sorted.iter().position(|x| x == "B").unwrap();
        let c_pos = result.sorted.iter().position(|x| x == "C").unwrap();
        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_compute_topological_sort_with_cycle() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string()],
            edges: vec![
                super::super::Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: None,
                },
                super::super::Edge {
                    from: "B".to_string(),
                    to: "A".to_string(),
                    weight: None,
                },
            ],
        };

        let result = compute_topological_sort(&graph);
        assert!(result.has_cycle);
    }
}
