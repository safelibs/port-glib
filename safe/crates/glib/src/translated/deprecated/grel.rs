extern "C" {
    pub type _GHashTable;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_foreach(hash_table: *mut GHashTable, func: GHFunc, user_data: gpointer);
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
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
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRelation {
    pub fields: gint,
    pub current_field: gint,
    pub all_tuples: *mut GHashTable,
    pub hashed_tuple_tables: *mut *mut GHashTable,
    pub count: gint,
}
pub type GHashTable = _GHashTable;
pub type GRelation = _GRelation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTuples {
    pub len: guint,
}
pub type GTuples = _GTuples;
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
pub type va_list = __builtin_va_list;
pub type GRealTuples = _GRealTuples;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealTuples {
    pub len: gint,
    pub width: gint,
    pub data: *mut gpointer,
}
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
unsafe extern "C" fn safe_c2rust_tuple_equal_2(
    mut v_a: gconstpointer,
    mut v_b: gconstpointer,
) -> gboolean {
    let mut a: *mut gpointer = v_a as *mut gpointer;
    let mut b: *mut gpointer = v_b as *mut gpointer;
    return (*a.offset(0 as ::core::ffi::c_int as isize)
        == *b.offset(0 as ::core::ffi::c_int as isize)
        && *a.offset(1 as ::core::ffi::c_int as isize)
            == *b.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_tuple_hash_2(mut v_a: gconstpointer) -> guint {
    let mut a: *mut gpointer = v_a as *mut gpointer;
    return (*a.offset(0 as ::core::ffi::c_int as isize) as gulong
        ^ *a.offset(1 as ::core::ffi::c_int as isize) as gulong) as guint;
}
unsafe extern "C" fn safe_c2rust_tuple_hash(mut fields: gint) -> GHashFunc {
    match fields {
        2 => {
            return Some(safe_c2rust_tuple_hash_2 as unsafe extern "C" fn(gconstpointer) -> guint);
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"no tuple hash for %d\0" as *const u8 as *const gchar,
                fields,
            );
            loop {}
        }
    };
}
unsafe extern "C" fn safe_c2rust_tuple_equal(mut fields: gint) -> GEqualFunc {
    match fields {
        2 => {
            return Some(
                safe_c2rust_tuple_equal_2
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            );
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"no tuple equal for %d\0" as *const u8 as *const gchar,
                fields,
            );
            loop {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_new(mut fields: gint) -> *mut GRelation {
    let mut rel: *mut GRelation = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRelation>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRelation;
    (*rel).fields = fields;
    (*rel).all_tuples = g_hash_table_new(
        safe_c2rust_tuple_hash(fields),
        safe_c2rust_tuple_equal(fields),
    );
    (*rel).hashed_tuple_tables = ({
        let mut __n: gsize = fields as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut GHashTable>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut GHashTable;
    return rel;
}
unsafe extern "C" fn safe_c2rust_relation_delete_value_tuple(
    mut tuple_key: gpointer,
    mut tuple_value: gpointer,
    mut user_data: gpointer,
) {
    let mut relation: *mut GRelation = user_data as *mut GRelation;
    let mut tuple: *mut gpointer = tuple_value as *mut gpointer;
    g_slice_free1(
        ((*relation).fields as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
        tuple as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_relation_free_array(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) {
    g_hash_table_destroy(value as *mut GHashTable);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_destroy(mut relation: *mut GRelation) {
    let mut i: gint = 0;
    if !relation.is_null() {
        i = 0 as ::core::ffi::c_int as gint;
        while i < (*relation).fields {
            if !(*(*relation).hashed_tuple_tables.offset(i as isize)).is_null() {
                g_hash_table_foreach(
                    *(*relation).hashed_tuple_tables.offset(i as isize),
                    Some(
                        safe_c2rust_g_relation_free_array
                            as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
                    ),
                    NULL,
                );
                g_hash_table_destroy(*(*relation).hashed_tuple_tables.offset(i as isize));
            }
            i += 1 as ::core::ffi::c_int;
        }
        g_hash_table_foreach(
            (*relation).all_tuples,
            Some(
                safe_c2rust_relation_delete_value_tuple
                    as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
            ),
            relation as gpointer,
        );
        g_hash_table_destroy((*relation).all_tuples);
        g_free((*relation).hashed_tuple_tables as gpointer);
        g_free(relation as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_index(
    mut relation: *mut GRelation,
    mut field: gint,
    mut hash_func: GHashFunc,
    mut key_equal_func: GEqualFunc,
) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !relation.is_null() {
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
            b"relation != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*relation).count == 0 as ::core::ffi::c_int
            && (*(*relation).hashed_tuple_tables.offset(field as isize)).is_null()
        {
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
            b"relation->count == 0 && relation->hashed_tuple_tables[field] == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let ref mut fresh0 = *(*relation).hashed_tuple_tables.offset(field as isize);
    *fresh0 = g_hash_table_new(hash_func, key_equal_func);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_insert(
    mut relation: *mut GRelation,
    mut args: ...
) {
    let mut tuple: *mut gpointer = g_slice_alloc(
        ((*relation).fields as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
    ) as *mut gpointer;
    let mut args_0: ::core::ffi::VaList;
    let mut i: gint = 0;
    args_0 = args.clone();
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*relation).fields {
        let ref mut fresh1 = *tuple.offset(i as isize);
        *fresh1 = args_0.arg::<gpointer>();
        i += 1 as ::core::ffi::c_int;
    }
    g_hash_table_insert((*relation).all_tuples, tuple as gpointer, tuple as gpointer);
    (*relation).count += 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*relation).fields {
        let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut per_key_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        table = *(*relation).hashed_tuple_tables.offset(i as isize);
        if !table.is_null() {
            key = *tuple.offset(i as isize);
            per_key_table = g_hash_table_lookup(table, key as gconstpointer) as *mut GHashTable;
            if per_key_table.is_null() {
                per_key_table = g_hash_table_new(
                    safe_c2rust_tuple_hash((*relation).fields),
                    safe_c2rust_tuple_equal((*relation).fields),
                );
                g_hash_table_insert(table, key, per_key_table as gpointer);
            }
            g_hash_table_insert(per_key_table, tuple as gpointer, tuple as gpointer);
        }
        i += 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn safe_c2rust_g_relation_delete_tuple(
    mut tuple_key: gpointer,
    mut tuple_value: gpointer,
    mut user_data: gpointer,
) {
    let mut tuple: *mut gpointer = tuple_value as *mut gpointer;
    let mut relation: *mut GRelation = user_data as *mut GRelation;
    let mut j: gint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if tuple_key == tuple_value {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/deprecated/grel.c\0" as *const u8 as *const ::core::ffi::c_char,
            344 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tuple_key == tuple_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    j = 0 as ::core::ffi::c_int as gint;
    while j < (*relation).fields {
        let mut one_table: *mut GHashTable = *(*relation).hashed_tuple_tables.offset(j as isize);
        let mut one_key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut per_key_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        if !one_table.is_null() {
            if !(j == (*relation).current_field) {
                one_key = *tuple.offset(j as isize);
                per_key_table =
                    g_hash_table_lookup(one_table, one_key as gconstpointer) as *mut GHashTable;
                g_hash_table_remove(per_key_table, tuple as gconstpointer);
            }
        }
        j += 1 as ::core::ffi::c_int;
    }
    if g_hash_table_remove((*relation).all_tuples, tuple as gconstpointer) != 0 {
        g_slice_free1(
            ((*relation).fields as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
            tuple as gpointer,
        );
    }
    (*relation).count -= 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_delete(
    mut relation: *mut GRelation,
    mut key: gconstpointer,
    mut field: gint,
) -> gint {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut key_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut count: gint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !relation.is_null() {
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
            b"relation != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    table = *(*relation).hashed_tuple_tables.offset(field as isize);
    count = (*relation).count;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !table.is_null() {
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
            b"table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    key_table = g_hash_table_lookup(table, key) as *mut GHashTable;
    if key_table.is_null() {
        return 0 as gint;
    }
    (*relation).current_field = field;
    g_hash_table_foreach(
        key_table,
        Some(
            safe_c2rust_g_relation_delete_tuple
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        relation as gpointer,
    );
    g_hash_table_remove(table, key);
    g_hash_table_destroy(key_table);
    return count - (*relation).count;
}
unsafe extern "C" fn safe_c2rust_g_relation_select_tuple(
    mut tuple_key: gpointer,
    mut tuple_value: gpointer,
    mut user_data: gpointer,
) {
    let mut tuple: *mut gpointer = tuple_value as *mut gpointer;
    let mut tuples: *mut GRealTuples = user_data as *mut GRealTuples;
    let mut stride: gint = (::core::mem::size_of::<gpointer>() as usize)
        .wrapping_mul((*tuples).width as usize) as gint;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if tuple_key == tuple_value {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/deprecated/grel.c\0" as *const u8 as *const ::core::ffi::c_char,
            428 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tuple_key == tuple_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memcpy(
        (*tuples)
            .data
            .offset(((*tuples).len * (*tuples).width) as isize) as *mut ::core::ffi::c_void,
        tuple as *const ::core::ffi::c_void,
        stride as size_t,
    );
    (*tuples).len += 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_select(
    mut relation: *mut GRelation,
    mut key: gconstpointer,
    mut field: gint,
) -> *mut GTuples {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut key_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut tuples: *mut GRealTuples = ::core::ptr::null_mut::<GRealTuples>();
    let mut count: gint = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !relation.is_null() {
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
            b"relation != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTuples>();
    }
    table = *(*relation).hashed_tuple_tables.offset(field as isize);
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !table.is_null() {
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
            b"table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTuples>();
    }
    tuples = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRealTuples>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRealTuples;
    key_table = g_hash_table_lookup(table, key) as *mut GHashTable;
    if key_table.is_null() {
        return tuples as *mut GTuples;
    }
    count = safe_c2rust_g_relation_count(relation, key, field);
    (*tuples).data = g_malloc(
        (::core::mem::size_of::<gpointer>() as gsize)
            .wrapping_mul((*relation).fields as gsize)
            .wrapping_mul(count as gsize),
    ) as *mut gpointer;
    (*tuples).width = (*relation).fields;
    g_hash_table_foreach(
        key_table,
        Some(
            safe_c2rust_g_relation_select_tuple
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        tuples as gpointer,
    );
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if count == (*tuples).len {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/deprecated/grel.c\0" as *const u8 as *const ::core::ffi::c_char,
            480 as ::core::ffi::c_int,
            G_STRFUNC,
            b"count == tuples->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return tuples as *mut GTuples;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_count(
    mut relation: *mut GRelation,
    mut key: gconstpointer,
    mut field: gint,
) -> gint {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut key_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !relation.is_null() {
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
            b"relation != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    table = *(*relation).hashed_tuple_tables.offset(field as isize);
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !table.is_null() {
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
            b"table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    key_table = g_hash_table_lookup(table, key) as *mut GHashTable;
    if key_table.is_null() {
        return 0 as gint;
    }
    return g_hash_table_size(key_table) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_exists(
    mut relation: *mut GRelation,
    mut args: ...
) -> gboolean {
    let mut tuple: *mut gpointer = g_slice_alloc(
        ((*relation).fields as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
    ) as *mut gpointer;
    let mut args_0: ::core::ffi::VaList;
    let mut i: gint = 0;
    let mut result: gboolean = 0;
    args_0 = args.clone();
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*relation).fields {
        let ref mut fresh2 = *tuple.offset(i as isize);
        *fresh2 = args_0.arg::<gpointer>();
        i += 1 as ::core::ffi::c_int;
    }
    result = (g_hash_table_lookup((*relation).all_tuples, tuple as gconstpointer) != NULL)
        as ::core::ffi::c_int as gboolean;
    g_slice_free1(
        ((*relation).fields as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
        tuple as gpointer,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tuples_destroy(mut tuples0: *mut GTuples) {
    let mut tuples: *mut GRealTuples = tuples0 as *mut GRealTuples;
    if !tuples.is_null() {
        g_free((*tuples).data as gpointer);
        g_free(tuples as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tuples_index(
    mut tuples0: *mut GTuples,
    mut index: gint,
    mut field: gint,
) -> gpointer {
    let mut tuples: *mut GRealTuples = tuples0 as *mut GRealTuples;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !tuples0.is_null() {
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
            b"tuples0 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if field < (*tuples).width {
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
            b"field < tuples->width\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return *(*tuples)
        .data
        .offset((index * (*tuples).width + field) as isize);
}
unsafe extern "C" fn safe_c2rust_g_relation_print_one(
    mut tuple_key: gpointer,
    mut tuple_value: gpointer,
    mut user_data: gpointer,
) {
    let mut i: gint = 0;
    let mut gstring: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut rel: *mut GRelation = user_data as *mut GRelation;
    let mut tuples: *mut gpointer = tuple_value as *mut gpointer;
    gstring = g_string_new(b"[\0" as *const u8 as *const gchar);
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*rel).fields {
        g_string_append_printf(
            gstring,
            b"%p\0" as *const u8 as *const gchar,
            *tuples.offset(i as isize),
        );
        if i < (*rel).fields as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b",\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        gstring,
                        __val,
                        if ({
                            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_21
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    gstring,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        i += 1 as ::core::ffi::c_int;
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"]\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                gstring,
                __val,
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            gstring,
            b"]\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_INFO,
        b"%s\0" as *const u8 as *const gchar,
        (*gstring).str_0,
    );
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                gstring,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(gstring);
        };
    } else {
        g_string_free(
            gstring,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_relation_print_index(
    mut tuple_key: gpointer,
    mut tuple_value: gpointer,
    mut user_data: gpointer,
) {
    let mut rel: *mut GRelation = user_data as *mut GRelation;
    let mut table: *mut GHashTable = tuple_value as *mut GHashTable;
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_INFO,
        b"*** key %p\0" as *const u8 as *const gchar,
        tuple_key,
    );
    g_hash_table_foreach(
        table,
        Some(
            safe_c2rust_g_relation_print_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        rel as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_relation_print(mut relation: *mut GRelation) {
    let mut i: gint = 0;
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_INFO,
        b"*** all tuples (%d)\0" as *const u8 as *const gchar,
        (*relation).count,
    );
    g_hash_table_foreach(
        (*relation).all_tuples,
        Some(
            safe_c2rust_g_relation_print_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        relation as gpointer,
    );
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*relation).fields {
        if !(*(*relation).hashed_tuple_tables.offset(i as isize)).is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_INFO,
                b"*** index %d\0" as *const u8 as *const gchar,
                i,
            );
            g_hash_table_foreach(
                *(*relation).hashed_tuple_tables.offset(i as isize),
                Some(
                    safe_c2rust_g_relation_print_index
                        as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
                ),
                relation as gpointer,
            );
        }
        i += 1 as ::core::ffi::c_int;
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_relation_index\0" as *const u8 as *const ::core::ffi::c_char;
