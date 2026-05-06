extern "C" {
    pub type _GVariantType;
    pub type _GVariant;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_type_peek_string(type_0: *const GVariantType) -> *const gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_floating(value: *mut GVariant) -> gboolean;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_classify(value: *mut GVariant) -> GVariantClass;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_new_byte(value: guint8) -> *mut GVariant;
    fn g_variant_new_int16(value: gint16) -> *mut GVariant;
    fn g_variant_new_uint16(value: guint16) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_uint32(value: guint32) -> *mut GVariant;
    fn g_variant_new_int64(value: gint64) -> *mut GVariant;
    fn g_variant_new_uint64(value: guint64) -> *mut GVariant;
    fn g_variant_new_double(value: gdouble) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_object_path(object_path: *const gchar) -> *mut GVariant;
    fn g_variant_new_signature(signature: *const gchar) -> *mut GVariant;
    fn g_variant_new_strv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_new_objv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_new_bytestring(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_bytestring_array(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_get_byte(value: *mut GVariant) -> guint8;
    fn g_variant_get_int16(value: *mut GVariant) -> gint16;
    fn g_variant_get_uint16(value: *mut GVariant) -> guint16;
    fn g_variant_get_int32(value: *mut GVariant) -> gint32;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_get_int64(value: *mut GVariant) -> gint64;
    fn g_variant_get_uint64(value: *mut GVariant) -> guint64;
    fn g_variant_get_double(value: *mut GVariant) -> gdouble;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_dup_strv(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_dup_objv(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_get_bytestring(value: *mut GVariant) -> *const gchar;
    fn g_variant_dup_bytestring_array(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_get_normal_form(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_from_data(
        type_0: *const GVariantType,
        data: gconstpointer,
        size: gsize,
        trusted: gboolean,
        notify: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GVariant;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_random_int() -> guint32;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_strv_get_type() -> GType;
    fn g_value_take_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
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
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_dup_variant(value: *const GValue) -> *mut GVariant;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
pub type GVariantClass = ::core::ffi::c_uint;
pub const G_VARIANT_CLASS_DICT_ENTRY: GVariantClass = 123;
pub const G_VARIANT_CLASS_TUPLE: GVariantClass = 40;
pub const G_VARIANT_CLASS_ARRAY: GVariantClass = 97;
pub const G_VARIANT_CLASS_MAYBE: GVariantClass = 109;
pub const G_VARIANT_CLASS_VARIANT: GVariantClass = 118;
pub const G_VARIANT_CLASS_SIGNATURE: GVariantClass = 103;
pub const G_VARIANT_CLASS_OBJECT_PATH: GVariantClass = 111;
pub const G_VARIANT_CLASS_STRING: GVariantClass = 115;
pub const G_VARIANT_CLASS_DOUBLE: GVariantClass = 100;
pub const G_VARIANT_CLASS_HANDLE: GVariantClass = 104;
pub const G_VARIANT_CLASS_UINT64: GVariantClass = 116;
pub const G_VARIANT_CLASS_INT64: GVariantClass = 120;
pub const G_VARIANT_CLASS_UINT32: GVariantClass = 117;
pub const G_VARIANT_CLASS_INT32: GVariantClass = 105;
pub const G_VARIANT_CLASS_UINT16: GVariantClass = 113;
pub const G_VARIANT_CLASS_INT16: GVariantClass = 110;
pub const G_VARIANT_CLASS_BYTE: GVariantClass = 121;
pub const G_VARIANT_CLASS_BOOLEAN: GVariantClass = 98;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
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
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
}
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_UCHAR: GType = ((4 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = ((6 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT: GType = ((7 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT64: GType = ((10 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT64: GType = ((11 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_DOUBLE: GType = ((15 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
unsafe extern "C" fn safe_c2rust_is_valid_bus_name_character(
    mut c: gint,
    mut allow_hyphen: gboolean,
) -> gboolean {
    return (c >= '0' as i32 && c <= '9' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= 'a' as i32 && c <= 'z' as i32
        || c == '_' as i32
        || allow_hyphen != 0 && c == '-' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_is_valid_initial_bus_name_character(
    mut c: gint,
    mut allow_initial_digit: gboolean,
    mut allow_hyphen: gboolean,
) -> gboolean {
    if allow_initial_digit != 0 {
        return safe_c2rust_is_valid_bus_name_character(c, allow_hyphen);
    } else {
        return (c >= 'A' as i32 && c <= 'Z' as i32
            || c >= 'a' as i32 && c <= 'z' as i32
            || c == '_' as i32
            || allow_hyphen != 0 && c == '-' as i32) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn safe_c2rust_is_valid_name(
    mut start: *const gchar,
    mut len: guint,
    mut allow_initial_digit: gboolean,
    mut allow_hyphen: gboolean,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    let mut has_dot: gboolean = 0;
    ret = FALSE as gboolean;
    if !(len == 0 as guint) {
        s = start;
        end = s.offset(len as isize);
        has_dot = FALSE as gboolean;
        loop {
            if !(s != end) {
                current_block = 5399440093318478209;
                break;
            }
            if *s as ::core::ffi::c_int == '.' as i32 {
                s = s.offset(1 as ::core::ffi::c_int as isize);
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if safe_c2rust_is_valid_initial_bus_name_character(
                        *s as gint,
                        allow_initial_digit,
                        allow_hyphen,
                    ) == 0
                    {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 4813932883274277996;
                    break;
                }
                has_dot = TRUE as gboolean;
            } else if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if safe_c2rust_is_valid_bus_name_character(*s as gint, allow_hyphen) == 0 {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
                current_block = 4813932883274277996;
                break;
            }
            s = s.offset(1 as ::core::ffi::c_int as isize);
        }
        match current_block {
            4813932883274277996 => {}
            _ => {
                if !(({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if has_dot == 0 {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0)
                {
                    ret = TRUE as gboolean;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_name(mut string: *const gchar) -> gboolean {
    let mut len: guint = 0;
    let mut ret: gboolean = 0;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    len = strlen(string as *const ::core::ffi::c_char) as guint;
    if !(({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if len == 0 as guint || len > 255 as guint {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0)
    {
        s = string;
        if *s as ::core::ffi::c_int == ':' as i32 {
            if !(safe_c2rust_is_valid_name(
                s.offset(1 as ::core::ffi::c_int as isize),
                len.wrapping_sub(1 as guint),
                TRUE,
                TRUE,
            ) == 0)
            {
                ret = TRUE as gboolean;
            }
        } else if !(({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if *s as ::core::ffi::c_int == '.' as i32 {
                _g_boolean_var_15 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_15 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_15
        }) as ::core::ffi::c_long
            != 0)
        {
            if !(({
                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                if safe_c2rust_is_valid_initial_bus_name_character(
                    *s as gint,
                    0 as gboolean,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                ) == 0
                {
                    _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_16
            }) as ::core::ffi::c_long
                != 0)
            {
                ret = safe_c2rust_is_valid_name(
                    s.offset(1 as ::core::ffi::c_int as isize),
                    len.wrapping_sub(1 as guint),
                    FALSE,
                    TRUE,
                );
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_unique_name(mut string: *const gchar) -> gboolean {
    let mut ret: gboolean = 0;
    let mut len: guint = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    len = strlen(string as *const ::core::ffi::c_char) as guint;
    if !(({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if len == 0 as guint || len > 255 as guint {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0)
    {
        if !(({
            let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
            if *string as ::core::ffi::c_int != ':' as i32 {
                _g_boolean_var_19 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_19 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_19
        }) as ::core::ffi::c_long
            != 0)
        {
            if !(({
                let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                if safe_c2rust_is_valid_name(
                    string.offset(1 as ::core::ffi::c_int as isize),
                    len.wrapping_sub(1 as guint),
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                ) == 0
                {
                    _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_20
            }) as ::core::ffi::c_long
                != 0)
            {
                ret = TRUE as gboolean;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_member_name(mut string: *const gchar) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut n: guint = 0;
    ret = FALSE as gboolean;
    if !(({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if string.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0)
    {
        if !(({
            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
            if safe_c2rust_is_valid_initial_bus_name_character(
                *string.offset(0 as ::core::ffi::c_int as isize) as gint,
                0 as gboolean,
                0 as gboolean,
            ) == 0
            {
                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_22
        }) as ::core::ffi::c_long
            != 0)
        {
            n = 1 as guint;
            loop {
                if !(*string.offset(n as isize) as ::core::ffi::c_int != '\0' as i32) {
                    current_block = 11006700562992250127;
                    break;
                }
                if ({
                    let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                    if safe_c2rust_is_valid_bus_name_character(
                        *string.offset(n as isize) as gint,
                        0 as gboolean,
                    ) == 0
                    {
                        _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_23
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 9100736773296546949;
                    break;
                }
                n = n.wrapping_add(1);
            }
            match current_block {
                9100736773296546949 => {}
                _ => {
                    ret = TRUE as gboolean;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_interface_name(
    mut string: *const gchar,
) -> gboolean {
    let mut len: guint = 0;
    let mut ret: gboolean = 0;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    len = strlen(string as *const ::core::ffi::c_char) as guint;
    if !(({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if len == 0 as guint || len > 255 as guint {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0)
    {
        s = string;
        if !(({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if *s as ::core::ffi::c_int == '.' as i32 {
                _g_boolean_var_26 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_26 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_26
        }) as ::core::ffi::c_long
            != 0)
        {
            if !(({
                let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                if safe_c2rust_is_valid_initial_bus_name_character(
                    *s as gint,
                    0 as gboolean,
                    0 as gboolean,
                ) == 0
                {
                    _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_27
            }) as ::core::ffi::c_long
                != 0)
            {
                ret = safe_c2rust_is_valid_name(
                    s.offset(1 as ::core::ffi::c_int as isize),
                    len.wrapping_sub(1 as guint),
                    FALSE,
                    FALSE,
                );
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_error_name(mut string: *const gchar) -> gboolean {
    return safe_c2rust_g_dbus_is_interface_name(string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_generate_guid() -> *mut gchar {
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut r1: guint32 = 0;
    let mut r2: guint32 = 0;
    let mut r3: guint32 = 0;
    let mut now_us: gint64 = 0;
    s = g_string_new(::core::ptr::null::<gchar>());
    r1 = g_random_int();
    r2 = g_random_int();
    r3 = g_random_int();
    now_us = g_get_real_time();
    g_string_append_printf(s, b"%08x\0" as *const u8 as *const gchar, r1);
    g_string_append_printf(s, b"%08x\0" as *const u8 as *const gchar, r2);
    g_string_append_printf(s, b"%08x\0" as *const u8 as *const gchar, r3);
    g_string_append_printf(
        s,
        b"%08x\0" as *const u8 as *const gchar,
        (now_us / G_USEC_PER_SEC as gint64) as guint32,
    );
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(s, 0 as gboolean)
        } else {
            g_string_free_and_steal(s)
        }
    } else {
        g_string_free(s, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_guid(mut string: *const gchar) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut n: guint = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    n = 0 as guint;
    loop {
        if !(n < 32 as guint) {
            current_block = 13109137661213826276;
            break;
        }
        if !(*safe_c2rust_g_ascii_table.offset(*string.offset(n as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_XDIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
        {
            current_block = 580662529514833243;
            break;
        }
        n = n.wrapping_add(1);
    }
    match current_block {
        13109137661213826276 => {
            if !(*string.offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32)
            {
                ret = TRUE as gboolean;
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_gvariant_to_gvalue(
    mut value: *mut GVariant,
    mut out_gvalue: *mut GValue,
) {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut array: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !out_gvalue.is_null() {
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
            b"out_gvalue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        out_gvalue as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<GValue>() as size_t,
    );
    match g_variant_classify(value) as ::core::ffi::c_uint {
        98 => {
            g_value_init(out_gvalue, G_TYPE_BOOLEAN);
            g_value_set_boolean(out_gvalue, g_variant_get_boolean(value));
        }
        121 => {
            g_value_init(out_gvalue, G_TYPE_UCHAR);
            g_value_set_uchar(out_gvalue, g_variant_get_byte(value) as guchar);
        }
        110 => {
            g_value_init(out_gvalue, G_TYPE_INT);
            g_value_set_int(out_gvalue, g_variant_get_int16(value) as gint);
        }
        113 => {
            g_value_init(out_gvalue, G_TYPE_UINT);
            g_value_set_uint(out_gvalue, g_variant_get_uint16(value) as guint);
        }
        105 => {
            g_value_init(out_gvalue, G_TYPE_INT);
            g_value_set_int(out_gvalue, g_variant_get_int32(value) as gint);
        }
        117 => {
            g_value_init(out_gvalue, G_TYPE_UINT);
            g_value_set_uint(out_gvalue, g_variant_get_uint32(value) as guint);
        }
        120 => {
            g_value_init(out_gvalue, G_TYPE_INT64);
            g_value_set_int64(out_gvalue, g_variant_get_int64(value));
        }
        116 => {
            g_value_init(out_gvalue, G_TYPE_UINT64);
            g_value_set_uint64(out_gvalue, g_variant_get_uint64(value));
        }
        100 => {
            g_value_init(out_gvalue, G_TYPE_DOUBLE);
            g_value_set_double(out_gvalue, g_variant_get_double(value));
        }
        115 => {
            g_value_init(out_gvalue, G_TYPE_STRING);
            g_value_set_string(
                out_gvalue,
                g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
            );
        }
        111 => {
            g_value_init(out_gvalue, G_TYPE_STRING);
            g_value_set_string(
                out_gvalue,
                g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
            );
        }
        103 => {
            g_value_init(out_gvalue, G_TYPE_STRING);
            g_value_set_string(
                out_gvalue,
                g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
            );
        }
        97 => {
            type_0 = g_variant_get_type(value);
            match *g_variant_type_peek_string(type_0).offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
            {
                121 => {
                    g_value_init(out_gvalue, G_TYPE_STRING);
                    g_value_set_string(out_gvalue, g_variant_get_bytestring(value));
                }
                115 => {
                    g_value_init(out_gvalue, g_strv_get_type());
                    array = g_variant_dup_strv(value, ::core::ptr::null_mut::<gsize>());
                    g_value_take_boxed(out_gvalue, array as gconstpointer);
                }
                111 => {
                    g_value_init(out_gvalue, g_strv_get_type());
                    array = g_variant_dup_objv(value, ::core::ptr::null_mut::<gsize>());
                    g_value_take_boxed(out_gvalue, array as gconstpointer);
                }
                97 => {
                    match *g_variant_type_peek_string(type_0)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                    {
                        121 => {
                            g_value_init(out_gvalue, g_strv_get_type());
                            array = g_variant_dup_bytestring_array(
                                value,
                                ::core::ptr::null_mut::<gsize>(),
                            );
                            g_value_take_boxed(out_gvalue, array as gconstpointer);
                        }
                        _ => {
                            g_value_init(out_gvalue, G_TYPE_VARIANT);
                            g_value_set_variant(out_gvalue, value);
                        }
                    }
                }
                _ => {
                    g_value_init(out_gvalue, G_TYPE_VARIANT);
                    g_value_set_variant(out_gvalue, value);
                }
            }
        }
        104 | 118 | 109 | 40 | 123 => {
            g_value_init(out_gvalue, G_TYPE_VARIANT);
            g_value_set_variant(out_gvalue, value);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_gvalue_to_gvariant(
    mut gvalue: *const GValue,
    mut type_0: *const GVariantType,
) -> *mut GVariant {
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut as_0: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut empty_strv: [*const gchar; 1] = [::core::ptr::null::<gchar>()];
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !gvalue.is_null() {
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
            b"gvalue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    ret = ::core::ptr::null_mut::<GVariant>();
    if (*(gvalue as *mut GValue)).g_type == G_TYPE_VARIANT {
        ret = g_value_dup_variant(gvalue);
    } else {
        match *g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
        {
            98 => {
                ret = g_variant_ref_sink(g_variant_new_boolean(g_value_get_boolean(gvalue)));
            }
            121 => {
                ret = g_variant_ref_sink(g_variant_new_byte(g_value_get_uchar(gvalue) as guint8));
            }
            110 => {
                ret = g_variant_ref_sink(g_variant_new_int16(g_value_get_int(gvalue) as gint16));
            }
            113 => {
                ret = g_variant_ref_sink(g_variant_new_uint16(g_value_get_uint(gvalue) as guint16));
            }
            105 => {
                ret = g_variant_ref_sink(g_variant_new_int32(g_value_get_int(gvalue) as gint32));
            }
            117 => {
                ret = g_variant_ref_sink(g_variant_new_uint32(g_value_get_uint(gvalue) as guint32));
            }
            120 => {
                ret = g_variant_ref_sink(g_variant_new_int64(g_value_get_int64(gvalue)));
            }
            116 => {
                ret = g_variant_ref_sink(g_variant_new_uint64(g_value_get_uint64(gvalue)));
            }
            100 => {
                ret = g_variant_ref_sink(g_variant_new_double(g_value_get_double(gvalue)));
            }
            115 => {
                s = g_value_get_string(gvalue);
                if s.is_null() {
                    s = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                ret = g_variant_ref_sink(g_variant_new_string(s));
            }
            111 => {
                s = g_value_get_string(gvalue);
                if s.is_null() {
                    s = b"/\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                ret = g_variant_ref_sink(g_variant_new_object_path(s));
            }
            103 => {
                s = g_value_get_string(gvalue);
                if s.is_null() {
                    s = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                ret = g_variant_ref_sink(g_variant_new_signature(s));
            }
            97 => {
                match *g_variant_type_peek_string(type_0).offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                {
                    121 => {
                        s = g_value_get_string(gvalue);
                        if s.is_null() {
                            s = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                        }
                        ret = g_variant_ref_sink(g_variant_new_bytestring(s));
                    }
                    115 => {
                        as_0 = g_value_get_boxed(gvalue) as *const *const gchar;
                        if as_0.is_null() {
                            as_0 = &raw mut empty_strv as *mut *const gchar;
                        }
                        ret = g_variant_ref_sink(g_variant_new_strv(
                            as_0,
                            -(1 as ::core::ffi::c_int) as gssize,
                        ));
                    }
                    111 => {
                        as_0 = g_value_get_boxed(gvalue) as *const *const gchar;
                        if as_0.is_null() {
                            as_0 = &raw mut empty_strv as *mut *const gchar;
                        }
                        ret = g_variant_ref_sink(g_variant_new_objv(
                            as_0,
                            -(1 as ::core::ffi::c_int) as gssize,
                        ));
                    }
                    97 => {
                        match *g_variant_type_peek_string(type_0)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                        {
                            121 => {
                                as_0 = g_value_get_boxed(gvalue) as *const *const gchar;
                                if as_0.is_null() {
                                    as_0 = &raw mut empty_strv as *mut *const gchar;
                                }
                                ret = g_variant_ref_sink(g_variant_new_bytestring_array(
                                    as_0,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                ));
                            }
                            _ => {
                                ret = g_value_dup_variant(gvalue);
                            }
                        }
                    }
                    _ => {
                        ret = g_value_dup_variant(gvalue);
                    }
                }
            }
            104 | 118 | 109 | 40 | 123 => {
                ret = g_value_dup_variant(gvalue);
            }
            _ => {}
        }
    }
    if ret.is_null() {
        let mut untrusted_empty: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        untrusted_empty = g_variant_new_from_data(
            type_0,
            ::core::ptr::null::<::core::ffi::c_void>(),
            0 as gsize,
            FALSE,
            None,
            NULL_0,
        );
        ret = g_variant_take_ref(g_variant_get_normal_form(untrusted_empty));
        g_variant_unref(untrusted_empty);
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if g_variant_is_floating(ret) == 0 {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusutils.c\0" as *const u8
                as *const ::core::ffi::c_char,
            713 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!g_variant_is_floating (ret)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_escape_object_path_bytestring(
    mut bytes: *const guint8,
) -> *mut gchar {
    let mut escaped: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const guint8 = ::core::ptr::null::<guint8>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if *bytes as ::core::ffi::c_int == '\0' as i32 {
        return safe_c2rust_g_strdup_inline(b"_\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    escaped = g_string_new(::core::ptr::null::<gchar>());
    p = bytes;
    while *p != 0 {
        if *safe_c2rust_g_ascii_table.offset(*p as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            safe_c2rust_g_string_append_c_inline(escaped, *p as gchar);
        } else {
            g_string_append_printf(
                escaped,
                b"_%02x\0" as *const u8 as *const gchar,
                *p as ::core::ffi::c_int,
            );
        }
        p = p.offset(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(escaped, 0 as gboolean)
        } else {
            g_string_free_and_steal(escaped)
        }
    } else {
        g_string_free(escaped, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_escape_object_path(mut s: *const gchar) -> *mut gchar {
    return safe_c2rust_g_dbus_escape_object_path_bytestring(s as *const guint8);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_unescape_object_path(
    mut s: *const gchar,
) -> *mut guint8 {
    let mut unescaped: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !s.is_null() {
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
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guint8>();
    }
    if strcmp(
        s as *const ::core::ffi::c_char,
        b"_\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut guint8;
    }
    unescaped = g_string_new(::core::ptr::null::<gchar>());
    p = s;
    while *p != 0 {
        let mut hi: gint = 0;
        let mut lo: gint = 0;
        if *safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            safe_c2rust_g_string_append_c_inline(unescaped, *p);
        } else if *p as ::core::ffi::c_int == '_' as i32
            && {
                hi = g_ascii_xdigit_value(*p.offset(1 as ::core::ffi::c_int as isize));
                hi >= 0 as ::core::ffi::c_int
            }
            && {
                lo = g_ascii_xdigit_value(*p.offset(2 as ::core::ffi::c_int as isize));
                lo >= 0 as ::core::ffi::c_int
            }
            && (hi != 0 || lo != 0)
            && !(*safe_c2rust_g_ascii_table
                .offset((hi << 4 as ::core::ffi::c_int | lo) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_ALNUM as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            safe_c2rust_g_string_append_c_inline(
                unescaped,
                (hi << 4 as ::core::ffi::c_int | lo) as gchar,
            );
            p = p.offset(2 as ::core::ffi::c_int as isize);
        } else {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        unescaped,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal(unescaped);
                };
            } else {
                g_string_free(
                    unescaped,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
            return ::core::ptr::null_mut::<guint8>();
        }
        p = p.offset(1);
    }
    return (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(unescaped, 0 as gboolean)
        } else {
            g_string_free_and_steal(unescaped)
        }
    } else {
        g_string_free(unescaped, 0 as gboolean)
    }) as *mut guint8;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
