use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlparser::{
    ast::{SetExpr, Statement},
    dialect::GenericDialect,
    parser::Parser,
};

use crate::{error::Error, AppState};

pub async fn parse_sql(State(state): State<AppState>, query: String) -> Response {
    let dialect = GenericDialect {};
    let asts = match Parser::parse_sql(&dialect, query.as_str()) {
        Ok(asts) => asts,
        Err(error) => return (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
    };

    println!("{:?}", asts);

    let mut exprs = Vec::new();
    for ast in asts {
        match ast {
            Statement::Query(query) => exprs.push(*query.body),
            _ => {
                return (StatusCode::BAD_REQUEST, Error::NotImplemented.to_string()).into_response()
            }
        }
    }

    let mutex = state.clone();
    let guard = mutex.lock().unwrap();

    for expr in exprs {
        match expr {
            SetExpr::Select(select) => {}
            SetExpr::Insert(insert) => {}
            SetExpr::Update(update) => {}
            SetExpr::Table(table) => {}
            _ => {
                return (StatusCode::BAD_REQUEST, Error::NotImplemented.to_string()).into_response()
            }
        }
    }

    StatusCode::OK.into_response()
}
