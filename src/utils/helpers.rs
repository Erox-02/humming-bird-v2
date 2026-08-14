use regex::Regex;

pub fn truncate_text(text: &str, max_length: usize, ellipsis: &str) -> String {
    if text.len() <= max_length {
        return text.to_string();
    }  
    let mut truncated = text[..max_length].to_string();
    if let Some(last_space) = truncated.rfind(' ') {
        if last_space > 0 {
            truncated.truncate(last_space);
        }
    }
    format!("{}{}", truncated, ellipsis)
}
pub fn safe_regex_escape(text: &str) -> String {
    regex::escape(text)
}
pub fn normalize_text(text: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(text.trim(), " ").to_string()
}
pub fn is_mostly_uppercase(text: &str, threshold: f32) -> bool {
    if text.is_empty() {
        return false;
    }
    let letters: usize = text.chars().filter(|c| c.is_alphabetic()).count();
    if letters == 0 {
        return false;
    }
    let uppercase: usize = text.chars().filter(|c| c.is_uppercase()).count();
    (uppercase as f32) / (letters as f32) >= threshold
}
pub fn extract_numbers(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}
pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}
