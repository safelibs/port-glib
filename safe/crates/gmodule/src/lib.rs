#![allow(non_camel_case_types)]

pub mod abi {
    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum GModuleFlags {
        BindLazy = 1 << 0,
        BindLocal = 1 << 1,
        BindMask = 0x03,
    }
}

pub const CRATE_ID: &str = "safe-gmodule";

pub fn bootstrap_marker() -> &'static str {
    "impl-safe-bootstrap"
}
