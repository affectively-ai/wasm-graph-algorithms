use super::{Graph, CycleDetectionResult};
use std::collections::{HashMap, HashSet};

/// Detect cycles in a directed graph using DFS
pub fn detect_cycles_in_graph(graph: &Graph) -> CycleDetectionResult {
    // Build adjacency list
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    for node in &graph.nodes {
        adjacency.insert(node.clone(), Vec::new());
    }

    for edge in &graph.edges {
        adjacency
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.clone());
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut rec_stack: HashSet<String> = HashSet::new();
    let mut cycles: Vec<Vec<String>> = Vec::new();

    for node in &graph.nodes {
        if !visited.contains(node) {
            let mut path = Vec::new();
            if dfs_cycle_detection(
                node,
                &adjacency,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            ) {
                // Cycle found, path contains the cycle
            }
        }
    }

    CycleDetectionResult {
        has_cycle: !cycles.is_empty(),
        cycles,
    }
}

/// DFS helper for cycle detection
fn dfs_cycle_detection(
    node: &String,
    adjacency: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) -> bool {
    visited.insert(node.clone());
    rec_stack.insert(node.clone());
    path.push(node.clone());

    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if dfs_cycle_detection(neighbor, adjacency, visited, rec_stack, path, cycles) {
                    return true;
                }
            } else if rec_stack.contains(neighbor) {
                // Found a back edge - cycle detected
                // Extract cycle from path
                if let Some(cycle_start) = path.iter().position(|x| x == neighbor) {
                    let cycle: Vec<String> = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                    return true;
                }
            }
        }
    }

    rec_stack.remove(node);
    path.pop();
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cycles_no_cycle() {
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

        let result = detect_cycles_in_graph(&graph);
        assert!(!result.has_cycle);
    }

    #[test]
    fn test_detect_cycles_with_cycle() {
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

        let result = detect_cycles_in_graph(&graph);
        assert!(result.has_cycle);
    }
}
