pub(crate) fn line_ending_width(input: &[u8]) -> usize {
    match input {
        [b'\r', b'\n', ..] => 2,
        [b'\r' | b'\n', ..] => 1,
        _ => 0,
    }
}

pub(crate) fn phase_marker() -> &'static str {
    "scanner"
}
