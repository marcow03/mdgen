# `mdgen`

`mdgen` is a markdown generator tool that helps you create elements for your markdown files efficently.

## Features

- Generate markdown tables, todo lists, code blocks, and quotes
- Easy-to-use command-line interface

```text
Markdown Generator Tool

Usage: mdgen <COMMAND>

Commands:
  table  Generate a markdown table
  todo   Generate a markdown todo list
  code   Generate a markdown code block
  quote  Generate a markdown quote
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

- Install Rust on your system
- Clone this repo
- Run `cargo install --path .` from the project directory

## Usage

Run the following from a terminal to add a markdown table to a file:

```sh
mdgen table 3x10 -t id,name,age >> file.md
```
