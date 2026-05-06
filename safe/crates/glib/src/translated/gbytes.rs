extern "C" {
    fn g_byte_array_new_take(data: *mut guint8, len: gsize) -> *mut GByteArray;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn g_free(mem: gpointer);
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_atomic_ref_count_compare(arc: *mut gatomicrefcount, val: gint) -> gboolean;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBytes {
    pub data: gconstpointer,
    pub size: gsize,
    pub ref_count: gatomicrefcount,
    pub free_func: GDestroyNotify,
    pub user_data: gpointer,
}
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_new(
    mut data: gconstpointer,
    mut size: gsize,
) -> *mut GBytes {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !data.is_null() || size == 0 as gsize {
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
            b"data != NULL || size == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    return safe_c2rust_g_bytes_new_take(g_memdup2(data, size), size);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_new_take(
    mut data: gpointer,
    mut size: gsize,
) -> *mut GBytes {
    return safe_c2rust_g_bytes_new_with_free_func(
        data as gconstpointer,
        size,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_new_static(
    mut data: gconstpointer,
    mut size: gsize,
) -> *mut GBytes {
    return safe_c2rust_g_bytes_new_with_free_func(data, size, None, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_new_with_free_func(
    mut data: gconstpointer,
    mut size: gsize,
    mut free_func: GDestroyNotify,
    mut user_data: gpointer,
) -> *mut GBytes {
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !data.is_null() || size == 0 as gsize {
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
            b"data != NULL || size == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    bytes = g_slice_alloc(::core::mem::size_of::<GBytes>() as gsize) as *mut GBytes;
    (*bytes).data = data;
    (*bytes).size = size;
    (*bytes).free_func = free_func;
    (*bytes).user_data = user_data;
    g_atomic_ref_count_init(&raw mut (*bytes).ref_count);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_new_from_bytes(
    mut bytes: *mut GBytes,
    mut offset: gsize,
    mut length: gsize,
) -> *mut GBytes {
    let mut base: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if offset <= (*bytes).size {
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
            b"offset <= bytes->size\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if offset.wrapping_add(length) <= (*bytes).size {
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
            b"offset + length <= bytes->size\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if offset == 0 as gsize && length == (*bytes).size {
        return safe_c2rust_g_bytes_ref(bytes);
    }
    base = ((*bytes).data as *mut gchar).offset(offset as isize);
    while (*bytes).free_func
        == ::core::mem::transmute::<gpointer, GDestroyNotify>(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GBytes) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> (),
        )))
    {
        bytes = (*bytes).user_data as *mut GBytes;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if base >= (*bytes).data as *mut gchar {
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
            b"base >= (gchar *)bytes->data\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if base <= ((*bytes).data as *mut gchar).offset((*bytes).size as isize) {
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
            b"base <= (gchar *)bytes->data + bytes->size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if base.offset(length as isize)
            <= ((*bytes).data as *mut gchar).offset((*bytes).size as isize)
        {
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
            b"base + length <= (gchar *)bytes->data + bytes->size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    return safe_c2rust_g_bytes_new_with_free_func(
        base as gconstpointer,
        length,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
        ),
        safe_c2rust_g_bytes_ref(bytes) as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_get_data(
    mut bytes: *mut GBytes,
    mut size: *mut gsize,
) -> gconstpointer {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    if !size.is_null() {
        *size = (*bytes).size;
    }
    return (*bytes).data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_get_size(mut bytes: *mut GBytes) -> gsize {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*bytes).size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_ref(mut bytes: *mut GBytes) -> *mut GBytes {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    g_atomic_ref_count_inc(&raw mut (*bytes).ref_count);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_unref(mut bytes: *mut GBytes) {
    if bytes.is_null() {
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*bytes).ref_count) != 0 {
        if (*bytes).free_func.is_some() {
            (*bytes).free_func.expect("non-null function pointer")((*bytes).user_data);
        }
        g_slice_free1(::core::mem::size_of::<GBytes>() as gsize, bytes as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_equal(
    mut bytes1: gconstpointer,
    mut bytes2: gconstpointer,
) -> gboolean {
    let mut b1: *const GBytes = bytes1 as *const GBytes;
    let mut b2: *const GBytes = bytes2 as *const GBytes;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !bytes1.is_null() {
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
            b"bytes1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !bytes2.is_null() {
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
            b"bytes2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*b1).size == (*b2).size
        && ((*b1).size == 0 as gsize
            || memcmp(
                (*b1).data as *const ::core::ffi::c_void,
                (*b2).data as *const ::core::ffi::c_void,
                (*b1).size as size_t,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_hash(mut bytes: gconstpointer) -> guint {
    let mut a: *const GBytes = bytes as *const GBytes;
    let mut p: *const ::core::ffi::c_schar = ::core::ptr::null::<::core::ffi::c_schar>();
    let mut e: *const ::core::ffi::c_schar = ::core::ptr::null::<::core::ffi::c_schar>();
    let mut h: guint32 = 5381 as guint32;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    p = (*a).data as *mut ::core::ffi::c_schar;
    e = ((*a).data as *mut ::core::ffi::c_schar).offset((*a).size as isize);
    while p != e {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_add(h)
            .wrapping_add(*p as guint32);
        p = p.offset(1);
    }
    return h as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_compare(
    mut bytes1: gconstpointer,
    mut bytes2: gconstpointer,
) -> gint {
    let mut b1: *const GBytes = bytes1 as *const GBytes;
    let mut b2: *const GBytes = bytes2 as *const GBytes;
    let mut ret: gint = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !bytes1.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bytes1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !bytes2.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bytes2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    ret = memcmp(
        (*b1).data as *const ::core::ffi::c_void,
        (*b2).data as *const ::core::ffi::c_void,
        if (*b1).size < (*b2).size {
            (*b1).size as size_t
        } else {
            (*b2).size as size_t
        },
    ) as gint;
    if ret == 0 as ::core::ffi::c_int && (*b1).size != (*b2).size {
        ret = (if (*b1).size < (*b2).size {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) as gint;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_try_steal_and_unref(
    mut bytes: *mut GBytes,
    mut free_func: GDestroyNotify,
    mut size: *mut gsize,
) -> gpointer {
    let mut result: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*bytes).free_func != free_func
        || (*bytes).data.is_null()
        || (*bytes).user_data != (*bytes).data as gpointer
    {
        return NULL;
    }
    if g_atomic_ref_count_compare(&raw mut (*bytes).ref_count, 1 as gint) != 0 {
        *size = (*bytes).size;
        result = (*bytes).data as gpointer;
        g_slice_free1(::core::mem::size_of::<GBytes>() as gsize, bytes as gpointer);
        return result;
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_unref_to_data(
    mut bytes: *mut GBytes,
    mut size: *mut gsize,
) -> gpointer {
    let mut result: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !size.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    result = safe_c2rust_try_steal_and_unref(
        bytes,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        size,
    );
    if result.is_null() {
        result = g_memdup2((*bytes).data, (*bytes).size);
        *size = (*bytes).size;
        safe_c2rust_g_bytes_unref(bytes);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_unref_to_array(
    mut bytes: *mut GBytes,
) -> *mut GByteArray {
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut size: gsize = 0;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    data = safe_c2rust_g_bytes_unref_to_data(bytes, &raw mut size);
    return g_byte_array_new_take(data as *mut guint8, size);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_get_region(
    mut bytes: *mut GBytes,
    mut element_size: gsize,
    mut offset: gsize,
    mut n_elements: gsize,
) -> gconstpointer {
    let mut total_size: gsize = 0;
    let mut end_offset: gsize = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if element_size > 0 as gsize {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"element_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    let (fresh0, fresh1) = element_size.overflowing_mul(n_elements);
    *&raw mut total_size = fresh0;
    if fresh1 {
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    let (fresh2, fresh3) = offset.overflowing_add(total_size);
    *&raw mut end_offset = fresh2;
    if fresh3 {
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    if end_offset > (*bytes).size {
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    return ((*bytes).data as *mut guchar).offset(offset as isize) as gconstpointer;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_bytes_new_with_free_func\0" as *const u8 as *const ::core::ffi::c_char;
