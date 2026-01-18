use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
#[clap(
    version = "0.0.1",
    author = "github.com/marcow03",
    about = "Markdown Generator Tool"
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Generate a markdown table")]
    Table {
        #[clap(
            short = 'd',
            long,
            value_parser = parse_table_dimensions,
            default_value = "3x3",
            help = "Table dimensions in the format <rows>x<columns> or <rows>,<columns> (e.g. 3x4 or 3,4)"
        )]
        dimensions: (u32, u32),
        #[clap(
            short = 't',
            long,
            value_delimiter = ',',
            help = "Table headers in the format <header1>,<header2>,..."
        )]
        headers: Option<Vec<String>>,
    },
    #[clap(about = "Generate a markdown todo list")]
    Todo {
        #[clap(
            short = 'n',
            long,
            default_value_t = 5,
            help = "Amount of todo list items"
        )]
        num_items: u32,
        #[clap(
            short = 'i',
            long,
            value_delimiter = ',',
            help = "Items in the format <item1>,<item2>,..."
        )]
        items: Option<Vec<String>>,
    },
    #[clap(about = "Generate a markdown code block")]
    Code {
        #[clap(help = "Code block language")]
        language: String,
    },
    #[clap(about = "Generate a markdown quote")]
    Quote {
        #[clap(help = "Length of the quote")]
        lines: u32,
        #[clap(short = 't', long, help = "Type of quote")]
        quote_type: Option<String>,
    },
}

fn parse_table_dimensions(value: &str) -> Result<(u32, u32), String> {
    // Split the value by 'x' and ',' and try to parse the values
    let re = Regex::new(r"[x,]").unwrap();
    let dimensions: Vec<&str> = re.split(value).collect();

    if dimensions.len() != 2 {
        return Err("Invalid seperator, use 'x' or ','".to_string());
    }

    let columns = dimensions[0]
        .parse()
        .map_err(|_| "Invalid columns".to_string())?;
    let rows = dimensions[1]
        .parse()
        .map_err(|_| "Invalid rows".to_string())?;

    Ok((columns, rows))
}
