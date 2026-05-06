extern "C" {
    pub type _GVariantType;
    pub type _GHashTable;
    fn g_variant_type_peek_string(type_0: *const GVariantType) -> *const gchar;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_element(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_first(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_next(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_n_items(type_0: *const GVariantType) -> gsize;
    fn g_variant_type_string_get_depth_(type_string: *const gchar) -> gsize;
    fn g_free(mem: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
    );
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_atomic_ref_count_compare(arc: *mut gatomicrefcount, val: gint) -> gboolean;
}
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type gatomicrefcount = gint;
pub type GVariantType = _GVariantType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantTypeInfo {
    pub fixed_size: gsize,
    pub alignment: guchar,
    pub container_class: guchar,
}
pub type GVariantTypeInfo = _GVariantTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVariantMemberInfo {
    pub type_info: *mut GVariantTypeInfo,
    pub i: gsize,
    pub a: gsize,
    pub b: gint8,
    pub c: gint8,
    pub ending_type: guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContainerInfo {
    pub info: GVariantTypeInfo,
    pub type_string: *mut gchar,
    pub ref_count: gatomicrefcount,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayInfo {
    pub container: ContainerInfo,
    pub element: *mut GVariantTypeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleInfo {
    pub container: ContainerInfo,
    pub members: *mut GVariantMemberInfo,
    pub n_members: gsize,
}
pub type GRecMutex = _GRecMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GHashTable = _GHashTable;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_TYPE_INFO_CHAR_MAYBE: ::core::ffi::c_int = 'm' as i32;
pub const G_VARIANT_TYPE_INFO_CHAR_ARRAY: ::core::ffi::c_int = 'a' as i32;
pub const G_VARIANT_TYPE_INFO_CHAR_TUPLE: ::core::ffi::c_int = '(' as i32;
pub const G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY: ::core::ffi::c_int = '{' as i32;
pub const G_VARIANT_MEMBER_ENDING_FIXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_VARIANT_MEMBER_ENDING_LAST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_VARIANT_MEMBER_ENDING_OFFSET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut safe_c2rust_g_variant_type_info_basic_table: [GVariantTypeInfo; 24] = [
    _GVariantTypeInfo {
        fixed_size: 1 as gsize,
        alignment: (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 8 as gsize,
        alignment: (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 4 as gsize,
        alignment: (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 4 as gsize,
        alignment: (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 2 as gsize,
        alignment: (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 2 as gsize,
        alignment: (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 8 as gsize,
        alignment: (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 4 as gsize,
        alignment: (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 0 as gsize,
        alignment: 0 as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 8 as gsize,
        alignment: (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
    _GVariantTypeInfo {
        fixed_size: 1 as gsize,
        alignment: (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guchar,
        container_class: 0 as guchar,
    },
];
static mut safe_c2rust_g_variant_type_info_basic_chars: [[::core::ffi::c_char; 2]; 24] = unsafe {
    [
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"b\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"d\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"g\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"h\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"i\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"n\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"o\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"q\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"s\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"t\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"u\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"v\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"x\0"),
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"y\0"),
    ]
};
unsafe extern "C" fn safe_c2rust_g_variant_type_info_check(
    mut info: *const GVariantTypeInfo,
    mut container_class: ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if container_class == 0
            || (*info).container_class as ::core::ffi::c_int
                == container_class as ::core::ffi::c_int
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
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            165 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!container_class || info->container_class == container_class\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*info).alignment as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*info).alignment as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            || (*info).alignment as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            || (*info).alignment as ::core::ffi::c_int == 7 as ::core::ffi::c_int
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            169 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info->alignment == 0 || info->alignment == 1 || info->alignment == 3 || info->alignment == 7\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*info).container_class != 0 {
        let mut container: *mut ContainerInfo = info as *mut ContainerInfo;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if g_atomic_ref_count_compare(&raw mut (*container).ref_count, 0 as gint) == 0 {
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
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!g_atomic_ref_count_compare (&container->ref_count, 0)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if !(*container).type_string.is_null() {
                _g_boolean_var_11 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_11 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_11
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int,
                G_STRFUNC,
                b"container->type_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        let mut index: gint = 0;
        index = info.offset_from(
            &raw const safe_c2rust_g_variant_type_info_basic_table as *const GVariantTypeInfo,
        ) as ::core::ffi::c_long as gint;
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if (::core::mem::size_of::<[GVariantTypeInfo; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<GVariantTypeInfo>() as usize)
                == 24 as usize
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int,
                G_STRFUNC,
                b"G_N_ELEMENTS (g_variant_type_info_basic_table) == 24\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if (::core::mem::size_of::<[[::core::ffi::c_char; 2]; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                == 24 as usize
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_int,
                G_STRFUNC,
                b"G_N_ELEMENTS (g_variant_type_info_basic_chars) == 24\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if 0 as ::core::ffi::c_int <= index && index < 24 as ::core::ffi::c_int {
                _g_boolean_var_14 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_14 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_14
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                190 as ::core::ffi::c_int,
                G_STRFUNC,
                b"0 <= index && index < 24\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if safe_c2rust_g_variant_type_info_basic_chars[index as usize]
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != ' ' as i32
            {
                _g_boolean_var_15 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_15 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_15
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                191 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_variant_type_info_basic_chars[index][0] != ' '\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_get_type_string(
    mut info: *mut GVariantTypeInfo,
) -> *const gchar {
    safe_c2rust_g_variant_type_info_check(info, 0 as ::core::ffi::c_char);
    if (*info).container_class != 0 {
        let mut container: *mut ContainerInfo = info as *mut ContainerInfo;
        return (*container).type_string;
    } else {
        let mut index: gint = 0;
        index = info.offset_from(
            &raw const safe_c2rust_g_variant_type_info_basic_table as *const GVariantTypeInfo,
        ) as ::core::ffi::c_long as gint;
        return &raw const *(&raw const safe_c2rust_g_variant_type_info_basic_chars
            as *const [::core::ffi::c_char; 2])
            .offset(index as isize) as *const gchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_query(
    mut info: *mut GVariantTypeInfo,
    mut alignment: *mut guint,
    mut fixed_size: *mut gsize,
) {
    if !alignment.is_null() {
        *alignment = (*info).alignment as guint;
    }
    if !fixed_size.is_null() {
        *fixed_size = (*info).fixed_size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_query_depth(
    mut info: *mut GVariantTypeInfo,
) -> gsize {
    safe_c2rust_g_variant_type_info_check(info, 0 as ::core::ffi::c_char);
    if (*info).container_class != 0 {
        let mut container: *mut ContainerInfo = info as *mut ContainerInfo;
        return g_variant_type_string_get_depth_((*container).type_string);
    }
    return 1 as gsize;
}
pub const GV_ARRAY_INFO_CLASS: ::core::ffi::c_int = 'a' as i32;
unsafe extern "C" fn safe_c2rust_GV_ARRAY_INFO(mut info: *mut GVariantTypeInfo) -> *mut ArrayInfo {
    safe_c2rust_g_variant_type_info_check(info, GV_ARRAY_INFO_CLASS as ::core::ffi::c_char);
    return info as *mut ArrayInfo;
}
unsafe extern "C" fn safe_c2rust_array_info_free(mut info: *mut GVariantTypeInfo) {
    let mut array_info: *mut ArrayInfo = ::core::ptr::null_mut::<ArrayInfo>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*info).container_class as ::core::ffi::c_int == 'a' as i32 {
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
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            299 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info->container_class == GV_ARRAY_INFO_CLASS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    array_info = info as *mut ArrayInfo;
    safe_c2rust_g_variant_type_info_unref((*array_info).element);
    g_slice_free1(
        ::core::mem::size_of::<ArrayInfo>() as gsize,
        array_info as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_array_info_new(
    mut type_0: *const GVariantType,
) -> *mut ContainerInfo {
    let mut info: *mut ArrayInfo = ::core::ptr::null_mut::<ArrayInfo>();
    info = g_slice_alloc(::core::mem::size_of::<ArrayInfo>() as gsize) as *mut ArrayInfo;
    (*info).container.info.container_class = GV_ARRAY_INFO_CLASS as guchar;
    (*info).element = safe_c2rust_g_variant_type_info_get(g_variant_type_element(type_0));
    (*info).container.info.alignment = (*(*info).element).alignment;
    (*info).container.info.fixed_size = 0 as gsize;
    return info as *mut ContainerInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_element(
    mut info: *mut GVariantTypeInfo,
) -> *mut GVariantTypeInfo {
    return (*safe_c2rust_GV_ARRAY_INFO(info)).element;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_query_element(
    mut info: *mut GVariantTypeInfo,
    mut alignment: *mut guint,
    mut fixed_size: *mut gsize,
) {
    safe_c2rust_g_variant_type_info_query(
        (*safe_c2rust_GV_ARRAY_INFO(info)).element,
        alignment,
        fixed_size,
    );
}
pub const GV_TUPLE_INFO_CLASS: ::core::ffi::c_int = 'r' as i32;
unsafe extern "C" fn safe_c2rust_GV_TUPLE_INFO(mut info: *mut GVariantTypeInfo) -> *mut TupleInfo {
    safe_c2rust_g_variant_type_info_check(info, GV_TUPLE_INFO_CLASS as ::core::ffi::c_char);
    return info as *mut TupleInfo;
}
unsafe extern "C" fn safe_c2rust_tuple_info_free(mut info: *mut GVariantTypeInfo) {
    let mut tuple_info: *mut TupleInfo = ::core::ptr::null_mut::<TupleInfo>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*info).container_class as ::core::ffi::c_int == 'r' as i32 {
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
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            369 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info->container_class == GV_TUPLE_INFO_CLASS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    tuple_info = info as *mut TupleInfo;
    i = 0 as gsize;
    while i < (*tuple_info).n_members {
        safe_c2rust_g_variant_type_info_unref(
            (*(*tuple_info).members.offset(i as isize)).type_info,
        );
        i = i.wrapping_add(1);
    }
    g_slice_free1(
        (::core::mem::size_of::<GVariantMemberInfo>() as gsize)
            .wrapping_mul((*tuple_info).n_members),
        (*tuple_info).members as gpointer,
    );
    g_slice_free1(
        ::core::mem::size_of::<TupleInfo>() as gsize,
        tuple_info as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_tuple_allocate_members(
    mut type_0: *const GVariantType,
    mut members: *mut *mut GVariantMemberInfo,
    mut n_members: *mut gsize,
) {
    let mut item_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut i: gsize = 0 as gsize;
    *n_members = g_variant_type_n_items(type_0);
    *members = g_slice_alloc(
        (::core::mem::size_of::<GVariantMemberInfo>() as gsize).wrapping_mul(*n_members),
    ) as *mut GVariantMemberInfo;
    item_type = g_variant_type_first(type_0);
    while !item_type.is_null() {
        let fresh1 = i;
        i = i.wrapping_add(1);
        let mut member: *mut GVariantMemberInfo =
            (*members).offset(fresh1 as isize) as *mut GVariantMemberInfo;
        (*member).type_info = safe_c2rust_g_variant_type_info_get(item_type);
        item_type = g_variant_type_next(item_type);
        if (*(*member).type_info).fixed_size != 0 {
            (*member).ending_type = G_VARIANT_MEMBER_ENDING_FIXED as guint8;
        } else if item_type.is_null() {
            (*member).ending_type = G_VARIANT_MEMBER_ENDING_LAST as guint8;
        } else {
            (*member).ending_type = G_VARIANT_MEMBER_ENDING_OFFSET as guint8;
        }
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if i == *n_members {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            407 as ::core::ffi::c_int,
            G_STRFUNC,
            b"i == *n_members\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_tuple_get_item(
    mut info: *mut TupleInfo,
    mut item: *mut GVariantMemberInfo,
    mut d: *mut gsize,
    mut e: *mut gsize,
) -> gboolean {
    if (*info).members.offset((*info).n_members as isize) as *mut GVariantMemberInfo == item {
        return FALSE;
    }
    *d = (*(*item).type_info).alignment as gsize;
    *e = (*(*item).type_info).fixed_size;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_tuple_table_append(
    mut items: *mut *mut GVariantMemberInfo,
    mut i: gsize,
    mut a: gsize,
    mut b: gsize,
    mut c: gsize,
) {
    let fresh0 = *items;
    *items = (*items).offset(1);
    let mut item: *mut GVariantMemberInfo = fresh0;
    a = a.wrapping_add(!b & c);
    c &= b;
    (*item).i = i;
    (*item).a = a.wrapping_add(b);
    (*item).b = !b as gint8;
    (*item).c = c as gint8;
}
unsafe extern "C" fn safe_c2rust_tuple_align(mut offset: gsize, mut alignment: guint) -> gsize {
    return offset.wrapping_add(offset.wrapping_neg() & alignment as gsize);
}
unsafe extern "C" fn safe_c2rust_tuple_generate_table(mut info: *mut TupleInfo) {
    let mut items: *mut GVariantMemberInfo = (*info).members;
    let mut i: gsize = -(1 as ::core::ffi::c_int) as gsize;
    let mut a: gsize = 0 as gsize;
    let mut b: gsize = 0 as gsize;
    let mut c: gsize = 0 as gsize;
    let mut d: gsize = 0;
    let mut e: gsize = 0;
    while safe_c2rust_tuple_get_item(info, items, &raw mut d, &raw mut e) != 0 {
        if d <= b {
            c = safe_c2rust_tuple_align(c, d as guint);
        } else {
            a = a.wrapping_add(safe_c2rust_tuple_align(c, b as guint));
            b = d;
            c = 0 as gsize;
        }
        safe_c2rust_tuple_table_append(&raw mut items, i, a, b, c);
        if e == 0 as gsize {
            i = i.wrapping_add(1);
            c = 0 as gsize;
            b = c;
            a = b;
        } else {
            c = c.wrapping_add(e);
        }
    }
}
unsafe extern "C" fn safe_c2rust_tuple_set_base_info(mut info: *mut TupleInfo) {
    let mut base: *mut GVariantTypeInfo = &raw mut (*info).container.info;
    if (*info).n_members > 0 as gsize {
        let mut m: *mut GVariantMemberInfo = ::core::ptr::null_mut::<GVariantMemberInfo>();
        (*base).alignment = 0 as guchar;
        m = (*info).members;
        while m < (*info).members.offset((*info).n_members as isize) as *mut GVariantMemberInfo {
            (*base).alignment = ((*base).alignment as ::core::ffi::c_int
                | (*(*m).type_info).alignment as ::core::ffi::c_int)
                as guchar;
            m = m.offset(1);
        }
        m = m.offset(-1);
        if (*m).i == -(1 as ::core::ffi::c_int) as gsize && (*(*m).type_info).fixed_size != 0 {
            (*base).fixed_size = safe_c2rust_tuple_align(
                ((*m).a & (*m).b as gsize | (*m).c as gsize)
                    .wrapping_add((*(*m).type_info).fixed_size),
                (*base).alignment as guint,
            );
        } else {
            (*base).fixed_size = 0 as gsize;
        }
    } else {
        (*base).alignment = 0 as guchar;
        (*base).fixed_size = 1 as gsize;
    };
}
unsafe extern "C" fn safe_c2rust_tuple_info_new(
    mut type_0: *const GVariantType,
) -> *mut ContainerInfo {
    let mut info: *mut TupleInfo = ::core::ptr::null_mut::<TupleInfo>();
    info = g_slice_alloc(::core::mem::size_of::<TupleInfo>() as gsize) as *mut TupleInfo;
    (*info).container.info.container_class = GV_TUPLE_INFO_CLASS as guchar;
    safe_c2rust_tuple_allocate_members(
        type_0,
        &raw mut (*info).members,
        &raw mut (*info).n_members,
    );
    safe_c2rust_tuple_generate_table(info);
    safe_c2rust_tuple_set_base_info(info);
    return info as *mut ContainerInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_n_members(
    mut info: *mut GVariantTypeInfo,
) -> gsize {
    return (*safe_c2rust_GV_TUPLE_INFO(info)).n_members;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_member_info(
    mut info: *mut GVariantTypeInfo,
    mut index: gsize,
) -> *const GVariantMemberInfo {
    let mut tuple_info: *mut TupleInfo = safe_c2rust_GV_TUPLE_INFO(info);
    if index < (*tuple_info).n_members {
        return (*tuple_info).members.offset(index as isize) as *mut GVariantMemberInfo;
    }
    return ::core::ptr::null::<GVariantMemberInfo>();
}
static mut safe_c2rust_g_variant_type_info_lock: GRecMutex = _GRecMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_g_variant_type_info_table: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_get(
    mut type_0: *const GVariantType,
) -> *mut GVariantTypeInfo {
    let mut type_char: ::core::ffi::c_char = 0;
    type_char = *g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_char;
    if type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_MAYBE
        || type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_ARRAY
        || type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_TUPLE
        || type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY
    {
        let mut info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
        let mut type_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
        type_string = g_variant_type_dup_string(type_0);
        g_rec_mutex_lock(&raw mut safe_c2rust_g_variant_type_info_lock);
        if safe_c2rust_g_variant_type_info_table.is_null() {
            safe_c2rust_g_variant_type_info_table = g_hash_table_new(
                Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            );
        }
        info = g_hash_table_lookup(
            safe_c2rust_g_variant_type_info_table,
            type_string as gconstpointer,
        ) as *mut GVariantTypeInfo;
        if info.is_null() {
            let mut container: *mut ContainerInfo = ::core::ptr::null_mut::<ContainerInfo>();
            if type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_MAYBE
                || type_char as ::core::ffi::c_int == G_VARIANT_TYPE_INFO_CHAR_ARRAY
            {
                container = safe_c2rust_array_info_new(type_0);
            } else {
                container = safe_c2rust_tuple_info_new(type_0);
            }
            info = container as *mut GVariantTypeInfo;
            (*container).type_string = type_string;
            g_atomic_ref_count_init(&raw mut (*container).ref_count);
            g_hash_table_insert(
                safe_c2rust_g_variant_type_info_table,
                type_string as gpointer,
                info as gpointer,
            );
            type_string = ::core::ptr::null_mut::<gchar>();
        } else {
            safe_c2rust_g_variant_type_info_ref(info);
        }
        g_rec_mutex_unlock(&raw mut safe_c2rust_g_variant_type_info_lock);
        safe_c2rust_g_variant_type_info_check(info, 0 as ::core::ffi::c_char);
        g_free(type_string as gpointer);
        return info;
    } else {
        let mut info_0: *const GVariantTypeInfo = ::core::ptr::null::<GVariantTypeInfo>();
        let mut index: ::core::ffi::c_int = 0;
        index = type_char as ::core::ffi::c_int - 'b' as i32;
        if ({
            let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
            if (::core::mem::size_of::<[GVariantTypeInfo; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<GVariantTypeInfo>() as usize)
                == 24 as usize
            {
                _g_boolean_var_19 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_19 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_19
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                818 as ::core::ffi::c_int,
                G_STRFUNC,
                b"G_N_ELEMENTS (g_variant_type_info_basic_table) == 24\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut __n1: gint64 = 0 as gint64;
        let mut __n2: gint64 = index as gint64;
        if !(__n1 <= __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                819 as ::core::ffi::c_int,
                G_STRFUNC,
                b"0 <= index\0" as *const u8 as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"<=\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        let mut __n1_0: gint64 = index as gint64;
        let mut __n2_0: gint64 = 24 as gint64;
        if !(__n1_0 < __n2_0) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_int,
                G_STRFUNC,
                b"index < 24\0" as *const u8 as *const ::core::ffi::c_char,
                __n1_0 as guint64,
                b"<\0" as *const u8 as *const ::core::ffi::c_char,
                __n2_0 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        info_0 = (&raw const safe_c2rust_g_variant_type_info_basic_table
            as *const GVariantTypeInfo)
            .offset(index as isize);
        safe_c2rust_g_variant_type_info_check(info_0, 0 as ::core::ffi::c_char);
        return info_0 as *mut GVariantTypeInfo;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_ref(
    mut info: *mut GVariantTypeInfo,
) -> *mut GVariantTypeInfo {
    safe_c2rust_g_variant_type_info_check(info, 0 as ::core::ffi::c_char);
    if (*info).container_class != 0 {
        let mut container: *mut ContainerInfo = info as *mut ContainerInfo;
        g_atomic_ref_count_inc(&raw mut (*container).ref_count);
    }
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_unref(mut info: *mut GVariantTypeInfo) {
    safe_c2rust_g_variant_type_info_check(info, 0 as ::core::ffi::c_char);
    if (*info).container_class != 0 {
        let mut container: *mut ContainerInfo = info as *mut ContainerInfo;
        g_rec_mutex_lock(&raw mut safe_c2rust_g_variant_type_info_lock);
        if g_atomic_ref_count_dec(&raw mut (*container).ref_count) != 0 {
            g_hash_table_remove(
                safe_c2rust_g_variant_type_info_table,
                (*container).type_string as gconstpointer,
            );
            if g_hash_table_size(safe_c2rust_g_variant_type_info_table) == 0 as guint {
                g_hash_table_unref(safe_c2rust_g_variant_type_info_table);
                safe_c2rust_g_variant_type_info_table = ::core::ptr::null_mut::<GHashTable>();
            }
            g_rec_mutex_unlock(&raw mut safe_c2rust_g_variant_type_info_lock);
            g_free((*container).type_string as gpointer);
            if (*info).container_class as ::core::ffi::c_int == GV_ARRAY_INFO_CLASS {
                safe_c2rust_array_info_free(info);
            } else if (*info).container_class as ::core::ffi::c_int == GV_TUPLE_INFO_CLASS {
                safe_c2rust_tuple_info_free(info);
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvarianttypeinfo.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    892 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        } else {
            g_rec_mutex_unlock(&raw mut safe_c2rust_g_variant_type_info_lock);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_info_assert_no_infos() {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_info_table.is_null() {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttypeinfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            902 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_type_info_table == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_variant_type_info_check\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
