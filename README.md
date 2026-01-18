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

- Install [Rust](https://www.rust-lang.org/) on your system
- Run `cargo install --git https://github.com/marcow03/mdgen`

## Usage

For example run the following from a terminal to add a markdown table to a file:

```sh
mdgen table 3x10 -t id,name,age >> file.md
```

Or when editing a file in VIM to insert the output of `mdgen` into the current buffer:

```sh
:r !mdgen todo 10
```

To create a table from pipe (`|`) input:
```sh
echo "id\nname\nage" | mdgen table
```
