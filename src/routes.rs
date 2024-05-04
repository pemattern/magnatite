use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{interpreter::parse_sql_query, AppState};

pub async fn sql(State(state): State<AppState>, query: String) -> Response {
    let db_requests = match parse_sql_query(query) {
        Ok(db_requests) => db_requests,
        Err(error) => return (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
    };

    let mutex = state.clone();
    let mut guard = mutex.lock().unwrap();

    match guard.process_requests(db_requests) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(error) => (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
    }
}
