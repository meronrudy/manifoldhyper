use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypergraph {
    pub name: String,
    pub nodes: Vec<String>,
    pub hyperedges: Vec<Vec<String>>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub node_count: usize,
    pub edge_count: usize,
}

#[derive(Debug, Serialize)]
pub struct NodeQuery {
    pub node: String,
    pub present: bool,
    pub incident_edges: Vec<usize>,
}

impl Hypergraph {
    pub fn create(
        name: String,
        nodes: Vec<String>,
        hyperedges: Vec<Vec<String>>,
        metadata: Option<Value>,
    ) -> Self {
        Self {
            name,
            nodes,
            hyperedges,
            metadata,
        }
    }

    pub fn summary(&self) -> Summary {
        Summary {
            node_count: self.nodes.len(),
            edge_count: self.hyperedges.len(),
        }
    }

    pub fn query_node(&self, node: &str) -> NodeQuery {
        let mut incident_edges = Vec::new();
        for (idx, edge) in self.hyperedges.iter().enumerate() {
            if edge.iter().any(|entry| entry == node) {
                incident_edges.push(idx);
            }
        }

        NodeQuery {
            node: node.to_string(),
            present: self.nodes.iter().any(|entry| entry == node),
            incident_edges,
        }
    }
}

pub fn load_from_path(path: &Path) -> Result<Hypergraph> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let graph = serde_json::from_str(&raw)
        .with_context(|| format!("parsing hypergraph JSON from {}", path.display()))?;
    Ok(graph)
}

pub fn export_to_path(graph: &Hypergraph, path: &Path, pretty: bool) -> Result<()> {
    let payload = if pretty {
        serde_json::to_string_pretty(graph)?
    } else {
        serde_json::to_string(graph)?
    };
    fs::write(path, payload).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
