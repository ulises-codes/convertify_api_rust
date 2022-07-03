pub fn format_key(key: &str) -> String {
    key.to_lowercase().replace(" ", "_")
}
