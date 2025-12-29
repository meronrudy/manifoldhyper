use std::net::SocketAddr;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::api::error::ApiError;
use crate::api::models::{
    CreateGraphRequest, ExportRequest, ExportResponse, GraphResponse, QueryRequest, QueryResponse,
};
use crate::library;

#[derive(Clone, Default)]
struct AppState {}

pub async fn serve(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::default();
    let app = Router::new()
        .route("/graphs", post(create_graph))
        .route("/graphs/:id", get(get_graph))
        .route("/query", post(run_query))
        .route("/export", post(export_graph))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn create_graph(
    State(_state): State<AppState>,
    Json(payload): Json<CreateGraphRequest>,
) -> Result<Json<GraphResponse>, ApiError> {
    let graph = library::create_graph(payload.name).map_err(|err| match err {
        library::LibraryError::NotFound(message) => ApiError::not_found(message),
        library::LibraryError::InvalidInput(message) => ApiError::bad_request(message),
    })?;
    Ok(Json(GraphResponse {
        id: graph.id,
        name: graph.name,
    }))
}

async fn get_graph(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GraphResponse>, ApiError> {
    let graph = library::get_graph(id).map_err(|err| match err {
        library::LibraryError::NotFound(message) => ApiError::not_found(message),
        library::LibraryError::InvalidInput(message) => ApiError::bad_request(message),
    })?;
    Ok(Json(GraphResponse {
        id: graph.id,
        name: graph.name,
    }))
}

async fn run_query(
    State(_state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, ApiError> {
    let result =
        library::run_query(payload.graph_id, payload.query).map_err(|err| match err {
            library::LibraryError::NotFound(message) => ApiError::not_found(message),
            library::LibraryError::InvalidInput(message) => ApiError::bad_request(message),
        })?;
    Ok(Json(QueryResponse {
        graph_id: result.graph_id,
        result: result.result,
    }))
}

async fn export_graph(
    State(_state): State<AppState>,
    Json(payload): Json<ExportRequest>,
) -> Result<Json<ExportResponse>, ApiError> {
    let result =
        library::export_graph(payload.graph_id, payload.format).map_err(|err| match err {
            library::LibraryError::NotFound(message) => ApiError::not_found(message),
            library::LibraryError::InvalidInput(message) => ApiError::bad_request(message),
        })?;
    Ok(Json(ExportResponse {
        graph_id: result.graph_id,
        format: result.format,
        payload: result.payload,
    }))
}
