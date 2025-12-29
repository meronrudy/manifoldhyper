use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateGraphRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GraphResponse {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub graph_id: String,
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub graph_id: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub graph_id: String,
    pub format: String,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub graph_id: String,
    pub format: String,
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}
