extern "C" {
    fn g_ptr_array_new_null_terminated(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
        null_terminated: gboolean,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_build_filenamev(args: *mut *mut gchar) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPathBuf {
    pub dummy: [gpointer; 8],
}
pub type GPathBuf = _GPathBuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RealPathBuf {
    pub path: *mut GPtrArray,
    pub extension: *mut ::core::ffi::c_char,
    pub padding: [gpointer; 6],
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_0 as gpointer;
    return ref_0;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_set_str(
    mut str_pointer: *mut *mut ::core::ffi::c_char,
    mut new_str: *const ::core::ffi::c_char,
) -> gboolean {
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *str_pointer == new_str as *mut ::core::ffi::c_char
        || !(*str_pointer).is_null()
            && !new_str.is_null()
            && strcmp(*str_pointer, new_str) == 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    copy = safe_c2rust_g_strdup_inline(new_str);
    g_free(*str_pointer as gpointer);
    *str_pointer = copy;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_init(mut buf: *mut GPathBuf) -> *mut GPathBuf {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    (*rbuf).path = ::core::ptr::null_mut::<GPtrArray>();
    (*rbuf).extension = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_init_from_path(
    mut buf: *mut GPathBuf,
    mut path: *const ::core::ffi::c_char,
) -> *mut GPathBuf {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPathBuf>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if path.is_null() || *path as ::core::ffi::c_int != '\0' as i32 {
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
            b"path == NULL || *path != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPathBuf>();
    }
    safe_c2rust_g_path_buf_init(buf);
    if path.is_null() {
        return buf;
    } else {
        return safe_c2rust_g_path_buf_push(buf, path);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_clear(mut buf: *mut GPathBuf) {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut _pp: *mut *mut GPtrArray = &raw mut (*rbuf).path;
    let mut _ptr: *mut GPtrArray = *_pp;
    *_pp = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr.is_null() {
        g_ptr_array_unref(_ptr as *mut GPtrArray);
    }
    let mut _pp_0: *mut *mut ::core::ffi::c_char = &raw mut (*rbuf).extension;
    let mut _ptr_0: *mut ::core::ffi::c_char = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_clear_to_path(
    mut buf: *mut GPathBuf,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    res = safe_c2rust_g_path_buf_to_path(buf);
    safe_c2rust_g_path_buf_clear(buf);
    return safe_c2rust_g_steal_pointer(&raw mut res as gpointer) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_new() -> *mut GPathBuf {
    return safe_c2rust_g_path_buf_init(
        ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GPathBuf>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GPathBuf,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_new_from_path(
    mut path: *const ::core::ffi::c_char,
) -> *mut GPathBuf {
    return safe_c2rust_g_path_buf_init_from_path(
        ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GPathBuf>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GPathBuf,
        path,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_free(mut buf: *mut GPathBuf) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_path_buf_clear(buf);
    g_free(buf as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_free_to_path(
    mut buf: *mut GPathBuf,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    res = safe_c2rust_g_path_buf_clear_to_path(buf);
    safe_c2rust_g_path_buf_free(buf);
    return safe_c2rust_g_steal_pointer(&raw mut res as gpointer) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_copy(mut buf: *mut GPathBuf) -> *mut GPathBuf {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    let mut rcopy: *mut RealPathBuf = ::core::ptr::null_mut::<RealPathBuf>();
    let mut copy: *mut GPathBuf = ::core::ptr::null_mut::<GPathBuf>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPathBuf>();
    }
    copy = safe_c2rust_g_path_buf_new();
    rcopy = copy as *mut RealPathBuf;
    if !(*rbuf).path.is_null() {
        (*rcopy).path = g_ptr_array_new_null_terminated(
            (*(*rbuf).path).len,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            TRUE,
        );
        let mut i: guint = 0 as guint;
        while i < (*(*rbuf).path).len {
            let mut p: *const ::core::ffi::c_char =
                *(*(*rbuf).path).pdata.offset(i as isize) as *const ::core::ffi::c_char;
            if !p.is_null() {
                g_ptr_array_add((*rcopy).path, safe_c2rust_g_strdup_inline(p) as gpointer);
            }
            i = i.wrapping_add(1);
        }
    }
    (*rcopy).extension = safe_c2rust_g_strdup_inline((*rbuf).extension);
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_push(
    mut buf: *mut GPathBuf,
    mut path: *const ::core::ffi::c_char,
) -> *mut GPathBuf {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPathBuf>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !path.is_null() && *path as ::core::ffi::c_int != '\0' as i32 {
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
            b"path != NULL && *path != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return buf;
    }
    if g_path_is_absolute(path as *const gchar) != 0 {
        let mut elements: *mut *mut ::core::ffi::c_char = g_strsplit(
            path as *const gchar,
            G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
            -(1 as gint),
        )
            as *mut *mut ::core::ffi::c_char;
        g_free(*elements.offset(0 as ::core::ffi::c_int as isize) as gpointer);
        let ref mut fresh0 = *elements.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char);
        let mut _pp: *mut *mut GPtrArray = &raw mut (*rbuf).path;
        let mut _ptr: *mut GPtrArray = *_pp;
        *_pp = ::core::ptr::null_mut::<GPtrArray>();
        if !_ptr.is_null() {
            g_ptr_array_unref(_ptr as *mut GPtrArray);
        }
        (*rbuf).path = g_ptr_array_new_null_terminated(
            g_strv_length(elements as *mut *mut gchar),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            TRUE,
        );
        let mut i: guint = 0 as guint;
        while !(*elements.offset(i as isize)).is_null() {
            if **elements.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
                g_ptr_array_add(
                    (*rbuf).path,
                    safe_c2rust_g_steal_pointer(elements.offset(i as isize)
                        as *mut *mut ::core::ffi::c_char
                        as gpointer) as *mut ::core::ffi::c_char as gpointer,
                );
            } else {
                g_free(*elements.offset(i as isize) as gpointer);
            }
            i = i.wrapping_add(1);
        }
        g_free(elements as gpointer);
    } else {
        let mut elements_0: *mut *mut ::core::ffi::c_char = g_strsplit(
            path as *const gchar,
            G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
            -(1 as gint),
        )
            as *mut *mut ::core::ffi::c_char;
        if (*rbuf).path.is_null() {
            (*rbuf).path = g_ptr_array_new_null_terminated(
                g_strv_length(elements_0 as *mut *mut gchar),
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                TRUE,
            );
        }
        let mut i_0: guint = 0 as guint;
        while !(*elements_0.offset(i_0 as isize)).is_null() {
            if **elements_0.offset(i_0 as isize) as ::core::ffi::c_int != '\0' as i32 {
                g_ptr_array_add(
                    (*rbuf).path,
                    safe_c2rust_g_steal_pointer(elements_0.offset(i_0 as isize)
                        as *mut *mut ::core::ffi::c_char
                        as gpointer) as *mut ::core::ffi::c_char as gpointer,
                );
            } else {
                g_free(*elements_0.offset(i_0 as isize) as gpointer);
            }
            i_0 = i_0.wrapping_add(1);
        }
        g_free(elements_0 as gpointer);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_pop(mut buf: *mut GPathBuf) -> gboolean {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !(*rbuf).path.is_null() {
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
            b"rbuf->path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*rbuf).path).len > 1 as guint {
        g_ptr_array_remove_index((*rbuf).path, (*(*rbuf).path).len.wrapping_sub(1 as guint));
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_set_filename(
    mut buf: *mut GPathBuf,
    mut file_name: *const ::core::ffi::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(buf as *mut RealPathBuf)).path.is_null() {
        return FALSE;
    }
    safe_c2rust_g_path_buf_pop(buf);
    safe_c2rust_g_path_buf_push(buf, file_name);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_set_extension(
    mut buf: *mut GPathBuf,
    mut extension: *const ::core::ffi::c_char,
) -> gboolean {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !buf.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !(*rbuf).path.is_null() {
        return safe_c2rust_g_set_str(&raw mut (*rbuf).extension, extension);
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_to_path(
    mut buf: *mut GPathBuf,
) -> *mut ::core::ffi::c_char {
    let mut rbuf: *mut RealPathBuf = buf as *mut RealPathBuf;
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !buf.is_null() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*rbuf).path.is_null() {
        path =
            g_build_filenamev((*(*rbuf).path).pdata as *mut *mut gchar) as *mut ::core::ffi::c_char;
    }
    if !path.is_null() && !(*rbuf).extension.is_null() {
        let mut tmp: *mut ::core::ffi::c_char = g_strconcat(
            path,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
            (*rbuf).extension,
            NULL,
        ) as *mut ::core::ffi::c_char;
        g_free(path as gpointer);
        path = safe_c2rust_g_steal_pointer(&raw mut tmp as gpointer) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_buf_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    if v1 == v2 {
        return TRUE;
    }
    let mut p1: *mut ::core::ffi::c_char = safe_c2rust_g_path_buf_to_path(v1 as *mut GPathBuf);
    let mut p2: *mut ::core::ffi::c_char = safe_c2rust_g_path_buf_to_path(v2 as *mut GPathBuf);
    let mut res: gboolean = if !p1.is_null() && !p2.is_null() {
        (strcmp(
            p1 as *const ::core::ffi::c_char,
            p2 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        FALSE
    };
    g_free(p1 as gpointer);
    g_free(p2 as gpointer);
    return res;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_path_buf_push\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
