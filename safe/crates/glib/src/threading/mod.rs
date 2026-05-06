#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! Threading and synchronization surface for the GLib phase.

pub fn phase_marker() -> &'static str {
    crate::bootstrap_marker()
}
