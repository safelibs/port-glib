extern "C" {
    pub type _GHashTable;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_foreach(hash_table: *mut GHashTable, func: GHFunc, user_data: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCache {
    pub value_new_func: GCacheNewFunc,
    pub value_destroy_func: GCacheDestroyFunc,
    pub key_dup_func: GCacheDupFunc,
    pub key_destroy_func: GCacheDestroyFunc,
    pub key_table: *mut GHashTable,
    pub value_table: *mut GHashTable,
}
pub type GHashTable = _GHashTable;
pub type GCacheDestroyFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GCacheDupFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GCacheNewFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GCache = _GCache;
pub type GCacheNode = _GCacheNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCacheNode {
    pub value: gpointer,
    pub ref_count: gint,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_cache_node_new(mut value: gpointer) -> *mut GCacheNode {
    let mut node: *mut GCacheNode =
        g_slice_alloc(::core::mem::size_of::<GCacheNode>() as gsize) as *mut GCacheNode;
    (*node).value = value;
    (*node).ref_count = 1 as ::core::ffi::c_int as gint;
    return node;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_cache_node_destroy(mut node: *mut GCacheNode) {
    g_slice_free1(
        ::core::mem::size_of::<GCacheNode>() as gsize,
        node as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_new(
    mut value_new_func: GCacheNewFunc,
    mut value_destroy_func: GCacheDestroyFunc,
    mut key_dup_func: GCacheDupFunc,
    mut key_destroy_func: GCacheDestroyFunc,
    mut hash_key_func: GHashFunc,
    mut hash_value_func: GHashFunc,
    mut key_equal_func: GEqualFunc,
) -> *mut GCache {
    let mut cache: *mut GCache = ::core::ptr::null_mut::<GCache>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if value_new_func.is_some() {
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
            b"value_new_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if value_destroy_func.is_some() {
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
            b"value_destroy_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if key_dup_func.is_some() {
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
            b"key_dup_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if key_destroy_func.is_some() {
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
            b"key_destroy_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if hash_key_func.is_some() {
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
            b"hash_key_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if hash_value_func.is_some() {
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
            b"hash_value_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if key_equal_func.is_some() {
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
            b"key_equal_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCache>();
    }
    cache = g_slice_alloc(::core::mem::size_of::<GCache>() as gsize) as *mut GCache;
    (*cache).value_new_func = value_new_func;
    (*cache).value_destroy_func = value_destroy_func;
    (*cache).key_dup_func = key_dup_func;
    (*cache).key_destroy_func = key_destroy_func;
    (*cache).key_table = g_hash_table_new(hash_key_func, key_equal_func);
    (*cache).value_table = g_hash_table_new(hash_value_func, None);
    return cache;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_destroy(mut cache: *mut GCache) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !cache.is_null() {
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
            b"cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_destroy((*cache).key_table);
    g_hash_table_destroy((*cache).value_table);
    g_slice_free1(::core::mem::size_of::<GCache>() as gsize, cache as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_insert(
    mut cache: *mut GCache,
    mut key: gpointer,
) -> gpointer {
    let mut node: *mut GCacheNode = ::core::ptr::null_mut::<GCacheNode>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !cache.is_null() {
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
            b"cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    node = g_hash_table_lookup((*cache).key_table, key as gconstpointer) as *mut GCacheNode;
    if !node.is_null() {
        (*node).ref_count += 1 as ::core::ffi::c_int;
        return (*node).value;
    }
    key = Some((*cache).key_dup_func.expect("non-null function pointer"))
        .expect("non-null function pointer")(key);
    value = Some((*cache).value_new_func.expect("non-null function pointer"))
        .expect("non-null function pointer")(key);
    node = safe_c2rust_g_cache_node_new(value);
    g_hash_table_insert((*cache).key_table, key, node as gpointer);
    g_hash_table_insert((*cache).value_table, value, key);
    return (*node).value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_remove(
    mut cache: *mut GCache,
    mut value: gconstpointer,
) {
    let mut node: *mut GCacheNode = ::core::ptr::null_mut::<GCacheNode>();
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !cache.is_null() {
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
            b"cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    key = g_hash_table_lookup((*cache).value_table, value);
    node = g_hash_table_lookup((*cache).key_table, key as gconstpointer) as *mut GCacheNode;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*node).ref_count -= 1 as ::core::ffi::c_int;
    if (*node).ref_count == 0 as ::core::ffi::c_int {
        g_hash_table_remove((*cache).value_table, value);
        g_hash_table_remove((*cache).key_table, key as gconstpointer);
        Some(
            (*cache)
                .key_destroy_func
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(key);
        Some(
            (*cache)
                .value_destroy_func
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*node).value);
        safe_c2rust_g_cache_node_destroy(node);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_key_foreach(
    mut cache: *mut GCache,
    mut func: GHFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !cache.is_null() {
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
            b"cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_foreach((*cache).value_table, func, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cache_value_foreach(
    mut cache: *mut GCache,
    mut func: GHFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !cache.is_null() {
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
            b"cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_foreach((*cache).key_table, func, user_data);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_cache_new\0" as *const u8 as *const ::core::ffi::c_char;
