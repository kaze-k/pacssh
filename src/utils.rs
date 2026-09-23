pub fn spaces(width: usize) -> String {
    " ".repeat(width)
}

pub fn unknown(value: &str) -> Option<&str> {
    if value == "unknown" {
        Some(value)
    } else {
        None
    }
}
