use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_insert_vals(
        array: *mut GArray,
        index_: guint,
        data: gconstpointer,
        len: guint,
    ) -> *mut GArray;
    fn g_array_set_size(array: *mut GArray, length: guint) -> *mut GArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_name_from_instance(instance: *mut GTypeInstance) -> *const gchar;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
}
pub type size_t = usize;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: GFileAttributeInfoFlags = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: GFileAttributeInfoFlags = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: GFileAttributeInfoFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfo {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GFileAttributeType,
    pub flags: GFileAttributeInfoFlags,
}
pub type GFileAttributeInfo = _GFileAttributeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfoList {
    pub infos: *mut GFileAttributeInfo,
    pub n_infos: ::core::ffi::c_int,
}
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GFileAttributeInfoList) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GFileAttributeInfoListPriv {
    pub public: GFileAttributeInfoList,
    pub array: *mut GArray,
    pub ref_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GFileAttributeInfoList) -> *mut GFileAttributeInfoList>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GFileAttributeInfoList) -> *mut GFileAttributeInfoList>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GFileAttributeValue {
    #[bitfield(name = "type_0", ty = "guint", bits = "0..=7")]
    #[bitfield(name = "status", ty = "guint", bits = "8..=15")]
    pub type_0_status: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub boolean: gboolean,
    pub int32: gint32,
    pub uint32: guint32,
    pub int64: gint64,
    pub uint64: guint64,
    pub string: *mut ::core::ffi::c_char,
    pub obj: *mut GObject,
    pub stringv: *mut *mut ::core::ffi::c_char,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_free(
    mut attr: *mut GFileAttributeValue,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    g_free(attr as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_clear(
    mut attr: *mut GFileAttributeValue,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
        || (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int
    {
        g_free((*attr).u.string as gpointer);
    }
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_STRINGV as ::core::ffi::c_int
    {
        g_strfreev((*attr).u.stringv as *mut *mut gchar);
    }
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_OBJECT as ::core::ffi::c_int
        && !(*attr).u.obj.is_null()
    {
        g_object_unref((*attr).u.obj as gpointer);
    }
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set(
    mut attr: *mut GFileAttributeValue,
    mut new_value: *const GFileAttributeValue,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !new_value.is_null() {
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
            b"new_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    *attr = *new_value;
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
        || (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int
    {
        (*attr).u.string = safe_c2rust_g_strdup_inline((*attr).u.string);
    }
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_STRINGV as ::core::ffi::c_int
    {
        (*attr).u.stringv =
            g_strdupv((*attr).u.stringv as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
    }
    if (*attr).type_0() as ::core::ffi::c_int == G_FILE_ATTRIBUTE_TYPE_OBJECT as ::core::ffi::c_int
        && !(*attr).u.obj.is_null()
    {
        g_object_ref((*attr).u.obj as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_new() -> *mut GFileAttributeValue {
    let mut attr: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    attr = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileAttributeValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GFileAttributeValue;
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int as guint as guint);
    return attr;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_peek_as_pointer(
    mut attr: *mut GFileAttributeValue,
) -> gpointer {
    match (*attr).type_0() as ::core::ffi::c_int {
        1 | 2 => return (*attr).u.string as gpointer,
        9 => return (*attr).u.stringv as gpointer,
        8 => return (*attr).u.obj as gpointer,
        _ => return &raw mut (*attr).u as gpointer,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_dup(
    mut other: *const GFileAttributeValue,
) -> *mut GFileAttributeValue {
    let mut attr: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !other.is_null() {
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
            b"other != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeValue>();
    }
    attr = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileAttributeValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GFileAttributeValue;
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int as guint as guint);
    safe_c2rust__g_file_attribute_value_set(attr, other);
    return attr;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GFileAttributeInfoList\0" as *const u8 as *const gchar),
        C2RustUnnamed_0 {
            do_copy_type: Some(
                safe_c2rust_g_file_attribute_info_list_dup
                    as unsafe extern "C" fn(
                        *mut GFileAttributeInfoList,
                    ) -> *mut GFileAttributeInfoList,
            ),
        },
        C2RustUnnamed {
            do_free_type: Some(
                safe_c2rust_g_file_attribute_info_list_unref
                    as unsafe extern "C" fn(*mut GFileAttributeInfoList) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_file_attribute_info_list_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_valid_char(mut c: ::core::ffi::c_char) -> gboolean {
    return (c as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 126 as ::core::ffi::c_int
        && c as ::core::ffi::c_int != '\\' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_escape_byte_string(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut num_invalid: size_t = 0;
    let mut escaped_val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_uchar = 0;
    let hex_digits: [::core::ffi::c_char; 17] =
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0");
    len = strlen(str);
    num_invalid = 0 as size_t;
    i = 0 as size_t;
    while i < len {
        if safe_c2rust_valid_char(*str.offset(i as isize)) == 0 {
            num_invalid = num_invalid.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if num_invalid == 0 as size_t {
        return safe_c2rust_g_strdup_inline(str);
    } else {
        if num_invalid
            >= (SIZE_MAX as size_t)
                .wrapping_sub(len)
                .wrapping_div(3 as size_t)
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        escaped_val = g_malloc(
            (len as gsize)
                .wrapping_add((num_invalid as gsize).wrapping_mul(3 as gsize))
                .wrapping_add(1 as gsize),
        ) as *mut ::core::ffi::c_char;
        p = escaped_val;
        i = 0 as size_t;
        while i < len {
            c = *str.offset(i as isize) as ::core::ffi::c_uchar;
            if safe_c2rust_valid_char(c as ::core::ffi::c_char) != 0 {
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = c as ::core::ffi::c_char;
            } else {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = '\\' as i32 as ::core::ffi::c_char;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = 'x' as i32 as ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = hex_digits[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as usize];
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 =
                    hex_digits[(c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
            }
            i = i.wrapping_add(1);
        }
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = 0 as ::core::ffi::c_char;
        return escaped_val;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_as_string(
    mut attr: *const GFileAttributeValue,
) -> *mut ::core::ffi::c_char {
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: ::core::ffi::c_int = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    match (*attr).type_0() as ::core::ffi::c_int {
        1 => {
            str = safe_c2rust_g_strdup_inline((*attr).u.string);
        }
        9 => {
            s = g_string_new(b"[\0" as *const u8 as *const gchar);
            i = 0 as ::core::ffi::c_int;
            while !(*(*attr).u.stringv.offset(i as isize)).is_null() {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            *(*attr).u.stringv.offset(i as isize);
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            __val,
                            if ({
                                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_16
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
                        s,
                        *(*attr).u.stringv.offset(i as isize),
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                if !(*(*attr)
                    .u
                    .stringv
                    .offset((i + 1 as ::core::ffi::c_int) as isize))
                .is_null()
                {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b", \0" as *const u8 as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                s,
                                __val,
                                if ({
                                    let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_17
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            b", \0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
                i += 1;
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"]\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        s,
                        __val,
                        if ({
                            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_18
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
                    s,
                    b"]\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            str = (if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(s, 0 as gboolean)
                } else {
                    g_string_free_and_steal(s)
                }
            } else {
                g_string_free(s, 0 as gboolean)
            }) as *mut ::core::ffi::c_char;
        }
        2 => {
            str = safe_c2rust_escape_byte_string((*attr).u.string);
        }
        3 => {
            str = g_strdup_printf(
                b"%s\0" as *const u8 as *const gchar,
                if (*attr).u.boolean != 0 {
                    b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
                },
            ) as *mut ::core::ffi::c_char;
        }
        4 => {
            str = g_strdup_printf(b"%u\0" as *const u8 as *const gchar, (*attr).u.uint32)
                as *mut ::core::ffi::c_char;
        }
        5 => {
            str = g_strdup_printf(b"%i\0" as *const u8 as *const gchar, (*attr).u.int32)
                as *mut ::core::ffi::c_char;
        }
        6 => {
            str = g_strdup_printf(b"%lu\0" as *const u8 as *const gchar, (*attr).u.uint64)
                as *mut ::core::ffi::c_char;
        }
        7 => {
            str = g_strdup_printf(b"%li\0" as *const u8 as *const gchar, (*attr).u.int64)
                as *mut ::core::ffi::c_char;
        }
        8 => {
            str = g_strdup_printf(
                b"%s:%p\0" as *const u8 as *const gchar,
                g_type_name_from_instance((*attr).u.obj as *mut GTypeInstance),
                (*attr).u.obj,
            ) as *mut ::core::ffi::c_char;
        }
        0 => {
            str = safe_c2rust_g_strdup_inline(
                b"<unset>\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Invalid type in GFileInfo attribute\0" as *const u8 as *const gchar,
            );
            str = safe_c2rust_g_strdup_inline(
                b"<invalid>\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_string(
    mut attr: *const GFileAttributeValue,
) -> *const ::core::ffi::c_char {
    if attr.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
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
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_STRING\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*attr).u.string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_byte_string(
    mut attr: *const GFileAttributeValue,
) -> *const ::core::ffi::c_char {
    if attr.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_BYTE_STRING\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*attr).u.string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_stringv(
    mut attr: *const GFileAttributeValue,
) -> *mut *mut ::core::ffi::c_char {
    if attr.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_STRINGV as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_STRINGV\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    return (*attr).u.stringv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_boolean(
    mut attr: *const GFileAttributeValue,
) -> gboolean {
    if attr.is_null() {
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_BOOLEAN as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_BOOLEAN\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*attr).u.boolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_uint32(
    mut attr: *const GFileAttributeValue,
) -> guint32 {
    if attr.is_null() {
        return 0 as guint32;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_UINT32 as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_UINT32\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return (*attr).u.uint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_int32(
    mut attr: *const GFileAttributeValue,
) -> gint32 {
    if attr.is_null() {
        return 0 as gint32;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_INT32 as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_INT32\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    return (*attr).u.int32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_uint64(
    mut attr: *const GFileAttributeValue,
) -> guint64 {
    if attr.is_null() {
        return 0 as guint64;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_UINT64 as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_UINT64\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    return (*attr).u.uint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_int64(
    mut attr: *const GFileAttributeValue,
) -> gint64 {
    if attr.is_null() {
        return 0 as gint64;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_INT64 as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_INT64\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    return (*attr).u.int64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_get_object(
    mut attr: *const GFileAttributeValue,
) -> *mut GObject {
    if attr.is_null() {
        return ::core::ptr::null_mut::<GObject>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*attr).type_0() as ::core::ffi::c_int
            == G_FILE_ATTRIBUTE_TYPE_OBJECT as ::core::ffi::c_int
        {
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
            b"attr->type == G_FILE_ATTRIBUTE_TYPE_OBJECT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    return (*attr).u.obj;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_from_pointer(
    mut value: *mut GFileAttributeValue,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
    mut dup: gboolean,
) {
    safe_c2rust__g_file_attribute_value_clear(value);
    (*value).set_type_0(type_0 as guint as guint);
    match type_0 as ::core::ffi::c_uint {
        1 | 2 => {
            if dup != 0 {
                (*value).u.string =
                    safe_c2rust_g_strdup_inline(value_p as *const ::core::ffi::c_char);
            } else {
                (*value).u.string = value_p as *mut ::core::ffi::c_char;
            }
        }
        9 => {
            if dup != 0 {
                (*value).u.stringv =
                    g_strdupv(value_p as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
            } else {
                (*value).u.stringv = value_p as *mut *mut ::core::ffi::c_char;
            }
        }
        8 => {
            if dup != 0 {
                (*value).u.obj = g_object_ref(value_p) as *mut GObject;
            } else {
                (*value).u.obj = value_p as *mut GObject;
            }
        }
        3 => {
            (*value).u.boolean = *(value_p as *mut gboolean);
        }
        4 => {
            (*value).u.uint32 = *(value_p as *mut guint32);
        }
        5 => {
            (*value).u.int32 = *(value_p as *mut gint32);
        }
        6 => {
            (*value).u.uint64 = *(value_p as *mut guint64);
        }
        7 => {
            (*value).u.int64 = *(value_p as *mut gint64);
        }
        0 => {}
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unknown type specified in g_file_info_set_attribute\0" as *const u8
                    as *const gchar,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_string(
    mut attr: *mut GFileAttributeValue,
    mut string: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int as guint as guint);
    (*attr).u.string = safe_c2rust_g_strdup_inline(string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_byte_string(
    mut attr: *mut GFileAttributeValue,
    mut string: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int as guint as guint);
    (*attr).u.string = safe_c2rust_g_strdup_inline(string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_stringv(
    mut attr: *mut GFileAttributeValue,
    mut value: *mut *mut ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !attr.is_null() {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_STRINGV as ::core::ffi::c_int as guint as guint);
    (*attr).u.stringv = g_strdupv(value as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_boolean(
    mut attr: *mut GFileAttributeValue,
    mut value: gboolean,
) {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_BOOLEAN as ::core::ffi::c_int as guint as guint);
    (*attr).u.boolean = (value != 0) as ::core::ffi::c_int as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_uint32(
    mut attr: *mut GFileAttributeValue,
    mut value: guint32,
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_UINT32 as ::core::ffi::c_int as guint as guint);
    (*attr).u.uint32 = value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_int32(
    mut attr: *mut GFileAttributeValue,
    mut value: gint32,
) {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_INT32 as ::core::ffi::c_int as guint as guint);
    (*attr).u.int32 = value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_uint64(
    mut attr: *mut GFileAttributeValue,
    mut value: guint64,
) {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !attr.is_null() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_UINT64 as ::core::ffi::c_int as guint as guint);
    (*attr).u.uint64 = value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_int64(
    mut attr: *mut GFileAttributeValue,
    mut value: gint64,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_INT64 as ::core::ffi::c_int as guint as guint);
    (*attr).u.int64 = value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_value_set_object(
    mut attr: *mut GFileAttributeValue,
    mut obj: *mut GObject,
) {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !attr.is_null() {
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
            b"attr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !obj.is_null() {
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
            b"obj != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_attribute_value_clear(attr);
    (*attr).set_type_0(G_FILE_ATTRIBUTE_TYPE_OBJECT as ::core::ffi::c_int as guint as guint);
    (*attr).u.obj = g_object_ref(obj as gpointer) as *mut GObject as *mut GObject;
}
unsafe extern "C" fn safe_c2rust_list_update_public(mut priv_0: *mut GFileAttributeInfoListPriv) {
    (*priv_0).public.infos = (*(*priv_0).array).data as *mut GFileAttributeInfo;
    (*priv_0).public.n_infos = (*(*priv_0).array).len as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_new() -> *mut GFileAttributeInfoList
{
    let mut priv_0: *mut GFileAttributeInfoListPriv =
        ::core::ptr::null_mut::<GFileAttributeInfoListPriv>();
    priv_0 = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileAttributeInfoListPriv>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GFileAttributeInfoListPriv;
    (*priv_0).ref_count = 1 as ::core::ffi::c_int;
    (*priv_0).array = g_array_new(
        TRUE,
        FALSE,
        ::core::mem::size_of::<GFileAttributeInfo>() as guint,
    );
    safe_c2rust_list_update_public(priv_0);
    return priv_0 as *mut GFileAttributeInfoList;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_dup(
    mut list: *mut GFileAttributeInfoList,
) -> *mut GFileAttributeInfoList {
    let mut new: *mut GFileAttributeInfoListPriv =
        ::core::ptr::null_mut::<GFileAttributeInfoListPriv>();
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !list.is_null() {
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
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    new = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileAttributeInfoListPriv>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GFileAttributeInfoListPriv;
    (*new).ref_count = 1 as ::core::ffi::c_int;
    (*new).array = g_array_new(
        TRUE,
        FALSE,
        ::core::mem::size_of::<GFileAttributeInfo>() as guint,
    );
    g_array_set_size((*new).array, (*list).n_infos as guint);
    safe_c2rust_list_update_public(new);
    i = 0 as ::core::ffi::c_int;
    while i < (*list).n_infos {
        let ref mut fresh0 = (*(*new).public.infos.offset(i as isize)).name;
        *fresh0 = safe_c2rust_g_strdup_inline((*(*list).infos.offset(i as isize)).name);
        (*(*new).public.infos.offset(i as isize)).type_0 =
            (*(*list).infos.offset(i as isize)).type_0;
        (*(*new).public.infos.offset(i as isize)).flags = (*(*list).infos.offset(i as isize)).flags;
        i += 1;
    }
    return new as *mut GFileAttributeInfoList;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_ref(
    mut list: *mut GFileAttributeInfoList,
) -> *mut GFileAttributeInfoList {
    let mut priv_0: *mut GFileAttributeInfoListPriv = list as *mut GFileAttributeInfoListPriv;
    let mut old_ref_count: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !list.is_null() {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    old_ref_count = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).ref_count;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut (*priv_0).ref_count,
            1 as ::core::ffi::c_int,
        )
    }) as ::core::ffi::c_int;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if old_ref_count > 0 as ::core::ffi::c_int {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"old_ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_unref(
    mut list: *mut GFileAttributeInfoList,
) {
    let mut priv_0: *mut GFileAttributeInfoListPriv = list as *mut GFileAttributeInfoListPriv;
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !list.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if (*priv_0).ref_count > 0 as ::core::ffi::c_int {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"priv->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).ref_count;
            (*priv_0).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*priv_0).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < (*list).n_infos {
            g_free((*(*list).infos.offset(i as isize)).name as gpointer);
            i += 1;
        }
        g_array_free((*priv_0).array, TRUE);
        g_free(list as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_bsearch(
    mut list: *mut GFileAttributeInfoList,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut start: ::core::ffi::c_int = 0;
    let mut end: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    start = 0 as ::core::ffi::c_int;
    end = (*list).n_infos;
    while start != end {
        mid = start + (end - start) / 2 as ::core::ffi::c_int;
        if strcmp(name, (*(*list).infos.offset(mid as isize)).name) < 0 as ::core::ffi::c_int {
            end = mid;
        } else if strcmp(name, (*(*list).infos.offset(mid as isize)).name) > 0 as ::core::ffi::c_int
        {
            start = mid + 1 as ::core::ffi::c_int;
        } else {
            return mid;
        }
    }
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_lookup(
    mut list: *mut GFileAttributeInfoList,
    mut name: *const ::core::ffi::c_char,
) -> *const GFileAttributeInfo {
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !list.is_null() {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GFileAttributeInfo>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GFileAttributeInfo>();
    }
    i = safe_c2rust_g_file_attribute_info_list_bsearch(list, name);
    if i < (*list).n_infos
        && strcmp((*(*list).infos.offset(i as isize)).name, name) == 0 as ::core::ffi::c_int
    {
        return (*list).infos.offset(i as isize) as *mut GFileAttributeInfo;
    }
    return ::core::ptr::null::<GFileAttributeInfo>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_list_add(
    mut list: *mut GFileAttributeInfoList,
    mut name: *const ::core::ffi::c_char,
    mut type_0: GFileAttributeType,
    mut flags: GFileAttributeInfoFlags,
) {
    let mut priv_0: *mut GFileAttributeInfoListPriv = list as *mut GFileAttributeInfoListPriv;
    let mut info: GFileAttributeInfo = _GFileAttributeInfo {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        type_0: G_FILE_ATTRIBUTE_TYPE_INVALID,
        flags: G_FILE_ATTRIBUTE_INFO_NONE,
    };
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !list.is_null() {
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
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    i = safe_c2rust_g_file_attribute_info_list_bsearch(list, name);
    if i < (*list).n_infos
        && strcmp((*(*list).infos.offset(i as isize)).name, name) == 0 as ::core::ffi::c_int
    {
        (*(*list).infos.offset(i as isize)).type_0 = type_0;
        return;
    }
    info.name = safe_c2rust_g_strdup_inline(name);
    info.type_0 = type_0;
    info.flags = flags;
    g_array_insert_vals(
        (*priv_0).array,
        i as guint,
        &raw mut info as gconstpointer,
        1 as guint,
    );
    safe_c2rust_list_update_public(priv_0);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
