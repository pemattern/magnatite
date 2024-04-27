pub mod database;
pub mod error;
pub mod routes;

use std::sync::{Arc, Mutex};

use axum::{routing::post, Router};
use database::database::Database;
use routes::parse_sql;
use tokio::net::TcpListener;

pub type AppState = Arc<Mutex<Database>>;

#[tokio::main]
async fn main() {
    let app = app();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let state: AppState = Arc::new(Mutex::new(Database::new()));
    Router::new()
        .route("/sql_query", post(parse_sql))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(4, 4);
    }
}
