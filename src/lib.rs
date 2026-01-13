use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod dag;
mod topological_sort;
mod cycle_detection;
mod path_finding;

use dag::{build_dag_from_relationships, Relationship};
use topological_sort::*;
use cycle_detection::*;
use path_finding::*;

/// Graph edge structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: Option<f64>,
}

/// Graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<Edge>,
}

/// Topological sort result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologicalSortResult {
    pub sorted: Vec<String>,
    pub has_cycle: bool,
}

/// Cycle detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleDetectionResult {
    pub has_cycle: bool,
    pub cycles: Vec<Vec<String>>,
}

/// Path finding result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathResult {
    pub path: Vec<String>,
    pub exists: bool,
    pub distance: Option<f64>,
}

/// Build a DAG from edges and perform topological sort
/// 
/// # Arguments
/// * `graph_json` - JSON string of Graph structure
/// 
/// # Returns
/// JSON string of TopologicalSortResult
#[wasm_bindgen]
pub fn topological_sort(graph_json: &str) -> String {
    let graph: Graph = match serde_json::from_str(graph_json) {
        Ok(g) => g,
        Err(_) => {
            return serde_json::to_string(&TopologicalSortResult {
                sorted: vec![],
                has_cycle: true,
            })
            .unwrap_or_else(|_| "{\"sorted\":[],\"hasCycle\":true}".to_string());
        }
    };

    let result = compute_topological_sort(&graph);
    
    serde_json::to_string(&result).unwrap_or_else(|_| "{\"sorted\":[],\"hasCycle\":true}".to_string())
}

/// Detect cycles in a directed graph
/// 
/// # Arguments
/// * `graph_json` - JSON string of Graph structure
/// 
/// # Returns
/// JSON string of CycleDetectionResult
#[wasm_bindgen]
pub fn detect_cycles(graph_json: &str) -> String {
    let graph: Graph = match serde_json::from_str(graph_json) {
        Ok(g) => g,
        Err(_) => {
            return serde_json::to_string(&CycleDetectionResult {
                has_cycle: false,
                cycles: vec![],
            })
            .unwrap_or_else(|_| "{\"hasCycle\":false,\"cycles\":[]}".to_string());
        }
    };

    let result = detect_cycles_in_graph(&graph);
    
    serde_json::to_string(&result).unwrap_or_else(|_| "{\"hasCycle\":false,\"cycles\":[]}".to_string())
}

/// Find path between two nodes in a directed graph
/// 
/// # Arguments
/// * `graph_json` - JSON string of Graph structure
/// * `from` - Starting node
/// * `to` - Target node
/// 
/// # Returns
/// JSON string of PathResult
#[wasm_bindgen]
pub fn find_path(graph_json: &str, from: &str, to: &str) -> String {
    let graph: Graph = match serde_json::from_str(graph_json) {
        Ok(g) => g,
        Err(_) => {
            return serde_json::to_string(&PathResult {
                path: vec![],
                exists: false,
                distance: None,
            })
            .unwrap_or_else(|_| "{\"path\":[],\"exists\":false,\"distance\":null}".to_string());
        }
    };

    let result = find_path_between_nodes(&graph, from, to);
    
    serde_json::to_string(&result).unwrap_or_else(|_| "{\"path\":[],\"exists\":false,\"distance\":null}".to_string())
}

/// Build a DAG from relationships
/// 
/// # Arguments
/// * `relationships_json` - JSON string of relationship array (each with from, to, confidence)
/// 
/// # Returns
/// JSON string of Graph structure
#[wasm_bindgen]
pub fn build_dag(relationships_json: &str) -> String {
    let relationships: Vec<Relationship> = match serde_json::from_str(relationships_json) {
        Ok(r) => r,
        Err(_) => {
            return serde_json::to_string(&Graph {
                nodes: vec![],
                edges: vec![],
            })
            .unwrap_or_else(|_| "{\"nodes\":[],\"edges\":[]}".to_string());
        }
    };

    let result = build_dag_from_relationships(&relationships);
    
    serde_json::to_string(&result).unwrap_or_else(|_| "{\"nodes\":[],\"edges\":[]}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            edges: vec![
                Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: None,
                },
                Edge {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    weight: None,
                },
            ],
        };

        let json = serde_json::to_string(&graph).unwrap();
        let result = topological_sort(&json);
        let parsed: TopologicalSortResult = serde_json::from_str(&result).unwrap();
        
        assert!(!parsed.has_cycle);
        assert_eq!(parsed.sorted.len(), 3);
    }

    #[test]
    fn test_detect_cycles() {
        let graph = Graph {
            nodes: vec!["A".to_string(), "B".to_string()],
            edges: vec![
                Edge {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: None,
                },
                Edge {
                    from: "B".to_string(),
                    to: "A".to_string(),
                    weight: None,
                },
            ],
        };

        let json = serde_json::to_string(&graph).unwrap();
        let result = detect_cycles(&json);
        let parsed: CycleDetectionResult = serde_json::from_str(&result).unwrap();
        
        assert!(parsed.has_cycle);
    }
}
