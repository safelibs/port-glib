pub(crate) fn scheme_prefix_len(text: &[u8]) -> Option<usize> {
    let mut iter = text.iter().copied();
    let first = iter.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    for (index, byte) in text.iter().copied().enumerate().skip(1) {
        if byte == b':' {
            return Some(index);
        }
        if !(byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.')) {
            return None;
        }
    }
    None
}

pub(crate) fn phase_marker() -> &'static str {
    "uri"
}
