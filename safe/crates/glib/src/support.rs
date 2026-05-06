use core::ffi::{c_char, c_void};

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdin: *mut c_void;
    static mut stdout: *mut c_void;
    static mut environ: *mut *mut c_char;
}

#[unsafe(export_name = "safe_c2rust_stderr")]
pub static mut SAFE_C2RUST_STDERR: *mut c_void = core::ptr::null_mut();
#[unsafe(export_name = "safe_c2rust_stdin")]
pub static mut SAFE_C2RUST_STDIN: *mut c_void = core::ptr::null_mut();
#[unsafe(export_name = "safe_c2rust_stdout")]
pub static mut SAFE_C2RUST_STDOUT: *mut c_void = core::ptr::null_mut();
#[unsafe(export_name = "safe_c2rust_environ")]
pub static mut SAFE_C2RUST_ENVIRON: *mut *mut c_char = core::ptr::null_mut();

extern "C" fn init_libc_globals() {
    unsafe {
        SAFE_C2RUST_STDERR = stderr;
        SAFE_C2RUST_STDIN = stdin;
        SAFE_C2RUST_STDOUT = stdout;
        SAFE_C2RUST_ENVIRON = environ;
        crate::translated::glib_init::safe_c2rust_glib_init();
    }
}

#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static SAFE_GLIB_INIT_LIBC_GLOBALS: extern "C" fn() = init_libc_globals;

#[unsafe(export_name = "__lsan_ignore_object")]
pub unsafe extern "C" fn lsan_ignore_object(_object: *const c_void) {}

#[unsafe(export_name = "__lsan_enable")]
pub unsafe extern "C" fn lsan_enable() {}

#[unsafe(export_name = "_g_locale_charset_raw")]
pub unsafe extern "C" fn locale_charset_raw() -> *const c_char {
    let codeset = libc::nl_langinfo(libc::CODESET);
    if codeset.is_null() || *codeset == 0 {
        c"US-ASCII".as_ptr()
    } else {
        codeset.cast_const()
    }
}

#[unsafe(export_name = "_g_locale_charset_unalias")]
pub unsafe extern "C" fn locale_charset_unalias(codeset: *const c_char) -> *const c_char {
    if codeset.is_null() {
        c"UTF-8".as_ptr()
    } else {
        codeset
    }
}

#[unsafe(export_name = "_g_locale_get_charset_aliases")]
pub unsafe extern "C" fn locale_get_charset_aliases() -> *const c_char {
    static EMPTY_ALIASES: [c_char; 2] = [0, 0];
    EMPTY_ALIASES.as_ptr()
}
