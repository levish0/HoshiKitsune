mod codegen;
mod parser;
mod semantic;

use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    let input_path = "ToParse.txt";
    let document_content = fs::read_to_string(input_path)?;
    let document_len = document_content.len();

    let start_time = Instant::now();

    let duration = start_time.elapsed();

    let parsed_json_str =
        serde_json::to_string_pretty("").unwrap_or_else(|_| "\"Serialization Error\"".to_string());

    println!("길이: {}, 소요 시간: {:?}", document_len, duration);

    let output_path = "res.json";

    let mut output_file = fs::File::create(output_path)?;
    write!(output_file, "{}", parsed_json_str)?;

    Ok(())
}
