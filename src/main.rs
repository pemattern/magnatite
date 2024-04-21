pub mod database;
pub mod errors;
pub mod routes;

use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use database::Database;
use routes::{get_record, insert_record};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(Database::new()));
    let app = Router::new()
        .route(
            "/:table_key/:record_key",
            get(get_record).post(insert_record),
        )
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
