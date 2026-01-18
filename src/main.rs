mod cli;
mod markdown;

use std::io::Read;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    // Parse the cli
    let cli = Cli::parse();

    // Execute subcommand
    match cli.command {
        Commands::Table {
            dimensions,
            headers,
        } => {
            let mut dimensions = dimensions;
            let mut headers = headers;

            let pipe = get_pipe_content();
            if !pipe.is_empty() {
                headers = try_parse_table_headers(pipe);
                if let Some(ref headers) = headers {
                    dimensions.0 = headers.len() as u32;
                }
            }

            let table = markdown::table(dimensions.0, dimensions.1, headers)?;
            println!("{}", table);
        }
        Commands::Todo { num_items, items } => {
            let mut items = items;

            let pipe = get_pipe_content();
            if !pipe.is_empty() {
                items = try_parse_todo_items(pipe);
            }

            let todo = markdown::todo_list(num_items, items);
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
    let _ = std::io::stdin().lock().read_to_string(&mut pipe);

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
