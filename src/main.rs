use axum::Router;

mod serices;
use serices::axum::server;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().nest("/api", Router::new());
    server(app).await;
}
