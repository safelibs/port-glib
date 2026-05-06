#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! Main-loop runtime state and ABI-forwarded source primitives.

pub fn phase_marker() -> &'static str {
    crate::bootstrap_marker()
}
