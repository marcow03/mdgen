mod cli;
mod markdown;

use std::io::{Read, IsTerminal};

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

const DEFAULT_NUM_ROWS: u32 = 10;
const DEFAULT_NUM_COLS: u32 = 3;
const DEFAULT_NUM_TODOS: u32 = 10;

fn main() -> Result<()> {
    // Parse the cli
    let cli = Cli::parse();

    // Execute subcommand
    match cli.command {
        Commands::Table {
            dimensions,
            mut headers,
        } => {
            let mut rows = DEFAULT_NUM_ROWS;
            let mut cols= DEFAULT_NUM_COLS;

            if headers.is_none() && dimensions.is_none() {
                let pipe = get_pipe_content();
                headers = try_parse_table_headers(pipe);
                if let Some(ref headers) = headers {
                    cols = headers.len() as u32;
                }
            }

            if let Some(d) = dimensions {
                cols = d.0;
                rows = d.1;
            }


            let table = markdown::table(cols, rows, headers)?;
            println!("{}", table);
        }
        Commands::Todo {
            num_items,
            mut items,
        } => {
            let mut num = DEFAULT_NUM_TODOS;

            if items.is_none() {
                let pipe = get_pipe_content();
                items = try_parse_todo_items(pipe);
                if let Some(ref items) = items {
                    num = items.len() as u32;
                }
            }

            if let Some(n) = num_items {
                num = n;
            } 

            let todo = markdown::todo_list(num, items);
            println!("{}", todo);
        }
        Commands::Code { language } => {
            let code = markdown::code_block(language);
            println!("{}", code);
        }
        Commands::Quote { lines, quote_type } => {
            let quote = markdown::quote(lines, quote_type);
            println!("{}", quote);
        }
    };

    Ok(())
}

fn get_pipe_content() -> Option<String> {
    // Check if stdin is a terminal (interactive) - if so, don't try to read from it
    if std::io::stdin().is_terminal() {
        return None;
    }

    let mut pipe = String::new();
    match std::io::stdin().read_to_string(&mut pipe) {
        Ok(_) => Some(pipe),
        Err(_) => None,
    }
}

fn try_parse_table_headers(content: Option<String>) -> Option<Vec<String>> {
    let content = content?;
    let lines: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    if lines.is_empty() {
        return None;
    }

    Some(lines)
}

fn try_parse_todo_items(content: Option<String>) -> Option<Vec<String>> {
    let content = content?;
    let lines: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    if lines.is_empty() {
        return None;
    }

    Some(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_table_headers_with_content() {
        let content = Some("header1\nheader2\nheader3".to_string());
        let headers = try_parse_table_headers(content);
        assert_eq!(headers, Some(vec!["header1".to_string(), "header2".to_string(), "header3".to_string()]));
    }

    #[test]
    fn test_try_parse_table_headers_with_whitespace() {
        let content = Some("  header1  \n  header2\nheader3  ".to_string());
        let headers = try_parse_table_headers(content);
        assert_eq!(headers, Some(vec!["header1".to_string(), "header2".to_string(), "header3".to_string()]));
    }

    #[test]
    fn test_try_parse_table_headers_filters_empty_lines() {
        let content = Some("header1\n\nheader2\n\n\nheader3".to_string());
        let headers = try_parse_table_headers(content);
        assert_eq!(headers, Some(vec!["header1".to_string(), "header2".to_string(), "header3".to_string()]));
    }

    #[test]
    fn test_try_parse_table_headers_with_empty_content() {
        let headers = try_parse_table_headers(Some("".to_string()));
        assert_eq!(headers, None);
    }

    #[test]
    fn test_try_parse_table_headers_with_only_whitespace() {
        let headers = try_parse_table_headers(Some("  \n  \n  ".to_string()));
        assert_eq!(headers, None);
    }

    #[test]
    fn test_try_parse_table_headers_with_none() {
        let headers = try_parse_table_headers(None);
        assert_eq!(headers, None);
    }

    #[test]
    fn test_try_parse_todo_items_with_content() {
        let content = Some("Task 1\nTask 2\nTask 3".to_string());
        let items = try_parse_todo_items(content);
        assert_eq!(items, Some(vec!["Task 1".to_string(), "Task 2".to_string(), "Task 3".to_string()]));
    }

    #[test]
    fn test_try_parse_todo_items_with_whitespace() {
        let content = Some("  Task 1  \n  Task 2\nTask 3  ".to_string());
        let items = try_parse_todo_items(content);
        assert_eq!(items, Some(vec!["Task 1".to_string(), "Task 2".to_string(), "Task 3".to_string()]));
    }

    #[test]
    fn test_try_parse_todo_items_filters_empty_lines() {
        let content = Some("Task 1\n\nTask 2\n\n\nTask 3".to_string());
        let items = try_parse_todo_items(content);
        assert_eq!(items, Some(vec!["Task 1".to_string(), "Task 2".to_string(), "Task 3".to_string()]));
    }

    #[test]
    fn test_try_parse_todo_items_with_empty_content() {
        let items = try_parse_todo_items(Some("".to_string()));
        assert_eq!(items, None);
    }

    #[test]
    fn test_try_parse_todo_items_with_only_whitespace() {
        let items = try_parse_todo_items(Some("  \n  \n  ".to_string()));
        assert_eq!(items, None);
    }

    #[test]
    fn test_try_parse_todo_items_with_none() {
        let items = try_parse_todo_items(None);
        assert_eq!(items, None);
    }
}

