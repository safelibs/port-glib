#![allow(non_camel_case_types)]

pub mod ffi;

pub fn workspace_marker() -> &'static str {
    "impl-safe-bootstrap"
}
