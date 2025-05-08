use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Define a struct for API responses
#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

// Define a struct for POST request payloads
#[derive(Deserialize)]
struct CreateMovie {
    id: u16,
    name: String,
    year: u16,
    was_good: bool,
}

#[tokio::main]
async fn main() {
    // Build the Axum application
    let app = Router::new()
        .route("/", get(root))
        .route("/movies/:id", get(get_movie))
        .route("/movies", post(create_movie));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Root handler
async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Welcome to the Rust Web Service!".to_string(),
    })
}

// GET /movies/:id handler
async fn get_movie(Path(id): Path<u32>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Movie with ID: {}", id),
    })
}

// POST /movies handler
async fn create_movie(Json(payload): Json<CreateMovie>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!(
            "Created movie: (Id: {}), (Name: {}), (Year: {}), (WasGood: {})", payload.id, payload.name, payload.year, payload.was_good),
    })
}
