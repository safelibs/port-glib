extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn g_quark_init();
    fn g_error_init();
    static mut safe_c2rust_stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDebugKey {
    pub key: *const gchar,
    pub value: guint,
}
pub type GDebugKey = _GDebugKey;
pub type FILE = _IO_FILE;
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
#[no_mangle]
pub static mut safe_c2rust_g_mem_gc_friendly: gboolean = FALSE;
#[no_mangle]
pub static mut safe_c2rust_g_log_msg_prefix: GLogLevelFlags =
    (G_LOG_LEVEL_ERROR as ::core::ffi::c_int
        | G_LOG_LEVEL_WARNING as ::core::ffi::c_int
        | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int
        | G_LOG_LEVEL_DEBUG as ::core::ffi::c_int) as GLogLevelFlags;
#[no_mangle]
pub static mut safe_c2rust_g_log_always_fatal: GLogLevelFlags =
    (G_LOG_FLAG_RECURSION as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int)
        as GLogLevelFlags;
unsafe extern "C" fn safe_c2rust_debug_key_matches(
    mut key: *const gchar,
    mut token: *const gchar,
    mut length: guint,
) -> gboolean {
    while length != 0 {
        let mut k: ::core::ffi::c_char = (if *key as ::core::ffi::c_int == '_' as i32 {
            '-' as i32
        } else {
            ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<gchar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *key as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = safe_c2rust_tolower(*key as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*key as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            })
        }) as ::core::ffi::c_char;
        let mut t: ::core::ffi::c_char = (if *token as ::core::ffi::c_int == '_' as i32 {
            '-' as i32
        } else {
            ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<gchar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *token as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = safe_c2rust_tolower(*token as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*token as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            })
        }) as ::core::ffi::c_char;
        if k as ::core::ffi::c_int != t as ::core::ffi::c_int {
            return FALSE;
        }
        length = length.wrapping_sub(1);
        key = key.offset(1);
        token = token.offset(1);
    }
    return (*key as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_parse_debug_string(
    mut string: *const gchar,
    mut keys: *const GDebugKey,
    mut nkeys: guint,
) -> guint {
    let mut i: guint = 0;
    let mut result: guint = 0 as guint;
    if string.is_null() {
        return 0 as guint;
    }
    if strcasecmp(
        string as *const ::core::ffi::c_char,
        b"help\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        fprintf(
            safe_c2rust_stderr,
            b"Supported debug values:\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as guint;
        while i < nkeys {
            fprintf(
                safe_c2rust_stderr,
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*keys.offset(i as isize)).key,
            );
            i = i.wrapping_add(1);
        }
        fprintf(
            safe_c2rust_stderr,
            b" all help\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        let mut p: *const gchar = string;
        let mut q: *const gchar = ::core::ptr::null::<gchar>();
        let mut invert: gboolean = FALSE;
        while *p != 0 {
            q = strpbrk(
                p as *const ::core::ffi::c_char,
                b":;, \t\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if q.is_null() {
                q = p.offset(strlen(p as *const ::core::ffi::c_char) as isize);
            }
            if safe_c2rust_debug_key_matches(
                b"all\0" as *const u8 as *const gchar,
                p,
                q.offset_from(p) as ::core::ffi::c_long as guint,
            ) != 0
            {
                invert = TRUE as gboolean;
            } else {
                i = 0 as guint;
                while i < nkeys {
                    if safe_c2rust_debug_key_matches(
                        (*keys.offset(i as isize)).key,
                        p,
                        q.offset_from(p) as ::core::ffi::c_long as guint,
                    ) != 0
                    {
                        result |= (*keys.offset(i as isize)).value;
                    }
                    i = i.wrapping_add(1);
                }
            }
            p = q;
            if *p != 0 {
                p = p.offset(1);
            }
        }
        if invert != 0 {
            let mut all_flags: guint = 0 as guint;
            i = 0 as guint;
            while i < nkeys {
                all_flags |= (*keys.offset(i as isize)).value;
                i = i.wrapping_add(1);
            }
            result = all_flags & !result;
        }
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_parse_debug_envvar(
    mut envvar: *const gchar,
    mut keys: *const GDebugKey,
    mut n_keys: gint,
    mut default_value: guint,
) -> guint {
    let mut value: *const gchar = ::core::ptr::null::<gchar>();
    value = getenv(envvar as *const ::core::ffi::c_char);
    if value.is_null() {
        return default_value;
    }
    return safe_c2rust_g_parse_debug_string(value, keys, n_keys as guint);
}
unsafe extern "C" fn safe_c2rust_g_messages_prefixed_init() {
    let keys: [GDebugKey; 6] = [
        _GDebugKey {
            key: b"error\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_ERROR as ::core::ffi::c_int as guint,
        },
        _GDebugKey {
            key: b"critical\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int as guint,
        },
        _GDebugKey {
            key: b"warning\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_WARNING as ::core::ffi::c_int as guint,
        },
        _GDebugKey {
            key: b"message\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int as guint,
        },
        _GDebugKey {
            key: b"info\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_INFO as ::core::ffi::c_int as guint,
        },
        _GDebugKey {
            key: b"debug\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_DEBUG as ::core::ffi::c_int as guint,
        },
    ];
    safe_c2rust_g_log_msg_prefix = safe_c2rust_g_parse_debug_envvar(
        b"G_MESSAGES_PREFIXED\0" as *const u8 as *const gchar,
        &raw const keys as *const GDebugKey,
        (::core::mem::size_of::<[GDebugKey; 6]>() as usize)
            .wrapping_div(::core::mem::size_of::<GDebugKey>() as usize) as gint,
        safe_c2rust_g_log_msg_prefix as guint,
    ) as GLogLevelFlags;
}
unsafe extern "C" fn safe_c2rust_g_debug_init() {
    let keys: [GDebugKey; 3] = [
        _GDebugKey {
            key: b"gc-friendly\0" as *const u8 as *const gchar,
            value: 1 as guint,
        },
        _GDebugKey {
            key: b"fatal-warnings\0" as *const u8 as *const gchar,
            value: (G_LOG_LEVEL_WARNING as ::core::ffi::c_int
                | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int) as guint,
        },
        _GDebugKey {
            key: b"fatal-criticals\0" as *const u8 as *const gchar,
            value: G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int as guint,
        },
    ];
    let mut flags: GLogLevelFlags = 0 as GLogLevelFlags;
    flags = safe_c2rust_g_parse_debug_envvar(
        b"G_DEBUG\0" as *const u8 as *const gchar,
        &raw const keys as *const GDebugKey,
        (::core::mem::size_of::<[GDebugKey; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<GDebugKey>() as usize) as gint,
        0 as guint,
    ) as GLogLevelFlags;
    safe_c2rust_g_log_always_fatal = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        safe_c2rust_g_log_always_fatal as ::core::ffi::c_int
            | flags as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int,
    );
    safe_c2rust_g_mem_gc_friendly =
        (flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_init() {
    static mut safe_c2rust_glib_inited: gboolean = 0;
    if safe_c2rust_glib_inited != 0 {
        return;
    }
    safe_c2rust_glib_inited = TRUE as gboolean;
    safe_c2rust_g_messages_prefixed_init();
    safe_c2rust_g_debug_init();
    g_quark_init();
    g_error_init();
}
