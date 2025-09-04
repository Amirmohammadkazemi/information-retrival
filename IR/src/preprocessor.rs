// preprocessor.rs
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
use rust_stemmers::{Stemmer, Algorithm};
use once_cell::sync::Lazy;

static ENGLISH_STOPWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words = vec![
        "the", "a", "an", "in", "on", "at", "and", "or", "to", "of", "for",
        "is", "are", "was", "were", "be", "been", "being", "this", "that",
        "with", "by", "as", "from", "it", "its", "it's", "but", "not", "have",
        "has", "had", "do", "does", "did", "will", "would", "could", "should",
        "can", "may", "might", "must", "i", "you", "he", "she", "we", "they",
        "me", "him", "her", "us", "them", "my", "your", "his", "our", "their",
        "mine", "yours", "hers", "ours", "theirs", "who", "what", "when", "where",
        "why", "how", "which", "there", "here", "other", "others", "if", "then",
        "else", "while", "because", "though", "although", "until", "unless", "since",
        "so", "such", "than", "too", "very", "just", "now", "out", "up", "down",
        "off", "over", "under", "again", "further", "once", "more", "most", "all",
        "any", "both", "each", "few", "more", "some", "no", "nor", "only", "own",
        "same", "so", "again", "against", "between", "through", "during", "before",
        "after", "above", "below", "into", "from", "about", "further", "then", "now",
    ];
    words.into_iter().collect()
});

pub struct TextProcessor;

impl TextProcessor {
    /// پردازش کامل متن
    pub fn process_text(text: &str) -> Vec<String> {
        let tokens = Self::tokenize(text);
        let lower_tokens = Self::to_lowercase(tokens);
        let filtered_tokens = Self::remove_stopwords(lower_tokens);
        filtered_tokens
    }

    pub fn tokenize(text: &str) -> Vec<String> {
        text.unicode_words()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn to_lowercase(tokens: Vec<String>) -> Vec<String> {
        tokens.into_iter()
            .map(|token| token.to_lowercase())
            .collect()
    }

    pub fn remove_stopwords(tokens: Vec<String>) -> Vec<String> {
        tokens.into_iter()
            .filter(|token| !ENGLISH_STOPWORDS.contains(token.as_str()))
            .collect()
    }

    /// تشخیص زبان متن (ساده)
    pub fn detect_language(text: &str) -> Language {
        // یک تشخیص ساده بر اساس کاراکترها
        let persian_chars = text.chars().filter(|c| {
            let c = *c as u32;
            (0x0600..=0x06FF).contains(&c) || // محدوده عربی/فارسی
            (0xFB50..=0xFDFF).contains(&c) || // فرم‌های ارائه عربی-الف
            (0xFE70..=0xFEFF).contains(&c)    // فرم‌های ارائه عربی-ب
        }).count();

        let total_chars = text.chars().count();

        if total_chars > 0 && (persian_chars as f32 / total_chars as f32) > 0.3 {
            Language::Persian
        } else if total_chars > 0 {
            Language::English
        } else {
            Language::Unknown
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Persian,
    Unknown,
}
