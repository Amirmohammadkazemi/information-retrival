// file_reader.rs
use std::path::Path;
use walkdir::WalkDir;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TextDocument {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub raw_content: String,
}

pub fn read_text_files(dir_path: &str) -> Result<Vec<TextDocument>> {
    let mut documents = Vec::new();

    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && path.extension().map(|ext| ext == "txt").unwrap_or(false) {
            match std::fs::read_to_string(path) {
                Ok(content) => {
                    let file_path = path.to_string_lossy().to_string();
                    let file_name = path.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| file_path.clone());

                    documents.push(TextDocument {
                        id: file_path.clone(),
                        file_path,
                        file_name,
                        raw_content: content,
                    });
                }
                Err(e) => eprintln!("⚠️  Error in readding files : {}: {}", path.display(), e),
            }
        }
    }

    Ok(documents)
}

pub fn read_single_text_file(file_path: &str) -> Result<TextDocument> {
    let path = Path::new(file_path);
    let content = std::fs::read_to_string(path)?;

    let file_name = path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| file_path.to_string());

    Ok(TextDocument {
        id: file_path.to_string(),
        file_path: file_path.to_string(),
        file_name,
        raw_content: content,
    })
}
