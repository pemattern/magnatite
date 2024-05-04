use std::collections::HashMap;

use sqlparser::{
    ast::{ColumnDef, DataType, Expr, Ident, ObjectName, Query, Statement, UnaryOperator, Value},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

use crate::{
    database::{
        cell::CellValue,
        column::{Column, ColumnConstraint},
        request::DatabaseRequest,
    },
    error::{Error, Result},
};

pub fn parse_sql_query(query: String) -> Result<Vec<DatabaseRequest>> {
    let dialect = PostgreSqlDialect {};

    let statements = match Parser::parse_sql(&dialect, query.as_str()) {
        Ok(statements) => statements,
        Err(error) => return Err(Error::BadSql { error }),
    };
    let mut db_requests = Vec::new();
    for statement in statements {
        let db_request = match interpret_sql_statement(statement) {
            Ok(create_table_request) => create_table_request,
            Err(error) => return Err(error),
        };
        db_requests.push(db_request);
    }
    Ok(db_requests)
}

fn interpret_sql_statement(statement: Statement) -> Result<DatabaseRequest> {
    match statement {
        Statement::CreateTable { name, columns, .. } => {
            return interpret_create_table_statement(name, columns)
        }
        Statement::Insert {
            table_name,
            columns,
            source,
            ..
        } => return interpret_insert_statement(table_name, columns, source),
        _ => Err(Error::NotImplemented),
    }
}

fn interpret_create_table_statement(
    name: ObjectName,
    column_defs: Vec<ColumnDef>,
) -> Result<DatabaseRequest> {
    let table_key = match name.0.first() {
        Some(ident) => ident.value.clone(),
        None => return Err(Error::NotImplemented),
    };

    if column_defs.len() == 0 {
        let create_table_request = DatabaseRequest::CreateTable {
            key: table_key,
            columns: Option::None,
        };
        return Ok(create_table_request);
    }
    let mut columns = Vec::new();
    for column_def in column_defs {
        let key = column_def.name.value;
        let r#type = match column_def.data_type {
            DataType::Text => CellValue::String("".to_string()),
            DataType::Int(_) => CellValue::Integer(0),
            DataType::Float(_) => CellValue::Float(0.),
            _ => return Err(Error::NotImplemented),
        };
        let constraints = ColumnConstraint::empty();

        let column = Column::new(key, r#type, constraints);
        columns.push(column);
    }
    let create_table_request = DatabaseRequest::CreateTable {
        key: table_key,
        columns: Some(columns),
    };
    Ok(create_table_request)
}

fn interpret_insert_statement(
    table_name: ObjectName,
    columns: Vec<Ident>,
    source: Option<Box<Query>>,
) -> Result<DatabaseRequest> {
    let Some(query) = source else {
        return Err(Error::NotImplemented);
    };

    let values = match *query.body {
        sqlparser::ast::SetExpr::Values(values) => values,
        _ => return Err(Error::NotImplemented),
    };

    let Some(exprs) = values.rows.first().cloned() else {
        return Err(Error::NotImplemented);
    };

    if exprs.len() != columns.len() {
        return Err(Error::NotImplemented);
    }

    fn parse_number_string_to_cell_value(string: String) -> CellValue {
        if string.contains(".") {
            return CellValue::Float(string.parse::<f64>().unwrap());
        }
        CellValue::Integer(string.parse::<i64>().unwrap())
    }

    let mut cell_values = Vec::new();
    for expr in exprs {
        match expr {
            Expr::Value(value) => {
                let cell_value = match value {
                    Value::Number(number_string, _) => {
                        parse_number_string_to_cell_value(number_string)
                    }
                    Value::SingleQuotedString(string) => CellValue::String(string),
                    _ => continue,
                };
                cell_values.push(cell_value);
            }
            Expr::UnaryOp { op, expr } => {
                if let UnaryOperator::Minus = op {
                    match *expr {
                        Expr::Value(value) => {
                            if let Value::Number(number_string, _) = value {
                                let cell_value = parse_number_string_to_cell_value(
                                    "-".to_string() + &number_string,
                                );
                                cell_values.push(cell_value);
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    let column_values = columns
        .into_iter()
        .map(|i| i.value)
        .zip(cell_values.into_iter())
        .collect::<HashMap<String, CellValue>>();

    let table_key = match table_name.0.first() {
        Some(ident) => ident.value.clone(),
        None => return Err(Error::NotImplemented),
    };

    Ok(DatabaseRequest::InsertInto {
        key: table_key,
        data: column_values,
    })
}
