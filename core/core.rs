use std::io::{Error, ErrorKind};

use regex::Regex;
use sqlparser::ast::{Ident, ObjectName, SetExpr, Values};
use sqlparser::{ast::Statement, dialect::MySqlDialect, parser::Parser};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub fn format_insert_queries(sql: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dialect = MySqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql)?;

    if !is_insert_only(&ast) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "this sql contains non-insert queries",
        )));
    }

    let comment_and_query_map = generate_comment_and_query_map(sql);

    let mut formatted_queries = ast
        .iter()
        .map(|query| {
            if let Statement::Insert {
                or: _,
                into: _,
                table_name,
                columns,
                overwrite: _,
                source,
                partitioned: _,
                after_columns: _,
                table: _,
                on: _,
                returning: _,
            } = query
            {
                if let SetExpr::Values(values) = &*source.body {
                    let max_char_length_vec = get_max_char_length_vec(columns, values);
                    let formatted_query =
                        generate_formatted_query(table_name, columns, values, &max_char_length_vec);
                    return formatted_query;
                }
            }
            return String::from("");
        })
        .collect::<Vec<String>>();

    let result = comment_and_query_map
        .into_iter()
        .map(|comment_or_query| {
            if comment_or_query.starts_with("INSERT INTO") {
                return formatted_queries.remove(0);
            } else {
                return comment_or_query;
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    return Ok(result);
}

#[wasm_bindgen]
pub fn format_insert_queries_wasm(sql: &str) -> Result<String, JsValue> {
    return match format_insert_queries(sql) {
        Ok(formatted_queries) => Ok(formatted_queries),
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    };
}

fn is_insert_only(ast: &Vec<Statement>) -> bool {
    return ast.iter().fold(true, |a, query| match query {
        Statement::Insert {
            or: _,
            into: _,
            table_name: _,
            columns: _,
            overwrite: _,
            source: _,
            partitioned: _,
            after_columns: _,
            table: _,
            on: _,
            returning: _,
        } => return a && true,
        _ => return a && false,
    });
}

// this func returns a vec of comments and query prefixes.
// the prefixes are used to identify where each comment is written.
fn generate_comment_and_query_map(sql_with_comment: &str) -> Vec<String> {
    return Regex::new(r"(--.*)|(INSERT INTO)")
        .unwrap()
        .captures_iter(sql_with_comment)
        .map(|capture| return String::from(&capture[0]))
        .collect::<Vec<String>>();
}

fn get_max_char_length_vec(columns: &Vec<Ident>, values: &Values) -> Vec<usize> {
    return get_char_length_matrix(columns, values)
        .iter()
        .fold(
            vec![vec![0 as usize; 0]; columns.len()],
            |mut transposed_matrix, char_length_of_row| {
                char_length_of_row
                    .iter()
                    .enumerate()
                    .for_each(|(column_index, char_length)| {
                        transposed_matrix[column_index].push(*char_length)
                    });
                return transposed_matrix;
            },
        )
        .iter()
        .map(|char_length_of_column| {
            return char_length_of_column
                .into_iter()
                .max()
                .unwrap_or(&(0 as usize))
                .clone();
        })
        .collect::<Vec<usize>>();
}

fn get_char_length_matrix(columns: &Vec<Ident>, values: &Values) -> Vec<Vec<usize>> {
    return columns
        .iter()
        .map(|column| {
            return column.to_string().len();
        })
        .collect::<Vec<usize>>()
        .chunks(columns.len())
        .map(|chunk| chunk.to_vec())
        .chain(values.rows.iter().map(|row| {
            row.iter()
                .map(|value| value.to_string().len())
                .collect::<Vec<usize>>()
        }))
        .collect::<Vec<Vec<usize>>>();
}

// TODO: make it functional
// construct formatted query from scratch by using ast data
fn generate_formatted_query(
    table_name: &ObjectName,
    columns: &Vec<Ident>,
    values: &Values,
    max_char_length_vec: &Vec<usize>,
) -> String {
    let table_name_part: String = String::from("INSERT INTO ") + &table_name.to_string() + "\n";

    let mut column_name_part: String = String::from("(");
    for (index, column) in columns.iter().enumerate() {
        let adjustment =
            String::from(" ").repeat(max_char_length_vec[index] - column.to_string().len());
        column_name_part = column_name_part + &column.to_string() + &adjustment;
        if index != columns.len() - 1 {
            column_name_part += ","
        }
    }
    column_name_part += ")\n";

    let values_part: &str = "VALUES\n";

    let mut rows_part: String = String::from("");
    for (row_index, row) in values.rows.iter().enumerate() {
        rows_part += "(";
        for (column_index, value) in row.iter().enumerate() {
            let adjustment = String::from(" ")
                .repeat(max_char_length_vec[column_index] - value.to_string().len());
            rows_part = rows_part + &value.to_string() + &adjustment;
            if column_index != row.len() - 1 {
                rows_part += ","
            }
        }
        rows_part += ")";
        if row_index != values.rows.len() - 1 {
            rows_part += ","
        } else {
            rows_part += ";"
        }
        rows_part += "\n";
    }

    return String::from("") + &table_name_part + &column_name_part + &values_part + &rows_part;
}
