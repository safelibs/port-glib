extern "C" {
    pub type _GBytes;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GVariantTypeInfo;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_ascii_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
    fn g_ascii_strtoull(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> guint64;
    fn g_ascii_strtoll(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> gint64;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
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
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_vprintf(
        string: *mut GString,
        format: *const gchar,
        args: ::core::ffi::VaList,
    );
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_type_string_is_valid(type_string: *const gchar) -> gboolean;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_new(type_string: *const gchar) -> *mut GVariantType;
    fn g_variant_type_peek_string(type_0: *const GVariantType) -> *const gchar;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_is_definite(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_maybe(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_array(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_tuple(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_dict_entry(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_type_is_subtype_of(
        type_0: *const GVariantType,
        supertype: *const GVariantType,
    ) -> gboolean;
    fn g_variant_type_element(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_first(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_next(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_key(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_value(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_type_string_get_depth_(type_string: *const gchar) -> gsize;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
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
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_new_signature(signature: *const gchar) -> *mut GVariant;
    fn g_variant_is_signature(string: *const gchar) -> gboolean;
    fn g_variant_new_variant(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_bytestring(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_maybe(child_type: *const GVariantType, child: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_variant_store(value: *mut GVariant, data: gpointer);
    fn g_variant_new_from_bytes(
        type_0: *const GVariantType,
        bytes: *mut GBytes,
        trusted: gboolean,
    ) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_new_va(
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    ) -> *mut GVariant;
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
    fn g_variant_type_info_query(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_get(type_0: *const GVariantType) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_unref(typeinfo: *mut GVariantTypeInfo);
    fn g_variant_is_trusted(value: *mut GVariant) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
pub type va_list = __builtin_va_list;
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
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type gunichar = guint32;
pub type GBytes = _GBytes;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: C2RustUnnamed_1,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_VARIANT_PARSE_ERROR_RECURSION: C2RustUnnamed_2 = 18;
pub const G_VARIANT_PARSE_ERROR_VALUE_EXPECTED: C2RustUnnamed_2 = 17;
pub const G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT: C2RustUnnamed_2 = 16;
pub const G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD: C2RustUnnamed_2 = 15;
pub const G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN: C2RustUnnamed_2 = 14;
pub const G_VARIANT_PARSE_ERROR_TYPE_ERROR: C2RustUnnamed_2 = 13;
pub const G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG: C2RustUnnamed_2 = 12;
pub const G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE: C2RustUnnamed_2 = 11;
pub const G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE: C2RustUnnamed_2 = 10;
pub const G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING: C2RustUnnamed_2 = 9;
pub const G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE: C2RustUnnamed_2 = 8;
pub const G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH: C2RustUnnamed_2 = 7;
pub const G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING: C2RustUnnamed_2 = 6;
pub const G_VARIANT_PARSE_ERROR_INVALID_CHARACTER: C2RustUnnamed_2 = 5;
pub const G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END: C2RustUnnamed_2 = 4;
pub const G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED: C2RustUnnamed_2 = 3;
pub const G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE: C2RustUnnamed_2 = 2;
pub const G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED: C2RustUnnamed_2 = 1;
pub const G_VARIANT_PARSE_ERROR_FAILED: C2RustUnnamed_2 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenStream {
    pub start: *const gchar,
    pub stream: *const gchar,
    pub end: *const gchar,
    pub this: *const gchar,
}
pub type AST = _AST;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AST {
    pub class: *const ASTClass,
    pub source_ref: SourceRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SourceRef {
    pub start: size_t,
    pub end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ASTClass {
    pub get_pattern: Option<unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar>,
    pub get_value: Option<
        unsafe extern "C" fn(*mut AST, *const GVariantType, *mut *mut GError) -> *mut GVariant,
    >,
    pub get_base_value: Option<
        unsafe extern "C" fn(*mut AST, *const GVariantType, *mut *mut GError) -> *mut GVariant,
    >,
    pub free: Option<unsafe extern "C" fn(*mut AST) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ByteString {
    pub ast: AST,
    pub string: *mut gchar,
}
pub type GVariantTypeInfo = _GVariantTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct String_0 {
    pub ast: AST,
    pub string: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeDecl {
    pub ast: AST,
    pub type_0: *mut GVariantType,
    pub child: *mut AST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Maybe {
    pub ast: AST,
    pub child: *mut AST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Number {
    pub ast: AST,
    pub token: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Boolean {
    pub ast: AST,
    pub value: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Positional {
    pub ast: AST,
    pub value: *mut GVariant,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dictionary {
    pub ast: AST,
    pub keys: *mut *mut AST,
    pub values: *mut *mut AST,
    pub n_children: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Variant {
    pub ast: AST,
    pub value: *mut AST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tuple {
    pub ast: AST,
    pub children: *mut *mut AST,
    pub n_children: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub ast: AST,
    pub children: *mut *mut AST,
    pub n_children: size_t,
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"parser_set_error_va\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXUINT8: guint8 = 0xff as ::core::ffi::c_int as guint8;
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
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
pub const G_VARIANT_TYPE_VARIANT: *const GVariantType =
    b"v\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_HANDLE: *const GVariantType =
    b"h\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_DICTIONARY: *const GVariantType =
    b"a{?*}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_MAX_RECURSION_DEPTH: gsize = 128 as ::core::ffi::c_int as gsize;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_parse_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(
            b"g-variant-parse-error-quark\0" as *const u8 as *const gchar,
        );
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_parser_get_error_quark() -> GQuark {
    return safe_c2rust_g_variant_parse_error_quark();
}
unsafe extern "C" fn safe_c2rust_parser_set_error_va(
    mut error: *mut *mut GError,
    mut location: *mut SourceRef,
    mut other: *mut SourceRef,
    mut code: gint,
    mut format: *const gchar,
    mut ap: ::core::ffi::VaList,
) {
    let mut msg: *mut GString = g_string_new(::core::ptr::null::<gchar>());
    if (*location).start == (*location).end {
        g_string_append_printf(
            msg,
            b"%lu\0" as *const u8 as *const gchar,
            (*location).start,
        );
    } else {
        g_string_append_printf(
            msg,
            b"%lu-%lu\0" as *const u8 as *const gchar,
            (*location).start,
            (*location).end,
        );
    }
    if !other.is_null() {
        if ({
            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
            if (*other).start != (*other).end {
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
                b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int,
                G_STRFUNC,
                b"other->start != other->end\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_string_append_printf(
            msg,
            b",%lu-%lu\0" as *const u8 as *const gchar,
            (*other).start,
            (*other).end,
        );
    }
    safe_c2rust_g_string_append_c_inline(msg, ':' as i32 as gchar);
    g_string_append_vprintf(msg, format, ap.clone());
    g_set_error_literal(
        error,
        safe_c2rust_g_variant_parse_error_quark(),
        code,
        (*msg).str_0,
    );
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(msg, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(msg);
        };
    } else {
        g_string_free(msg, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn safe_c2rust_parser_set_error(
    mut error: *mut *mut GError,
    mut location: *mut SourceRef,
    mut other: *mut SourceRef,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    safe_c2rust_parser_set_error_va(error, location, other, code, format, ap.clone());
}
unsafe extern "C" fn safe_c2rust_token_stream_set_error(
    mut stream: *mut TokenStream,
    mut error: *mut *mut GError,
    mut this_token: gboolean,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut ref_0: SourceRef = SourceRef { start: 0, end: 0 };
    let mut ap: ::core::ffi::VaList;
    ref_0.start = (*stream).this.offset_from((*stream).start) as ::core::ffi::c_long as size_t;
    if this_token != 0 {
        ref_0.end = (*stream).stream.offset_from((*stream).start) as ::core::ffi::c_long as size_t;
    } else {
        ref_0.end = ref_0.start;
    }
    ap = args.clone();
    safe_c2rust_parser_set_error_va(
        error,
        &raw mut ref_0,
        ::core::ptr::null_mut::<SourceRef>(),
        code,
        format,
        ap.clone(),
    );
}
unsafe extern "C" fn safe_c2rust_token_stream_prepare(mut stream: *mut TokenStream) -> gboolean {
    let mut brackets: gssize = 0 as gssize;
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    if !(*stream).this.is_null() {
        return TRUE;
    }
    while (*stream).stream != (*stream).end
        && *safe_c2rust_g_ascii_table.offset(*(*stream).stream as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        (*stream).stream = (*stream).stream.offset(1);
    }
    if (*stream).stream == (*stream).end || *(*stream).stream as ::core::ffi::c_int == '\0' as i32 {
        (*stream).this = (*stream).stream;
        return FALSE;
    }
    let mut current_block_24: u64;
    match *(*stream).stream.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        45 | 43 | 46 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            end = (*stream).stream;
            while end != (*stream).end {
                if !(*safe_c2rust_g_ascii_table.offset(*end as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_ALNUM as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
                    && *end as ::core::ffi::c_int != '-' as i32
                    && *end as ::core::ffi::c_int != '+' as i32
                    && *end as ::core::ffi::c_int != '.' as i32
                {
                    break;
                }
                end = end.offset(1);
            }
            current_block_24 = 1423531122933789233;
        }
        98 => {
            if (*stream).stream.offset(1 as ::core::ffi::c_int as isize) != (*stream).end
                && (*(*stream).stream.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\'' as i32
                    || *(*stream).stream.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '"' as i32)
            {
                end = (*stream).stream.offset(2 as ::core::ffi::c_int as isize);
                while end != (*stream).end {
                    if *end as ::core::ffi::c_int
                        == *(*stream).stream.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                        || *end as ::core::ffi::c_int == '\0' as i32
                        || *end as ::core::ffi::c_int == '\\' as i32 && {
                            end = end.offset(1);
                            end == (*stream).end || *end as ::core::ffi::c_int == '\0' as i32
                        }
                    {
                        break;
                    }
                    end = end.offset(1);
                }
                if end != (*stream).end && *end as ::core::ffi::c_int != 0 {
                    end = end.offset(1);
                }
                current_block_24 = 1423531122933789233;
            } else {
                current_block_24 = 9114759541800921738;
            }
        }
        97 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112
        | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            current_block_24 = 9114759541800921738;
        }
        39 | 34 => {
            end = (*stream).stream.offset(1 as ::core::ffi::c_int as isize);
            while end != (*stream).end {
                if *end as ::core::ffi::c_int
                    == *(*stream).stream.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                    || *end as ::core::ffi::c_int == '\0' as i32
                    || *end as ::core::ffi::c_int == '\\' as i32 && {
                        end = end.offset(1);
                        end == (*stream).end || *end as ::core::ffi::c_int == '\0' as i32
                    }
                {
                    break;
                }
                end = end.offset(1);
            }
            if end != (*stream).end && *end as ::core::ffi::c_int != 0 {
                end = end.offset(1);
            }
            current_block_24 = 1423531122933789233;
        }
        64 | 37 => {
            end = (*stream).stream.offset(1 as ::core::ffi::c_int as isize);
            while end != (*stream).end
                && *end as ::core::ffi::c_int != '\0' as i32
                && *end as ::core::ffi::c_int != ',' as i32
                && *end as ::core::ffi::c_int != ':' as i32
                && *end as ::core::ffi::c_int != '>' as i32
                && *end as ::core::ffi::c_int != ']' as i32
                && !(*safe_c2rust_g_ascii_table.offset(*end as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_SPACE as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
            {
                if *end as ::core::ffi::c_int == '(' as i32
                    || *end as ::core::ffi::c_int == '{' as i32
                {
                    brackets += 1;
                } else if (*end as ::core::ffi::c_int == ')' as i32
                    || *end as ::core::ffi::c_int == '}' as i32)
                    && {
                        let fresh19 = brackets;
                        brackets = brackets - 1;
                        fresh19 == 0
                    }
                {
                    break;
                }
                end = end.offset(1);
            }
            current_block_24 = 1423531122933789233;
        }
        _ => {
            end = (*stream).stream.offset(1 as ::core::ffi::c_int as isize);
            current_block_24 = 1423531122933789233;
        }
    }
    match current_block_24 {
        9114759541800921738 => {
            end = (*stream).stream;
            while end != (*stream).end {
                if !(*safe_c2rust_g_ascii_table.offset(*end as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_ALNUM as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
                {
                    break;
                }
                end = end.offset(1);
            }
        }
        _ => {}
    }
    (*stream).this = (*stream).stream;
    (*stream).stream = end;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
            >= 1 as ::core::ffi::c_long
        {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            275 as ::core::ffi::c_int,
            G_STRFUNC,
            b"stream->stream - stream->this >= 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_token_stream_next(mut stream: *mut TokenStream) {
    (*stream).this = ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn safe_c2rust_token_stream_peek(
    mut stream: *mut TokenStream,
    mut first_char: gchar,
) -> gboolean {
    if safe_c2rust_token_stream_prepare(stream) == 0 {
        return FALSE;
    }
    return ((*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
        >= 1 as ::core::ffi::c_long
        && *(*stream).this.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == first_char as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_token_stream_peek2(
    mut stream: *mut TokenStream,
    mut first_char: gchar,
    mut second_char: gchar,
) -> gboolean {
    if safe_c2rust_token_stream_prepare(stream) == 0 {
        return FALSE;
    }
    return ((*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
        >= 2 as ::core::ffi::c_long
        && *(*stream).this.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == first_char as ::core::ffi::c_int
        && *(*stream).this.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == second_char as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_token_stream_is_keyword(mut stream: *mut TokenStream) -> gboolean {
    if safe_c2rust_token_stream_prepare(stream) == 0 {
        return FALSE;
    }
    return ((*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
        >= 2 as ::core::ffi::c_long
        && *safe_c2rust_g_ascii_table
            .offset(*(*stream).this.offset(0 as ::core::ffi::c_int as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_ALPHA as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        && *safe_c2rust_g_ascii_table
            .offset(*(*stream).this.offset(1 as ::core::ffi::c_int as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_ALPHA as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_token_stream_is_numeric(mut stream: *mut TokenStream) -> gboolean {
    if safe_c2rust_token_stream_prepare(stream) == 0 {
        return FALSE;
    }
    return ((*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
        >= 1 as ::core::ffi::c_long
        && (*safe_c2rust_g_ascii_table
            .offset(*(*stream).this.offset(0 as ::core::ffi::c_int as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_DIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || *(*stream).this.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
            || *(*stream).this.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32
            || *(*stream).this.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_token_stream_peek_string(
    mut stream: *mut TokenStream,
    mut token: *const gchar,
) -> gboolean {
    let mut length: gint = strlen(token as *const ::core::ffi::c_char) as gint;
    return (safe_c2rust_token_stream_prepare(stream) != 0
        && (*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long
            == length as ::core::ffi::c_long
        && memcmp(
            (*stream).this as *const ::core::ffi::c_void,
            token as *const ::core::ffi::c_void,
            length as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_token_stream_consume(
    mut stream: *mut TokenStream,
    mut token: *const gchar,
) -> gboolean {
    if safe_c2rust_token_stream_peek_string(stream, token) == 0 {
        return FALSE;
    }
    safe_c2rust_token_stream_next(stream);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_token_stream_require(
    mut stream: *mut TokenStream,
    mut token: *const gchar,
    mut purpose: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    if safe_c2rust_token_stream_consume(stream, token) == 0 {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            FALSE,
            G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN as ::core::ffi::c_int as gint,
            b"expected '%s'%s\0" as *const u8 as *const gchar,
            token,
            purpose,
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_token_stream_assert(
    mut stream: *mut TokenStream,
    mut token: *const gchar,
) {
    let mut correct_token: gboolean = 0;
    correct_token = safe_c2rust_token_stream_consume(stream, token);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if correct_token != 0 {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            381 as ::core::ffi::c_int,
            G_STRFUNC,
            b"correct_token\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_token_stream_get(mut stream: *mut TokenStream) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if safe_c2rust_token_stream_prepare(stream) == 0 {
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_strndup(
        (*stream).this,
        (*stream).stream.offset_from((*stream).this) as ::core::ffi::c_long as gsize,
    );
    return result;
}
unsafe extern "C" fn safe_c2rust_token_stream_start_ref(
    mut stream: *mut TokenStream,
    mut ref_0: *mut SourceRef,
) {
    safe_c2rust_token_stream_prepare(stream);
    (*ref_0).start = (*stream).this.offset_from((*stream).start) as ::core::ffi::c_long as size_t;
}
unsafe extern "C" fn safe_c2rust_token_stream_end_ref(
    mut stream: *mut TokenStream,
    mut ref_0: *mut SourceRef,
) {
    (*ref_0).end = (*stream).stream.offset_from((*stream).start) as ::core::ffi::c_long as size_t;
}
unsafe extern "C" fn safe_c2rust_pattern_copy(
    mut out: *mut *mut gchar,
    mut in_0: *mut *const gchar,
) {
    let mut brackets: gssize = 0 as gssize;
    while **in_0 as ::core::ffi::c_int == 'a' as i32
        || **in_0 as ::core::ffi::c_int == 'm' as i32
        || **in_0 as ::core::ffi::c_int == 'M' as i32
    {
        let fresh41 = *in_0;
        *in_0 = (*in_0).offset(1);
        let fresh42 = *out;
        *out = (*out).offset(1);
        *fresh42 = *fresh41;
    }
    loop {
        if **in_0 as ::core::ffi::c_int == '(' as i32 || **in_0 as ::core::ffi::c_int == '{' as i32
        {
            brackets += 1;
        } else if **in_0 as ::core::ffi::c_int == ')' as i32
            || **in_0 as ::core::ffi::c_int == '}' as i32
        {
            brackets -= 1;
        }
        let fresh43 = *in_0;
        *in_0 = (*in_0).offset(1);
        let fresh44 = *out;
        *out = (*out).offset(1);
        *fresh44 = *fresh43;
        if !(brackets != 0) {
            break;
        }
    }
}
unsafe extern "C" fn safe_c2rust_pattern_coalesce(
    mut left: *const gchar,
    mut right: *const gchar,
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    result = g_malloc(
        (strlen(left as *const ::core::ffi::c_char) as gsize)
            .wrapping_add(strlen(right as *const ::core::ffi::c_char) as gsize),
    ) as *mut gchar;
    out = result;
    's_8: while *left as ::core::ffi::c_int != 0 && *right as ::core::ffi::c_int != 0 {
        if *left as ::core::ffi::c_int == *right as ::core::ffi::c_int {
            let fresh32 = left;
            left = left.offset(1);
            let fresh33 = out;
            out = out.offset(1);
            *fresh33 = *fresh32;
            right = right.offset(1);
        } else {
            let mut one: *mut *const gchar = &raw mut left;
            let mut the_other: *mut *const gchar = &raw mut right;
            loop {
                if **one as ::core::ffi::c_int == '*' as i32
                    && **the_other as ::core::ffi::c_int != ')' as i32
                {
                    safe_c2rust_pattern_copy(&raw mut out, the_other);
                    *one = (*one).offset(1);
                    break;
                } else if **one as ::core::ffi::c_int == 'M' as i32
                    && **the_other as ::core::ffi::c_int == 'm' as i32
                {
                    let fresh34 = *the_other;
                    *the_other = (*the_other).offset(1);
                    let fresh35 = out;
                    out = out.offset(1);
                    *fresh35 = *fresh34;
                    break;
                } else if **one as ::core::ffi::c_int == 'M' as i32
                    && **the_other as ::core::ffi::c_int != 'm' as i32
                    && **the_other as ::core::ffi::c_int != '*' as i32
                {
                    *one = (*one).offset(1);
                    break;
                } else if **one as ::core::ffi::c_int == 'N' as i32
                    && !strchr(
                        b"ynqiuxthd\0" as *const u8 as *const ::core::ffi::c_char,
                        **the_other as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    let fresh36 = *the_other;
                    *the_other = (*the_other).offset(1);
                    let fresh37 = out;
                    out = out.offset(1);
                    *fresh37 = *fresh36;
                    *one = (*one).offset(1);
                    break;
                } else if **one as ::core::ffi::c_int == 'S' as i32
                    && !strchr(
                        b"sog\0" as *const u8 as *const ::core::ffi::c_char,
                        **the_other as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    let fresh38 = *the_other;
                    *the_other = (*the_other).offset(1);
                    let fresh39 = out;
                    out = out.offset(1);
                    *fresh39 = *fresh38;
                    *one = (*one).offset(1);
                    break;
                } else {
                    if !(one == &raw mut left) {
                        break 's_8;
                    }
                    one = &raw mut right;
                    the_other = &raw mut left;
                }
            }
        }
    }
    if *left as ::core::ffi::c_int != 0 || *right as ::core::ffi::c_int != 0 {
        g_free(result as gpointer);
        result = ::core::ptr::null_mut::<gchar>();
    } else {
        let fresh40 = out;
        out = out.offset(1);
        *fresh40 = '\0' as i32 as gchar;
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_ast_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    return (*(*ast).class)
        .get_pattern
        .expect("non-null function pointer")(ast, error);
}
unsafe extern "C" fn safe_c2rust_ast_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return (*(*ast).class)
        .get_value
        .expect("non-null function pointer")(ast, type_0, error);
}
unsafe extern "C" fn safe_c2rust_ast_free(mut ast: *mut AST) {
    (*(*ast).class).free.expect("non-null function pointer")(ast);
}
unsafe extern "C" fn safe_c2rust_ast_set_error(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
    mut other_ast: *mut AST,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    safe_c2rust_parser_set_error_va(
        error,
        &raw mut (*ast).source_ref,
        if !other_ast.is_null() {
            &raw mut (*other_ast).source_ref
        } else {
            ::core::ptr::null_mut::<SourceRef>()
        },
        code,
        format,
        ap.clone(),
    );
}
unsafe extern "C" fn safe_c2rust_ast_type_error(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut typestr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    typestr = g_variant_type_dup_string(type_0);
    safe_c2rust_ast_set_error(
        ast,
        error,
        ::core::ptr::null_mut::<AST>(),
        G_VARIANT_PARSE_ERROR_TYPE_ERROR as ::core::ffi::c_int as gint,
        b"can not parse as value of type '%s'\0" as *const u8 as *const gchar,
        typestr,
    );
    g_free(typestr as gpointer);
    return ::core::ptr::null_mut::<GVariant>();
}
unsafe extern "C" fn safe_c2rust_ast_resolve(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    let mut j: size_t = 0 as size_t;
    pattern = safe_c2rust_ast_get_pattern(ast, error);
    if pattern.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    i = 0 as size_t;
    while *pattern.offset(i as isize) != 0 {
        match *pattern.offset(i as isize) as ::core::ffi::c_int {
            42 => {
                safe_c2rust_ast_set_error(
                    ast,
                    error,
                    ::core::ptr::null_mut::<AST>(),
                    G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE as ::core::ffi::c_int as gint,
                    b"unable to infer type\0" as *const u8 as *const gchar,
                );
                g_free(pattern as gpointer);
                return ::core::ptr::null_mut::<GVariant>();
            }
            77 => {}
            83 => {
                let fresh1 = j;
                j = j.wrapping_add(1);
                *pattern.offset(fresh1 as isize) = 's' as i32 as gchar;
            }
            78 => {
                let fresh2 = j;
                j = j.wrapping_add(1);
                *pattern.offset(fresh2 as isize) = 'i' as i32 as gchar;
            }
            _ => {
                let fresh3 = j;
                j = j.wrapping_add(1);
                *pattern.offset(fresh3 as isize) = *pattern.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    let fresh4 = j;
    j = j.wrapping_add(1);
    *pattern.offset(fresh4 as isize) = '\0' as i32 as gchar;
    value = safe_c2rust_ast_get_value(ast, g_variant_type_checked_(pattern), error);
    g_free(pattern as gpointer);
    return value;
}
unsafe extern "C" fn safe_c2rust_ast_array_append(
    mut array: *mut *mut *mut AST,
    mut n_items: *mut size_t,
    mut ast: *mut AST,
) {
    if *n_items & (*n_items).wrapping_sub(1 as size_t) == 0 as size_t {
        *array = ({
            let mut __n: gsize = (if *n_items != 0 {
                (2 as size_t).wrapping_mul(*n_items)
            } else {
                1 as size_t
            }) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut AST>() as gsize;
            let mut __p: gpointer = *array as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut *mut AST;
    }
    let fresh30 = *n_items;
    *n_items = (*n_items).wrapping_add(1);
    let ref mut fresh31 = *(*array).offset(fresh30 as isize);
    *fresh31 = ast;
}
unsafe extern "C" fn safe_c2rust_ast_array_free(mut array: *mut *mut AST, mut n_items: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_items {
        safe_c2rust_ast_free(*array.offset(i as isize));
        i = i.wrapping_add(1);
    }
    g_free(array as gpointer);
}
unsafe extern "C" fn safe_c2rust_ast_array_get_pattern(
    mut array: *mut *mut AST,
    mut n_items: size_t,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    pattern = safe_c2rust_ast_get_pattern(*array.offset(0 as ::core::ffi::c_int as isize), error);
    if pattern.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    i = 1 as size_t;
    while i < n_items {
        let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut merged: *mut gchar = ::core::ptr::null_mut::<gchar>();
        tmp = safe_c2rust_ast_get_pattern(*array.offset(i as isize), error);
        if tmp.is_null() {
            g_free(pattern as gpointer);
            return ::core::ptr::null_mut::<gchar>();
        }
        merged = safe_c2rust_pattern_coalesce(pattern, tmp);
        g_free(pattern as gpointer);
        pattern = merged;
        if merged.is_null() {
            let mut j: size_t = 0 as size_t;
            while FALSE == 0 {
                let mut tmp2: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut m: *mut gchar = ::core::ptr::null_mut::<gchar>();
                if j >= i {
                    safe_c2rust_ast_set_error(
                        *array.offset(i as isize),
                        error,
                        ::core::ptr::null_mut::<AST>(),
                        G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE as ::core::ffi::c_int as gint,
                        b"unable to find a common type\0" as *const u8 as *const gchar,
                    );
                    g_free(tmp as gpointer);
                    return ::core::ptr::null_mut::<gchar>();
                }
                tmp2 = safe_c2rust_ast_get_pattern(
                    *array.offset(j as isize),
                    ::core::ptr::null_mut::<*mut GError>(),
                );
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if !tmp2.is_null() {
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
                        b"../original/glib/gvariant-parser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        741 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"tmp2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                m = safe_c2rust_pattern_coalesce(tmp, tmp2);
                g_free(tmp2 as gpointer);
                g_free(m as gpointer);
                if m.is_null() {
                    safe_c2rust_ast_set_error(
                        *array.offset(j as isize),
                        error,
                        *array.offset(i as isize),
                        G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE as ::core::ffi::c_int as gint,
                        b"unable to find a common type\0" as *const u8 as *const gchar,
                    );
                    g_free(tmp as gpointer);
                    return ::core::ptr::null_mut::<gchar>();
                }
                j = j.wrapping_add(1);
            }
        }
        g_free(tmp as gpointer);
        i = i.wrapping_add(1);
    }
    return pattern;
}
unsafe extern "C" fn safe_c2rust_maybe_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut maybe: *mut Maybe = ast as *mut Maybe;
    if !(*maybe).child.is_null() {
        let mut child_pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
        child_pattern = safe_c2rust_ast_get_pattern((*maybe).child, error);
        if child_pattern.is_null() {
            return ::core::ptr::null_mut::<gchar>();
        }
        pattern = g_strdup_printf(b"m%s\0" as *const u8 as *const gchar, child_pattern);
        g_free(child_pattern as gpointer);
        return pattern;
    }
    return safe_c2rust_g_strdup_inline(b"m*\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_maybe_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut maybe: *mut Maybe = ast as *mut Maybe;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if g_variant_type_is_maybe(type_0) == 0 {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    type_0 = g_variant_type_element(type_0);
    if !(*maybe).child.is_null() {
        value = safe_c2rust_ast_get_value((*maybe).child, type_0, error);
        if value.is_null() {
            return ::core::ptr::null_mut::<GVariant>();
        }
    } else {
        value = ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new_maybe(type_0, value);
}
unsafe extern "C" fn safe_c2rust_maybe_free(mut ast: *mut AST) {
    let mut maybe: *mut Maybe = ast as *mut Maybe;
    if !(*maybe).child.is_null() {
        safe_c2rust_ast_free((*maybe).child);
    }
    g_slice_free1(::core::mem::size_of::<Maybe>() as gsize, maybe as gpointer);
}
unsafe extern "C" fn safe_c2rust_maybe_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_maybe_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_maybe_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: None,
            free: Some(safe_c2rust_maybe_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut child: *mut AST = ::core::ptr::null_mut::<AST>();
    let mut maybe: *mut Maybe = ::core::ptr::null_mut::<Maybe>();
    if safe_c2rust_token_stream_consume(stream, b"just\0" as *const u8 as *const gchar) != 0 {
        child = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
        if child.is_null() {
            return ::core::ptr::null_mut::<AST>();
        }
    } else if safe_c2rust_token_stream_consume(stream, b"nothing\0" as *const u8 as *const gchar)
        == 0
    {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            TRUE,
            G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD as ::core::ffi::c_int as gint,
            b"unknown keyword\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<AST>();
    }
    maybe = g_slice_alloc(::core::mem::size_of::<Maybe>() as gsize) as *mut Maybe;
    (*maybe).ast.class = &raw const safe_c2rust_maybe_class;
    (*maybe).child = child;
    return maybe as *mut AST;
}
unsafe extern "C" fn safe_c2rust_maybe_wrapper(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut base_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut base_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut depth: ::core::ffi::c_uint = 0;
    let mut trusted: gboolean = 0;
    let mut base_type_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    let mut base_serialised_fixed_size: gsize = 0;
    let mut base_serialised_size: gsize = 0;
    let mut serialised_size: gsize = 0;
    let mut n_suffix_zeros: gsize = 0;
    let mut serialised: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut i: gsize = 0;
    depth = 0 as ::core::ffi::c_uint;
    base_type = type_0;
    while g_variant_type_is_maybe(base_type) != 0 {
        depth = depth.wrapping_add(1);
        base_type = g_variant_type_element(base_type);
    }
    base_value = (*(*ast).class)
        .get_base_value
        .expect("non-null function pointer")(ast, base_type, error);
    if base_value.is_null() || depth == 0 as ::core::ffi::c_uint {
        return safe_c2rust_g_steal_pointer(&raw mut base_value as gpointer) as *mut GVariant;
    }
    trusted = g_variant_is_trusted(base_value);
    base_type_info = g_variant_type_info_get(base_type);
    g_variant_type_info_query(
        base_type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut base_serialised_fixed_size,
    );
    g_variant_type_info_unref(base_type_info);
    base_serialised_size = g_variant_get_size(base_value);
    n_suffix_zeros = (if base_serialised_fixed_size > 0 as gsize {
        depth.wrapping_sub(1 as ::core::ffi::c_uint)
    } else {
        depth
    }) as gsize;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if base_serialised_size
            <= (9223372036854775807 as ::core::ffi::c_long as gsize)
                .wrapping_mul(2 as gsize)
                .wrapping_add(1 as gsize)
                .wrapping_sub(n_suffix_zeros)
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            941 as ::core::ffi::c_int,
            G_STRFUNC,
            b"base_serialised_size <= G_MAXSIZE - n_suffix_zeros\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    serialised_size = base_serialised_size.wrapping_add(n_suffix_zeros);
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if serialised_size >= base_serialised_size {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            944 as ::core::ffi::c_int,
            G_STRFUNC,
            b"serialised_size >= base_serialised_size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    serialised = g_malloc(serialised_size) as *mut guint8;
    g_variant_store(base_value, serialised as gpointer);
    i = base_serialised_size;
    while i < serialised_size {
        *serialised.offset(i as isize) = 0 as guint8;
        i = i.wrapping_add(1);
    }
    bytes = g_bytes_new_take(
        safe_c2rust_g_steal_pointer(&raw mut serialised as gpointer) as *mut guint8 as gpointer,
        serialised_size,
    );
    value = g_variant_new_from_bytes(type_0, bytes, trusted);
    g_bytes_unref(bytes);
    g_variant_unref(base_value);
    return safe_c2rust_g_steal_pointer(&raw mut value as gpointer) as *mut GVariant;
}
unsafe extern "C" fn safe_c2rust_array_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut array: *mut Array = ast as *mut Array;
    let mut pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if (*array).n_children == 0 as size_t {
        return safe_c2rust_g_strdup_inline(b"Ma*\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    pattern = safe_c2rust_ast_array_get_pattern((*array).children, (*array).n_children, error);
    if pattern.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_strdup_printf(b"Ma%s\0" as *const u8 as *const gchar, pattern);
    g_free(pattern as gpointer);
    return result;
}
unsafe extern "C" fn safe_c2rust_array_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut array: *mut Array = ast as *mut Array;
    let mut childtype: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut i: size_t = 0;
    if g_variant_type_is_array(type_0) == 0 {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    g_variant_builder_init(&raw mut builder, type_0);
    childtype = g_variant_type_element(type_0);
    i = 0 as size_t;
    while i < (*array).n_children {
        let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        child = safe_c2rust_ast_get_value(*(*array).children.offset(i as isize), childtype, error);
        if child.is_null() {
            g_variant_builder_clear(&raw mut builder);
            return ::core::ptr::null_mut::<GVariant>();
        }
        g_variant_builder_add_value(&raw mut builder, child);
        i = i.wrapping_add(1);
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_array_free(mut ast: *mut AST) {
    let mut array: *mut Array = ast as *mut Array;
    safe_c2rust_ast_array_free((*array).children, (*array).n_children);
    g_slice_free1(::core::mem::size_of::<Array>() as gsize, array as gpointer);
}
unsafe extern "C" fn safe_c2rust_array_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    let mut current_block: u64;
    static mut safe_c2rust_array_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_array_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_array_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_array_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut need_comma: gboolean = FALSE;
    let mut array: *mut Array = ::core::ptr::null_mut::<Array>();
    array = g_slice_alloc(::core::mem::size_of::<Array>() as gsize) as *mut Array;
    (*array).ast.class = &raw const safe_c2rust_array_class;
    (*array).children = ::core::ptr::null_mut::<*mut AST>();
    (*array).n_children = 0 as size_t;
    safe_c2rust_token_stream_assert(stream, b"[\0" as *const u8 as *const gchar);
    loop {
        if !(safe_c2rust_token_stream_consume(stream, b"]\0" as *const u8 as *const gchar) == 0) {
            current_block = 3640593987805443782;
            break;
        }
        let mut child: *mut AST = ::core::ptr::null_mut::<AST>();
        if need_comma != 0
            && safe_c2rust_token_stream_require(
                stream,
                b",\0" as *const u8 as *const gchar,
                b" or ']' to follow array element\0" as *const u8 as *const gchar,
                error,
            ) == 0
        {
            current_block = 62337711710658791;
            break;
        }
        child = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
        if child.is_null() {
            current_block = 62337711710658791;
            break;
        }
        safe_c2rust_ast_array_append(
            &raw mut (*array).children,
            &raw mut (*array).n_children,
            child,
        );
        need_comma = TRUE as gboolean;
    }
    match current_block {
        3640593987805443782 => return array as *mut AST,
        _ => {
            safe_c2rust_ast_array_free((*array).children, (*array).n_children);
            g_slice_free1(::core::mem::size_of::<Array>() as gsize, array as gpointer);
            return ::core::ptr::null_mut::<AST>();
        }
    };
}
unsafe extern "C" fn safe_c2rust_tuple_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut tuple: *mut Tuple = ast as *mut Tuple;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut parts: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: size_t = 0;
    parts = ({
        let mut __n: gsize = (*tuple).n_children.wrapping_add(4 as size_t) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut gchar;
    let ref mut fresh45 = *parts.offset((*tuple).n_children.wrapping_add(1 as size_t) as isize);
    *fresh45 = b")\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    let ref mut fresh46 = *parts.offset((*tuple).n_children.wrapping_add(2 as size_t) as isize);
    *fresh46 = ::core::ptr::null_mut::<gchar>();
    let ref mut fresh47 = *parts.offset(0 as ::core::ffi::c_int as isize);
    *fresh47 = b"M(\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    i = 0 as size_t;
    while i < (*tuple).n_children {
        let ref mut fresh48 = *parts.offset(i.wrapping_add(1 as size_t) as isize);
        *fresh48 = safe_c2rust_ast_get_pattern(*(*tuple).children.offset(i as isize), error);
        if (*fresh48).is_null() {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == (*tuple).n_children {
        result = g_strjoinv(b"\0" as *const u8 as *const gchar, parts);
    }
    while i != 0 {
        let fresh49 = i;
        i = i.wrapping_sub(1);
        g_free(*parts.offset(fresh49 as isize) as gpointer);
    }
    g_free(parts as gpointer);
    return result;
}
unsafe extern "C" fn safe_c2rust_tuple_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut tuple: *mut Tuple = ast as *mut Tuple;
    let mut childtype: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut i: size_t = 0;
    if g_variant_type_is_tuple(type_0) == 0 {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    g_variant_builder_init(&raw mut builder, type_0);
    childtype = g_variant_type_first(type_0);
    i = 0 as size_t;
    while i < (*tuple).n_children {
        let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        if childtype.is_null() {
            g_variant_builder_clear(&raw mut builder);
            return safe_c2rust_ast_type_error(ast, type_0, error);
        }
        child = safe_c2rust_ast_get_value(*(*tuple).children.offset(i as isize), childtype, error);
        if child.is_null() {
            g_variant_builder_clear(&raw mut builder);
            return ::core::ptr::null_mut::<GVariant>();
        }
        g_variant_builder_add_value(&raw mut builder, child);
        childtype = g_variant_type_next(childtype);
        i = i.wrapping_add(1);
    }
    if !childtype.is_null() {
        g_variant_builder_clear(&raw mut builder);
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_tuple_free(mut ast: *mut AST) {
    let mut tuple: *mut Tuple = ast as *mut Tuple;
    safe_c2rust_ast_array_free((*tuple).children, (*tuple).n_children);
    g_slice_free1(::core::mem::size_of::<Tuple>() as gsize, tuple as gpointer);
}
unsafe extern "C" fn safe_c2rust_tuple_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    let mut current_block: u64;
    static mut safe_c2rust_tuple_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_tuple_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_tuple_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_tuple_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut need_comma: gboolean = FALSE;
    let mut first: gboolean = TRUE;
    let mut tuple: *mut Tuple = ::core::ptr::null_mut::<Tuple>();
    tuple = g_slice_alloc(::core::mem::size_of::<Tuple>() as gsize) as *mut Tuple;
    (*tuple).ast.class = &raw const safe_c2rust_tuple_class;
    (*tuple).children = ::core::ptr::null_mut::<*mut AST>();
    (*tuple).n_children = 0 as size_t;
    safe_c2rust_token_stream_assert(stream, b"(\0" as *const u8 as *const gchar);
    loop {
        if !(safe_c2rust_token_stream_consume(stream, b")\0" as *const u8 as *const gchar) == 0) {
            current_block = 12800627514080957624;
            break;
        }
        let mut child: *mut AST = ::core::ptr::null_mut::<AST>();
        if need_comma != 0
            && safe_c2rust_token_stream_require(
                stream,
                b",\0" as *const u8 as *const gchar,
                b" or ')' to follow tuple element\0" as *const u8 as *const gchar,
                error,
            ) == 0
        {
            current_block = 16291136137530058153;
            break;
        }
        child = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
        if child.is_null() {
            current_block = 16291136137530058153;
            break;
        }
        safe_c2rust_ast_array_append(
            &raw mut (*tuple).children,
            &raw mut (*tuple).n_children,
            child,
        );
        if first != 0 {
            if safe_c2rust_token_stream_require(
                stream,
                b",\0" as *const u8 as *const gchar,
                b" after first tuple element\0" as *const u8 as *const gchar,
                error,
            ) == 0
            {
                current_block = 16291136137530058153;
                break;
            }
            first = FALSE as gboolean;
        } else {
            need_comma = TRUE as gboolean;
        }
    }
    match current_block {
        12800627514080957624 => return tuple as *mut AST,
        _ => {
            safe_c2rust_ast_array_free((*tuple).children, (*tuple).n_children);
            g_slice_free1(::core::mem::size_of::<Tuple>() as gsize, tuple as gpointer);
            return ::core::ptr::null_mut::<AST>();
        }
    };
}
unsafe extern "C" fn safe_c2rust_variant_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    return safe_c2rust_g_strdup_inline(b"Mv\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_variant_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut variant: *mut Variant = ast as *mut Variant;
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_VARIANT as gconstpointer,
    ) == 0
    {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    child = safe_c2rust_ast_resolve((*variant).value, error);
    if child.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new_variant(child);
}
unsafe extern "C" fn safe_c2rust_variant_free(mut ast: *mut AST) {
    let mut variant: *mut Variant = ast as *mut Variant;
    safe_c2rust_ast_free((*variant).value);
    g_slice_free1(
        ::core::mem::size_of::<Variant>() as gsize,
        variant as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_variant_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_variant_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_variant_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_variant_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_variant_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut variant: *mut Variant = ::core::ptr::null_mut::<Variant>();
    let mut value: *mut AST = ::core::ptr::null_mut::<AST>();
    safe_c2rust_token_stream_assert(stream, b"<\0" as *const u8 as *const gchar);
    value = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
    if value.is_null() {
        return ::core::ptr::null_mut::<AST>();
    }
    if safe_c2rust_token_stream_require(
        stream,
        b">\0" as *const u8 as *const gchar,
        b" to follow variant value\0" as *const u8 as *const gchar,
        error,
    ) == 0
    {
        safe_c2rust_ast_free(value);
        return ::core::ptr::null_mut::<AST>();
    }
    variant = g_slice_alloc(::core::mem::size_of::<Variant>() as gsize) as *mut Variant;
    (*variant).ast.class = &raw const safe_c2rust_variant_class;
    (*variant).value = value;
    return variant as *mut AST;
}
pub const DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY: size_t = -(1 as ::core::ffi::c_int) as size_t;
unsafe extern "C" fn safe_c2rust_dictionary_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut dict: *mut Dictionary = ast as *mut Dictionary;
    let mut value_pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_pattern: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_char: gchar = 0;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if (*dict).n_children == 0 as size_t {
        return safe_c2rust_g_strdup_inline(b"Ma{**}\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    key_pattern = safe_c2rust_ast_array_get_pattern(
        (*dict).keys,
        if (*dict).n_children == DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY {
            1 as size_t
        } else {
            (*dict).n_children
        },
        error,
    );
    if key_pattern.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    if *key_pattern.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32 {
        key_char = *key_pattern.offset(1 as ::core::ffi::c_int as isize);
    } else {
        key_char = *key_pattern.offset(0 as ::core::ffi::c_int as isize);
    }
    g_free(key_pattern as gpointer);
    if strchr(
        b"bynqiuxthdsogNS\0" as *const u8 as *const ::core::ffi::c_char,
        key_char as ::core::ffi::c_int,
    )
    .is_null()
    {
        safe_c2rust_ast_set_error(
            ast,
            error,
            ::core::ptr::null_mut::<AST>(),
            G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED as ::core::ffi::c_int as gint,
            b"dictionary keys must have basic types\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    value_pattern = safe_c2rust_ast_get_pattern(
        *(*dict).values.offset(0 as ::core::ffi::c_int as isize),
        error,
    );
    if value_pattern.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_strdup_printf(
        b"M%s{%c%s}\0" as *const u8 as *const gchar,
        if (*dict).n_children > 0 as size_t
            && (*dict).n_children != DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY
        {
            b"a\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        key_char as ::core::ffi::c_int,
        value_pattern,
    );
    g_free(value_pattern as gpointer);
    return result;
}
unsafe extern "C" fn safe_c2rust_dictionary_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut dict: *mut Dictionary = ast as *mut Dictionary;
    if (*dict).n_children == DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY {
        let mut subtype: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed_0 {
                s: C2RustUnnamed_1 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut subvalue: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        if g_variant_type_is_dict_entry(type_0) == 0 {
            return safe_c2rust_ast_type_error(ast, type_0, error);
        }
        g_variant_builder_init(&raw mut builder, type_0);
        subtype = g_variant_type_key(type_0);
        subvalue = safe_c2rust_ast_get_value(
            *(*dict).keys.offset(0 as ::core::ffi::c_int as isize),
            subtype,
            error,
        );
        if subvalue.is_null() {
            g_variant_builder_clear(&raw mut builder);
            return ::core::ptr::null_mut::<GVariant>();
        }
        g_variant_builder_add_value(&raw mut builder, subvalue);
        subtype = g_variant_type_value(type_0);
        subvalue = safe_c2rust_ast_get_value(
            *(*dict).values.offset(0 as ::core::ffi::c_int as isize),
            subtype,
            error,
        );
        if subvalue.is_null() {
            g_variant_builder_clear(&raw mut builder);
            return ::core::ptr::null_mut::<GVariant>();
        }
        g_variant_builder_add_value(&raw mut builder, subvalue);
        return g_variant_builder_end(&raw mut builder);
    } else {
        let mut entry: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut key: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut val: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut builder_0: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed_0 {
                s: C2RustUnnamed_1 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut i: size_t = 0;
        if g_variant_type_is_subtype_of(type_0, G_VARIANT_TYPE_DICTIONARY) == 0 {
            return safe_c2rust_ast_type_error(ast, type_0, error);
        }
        entry = g_variant_type_element(type_0);
        key = g_variant_type_key(entry);
        val = g_variant_type_value(entry);
        g_variant_builder_init(&raw mut builder_0, type_0);
        i = 0 as size_t;
        while i < (*dict).n_children {
            let mut subvalue_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            g_variant_builder_open(&raw mut builder_0, entry);
            subvalue_0 = safe_c2rust_ast_get_value(*(*dict).keys.offset(i as isize), key, error);
            if subvalue_0.is_null() {
                g_variant_builder_clear(&raw mut builder_0);
                return ::core::ptr::null_mut::<GVariant>();
            }
            g_variant_builder_add_value(&raw mut builder_0, subvalue_0);
            subvalue_0 = safe_c2rust_ast_get_value(*(*dict).values.offset(i as isize), val, error);
            if subvalue_0.is_null() {
                g_variant_builder_clear(&raw mut builder_0);
                return ::core::ptr::null_mut::<GVariant>();
            }
            g_variant_builder_add_value(&raw mut builder_0, subvalue_0);
            g_variant_builder_close(&raw mut builder_0);
            i = i.wrapping_add(1);
        }
        return g_variant_builder_end(&raw mut builder_0);
    };
}
unsafe extern "C" fn safe_c2rust_dictionary_free(mut ast: *mut AST) {
    let mut dict: *mut Dictionary = ast as *mut Dictionary;
    let mut n_children: size_t = 0;
    if (*dict).n_children == DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY {
        n_children = 1 as size_t;
    } else {
        n_children = (*dict).n_children;
    }
    safe_c2rust_ast_array_free((*dict).keys, n_children);
    safe_c2rust_ast_array_free((*dict).values, n_children);
    g_slice_free1(
        ::core::mem::size_of::<Dictionary>() as gsize,
        dict as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_dictionary_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    let mut current_block: u64;
    static mut safe_c2rust_dictionary_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_dictionary_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_dictionary_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_dictionary_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut n_keys: size_t = 0;
    let mut n_values: size_t = 0;
    let mut only_one: gboolean = 0;
    let mut dict: *mut Dictionary = ::core::ptr::null_mut::<Dictionary>();
    let mut first: *mut AST = ::core::ptr::null_mut::<AST>();
    dict = g_slice_alloc(::core::mem::size_of::<Dictionary>() as gsize) as *mut Dictionary;
    (*dict).ast.class = &raw const safe_c2rust_dictionary_class;
    (*dict).keys = ::core::ptr::null_mut::<*mut AST>();
    (*dict).values = ::core::ptr::null_mut::<*mut AST>();
    n_values = 0 as size_t;
    n_keys = n_values;
    safe_c2rust_token_stream_assert(stream, b"{\0" as *const u8 as *const gchar);
    if safe_c2rust_token_stream_consume(stream, b"}\0" as *const u8 as *const gchar) != 0 {
        (*dict).n_children = 0 as size_t;
        return dict as *mut AST;
    }
    first = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
    if !first.is_null() {
        safe_c2rust_ast_array_append(&raw mut (*dict).keys, &raw mut n_keys, first);
        only_one = safe_c2rust_token_stream_consume(stream, b",\0" as *const u8 as *const gchar);
        if !(only_one == 0
            && safe_c2rust_token_stream_require(
                stream,
                b":\0" as *const u8 as *const gchar,
                b" or ',' to follow dictionary entry key\0" as *const u8 as *const gchar,
                error,
            ) == 0)
        {
            first = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
            if !first.is_null() {
                safe_c2rust_ast_array_append(&raw mut (*dict).values, &raw mut n_values, first);
                if only_one != 0 {
                    if !(safe_c2rust_token_stream_require(
                        stream,
                        b"}\0" as *const u8 as *const gchar,
                        b" at end of dictionary entry\0" as *const u8 as *const gchar,
                        error,
                    ) == 0)
                    {
                        if ({
                            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                            if n_keys == 1 as size_t && n_values == 1 as size_t {
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
                                b"../original/glib/gvariant-parser.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                1527 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"n_keys == 1 && n_values == 1\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        (*dict).n_children = DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY;
                        return dict as *mut AST;
                    }
                } else {
                    loop {
                        if !(safe_c2rust_token_stream_consume(
                            stream,
                            b"}\0" as *const u8 as *const gchar,
                        ) == 0)
                        {
                            current_block = 1608152415753874203;
                            break;
                        }
                        let mut child: *mut AST = ::core::ptr::null_mut::<AST>();
                        if safe_c2rust_token_stream_require(
                            stream,
                            b",\0" as *const u8 as *const gchar,
                            b" or '}' to follow dictionary entry\0" as *const u8 as *const gchar,
                            error,
                        ) == 0
                        {
                            current_block = 5384159925508488278;
                            break;
                        }
                        child = safe_c2rust_parse(
                            stream,
                            max_depth.wrapping_sub(1 as guint),
                            app,
                            error,
                        );
                        if child.is_null() {
                            current_block = 5384159925508488278;
                            break;
                        }
                        safe_c2rust_ast_array_append(&raw mut (*dict).keys, &raw mut n_keys, child);
                        if safe_c2rust_token_stream_require(
                            stream,
                            b":\0" as *const u8 as *const gchar,
                            b" to follow dictionary entry key\0" as *const u8 as *const gchar,
                            error,
                        ) == 0
                        {
                            current_block = 5384159925508488278;
                            break;
                        }
                        child = safe_c2rust_parse(
                            stream,
                            max_depth.wrapping_sub(1 as guint),
                            app,
                            error,
                        );
                        if child.is_null() {
                            current_block = 5384159925508488278;
                            break;
                        }
                        safe_c2rust_ast_array_append(
                            &raw mut (*dict).values,
                            &raw mut n_values,
                            child,
                        );
                    }
                    match current_block {
                        5384159925508488278 => {}
                        _ => {
                            if ({
                                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                                if n_keys == n_values {
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
                                    b"../original/glib/gvariant-parser.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    1560 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"n_keys == n_values\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            if ({
                                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                                if n_keys != -(1 as ::core::ffi::c_int) as size_t {
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
                                    b"../original/glib/gvariant-parser.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    1561 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"n_keys != DICTIONARY_N_CHILDREN_FREESTANDING_ENTRY\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            (*dict).n_children = n_keys;
                            return dict as *mut AST;
                        }
                    }
                }
            }
        }
    }
    safe_c2rust_ast_array_free((*dict).keys, n_keys);
    safe_c2rust_ast_array_free((*dict).values, n_values);
    g_slice_free1(
        ::core::mem::size_of::<Dictionary>() as gsize,
        dict as gpointer,
    );
    return ::core::ptr::null_mut::<AST>();
}
unsafe extern "C" fn safe_c2rust_string_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    return safe_c2rust_g_strdup_inline(b"MS\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_string_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut string: *mut String_0 = ast as *mut String_0;
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_STRING as gconstpointer,
    ) != 0
    {
        return g_variant_new_string((*string).string);
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_OBJECT_PATH as gconstpointer,
    ) != 0
    {
        if g_variant_is_object_path((*string).string) == 0 {
            safe_c2rust_ast_set_error(
                ast,
                error,
                ::core::ptr::null_mut::<AST>(),
                G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH as ::core::ffi::c_int as gint,
                b"not a valid object path\0" as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        return g_variant_new_object_path((*string).string);
    } else if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_SIGNATURE as gconstpointer,
    ) != 0
    {
        if g_variant_is_signature((*string).string) == 0 {
            safe_c2rust_ast_set_error(
                ast,
                error,
                ::core::ptr::null_mut::<AST>(),
                G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE as ::core::ffi::c_int as gint,
                b"not a valid signature\0" as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        return g_variant_new_signature((*string).string);
    } else {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    };
}
unsafe extern "C" fn safe_c2rust_string_free(mut ast: *mut AST) {
    let mut string: *mut String_0 = ast as *mut String_0;
    g_free((*string).string as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<String_0>() as gsize,
        string as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_unicode_unescape(
    mut src: *const gchar,
    mut src_ofs: *mut size_t,
    mut dest: *mut gchar,
    mut dest_ofs: *mut size_t,
    mut length: gsize,
    mut ref_0: *mut SourceRef,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut buffer: [gchar; 9] = [0; 9];
    let mut value: guint64 = 0 as guint64;
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n_valid_chars: gsize = 0;
    *src_ofs = (*src_ofs).wrapping_add(1);
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (length as usize) < ::core::mem::size_of::<[gchar; 9]>() as usize {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            G_STRFUNC,
            b"length < sizeof (buffer)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    strncpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        src.offset(*src_ofs as isize),
        length as size_t,
    );
    buffer[length as usize] = '\0' as i32 as gchar;
    n_valid_chars = 0 as gsize;
    while n_valid_chars < length {
        if !(*safe_c2rust_g_ascii_table.offset(buffer[n_valid_chars as usize] as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_XDIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
        {
            break;
        }
        n_valid_chars = n_valid_chars.wrapping_add(1);
    }
    if n_valid_chars == length {
        value = g_ascii_strtoull(&raw mut buffer as *mut gchar, &raw mut end, 0x10 as guint);
    }
    if value == 0 as guint64 || end != (&raw mut buffer as *mut gchar).offset(length as isize) {
        let mut escape_ref: SourceRef = SourceRef { start: 0, end: 0 };
        escape_ref = *ref_0;
        escape_ref.start = escape_ref.start.wrapping_add(*src_ofs);
        escape_ref.end = escape_ref.start.wrapping_add(n_valid_chars as size_t);
        safe_c2rust_parser_set_error(
            error,
            &raw mut escape_ref,
            ::core::ptr::null_mut::<SourceRef>(),
            G_VARIANT_PARSE_ERROR_INVALID_CHARACTER as ::core::ffi::c_int as gint,
            b"invalid %lu-character unicode escape\0" as *const u8 as *const gchar,
            length,
        );
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if value <= 0xffffffff as ::core::ffi::c_uint as guint64 {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1679 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value <= G_MAXUINT32\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *dest_ofs = (*dest_ofs)
        .wrapping_add(
            g_unichar_to_utf8(value as gunichar, dest.offset(*dest_ofs as isize)) as size_t,
        );
    *src_ofs = (*src_ofs as ::core::ffi::c_ulong).wrapping_add(length as ::core::ffi::c_ulong)
        as size_t as size_t;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_string_parse(
    mut stream: *mut TokenStream,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_string_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_string_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_string_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_string_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut string: *mut String_0 = ::core::ptr::null_mut::<String_0>();
    let mut ref_0: SourceRef = SourceRef { start: 0, end: 0 };
    let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    let mut quote: gchar = 0;
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    safe_c2rust_token_stream_start_ref(stream, &raw mut ref_0);
    token = safe_c2rust_token_stream_get(stream);
    safe_c2rust_token_stream_end_ref(stream, &raw mut ref_0);
    length = strlen(token) as gsize;
    quote = *token.offset(0 as ::core::ffi::c_int as isize);
    str = g_malloc(length) as *mut gchar;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if quote as ::core::ffi::c_int == '"' as i32 || quote as ::core::ffi::c_int == '\'' as i32 {
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
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1712 as ::core::ffi::c_int,
            G_STRFUNC,
            b"quote == '\"' || quote == '\\''\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    j = 0 as size_t;
    i = 1 as size_t;
    let mut current_block_47: u64;
    while *token.offset(i as isize) as ::core::ffi::c_int != quote as ::core::ffi::c_int {
        match *token.offset(i as isize) as ::core::ffi::c_int {
            0 => {
                safe_c2rust_parser_set_error(
                    error,
                    &raw mut ref_0,
                    ::core::ptr::null_mut::<SourceRef>(),
                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT as ::core::ffi::c_int
                        as gint,
                    b"unterminated string constant\0" as *const u8 as *const gchar,
                );
                g_free(token as gpointer);
                g_free(str as gpointer);
                return ::core::ptr::null_mut::<AST>();
            }
            92 => {
                i = i.wrapping_add(1);
                match *token.offset(i as isize) as ::core::ffi::c_int {
                    0 => {
                        current_block_47 = 16043698553273606360;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    117 => {
                        current_block_47 = 9069637408997036600;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    85 => {
                        current_block_47 = 5834853401383615185;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    97 => {
                        current_block_47 = 18086183351333264011;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    98 => {
                        current_block_47 = 2289199519686980607;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    102 => {
                        current_block_47 = 5111587104494200972;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    110 => {
                        current_block_47 = 15967253824745734317;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    114 => {
                        current_block_47 = 11481713732801547447;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    116 => {
                        current_block_47 = 17287073966728009172;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    118 => {
                        current_block_47 = 2693192215096893985;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    10 => {
                        current_block_47 = 8218073211419798093;
                        match current_block_47 {
                            8218073211419798093 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2693192215096893985 => {
                                let fresh26 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh26 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            17287073966728009172 => {
                                let fresh25 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh25 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11481713732801547447 => {
                                let fresh24 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh24 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15967253824745734317 => {
                                let fresh23 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh23 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            5111587104494200972 => {
                                let fresh22 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh22 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2289199519686980607 => {
                                let fresh21 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh21 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18086183351333264011 => {
                                let fresh20 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh20 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            9069637408997036600 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    4 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            5834853401383615185 => {
                                if safe_c2rust_unicode_unescape(
                                    token,
                                    &raw mut i,
                                    str,
                                    &raw mut j,
                                    8 as gsize,
                                    &raw mut ref_0,
                                    error,
                                ) == 0
                                {
                                    g_free(token as gpointer);
                                    g_free(str as gpointer);
                                    return ::core::ptr::null_mut::<AST>();
                                }
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(token as gpointer);
                                g_free(str as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        let fresh27 = i;
        i = i.wrapping_add(1);
        let fresh28 = j;
        j = j.wrapping_add(1);
        *str.offset(fresh28 as isize) = *token.offset(fresh27 as isize);
    }
    let fresh29 = j;
    j = j.wrapping_add(1);
    *str.offset(fresh29 as isize) = '\0' as i32 as gchar;
    g_free(token as gpointer);
    string = g_slice_alloc(::core::mem::size_of::<String_0>() as gsize) as *mut String_0;
    (*string).ast.class = &raw const safe_c2rust_string_class;
    (*string).string = str;
    safe_c2rust_token_stream_next(stream);
    return string as *mut AST;
}
unsafe extern "C" fn safe_c2rust_bytestring_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    return safe_c2rust_g_strdup_inline(b"May\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_bytestring_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut string: *mut ByteString = ast as *mut ByteString;
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_BYTESTRING as gconstpointer,
    ) == 0
    {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    return g_variant_new_bytestring((*string).string);
}
unsafe extern "C" fn safe_c2rust_bytestring_free(mut ast: *mut AST) {
    let mut string: *mut ByteString = ast as *mut ByteString;
    g_free((*string).string as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<ByteString>() as gsize,
        string as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_bytestring_parse(
    mut stream: *mut TokenStream,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_bytestring_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_bytestring_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_bytestring_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_bytestring_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut string: *mut ByteString = ::core::ptr::null_mut::<ByteString>();
    let mut ref_0: SourceRef = SourceRef { start: 0, end: 0 };
    let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    let mut quote: gchar = 0;
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    safe_c2rust_token_stream_start_ref(stream, &raw mut ref_0);
    token = safe_c2rust_token_stream_get(stream);
    safe_c2rust_token_stream_end_ref(stream, &raw mut ref_0);
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'b' as i32 {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1838 as ::core::ffi::c_int,
            G_STRFUNC,
            b"token[0] == 'b'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    length = strlen(token) as gsize;
    quote = *token.offset(1 as ::core::ffi::c_int as isize);
    str = g_malloc(length) as *mut gchar;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if quote as ::core::ffi::c_int == '"' as i32 || quote as ::core::ffi::c_int == '\'' as i32 {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1843 as ::core::ffi::c_int,
            G_STRFUNC,
            b"quote == '\"' || quote == '\\''\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    j = 0 as size_t;
    i = 2 as size_t;
    let mut current_block_48: u64;
    while *token.offset(i as isize) as ::core::ffi::c_int != quote as ::core::ffi::c_int {
        match *token.offset(i as isize) as ::core::ffi::c_int {
            0 => {
                safe_c2rust_parser_set_error(
                    error,
                    &raw mut ref_0,
                    ::core::ptr::null_mut::<SourceRef>(),
                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT as ::core::ffi::c_int
                        as gint,
                    b"unterminated string constant\0" as *const u8 as *const gchar,
                );
                g_free(str as gpointer);
                g_free(token as gpointer);
                return ::core::ptr::null_mut::<AST>();
            }
            92 => {
                i = i.wrapping_add(1);
                match *token.offset(i as isize) as ::core::ffi::c_int {
                    0 => {
                        current_block_48 = 15746111448571087051;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        current_block_48 = 2433003649423799153;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    97 => {
                        current_block_48 = 11442208050987234306;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    98 => {
                        current_block_48 = 2097885661151323473;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    102 => {
                        current_block_48 = 18130095164818162132;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    110 => {
                        current_block_48 = 11888293203943784705;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    114 => {
                        current_block_48 = 7538714909055336126;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    116 => {
                        current_block_48 = 3819777561782277699;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    118 => {
                        current_block_48 = 15027391174080116654;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    10 => {
                        current_block_48 = 11037613425661949427;
                        match current_block_48 {
                            11037613425661949427 => {
                                i = i.wrapping_add(1);
                                continue;
                            }
                            15027391174080116654 => {
                                let fresh15 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh15 as isize) = '\u{b}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            3819777561782277699 => {
                                let fresh14 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh14 as isize) = '\t' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            7538714909055336126 => {
                                let fresh13 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh13 as isize) = '\r' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11888293203943784705 => {
                                let fresh12 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh12 as isize) = '\n' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            18130095164818162132 => {
                                let fresh11 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh11 as isize) = '\u{c}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2097885661151323473 => {
                                let fresh10 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh10 as isize) = '\u{8}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            11442208050987234306 => {
                                let fresh9 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh9 as isize) = '\u{7}' as i32 as gchar;
                                i = i.wrapping_add(1);
                                continue;
                            }
                            2433003649423799153 => {
                                let fresh5 = i;
                                i = i.wrapping_add(1);
                                let mut val: guchar =
                                    (*token.offset(fresh5 as isize) as ::core::ffi::c_int
                                        - '0' as i32) as guchar;
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh6 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh6 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                if '0' as i32 <= *token.offset(i as isize) as ::core::ffi::c_int
                                    && (*token.offset(i as isize) as ::core::ffi::c_int)
                                        < '8' as i32
                                {
                                    let fresh7 = i;
                                    i = i.wrapping_add(1);
                                    val = ((val as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | *token.offset(fresh7 as isize) as ::core::ffi::c_int
                                            - '0' as i32)
                                        as guchar;
                                }
                                let fresh8 = j;
                                j = j.wrapping_add(1);
                                *str.offset(fresh8 as isize) = val as gchar;
                                continue;
                            }
                            _ => {
                                safe_c2rust_parser_set_error(
                                    error,
                                    &raw mut ref_0,
                                    ::core::ptr::null_mut::<SourceRef>(),
                                    G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT
                                        as ::core::ffi::c_int
                                        as gint,
                                    b"unterminated string constant\0" as *const u8 as *const gchar,
                                );
                                g_free(str as gpointer);
                                g_free(token as gpointer);
                                return ::core::ptr::null_mut::<AST>();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        let fresh16 = i;
        i = i.wrapping_add(1);
        let fresh17 = j;
        j = j.wrapping_add(1);
        *str.offset(fresh17 as isize) = *token.offset(fresh16 as isize);
    }
    let fresh18 = j;
    j = j.wrapping_add(1);
    *str.offset(fresh18 as isize) = '\0' as i32 as gchar;
    g_free(token as gpointer);
    string = g_slice_alloc(::core::mem::size_of::<ByteString>() as gsize) as *mut ByteString;
    (*string).ast.class = &raw const safe_c2rust_bytestring_class;
    (*string).string = str;
    safe_c2rust_token_stream_next(stream);
    return string as *mut AST;
}
unsafe extern "C" fn safe_c2rust_number_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut number: *mut Number = ast as *mut Number;
    if !strchr((*number).token, '.' as i32).is_null()
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = (*number).token;
                let __prefix: *const ::core::ffi::c_char =
                    b"0x\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_23
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix((*number).token, b"0x\0" as *const u8 as *const gchar)
        }) == 0
            && !strchr((*number).token, 'e' as i32).is_null()
        || !strstr(
            (*number).token,
            b"inf\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            (*number).token,
            b"nan\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return safe_c2rust_g_strdup_inline(b"Md\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    return safe_c2rust_g_strdup_inline(b"MN\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_number_overflow(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    safe_c2rust_ast_set_error(
        ast,
        error,
        ::core::ptr::null_mut::<AST>(),
        G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE as ::core::ffi::c_int as gint,
        b"number out of range for type '%c'\0" as *const u8 as *const gchar,
        *g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int,
    );
    return ::core::ptr::null_mut::<GVariant>();
}
unsafe extern "C" fn safe_c2rust_number_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut number: *mut Number = ast as *mut Number;
    let mut token: *const gchar = ::core::ptr::null::<gchar>();
    let mut negative: gboolean = 0;
    let mut floating: gboolean = 0;
    let mut abs_val: guint64 = 0;
    let mut dbl_val: gdouble = 0.;
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    token = (*number).token;
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_DOUBLE as gconstpointer,
    ) != 0
    {
        floating = TRUE as gboolean;
        *__errno_location() = 0 as ::core::ffi::c_int;
        dbl_val = g_ascii_strtod(token, &raw mut end);
        if dbl_val != 0.0f64 && *__errno_location() == ERANGE {
            safe_c2rust_ast_set_error(
                ast,
                error,
                ::core::ptr::null_mut::<AST>(),
                G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG as ::core::ffi::c_int as gint,
                b"number too big for any type\0" as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        negative = FALSE as gboolean;
        abs_val = 0 as guint64;
    } else {
        floating = FALSE as gboolean;
        negative = (*token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32) as ::core::ffi::c_int as gboolean;
        if *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
            token = token.offset(1);
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        abs_val = g_ascii_strtoull(token, &raw mut end, 0 as guint);
        if abs_val == G_MAXUINT64 && *__errno_location() == ERANGE {
            safe_c2rust_ast_set_error(
                ast,
                error,
                ::core::ptr::null_mut::<AST>(),
                G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG as ::core::ffi::c_int as gint,
                b"integer too big for any type\0" as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        if abs_val == 0 as guint64 {
            negative = FALSE as gboolean;
        }
        dbl_val = 0.0f64 as gdouble;
    }
    if *end as ::core::ffi::c_int != '\0' as i32 {
        let mut ref_0: SourceRef = SourceRef { start: 0, end: 0 };
        ref_0 = (*ast).source_ref;
        ref_0.start = ref_0
            .start
            .wrapping_add(end.offset_from((*number).token) as ::core::ffi::c_long as size_t);
        ref_0.end = ref_0.start.wrapping_add(1 as size_t);
        safe_c2rust_parser_set_error(
            error,
            &raw mut ref_0,
            ::core::ptr::null_mut::<SourceRef>(),
            G_VARIANT_PARSE_ERROR_INVALID_CHARACTER as ::core::ffi::c_int as gint,
            b"invalid character in number\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if floating != 0 {
        return g_variant_new_double(dbl_val);
    }
    match *g_variant_type_peek_string(type_0) as ::core::ffi::c_int {
        121 => {
            if negative != 0 || abs_val > G_MAXUINT8 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            return g_variant_new_byte(abs_val as guint8);
        }
        110 => {
            if abs_val.wrapping_sub(negative as guint64) > G_MAXINT16 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            if negative != 0 && abs_val > G_MAXINT16 as guint64 {
                return g_variant_new_int16(G_MININT16);
            }
            return g_variant_new_int16(
                (if negative != 0 {
                    -(abs_val as gint16 as ::core::ffi::c_int)
                } else {
                    abs_val as gint16 as ::core::ffi::c_int
                }) as gint16,
            );
        }
        113 => {
            if negative != 0 || abs_val > G_MAXUINT16 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            return g_variant_new_uint16(abs_val as guint16);
        }
        105 => {
            if abs_val.wrapping_sub(negative as guint64) > G_MAXINT32 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            if negative != 0 && abs_val > G_MAXINT32 as guint64 {
                return g_variant_new_int32(G_MININT32);
            }
            return g_variant_new_int32(if negative != 0 {
                -(abs_val as gint32)
            } else {
                abs_val as gint32
            });
        }
        117 => {
            if negative != 0 || abs_val > G_MAXUINT32 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            return g_variant_new_uint32(abs_val as guint32);
        }
        120 => {
            if abs_val.wrapping_sub(negative as guint64) > G_MAXINT64 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            if negative != 0 && abs_val > G_MAXINT64 as guint64 {
                return g_variant_new_int64(G_MININT64);
            }
            return g_variant_new_int64(if negative != 0 {
                -(abs_val as gint64)
            } else {
                abs_val as gint64
            });
        }
        116 => {
            if negative != 0 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            return g_variant_new_uint64(abs_val);
        }
        104 => {
            if abs_val.wrapping_sub(negative as guint64) > G_MAXINT32 as guint64 {
                return safe_c2rust_number_overflow(ast, type_0, error);
            }
            if negative != 0 && abs_val > G_MAXINT32 as guint64 {
                return g_variant_new_handle(G_MININT32);
            }
            return g_variant_new_handle(if negative != 0 {
                -(abs_val as gint32)
            } else {
                abs_val as gint32
            });
        }
        _ => return safe_c2rust_ast_type_error(ast, type_0, error),
    };
}
unsafe extern "C" fn safe_c2rust_number_free(mut ast: *mut AST) {
    let mut number: *mut Number = ast as *mut Number;
    g_free((*number).token as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<Number>() as gsize,
        number as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_number_parse(
    mut stream: *mut TokenStream,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_number_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_number_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_number_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_number_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut number: *mut Number = ::core::ptr::null_mut::<Number>();
    number = g_slice_alloc(::core::mem::size_of::<Number>() as gsize) as *mut Number;
    (*number).ast.class = &raw const safe_c2rust_number_class;
    (*number).token = safe_c2rust_token_stream_get(stream);
    safe_c2rust_token_stream_next(stream);
    return number as *mut AST;
}
unsafe extern "C" fn safe_c2rust_boolean_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    return safe_c2rust_g_strdup_inline(b"Mb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_boolean_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut boolean: *mut Boolean = ast as *mut Boolean;
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_BOOLEAN as gconstpointer,
    ) == 0
    {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    return g_variant_new_boolean((*boolean).value);
}
unsafe extern "C" fn safe_c2rust_boolean_free(mut ast: *mut AST) {
    let mut boolean: *mut Boolean = ast as *mut Boolean;
    g_slice_free1(
        ::core::mem::size_of::<Boolean>() as gsize,
        boolean as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_boolean_new(mut value: gboolean) -> *mut AST {
    static mut safe_c2rust_boolean_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_boolean_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_maybe_wrapper
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: Some(
                safe_c2rust_boolean_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            free: Some(safe_c2rust_boolean_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut boolean: *mut Boolean = ::core::ptr::null_mut::<Boolean>();
    boolean = g_slice_alloc(::core::mem::size_of::<Boolean>() as gsize) as *mut Boolean;
    (*boolean).ast.class = &raw const safe_c2rust_boolean_class;
    (*boolean).value = value;
    return boolean as *mut AST;
}
unsafe extern "C" fn safe_c2rust_positional_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut positional: *mut Positional = ast as *mut Positional;
    return safe_c2rust_g_strdup_inline(
        g_variant_get_type_string((*positional).value) as *const ::core::ffi::c_char
    ) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_positional_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut positional: *mut Positional = ast as *mut Positional;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !(*positional).value.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            2182 as ::core::ffi::c_int,
            G_STRFUNC,
            b"positional->value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if g_variant_is_of_type((*positional).value, type_0) == 0 {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
        return safe_c2rust_ast_type_error(ast, type_0, error);
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !(*positional).value.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            2192 as ::core::ffi::c_int,
            G_STRFUNC,
            b"positional->value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = (*positional).value;
    (*positional).value = ::core::ptr::null_mut::<GVariant>();
    return value;
}
unsafe extern "C" fn safe_c2rust_positional_free(mut ast: *mut AST) {
    let mut positional: *mut Positional = ast as *mut Positional;
    g_slice_free1(
        ::core::mem::size_of::<Positional>() as gsize,
        positional as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_positional_parse(
    mut stream: *mut TokenStream,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_positional_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_positional_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_positional_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: None,
            free: Some(safe_c2rust_positional_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut positional: *mut Positional = ::core::ptr::null_mut::<Positional>();
    let mut endptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
    token = safe_c2rust_token_stream_get(stream);
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32 {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-parser.c\0" as *const u8 as *const ::core::ffi::c_char,
            2225 as ::core::ffi::c_int,
            G_STRFUNC,
            b"token[0] == '%'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    positional = g_slice_alloc(::core::mem::size_of::<Positional>() as gsize) as *mut Positional;
    (*positional).ast.class = &raw const safe_c2rust_positional_class;
    (*positional).value = g_variant_new_va(
        token.offset(1 as ::core::ffi::c_int as isize),
        &raw mut endptr,
        app,
    );
    if *endptr as ::core::ffi::c_int != 0 || (*positional).value.is_null() {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            TRUE,
            G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING as ::core::ffi::c_int as gint,
            b"invalid GVariant format string\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<AST>();
    }
    safe_c2rust_token_stream_next(stream);
    g_free(token as gpointer);
    return positional as *mut AST;
}
unsafe extern "C" fn safe_c2rust_typedecl_get_pattern(
    mut ast: *mut AST,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut decl: *mut TypeDecl = ast as *mut TypeDecl;
    return g_variant_type_dup_string((*decl).type_0);
}
unsafe extern "C" fn safe_c2rust_typedecl_get_value(
    mut ast: *mut AST,
    mut type_0: *const GVariantType,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut decl: *mut TypeDecl = ast as *mut TypeDecl;
    return safe_c2rust_ast_get_value((*decl).child, type_0, error);
}
unsafe extern "C" fn safe_c2rust_typedecl_free(mut ast: *mut AST) {
    let mut decl: *mut TypeDecl = ast as *mut TypeDecl;
    safe_c2rust_ast_free((*decl).child);
    g_variant_type_free((*decl).type_0);
    g_slice_free1(
        ::core::mem::size_of::<TypeDecl>() as gsize,
        decl as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_typedecl_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    static mut safe_c2rust_typedecl_class: ASTClass = unsafe {
        ASTClass {
            get_pattern: Some(
                safe_c2rust_typedecl_get_pattern
                    as unsafe extern "C" fn(*mut AST, *mut *mut GError) -> *mut gchar,
            ),
            get_value: Some(
                safe_c2rust_typedecl_get_value
                    as unsafe extern "C" fn(
                        *mut AST,
                        *const GVariantType,
                        *mut *mut GError,
                    ) -> *mut GVariant,
            ),
            get_base_value: None,
            free: Some(safe_c2rust_typedecl_free as unsafe extern "C" fn(*mut AST) -> ()),
        }
    };
    let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut decl: *mut TypeDecl = ::core::ptr::null_mut::<TypeDecl>();
    let mut child: *mut AST = ::core::ptr::null_mut::<AST>();
    if safe_c2rust_token_stream_peek(stream, '@' as i32 as gchar) != 0 {
        let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
        token = safe_c2rust_token_stream_get(stream);
        if g_variant_type_string_is_valid(token.offset(1 as ::core::ffi::c_int as isize)) == 0 {
            safe_c2rust_token_stream_set_error(
                stream,
                error,
                TRUE,
                G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING as ::core::ffi::c_int as gint,
                b"invalid type declaration\0" as *const u8 as *const gchar,
            );
            g_free(token as gpointer);
            return ::core::ptr::null_mut::<AST>();
        }
        if g_variant_type_string_get_depth_(token.offset(1 as ::core::ffi::c_int as isize))
            > max_depth as gsize
        {
            safe_c2rust_token_stream_set_error(
                stream,
                error,
                TRUE,
                G_VARIANT_PARSE_ERROR_RECURSION as ::core::ffi::c_int as gint,
                b"type declaration recurses too deeply\0" as *const u8 as *const gchar,
            );
            g_free(token as gpointer);
            return ::core::ptr::null_mut::<AST>();
        }
        type_0 = g_variant_type_new(token.offset(1 as ::core::ffi::c_int as isize));
        if g_variant_type_is_definite(type_0) == 0 {
            safe_c2rust_token_stream_set_error(
                stream,
                error,
                TRUE,
                G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED as ::core::ffi::c_int as gint,
                b"type declarations must be definite\0" as *const u8 as *const gchar,
            );
            g_variant_type_free(type_0);
            g_free(token as gpointer);
            return ::core::ptr::null_mut::<AST>();
        }
        safe_c2rust_token_stream_next(stream);
        g_free(token as gpointer);
    } else if safe_c2rust_token_stream_consume(stream, b"boolean\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_BOOLEAN);
    } else if safe_c2rust_token_stream_consume(stream, b"byte\0" as *const u8 as *const gchar) != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_BYTE);
    } else if safe_c2rust_token_stream_consume(stream, b"int16\0" as *const u8 as *const gchar) != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_INT16);
    } else if safe_c2rust_token_stream_consume(stream, b"uint16\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_UINT16);
    } else if safe_c2rust_token_stream_consume(stream, b"int32\0" as *const u8 as *const gchar) != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_INT32);
    } else if safe_c2rust_token_stream_consume(stream, b"handle\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_HANDLE);
    } else if safe_c2rust_token_stream_consume(stream, b"uint32\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_UINT32);
    } else if safe_c2rust_token_stream_consume(stream, b"int64\0" as *const u8 as *const gchar) != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_INT64);
    } else if safe_c2rust_token_stream_consume(stream, b"uint64\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_UINT64);
    } else if safe_c2rust_token_stream_consume(stream, b"double\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_DOUBLE);
    } else if safe_c2rust_token_stream_consume(stream, b"string\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_STRING);
    } else if safe_c2rust_token_stream_consume(stream, b"objectpath\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_OBJECT_PATH);
    } else if safe_c2rust_token_stream_consume(stream, b"signature\0" as *const u8 as *const gchar)
        != 0
    {
        type_0 = g_variant_type_copy(G_VARIANT_TYPE_SIGNATURE);
    } else {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            TRUE,
            G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD as ::core::ffi::c_int as gint,
            b"unknown keyword\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<AST>();
    }
    child = safe_c2rust_parse(stream, max_depth.wrapping_sub(1 as guint), app, error);
    if child.is_null() {
        g_variant_type_free(type_0);
        return ::core::ptr::null_mut::<AST>();
    }
    decl = g_slice_alloc(::core::mem::size_of::<TypeDecl>() as gsize) as *mut TypeDecl;
    (*decl).ast.class = &raw const safe_c2rust_typedecl_class;
    (*decl).type_0 = type_0;
    (*decl).child = child;
    return decl as *mut AST;
}
unsafe extern "C" fn safe_c2rust_parse(
    mut stream: *mut TokenStream,
    mut max_depth: guint,
    mut app: *mut ::core::ffi::VaList,
    mut error: *mut *mut GError,
) -> *mut AST {
    let mut source_ref: SourceRef = SourceRef { start: 0, end: 0 };
    let mut result: *mut AST = ::core::ptr::null_mut::<AST>();
    if max_depth == 0 as guint {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            FALSE,
            G_VARIANT_PARSE_ERROR_RECURSION as ::core::ffi::c_int as gint,
            b"variant nested too deeply\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<AST>();
    }
    safe_c2rust_token_stream_prepare(stream);
    safe_c2rust_token_stream_start_ref(stream, &raw mut source_ref);
    if safe_c2rust_token_stream_peek(stream, '[' as i32 as gchar) != 0 {
        result = safe_c2rust_array_parse(stream, max_depth, app, error);
    } else if safe_c2rust_token_stream_peek(stream, '(' as i32 as gchar) != 0 {
        result = safe_c2rust_tuple_parse(stream, max_depth, app, error);
    } else if safe_c2rust_token_stream_peek(stream, '<' as i32 as gchar) != 0 {
        result = safe_c2rust_variant_parse(stream, max_depth, app, error);
    } else if safe_c2rust_token_stream_peek(stream, '{' as i32 as gchar) != 0 {
        result = safe_c2rust_dictionary_parse(stream, max_depth, app, error);
    } else if !app.is_null() && safe_c2rust_token_stream_peek(stream, '%' as i32 as gchar) != 0 {
        result = safe_c2rust_positional_parse(stream, app, error);
    } else if safe_c2rust_token_stream_consume(stream, b"true\0" as *const u8 as *const gchar) != 0
    {
        result = safe_c2rust_boolean_new(TRUE);
    } else if safe_c2rust_token_stream_consume(stream, b"false\0" as *const u8 as *const gchar) != 0
    {
        result = safe_c2rust_boolean_new(FALSE);
    } else if safe_c2rust_token_stream_is_numeric(stream) != 0
        || safe_c2rust_token_stream_peek_string(stream, b"inf\0" as *const u8 as *const gchar) != 0
        || safe_c2rust_token_stream_peek_string(stream, b"nan\0" as *const u8 as *const gchar) != 0
    {
        result = safe_c2rust_number_parse(stream, app, error);
    } else if safe_c2rust_token_stream_peek(stream, 'n' as i32 as gchar) != 0
        || safe_c2rust_token_stream_peek(stream, 'j' as i32 as gchar) != 0
    {
        result = safe_c2rust_maybe_parse(stream, max_depth, app, error);
    } else if safe_c2rust_token_stream_peek(stream, '@' as i32 as gchar) != 0
        || safe_c2rust_token_stream_is_keyword(stream) != 0
    {
        result = safe_c2rust_typedecl_parse(stream, max_depth, app, error);
    } else if safe_c2rust_token_stream_peek(stream, '\'' as i32 as gchar) != 0
        || safe_c2rust_token_stream_peek(stream, '"' as i32 as gchar) != 0
    {
        result = safe_c2rust_string_parse(stream, app, error);
    } else if safe_c2rust_token_stream_peek2(stream, 'b' as i32 as gchar, '\'' as i32 as gchar) != 0
        || safe_c2rust_token_stream_peek2(stream, 'b' as i32 as gchar, '"' as i32 as gchar) != 0
    {
        result = safe_c2rust_bytestring_parse(stream, app, error);
    } else {
        safe_c2rust_token_stream_set_error(
            stream,
            error,
            FALSE,
            G_VARIANT_PARSE_ERROR_VALUE_EXPECTED as ::core::ffi::c_int as gint,
            b"expected value\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<AST>();
    }
    if !result.is_null() {
        safe_c2rust_token_stream_end_ref(stream, &raw mut source_ref);
        (*result).source_ref = source_ref;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_parse(
    mut type_0: *const GVariantType,
    mut text: *const gchar,
    mut limit: *const gchar,
    mut endptr: *mut *const gchar,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut stream: TokenStream = TokenStream {
        start: ::core::ptr::null::<gchar>(),
        stream: ::core::ptr::null::<gchar>(),
        end: ::core::ptr::null::<gchar>(),
        this: ::core::ptr::null::<gchar>(),
    };
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ast: *mut AST = ::core::ptr::null_mut::<AST>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !text.is_null() {
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
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if text == limit || !text.is_null() {
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
            b"text == limit || text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    stream.start = text;
    stream.stream = text;
    stream.end = limit;
    ast = safe_c2rust_parse(
        &raw mut stream,
        G_VARIANT_MAX_RECURSION_DEPTH as guint,
        ::core::ptr::null_mut::<::core::ffi::VaList>(),
        error,
    );
    if !ast.is_null() {
        if type_0.is_null() {
            result = safe_c2rust_ast_resolve(ast, error);
        } else {
            result = safe_c2rust_ast_get_value(ast, type_0, error);
        }
        if !result.is_null() {
            g_variant_ref_sink(result);
            if endptr.is_null() {
                while stream.stream != limit
                    && *safe_c2rust_g_ascii_table.offset(*stream.stream as guchar as isize)
                        as ::core::ffi::c_int
                        & G_ASCII_SPACE as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    stream.stream = stream.stream.offset(1);
                }
                if stream.stream != limit && *stream.stream as ::core::ffi::c_int != '\0' as i32 {
                    let mut ref_0: SourceRef = SourceRef {
                        start: stream.stream.offset_from(text) as ::core::ffi::c_long as size_t,
                        end: stream.stream.offset_from(text) as ::core::ffi::c_long as size_t,
                    };
                    safe_c2rust_parser_set_error(
                        error,
                        &raw mut ref_0,
                        ::core::ptr::null_mut::<SourceRef>(),
                        G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END as ::core::ffi::c_int as gint,
                        b"expected end of input\0" as *const u8 as *const gchar,
                    );
                    g_variant_unref(result);
                    result = ::core::ptr::null_mut::<GVariant>();
                }
            } else {
                *endptr = stream.stream;
            }
        }
        safe_c2rust_ast_free(ast);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_parsed_va(
    mut format: *const gchar,
    mut app: *mut ::core::ffi::VaList,
) -> *mut GVariant {
    let mut stream: TokenStream = TokenStream {
        start: ::core::ptr::null::<gchar>(),
        stream: ::core::ptr::null::<gchar>(),
        end: ::core::ptr::null::<gchar>(),
        this: ::core::ptr::null::<gchar>(),
    };
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ast: *mut AST = ::core::ptr::null_mut::<AST>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !app.is_null() {
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
            b"app != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    stream.start = format;
    stream.stream = format;
    stream.end = ::core::ptr::null::<gchar>();
    ast = safe_c2rust_parse(
        &raw mut stream,
        G_VARIANT_MAX_RECURSION_DEPTH as guint,
        app,
        &raw mut error,
    );
    if !ast.is_null() {
        result = safe_c2rust_ast_resolve(ast, &raw mut error);
        safe_c2rust_ast_free(ast);
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_variant_new_parsed: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        loop {}
    }
    if *stream.stream != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_variant_new_parsed: trailing text after value\0" as *const u8 as *const gchar,
        );
        loop {}
    }
    g_clear_error(&raw mut error);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_parsed(
    mut format: *const gchar,
    mut args: ...
) -> *mut GVariant {
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    result = safe_c2rust_g_variant_new_parsed_va(format, &raw mut ap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_add_parsed(
    mut builder: *mut GVariantBuilder,
    mut format: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    g_variant_builder_add_value(
        builder,
        safe_c2rust_g_variant_new_parsed_va(format, &raw mut ap),
    );
}
unsafe extern "C" fn safe_c2rust_parse_num(
    mut num: *const gchar,
    mut limit: *const gchar,
    mut result: *mut size_t,
) -> gboolean {
    let mut endptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut bignum: gint64 = 0;
    bignum = g_ascii_strtoll(num, &raw mut endptr, 10 as guint);
    if endptr != limit as *mut gchar {
        return FALSE;
    }
    if bignum < 0 as gint64 || bignum > G_MAXINT as gint64 {
        return FALSE;
    }
    *result = bignum as size_t;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_add_last_line(mut err: *mut GString, mut str: *const gchar) {
    let mut last_nl: *const gchar = ::core::ptr::null::<gchar>();
    let mut chomped: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    chomped =
        g_strchomp(safe_c2rust_g_strdup_inline(str as *const ::core::ffi::c_char) as *mut gchar);
    last_nl = strrchr(chomped, '\n' as i32);
    if last_nl.is_null() {
        last_nl = chomped;
    } else {
        last_nl = last_nl.offset(1);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"  \0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                err,
                __val,
                if ({
                    let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_32
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
            err,
            b"  \0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if *last_nl.offset(0 as ::core::ffi::c_int as isize) != 0 {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = last_nl as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    err,
                    __val,
                    if ({
                        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_33
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
                err,
                last_nl as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"(empty input)\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    err,
                    __val,
                    if ({
                        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_34
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
                err,
                b"(empty input)\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"\n  \0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                err,
                __val,
                if ({
                    let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_35 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_35 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_35
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
            err,
            b"\n  \0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    i = 0 as size_t;
    while *last_nl.offset(i as isize) != 0 {
        safe_c2rust_g_string_append_c_inline(err, ' ' as i32 as gchar);
        i = i.wrapping_add(1);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"^\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                err,
                __val,
                if ({
                    let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_36 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_36 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_36
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
            err,
            b"^\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_free(chomped as gpointer);
}
unsafe extern "C" fn safe_c2rust_add_lines_from_range(
    mut err: *mut GString,
    mut str: *const gchar,
    mut start1: *const gchar,
    mut end1: *const gchar,
    mut start2: *const gchar,
    mut end2: *const gchar,
) {
    while str < end1 || str < end2 {
        let mut nl: *const gchar = ::core::ptr::null::<gchar>();
        nl = str.offset(strcspn(
            str as *const ::core::ffi::c_char,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
        ) as isize);
        if start1 < nl && str < end1 || start2 < nl && str < end2 {
            let mut s: *const gchar = ::core::ptr::null::<gchar>();
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"  \0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        err,
                        __val,
                        if ({
                            let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_37 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_37 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_37
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
                    err,
                    b"  \0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            safe_c2rust_g_string_append_len_inline(
                err,
                str as *const ::core::ffi::c_char,
                nl.offset_from(str) as gssize,
            );
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"\n  \0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        err,
                        __val,
                        if ({
                            let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_38 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_38 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_38
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
                    err,
                    b"\n  \0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            s = str;
            while s < nl {
                if start1 <= s && s < end1 || start2 <= s && s < end2 {
                    safe_c2rust_g_string_append_c_inline(err, '^' as i32 as gchar);
                } else {
                    safe_c2rust_g_string_append_c_inline(err, ' ' as i32 as gchar);
                }
                s = s.offset(1);
            }
            safe_c2rust_g_string_append_c_inline(err, '\n' as i32 as gchar);
        }
        if *nl == 0 {
            break;
        }
        str = nl.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_parse_error_print_context(
    mut error: *mut GError,
    mut source_str: *const gchar,
) -> *mut gchar {
    let mut current_block: u64;
    let mut colon: *const gchar = ::core::ptr::null::<gchar>();
    let mut dash: *const gchar = ::core::ptr::null::<gchar>();
    let mut comma: *const gchar = ::core::ptr::null::<gchar>();
    let mut success: gboolean = FALSE;
    let mut err: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (*error).domain == safe_c2rust_g_variant_parse_error_quark() {
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
            b"error->domain == G_VARIANT_PARSE_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    colon = strchr((*error).message, ':' as i32);
    dash = strchr((*error).message, '-' as i32);
    comma = strchr((*error).message, ',' as i32);
    if colon.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    err = g_string_new(colon.offset(1 as ::core::ffi::c_int as isize));
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b":\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                err,
                __val,
                if ({
                    let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_40 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_40 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_40
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
            err,
            b":\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if dash.is_null() || colon < dash {
        let mut point: size_t = 0;
        if safe_c2rust_parse_num((*error).message, colon, &raw mut point) == 0 {
            current_block = 11737714976466791201;
        } else {
            if point >= strlen(source_str as *const ::core::ffi::c_char) {
                safe_c2rust_add_last_line(err, source_str);
            } else {
                safe_c2rust_add_lines_from_range(
                    err,
                    source_str,
                    source_str.offset(point as isize),
                    source_str
                        .offset(point as isize)
                        .offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null::<gchar>(),
                    ::core::ptr::null::<gchar>(),
                );
            }
            current_block = 10043043949733653460;
        }
    } else if !comma.is_null() && comma < colon {
        let mut start1: size_t = 0;
        let mut end1: size_t = 0;
        let mut start2: size_t = 0;
        let mut end2: size_t = 0;
        let mut dash2: *const gchar = ::core::ptr::null::<gchar>();
        dash2 = strchr(comma as *const ::core::ffi::c_char, '-' as i32);
        if safe_c2rust_parse_num((*error).message, dash, &raw mut start1) == 0
            || safe_c2rust_parse_num(
                dash.offset(1 as ::core::ffi::c_int as isize),
                comma,
                &raw mut end1,
            ) == 0
            || safe_c2rust_parse_num(
                comma.offset(1 as ::core::ffi::c_int as isize),
                dash2,
                &raw mut start2,
            ) == 0
            || safe_c2rust_parse_num(
                dash2.offset(1 as ::core::ffi::c_int as isize),
                colon,
                &raw mut end2,
            ) == 0
        {
            current_block = 11737714976466791201;
        } else {
            safe_c2rust_add_lines_from_range(
                err,
                source_str,
                source_str.offset(start1 as isize),
                source_str.offset(end1 as isize),
                source_str.offset(start2 as isize),
                source_str.offset(end2 as isize),
            );
            current_block = 10043043949733653460;
        }
    } else {
        let mut start: size_t = 0;
        let mut end: size_t = 0;
        if safe_c2rust_parse_num((*error).message, dash, &raw mut start) == 0
            || safe_c2rust_parse_num(
                dash.offset(1 as ::core::ffi::c_int as isize),
                colon,
                &raw mut end,
            ) == 0
        {
            current_block = 11737714976466791201;
        } else {
            safe_c2rust_add_lines_from_range(
                err,
                source_str,
                source_str.offset(start as isize),
                source_str.offset(end as isize),
                ::core::ptr::null::<gchar>(),
                ::core::ptr::null::<gchar>(),
            );
            current_block = 10043043949733653460;
        }
    }
    match current_block {
        10043043949733653460 => {
            success = TRUE as gboolean;
        }
        _ => {}
    }
    return if 0 != 0 {
        if success == 0 {
            g_string_free(err, (success == 0) as ::core::ffi::c_int)
        } else {
            g_string_free_and_steal(err)
        }
    } else {
        g_string_free(err, (success == 0) as ::core::ffi::c_int)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
