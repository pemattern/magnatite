use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};

use crate::database::{ColumnValue, Database, Key, Record};

pub async fn get_record(
    Path((table_key, record_key)): Path<(String, String)>,
    state: State<Arc<Mutex<Database>>>,
) -> Response {
    let table_key = Key::String(table_key);
    let record_key = Key::String(record_key);

    let mutex = state.clone();
    let guard = mutex.lock().unwrap();
    match guard.get(&table_key, &record_key) {
        Ok(record) => (StatusCode::OK, format!("{:?}", record.get_all_values())).into_response(),
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

pub async fn insert_record(
    Path((table_key, record_key)): Path<(String, String)>,
    state: State<Arc<Mutex<Database>>>,
    Form(record_data): Form<HashMap<String, String>>,
) -> Response {
    let table_key = Key::String(table_key);
    let record_key = Key::String(record_key);

    let mutex = state.clone();
    let mut guard = mutex.lock().unwrap();

    let mut record = Record::new();
    record.set_data(record_data);

    guard.set(table_key, record_key, record);
    (StatusCode::CREATED).into_response()
}
