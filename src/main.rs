mod routing;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use routing::{solve, Graph, Request};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let graph: Graph =
        serde_json::from_str(include_str!("../data/network.json")).expect("valid bundled network");
    graph.validate().expect("valid graph topology");
    let app = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../web/index.html")) }),
        )
        .route(
            "/app.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../web/app.js"),
                )
            }),
        )
        .route(
            "/style.css",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/css")],
                    include_str!("../web/style.css"),
                )
            }),
        )
        .route("/api/network", get(network))
        .route("/api/route", post(route))
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({"status":"ok","engine":"rust"})) }),
        )
        .with_state(Arc::new(graph));
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind routing service");
    println!("Sustain Route · Rust engine · http://{addr}");
    axum::serve(listener, app).await.expect("serve routing API");
}
async fn network(State(graph): State<Arc<Graph>>) -> impl IntoResponse {
    Json((*graph).clone())
}
async fn route(State(graph): State<Arc<Graph>>, Json(request): Json<Request>) -> impl IntoResponse {
    match solve(&graph, &request) {
        Ok(result) => (StatusCode::OK, Json(serde_json::to_value(result).unwrap())),
        Err(error) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({"error":error})),
        ),
    }
}
