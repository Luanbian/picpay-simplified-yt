use axum::{Json, Router, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

pub async fn server(app: Router) {
    let app = health_check(app);
    let port = 3005;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Server running on port: {}", port);
    axum::serve(listener, app).await.unwrap();
}

fn health_check(app: Router) -> Router {
    app.route("/api", get(|| async { Json(json!({"message": "Echo"})) }))
}
