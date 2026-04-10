#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/generated-exports.rs"));

#[used]
#[export_name = "g_param_spec_types"]
pub static mut g_param_spec_types: *mut crate::ffi::GType = core::ptr::null_mut();

pub(crate) static mut __safe_gobject_param_spec_types_storage: [crate::ffi::GType; 23] = [0; 23];
