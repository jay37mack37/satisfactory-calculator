mod recipe_engine;

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::Method,
};
use recipe_engine::{RecipeDatabase, CalculationRequest, CalculationResult, RecipeInfo};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};

pub struct AppState {
    pub db: RecipeDatabase,
    pub icon_map: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
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
        .route("/api/alternates", get(get_alternates))
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

async fn get_icons(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<HashMap<String, String>> {
    let state = state.lock().await;
    Json(state.icon_map.clone())
}

async fn get_alternates(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<HashMap<String, Vec<RecipeInfo>>> {
    let state = state.lock().await;
    Json(state.db.all_alternates())
}

async fn calculate(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CalculationRequest>,
) -> Json<CalculationResult> {
    let state = state.lock().await;
    let result = state.db.calculate_requirements(&request.item, request.rate, &request.recipe_overrides);
    Json(result)
}
