// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_register_static(
        parent_type: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_fundamental(
        type_id: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        finfo: *const GTypeFundamentalInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_check_class_is_a(g_class: *mut GTypeClass, is_a_type: GType) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub union _GTypeCValue {
    pub v_int: gint,
    pub v_long: glong,
    pub v_int64: gint64,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GTypeCValue = _GTypeCValue;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInfo {
    pub class_size: guint16,
    pub base_init: GBaseInitFunc,
    pub base_finalize: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: GInstanceInitFunc,
    pub value_table: *const GTypeValueTable,
}
pub type GTypeValueTable = _GTypeValueTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeValueTable {
    pub value_init: GTypeValueInitFunc,
    pub value_free: GTypeValueFreeFunc,
    pub value_copy: GTypeValueCopyFunc,
    pub value_peek_pointer: GTypeValuePeekPointerFunc,
    pub collect_format: *const gchar,
    pub collect_value: GTypeValueCollectFunc,
    pub lcopy_format: *const gchar,
    pub lcopy_value: GTypeValueLCopyFunc,
}
pub type GTypeValueLCopyFunc =
    Option<unsafe extern "C" fn(*const GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValueCollectFunc =
    Option<unsafe extern "C" fn(*mut GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValuePeekPointerFunc = Option<unsafe extern "C" fn(*const GValue) -> gpointer>;
pub type GTypeValueCopyFunc = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
pub type GTypeValueFreeFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GTypeValueInitFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GBaseFinalizeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBaseInitFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GTypeInfo = _GTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeFundamentalInfo {
    pub type_flags: GTypeFundamentalFlags,
}
pub type GTypeFundamentalFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEEP_DERIVABLE: GTypeFundamentalFlags = 8;
pub const G_TYPE_FLAG_DERIVABLE: GTypeFundamentalFlags = 4;
pub const G_TYPE_FLAG_INSTANTIATABLE: GTypeFundamentalFlags = 2;
pub const G_TYPE_FLAG_CLASSED: GTypeFundamentalFlags = 1;
pub type GTypeFundamentalInfo = _GTypeFundamentalInfo;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsClass {
    pub g_type_class: GTypeClass,
    pub mask: guint,
    pub n_values: guint,
    pub values: *mut GFlagsValue,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsClass = _GFlagsClass;
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
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
unsafe extern "C" fn g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if gstring.is_null() {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if val.is_null() {
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
    if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
            || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
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
pub unsafe extern "C" fn _g_enum_types_init() {
    static mut initialized: gboolean = 0 as gboolean;
    static mut flags_enum_value_table: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_flags_enum_init as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_flags_enum_copy_value
                    as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"i\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_flags_enum_collect_value
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_flags_enum_lcopy_value
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    let mut info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 0 as guint16,
        instance_init: None,
        value_table: &raw const flags_enum_value_table,
    };
    static mut finfo: GTypeFundamentalInfo = _GTypeFundamentalInfo {
        type_flags: (G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
            | G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int)
            as GTypeFundamentalFlags,
    };
    let mut type_0: GType = 0;
    if initialized == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_enum_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"initialized == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    initialized = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    info.class_size = ::core::mem::size_of::<GEnumClass>() as guint16;
    type_0 = g_type_register_fundamental(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GEnum\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        (G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int) as GTypeFlags,
    );
    if type_0 == ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/genums.c\0" as *const u8
                as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_int,
            b"_g_enum_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_ENUM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info.class_size = ::core::mem::size_of::<GFlagsClass>() as guint16;
    type_0 = g_type_register_fundamental(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GFlags\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        (G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int) as GTypeFlags,
    );
    if type_0 == ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/genums.c\0" as *const u8
                as *const ::core::ffi::c_char,
            98 as ::core::ffi::c_int,
            b"_g_enum_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn value_flags_enum_init(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_long = 0 as glong;
}
unsafe extern "C" fn value_flags_enum_copy_value(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_long;
}
unsafe extern "C" fn value_flags_enum_collect_value(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_long =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_int as glong;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_ulong =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_int as guint as gulong;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_flags_enum_lcopy_value(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut int_p: *mut gint =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gint;
    if !int_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_flags_enum_lcopy_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"int_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *int_p = (*value).data[0 as ::core::ffi::c_int as usize].v_long as gint;
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_register_static(
    mut name: *const gchar,
    mut const_static_values: *const GEnumValue,
) -> GType {
    let mut enum_type_info: GTypeInfo = _GTypeInfo {
        class_size: ::core::mem::size_of::<GEnumClass>() as guint16,
        base_init: None,
        base_finalize: None,
        class_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GEnumClass, gpointer) -> ()>,
            GClassInitFunc,
        >(Some(
            g_enum_class_init as unsafe extern "C" fn(*mut GEnumClass, gpointer) -> (),
        )),
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 0 as guint16,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    let mut type_0: GType = 0;
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !const_static_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_static_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    enum_type_info.class_data = const_static_values as gconstpointer;
    type_0 = g_type_register_static(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut enum_type_info,
        G_TYPE_FLAG_NONE,
    );
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_register_static(
    mut name: *const gchar,
    mut const_static_values: *const GFlagsValue,
) -> GType {
    let mut flags_type_info: GTypeInfo = _GTypeInfo {
        class_size: ::core::mem::size_of::<GFlagsClass>() as guint16,
        base_init: None,
        base_finalize: None,
        class_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFlagsClass, gpointer) -> ()>,
            GClassInitFunc,
        >(Some(
            g_flags_class_init as unsafe extern "C" fn(*mut GFlagsClass, gpointer) -> (),
        )),
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 0 as guint16,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    let mut type_0: GType = 0;
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !const_static_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_static_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    flags_type_info.class_data = const_static_values as gconstpointer;
    type_0 = g_type_register_static(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut flags_type_info,
        G_TYPE_FLAG_NONE,
    );
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_complete_type_info(
    mut g_enum_type: GType,
    mut info: *mut GTypeInfo,
    mut const_values: *const GEnumValue,
) {
    if g_type_fundamental(g_enum_type)
        == ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_ENUM (g_enum_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !const_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*info).class_size = ::core::mem::size_of::<GEnumClass>() as guint16;
    (*info).base_init = None;
    (*info).base_finalize = None;
    (*info).class_init = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GEnumClass, gpointer) -> ()>,
        GClassInitFunc,
    >(Some(
        g_enum_class_init as unsafe extern "C" fn(*mut GEnumClass, gpointer) -> (),
    ));
    (*info).class_finalize = None;
    (*info).class_data = const_values as gconstpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_complete_type_info(
    mut g_flags_type: GType,
    mut info: *mut GTypeInfo,
    mut const_values: *const GFlagsValue,
) {
    if g_type_fundamental(g_flags_type)
        == ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_FLAGS (g_flags_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !const_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*info).class_size = ::core::mem::size_of::<GFlagsClass>() as guint16;
    (*info).base_init = None;
    (*info).base_finalize = None;
    (*info).class_init = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GFlagsClass, gpointer) -> ()>,
        GClassInitFunc,
    >(Some(
        g_flags_class_init as unsafe extern "C" fn(*mut GFlagsClass, gpointer) -> (),
    ));
    (*info).class_finalize = None;
    (*info).class_data = const_values as gconstpointer;
}
unsafe extern "C" fn g_enum_class_init(mut class: *mut GEnumClass, mut class_data: gpointer) {
    if ({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_class_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_ENUM_CLASS (class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*class).minimum = 0 as ::core::ffi::c_int as gint;
    (*class).maximum = 0 as ::core::ffi::c_int as gint;
    (*class).n_values = 0 as guint;
    (*class).values = class_data as *mut GEnumValue;
    if !(*class).values.is_null() {
        let mut values: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
        (*class).minimum = (*(*class).values).value;
        (*class).maximum = (*(*class).values).value;
        values = (*class).values;
        while !(*values).value_name.is_null() {
            (*class).minimum = if (*class).minimum < (*values).value {
                (*class).minimum
            } else {
                (*values).value
            };
            (*class).maximum = if (*class).maximum > (*values).value {
                (*class).maximum
            } else {
                (*values).value
            };
            (*class).n_values = (*class).n_values.wrapping_add(1);
            values = values.offset(1);
        }
    }
}
unsafe extern "C" fn g_flags_class_init(mut class: *mut GFlagsClass, mut class_data: gpointer) {
    if ({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_class_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_FLAGS_CLASS (class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*class).mask = 0 as guint;
    (*class).n_values = 0 as guint;
    (*class).values = class_data as *mut GFlagsValue;
    if !(*class).values.is_null() {
        let mut values: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
        values = (*class).values;
        while !(*values).value_name.is_null() {
            (*class).mask |= (*values).value;
            (*class).n_values = (*class).n_values.wrapping_add(1);
            values = values.offset(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_get_value_by_name(
    mut enum_class: *mut GEnumClass,
    mut name: *const gchar,
) -> *mut GEnumValue {
    if ({
        let mut __class: *mut GTypeClass = enum_class as *mut GTypeClass;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_ENUM_CLASS (enum_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GEnumValue>();
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GEnumValue>();
    }
    if (*enum_class).n_values != 0 {
        let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
        enum_value = (*enum_class).values;
        while !(*enum_value).value_name.is_null() {
            if strcmp(
                name as *const ::core::ffi::c_char,
                (*enum_value).value_name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return enum_value;
            }
            enum_value = enum_value.offset(1);
        }
    }
    return ::core::ptr::null_mut::<GEnumValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_get_value_by_name(
    mut flags_class: *mut GFlagsClass,
    mut name: *const gchar,
) -> *mut GFlagsValue {
    if ({
        let mut __class: *mut GTypeClass = flags_class as *mut GTypeClass;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_value_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_FLAGS_CLASS (flags_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFlagsValue>();
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_value_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFlagsValue>();
    }
    if (*flags_class).n_values != 0 {
        let mut flags_value: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
        flags_value = (*flags_class).values;
        while !(*flags_value).value_name.is_null() {
            if strcmp(
                name as *const ::core::ffi::c_char,
                (*flags_value).value_name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return flags_value;
            }
            flags_value = flags_value.offset(1);
        }
    }
    return ::core::ptr::null_mut::<GFlagsValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_get_value_by_nick(
    mut enum_class: *mut GEnumClass,
    mut nick: *const gchar,
) -> *mut GEnumValue {
    if ({
        let mut __class: *mut GTypeClass = enum_class as *mut GTypeClass;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value_by_nick\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_ENUM_CLASS (enum_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GEnumValue>();
    }
    if !nick.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value_by_nick\0" as *const u8 as *const ::core::ffi::c_char,
            b"nick != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GEnumValue>();
    }
    if (*enum_class).n_values != 0 {
        let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
        enum_value = (*enum_class).values;
        while !(*enum_value).value_name.is_null() {
            if !(*enum_value).value_nick.is_null()
                && strcmp(
                    nick as *const ::core::ffi::c_char,
                    (*enum_value).value_nick as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                return enum_value;
            }
            enum_value = enum_value.offset(1);
        }
    }
    return ::core::ptr::null_mut::<GEnumValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_get_value_by_nick(
    mut flags_class: *mut GFlagsClass,
    mut nick: *const gchar,
) -> *mut GFlagsValue {
    if ({
        let mut __class: *mut GTypeClass = flags_class as *mut GTypeClass;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_value_by_nick\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_FLAGS_CLASS (flags_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFlagsValue>();
    }
    if !nick.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_value_by_nick\0" as *const u8 as *const ::core::ffi::c_char,
            b"nick != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFlagsValue>();
    }
    if (*flags_class).n_values != 0 {
        let mut flags_value: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
        flags_value = (*flags_class).values;
        while !(*flags_value).value_nick.is_null() {
            if !(*flags_value).value_nick.is_null()
                && strcmp(
                    nick as *const ::core::ffi::c_char,
                    (*flags_value).value_nick as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                return flags_value;
            }
            flags_value = flags_value.offset(1);
        }
    }
    return ::core::ptr::null_mut::<GFlagsValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_get_value(
    mut enum_class: *mut GEnumClass,
    mut value: gint,
) -> *mut GEnumValue {
    if ({
        let mut __class: *mut GTypeClass = enum_class as *mut GTypeClass;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_ENUM_CLASS (enum_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GEnumValue>();
    }
    if (*enum_class).n_values != 0 {
        let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
        enum_value = (*enum_class).values;
        while !(*enum_value).value_name.is_null() {
            if (*enum_value).value == value {
                return enum_value;
            }
            enum_value = enum_value.offset(1);
        }
    }
    return ::core::ptr::null_mut::<GEnumValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_get_first_value(
    mut flags_class: *mut GFlagsClass,
    mut value: guint,
) -> *mut GFlagsValue {
    if ({
        let mut __class: *mut GTypeClass = flags_class as *mut GTypeClass;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_first_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_FLAGS_CLASS (flags_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFlagsValue>();
    }
    if (*flags_class).n_values != 0 {
        let mut flags_value: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
        if value == 0 as guint {
            flags_value = (*flags_class).values;
            while !(*flags_value).value_name.is_null() {
                if (*flags_value).value == 0 as guint {
                    return flags_value;
                }
                flags_value = flags_value.offset(1);
            }
        } else {
            flags_value = (*flags_class).values;
            while !(*flags_value).value_name.is_null() {
                if (*flags_value).value != 0 as guint
                    && (*flags_value).value & value == (*flags_value).value
                {
                    return flags_value;
                }
                flags_value = flags_value.offset(1);
            }
        }
    }
    return ::core::ptr::null_mut::<GFlagsValue>();
}
#[no_mangle]
pub unsafe extern "C" fn g_enum_to_string(mut g_enum_type: GType, mut value: gint) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut enum_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    if g_type_fundamental(g_enum_type)
        == ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_to_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_ENUM (g_enum_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    enum_class = g_type_class_ref(g_enum_type) as *mut GEnumClass;
    if enum_class.is_null() {
        return g_strdup_printf(b"%d\0" as *const u8 as *const gchar, value);
    }
    enum_value = g_enum_get_value(enum_class, value);
    if enum_value.is_null() {
        result = g_strdup_printf(b"%d\0" as *const u8 as *const gchar, value);
    } else {
        result =
            g_strdup_inline((*enum_value).value_name as *const ::core::ffi::c_char) as *mut gchar;
    }
    g_type_class_unref(enum_class as gpointer);
    return result;
}
unsafe extern "C" fn g_flags_get_value_string(
    mut flags_class: *mut GFlagsClass,
    mut value: guint,
) -> *mut gchar {
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut flags_value: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
    if ({
        let mut __class: *mut GTypeClass = flags_class as *mut GTypeClass;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_get_value_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_FLAGS_CLASS (flags_class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    str = g_string_new(::core::ptr::null::<gchar>());
    while ((*str).len == 0 as gsize || value != 0 as guint) && {
        flags_value = g_flags_get_first_value(flags_class, value);
        !flags_value.is_null()
    } {
        if (*str).len > 0 as gsize {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b" | \0" as *const u8 as *const ::core::ffi::c_char;
                    g_string_append_len_inline(
                        str,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(
                    str,
                    b" | \0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    (*flags_value).value_name as *const ::core::ffi::c_char;
                g_string_append_len_inline(
                    str,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(
                str,
                (*flags_value).value_name as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        value &= !(*flags_value).value;
    }
    if value != 0 as guint || (*str).len == 0 as gsize {
        if (*str).len > 0 as gsize {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b" | \0" as *const u8 as *const ::core::ffi::c_char;
                    g_string_append_len_inline(
                        str,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(
                    str,
                    b" | \0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        g_string_append_printf(str, b"0x%x\0" as *const u8 as *const gchar, value);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(str, 0 as gboolean)
        } else {
            g_string_free_and_steal(str)
        }
    } else {
        g_string_free(str, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_flags_to_string(mut flags_type: GType, mut value: guint) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut flags_class: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
    if g_type_fundamental(flags_type)
        == ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_flags_to_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_FLAGS (flags_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    flags_class = g_type_class_ref(flags_type) as *mut GFlagsClass;
    if flags_class.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_flags_get_value_string(flags_class, value);
    g_type_class_unref(flags_class as gpointer);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_enum(mut value: *mut GValue, mut v_enum: gint) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_ENUM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_long = v_enum as glong;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_enum(mut value: *const GValue) -> gint {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_get_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_ENUM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_long as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_flags(mut value: *mut GValue, mut v_flags: guint) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_FLAGS (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_ulong = v_flags as gulong;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_flags(mut value: *const GValue) -> guint {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_get_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_FLAGS (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint;
}
