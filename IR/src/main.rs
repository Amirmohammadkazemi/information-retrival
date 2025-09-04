// main.rs
mod file_reader;
mod preprocessor;

use std::env;
use std::io;
use anyhow::Result;
use rayon::prelude::*;

use file_reader::{read_text_files, TextDocument};
use preprocessor::TextProcessor;

fn main() -> Result<()> {
    // getting address from user
    let dir_path = if let Some(path) = env::args().nth(1) {
        path
    } else {
        println!("📁 Import documents directory: ");
        let mut input_path = String::new();
        io::stdin().read_line(&mut input_path)?;
        input_path.trim().to_string()
    };

    println!("📁 Reading from: {}", dir_path);

    let documents = read_text_files(&dir_path)?;
    println!("📊 Number of documents: {}", documents.len());

    if documents.is_empty() {
        eprintln!("⚠️  0 Documents");
        return Ok(());
    }

    println!("🔧 Prosessing documents...");

    let processed_docs: Vec<(String, Vec<String>)> = documents
        .par_iter()
        .map(|doc| {
            let processed_tokens = TextProcessor::process_text(&doc.raw_content);
            (doc.file_name.clone(), processed_tokens)
        })
        .collect();

    println!("\n✅ Process completed :\n");

    for (file_name, tokens) in processed_docs {
        println!("📄 File: {}", file_name);
        println!("Number of tokens: {}", tokens.len());
        if !tokens.is_empty() {
            println!("Sample of tokens: {:?}", &tokens[..10.min(tokens.len())]);
        }
        println!();
    }

    Ok(())
}
