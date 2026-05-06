extern "C" {
    pub type _GHashTable;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove_all(hash_table: *mut GHashTable);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
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
}
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStringChunk {
    pub const_table: *mut GHashTable,
    pub storage_list: *mut GSList,
    pub storage_next: gsize,
    pub this_size: gsize,
    pub default_size: gsize,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GHashTable = _GHashTable;
pub type GStringChunk = _GStringChunk;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_new(mut size: gsize) -> *mut GStringChunk {
    let mut new_chunk: *mut GStringChunk = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GStringChunk>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GStringChunk;
    let mut actual_size: gsize = 1 as gsize;
    actual_size = safe_c2rust_g_nearest_pow(if 1 as gsize > size { 1 as gsize } else { size });
    (*new_chunk).const_table = ::core::ptr::null_mut::<GHashTable>();
    (*new_chunk).storage_list = ::core::ptr::null_mut::<GSList>();
    (*new_chunk).storage_next = actual_size;
    (*new_chunk).default_size = actual_size;
    (*new_chunk).this_size = actual_size;
    return new_chunk;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_free(mut chunk: *mut GStringChunk) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !chunk.is_null() {
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
            b"chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*chunk).storage_list.is_null() {
        g_slist_free_full(
            (*chunk).storage_list,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if !(*chunk).const_table.is_null() {
        g_hash_table_destroy((*chunk).const_table);
    }
    g_free(chunk as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_clear(mut chunk: *mut GStringChunk) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !chunk.is_null() {
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
            b"chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*chunk).storage_list.is_null() {
        g_slist_free_full(
            (*chunk).storage_list,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*chunk).storage_list = ::core::ptr::null_mut::<GSList>();
        (*chunk).storage_next = (*chunk).default_size;
        (*chunk).this_size = (*chunk).default_size;
    }
    if !(*chunk).const_table.is_null() {
        g_hash_table_remove_all((*chunk).const_table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_insert(
    mut chunk: *mut GStringChunk,
    mut string: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !chunk.is_null() {
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
            b"chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_string_chunk_insert_len(
        chunk,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_insert_const(
    mut chunk: *mut GStringChunk,
    mut string: *const gchar,
) -> *mut gchar {
    let mut lookup: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !chunk.is_null() {
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
            b"chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if (*chunk).const_table.is_null() {
        (*chunk).const_table = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    lookup = g_hash_table_lookup((*chunk).const_table, string as *mut gchar as gconstpointer)
        as *mut ::core::ffi::c_char;
    if lookup.is_null() {
        lookup = safe_c2rust_g_string_chunk_insert(chunk, string) as *mut ::core::ffi::c_char;
        g_hash_table_add((*chunk).const_table, lookup as gpointer);
    }
    return lookup as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_chunk_insert_len(
    mut chunk: *mut GStringChunk,
    mut string: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut size: gsize = 0;
    let mut pos: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !chunk.is_null() {
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
            b"chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        size = strlen(string as *const ::core::ffi::c_char) as gsize;
    } else {
        size = len as gsize;
    }
    if G_MAXSIZE.wrapping_sub((*chunk).storage_next) < size.wrapping_add(1 as gsize)
        || (*chunk)
            .storage_next
            .wrapping_add(size)
            .wrapping_add(1 as gsize)
            > (*chunk).this_size
    {
        let mut new_size: gsize =
            safe_c2rust_g_nearest_pow(if (*chunk).default_size > size.wrapping_add(1 as gsize) {
                (*chunk).default_size
            } else {
                size.wrapping_add(1 as gsize)
            });
        if new_size == 0 as gsize {
            new_size = size.wrapping_add(1 as gsize);
        }
        (*chunk).storage_list = g_slist_prepend(
            (*chunk).storage_list,
            ({
                let mut __n: gsize = new_size;
                let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut gchar as gpointer,
        );
        (*chunk).this_size = new_size;
        (*chunk).storage_next = 0 as gsize;
    }
    pos = ((*(*chunk).storage_list).data as *mut gchar).offset((*chunk).storage_next as isize);
    *pos.offset(size as isize) = '\0' as i32 as gchar;
    memcpy(
        pos as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*chunk).storage_next = (*chunk)
        .storage_next
        .wrapping_add(size.wrapping_add(1 as gsize));
    return pos;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_nearest_pow\0" as *const u8 as *const ::core::ffi::c_char;
