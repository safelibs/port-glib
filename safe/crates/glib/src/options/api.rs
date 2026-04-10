use crate::abi::GOptionEntry;

pub(crate) fn has_long_name(entry: &GOptionEntry) -> bool {
    !entry.long_name.is_null()
}

pub(crate) fn phase_marker() -> &'static str {
    "options"
}
