extern "C" {
    pub type _GHashTable;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_str_hash(v: gconstpointer) -> guint;
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
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_atomic_rc_box_alloc(block_size: gsize) -> gpointer;
    fn g_atomic_rc_box_dup(block_size: gsize, mem_block: gconstpointer) -> gpointer;
    fn g_atomic_rc_box_acquire(mem_block: gpointer) -> gpointer;
    fn g_atomic_rc_box_release_full(mem_block: gpointer, clear_func: GDestroyNotify);
    fn g_atomic_rc_box_get_size(mem_block: gpointer) -> gsize;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
}
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GHashTable = _GHashTable;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_g__interned_ref_strings_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_interned_ref_strings: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_new(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    len = strlen(str) as gsize;
    res = g_atomic_rc_box_dup(
        (::core::mem::size_of::<::core::ffi::c_char>() as gsize)
            .wrapping_mul(len)
            .wrapping_add(1 as gsize),
        str as gconstpointer,
    ) as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_new_len(
    mut str: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if len < 0 as gssize {
        return safe_c2rust_g_ref_string_new(str);
    }
    res =
        g_atomic_rc_box_alloc((len as gsize).wrapping_add(1 as gsize)) as *mut ::core::ffi::c_char;
    memcpy(
        res as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len as size_t,
    );
    *res.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return res;
}
unsafe extern "C" fn safe_c2rust_interned_str_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    let mut str1: *const ::core::ffi::c_char = v1 as *const ::core::ffi::c_char;
    let mut str2: *const ::core::ffi::c_char = v2 as *const ::core::ffi::c_char;
    if v1 == v2 {
        return TRUE;
    }
    return (strcmp(str1, str2) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_new_intern(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__interned_ref_strings_lock);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_interned_ref_strings.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_interned_ref_strings = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                safe_c2rust_interned_str_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
        );
    }
    res = g_hash_table_lookup(safe_c2rust_interned_ref_strings, str as gconstpointer)
        as *mut ::core::ffi::c_char;
    if !res.is_null() {
        g_atomic_rc_box_acquire(res as gpointer);
        g_mutex_unlock(&raw mut safe_c2rust_g__interned_ref_strings_lock);
        return res;
    }
    res = safe_c2rust_g_ref_string_new(str);
    g_hash_table_add(safe_c2rust_interned_ref_strings, res as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__interned_ref_strings_lock);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_acquire(
    mut str: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return g_atomic_rc_box_acquire(str as gpointer) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_remove_if_interned(mut data: gpointer) {
    let mut str: *mut ::core::ffi::c_char = data as *mut ::core::ffi::c_char;
    g_mutex_lock(&raw mut safe_c2rust_g__interned_ref_strings_lock);
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !safe_c2rust_interned_ref_strings.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        g_hash_table_remove(safe_c2rust_interned_ref_strings, str as gconstpointer);
        if g_hash_table_size(safe_c2rust_interned_ref_strings) == 0 as guint {
            let mut _pp: *mut *mut GHashTable = &raw mut safe_c2rust_interned_ref_strings;
            let mut _ptr: *mut GHashTable = *_pp;
            *_pp = ::core::ptr::null_mut::<GHashTable>();
            if !_ptr.is_null() {
                g_hash_table_destroy(_ptr as *mut GHashTable);
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__interned_ref_strings_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_release(mut str: *mut ::core::ffi::c_char) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_atomic_rc_box_release_full(
        str as gpointer,
        Some(safe_c2rust_remove_if_interned as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_string_length(
    mut str: *mut ::core::ffi::c_char,
) -> gsize {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return g_atomic_rc_box_get_size(str as gpointer).wrapping_sub(1 as gsize);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_ref_string_new\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
