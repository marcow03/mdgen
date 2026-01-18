mod cli;
mod markdown;

use std::io::Read;

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

fn get_pipe_content() -> String {
    let mut pipe = String::new();
    let _ = std::io::stdin().read_to_string(&mut pipe);

    pipe
}

fn try_parse_table_headers(content: String) -> Option<Vec<String>> {
    let lines = content.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    if lines.is_empty() {
        return None;
    }

    Some(lines)
}

fn try_parse_todo_items(content: String) -> Option<Vec<String>> {
    let lines = content.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    if lines.is_empty() {
        return None;
    }

    Some(lines)
}
