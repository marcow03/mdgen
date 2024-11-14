mod cli;
mod markdown;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    // Parse the cli
    let cli = Cli::parse();

    // Execute subcommand
    match cli.command {
        Commands::Table { dimensions, headers } => {
            let table = markdown::table(dimensions.0, dimensions.1, headers)?;
            println!("{}", table);
        },
        Commands::Todo { items } => {
            let todo = markdown::todo_list(items);
            println!("{}", todo);
        },
        Commands::Code { language } => {
            let code = markdown::code_block(language);
            println!("{}", code);
        },
        Commands::Quote { lines, quote_type } => {
            let quote = markdown::quote(lines, quote_type);
            println!("{}", quote);
        },
    };

    Ok(())
}
