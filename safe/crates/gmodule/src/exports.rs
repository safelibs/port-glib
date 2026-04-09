#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[export_name = "g_module_build_path"]
pub static PLACEHOLDER_00003: u8 = 0;

#[export_name = "g_module_close"]
pub unsafe extern "C" fn export_g_module_close(
    module: *mut crate::runtime::GModuleHandle,
) -> crate::ffi::gboolean {
    crate::runtime::g_module_close_impl(module)
}

#[export_name = "g_module_error"]
pub static PLACEHOLDER_00008: u8 = 0;

#[export_name = "g_module_error_quark"]
pub static PLACEHOLDER_00011: u8 = 0;

#[export_name = "g_module_make_resident"]
pub static PLACEHOLDER_00014: u8 = 0;

#[export_name = "g_module_name"]
pub static PLACEHOLDER_00017: u8 = 0;

#[export_name = "g_module_open"]
pub unsafe extern "C" fn export_g_module_open(
    file_name: *const crate::ffi::gchar,
    flags: crate::ffi::gint,
) -> *mut crate::runtime::GModuleHandle {
    crate::runtime::g_module_open_impl(file_name, flags)
}

#[export_name = "g_module_open_full"]
pub static PLACEHOLDER_00022: u8 = 0;

#[export_name = "g_module_supported"]
pub unsafe extern "C" fn export_g_module_supported() -> crate::ffi::gboolean {
    crate::runtime::g_module_supported_impl()
}

#[export_name = "g_module_symbol"]
pub static PLACEHOLDER_00027: u8 = 0;
