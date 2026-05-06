// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
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
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_free(mem: gpointer);
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_qsort_with_data(
        pbase: gconstpointer,
        total_elems: gint,
        size: gsize,
        compare_func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_unset(value: *mut GValue);
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValueArray {
    pub n_values: guint,
    pub values: *mut GValue,
    pub n_prealloced: guint,
}
pub type GValueArray = _GValueArray;
#[no_mangle]
pub unsafe extern "C" fn g_value_array_get_nth(
    mut value_array: *mut GValueArray,
    mut index: guint,
) -> *mut GValue {
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_get_nth\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValue>();
    }
    if index < (*value_array).n_values {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_get_nth\0" as *const u8 as *const ::core::ffi::c_char,
            b"index < value_array->n_values\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValue>();
    }
    return (*value_array).values.offset(index as isize);
}
#[inline]
unsafe extern "C" fn value_array_grow(
    mut value_array: *mut GValueArray,
    mut n_values: guint,
    mut zero_init: gboolean,
) {
    if n_values >= (*value_array).n_values {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array_grow\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_values >= value_array->n_values\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value_array).n_values = n_values;
    if (*value_array).n_values > (*value_array).n_prealloced {
        let mut i: guint = (*value_array).n_prealloced;
        (*value_array).n_prealloced = (*value_array)
            .n_values
            .wrapping_add(8 as guint)
            .wrapping_sub(1 as guint)
            & !(8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guint;
        (*value_array).values = g_realloc_n(
            (*value_array).values as gpointer,
            (*value_array).n_prealloced as gsize,
            ::core::mem::size_of::<GValue>() as gsize,
        ) as *mut GValue;
        if zero_init == 0 {
            i = (*value_array).n_values;
        }
        memset(
            (*value_array).values.offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*value_array).n_prealloced.wrapping_sub(i) as size_t)
                .wrapping_mul(::core::mem::size_of::<GValue>() as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_new(mut n_prealloced: guint) -> *mut GValueArray {
    let mut value_array: *mut GValueArray =
        g_slice_alloc(::core::mem::size_of::<GValueArray>() as gsize) as *mut GValueArray;
    (*value_array).n_values = 0 as guint;
    (*value_array).n_prealloced = 0 as guint;
    (*value_array).values = ::core::ptr::null_mut::<GValue>();
    value_array_grow(
        value_array,
        n_prealloced,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    (*value_array).n_values = 0 as guint;
    return value_array;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_free(mut value_array: *mut GValueArray) {
    let mut i: guint = 0;
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as guint;
    while i < (*value_array).n_values {
        let mut value: *mut GValue = (*value_array).values.offset(i as isize);
        if (*value).g_type != 0 as GType {
            g_value_unset(value);
        }
        i = i.wrapping_add(1);
    }
    g_free((*value_array).values as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GValueArray>() as gsize,
        value_array as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_copy(
    mut value_array: *const GValueArray,
) -> *mut GValueArray {
    let mut new_array: *mut GValueArray = ::core::ptr::null_mut::<GValueArray>();
    let mut i: guint = 0;
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    new_array = g_slice_alloc(::core::mem::size_of::<GValueArray>() as gsize) as *mut GValueArray;
    (*new_array).n_values = 0 as guint;
    (*new_array).values = ::core::ptr::null_mut::<GValue>();
    (*new_array).n_prealloced = 0 as guint;
    value_array_grow(
        new_array,
        (*value_array).n_values,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    i = 0 as guint;
    while i < (*new_array).n_values {
        if (*(*value_array).values.offset(i as isize)).g_type != 0 as GType {
            let mut value: *mut GValue = (*new_array).values.offset(i as isize);
            g_value_init(value, (*(*value_array).values.offset(i as isize)).g_type);
            g_value_copy((*value_array).values.offset(i as isize), value);
        }
        i = i.wrapping_add(1);
    }
    return new_array;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_prepend(
    mut value_array: *mut GValueArray,
    mut value: *const GValue,
) -> *mut GValueArray {
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_prepend\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    return g_value_array_insert(value_array, 0 as guint, value);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_append(
    mut value_array: *mut GValueArray,
    mut value: *const GValue,
) -> *mut GValueArray {
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_append\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    return g_value_array_insert(value_array, (*value_array).n_values, value);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_insert(
    mut value_array: *mut GValueArray,
    mut index: guint,
    mut value: *const GValue,
) -> *mut GValueArray {
    let mut i: guint = 0;
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_insert\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    if index <= (*value_array).n_values {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_insert\0" as *const u8 as *const ::core::ffi::c_char,
            b"index <= value_array->n_values\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return value_array;
    }
    i = (*value_array).n_values;
    value_array_grow(
        value_array,
        (*value_array).n_values.wrapping_add(1 as guint),
        0 as gboolean,
    );
    if index.wrapping_add(1 as guint) < (*value_array).n_values {
        memmove(
            (*value_array)
                .values
                .offset(index as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*value_array).values.offset(index as isize) as *const ::core::ffi::c_void,
            (i.wrapping_sub(index) as size_t)
                .wrapping_mul(::core::mem::size_of::<GValue>() as size_t),
        );
    }
    memset(
        (*value_array).values.offset(index as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GValue>() as size_t,
    );
    if !value.is_null() {
        g_value_init(
            (*value_array).values.offset(index as isize),
            (*(value as *mut GValue)).g_type,
        );
        g_value_copy(value, (*value_array).values.offset(index as isize));
    }
    return value_array;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_remove(
    mut value_array: *mut GValueArray,
    mut index: guint,
) -> *mut GValueArray {
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_remove\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    if index < (*value_array).n_values {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_remove\0" as *const u8 as *const ::core::ffi::c_char,
            b"index < value_array->n_values\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return value_array;
    }
    if (*(*value_array).values.offset(index as isize)).g_type != 0 as GType {
        g_value_unset((*value_array).values.offset(index as isize));
    }
    (*value_array).n_values = (*value_array).n_values.wrapping_sub(1);
    if index < (*value_array).n_values {
        memmove(
            (*value_array).values.offset(index as isize) as *mut ::core::ffi::c_void,
            (*value_array)
                .values
                .offset(index as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            ((*value_array).n_values.wrapping_sub(index) as size_t)
                .wrapping_mul(::core::mem::size_of::<GValue>() as size_t),
        );
    }
    if (*value_array).n_prealloced > (*value_array).n_values {
        memset(
            (*value_array)
                .values
                .offset((*value_array).n_values as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GValue>() as size_t,
        );
    }
    return value_array;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_sort(
    mut value_array: *mut GValueArray,
    mut compare_func: GCompareFunc,
) -> *mut GValueArray {
    if compare_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_sort\0" as *const u8 as *const ::core::ffi::c_char,
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    if (*value_array).n_values != 0 {
        qsort(
            (*value_array).values as *mut ::core::ffi::c_void,
            (*value_array).n_values as size_t,
            ::core::mem::size_of::<GValue>() as size_t,
            compare_func as __compar_fn_t,
        );
    }
    return value_array;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_sort_with_data(
    mut value_array: *mut GValueArray,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) -> *mut GValueArray {
    if !value_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_sort_with_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    if compare_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_array_sort_with_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValueArray>();
    }
    if (*value_array).n_values != 0 {
        g_qsort_with_data(
            (*value_array).values as gconstpointer,
            (*value_array).n_values as gint,
            ::core::mem::size_of::<GValue>() as gsize,
            compare_func,
            user_data,
        );
    }
    return value_array;
}
