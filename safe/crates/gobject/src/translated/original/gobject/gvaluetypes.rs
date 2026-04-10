// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    pub type _GVariant;
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strescape(source: *const gchar, exceptions: *const gchar) -> *mut gchar;
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
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
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
    fn g_type_from_name(name: *const gchar) -> GType;
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
    fn g_type_check_value(value: *const GValue) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_fits_pointer(value: *const GValue) -> gboolean;
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
    fn g_value_type_transformable(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_transform(src_value: *const GValue, dest_value: *mut GValue) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
}
pub type size_t = usize;
pub type gint8 = ::core::ffi::c_schar;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GStrv = *mut *mut gchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariant = _GVariant;
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
unsafe extern "C" fn value_init_long0(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_long = 0 as glong;
}
unsafe extern "C" fn value_copy_long0(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_long;
}
unsafe extern "C" fn value_lcopy_char(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut int8_p: *mut gint8 =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gint8;
    if !int8_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_char\0" as *const u8 as *const ::core::ffi::c_char,
            b"int8_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *int8_p = (*value).data[0 as ::core::ffi::c_int as usize].v_int as gint8;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_boolean(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut bool_p: *mut gboolean =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gboolean;
    if !bool_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_boolean\0" as *const u8 as *const ::core::ffi::c_char,
            b"bool_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *bool_p = (*value).data[0 as ::core::ffi::c_int as usize].v_int as gboolean;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_collect_int(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_int;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_int(
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
            b"value_lcopy_int\0" as *const u8 as *const ::core::ffi::c_char,
            b"int_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *int_p = (*value).data[0 as ::core::ffi::c_int as usize].v_int;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_collect_long(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_long =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_long;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_long(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut long_p: *mut glong =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut glong;
    if !long_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_long\0" as *const u8 as *const ::core::ffi::c_char,
            b"long_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *long_p = (*value).data[0 as ::core::ffi::c_int as usize].v_long;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_init_int64(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int64 = 0 as gint64;
}
unsafe extern "C" fn value_copy_int64(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64;
}
unsafe extern "C" fn value_collect_int64(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int64 =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_int64;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_int64(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut int64_p: *mut gint64 =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gint64;
    if !int64_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_int64\0" as *const u8 as *const ::core::ffi::c_char,
            b"int64_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *int64_p = (*value).data[0 as ::core::ffi::c_int as usize].v_int64;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_init_float(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_float = 0.0f32;
}
unsafe extern "C" fn value_copy_float(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_float;
}
unsafe extern "C" fn value_collect_float(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_float =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_double as gfloat;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_float(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut float_p: *mut gfloat =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gfloat;
    if !float_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_float\0" as *const u8 as *const ::core::ffi::c_char,
            b"float_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *float_p = (*value).data[0 as ::core::ffi::c_int as usize].v_float;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_init_double(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_double = 0.0f64 as gdouble;
}
unsafe extern "C" fn value_copy_double(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double;
}
unsafe extern "C" fn value_collect_double(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_double =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_double;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_double(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut double_p: *mut gdouble =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gdouble;
    if !double_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_double\0" as *const u8 as *const ::core::ffi::c_char,
            b"double_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *double_p = (*value).data[0 as ::core::ffi::c_int as usize].v_double;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_init_string(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn value_free_string(mut value: *mut GValue) {
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        == 0
    {
        g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    }
}
unsafe extern "C" fn value_copy_string(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    if (*src_value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as guint
        != 0
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer;
        (*dest_value).data[1 as ::core::ffi::c_int as usize].v_uint =
            (*src_value).data[1 as ::core::ffi::c_int as usize].v_uint;
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_inline(
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer
                as *const ::core::ffi::c_char,
        ) as gpointer;
    };
}
unsafe extern "C" fn value_collect_string(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if (*collect_values.offset(0 as ::core::ffi::c_int as isize))
        .v_pointer
        .is_null()
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer;
        (*value).data[1 as ::core::ffi::c_int as usize].v_uint =
            ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_inline(
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer
                as *const ::core::ffi::c_char,
        ) as gpointer;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_string(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut string_p: *mut *mut gchar =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut *mut gchar;
    if !string_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"string_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        *string_p = ::core::ptr::null_mut::<gchar>();
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        *string_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
    } else {
        *string_p = g_strdup_inline(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_init_pointer(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn value_copy_pointer(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
unsafe extern "C" fn value_peek_pointer0(mut value: *const GValue) -> gpointer {
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
unsafe extern "C" fn value_collect_pointer(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_pointer(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut pointer_p: *mut gpointer =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gpointer;
    if !pointer_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"pointer_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    *pointer_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_free_variant(mut value: *mut GValue) {
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        == 0
        && !(*value).data[0 as ::core::ffi::c_int as usize]
            .v_pointer
            .is_null()
    {
        g_variant_unref((*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant);
    }
}
unsafe extern "C" fn value_copy_variant(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    if !(*src_value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_variant_ref_sink(
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant,
        ) as gpointer;
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    };
}
unsafe extern "C" fn value_collect_variant(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if (*collect_values.offset(0 as ::core::ffi::c_int as isize))
        .v_pointer
        .is_null()
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_variant_ref_sink(
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut GVariant,
        ) as gpointer;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_lcopy_variant(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut variant_p: *mut *mut GVariant =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut *mut GVariant;
    if !variant_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_lcopy_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"variant_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        *variant_p = ::core::ptr::null_mut::<GVariant>();
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        *variant_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    } else {
        *variant_p = g_variant_ref_sink(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant,
        );
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn _g_value_types_init() {
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
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    let finfo: GTypeFundamentalInfo = _GTypeFundamentalInfo {
        type_flags: G_TYPE_FLAG_DERIVABLE,
    };
    let mut type_0: GType = 0;
    static mut value_table: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_long0 as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_long0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"i\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_int
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_char
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table;
    type_0 = g_type_register_fundamental(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gchar\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            452 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_CHAR\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    type_0 = g_type_register_fundamental(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"guchar\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            454 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_UCHAR\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_0: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_long0 as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_long0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"i\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_int
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_boolean
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_0;
    type_0 = g_type_register_fundamental(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gboolean\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            472 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_BOOLEAN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_1: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_long0 as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_long0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"i\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_int
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_int
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_1;
    type_0 = g_type_register_fundamental(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gint\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            490 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_INT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    type_0 = g_type_register_fundamental(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"guint\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            492 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_UINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_2: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_long0 as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_long0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"l\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_long
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_long
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_2;
    type_0 = g_type_register_fundamental(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"glong\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            510 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_LONG\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    type_0 = g_type_register_fundamental(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gulong\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            512 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_ULONG\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_3: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_int64 as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"q\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_int64
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_int64
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_3;
    type_0 = g_type_register_fundamental(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gint64\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            530 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_INT64\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    type_0 = g_type_register_fundamental(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"guint64\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            532 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_UINT64\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_4: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_float as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"d\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_float
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_float
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_4;
    type_0 = g_type_register_fundamental(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gfloat\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            550 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_FLOAT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_5: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_double as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: None,
            collect_format: b"d\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_double
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_double
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_5;
    type_0 = g_type_register_fundamental(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gdouble\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            568 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_DOUBLE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_6: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_string as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: Some(value_free_string as unsafe extern "C" fn(*mut GValue) -> ()),
            value_copy: Some(
                value_copy_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                value_peek_pointer0 as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_string
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_string
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_6;
    type_0 = g_type_register_fundamental(
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gchararray\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            586 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_7: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_pointer as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: None,
            value_copy: Some(
                value_copy_pointer as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                value_peek_pointer0 as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_pointer
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_pointer
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_7;
    type_0 = g_type_register_fundamental(
        ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"gpointer\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            604 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_POINTER\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    static mut value_table_8: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_init_pointer as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: Some(value_free_variant as unsafe extern "C" fn(*mut GValue) -> ()),
            value_copy: Some(
                value_copy_variant as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                value_peek_pointer0 as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_collect_variant
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_lcopy_variant
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    info.value_table = &raw const value_table_8;
    type_0 = g_type_register_fundamental(
        ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GVariant\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvaluetypes.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            622 as ::core::ffi::c_int,
            b"_g_value_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_VARIANT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_char(mut value: *mut GValue, mut v_char: gchar) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_char\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_CHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_int = v_char as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_char(mut value: *const GValue) -> gchar {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_char\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_CHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gchar;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_int as gchar;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_schar(mut value: *mut GValue, mut v_char: gint8) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_schar\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_CHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_int = v_char as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_schar(mut value: *const GValue) -> gint8 {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_schar\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_CHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint8;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_int as gint8;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_uchar(mut value: *mut GValue, mut v_uchar: guchar) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_uchar\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UCHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint = v_uchar as guint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_uchar(mut value: *const GValue) -> guchar {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_uchar\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UCHAR (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guchar;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_uint as guchar;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_boolean(mut value: *mut GValue, mut v_boolean: gboolean) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_boolean\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOOLEAN (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (v_boolean != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_boolean(mut value: *const GValue) -> gboolean {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_boolean\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOOLEAN (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_int as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_int(mut value: *mut GValue, mut v_int: gint) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_int\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_INT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_int = v_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_int(mut value: *const GValue) -> gint {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_int\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_INT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_uint(mut value: *mut GValue, mut v_uint: guint) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_uint\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UINT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint = v_uint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_uint(mut value: *const GValue) -> guint {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_uint\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UINT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_uint;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_long(mut value: *mut GValue, mut v_long: glong) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_long\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_LONG (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_long = v_long;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_long(mut value: *const GValue) -> glong {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_long\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_LONG (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as glong;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_long;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_ulong(mut value: *mut GValue, mut v_ulong: gulong) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_ulong\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_ULONG (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_ulong = v_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_ulong(mut value: *const GValue) -> gulong {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_ulong\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_ULONG (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_int64(mut value: *mut GValue, mut v_int64: gint64) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_int64\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_INT64 (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_int64 = v_int64;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_int64(mut value: *const GValue) -> gint64 {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_int64\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_INT64 (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_int64;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_uint64(mut value: *mut GValue, mut v_uint64: guint64) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_uint64\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UINT64 (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint64 = v_uint64;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_uint64(mut value: *const GValue) -> guint64 {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_uint64\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_UINT64 (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_uint64;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_float(mut value: *mut GValue, mut v_float: gfloat) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_float\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_FLOAT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_float = v_float;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_float(mut value: *const GValue) -> gfloat {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_float\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_FLOAT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gfloat;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_float;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_double(mut value: *mut GValue, mut v_double: gdouble) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_double\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_DOUBLE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_double = v_double;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_double(mut value: *const GValue) -> gdouble {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_double\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_DOUBLE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_double;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_string(mut value: *mut GValue, mut v_string: *const gchar) {
    let mut new_val: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    new_val = g_strdup_inline(v_string as *const ::core::ffi::c_char) as *mut gchar;
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        != 0
    {
        (*value).data[1 as ::core::ffi::c_int as usize].v_uint = 0 as guint;
    } else {
        g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = new_val as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_static_string(
    mut value: *mut GValue,
    mut v_string: *const gchar,
) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_static_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        == 0
    {
        g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    }
    (*value).data[1 as ::core::ffi::c_int as usize].v_uint =
        ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = v_string as *mut gchar as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_interned_string(
    mut value: *mut GValue,
    mut v_string: *const gchar,
) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_interned_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        == 0
    {
        g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    }
    (*value).data[1 as ::core::ffi::c_int as usize].v_uint =
        ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as guint;
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = v_string as *mut gchar as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_string_take_ownership(
    mut value: *mut GValue,
    mut v_string: *mut gchar,
) {
    g_value_take_string(value, v_string);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_take_string(mut value: *mut GValue, mut v_string: *mut gchar) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_take_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        != 0
    {
        (*value).data[1 as ::core::ffi::c_int as usize].v_uint = 0 as guint;
    } else {
        g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = v_string as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_string(mut value: *const GValue) -> *const gchar {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_dup_string(mut value: *const GValue) -> *mut gchar {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_dup_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return g_strdup_inline(
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *const ::core::ffi::c_char,
    ) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_steal_string(mut value: *mut GValue) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_steal_string\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_STRING (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    ret = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
        != 0
    {
        return g_strdup_inline(ret) as *mut gchar;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_pointer(mut value: *mut GValue, mut v_pointer: gpointer) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_POINTER (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = v_pointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_pointer(mut value: *const GValue) -> gpointer {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_POINTER (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
#[inline(never)]
unsafe extern "C" fn g_gtype_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_pointer_type_register_static(g_intern_static_string(
        b"GType\0" as *const u8 as *const gchar,
    ));
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_gtype_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_gtype_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_gtype(mut value: *mut GValue, mut v_gtype: GType) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = g_gtype_get_type();
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
            b"g_value_set_gtype\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_GTYPE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = v_gtype as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_gtype(mut value: *const GValue) -> GType {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = g_gtype_get_type();
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
            b"g_value_get_gtype\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_GTYPE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as guintptr;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_variant(mut value: *mut GValue, mut variant: *mut GVariant) {
    let mut old_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_VARIANT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    old_variant = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if !variant.is_null() {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_variant_ref_sink(variant) as gpointer;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    if !old_variant.is_null() {
        g_variant_unref(old_variant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_take_variant(mut value: *mut GValue, mut variant: *mut GVariant) {
    let mut old_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_take_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_VARIANT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    old_variant = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if !variant.is_null() {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_variant_take_ref(variant) as gpointer;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    if !old_variant.is_null() {
        g_variant_unref(old_variant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_variant(mut value: *const GValue) -> *mut GVariant {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_VARIANT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_dup_variant(mut value: *const GValue) -> *mut GVariant {
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_dup_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_VARIANT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    variant = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if !variant.is_null() {
        g_variant_ref_sink(variant);
    }
    return variant;
}
#[no_mangle]
pub unsafe extern "C" fn g_strdup_value_contents(mut value: *const GValue) -> *mut gchar {
    let mut src: *const gchar = ::core::ptr::null::<gchar>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_type_check_value(value as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_strdup_value_contents\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
        src = g_value_get_string(value);
        if src.is_null() {
            contents =
                g_strdup_inline(b"NULL\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
        } else {
            let mut s: *mut gchar = g_strescape(src, ::core::ptr::null::<gchar>());
            contents = g_strdup_printf(b"\"%s\"\0" as *const u8 as *const gchar, s);
            g_free(s as gpointer);
        }
    } else if g_value_type_transformable(
        (*(value as *mut GValue)).g_type,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
        let mut tmp_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        let mut s_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        g_value_init(
            &raw mut tmp_value,
            ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        );
        g_value_transform(value, &raw mut tmp_value);
        s_0 = g_strescape(
            g_value_get_string(&raw mut tmp_value),
            ::core::ptr::null::<gchar>(),
        );
        g_value_unset(&raw mut tmp_value);
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
            || ({
                let mut __val: *const GValue = value;
                let mut __t: GType =
                    ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            contents = g_strdup_printf(
                b"((%s) %s)\0" as *const u8 as *const gchar,
                g_type_name((*(value as *mut GValue)).g_type),
                s_0,
            );
        } else {
            contents = g_strdup_inline(if !s_0.is_null() {
                s_0 as *const ::core::ffi::c_char
            } else {
                b"NULL\0" as *const u8 as *const ::core::ffi::c_char
            }) as *mut gchar;
        }
        g_free(s_0 as gpointer);
    } else if g_value_fits_pointer(value) != 0 {
        let mut p: gpointer = g_value_peek_pointer(value);
        if p.is_null() {
            contents =
                g_strdup_inline(b"NULL\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
        } else if ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            contents = g_strdup_printf(
                b"((%s*) %p)\0" as *const u8 as *const gchar,
                g_type_name((*(*(p as *mut GTypeInstance)).g_class).g_type),
                p,
            );
        } else if ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            contents = g_strdup_printf(
                b"((%s*) %p)\0" as *const u8 as *const gchar,
                g_type_name((*(*(p as *mut GTypeInstance)).g_class).g_type),
                p,
            );
        } else if ({
            let mut __val: *const GValue = value;
            let mut __t: GType = g_strv_get_type();
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
            let mut strv: GStrv = g_value_get_boxed(value) as GStrv;
            let mut tmp: *mut GString = g_string_new(b"[\0" as *const u8 as *const gchar);
            while !(*strv).is_null() {
                let mut escaped: *mut gchar = g_strescape(*strv, ::core::ptr::null::<gchar>());
                g_string_append_printf(tmp, b"\"%s\"\0" as *const u8 as *const gchar, escaped);
                g_free(escaped as gpointer);
                strv = strv.offset(1);
                if !(*strv).is_null() {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b", \0" as *const u8 as *const ::core::ffi::c_char;
                            g_string_append_len_inline(
                                tmp,
                                __val,
                                if !__val.is_null() {
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
                        g_string_append_len_inline(
                            tmp,
                            b", \0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"]\0" as *const u8 as *const ::core::ffi::c_char;
                    g_string_append_len_inline(
                        tmp,
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
                    tmp,
                    b"]\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            contents = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(tmp, 0 as gboolean)
                } else {
                    g_string_free_and_steal(tmp)
                }
            } else {
                g_string_free(tmp, 0 as gboolean)
            };
        } else if ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            contents = g_strdup_printf(
                b"((%s*) %p)\0" as *const u8 as *const gchar,
                g_type_name((*(value as *mut GValue)).g_type),
                p,
            );
        } else if ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            contents = g_strdup_printf(b"((gpointer) %p)\0" as *const u8 as *const gchar, p);
        } else {
            contents =
                g_strdup_inline(b"???\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
        }
    } else {
        contents =
            g_strdup_inline(b"???\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
    }
    return contents;
}
#[no_mangle]
pub unsafe extern "C" fn g_pointer_type_register_static(mut name: *const gchar) -> GType {
    let type_info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
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
            b"g_pointer_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if g_type_from_name(name) == 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_pointer_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_from_name (name) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    type_0 = g_type_register_static(
        ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw const type_info,
        G_TYPE_FLAG_NONE,
    );
    return type_0;
}
