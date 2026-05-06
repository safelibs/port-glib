extern "C" {
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut safe_c2rust_environ: *mut *mut ::core::ffi::c_char;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_thread_n_created() -> guint;
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GLogLevelFlags = ::core::ffi::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_environ_matches(
    mut env: *const gchar,
    mut variable: *const gchar,
    mut len: gsize,
) -> gboolean {
    return (strncmp(
        env as *const ::core::ffi::c_char,
        variable as *const ::core::ffi::c_char,
        len as size_t,
    ) == 0 as ::core::ffi::c_int
        && *env.offset(len as isize) as ::core::ffi::c_int == '=' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_environ_find(
    mut envp: *mut *mut gchar,
    mut variable: *const gchar,
) -> gint {
    let mut len: gsize = 0;
    let mut i: gint = 0;
    if envp.is_null() {
        return -(1 as gint);
    }
    len = strlen(variable as *const ::core::ffi::c_char) as gsize;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*envp.offset(i as isize)).is_null() {
        if safe_c2rust_g_environ_matches(*envp.offset(i as isize), variable, len) != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_environ_getenv(
    mut envp: *mut *mut gchar,
    mut variable: *const gchar,
) -> *const gchar {
    let mut index: gint = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !variable.is_null() {
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
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    index = safe_c2rust_g_environ_find(envp, variable);
    if index != -(1 as ::core::ffi::c_int) {
        return (*envp.offset(index as isize))
            .offset(strlen(variable as *const ::core::ffi::c_char) as isize)
            .offset(1 as ::core::ffi::c_int as isize);
    } else {
        return ::core::ptr::null::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_environ_setenv(
    mut envp: *mut *mut gchar,
    mut variable: *const gchar,
    mut value: *const gchar,
    mut overwrite: gboolean,
) -> *mut *mut gchar {
    let mut index: gint = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !variable.is_null() {
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
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if strchr(variable as *const ::core::ffi::c_char, '=' as i32).is_null() {
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
            b"strchr (variable, '=') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    index = safe_c2rust_g_environ_find(envp, variable);
    if index != -(1 as ::core::ffi::c_int) {
        if overwrite != 0 {
            g_free(*envp.offset(index as isize) as gpointer);
            let ref mut fresh3 = *envp.offset(index as isize);
            *fresh3 = g_strdup_printf(b"%s=%s\0" as *const u8 as *const gchar, variable, value);
        }
    } else {
        let mut length: gint = 0;
        length = (if !envp.is_null() {
            g_strv_length(envp)
        } else {
            0 as guint
        }) as gint;
        envp = ({
            let mut __n: gsize = (length as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = envp as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut *mut gchar;
        let ref mut fresh4 = *envp.offset(length as isize);
        *fresh4 = g_strdup_printf(b"%s=%s\0" as *const u8 as *const gchar, variable, value);
        let ref mut fresh5 =
            *envp.offset((length as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        *fresh5 = ::core::ptr::null_mut::<gchar>();
    }
    return envp;
}
unsafe extern "C" fn safe_c2rust_g_environ_unsetenv_internal(
    mut envp: *mut *mut gchar,
    mut variable: *const gchar,
    mut free_value: gboolean,
) -> *mut *mut gchar {
    let mut len: gsize = 0;
    let mut e: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut f: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    len = strlen(variable as *const ::core::ffi::c_char) as gsize;
    f = envp;
    e = f;
    while !(*e).is_null() {
        if safe_c2rust_g_environ_matches(*e, variable, len) == 0 {
            *f = *e;
            f = f.offset(1);
        } else if free_value != 0 {
            g_free(*e as gpointer);
        }
        e = e.offset(1);
    }
    *f = ::core::ptr::null_mut::<gchar>();
    return envp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_environ_unsetenv(
    mut envp: *mut *mut gchar,
    mut variable: *const gchar,
) -> *mut *mut gchar {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !variable.is_null() {
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
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if strchr(variable as *const ::core::ffi::c_char, '=' as i32).is_null() {
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
            b"strchr (variable, '=') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if envp.is_null() {
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    return safe_c2rust_g_environ_unsetenv_internal(envp, variable, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_getenv(mut variable: *const gchar) -> *const gchar {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !variable.is_null() {
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
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return getenv(variable as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_setenv(
    mut variable: *const gchar,
    mut value: *const gchar,
    mut overwrite: gboolean,
) -> gboolean {
    let mut result: gint = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !variable.is_null() {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if strchr(variable as *const ::core::ffi::c_char, '=' as i32).is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"strchr (variable, '=') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_thread_n_created() > 0 as guint {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"setenv()/putenv() are not thread-safe and should not be used after threads are created\0"
                as *const u8 as *const gchar,
        );
    }
    result = setenv(
        variable as *const ::core::ffi::c_char,
        value as *const ::core::ffi::c_char,
        overwrite as ::core::ffi::c_int,
    ) as gint;
    return (result == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unsetenv(mut variable: *const gchar) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !variable.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if strchr(variable as *const ::core::ffi::c_char, '=' as i32).is_null() {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"strchr (variable, '=') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_thread_n_created() > 0 as guint {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"unsetenv() is not thread-safe and should not be used after threads are created\0"
                as *const u8 as *const gchar,
        );
    }
    unsetenv(variable as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_listenv() -> *mut *mut gchar {
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut eq: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gint = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    len = g_strv_length(safe_c2rust_environ as *mut *mut gchar) as gint;
    result = ({
        let mut __n: gsize = (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut gchar;
    j = 0 as ::core::ffi::c_int as gint;
    i = 0 as ::core::ffi::c_int as gint;
    while i < len {
        eq = strchr(*safe_c2rust_environ.offset(i as isize), '=' as i32) as *mut gchar;
        if !eq.is_null() {
            let fresh0 = j;
            j = j + 1;
            let ref mut fresh1 = *result.offset(fresh0 as isize);
            *fresh1 = g_strndup(
                *safe_c2rust_environ.offset(i as isize),
                eq.offset_from(*safe_c2rust_environ.offset(i as isize)) as ::core::ffi::c_long
                    as gsize,
            );
        }
        i += 1;
    }
    let ref mut fresh2 = *result.offset(j as isize);
    *fresh2 = ::core::ptr::null_mut::<gchar>();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_environ() -> *mut *mut gchar {
    return g_strdupv(safe_c2rust_environ as *mut *mut gchar);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_getenv\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
