use crate::ffi::{gchar, gpointer};

pub const CLUSTER: &str = "application";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn g_application_new(_application_id: *const gchar, _flags: i32) -> gpointer {
    crate::runtime::opaque_handle(0x6170_706c)
}
