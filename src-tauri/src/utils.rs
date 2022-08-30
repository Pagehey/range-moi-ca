pub fn pluralize(str: &str, count: usize) -> String {
    if count > 0 {
        return format!("{} {}s", count, str);
    }
    return format!("{} {}", count, str);
}
