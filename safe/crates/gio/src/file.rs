use crate::ffi::{gchar, gpointer};

pub const CLUSTER: &str = "file";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn g_file_new_for_path(_path: *const gchar) -> gpointer {
    crate::runtime::opaque_handle(0x6669_6c65)
}
