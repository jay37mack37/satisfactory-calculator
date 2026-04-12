mod recipe_engine;

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::Method,
};
use recipe_engine::{RecipeDatabase, CalculationRequest, CalculationResult};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};

pub struct AppState {
    pub db: RecipeDatabase,
    pub icon_map: HashMap<String, String>,
}

/// Parse CLI arguments for host and port.
/// Usage: backend [host] [port]
///   host: defaults to "0.0.0.0" (or BACKEND_HOST env var)
///   port: defaults to 3000 (or BACKEND_PORT env var)
fn parse_args() -> (String, u16) {
    let args: Vec<String> = std::env::args().collect();
    let host = if args.len() > 1 {
        args[1].clone()
    } else {
        std::env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
    };
    let port = if args.len() > 2 {
        args[2].parse().expect("Invalid port number")
    } else {
        std::env::var("BACKEND_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000)
    };
    (host, port)
}

#[tokio::main]
async fn main() {
    let (host, port) = parse_args();
    let db = RecipeDatabase::new();
    let icon_map: HashMap<String, String> = serde_json::from_str(include_str!("icon_map.json"))
        .expect("Failed to parse icon_map.json");
    let state = Arc::new(Mutex::new(AppState { db, icon_map }));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/items", get(get_items))
        .route("/api/icons", get(get_icons))
        .route("/api/calculate", post(calculate))
        .with_state(state)
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid host:port address");
    println!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_items(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<Vec<String>> {
    let state = state.lock().await;
    Json(state.db.items())
}

async fn get_icons(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<HashMap<String, String>> {
    let state = state.lock().await;
    Json(state.icon_map.clone())
}

async fn calculate(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CalculationRequest>,
) -> Json<CalculationResult> {
    let state = state.lock().await;
    let result = state.db.calculate_requirements(&request.item, request.rate);
    Json(result)
}
