pub mod error;
pub mod structs;

#[inline]
pub fn strip_0x_prefix(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

#[inline]
pub fn add_0x_prefix(s: &str) -> String {
    if s.starts_with("0x") {
        s.to_string()
    } else {
        format!("0x{s}")
    }
}
