use std::ffi::{c_char, CStr};
use std::sync::{Mutex, OnceLock};

use crate::abi::GError;
use crate::bridge;
use crate::ffi::{gboolean, gchar, gsize, gssize};

#[cfg(unix)]
unsafe extern "C" {
    fn getuid() -> u32;
    fn geteuid() -> u32;
    fn getgid() -> u32;
    fn getegid() -> u32;
    fn getenv(name: *const c_char) -> *const c_char;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: i32) -> i32;
    fn unsetenv(name: *const c_char) -> i32;
}

#[cfg(unix)]
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(unix)]
fn forced_privileged_context() -> bool {
    std::env::var_os("SAFE_GLIB_TEST_FORCE_PRIVILEGED").is_some()
}

#[cfg(unix)]
fn is_privileged_context() -> bool {
    forced_privileged_context() || unsafe { getuid() != geteuid() || getgid() != getegid() }
}

#[cfg(not(unix))]
fn is_privileged_context() -> bool {
    false
}

#[cfg(unix)]
unsafe fn with_sanitized_env<T>(f: impl FnOnce() -> T) -> T {
    const NAMES: [&CStr; 3] = [c"CHARSET", c"G_FILENAME_ENCODING", c"G_BROKEN_FILENAMES"];

    let _guard = env_lock().lock().expect("env mutex poisoned");
    let mut saved = Vec::with_capacity(NAMES.len());

    for name in NAMES {
        let value = getenv(name.as_ptr());
        let saved_value = if value.is_null() {
            None
        } else {
            Some(CStr::from_ptr(value).to_owned())
        };
        saved.push((name, saved_value));
        unsetenv(name.as_ptr());
    }

    let result = f();

    for (name, value) in saved {
        if let Some(value) = value {
            setenv(name.as_ptr(), value.as_ptr(), 1);
        } else {
            unsetenv(name.as_ptr());
        }
    }

    result
}

unsafe fn sanitize_if_needed<T>(f: impl FnOnce() -> T) -> T {
    #[cfg(unix)]
    {
        if is_privileged_context() {
            return with_sanitized_env(f);
        }
    }

    f()
}

#[unsafe(export_name = "g_get_charset")]
pub unsafe extern "C" fn get_charset(charset: *mut *const gchar) -> gboolean {
    sanitize_if_needed(|| bridge::get_charset(charset))
}

#[unsafe(export_name = "g_get_filename_charsets")]
pub unsafe extern "C" fn get_filename_charsets(
    filename_charsets: *mut *const *const gchar,
) -> gboolean {
    sanitize_if_needed(|| bridge::get_filename_charsets(filename_charsets))
}

#[unsafe(export_name = "g_locale_to_utf8")]
pub unsafe extern "C" fn locale_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    sanitize_if_needed(|| bridge::locale_to_utf8(opsysstring, len, bytes_read, bytes_written, error))
}

#[unsafe(export_name = "g_locale_from_utf8")]
pub unsafe extern "C" fn locale_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    sanitize_if_needed(|| bridge::locale_from_utf8(utf8string, len, bytes_read, bytes_written, error))
}

#[unsafe(export_name = "g_filename_to_utf8")]
pub unsafe extern "C" fn filename_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    sanitize_if_needed(|| bridge::filename_to_utf8(opsysstring, len, bytes_read, bytes_written, error))
}

#[unsafe(export_name = "g_filename_from_utf8")]
pub unsafe extern "C" fn filename_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    sanitize_if_needed(|| bridge::filename_from_utf8(utf8string, len, bytes_read, bytes_written, error))
}

#[unsafe(export_name = "g_filename_display_name")]
pub unsafe extern "C" fn filename_display_name(filename: *const gchar) -> *mut gchar {
    sanitize_if_needed(|| bridge::filename_display_name(filename))
}

#[unsafe(export_name = "g_filename_display_basename")]
pub unsafe extern "C" fn filename_display_basename(filename: *const gchar) -> *mut gchar {
    sanitize_if_needed(|| bridge::filename_display_basename(filename))
}
