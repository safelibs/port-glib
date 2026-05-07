extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    static mut safe_c2rust_stdout: *mut FILE;
    static mut safe_c2rust_stderr: *mut FILE;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vsprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vasprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __f: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gulong = ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_vprintf(
    mut __fmt: *const ::core::ffi::c_char,
    mut __arg: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    return vfprintf(safe_c2rust_stdout, __fmt, __arg.clone());
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_vsnprintf\0" as *const u8 as *const ::core::ffi::c_char;
fn safe_c2rust_is_ascii_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}
fn safe_c2rust_is_printf_conversion(byte: u8) -> bool {
    matches!(
        byte,
        b'd' | b'i'
            | b'o'
            | b'u'
            | b'x'
            | b'X'
            | b'f'
            | b'F'
            | b'e'
            | b'E'
            | b'g'
            | b'G'
            | b'a'
            | b'A'
            | b'c'
            | b'C'
            | b's'
            | b'S'
            | b'p'
            | b'n'
            | b'm'
    )
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_printf(mut format: *const gchar, mut args: ...) -> gint {
    let mut args_0: ::core::ffi::VaList;
    let mut retval: gint = 0;
    args_0 = args.clone();
    retval = safe_c2rust_g_vprintf(format, args_0.clone());
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_fprintf(
    mut file: *mut FILE,
    mut format: *const gchar,
    mut args: ...
) -> gint {
    let mut args_0: ::core::ffi::VaList;
    let mut retval: gint = 0;
    args_0 = args.clone();
    retval = safe_c2rust_g_vfprintf(file, format, args_0.clone());
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sprintf(
    mut string: *mut gchar,
    mut format: *const gchar,
    mut args: ...
) -> gint {
    let mut args_0: ::core::ffi::VaList;
    let mut retval: gint = 0;
    args_0 = args.clone();
    retval = safe_c2rust_g_vsprintf(string, format, args_0.clone());
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_snprintf(
    mut string: *mut gchar,
    mut n: gulong,
    mut format: *const gchar,
    mut args: ...
) -> gint {
    let mut args_0: ::core::ffi::VaList;
    let mut retval: gint = 0;
    args_0 = args.clone();
    retval = safe_c2rust_g_vsnprintf(string, n, format, args_0.clone());
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vprintf(
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gint {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return safe_c2rust_vprintf(format as *const ::core::ffi::c_char, args.clone()) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfprintf(
    mut file: *mut FILE,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gint {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return vfprintf(
        file,
        format as *const ::core::ffi::c_char,
        args.clone(),
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vsprintf(
    mut string: *mut gchar,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gint {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return vsprintf(
        string as *mut ::core::ffi::c_char,
        format as *const ::core::ffi::c_char,
        args.clone(),
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vsnprintf(
    mut string: *mut gchar,
    mut n: gulong,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gint {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if n == 0 as gulong || !string.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n == 0 || string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return vsnprintf(
        string as *mut ::core::ffi::c_char,
        n as size_t,
        format as *const ::core::ffi::c_char,
        args.clone(),
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vasprintf(
    mut string: *mut *mut gchar,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gint {
    let mut len: gint = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    let mut saved_errno: ::core::ffi::c_int = 0;
    if !format.is_null() {
        let mut cursor = format as *const u8;
        while *cursor != 0 {
            if *cursor != b'%' {
                cursor = cursor.add(1);
                continue;
            }

            cursor = cursor.add(1);
            if *cursor == b'%' {
                cursor = cursor.add(1);
                continue;
            }

            let positional_start = cursor;
            while safe_c2rust_is_ascii_digit(*cursor) {
                cursor = cursor.add(1);
            }
            if *cursor == b'$' {
                cursor = cursor.add(1);
            } else {
                cursor = positional_start;
            }

            while matches!(*cursor, b'\'' | b'-' | b'+' | b' ' | b'#' | b'0' | b'I') {
                cursor = cursor.add(1);
            }

            if *cursor == b'*' {
                cursor = cursor.add(1);
                let width_start = cursor;
                while safe_c2rust_is_ascii_digit(*cursor) {
                    cursor = cursor.add(1);
                }
                if *cursor == b'$' {
                    cursor = cursor.add(1);
                } else {
                    cursor = width_start;
                }
            } else {
                while safe_c2rust_is_ascii_digit(*cursor) {
                    cursor = cursor.add(1);
                }
            }

            if *cursor == b'.' {
                cursor = cursor.add(1);
                if *cursor == b'*' {
                    cursor = cursor.add(1);
                    let precision_start = cursor;
                    while safe_c2rust_is_ascii_digit(*cursor) {
                        cursor = cursor.add(1);
                    }
                    if *cursor == b'$' {
                        cursor = cursor.add(1);
                    } else {
                        cursor = precision_start;
                    }
                } else {
                    while safe_c2rust_is_ascii_digit(*cursor) {
                        cursor = cursor.add(1);
                    }
                }
            }

            let mut saw_length_modifier = 1 as ::core::ffi::c_int;
            match *cursor {
                b'h' => {
                    cursor = cursor.add(1);
                    if *cursor == b'h' {
                        cursor = cursor.add(1);
                    }
                }
                b'l' => {
                    cursor = cursor.add(1);
                    if *cursor == b'l' {
                        cursor = cursor.add(1);
                    }
                }
                b'j' | b'z' | b't' | b'L' | b'q' | b'Z' => {
                    cursor = cursor.add(1);
                }
                _ => saw_length_modifier = 0 as ::core::ffi::c_int,
            }
            if saw_length_modifier != 0 && !safe_c2rust_is_printf_conversion(*cursor) {
                *string = ::core::ptr::null_mut::<gchar>();
                return -(1 as gint);
            }
            if *cursor != 0 {
                cursor = cursor.add(1);
            }
        }
    }
    len = vasprintf(
        string as *mut *mut ::core::ffi::c_char,
        format as *const ::core::ffi::c_char,
        args.clone(),
    ) as gint;
    saved_errno = *__errno_location();
    if len < 0 as ::core::ffi::c_int {
        if saved_errno == ENOMEM {
            fputs(
                b"../original/glib/gprintf.c:350\0" as *const u8 as *const ::core::ffi::c_char,
                safe_c2rust_stderr,
            );
            fputs(
                b": failed to allocate memory\n\0" as *const u8 as *const ::core::ffi::c_char,
                safe_c2rust_stderr,
            );
            abort();
        } else {
            *string = ::core::ptr::null_mut::<gchar>();
        }
    }
    return len;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
