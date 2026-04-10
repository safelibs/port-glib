use std::ffi::c_void;

use crate::abi::GError;
use crate::backend;
use crate::ffi::{gboolean, gchar, gint, gsize, GQuark};

type GKeyFile = c_void;
type GKeyFileFlags = gint;

unsafe extern "C" {
    fn g_key_file_error_quark() -> GQuark;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
}

const FALSE: gboolean = 0;
const G_KEY_FILE_ERROR_PARSE: gint = 1;

#[unsafe(export_name = "g_key_file_load_from_data")]
pub unsafe extern "C" fn key_file_load_from_data(
    key_file: *mut GKeyFile,
    data: *const gchar,
    length: gsize,
    flags: GKeyFileFlags,
    error: *mut *mut GError,
) -> gboolean {
    if data.is_null() && length != 0 {
        if !error.is_null() && (*error).is_null() {
            *error = g_error_new_literal(
                g_key_file_error_quark(),
                G_KEY_FILE_ERROR_PARSE,
                c"Key file buffer is NULL for a non-empty input".as_ptr(),
            );
        }
        return FALSE;
    }

    backend::key_file_load_from_data(key_file, data, length, flags, error)
}
