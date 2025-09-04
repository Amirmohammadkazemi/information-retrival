// lib.rs
pub mod file_reader;
pub mod preprocessor;

pub use file_reader::read_text_files;
pub use preprocessor::TextProcessor;
