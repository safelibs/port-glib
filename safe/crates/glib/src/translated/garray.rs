use ::c2rust_bitfields;
extern "C" {
    pub type _GBytes;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static mut safe_c2rust_g_mem_gc_friendly: gboolean;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_qsort_with_data(
        pbase: gconstpointer,
        total_elems: gint,
        size: gsize,
        compare_func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
pub type gatomicrefcount = gint;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GRealArray = _GRealArray;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GRealArray {
    pub data: *mut guint8,
    pub len: guint,
    pub elt_capacity: guint,
    pub elt_size: guint,
    #[bitfield(name = "zero_terminated", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "clear", ty = "guint", bits = "1..=1")]
    pub zero_terminated_clear: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ref_count: gatomicrefcount,
    pub clear_func: GDestroyNotify,
}
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
pub type ArrayFreeFlags = ::core::ffi::c_uint;
pub const PRESERVE_WRAPPER: ArrayFreeFlags = 2;
pub const FREE_SEGMENT: ArrayFreeFlags = 1;
pub type GRealPtrArray = _GRealPtrArray;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GRealPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
    pub alloc: guint,
    pub ref_count: gatomicrefcount,
    #[bitfield(name = "null_terminated", ty = "guint8", bits = "0..=0")]
    pub null_terminated: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub element_free_func: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPtrArraySortValuesData {
    pub compare_func: GCompareDataFunc,
    pub user_data: gpointer,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_array_sized_new\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_nearest_pow(mut num: gsize) -> gsize {
    let mut n: gsize = num.wrapping_sub(1 as gsize);
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if num > 0 as gsize
            && num
                <= (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
        {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gutilsprivate.h\0" as *const u8 as *const ::core::ffi::c_char,
            44 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num > 0 && num <= G_MAXSIZE / 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_new(
    mut zero_terminated: gboolean,
    mut clear: gboolean,
    mut elt_size: guint,
) -> *mut GArray {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if elt_size > 0 as guint {
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
            b"elt_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    return safe_c2rust_g_array_sized_new(zero_terminated, clear, elt_size, 0 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_new_take(
    mut data: gpointer,
    mut len: gsize,
    mut clear: gboolean,
    mut element_size: gsize,
) -> *mut GArray {
    let mut rarray: *mut GRealArray = ::core::ptr::null_mut::<GRealArray>();
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
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
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
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
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if element_size
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
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
            b"element_size <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    array = safe_c2rust_g_array_sized_new(FALSE, clear, element_size as guint, 0 as guint);
    rarray = array as *mut GRealArray;
    (*rarray).data = safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut guint8;
    (*rarray).len = len as guint;
    (*rarray).elt_capacity = len as guint;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_new_take_zero_terminated(
    mut data: gpointer,
    mut clear: gboolean,
    mut element_size: gsize,
) -> *mut GArray {
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut len: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if element_size
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
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
            b"element_size <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if !data.is_null() {
        let mut array_data: *mut guint8 = data as *mut guint8;
        let mut i: gsize = 0 as gsize;
        loop {
            let mut element_start: *const guint8 =
                array_data.offset(i.wrapping_mul(element_size) as isize);
            if *element_start as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && memcmp(
                    element_start as *const ::core::ffi::c_void,
                    element_start.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    (element_size as size_t).wrapping_sub(1 as size_t),
                ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            len = len.wrapping_add(1 as gsize);
            i = i.wrapping_add(1);
        }
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
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
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    array = safe_c2rust_g_array_new_take(data, len, clear, element_size);
    let ref mut fresh0 = *(array as *mut GRealArray);
    (*fresh0).set_zero_terminated(TRUE as guint as guint);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_steal(
    mut array: *mut GArray,
    mut len: *mut gsize,
) -> gpointer {
    let mut rarray: *mut GRealArray = ::core::ptr::null_mut::<GRealArray>();
    let mut segment: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    rarray = array as *mut GRealArray;
    segment = (*rarray).data as gpointer;
    if !len.is_null() {
        *len = (*rarray).len as gsize;
    }
    (*rarray).data = ::core::ptr::null_mut::<guint8>();
    (*rarray).len = 0 as guint;
    (*rarray).elt_capacity = 0 as guint;
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_sized_new(
    mut zero_terminated: gboolean,
    mut clear: gboolean,
    mut elt_size: guint,
    mut reserved_size: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = ::core::ptr::null_mut::<GRealArray>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if elt_size > 0 as guint {
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
            b"elt_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    array = g_slice_alloc(::core::mem::size_of::<GRealArray>() as gsize) as *mut GRealArray;
    (*array).data = ::core::ptr::null_mut::<guint8>();
    (*array).len = 0 as guint;
    (*array).elt_capacity = 0 as guint;
    (*array).set_zero_terminated(
        (if zero_terminated != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guint as guint,
    );
    (*array).set_clear(
        (if clear != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guint as guint,
    );
    (*array).elt_size = elt_size;
    (*array).clear_func = None;
    g_atomic_ref_count_init(&raw mut (*array).ref_count);
    if (*array).zero_terminated() as ::core::ffi::c_int != 0 || reserved_size != 0 as guint {
        safe_c2rust_g_array_maybe_expand(array, reserved_size);
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if !(*array).data.is_null() {
                _g_boolean_var_17 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_17 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_17
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
                359 as ::core::ffi::c_int,
                G_STRFUNC,
                b"array->data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*array).zero_terminated() != 0 {
            memset(
                (*array).data.offset(
                    ((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize,
                ) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
            );
        }
    }
    return array as *mut GArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_set_clear_func(
    mut array: *mut GArray,
    mut clear_func: GDestroyNotify,
) {
    let mut rarray: *mut GRealArray = array as *mut GRealArray;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*rarray).clear_func = clear_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_ref(mut array: *mut GArray) -> *mut GArray {
    let mut rarray: *mut GRealArray = array as *mut GRealArray;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    g_atomic_ref_count_inc(&raw mut (*rarray).ref_count);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_unref(mut array: *mut GArray) {
    let mut rarray: *mut GRealArray = array as *mut GRealArray;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*rarray).ref_count) != 0 {
        safe_c2rust_array_free(rarray, FREE_SEGMENT);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_get_element_size(mut array: *mut GArray) -> guint {
    let mut rarray: *mut GRealArray = array as *mut GRealArray;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*rarray).elt_size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_free(
    mut farray: *mut GArray,
    mut free_segment: gboolean,
) -> *mut gchar {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    let mut flags: ArrayFreeFlags = 0 as ArrayFreeFlags;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    flags = (if free_segment != 0 {
        FREE_SEGMENT as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ArrayFreeFlags;
    if g_atomic_ref_count_dec(&raw mut (*array).ref_count) == 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, ArrayFreeFlags>(
            flags as ::core::ffi::c_uint
                | PRESERVE_WRAPPER as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return safe_c2rust_array_free(array, flags);
}
unsafe extern "C" fn safe_c2rust_array_free(
    mut array: *mut GRealArray,
    mut flags: ArrayFreeFlags,
) -> *mut gchar {
    let mut segment: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if flags as ::core::ffi::c_uint & FREE_SEGMENT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        if (*array).clear_func.is_some() {
            let mut i: guint = 0;
            i = 0 as guint;
            while i < (*array).len {
                (*array).clear_func.expect("non-null function pointer")(
                    (*array)
                        .data
                        .offset(((*array).elt_size as gsize).wrapping_mul(i as gsize) as isize)
                        as gpointer,
                );
                i = i.wrapping_add(1);
            }
        }
        g_free((*array).data as gpointer);
        segment = ::core::ptr::null_mut::<gchar>();
    } else {
        segment = (*array).data as *mut gchar;
    }
    if flags as ::core::ffi::c_uint & PRESERVE_WRAPPER as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*array).data = ::core::ptr::null_mut::<guint8>();
        (*array).len = 0 as guint;
        (*array).elt_capacity = 0 as guint;
    } else {
        g_slice_free1(
            ::core::mem::size_of::<GRealArray>() as gsize,
            array as gpointer,
        );
    }
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_append_vals(
    mut farray: *mut GArray,
    mut data: gconstpointer,
    mut len: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if len == 0 as guint {
        return farray;
    }
    safe_c2rust_g_array_maybe_expand(array, len);
    memcpy(
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
            as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        ((*array).elt_size as size_t).wrapping_mul(len as size_t),
    );
    (*array).len = (*array).len.wrapping_add(len);
    if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_prepend_vals(
    mut farray: *mut GArray,
    mut data: gconstpointer,
    mut len: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if len == 0 as guint {
        return farray;
    }
    safe_c2rust_g_array_maybe_expand(array, len);
    memmove(
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul(len as gsize) as isize)
            as *mut ::core::ffi::c_void,
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul(0 as gsize) as isize)
            as *const ::core::ffi::c_void,
        ((*array).elt_size as size_t).wrapping_mul((*array).len as size_t),
    );
    memcpy(
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul(0 as gsize) as isize)
            as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        ((*array).elt_size as size_t).wrapping_mul(len as size_t),
    );
    (*array).len = (*array).len.wrapping_add(len);
    if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_insert_vals(
    mut farray: *mut GArray,
    mut index_: guint,
    mut data: gconstpointer,
    mut len: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if len == 0 as guint {
        return farray;
    }
    if index_ >= (*array).len {
        safe_c2rust_g_array_maybe_expand(
            array,
            index_.wrapping_sub((*array).len).wrapping_add(len),
        );
        return safe_c2rust_g_array_append_vals(
            safe_c2rust_g_array_set_size(farray, index_),
            data,
            len,
        );
    }
    safe_c2rust_g_array_maybe_expand(array, len);
    memmove(
        (*array).data.offset(
            ((*array).elt_size as gsize).wrapping_mul(len.wrapping_add(index_) as gsize) as isize,
        ) as *mut ::core::ffi::c_void,
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
            as *const ::core::ffi::c_void,
        ((*array).elt_size as size_t).wrapping_mul((*array).len.wrapping_sub(index_) as size_t),
    );
    memcpy(
        (*array)
            .data
            .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
            as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        ((*array).elt_size as size_t).wrapping_mul(len as size_t),
    );
    (*array).len = (*array).len.wrapping_add(len);
    if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_set_size(
    mut farray: *mut GArray,
    mut length: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if length > (*array).len {
        safe_c2rust_g_array_maybe_expand(array, length.wrapping_sub((*array).len));
        if (*array).clear() != 0 {
            memset(
                (*array).data.offset(
                    ((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize,
                ) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*array).elt_size as size_t)
                    .wrapping_mul(length.wrapping_sub((*array).len) as size_t),
            );
        }
    } else if length < (*array).len {
        safe_c2rust_g_array_remove_range(farray, length, (*array).len.wrapping_sub(length));
    }
    (*array).len = length;
    if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_remove_index(
    mut farray: *mut GArray,
    mut index_: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !array.is_null() {
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
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if index_ < (*array).len {
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
            b"index_ < array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if (*array).clear_func.is_some() {
        (*array).clear_func.expect("non-null function pointer")(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
                as gpointer,
        );
    }
    if index_ != (*array).len.wrapping_sub(1 as guint) {
        memmove(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
                as *mut ::core::ffi::c_void,
            (*array).data.offset(
                ((*array).elt_size as gsize).wrapping_mul(index_.wrapping_add(1 as guint) as gsize)
                    as isize,
            ) as *const ::core::ffi::c_void,
            ((*array).elt_size as size_t)
                .wrapping_mul((*array).len.wrapping_sub(index_).wrapping_sub(1 as guint) as size_t),
        );
    }
    (*array).len = (*array).len.wrapping_sub(1 as guint);
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_g_mem_gc_friendly != 0 {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    } else if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_remove_index_fast(
    mut farray: *mut GArray,
    mut index_: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if index_ < (*array).len {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ < array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if (*array).clear_func.is_some() {
        (*array).clear_func.expect("non-null function pointer")(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
                as gpointer,
        );
    }
    if index_ != (*array).len.wrapping_sub(1 as guint) {
        memcpy(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
                as *mut ::core::ffi::c_void,
            (*array).data.offset(
                ((*array).elt_size as gsize)
                    .wrapping_mul((*array).len.wrapping_sub(1 as guint) as gsize)
                    as isize,
            ) as *const ::core::ffi::c_void,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    (*array).len = (*array).len.wrapping_sub(1 as guint);
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_g_mem_gc_friendly != 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    } else if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_remove_range(
    mut farray: *mut GArray,
    mut index_: guint,
    mut length: guint,
) -> *mut GArray {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if index_ <= (*array).len {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if index_
            <= (2147483647 as ::core::ffi::c_int as guint)
                .wrapping_mul(2 as guint)
                .wrapping_add(1 as guint)
                .wrapping_sub(length)
        {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= G_MAXUINT - length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if index_.wrapping_add(length) <= (*array).len {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ + length <= array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    if (*array).clear_func.is_some() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < length {
            (*array).clear_func.expect("non-null function pointer")((*array).data.offset(
                ((*array).elt_size as gsize).wrapping_mul(index_.wrapping_add(i) as gsize) as isize,
            ) as gpointer);
            i = i.wrapping_add(1);
        }
    }
    if index_.wrapping_add(length) != (*array).len {
        memmove(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul(index_ as gsize) as isize)
                as *mut ::core::ffi::c_void,
            (*array).data.offset(
                ((*array).elt_size as gsize).wrapping_mul(index_.wrapping_add(length) as gsize)
                    as isize,
            ) as *const ::core::ffi::c_void,
            (*array)
                .len
                .wrapping_sub(index_.wrapping_add(length))
                .wrapping_mul((*array).elt_size) as size_t,
        );
    }
    (*array).len = (*array).len.wrapping_sub(length);
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if safe_c2rust_g_mem_gc_friendly != 0 {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(length as size_t),
        );
    } else if (*array).zero_terminated() != 0 {
        memset(
            (*array)
                .data
                .offset(((*array).elt_size as gsize).wrapping_mul((*array).len as gsize) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*array).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return farray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_sort(
    mut farray: *mut GArray,
    mut compare_func: GCompareFunc,
) {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*array).len > 0 as guint {
        g_qsort_with_data(
            (*array).data as gconstpointer,
            (*array).len as gint,
            (*array).elt_size as gsize,
            ::core::mem::transmute::<GCompareFunc, GCompareDataFunc>(compare_func),
            NULL,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_sort_with_data(
    mut farray: *mut GArray,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut array: *mut GRealArray = farray as *mut GRealArray;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*array).len > 0 as guint {
        g_qsort_with_data(
            (*array).data as gconstpointer,
            (*array).len as gint,
            (*array).elt_size as gsize,
            compare_func,
            user_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_binary_search(
    mut array: *mut GArray,
    mut target: gconstpointer,
    mut compare_func: GCompareFunc,
    mut out_match_index: *mut guint,
) -> gboolean {
    let mut result: gboolean = FALSE;
    let mut _array: *mut GRealArray = array as *mut GRealArray;
    let mut left: guint = 0;
    let mut middle: guint = 0 as guint;
    let mut right: guint = 0;
    let mut val: gint = 0;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !_array.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if compare_func.is_some() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if (*_array).len != 0 {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
        left = 0 as guint;
        right = (*_array).len.wrapping_sub(1 as guint);
        while left <= right {
            middle = left.wrapping_add(right.wrapping_sub(left).wrapping_div(2 as guint));
            val = compare_func.expect("non-null function pointer")(
                (*_array)
                    .data
                    .offset((*_array).elt_size.wrapping_mul(middle) as isize)
                    as gconstpointer,
                target,
            );
            if val == 0 as ::core::ffi::c_int {
                result = TRUE as gboolean;
                break;
            } else if val < 0 as ::core::ffi::c_int {
                left = middle.wrapping_add(1 as guint);
            } else {
                if !(middle > 0 as guint) {
                    break;
                }
                right = middle.wrapping_sub(1 as guint);
            }
        }
    }
    if result != 0 && !out_match_index.is_null() {
        *out_match_index = middle;
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_array_maybe_expand(mut array: *mut GRealArray, mut len: guint) {
    let mut max_len: guint = 0;
    let mut want_len: guint = 0;
    max_len = (if (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
        .wrapping_mul(2 as ::core::ffi::c_ulong)
        .wrapping_add(1 as ::core::ffi::c_ulong)
        .wrapping_div(2 as ::core::ffi::c_ulong)
        .wrapping_div((*array).elt_size as ::core::ffi::c_ulong)
        < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_ulong
    {
        (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
            .wrapping_mul(2 as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong)
            .wrapping_div(2 as ::core::ffi::c_ulong)
            .wrapping_div((*array).elt_size as ::core::ffi::c_ulong)
    } else {
        (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_ulong
    })
    .wrapping_sub((*array).zero_terminated() as ::core::ffi::c_ulong) as guint;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if max_len.wrapping_sub((*array).len) < len {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"adding %u to array would overflow\0" as *const u8 as *const gchar,
            len,
        );
        loop {}
    }
    want_len = (*array)
        .len
        .wrapping_add(len)
        .wrapping_add((*array).zero_terminated());
    if want_len > (*array).elt_capacity {
        let mut want_alloc: gsize =
            safe_c2rust_g_nearest_pow(((*array).elt_size as gsize).wrapping_mul(want_len as gsize));
        want_alloc = if want_alloc > 16 as gsize {
            want_alloc
        } else {
            16 as gsize
        };
        (*array).data = g_realloc((*array).data as gpointer, want_alloc) as *mut guint8;
        if ({
            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
            if safe_c2rust_g_mem_gc_friendly != 0 {
                _g_boolean_var_44 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_44 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_44
        }) as ::core::ffi::c_long
            != 0
        {
            memset(
                (*array).data.offset(
                    ((*array).elt_size as gsize).wrapping_mul((*array).elt_capacity as gsize)
                        as isize,
                ) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*array).elt_size as size_t)
                    .wrapping_mul(want_len.wrapping_sub((*array).elt_capacity) as size_t),
            );
        }
        (*array).elt_capacity = (if want_alloc.wrapping_div((*array).elt_size as gsize)
            < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            want_alloc.wrapping_div((*array).elt_size as gsize)
        } else {
            (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        }) as guint;
    }
}
unsafe extern "C" fn safe_c2rust_ptr_array_maybe_null_terminate(mut rarray: *mut GRealPtrArray) {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if (*rarray).null_terminated() != 0 {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
        let ref mut fresh6 = *(*rarray).pdata.offset((*rarray).len as isize);
        *fresh6 = NULL as gpointer;
    }
}
unsafe extern "C" fn safe_c2rust_ptr_array_new(
    mut reserved_size: guint,
    mut element_free_func: GDestroyNotify,
    mut null_terminated: gboolean,
) -> *mut GPtrArray {
    let mut array: *mut GRealPtrArray = ::core::ptr::null_mut::<GRealPtrArray>();
    array = g_slice_alloc(::core::mem::size_of::<GRealPtrArray>() as gsize) as *mut GRealPtrArray;
    (*array).pdata = ::core::ptr::null_mut::<gpointer>();
    (*array).len = 0 as guint;
    (*array).alloc = 0 as guint;
    (*array).set_null_terminated(
        (if null_terminated != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guint8 as guint8,
    );
    (*array).element_free_func = element_free_func;
    g_atomic_ref_count_init(&raw mut (*array).ref_count);
    if reserved_size != 0 as guint {
        if ({
            let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
            if reserved_size
                < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
            {
                _g_boolean_var_46 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_46 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_46
        }) as ::core::ffi::c_long
            != 0
            && null_terminated != 0
        {
            reserved_size = reserved_size.wrapping_add(1);
        }
        safe_c2rust_g_ptr_array_maybe_expand(array, reserved_size);
        if ({
            let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
            if !(*array).pdata.is_null() {
                _g_boolean_var_47 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_47 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_47
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
                1150 as ::core::ffi::c_int,
                G_STRFUNC,
                b"array->pdata != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if null_terminated != 0 {
            let ref mut fresh1 = *(*array).pdata.offset(0 as ::core::ffi::c_int as isize);
            *fresh1 = NULL as gpointer;
        }
    }
    return array as *mut GPtrArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new() -> *mut GPtrArray {
    return safe_c2rust_ptr_array_new(0 as guint, None, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_take(
    mut data: *mut gpointer,
    mut len: gsize,
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut rarray: *mut GRealPtrArray = ::core::ptr::null_mut::<GRealPtrArray>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = safe_c2rust_ptr_array_new(0 as guint, element_free_func, FALSE);
    rarray = array as *mut GRealPtrArray;
    (*rarray).pdata =
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut gpointer as *mut gpointer;
    (*rarray).len = len as guint;
    (*rarray).alloc = len as guint;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_take_null_terminated(
    mut data: *mut gpointer,
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut len: gsize = 0 as gsize;
    if !data.is_null() {
        let mut i: gsize = 0 as gsize;
        while !(*data.offset(i as isize)).is_null() {
            len = len.wrapping_add(1 as gsize);
            i = i.wrapping_add(1);
        }
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = safe_c2rust_g_ptr_array_new_take(
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut gpointer,
        len,
        element_free_func,
    );
    let ref mut fresh7 = *(array as *mut GRealPtrArray);
    (*fresh7).set_null_terminated(TRUE as guint8 as guint8);
    return array;
}
unsafe extern "C" fn safe_c2rust_ptr_array_new_from_array(
    mut data: *mut gpointer,
    mut len: gsize,
    mut copy_func: GCopyFunc,
    mut copy_func_user_data: gpointer,
    mut element_free_func: GDestroyNotify,
    mut null_terminated: gboolean,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut rarray: *mut GRealPtrArray = ::core::ptr::null_mut::<GRealPtrArray>();
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
            1288 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
            1289 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    array = safe_c2rust_ptr_array_new(len as guint, element_free_func, null_terminated);
    rarray = array as *mut GRealPtrArray;
    if copy_func.is_some() {
        let mut i: gsize = 0 as gsize;
        while i < len {
            let ref mut fresh3 = *(*rarray).pdata.offset(i as isize);
            *fresh3 = copy_func.expect("non-null function pointer")(
                *data.offset(i as isize) as gconstpointer,
                copy_func_user_data,
            );
            i = i.wrapping_add(1);
        }
    } else if len != 0 as gsize {
        memcpy(
            (*rarray).pdata as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            (len as size_t).wrapping_mul(::core::mem::size_of::<gpointer>() as size_t),
        );
    }
    if null_terminated != 0 && !(*rarray).pdata.is_null() {
        let ref mut fresh4 = *(*rarray).pdata.offset(len as isize);
        *fresh4 = NULL as gpointer;
    }
    (*rarray).len = len as guint;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_from_array(
    mut data: *mut gpointer,
    mut len: gsize,
    mut copy_func: GCopyFunc,
    mut copy_func_user_data: gpointer,
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    return safe_c2rust_ptr_array_new_from_array(
        data,
        len,
        copy_func,
        copy_func_user_data,
        element_free_func,
        FALSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_from_null_terminated_array(
    mut data: *mut gpointer,
    mut copy_func: GCopyFunc,
    mut copy_func_user_data: gpointer,
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    let mut len: gsize = 0 as gsize;
    if !data.is_null() {
        let mut i: gsize = 0 as gsize;
        while !(*data.offset(i as isize)).is_null() {
            len = len.wrapping_add(1 as gsize);
            i = i.wrapping_add(1);
        }
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
            1399 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    return safe_c2rust_ptr_array_new_from_array(
        data,
        len,
        copy_func,
        copy_func_user_data,
        element_free_func,
        TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_steal(
    mut array: *mut GPtrArray,
    mut len: *mut gsize,
) -> *mut gpointer {
    let mut rarray: *mut GRealPtrArray = ::core::ptr::null_mut::<GRealPtrArray>();
    let mut segment: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gpointer>();
    }
    rarray = array as *mut GRealPtrArray;
    segment = (*rarray).pdata;
    if !len.is_null() {
        *len = (*rarray).len as gsize;
    }
    (*rarray).pdata = ::core::ptr::null_mut::<gpointer>();
    (*rarray).len = 0 as guint;
    (*rarray).alloc = 0 as guint;
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_copy(
    mut array: *mut GPtrArray,
    mut func: GCopyFunc,
    mut user_data: gpointer,
) -> *mut GPtrArray {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut new_array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    new_array = safe_c2rust_ptr_array_new(
        0 as guint,
        (*rarray).element_free_func,
        (*rarray).null_terminated() as gboolean,
    );
    if (*rarray).alloc > 0 as guint {
        safe_c2rust_g_ptr_array_maybe_expand(
            new_array as *mut GRealPtrArray,
            (*array)
                .len
                .wrapping_add((*rarray).null_terminated() as guint),
        );
        if (*array).len > 0 as guint {
            if func.is_some() {
                let mut i: guint = 0;
                i = 0 as guint;
                while i < (*array).len {
                    let ref mut fresh5 = *(*new_array).pdata.offset(i as isize);
                    *fresh5 = func.expect("non-null function pointer")(
                        *(*array).pdata.offset(i as isize) as gconstpointer,
                        user_data,
                    );
                    i = i.wrapping_add(1);
                }
            } else {
                memcpy(
                    (*new_array).pdata as *mut ::core::ffi::c_void,
                    (*array).pdata as *const ::core::ffi::c_void,
                    ((*array).len as size_t)
                        .wrapping_mul(::core::mem::size_of::<gpointer>() as size_t),
                );
            }
            (*new_array).len = (*array).len;
        }
        safe_c2rust_ptr_array_maybe_null_terminate(new_array as *mut GRealPtrArray);
    }
    return new_array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_sized_new(
    mut reserved_size: guint,
) -> *mut GPtrArray {
    return safe_c2rust_ptr_array_new(reserved_size, None, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_array_copy(mut array: *mut GArray) -> *mut GArray {
    let mut rarray: *mut GRealArray = array as *mut GRealArray;
    let mut new_rarray: *mut GRealArray = ::core::ptr::null_mut::<GRealArray>();
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GArray>();
    }
    new_rarray = safe_c2rust_g_array_sized_new(
        (*rarray).zero_terminated() as gboolean,
        (*rarray).clear() as gboolean,
        (*rarray).elt_size,
        (*rarray).elt_capacity,
    ) as *mut GRealArray;
    (*new_rarray).len = (*rarray).len;
    if (*rarray).len > 0 as guint {
        memcpy(
            (*new_rarray).data as *mut ::core::ffi::c_void,
            (*rarray).data as *const ::core::ffi::c_void,
            (*rarray).len.wrapping_mul((*rarray).elt_size) as size_t,
        );
    }
    if (*new_rarray).zero_terminated() != 0 {
        memset(
            (*new_rarray).data.offset(
                ((*new_rarray).elt_size as gsize).wrapping_mul((*new_rarray).len as gsize) as isize,
            ) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*new_rarray).elt_size as size_t).wrapping_mul(1 as size_t),
        );
    }
    return new_rarray as *mut GArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_with_free_func(
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    return safe_c2rust_ptr_array_new(0 as guint, element_free_func, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_full(
    mut reserved_size: guint,
    mut element_free_func: GDestroyNotify,
) -> *mut GPtrArray {
    return safe_c2rust_ptr_array_new(reserved_size, element_free_func, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_new_null_terminated(
    mut reserved_size: guint,
    mut element_free_func: GDestroyNotify,
    mut null_terminated: gboolean,
) -> *mut GPtrArray {
    return safe_c2rust_ptr_array_new(reserved_size, element_free_func, null_terminated);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_set_free_func(
    mut array: *mut GPtrArray,
    mut element_free_func: GDestroyNotify,
) {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*rarray).element_free_func = element_free_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_is_null_terminated(
    mut array: *mut GPtrArray,
) -> gboolean {
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(array as *mut GRealPtrArray)).null_terminated() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_ref(mut array: *mut GPtrArray) -> *mut GPtrArray {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    g_atomic_ref_count_inc(&raw mut (*rarray).ref_count);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_unref(mut array: *mut GPtrArray) {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*rarray).ref_count) != 0 {
        safe_c2rust_ptr_array_free(array, FREE_SEGMENT);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_free(
    mut array: *mut GPtrArray,
    mut free_segment: gboolean,
) -> *mut gpointer {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut flags: ArrayFreeFlags = 0 as ArrayFreeFlags;
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gpointer>();
    }
    flags = (if free_segment != 0 {
        FREE_SEGMENT as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ArrayFreeFlags;
    if g_atomic_ref_count_dec(&raw mut (*rarray).ref_count) == 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, ArrayFreeFlags>(
            flags as ::core::ffi::c_uint
                | PRESERVE_WRAPPER as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return safe_c2rust_ptr_array_free(array, flags);
}
unsafe extern "C" fn safe_c2rust_ptr_array_free(
    mut array: *mut GPtrArray,
    mut flags: ArrayFreeFlags,
) -> *mut gpointer {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut segment: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    if flags as ::core::ffi::c_uint & FREE_SEGMENT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        let mut stolen_pdata: *mut gpointer =
            safe_c2rust_g_steal_pointer(&raw mut (*rarray).pdata as gpointer) as *mut gpointer;
        if (*rarray).element_free_func.is_some() {
            let mut i: guint = 0;
            i = 0 as guint;
            while i < (*rarray).len {
                (*rarray)
                    .element_free_func
                    .expect("non-null function pointer")(
                    *stolen_pdata.offset(i as isize)
                );
                i = i.wrapping_add(1);
            }
        }
        g_free(stolen_pdata as gpointer);
        segment = ::core::ptr::null_mut::<gpointer>();
    } else {
        segment = (*rarray).pdata;
        if segment.is_null() && (*rarray).null_terminated() as ::core::ffi::c_int != 0 {
            segment = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut *mut ::core::ffi::c_char as *mut gpointer;
        }
    }
    if flags as ::core::ffi::c_uint & PRESERVE_WRAPPER as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*rarray).pdata = ::core::ptr::null_mut::<gpointer>();
        (*rarray).len = 0 as guint;
        (*rarray).alloc = 0 as guint;
    } else {
        g_slice_free1(
            ::core::mem::size_of::<GRealPtrArray>() as gsize,
            rarray as gpointer,
        );
    }
    return segment;
}
unsafe extern "C" fn safe_c2rust_g_ptr_array_maybe_expand(
    mut array: *mut GRealPtrArray,
    mut len: guint,
) {
    let mut max_len: guint = 0;
    max_len = (if (9223372036854775807 as ::core::ffi::c_long as usize)
        .wrapping_mul(2 as usize)
        .wrapping_add(1 as usize)
        .wrapping_div(2 as usize)
        .wrapping_div(::core::mem::size_of::<gpointer>() as usize)
        < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize
    {
        (9223372036854775807 as ::core::ffi::c_long as usize)
            .wrapping_mul(2 as usize)
            .wrapping_add(1 as usize)
            .wrapping_div(2 as usize)
            .wrapping_div(::core::mem::size_of::<gpointer>() as usize)
    } else {
        (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize
    }) as guint;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if max_len.wrapping_sub((*array).len) < len {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"adding %u to array would overflow\0" as *const u8 as *const gchar,
            len,
        );
        loop {}
    }
    if (*array).len.wrapping_add(len) > (*array).alloc {
        let mut old_alloc: guint = (*array).alloc;
        let mut want_alloc: gsize = safe_c2rust_g_nearest_pow(
            (::core::mem::size_of::<gpointer>() as gsize)
                .wrapping_mul((*array).len.wrapping_add(len) as gsize),
        );
        want_alloc = if want_alloc > 16 as gsize {
            want_alloc
        } else {
            16 as gsize
        };
        (*array).alloc = (if (want_alloc as usize)
            .wrapping_div(::core::mem::size_of::<gpointer>() as usize)
            < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as usize
        {
            (want_alloc as usize).wrapping_div(::core::mem::size_of::<gpointer>() as usize)
        } else {
            (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as usize
        }) as guint;
        (*array).pdata = g_realloc((*array).pdata as gpointer, want_alloc) as *mut gpointer;
        if ({
            let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
            if safe_c2rust_g_mem_gc_friendly != 0 {
                _g_boolean_var_66 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_66 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_66
        }) as ::core::ffi::c_long
            != 0
        {
            while old_alloc < (*array).alloc {
                let ref mut fresh2 = *(*array).pdata.offset(old_alloc as isize);
                *fresh2 = NULL as gpointer;
                old_alloc = old_alloc.wrapping_add(1);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_set_size(
    mut array: *mut GPtrArray,
    mut length: gint,
) {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut length_unsigned: guint = 0;
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if (*rarray).len == 0 as guint || (*rarray).len != 0 as guint && !(*rarray).pdata.is_null()
        {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray->len == 0 || (rarray->len != 0 && rarray->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if length >= 0 as ::core::ffi::c_int {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    length_unsigned = length as guint;
    if length_unsigned > (*rarray).len {
        let mut i: guint = 0;
        if ({
            let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
            if (*rarray).null_terminated() != 0 {
                _g_boolean_var_70 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_70 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_70
        }) as ::core::ffi::c_long
            != 0
            && length_unsigned.wrapping_sub((*rarray).len)
                > G_MAXUINT.wrapping_sub(1 as ::core::ffi::c_uint)
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"array would overflow\0" as *const u8 as *const gchar,
            );
            loop {}
        }
        safe_c2rust_g_ptr_array_maybe_expand(
            rarray,
            length_unsigned
                .wrapping_sub((*rarray).len)
                .wrapping_add((*rarray).null_terminated() as guint),
        );
        i = (*rarray).len;
        while i < length_unsigned {
            let ref mut fresh8 = *(*rarray).pdata.offset(i as isize);
            *fresh8 = NULL as gpointer;
            i = i.wrapping_add(1);
        }
        (*rarray).len = length_unsigned;
        safe_c2rust_ptr_array_maybe_null_terminate(rarray);
    } else if length_unsigned < (*rarray).len {
        safe_c2rust_g_ptr_array_remove_range(
            array,
            length_unsigned,
            (*rarray).len.wrapping_sub(length_unsigned),
        );
    }
}
unsafe extern "C" fn safe_c2rust_ptr_array_remove_index(
    mut array: *mut GPtrArray,
    mut index_: guint,
    mut fast: gboolean,
    mut free_element: gboolean,
) -> gpointer {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut result: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if (*rarray).len == 0 as guint || (*rarray).len != 0 as guint && !(*rarray).pdata.is_null()
        {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray->len == 0 || (rarray->len != 0 && rarray->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if index_ < (*rarray).len {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ < rarray->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    result = *(*rarray).pdata.offset(index_ as isize);
    if (*rarray).element_free_func.is_some() && free_element != 0 {
        (*rarray)
            .element_free_func
            .expect("non-null function pointer")(*(*rarray).pdata.offset(index_ as isize));
    }
    if index_ != (*rarray).len.wrapping_sub(1 as guint) && fast == 0 {
        memmove(
            (*rarray).pdata.offset(index_ as isize) as *mut ::core::ffi::c_void,
            (*rarray)
                .pdata
                .offset(index_ as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<gpointer>() as size_t)
                .wrapping_mul(
                    (*rarray).len.wrapping_sub(index_).wrapping_sub(1 as guint) as size_t,
                ),
        );
    } else if index_ != (*rarray).len.wrapping_sub(1 as guint) {
        let ref mut fresh10 = *(*rarray).pdata.offset(index_ as isize);
        *fresh10 = *(*rarray)
            .pdata
            .offset((*rarray).len.wrapping_sub(1 as guint) as isize);
    }
    (*rarray).len = (*rarray).len.wrapping_sub(1 as guint);
    if (*rarray).null_terminated() as ::core::ffi::c_int != 0
        || ({
            let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
            if safe_c2rust_g_mem_gc_friendly != 0 {
                _g_boolean_var_74 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_74 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_74
        }) as ::core::ffi::c_long
            != 0
    {
        let ref mut fresh11 = *(*rarray).pdata.offset((*rarray).len as isize);
        *fresh11 = NULL as gpointer;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_remove_index(
    mut array: *mut GPtrArray,
    mut index_: guint,
) -> gpointer {
    return safe_c2rust_ptr_array_remove_index(array, index_, FALSE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_remove_index_fast(
    mut array: *mut GPtrArray,
    mut index_: guint,
) -> gpointer {
    return safe_c2rust_ptr_array_remove_index(array, index_, TRUE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_steal_index(
    mut array: *mut GPtrArray,
    mut index_: guint,
) -> gpointer {
    return safe_c2rust_ptr_array_remove_index(array, index_, FALSE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_steal_index_fast(
    mut array: *mut GPtrArray,
    mut index_: guint,
) -> gpointer {
    return safe_c2rust_ptr_array_remove_index(array, index_, TRUE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_remove_range(
    mut array: *mut GPtrArray,
    mut index_: guint,
    mut length: guint,
) -> *mut GPtrArray {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if (*rarray).len == 0 as guint || (*rarray).len != 0 as guint && !(*rarray).pdata.is_null()
        {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray->len == 0 || (rarray->len != 0 && rarray->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if index_ <= (*rarray).len {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= rarray->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if index_
            <= (2147483647 as ::core::ffi::c_int as guint)
                .wrapping_mul(2 as guint)
                .wrapping_add(1 as guint)
                .wrapping_sub(length)
        {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= G_MAXUINT - length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if length == 0 as guint || index_.wrapping_add(length) <= (*rarray).len {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || index_ + length <= rarray->len\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    if length == 0 as guint {
        return array;
    }
    if (*rarray).element_free_func.is_some() {
        i = index_;
        while i < index_.wrapping_add(length) {
            (*rarray)
                .element_free_func
                .expect("non-null function pointer")(
                *(*rarray).pdata.offset(i as isize)
            );
            i = i.wrapping_add(1);
        }
    }
    if index_.wrapping_add(length) != (*rarray).len {
        memmove(
            (*rarray).pdata.offset(index_ as isize) as *mut gpointer as *mut ::core::ffi::c_void,
            (*rarray).pdata.offset(index_.wrapping_add(length) as isize) as *mut gpointer
                as *const ::core::ffi::c_void,
            ((*rarray).len.wrapping_sub(index_.wrapping_add(length)) as size_t)
                .wrapping_mul(::core::mem::size_of::<gpointer>() as size_t),
        );
    }
    (*rarray).len = (*rarray).len.wrapping_sub(length);
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if safe_c2rust_g_mem_gc_friendly != 0 {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
        i = 0 as guint;
        while i < length {
            let ref mut fresh9 = *(*rarray)
                .pdata
                .offset((*rarray).len.wrapping_add(i) as isize);
            *fresh9 = NULL as gpointer;
            i = i.wrapping_add(1);
        }
    } else {
        safe_c2rust_ptr_array_maybe_null_terminate(rarray);
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_remove(
    mut array: *mut GPtrArray,
    mut data: gpointer,
) -> gboolean {
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if (*array).len == 0 as guint || (*array).len != 0 as guint && !(*array).pdata.is_null() {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array->len == 0 || (array->len != 0 && array->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 0 as guint;
    while i < (*array).len {
        if *(*array).pdata.offset(i as isize) == data {
            safe_c2rust_g_ptr_array_remove_index(array, i);
            return TRUE;
        }
        i = i.wrapping_add(1 as guint);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_remove_fast(
    mut array: *mut GPtrArray,
    mut data: gpointer,
) -> gboolean {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if (*rarray).len == 0 as guint || (*rarray).len != 0 as guint && !(*rarray).pdata.is_null()
        {
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray->len == 0 || (rarray->len != 0 && rarray->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 0 as guint;
    while i < (*rarray).len {
        if *(*rarray).pdata.offset(i as isize) == data {
            safe_c2rust_g_ptr_array_remove_index_fast(array, i);
            return TRUE;
        }
        i = i.wrapping_add(1 as guint);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_add(
    mut array: *mut GPtrArray,
    mut data: gpointer,
) {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if (*rarray).len == 0 as guint || (*rarray).len != 0 as guint && !(*rarray).pdata.is_null()
        {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray->len == 0 || (rarray->len != 0 && rarray->pdata != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_ptr_array_maybe_expand(
        rarray,
        (1 as guint).wrapping_add((*rarray).null_terminated() as guint),
    );
    let fresh12 = (*rarray).len;
    (*rarray).len = (*rarray).len.wrapping_add(1);
    let ref mut fresh13 = *(*rarray).pdata.offset(fresh12 as isize);
    *fresh13 = data;
    safe_c2rust_ptr_array_maybe_null_terminate(rarray);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_extend(
    mut array_to_extend: *mut GPtrArray,
    mut array: *mut GPtrArray,
    mut func: GCopyFunc,
    mut user_data: gpointer,
) {
    let mut rarray_to_extend: *mut GRealPtrArray = array_to_extend as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if !array_to_extend.is_null() {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array_to_extend != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*array).len == 0 as ::core::ffi::c_uint {
        return;
    }
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if (*array).len
            == (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
        {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0
        && (*rarray_to_extend).null_terminated() as ::core::ffi::c_int != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"adding %u to array would overflow\0" as *const u8 as *const gchar,
            (*array).len,
        );
        loop {}
    }
    safe_c2rust_g_ptr_array_maybe_expand(
        rarray_to_extend,
        (*array)
            .len
            .wrapping_add((*rarray_to_extend).null_terminated() as guint),
    );
    if func.is_some() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*array).len {
            let ref mut fresh14 = *(*rarray_to_extend)
                .pdata
                .offset(i.wrapping_add((*rarray_to_extend).len) as isize);
            *fresh14 = func.expect("non-null function pointer")(
                *(*array).pdata.offset(i as isize) as gconstpointer,
                user_data,
            );
            i = i.wrapping_add(1);
        }
    } else if (*array).len > 0 as guint {
        memcpy(
            (*rarray_to_extend)
                .pdata
                .offset((*rarray_to_extend).len as isize) as *mut ::core::ffi::c_void,
            (*array).pdata as *const ::core::ffi::c_void,
            ((*array).len as size_t).wrapping_mul(::core::mem::size_of::<gpointer>() as size_t),
        );
    }
    (*rarray_to_extend).len = (*rarray_to_extend).len.wrapping_add((*array).len);
    safe_c2rust_ptr_array_maybe_null_terminate(rarray_to_extend);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_extend_and_steal(
    mut array_to_extend: *mut GPtrArray,
    mut array: *mut GPtrArray,
) {
    let mut pdata: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    safe_c2rust_g_ptr_array_extend(array_to_extend, array, None, NULL);
    pdata = safe_c2rust_g_steal_pointer(&raw mut (*array).pdata as gpointer) as *mut gpointer
        as *mut gpointer;
    (*array).len = 0 as guint;
    (*(array as *mut GRealPtrArray)).alloc = 0 as guint;
    safe_c2rust_g_ptr_array_unref(array);
    g_free(pdata as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_insert(
    mut array: *mut GPtrArray,
    mut index_: gint,
    mut data: gpointer,
) {
    let mut rarray: *mut GRealPtrArray = array as *mut GRealPtrArray;
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !rarray.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rarray\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if index_ >= -(1 as ::core::ffi::c_int) {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if index_ <= (*rarray).len as gint {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= (gint)rarray->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_ptr_array_maybe_expand(
        rarray,
        (1 as guint).wrapping_add((*rarray).null_terminated() as guint),
    );
    if index_ < 0 as ::core::ffi::c_int {
        index_ = (*rarray).len as gint;
    }
    if (index_ as guint) < (*rarray).len {
        memmove(
            (*rarray)
                .pdata
                .offset((index_ as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as *mut gpointer as *mut ::core::ffi::c_void,
            (*rarray).pdata.offset(index_ as isize) as *mut gpointer as *const ::core::ffi::c_void,
            ((*rarray).len.wrapping_sub(index_ as guint) as size_t)
                .wrapping_mul(::core::mem::size_of::<gpointer>() as size_t),
        );
    }
    (*rarray).len = (*rarray).len.wrapping_add(1);
    let ref mut fresh15 = *(*rarray).pdata.offset(index_ as isize);
    *fresh15 = data;
    safe_c2rust_ptr_array_maybe_null_terminate(rarray);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_sort(
    mut array: *mut GPtrArray,
    mut compare_func: GCompareFunc,
) {
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*array).len > 0 as guint {
        g_qsort_with_data(
            (*array).pdata as gconstpointer,
            (*array).len as gint,
            ::core::mem::size_of::<gpointer>() as gsize,
            ::core::mem::transmute::<GCompareFunc, GCompareDataFunc>(compare_func),
            NULL,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_sort_with_data(
    mut array: *mut GPtrArray,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*array).len > 0 as guint {
        g_qsort_with_data(
            (*array).pdata as gconstpointer,
            (*array).len as gint,
            ::core::mem::size_of::<gpointer>() as gsize,
            compare_func,
            user_data,
        );
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_compare_ptr_array_values(
    mut a: gconstpointer,
    mut b: gconstpointer,
    mut user_data: gpointer,
) -> gint {
    let mut aa: gconstpointer = *(a as *mut gconstpointer);
    let mut bb: gconstpointer = *(b as *mut gconstpointer);
    let mut compare_func: GCompareFunc =
        ::core::mem::transmute::<gpointer, GCompareFunc>(user_data);
    return compare_func.expect("non-null function pointer")(aa, bb);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_sort_values(
    mut array: *mut GPtrArray,
    mut compare_func: GCompareFunc,
) {
    safe_c2rust_g_ptr_array_sort_with_data(
        array,
        Some(
            safe_c2rust_compare_ptr_array_values
                as unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
        ),
        ::core::mem::transmute::<GCompareFunc, gpointer>(compare_func),
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_compare_ptr_array_values_with_data(
    mut a: gconstpointer,
    mut b: gconstpointer,
    mut user_data: gpointer,
) -> gint {
    let mut aa: gconstpointer = *(a as *mut gconstpointer);
    let mut bb: gconstpointer = *(b as *mut gconstpointer);
    let mut data: *mut GPtrArraySortValuesData = user_data as *mut GPtrArraySortValuesData;
    return (*data).compare_func.expect("non-null function pointer")(aa, bb, (*data).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_sort_values_with_data(
    mut array: *mut GPtrArray,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut fresh16: GPtrArraySortValuesData = GPtrArraySortValuesData {
        compare_func: compare_func,
        user_data: user_data,
    };
    safe_c2rust_g_ptr_array_sort_with_data(
        array,
        Some(
            safe_c2rust_compare_ptr_array_values_with_data
                as unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
        ),
        &raw mut fresh16 as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_foreach(
    mut array: *mut GPtrArray,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as guint;
    while i < (*array).len {
        Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            *(*array).pdata.offset(i as isize),
            user_data,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_find(
    mut haystack: *mut GPtrArray,
    mut needle: gconstpointer,
    mut index_: *mut guint,
) -> gboolean {
    return safe_c2rust_g_ptr_array_find_with_equal_func(haystack, needle, None, index_);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ptr_array_find_with_equal_func(
    mut haystack: *mut GPtrArray,
    mut needle: gconstpointer,
    mut equal_func: GEqualFunc,
    mut index_: *mut guint,
) -> gboolean {
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !haystack.is_null() {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"haystack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if equal_func.is_none() {
        equal_func =
            Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean)
                as GEqualFunc;
    }
    i = 0 as guint;
    while i < (*haystack).len {
        if equal_func.expect("non-null function pointer")(
            *(*haystack).pdata.offset(i as isize) as gconstpointer,
            needle,
        ) != 0
        {
            if !index_.is_null() {
                *index_ = i;
            }
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_new() -> *mut GByteArray {
    return safe_c2rust_g_array_sized_new(FALSE, FALSE, 1 as guint, 0 as guint) as *mut GByteArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_steal(
    mut array: *mut GByteArray,
    mut len: *mut gsize,
) -> *mut guint8 {
    return safe_c2rust_g_array_steal(array as *mut GArray, len) as *mut guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_new_take(
    mut data: *mut guint8,
    mut len: gsize,
) -> *mut GByteArray {
    let mut array: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut real: *mut GRealArray = ::core::ptr::null_mut::<GRealArray>();
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if len
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as gsize
        {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"len <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    array = safe_c2rust_g_byte_array_new();
    real = array as *mut GRealArray;
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if (*real).data.is_null() {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
            2747 as ::core::ffi::c_int,
            G_STRFUNC,
            b"real->data == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if (*real).len == 0 as guint {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/garray.c\0" as *const u8 as *const ::core::ffi::c_char,
            2748 as ::core::ffi::c_int,
            G_STRFUNC,
            b"real->len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*real).data = data;
    (*real).len = len as guint;
    (*real).elt_capacity = len as guint;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_sized_new(
    mut reserved_size: guint,
) -> *mut GByteArray {
    return safe_c2rust_g_array_sized_new(FALSE, FALSE, 1 as guint, reserved_size)
        as *mut GByteArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_free(
    mut array: *mut GByteArray,
    mut free_segment: gboolean,
) -> *mut guint8 {
    return safe_c2rust_g_array_free(array as *mut GArray, free_segment) as *mut guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_free_to_bytes(
    mut array: *mut GByteArray,
) -> *mut GBytes {
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    length = (*array).len as gsize;
    return g_bytes_new_take(
        safe_c2rust_g_byte_array_free(array, FALSE) as gpointer,
        length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_ref(
    mut array: *mut GByteArray,
) -> *mut GByteArray {
    return safe_c2rust_g_array_ref(array as *mut GArray) as *mut GByteArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_unref(mut array: *mut GByteArray) {
    safe_c2rust_g_array_unref(array as *mut GArray);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_append(
    mut array: *mut GByteArray,
    mut data: *const guint8,
    mut len: guint,
) -> *mut GByteArray {
    safe_c2rust_g_array_append_vals(
        array as *mut GArray,
        data as *mut guint8 as gconstpointer,
        len,
    );
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_prepend(
    mut array: *mut GByteArray,
    mut data: *const guint8,
    mut len: guint,
) -> *mut GByteArray {
    safe_c2rust_g_array_prepend_vals(
        array as *mut GArray,
        data as *mut guint8 as gconstpointer,
        len,
    );
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_set_size(
    mut array: *mut GByteArray,
    mut length: guint,
) -> *mut GByteArray {
    safe_c2rust_g_array_set_size(array as *mut GArray, length);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_remove_index(
    mut array: *mut GByteArray,
    mut index_: guint,
) -> *mut GByteArray {
    safe_c2rust_g_array_remove_index(array as *mut GArray, index_);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_remove_index_fast(
    mut array: *mut GByteArray,
    mut index_: guint,
) -> *mut GByteArray {
    safe_c2rust_g_array_remove_index_fast(array as *mut GArray, index_);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_remove_range(
    mut array: *mut GByteArray,
    mut index_: guint,
    mut length: guint,
) -> *mut GByteArray {
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if !array.is_null() {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if index_ <= (*array).len {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if index_
            <= (2147483647 as ::core::ffi::c_int as guint)
                .wrapping_mul(2 as guint)
                .wrapping_add(1 as guint)
                .wrapping_sub(length)
        {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ <= G_MAXUINT - length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if index_.wrapping_add(length) <= (*array).len {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"index_ + length <= array->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GByteArray>();
    }
    return safe_c2rust_g_array_remove_range(array as *mut GArray, index_, length)
        as *mut GByteArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_sort(
    mut array: *mut GByteArray,
    mut compare_func: GCompareFunc,
) {
    safe_c2rust_g_array_sort(array as *mut GArray, compare_func);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_byte_array_sort_with_data(
    mut array: *mut GByteArray,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    safe_c2rust_g_array_sort_with_data(array as *mut GArray, compare_func, user_data);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
