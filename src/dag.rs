use super::{Graph, Edge};
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub confidence: Option<f64>,
}

/// Build a DAG from relationships
pub fn build_dag_from_relationships(relationships: &[Relationship]) -> Graph {
    let mut nodes: HashSet<String> = HashSet::new();
    let mut edges: Vec<Edge> = Vec::new();

    for rel in relationships {
        nodes.insert(rel.from.clone());
        nodes.insert(rel.to.clone());
        edges.push(Edge {
            from: rel.from.clone(),
            to: rel.to.clone(),
            weight: rel.confidence,
        });
    }

    Graph {
        nodes: nodes.into_iter().collect(),
        edges,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_dag_from_relationships() {
        let relationships = vec![
            Relationship {
                from: "A".to_string(),
                to: "B".to_string(),
                confidence: Some(0.8),
            },
            Relationship {
                from: "B".to_string(),
                to: "C".to_string(),
                confidence: Some(0.9),
            },
        ];

        let graph = build_dag_from_relationships(&relationships);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
    }
}
