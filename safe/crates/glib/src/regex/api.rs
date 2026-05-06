pub(crate) fn looks_like_literal(pattern: &str) -> bool {
    !pattern.bytes().any(|byte| matches!(byte, b'[' | b'(' | b'{' | b'\\' | b'+' | b'*' | b'?'))
}

pub(crate) fn phase_marker() -> &'static str {
    "regex"
}
