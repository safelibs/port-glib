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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_value_register_transform_func(
        src_type: GType,
        dest_type: GType,
        transform_func: GValueTransform,
    );
    fn g_flags_get_first_value(flags_class: *mut GFlagsClass, value: guint) -> *mut GFlagsValue;
    fn g_enum_to_string(g_enum_type: GType, value: gint) -> *mut gchar;
}
pub type size_t = usize;
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
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
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
pub type GValueTransform = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
pub type GFlagsClass = _GFlagsClass;
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
unsafe extern "C" fn value_transform_memcpy_data0(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    memcpy(
        (&raw mut (*dest_value).data as *mut C2RustUnnamed).offset(0 as ::core::ffi::c_int as isize)
            as *mut C2RustUnnamed as *mut ::core::ffi::c_void,
        (&raw const (*src_value).data as *const C2RustUnnamed)
            .offset(0 as ::core::ffi::c_int as isize) as *const C2RustUnnamed
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<C2RustUnnamed>() as size_t,
    );
}
unsafe extern "C" fn value_transform_int_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_int_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_int_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_int_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_int_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_int_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_int_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_int_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_int_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_uint_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_uint_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_uint_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_uint_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_uint_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_uint_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_uint_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_uint_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_uint_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_long_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_long_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_long_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_long_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_long_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_long_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_long_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_long_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_long_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_ulong_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_ulong_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_ulong_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_ulong_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_ulong_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_ulong_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_ulong_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_ulong_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat = (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_ulong_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_int64_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_int64_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_int64_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_int64_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_int64_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_int64_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_int64_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_int64_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat = (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_int64_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_uint64_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_uint64_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_uint64_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_uint64_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_uint64_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_uint64_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_uint64_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_uint64_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_uint64_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_float_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_float_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_float_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_float_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_float_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_float_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_float_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_float_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_float_double(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gdouble =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as gdouble;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_double = c_value;
}
unsafe extern "C" fn value_transform_double_s8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint8 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as gint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value as gint;
}
unsafe extern "C" fn value_transform_double_u8(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint8 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as guint8;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value as guint;
}
unsafe extern "C" fn value_transform_double_int(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as gint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int = c_value;
}
unsafe extern "C" fn value_transform_double_uint(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as guint;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint = c_value;
}
unsafe extern "C" fn value_transform_double_long(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: glong = (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as glong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_long = c_value;
}
unsafe extern "C" fn value_transform_double_ulong(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gulong =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as gulong;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_ulong = c_value;
}
unsafe extern "C" fn value_transform_double_int64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as gint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int64 = c_value;
}
unsafe extern "C" fn value_transform_double_uint64(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: guint64 =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as guint64;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_uint64 = c_value;
}
unsafe extern "C" fn value_transform_double_float(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut c_value: gfloat =
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double as gfloat;
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_float = c_value;
}
unsafe extern "C" fn value_transform_int_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_uint_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_uint != 0 as guint)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_long_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_long != 0 as glong)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_ulong_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong != 0 as gulong)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_int64_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_int64 != 0 as gint64)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_uint64_bool(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64 != 0 as guint64)
            as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn value_transform_int_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%d\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_int,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_uint_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%u\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_long_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%ld\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_long,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_ulong_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%lu\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_int64_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%li\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_int64,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_uint64_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%lu\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_uint64,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_float_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%f\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_float as ::core::ffi::c_double,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_double_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%f\0" as *const u8 as *const gchar,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_double,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_bool_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
        b"%s\0" as *const u8 as *const gchar,
        if (*src_value).data[0 as ::core::ffi::c_int as usize].v_int != 0 {
            b"TRUE\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"FALSE\0" as *const u8 as *const ::core::ffi::c_char
        },
    ) as gpointer;
}
unsafe extern "C" fn value_transform_string_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_inline(
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *const ::core::ffi::c_char,
    ) as gpointer;
}
unsafe extern "C" fn value_transform_enum_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut v_enum: gint = (*src_value).data[0 as ::core::ffi::c_int as usize].v_long as gint;
    let mut str: *mut gchar = g_enum_to_string((*(src_value as *mut GValue)).g_type, v_enum);
    (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = str as gpointer;
}
unsafe extern "C" fn value_transform_flags_string(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut class: *mut GFlagsClass =
        g_type_class_ref((*(src_value as *mut GValue)).g_type) as *mut GFlagsClass;
    let mut flags_value: *mut GFlagsValue = g_flags_get_first_value(
        class,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint,
    );
    if !flags_value.is_null() {
        let mut gstring: *mut GString = g_string_new(::core::ptr::null::<gchar>());
        let mut v_flags: guint =
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong as guint;
        loop {
            v_flags &= !(*flags_value).value;
            if *(*gstring).str_0.offset(0 as ::core::ffi::c_int as isize) != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b" | \0" as *const u8 as *const ::core::ffi::c_char;
                        g_string_append_len_inline(
                            gstring,
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
                        gstring,
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
                        gstring,
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
                    gstring,
                    (*flags_value).value_name as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            flags_value = g_flags_get_first_value(class, v_flags);
            if !(!flags_value.is_null() && v_flags != 0) {
                break;
            }
        }
        if v_flags != 0 {
            (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
                b"%s | %u\0" as *const u8 as *const gchar,
                (*gstring).str_0,
                v_flags,
            )
                as gpointer;
        } else {
            (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
                g_strdup_inline((*gstring).str_0) as gpointer;
        }
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
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_printf(
            b"%lu\0" as *const u8 as *const gchar,
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_ulong,
        ) as gpointer;
    }
    g_type_class_unref(class as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn _g_value_transforms_init() {
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_bool_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_ulong_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_ulong_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_ulong_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_int64_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_int64_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_int64_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_int64_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_bool as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_uint64_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_uint64_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_long_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_enum_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_ulong_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_ulong_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_flags_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_float_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_float_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_float_double as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_float_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_double_s8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_double_u8 as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_double_int as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_double_uint as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_transform_double_long as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_double_ulong as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_double_int64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_double_uint64 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_double_float as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_memcpy_data0 as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_double_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
    g_value_register_transform_func(
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            value_transform_string_string as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
}
