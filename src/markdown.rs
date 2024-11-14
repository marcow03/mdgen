use anyhow::{anyhow, Result};

pub fn table(columns: u32, rows: u32, headers: Option<Vec<String>>) -> Result<String> {
    // Check if the dimensions are valid
    if columns < 1 || rows < 1 {
        return Err(anyhow!("Invalid dimensions"));
    }

    // Chek if the number of headers is equal to the number of columns
    if let Some(headers) = &headers {
        if headers.len() as u32 != columns {
            return Err(anyhow!("Invalid number of headers"));
        }
    }

    let mut table = String::new();

    // Generate the header
    table.push_str("|");
    for column in 0..columns {
        if let Some(headers) = &headers {
            table.push_str(&format!(" {} |", headers[column as usize]));
        } else {
            table.push_str("   |");
        }
    }
    table.push_str("\n");

    // Generate the separator
    table.push_str("|");
    for column in 0..columns {
        if let Some(headers) = &headers {
            let header_length = headers[column as usize].len() as i32;
            table.push_str(&format!("-{}-|", multiply_string("-", header_length)));
        } else {
            table.push_str("---|");
        }
    }
    table.push_str("\n");

    // Generate the rows
    for _ in 0..rows {
        table.push_str("|");
        for column in 0..columns {
            if let Some(headers) = &headers {
                let header_length = headers[column as usize].len() as i32;
                table.push_str(&format!(" {} |", multiply_string(" ", header_length)));
            } else {
                table.push_str("   |");
            }
        }
        table.push_str("\n");
    }

    Ok(table)
}

pub fn todo_list(items: u32) -> String {
    let mut todo = String::new();
    for _ in 0..items {
        todo.push_str("- [ ] \n");
    }

    todo
}

pub fn code_block(language: String) -> String {
    format!("```{}\n\n```", language)
}

pub fn quote(lines: u32, quote_type: Option<String>) -> String {
    let mut quote_block = String::new();

    if let Some(quote_type) = quote_type {
        quote_block.push_str(&format!("> [!{}]\n", quote_type));
    }

    for _ in 0..lines {
        quote_block.push_str("> \n");
    }

    quote_block
}

fn multiply_string(string: &str, length: i32) -> String {
    let mut sequence = String::new();
    for _ in 0..length {
        sequence.push_str(string);
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let headers = vec!["Header 1".into(), "Header 2".into(), "Header 3".into()];
        assert_eq!(
            table(3, 3, Some(headers)).unwrap(),
            "| Header 1 | Header 2 | Header 3 |\n\
             |----------|----------|----------|\n\
             |          |          |          |\n\
             |          |          |          |\n\
             |          |          |          |\n"
        );
    }

    #[test]
    fn test_table_wrong_dimensions() {
        assert_eq!(
            format!("{}",
                table(0, 0, None).unwrap_err().root_cause()
            ),
            "Invalid dimensions"
        );
    }

    #[test]
    fn test_table_wrong_headers() {
        let headers = vec!["Header 1".into(), "Header 2".into()];
        assert_eq!(
            format!("{}",
                table(3, 3, Some(headers)).unwrap_err().root_cause()
            ),
            "Invalid number of headers"
        );
    }

    #[test]
    fn test_todo_list() {
        assert_eq!(
            todo_list(3),
            "- [ ] \n- [ ] \n- [ ] \n"
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            code_block("rust".into()),
            "```rust\n\n```"
        );
    }

    #[test]
    fn test_quote() {
        assert_eq!(
            quote(3, Some("important".into())),
            "> [!important]\n\
             > \n\
             > \n\
             > \n"
        );
    }

    #[test]
    fn test_multiply_string() {
        assert_eq!(
            multiply_string("-", 3),
            "---"
        );
    }

    #[test]
    fn test_multiply_string_empty() {
        assert_eq!(
            multiply_string("-", 0),
            ""
        );
    }
}