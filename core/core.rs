use std::io::{Error, ErrorKind};

use regex::Regex;
use sqlparser::ast::{Ident, ObjectName, SetExpr, Statement, Values};
use sqlparser::{dialect::MySqlDialect, parser::Parser, parser::ParserOptions};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub fn format_insert_queries(sql: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dialect = MySqlDialect {};
    let options = ParserOptions::default().with_unescape(false);
    let ast = Parser::new(&dialect)
        .with_options(options)
        .try_with_sql(sql)?
        .parse_statements()?;

    if !is_insert_only(&ast) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "this sql contains queries other than insert.",
        )));
    }

    let comment_and_query_map = generate_comment_and_query_map(sql);

    let mut formatted_queries: Vec<String> = ast
        .iter()
        .filter_map(|query| {
            if let Statement::Insert {
                table_name,
                columns,
                source,
                ..
            } = query
            {
                if let SetExpr::Values(values) = *source.clone().unwrap().body {
                    let max_char_length_vec = get_max_char_length_vec(columns, &values);
                    let formatted_query = generate_formatted_query(
                        table_name,
                        columns,
                        &values,
                        &max_char_length_vec,
                    );
                    return Some(formatted_query);
                }
            }
            return None;
        })
        .collect();

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

fn is_insert_only(ast: &[Statement]) -> bool {
    ast.iter()
        .all(|query| matches!(query, Statement::Insert { .. }))
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
                .map(|value| {
                    return value.to_string().len();
                })
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

// // The escape behavior of the sqlparser of v0.33.0 depends on if the value is quoted by backtick or else.
// // If the value is quoted by backtick, the sqlparser escapes the backslash.
// // But if the value is quoted by single quote or double quote, the sqlparser does NOT escape the backslash.
// // This module escape the value WHEN it is quoted by single quote or double quote.
// fn escape_stringified_value(str: &str) -> String {
//     let ch1 = str.chars().nth(0).unwrap();
//     match ch1 {
//         '\'' => {
//             return str
//                 .replace("\n", "\\n")
//                 .replace("\"", "\\\"")
//                 .replace("\t", "\\t")
//                 .replace("\r", "\\r")
//         }
//         '"' => {
//             return str
//                 .replace("\n", "\\n")
//                 .replace("'", "\\'")
//                 .replace("\t", "\\t")
//                 .replace("\r", "\\r")
//         }
//         _ => return str.to_string(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_backslashes_in_query_enclosed_with_backticks() {
        let query_with_backslash_with_backtick =
            r#"INSERT INTO `table` (`id`, `content`) VALUES (1, `\"e\nxample\"`);"#;
        let formatted = r#"INSERT INTO `table`
(`id`,`content`      )
VALUES
(1   ,`\"e\nxample\"`);
"#;
        assert_eq!(
            format_insert_queries(query_with_backslash_with_backtick).unwrap(),
            formatted
        );
    }

    #[test]
    fn keep_backslashes_in_query_enclosed_with_single_quotes() {
        let query_with_backslash_with_single_quote =
            r#"INSERT INTO `table` ('id', 'content') VALUES (1, '\"e\nxample\"');"#;
        let formatted = r#"INSERT INTO `table`
('id','content'      )
VALUES
(1   ,'\"e\nxample\"');
"#;
        assert_eq!(
            format_insert_queries(query_with_backslash_with_single_quote).unwrap(),
            formatted
        );
    }

    #[test]
    fn keep_backslashes_in_query_enclosed_with_double_quotes() {
        let query_with_backslash_with_double_quote =
            r#"INSERT INTO `table` ("id", "content") VALUES (1, "\'e\nxample\'");"#;
        let formatted = r#"INSERT INTO `table`
("id","content"      )
VALUES
(1   ,"\'e\nxample\'");
"#;
        assert_eq!(
            format_insert_queries(query_with_backslash_with_double_quote).unwrap(),
            formatted
        );
    }

    #[test]
    fn not_add_backslashes_to_query_enclosed_with_backticks() {
        let query_without_backslash_with_backticks =
            r#"INSERT INTO `table` (`id`, `content`) VALUES (1, `'"example"'`);"#;
        let formatted = r#"INSERT INTO `table`
(`id`,`content`    )
VALUES
(1   ,`'"example"'`);
"#;
        assert_eq!(
            format_insert_queries(query_without_backslash_with_backticks).unwrap(),
            formatted
        );
    }

    #[test]
    fn not_add_backslashes_to_query_enclosed_with_single_quotes() {
        let query_without_backslash_with_single_quotes =
            r#"INSERT INTO `table` ('id', 'content') VALUES (1, '"example"');"#;
        let formatted = r#"INSERT INTO `table`
('id','content'  )
VALUES
(1   ,'"example"');
"#;
        assert_eq!(
            format_insert_queries(query_without_backslash_with_single_quotes).unwrap(),
            formatted
        );
    }

    #[test]
    fn not_add_backslashes_to_query_enclosed_with_double_quotes() {
        let query_without_backslash_with_double_quotes =
            r#"INSERT INTO `table` ("id", "content") VALUES (1, "'example'");"#;
        let formatted = r#"INSERT INTO `table`
("id","content"  )
VALUES
(1   ,"'example'");
"#;
        assert_eq!(
            format_insert_queries(query_without_backslash_with_double_quotes).unwrap(),
            formatted
        );
    }

    #[test]
    fn work_with_function() {
        let query_with_function = "INSERT INTO `table` (`id`, `created_at`) VALUES (1, now());";
        let formatted = "INSERT INTO `table`\n(`id`,`created_at`)\nVALUES\n(1   ,now()       );\n";
        assert_eq!(
            format_insert_queries(query_with_function).unwrap(),
            formatted
        );
    }
}
