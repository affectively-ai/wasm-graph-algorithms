use super::{Graph, PathResult};
use std::collections::{HashMap, HashSet, VecDeque};

/// Find path between two nodes using BFS
pub fn find_path_between_nodes(graph: &Graph, from: &str, to: &str) -> PathResult {
    if from == to {
        return PathResult {
            path: vec![from.to_string()],
            exists: true,
            distance: Some(0.0),
        };
    }

    // Build adjacency list
    let mut adjacency: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for node in &graph.nodes {
        adjacency.insert(node.clone(), Vec::new());
    }

    for edge in &graph.edges {
        let weight = edge.weight.unwrap_or(1.0);
        adjacency
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push((edge.to.clone(), weight));
    }

    // BFS to find shortest path
    let mut queue: VecDeque<(String, Vec<String>, f64)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    queue.push_back((from.to_string(), vec![from.to_string()], 0.0));
    visited.insert(from.to_string());

    while let Some((current, path, distance)) = queue.pop_front() {
        if current == to {
            return PathResult {
                path,
                exists: true,
                distance: Some(distance),
            };
        }

        if let Some(neighbors) = adjacency.get(&current) {
            for (neighbor, weight) in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back((neighbor.clone(), new_path, distance + weight));
                }
            }
        }
    }

    PathResult {
        path: vec![],
        exists: false,
        distance: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path_exists() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            edges: vec![
                super::super::Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: Some(1.0),
                },
                super::super::Edge {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    weight: Some(1.0),
                },
            ],
        };

        let result = find_path_between_nodes(&graph, "A", "C");
        assert!(result.exists);
        assert_eq!(result.path.len(), 3);
        assert_eq!(result.path[0], "A");
        assert_eq!(result.path[2], "C");
    }

    #[test]
    fn test_find_path_not_exists() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            edges: vec![
                super::super::Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: None,
                },
            ],
        };

        let result = find_path_between_nodes(&graph, "A", "C");
        assert!(!result.exists);
    }

    #[test]
    fn test_find_path_same_node() {
        let graph = Graph {
            nodes: vec!["A".to_string()],
            edges: vec![],
        };

        let result = find_path_between_nodes(&graph, "A", "A");
        assert!(result.exists);
        assert_eq!(result.path, vec!["A"]);
    }
}
