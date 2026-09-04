//! Basic SQL parser
//!
//! Parses SQL statements into an AST.

use crate::error::{DatabaseError, Result};
use serde::{Deserialize, Serialize};

/// SQL statement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SQLStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    CreateTable(CreateTableStatement),
}

/// SELECT statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectStatement {
    pub columns: Vec<String>,
    pub from: String,
    pub where_clause: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<usize>,
}

/// INSERT statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
}

/// UPDATE statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatement {
    pub table: String,
    pub assignments: Vec<(String, String)>,
    pub where_clause: Option<String>,
}

/// DELETE statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStatement {
    pub table: String,
    pub where_clause: Option<String>,
}

/// CREATE TABLE statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableStatement {
    pub name: String,
    pub columns: Vec<(String, String)>, // (name, type)
}

/// SQL Parser
pub struct SQLParser;

impl SQLParser {
    /// Parse a SQL statement
    pub fn parse(sql: &str) -> Result<SQLStatement> {
        let tokens: Vec<&str> = sql.split_whitespace().collect();

        if tokens.is_empty() {
            return Err(DatabaseError::ParseError("Empty SQL statement".to_string()));
        }

        match tokens[0].to_uppercase().as_str() {
            "SELECT" => Self::parse_select(&tokens),
            "INSERT" => Self::parse_insert(&tokens),
            "UPDATE" => Self::parse_update(&tokens),
            "DELETE" => Self::parse_delete(&tokens),
            "CREATE" => Self::parse_create(&tokens),
            _ => Err(DatabaseError::ParseError(format!(
                "Unknown statement: {}",
                tokens[0]
            ))),
        }
    }

    fn parse_select(tokens: &[&str]) -> Result<SQLStatement> {
        // Simplified: SELECT col1, col2 FROM table WHERE ...
        let mut columns = Vec::new();
        let mut from = String::new();
        let mut where_clause = None;

        let mut i = 1;

        // Parse columns
        while i < tokens.len() && tokens[i].to_uppercase() != "FROM" {
            columns.push(tokens[i].trim_end_matches(',').to_string());
            i += 1;
        }

        // Parse FROM
        if i < tokens.len() && tokens[i].to_uppercase() == "FROM" {
            i += 1;
            if i < tokens.len() {
                from = tokens[i].to_string();
                i += 1;
            }
        }

        // Parse WHERE
        if i < tokens.len() && tokens[i].to_uppercase() == "WHERE" {
            i += 1;
            where_clause = Some(tokens[i..].join(" "));
        }

        Ok(SQLStatement::Select(SelectStatement {
            columns,
            from,
            where_clause,
            order_by: None,
            limit: None,
        }))
    }

    fn parse_insert(_tokens: &[&str]) -> Result<SQLStatement> {
        // TODO: Implement INSERT parsing
        Ok(SQLStatement::Insert(InsertStatement {
            table: "table".to_string(),
            columns: Vec::new(),
            values: Vec::new(),
        }))
    }

    fn parse_update(_tokens: &[&str]) -> Result<SQLStatement> {
        // TODO: Implement UPDATE parsing
        Ok(SQLStatement::Update(UpdateStatement {
            table: "table".to_string(),
            assignments: Vec::new(),
            where_clause: None,
        }))
    }

    fn parse_delete(tokens: &[&str]) -> Result<SQLStatement> {
        // DELETE FROM table WHERE ...
        let table = if tokens.len() > 2 && tokens[1].to_uppercase() == "FROM" {
            tokens[2].to_string()
        } else {
            return Err(DatabaseError::ParseError("Invalid DELETE syntax".to_string()));
        };

        let where_clause = if tokens.len() > 4 && tokens[3].to_uppercase() == "WHERE" {
            Some(tokens[4..].join(" "))
        } else {
            None
        };

        Ok(SQLStatement::Delete(DeleteStatement { table, where_clause }))
    }

    fn parse_create(tokens: &[&str]) -> Result<SQLStatement> {
        // CREATE TABLE name (col1 type1, col2 type2, ...)
        if tokens.len() < 3 || tokens[1].to_uppercase() != "TABLE" {
            return Err(DatabaseError::ParseError("Invalid CREATE syntax".to_string()));
        }

        let name = tokens[2].to_string();
        // TODO: Parse column definitions

        Ok(SQLStatement::CreateTable(CreateTableStatement {
            name,
            columns: Vec::new(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_select() {
        let sql = "SELECT id name FROM users WHERE age > 18";
        let stmt = SQLParser::parse(sql).unwrap();
        match stmt {
            SQLStatement::Select(s) => {
                assert_eq!(s.from, "users");
                assert!(s.where_clause.is_some());
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[test]
    fn test_parse_delete() {
        let sql = "DELETE FROM users WHERE id = 1";
        let stmt = SQLParser::parse(sql).unwrap();
        match stmt {
            SQLStatement::Delete(d) => {
                assert_eq!(d.table, "users");
                assert!(d.where_clause.is_some());
            }
            _ => panic!("Expected DELETE statement"),
        }
    }
}
