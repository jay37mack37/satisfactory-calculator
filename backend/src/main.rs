mod recipe_engine;

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::Method,
};
use recipe_engine::{RecipeDatabase, CalculationRequest, CalculationResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};

pub struct AppState {
    pub db: RecipeDatabase,
}

#[tokio::main]
async fn main() {
    let db = RecipeDatabase::new();
    let state = Arc::new(Mutex::new(AppState { db }));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/items", get(get_items))
        .route("/api/calculate", post(calculate))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

async fn calculate(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CalculationRequest>,
) -> Json<CalculationResult> {
    let state = state.lock().await;
    let result = state.db.calculate_requirements(&request.item, request.rate);
    Json(result)
}
