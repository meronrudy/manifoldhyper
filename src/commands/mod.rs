use crate::hypergraph::{self, Hypergraph};
use anyhow::{Result, anyhow};
use serde_json::json;
use std::path::PathBuf;

pub struct CreateArgs {
    pub name: String,
    pub nodes: Vec<String>,
    pub edges: Vec<String>,
    pub metadata: Option<String>,
}

pub struct LoadArgs {
    pub input: PathBuf,
}

pub struct QueryArgs {
    pub input: PathBuf,
    pub node: Option<String>,
}

pub struct ExportArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub pretty: bool,
}

pub fn create(args: CreateArgs) -> Result<serde_json::Value> {
    let edges = parse_edges(&args.edges)?;
    let metadata = parse_metadata(args.metadata.as_deref())?;

    let graph = Hypergraph::create(args.name, args.nodes, edges, metadata);

    Ok(json!({
        "status": "ok",
        "hypergraph": graph,
    }))
}

pub fn load(args: LoadArgs) -> Result<serde_json::Value> {
    let graph = hypergraph::load_from_path(&args.input)?;

    Ok(json!({
        "status": "ok",
        "hypergraph": graph,
    }))
}

pub fn query(args: QueryArgs) -> Result<serde_json::Value> {
    let graph = hypergraph::load_from_path(&args.input)?;

    let result = match args.node.as_deref() {
        Some(node) => json!({
            "type": "node",
            "result": graph.query_node(node),
        }),
        None => json!({
            "type": "summary",
            "result": graph.summary(),
        }),
    };

    Ok(json!({
        "status": "ok",
        "query": result,
    }))
}

pub fn export(args: ExportArgs) -> Result<serde_json::Value> {
    let graph = hypergraph::load_from_path(&args.input)?;
    hypergraph::export_to_path(&graph, &args.output, args.pretty)?;

    Ok(json!({
        "status": "ok",
        "output": args.output,
    }))
}

fn parse_edges(edges: &[String]) -> Result<Vec<Vec<String>>> {
    edges.iter().map(|edge| parse_edge(edge)).collect()
}

fn parse_edge(raw: &str) -> Result<Vec<String>> {
    let nodes: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect();

    if nodes.is_empty() {
        return Err(anyhow!("edge definition '{raw}' contains no nodes"));
    }

    Ok(nodes)
}

fn parse_metadata(raw: Option<&str>) -> Result<Option<serde_json::Value>> {
    match raw {
        Some(value) => {
            let parsed = serde_json::from_str(value)
                .map_err(|err| anyhow!("invalid metadata JSON: {err}"))?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}
