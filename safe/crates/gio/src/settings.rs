use crate::ffi::{gchar, gpointer};

pub const CLUSTER: &str = "settings";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn g_settings_new(_schema_id: *const gchar) -> gpointer {
    crate::runtime::opaque_handle(0x7365_7474)
}
