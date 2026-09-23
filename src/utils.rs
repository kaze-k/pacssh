pub fn spaces(width: usize) -> String {
    " ".repeat(width)
}

pub fn unknown(value: &str) -> Option<()> {
    if value == "unknown" { Some(()) } else { None }
}
