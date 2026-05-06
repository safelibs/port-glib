extern "C" {
    pub type _GVariantType;
    pub type _GVariant;
    fn g_free(mem: gpointer);
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_new_byte(value: guint8) -> *mut GVariant;
    fn g_variant_new_int16(value: gint16) -> *mut GVariant;
    fn g_variant_new_uint16(value: guint16) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_uint32(value: guint32) -> *mut GVariant;
    fn g_variant_new_int64(value: gint64) -> *mut GVariant;
    fn g_variant_new_uint64(value: guint64) -> *mut GVariant;
    fn g_variant_new_handle(value: gint32) -> *mut GVariant;
    fn g_variant_new_double(value: gdouble) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_object_path(object_path: *const gchar) -> *mut GVariant;
    fn g_variant_new_signature(signature: *const gchar) -> *mut GVariant;
    fn g_variant_new_strv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_new_bytestring(string: *const gchar) -> *mut GVariant;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_get_byte(value: *mut GVariant) -> guint8;
    fn g_variant_get_int16(value: *mut GVariant) -> gint16;
    fn g_variant_get_uint16(value: *mut GVariant) -> guint16;
    fn g_variant_get_int32(value: *mut GVariant) -> gint32;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_get_int64(value: *mut GVariant) -> gint64;
    fn g_variant_get_uint64(value: *mut GVariant) -> guint64;
    fn g_variant_get_handle(value: *mut GVariant) -> gint32;
    fn g_variant_get_double(value: *mut GVariant) -> gdouble;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_dup_strv(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_get_bytestring(value: *mut GVariant) -> *const gchar;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_type_class_peek(type_0: GType) -> gpointer;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_take_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_enum_get_value_by_nick(enum_class: *mut GEnumClass, nick: *const gchar)
        -> *mut GEnumValue;
    fn g_flags_get_first_value(flags_class: *mut GFlagsClass, value: guint) -> *mut GFlagsValue;
    fn g_flags_get_value_by_nick(
        flags_class: *mut GFlagsClass,
        nick: *const gchar,
    ) -> *mut GFlagsValue;
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_value_set_schar(value: *mut GValue, v_char: gint8);
    fn g_value_get_schar(value: *const GValue) -> gint8;
    fn g_value_set_uchar(value: *mut GValue, v_uchar: guchar);
    fn g_value_get_uchar(value: *const GValue) -> guchar;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_int64(value: *mut GValue, v_int64: gint64);
    fn g_value_get_int64(value: *const GValue) -> gint64;
    fn g_value_set_uint64(value: *mut GValue, v_uint64: guint64);
    fn g_value_get_uint64(value: *const GValue) -> guint64;
    fn g_value_set_double(value: *mut GValue, v_double: gdouble);
    fn g_value_get_double(value: *const GValue) -> gdouble;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
}
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MININT16: gint16 =
    (-(G_MAXINT16 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as gint16;
pub const G_MAXINT16: gint16 = 0x7fff as ::core::ffi::c_int as gint16;
pub const G_MAXUINT16: guint16 = 0xffff as ::core::ffi::c_int as guint16;
pub const G_MININT32: gint32 = -G_MAXINT32 - 1 as ::core::ffi::c_int;
pub const G_MAXINT32: gint32 = 0x7fffffff as ::core::ffi::c_int;
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
pub const G_MININT64: gint64 = -G_MAXINT64 - 1 as ::core::ffi::c_long;
pub const G_MAXINT64: ::core::ffi::c_long = 0x7fffffffffffffff as ::core::ffi::c_long;
pub const G_MAXUINT64: ::core::ffi::c_ulong = 0xffffffffffffffff as ::core::ffi::c_ulong;
pub const G_VARIANT_TYPE_BOOLEAN: *const GVariantType =
    b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTE: *const GVariantType =
    b"y\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT16: *const GVariantType =
    b"n\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT16: *const GVariantType =
    b"q\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT32: *const GVariantType =
    b"i\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT32: *const GVariantType =
    b"u\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT64: *const GVariantType =
    b"x\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT64: *const GVariantType =
    b"t\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_DOUBLE: *const GVariantType =
    b"d\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_OBJECT_PATH: *const GVariantType =
    b"o\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_SIGNATURE: *const GVariantType =
    b"g\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_HANDLE: *const GVariantType =
    b"h\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_CHAR: GType = ((3 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UCHAR: GType = ((4 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = ((6 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT: GType = ((7 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT64: GType = ((10 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT64: GType = ((11 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_ENUM: GType = ((12 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_FLAGS: GType = ((13 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_DOUBLE: GType = ((15 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
unsafe extern "C" fn safe_c2rust_g_settings_set_mapping_int(
    mut value: *const GValue,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut l: gint64 = 0;
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        l = g_value_get_int(value) as gint64;
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        l = g_value_get_int64(value);
    } else {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT16 as gconstpointer,
    ) != 0
    {
        if G_MININT16 as gint64 <= l && l <= G_MAXINT16 as gint64 {
            variant = g_variant_new_int16(l as gint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT16 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT16 as gint64 {
            variant = g_variant_new_uint16(l as guint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT32 as gconstpointer,
    ) != 0
    {
        if G_MININT32 as gint64 <= l && l <= G_MAXINT32 as gint64 {
            variant = g_variant_new_int32(l as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT32 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT32 as gint64 {
            variant = g_variant_new_uint32(l as guint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT64 as gconstpointer,
    ) != 0
    {
        if G_MININT64 <= l && l <= G_MAXINT64 {
            variant = g_variant_new_int64(l);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT64 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l as guint64 <= G_MAXUINT64 {
            variant = g_variant_new_uint64(l as guint64);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_HANDLE as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT32 as gint64 {
            variant = g_variant_new_handle(l as guint as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_DOUBLE as gconstpointer,
    ) != 0
    {
        variant = g_variant_new_double(l as gdouble);
    }
    return variant;
}
unsafe extern "C" fn safe_c2rust_g_settings_set_mapping_float(
    mut value: *const GValue,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut d: gdouble = 0.;
    let mut l: gint64 = 0;
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        d = g_value_get_double(value);
    } else {
        return ::core::ptr::null_mut::<GVariant>();
    }
    l = d as gint64;
    if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT16 as gconstpointer,
    ) != 0
    {
        if G_MININT16 as gint64 <= l && l <= G_MAXINT16 as gint64 {
            variant = g_variant_new_int16(l as gint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT16 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT16 as gint64 {
            variant = g_variant_new_uint16(l as guint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT32 as gconstpointer,
    ) != 0
    {
        if G_MININT32 as gint64 <= l && l <= G_MAXINT32 as gint64 {
            variant = g_variant_new_int32(l as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT32 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT32 as gint64 {
            variant = g_variant_new_uint32(l as guint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT64 as gconstpointer,
    ) != 0
    {
        if G_MININT64 <= l && l <= G_MAXINT64 {
            variant = g_variant_new_int64(l);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT64 as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l as guint64 <= G_MAXUINT64 {
            variant = g_variant_new_uint64(l as guint64);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_HANDLE as gconstpointer,
    ) != 0
    {
        if 0 as gint64 <= l && l <= G_MAXUINT32 as gint64 {
            variant = g_variant_new_handle(l as guint as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_DOUBLE as gconstpointer,
    ) != 0
    {
        variant = g_variant_new_double(d);
    }
    return variant;
}
unsafe extern "C" fn safe_c2rust_g_settings_set_mapping_unsigned_int(
    mut value: *const GValue,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut u: guint64 = 0;
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        u = g_value_get_uint(value) as guint64;
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        u = g_value_get_uint64(value);
    } else {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT16 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXINT16 as guint64 {
            variant = g_variant_new_int16(u as gint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT16 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXUINT16 as guint64 {
            variant = g_variant_new_uint16(u as guint16);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT32 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXINT32 as guint64 {
            variant = g_variant_new_int32(u as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT32 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXUINT32 as guint64 {
            variant = g_variant_new_uint32(u as guint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_INT64 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXINT64 as guint64 {
            variant = g_variant_new_int64(u as gint64);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_UINT64 as gconstpointer,
    ) != 0
    {
        if u <= G_MAXUINT64 {
            variant = g_variant_new_uint64(u);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_HANDLE as gconstpointer,
    ) != 0
    {
        if u <= G_MAXUINT32 as guint64 {
            variant = g_variant_new_handle(u as guint as gint32);
        }
    } else if g_variant_type_equal(
        expected_type as gconstpointer,
        G_VARIANT_TYPE_DOUBLE as gconstpointer,
    ) != 0
    {
        variant = g_variant_new_double(u as gdouble);
    }
    return variant;
}
unsafe extern "C" fn safe_c2rust_g_settings_get_mapping_int(
    mut value: *mut GValue,
    mut variant: *mut GVariant,
) -> gboolean {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut l: gint64 = 0;
    type_0 = g_variant_get_type(variant);
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_INT16 as gconstpointer,
    ) != 0
    {
        l = g_variant_get_int16(variant) as gint64;
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_INT32 as gconstpointer,
    ) != 0
    {
        l = g_variant_get_int32(variant) as gint64;
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_INT64 as gconstpointer,
    ) != 0
    {
        l = g_variant_get_int64(variant);
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_HANDLE as gconstpointer,
    ) != 0
    {
        l = g_variant_get_handle(variant) as gint64;
    } else {
        return FALSE;
    }
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int(value, l as gint);
        return (G_MININT32 as gint64 <= l && l <= G_MAXINT32 as gint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint(value, l as guint);
        return (0 as gint64 <= l && l <= G_MAXUINT32 as gint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int64(value, l);
        return (G_MININT64 <= l && l <= G_MAXINT64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint64(value, l as guint64);
        return (0 as gint64 <= l && l as guint64 <= G_MAXUINT64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_double(value, l as gdouble);
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_settings_get_mapping_float(
    mut value: *mut GValue,
    mut variant: *mut GVariant,
) -> gboolean {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut d: gdouble = 0.;
    let mut l: gint64 = 0;
    type_0 = g_variant_get_type(variant);
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_DOUBLE as gconstpointer,
    ) != 0
    {
        d = g_variant_get_double(variant);
    } else {
        return FALSE;
    }
    l = d as gint64;
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int(value, l as gint);
        return (G_MININT32 as gint64 <= l && l <= G_MAXINT32 as gint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint(value, l as guint);
        return (0 as gint64 <= l && l <= G_MAXUINT32 as gint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int64(value, l);
        return (G_MININT64 <= l && l <= G_MAXINT64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint64(value, l as guint64);
        return (0 as gint64 <= l && l as guint64 <= G_MAXUINT64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_double(value, d);
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_settings_get_mapping_unsigned_int(
    mut value: *mut GValue,
    mut variant: *mut GVariant,
) -> gboolean {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut u: guint64 = 0;
    type_0 = g_variant_get_type(variant);
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_UINT16 as gconstpointer,
    ) != 0
    {
        u = g_variant_get_uint16(variant) as guint64;
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_UINT32 as gconstpointer,
    ) != 0
    {
        u = g_variant_get_uint32(variant) as guint64;
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_UINT64 as gconstpointer,
    ) != 0
    {
        u = g_variant_get_uint64(variant);
    } else {
        return FALSE;
    }
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int(value, u as gint);
        return (u <= G_MAXINT32 as guint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint(value, u as guint);
        return (u <= G_MAXUINT32 as guint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_int64(value, u as gint64);
        return (u <= G_MAXINT64 as guint64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_uint64(value, u);
        return (u <= G_MAXUINT64) as ::core::ffi::c_int;
    } else if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_double(value, u as gdouble);
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_mapping(
    mut value: *const GValue,
    mut expected_type: *const GVariantType,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut type_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_BOOLEAN as gconstpointer,
        ) != 0
        {
            return g_variant_new_boolean(g_value_get_boolean(value));
        }
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
        || ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
    {
        if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_BYTE as gconstpointer,
        ) != 0
        {
            if ({
                let mut __val: *const GValue = value;
                let mut __t: GType =
                    ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
                let mut __r: gboolean = 0;
                if __val.is_null() {
                    __r = FALSE as gboolean;
                } else if (*__val).g_type == __t {
                    __r = TRUE as gboolean;
                } else {
                    __r = g_type_check_value_holds(__val, __t);
                }
                __r
            }) != 0
            {
                return g_variant_new_byte(g_value_get_schar(value) as guint8);
            } else {
                return g_variant_new_byte(g_value_get_uchar(value) as guint8);
            }
        }
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
        || ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
    {
        return safe_c2rust_g_settings_set_mapping_int(value, expected_type);
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        return safe_c2rust_g_settings_set_mapping_float(value, expected_type);
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
        || ({
            let mut __val: *const GValue = value;
            let mut __t: GType = ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
    {
        return safe_c2rust_g_settings_set_mapping_unsigned_int(value, expected_type);
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        if g_value_get_string(value).is_null() {
            return ::core::ptr::null_mut::<GVariant>();
        } else if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_STRING as gconstpointer,
        ) != 0
        {
            return g_variant_new_string(g_value_get_string(value));
        } else if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_BYTESTRING as gconstpointer,
        ) != 0
        {
            return g_variant_new_bytestring(g_value_get_string(value));
        } else if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_OBJECT_PATH as gconstpointer,
        ) != 0
        {
            return g_variant_new_object_path(g_value_get_string(value));
        } else if g_variant_type_equal(
            expected_type as gconstpointer,
            G_VARIANT_TYPE_SIGNATURE as gconstpointer,
        ) != 0
        {
            return g_variant_new_signature(g_value_get_string(value));
        }
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = g_strv_get_type();
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        if g_value_get_boxed(value).is_null() {
            return ::core::ptr::null_mut::<GVariant>();
        }
        return g_variant_new_strv(
            g_value_get_boxed(value) as *mut *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        let mut enumval: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
        let mut eclass: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
        eclass = g_type_class_peek((*(value as *mut GValue)).g_type) as *mut GEnumClass;
        enumval = g_enum_get_value(eclass, g_value_get_enum(value));
        if !enumval.is_null() {
            return g_variant_new_string((*enumval).value_nick);
        } else {
            return ::core::ptr::null_mut::<GVariant>();
        }
    } else if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = FALSE as gboolean;
        } else if (*__val).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        let mut builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut flagsval: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
        let mut fclass: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
        let mut flags: guint = 0;
        fclass = g_type_class_peek((*(value as *mut GValue)).g_type) as *mut GFlagsClass;
        flags = g_value_get_flags(value);
        g_variant_builder_init(
            &raw mut builder,
            g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
        );
        while flags != 0 {
            flagsval = g_flags_get_first_value(fclass, flags);
            if flagsval.is_null() {
                g_variant_builder_clear(&raw mut builder);
                return ::core::ptr::null_mut::<GVariant>();
            }
            g_variant_builder_add(
                &raw mut builder,
                b"s\0" as *const u8 as *const gchar,
                (*flagsval).value_nick,
            );
            flags &= !(*flagsval).value;
        }
        return g_variant_builder_end(&raw mut builder);
    }
    type_string = g_variant_type_dup_string(expected_type);
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_CRITICAL,
        b"No GSettings bind handler for type \"%s\".\0" as *const u8 as *const gchar,
        type_string,
    );
    g_free(type_string as gpointer);
    return ::core::ptr::null_mut::<GVariant>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_mapping(
    mut value: *mut GValue,
    mut variant: *mut GVariant,
    mut user_data: gpointer,
) -> gboolean {
    if g_variant_is_of_type(variant, G_VARIANT_TYPE_BOOLEAN) != 0 {
        if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) == 0
        {
            return FALSE;
        }
        g_value_set_boolean(value, g_variant_get_boolean(variant));
        return TRUE;
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_BYTE) != 0 {
        if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            g_value_set_uchar(value, g_variant_get_byte(variant) as guchar);
        } else if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            g_value_set_schar(value, g_variant_get_byte(variant) as gint8);
        } else {
            return FALSE;
        }
        return TRUE;
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_INT16) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_INT32) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_INT64) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_HANDLE) != 0
    {
        return safe_c2rust_g_settings_get_mapping_int(value, variant);
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_DOUBLE) != 0 {
        return safe_c2rust_g_settings_get_mapping_float(value, variant);
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_UINT16) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_UINT32) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_UINT64) != 0
    {
        return safe_c2rust_g_settings_get_mapping_unsigned_int(value, variant);
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_STRING) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_OBJECT_PATH) != 0
        || g_variant_is_of_type(variant, G_VARIANT_TYPE_SIGNATURE) != 0
    {
        if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            g_value_set_string(
                value,
                g_variant_get_string(variant, ::core::ptr::null_mut::<gsize>()),
            );
            return TRUE;
        } else if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            let mut eclass: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
            let mut evalue: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
            let mut nick: *const gchar = ::core::ptr::null::<gchar>();
            eclass = g_type_class_peek((*value).g_type) as *mut GEnumClass;
            nick = g_variant_get_string(variant, ::core::ptr::null_mut::<gsize>());
            evalue = g_enum_get_value_by_nick(eclass, nick);
            if !evalue.is_null() {
                g_value_set_enum(value, (*evalue).value);
                return TRUE;
            }
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unable to look up enum nick \xE2\x80\x98%s\xE2\x80\x99 via GType\0" as *const u8
                    as *const gchar,
                nick,
            );
            return FALSE;
        }
    } else if g_variant_is_of_type(
        variant,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    ) != 0
    {
        if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = g_strv_get_type();
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            g_value_take_boxed(
                value,
                g_variant_dup_strv(variant, ::core::ptr::null_mut::<gsize>()) as gconstpointer,
            );
            return TRUE;
        } else if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType = ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = FALSE as gboolean;
            } else if (*__val).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
            let mut fclass: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
            let mut fvalue: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
            let mut nick_0: *const gchar = ::core::ptr::null::<gchar>();
            let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
            let mut flags: guint = 0 as guint;
            fclass = g_type_class_peek((*value).g_type) as *mut GFlagsClass;
            g_variant_iter_init(&raw mut iter, variant);
            while g_variant_iter_next(
                &raw mut iter,
                b"&s\0" as *const u8 as *const gchar,
                &raw mut nick_0,
            ) != 0
            {
                fvalue = g_flags_get_value_by_nick(fclass, nick_0);
                if !fvalue.is_null() {
                    flags |= (*fvalue).value;
                } else {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"Unable to lookup flags nick '%s' via GType\0" as *const u8
                            as *const gchar,
                        nick_0,
                    );
                    return FALSE;
                }
            }
            g_value_set_flags(value, flags);
            return TRUE;
        }
    } else if g_variant_is_of_type(variant, G_VARIANT_TYPE_BYTESTRING) != 0 {
        g_value_set_string(value, g_variant_get_bytestring(variant));
        return TRUE;
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_CRITICAL,
        b"No GSettings bind handler for type \"%s\".\0" as *const u8 as *const gchar,
        g_variant_get_type_string(variant),
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_mapping_is_compatible(
    mut gvalue_type: GType,
    mut variant_type: *const GVariantType,
) -> gboolean {
    let mut ok: gboolean = FALSE;
    if gvalue_type == G_TYPE_BOOLEAN {
        ok = g_variant_type_equal(
            variant_type as gconstpointer,
            G_VARIANT_TYPE_BOOLEAN as gconstpointer,
        );
    } else if gvalue_type == G_TYPE_CHAR || gvalue_type == G_TYPE_UCHAR {
        ok = g_variant_type_equal(
            variant_type as gconstpointer,
            G_VARIANT_TYPE_BYTE as gconstpointer,
        );
    } else if gvalue_type == G_TYPE_INT
        || gvalue_type == G_TYPE_UINT
        || gvalue_type == G_TYPE_INT64
        || gvalue_type == G_TYPE_UINT64
        || gvalue_type == G_TYPE_DOUBLE
    {
        ok = (g_variant_type_equal(
            variant_type as gconstpointer,
            G_VARIANT_TYPE_INT16 as gconstpointer,
        ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_UINT16 as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_INT32 as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_UINT32 as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_INT64 as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_UINT64 as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_HANDLE as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_DOUBLE as gconstpointer,
            ) != 0) as ::core::ffi::c_int as gboolean;
    } else if gvalue_type == G_TYPE_STRING {
        ok = (g_variant_type_equal(
            variant_type as gconstpointer,
            G_VARIANT_TYPE_STRING as gconstpointer,
        ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                g_variant_type_checked_(b"ay\0" as *const u8 as *const gchar) as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_OBJECT_PATH as gconstpointer,
            ) != 0
            || g_variant_type_equal(
                variant_type as gconstpointer,
                G_VARIANT_TYPE_SIGNATURE as gconstpointer,
            ) != 0) as ::core::ffi::c_int as gboolean;
    } else if gvalue_type == g_strv_get_type() {
        ok = g_variant_type_equal(
            variant_type as gconstpointer,
            g_variant_type_checked_(b"as\0" as *const u8 as *const gchar) as gconstpointer,
        );
    } else if g_type_fundamental(gvalue_type) == G_TYPE_ENUM {
        ok = g_variant_type_equal(
            variant_type as gconstpointer,
            G_VARIANT_TYPE_STRING as gconstpointer,
        );
    } else if g_type_fundamental(gvalue_type) == G_TYPE_FLAGS {
        ok = g_variant_type_equal(
            variant_type as gconstpointer,
            g_variant_type_checked_(b"as\0" as *const u8 as *const gchar) as gconstpointer,
        );
    }
    return ok;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
