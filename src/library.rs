use serde_json::json;

#[derive(Debug)]
pub struct GraphInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct QueryResult {
    pub graph_id: String,
    pub result: serde_json::Value,
}

#[derive(Debug)]
pub struct ExportResult {
    pub graph_id: String,
    pub format: String,
    pub payload: String,
}

#[derive(Debug)]
pub enum LibraryError {
    NotFound(String),
    InvalidInput(String),
}

pub fn create_graph(name: String) -> Result<GraphInfo, LibraryError> {
    if name.trim().is_empty() {
        return Err(LibraryError::InvalidInput(
            "graph name cannot be empty".to_string(),
        ));
    }
    Ok(GraphInfo {
        id: "graph-1".to_string(),
        name,
    })
}

pub fn get_graph(id: String) -> Result<GraphInfo, LibraryError> {
    if id.trim().is_empty() {
        return Err(LibraryError::InvalidInput(
            "graph id cannot be empty".to_string(),
        ));
    }
    if id != "graph-1" {
        return Err(LibraryError::NotFound(format!(
            "graph {} not found",
            id
        )));
    }
    Ok(GraphInfo {
        id,
        name: "Example Graph".to_string(),
    })
}

pub fn run_query(graph_id: String, query: String) -> Result<QueryResult, LibraryError> {
    if graph_id.trim().is_empty() {
        return Err(LibraryError::InvalidInput(
            "graph id cannot be empty".to_string(),
        ));
    }
    Ok(QueryResult {
        graph_id,
        result: json!({
            "message": format!("query executed: {}", query)
        }),
    })
}

pub fn export_graph(graph_id: String, format: String) -> Result<ExportResult, LibraryError> {
    if graph_id.trim().is_empty() {
        return Err(LibraryError::InvalidInput(
            "graph id cannot be empty".to_string(),
        ));
    }
    Ok(ExportResult {
        graph_id,
        format,
        payload: "exported-data".to_string(),
    })
}
