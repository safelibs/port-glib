use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GData;
    pub type _GHashTable;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut safe_c2rust_stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn g_datalist_init(datalist: *mut *mut GData);
    fn g_datalist_clear(datalist: *mut *mut GData);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_foreach(hash_table: *mut GHashTable, func: GHFunc, user_data: gpointer);
    fn g_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
    fn g_ascii_strtoull(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> guint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
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
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
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
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GScanner {
    pub user_data: gpointer,
    pub max_parse_errors: guint,
    pub parse_errors: guint,
    pub input_name: *const gchar,
    pub qdata: *mut GData,
    pub config: *mut GScannerConfig,
    pub token: GTokenType,
    pub value: GTokenValue,
    pub line: guint,
    pub position: guint,
    pub next_token: GTokenType,
    pub next_value: GTokenValue,
    pub next_line: guint,
    pub next_position: guint,
    pub symbol_table: *mut GHashTable,
    pub input_fd: gint,
    pub text: *const gchar,
    pub text_end: *const gchar,
    pub buffer: *mut gchar,
    pub scope_id: guint,
    pub msg_handler: GScannerMsgFunc,
}
pub type GScannerMsgFunc = Option<unsafe extern "C" fn(*mut GScanner, *mut gchar, gboolean) -> ()>;
pub type GScanner = _GScanner;
pub type GTokenValue = _GTokenValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GTokenValue {
    pub v_symbol: gpointer,
    pub v_identifier: *mut gchar,
    pub v_binary: gulong,
    pub v_octal: gulong,
    pub v_int: gulong,
    pub v_int64: guint64,
    pub v_float: gdouble,
    pub v_hex: gulong,
    pub v_string: *mut gchar,
    pub v_comment: *mut gchar,
    pub v_char: guchar,
    pub v_error: guint,
}
pub type GTokenType = ::core::ffi::c_uint;
pub const G_TOKEN_LAST: GTokenType = 270;
pub const G_TOKEN_COMMENT_MULTI: GTokenType = 269;
pub const G_TOKEN_COMMENT_SINGLE: GTokenType = 268;
pub const G_TOKEN_IDENTIFIER_NULL: GTokenType = 267;
pub const G_TOKEN_IDENTIFIER: GTokenType = 266;
pub const G_TOKEN_SYMBOL: GTokenType = 265;
pub const G_TOKEN_STRING: GTokenType = 264;
pub const G_TOKEN_FLOAT: GTokenType = 263;
pub const G_TOKEN_HEX: GTokenType = 262;
pub const G_TOKEN_INT: GTokenType = 261;
pub const G_TOKEN_OCTAL: GTokenType = 260;
pub const G_TOKEN_BINARY: GTokenType = 259;
pub const G_TOKEN_CHAR: GTokenType = 258;
pub const G_TOKEN_ERROR: GTokenType = 257;
pub const G_TOKEN_NONE: GTokenType = 256;
pub const G_TOKEN_COMMA: GTokenType = 44;
pub const G_TOKEN_EQUAL_SIGN: GTokenType = 61;
pub const G_TOKEN_RIGHT_BRACE: GTokenType = 93;
pub const G_TOKEN_LEFT_BRACE: GTokenType = 91;
pub const G_TOKEN_RIGHT_CURLY: GTokenType = 125;
pub const G_TOKEN_LEFT_CURLY: GTokenType = 123;
pub const G_TOKEN_RIGHT_PAREN: GTokenType = 41;
pub const G_TOKEN_LEFT_PAREN: GTokenType = 40;
pub const G_TOKEN_EOF: GTokenType = 0;
pub type GScannerConfig = _GScannerConfig;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GScannerConfig {
    pub cset_skip_characters: *mut gchar,
    pub cset_identifier_first: *mut gchar,
    pub cset_identifier_nth: *mut gchar,
    pub cpair_comment_single: *mut gchar,
    #[bitfield(name = "case_sensitive", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "skip_comment_multi", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "skip_comment_single", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "scan_comment_multi", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "scan_identifier", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "scan_identifier_1char", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "scan_identifier_NULL", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "scan_symbols", ty = "guint", bits = "7..=7")]
    #[bitfield(name = "scan_binary", ty = "guint", bits = "8..=8")]
    #[bitfield(name = "scan_octal", ty = "guint", bits = "9..=9")]
    #[bitfield(name = "scan_float", ty = "guint", bits = "10..=10")]
    #[bitfield(name = "scan_hex", ty = "guint", bits = "11..=11")]
    #[bitfield(name = "scan_hex_dollar", ty = "guint", bits = "12..=12")]
    #[bitfield(name = "scan_string_sq", ty = "guint", bits = "13..=13")]
    #[bitfield(name = "scan_string_dq", ty = "guint", bits = "14..=14")]
    #[bitfield(name = "numbers_2_int", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "int_2_float", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "identifier_2_string", ty = "guint", bits = "17..=17")]
    #[bitfield(name = "char_2_token", ty = "guint", bits = "18..=18")]
    #[bitfield(name = "symbol_2_token", ty = "guint", bits = "19..=19")]
    #[bitfield(name = "scope_0_fallback", ty = "guint", bits = "20..=20")]
    #[bitfield(name = "store_int64", ty = "guint", bits = "21..=21")]
    pub case_sensitive_skip_comment_multi_skip_comment_single_scan_comment_multi_scan_identifier_scan_identifier_1char_scan_identifier_NULL_scan_symbols_scan_binary_scan_octal_scan_float_scan_hex_scan_hex_dollar_scan_string_sq_scan_string_dq_numbers_2_int_int_2_float_identifier_2_string_char_2_token_symbol_2_token_scope_0_fallback_store_int64:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub padding_dummy: guint,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ERR_FLOAT_MALFORMED: C2RustUnnamed = 7;
pub const G_ERR_FLOAT_RADIX: C2RustUnnamed = 6;
pub const G_ERR_DIGIT_RADIX: C2RustUnnamed = 5;
pub const G_ERR_NON_DIGIT_IN_CONST: C2RustUnnamed = 4;
pub const G_ERR_UNEXP_EOF_IN_COMMENT: C2RustUnnamed = 3;
pub const G_ERR_UNEXP_EOF_IN_STRING: C2RustUnnamed = 2;
pub const G_ERR_UNEXP_EOF: C2RustUnnamed = 1;
pub const G_ERR_UNKNOWN: C2RustUnnamed = 0;
pub type GScannerKey = _GScannerKey;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GScannerKey {
    pub scope_id: guint,
    pub symbol: *mut gchar,
    pub value: gpointer,
}
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_scanner_msg_handler\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
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
pub const READ_BUFFER_SIZE: ::core::ffi::c_int = 4000 as ::core::ffi::c_int;
static mut safe_c2rust_g_scanner_config_template: GScannerConfig = _GScannerConfig {
    cset_skip_characters: ::core::ptr::null::<gchar>() as *mut gchar,
    cset_identifier_first: ::core::ptr::null::<gchar>() as *mut gchar,
    cset_identifier_nth: ::core::ptr::null::<gchar>() as *mut gchar,
    cpair_comment_single: ::core::ptr::null::<gchar>() as *mut gchar,
    case_sensitive_skip_comment_multi_skip_comment_single_scan_comment_multi_scan_identifier_scan_identifier_1char_scan_identifier_NULL_scan_symbols_scan_binary_scan_octal_scan_float_scan_hex_scan_hex_dollar_scan_string_sq_scan_string_dq_numbers_2_int_int_2_float_identifier_2_string_char_2_token_symbol_2_token_scope_0_fallback_store_int64: [0; 3],
    c2rust_padding: [0; 1],
    padding_dummy: 0,
};
#[inline]
unsafe extern "C" fn safe_c2rust_g_scanner_char_2_num(mut c: guchar, mut base: guchar) -> gint {
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        c = (c as ::core::ffi::c_int - '0' as i32) as guchar;
    } else if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32 {
        c = (c as ::core::ffi::c_int - ('A' as i32 - 10 as ::core::ffi::c_int)) as guchar;
    } else if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32 {
        c = (c as ::core::ffi::c_int - ('a' as i32 - 10 as ::core::ffi::c_int)) as guchar;
    } else {
        return -(1 as gint);
    }
    if (c as ::core::ffi::c_int) < base as ::core::ffi::c_int {
        return c as gint;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_new(
    mut config_templ: *const GScannerConfig,
) -> *mut GScanner {
    let mut scanner: *mut GScanner = ::core::ptr::null_mut::<GScanner>();
    if config_templ.is_null() {
        config_templ = &raw const safe_c2rust_g_scanner_config_template;
    }
    scanner = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GScanner>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GScanner;
    (*scanner).user_data = NULL_0 as gpointer;
    (*scanner).max_parse_errors = 1 as guint;
    (*scanner).parse_errors = 0 as guint;
    (*scanner).input_name = ::core::ptr::null::<gchar>();
    g_datalist_init(&raw mut (*scanner).qdata);
    (*scanner).config = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GScannerConfig>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GScannerConfig;
    (*(*scanner).config).set_case_sensitive((*config_templ).case_sensitive() as guint);
    (*(*scanner).config).cset_skip_characters = (*config_templ).cset_skip_characters;
    if (*(*scanner).config).cset_skip_characters.is_null() {
        (*(*scanner).config).cset_skip_characters =
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    }
    (*(*scanner).config).cset_identifier_first = (*config_templ).cset_identifier_first;
    (*(*scanner).config).cset_identifier_nth = (*config_templ).cset_identifier_nth;
    (*(*scanner).config).cpair_comment_single = (*config_templ).cpair_comment_single;
    (*(*scanner).config).set_skip_comment_multi((*config_templ).skip_comment_multi() as guint);
    (*(*scanner).config).set_skip_comment_single((*config_templ).skip_comment_single() as guint);
    (*(*scanner).config).set_scan_comment_multi((*config_templ).scan_comment_multi() as guint);
    (*(*scanner).config).set_scan_identifier((*config_templ).scan_identifier() as guint);
    (*(*scanner).config)
        .set_scan_identifier_1char((*config_templ).scan_identifier_1char() as guint);
    (*(*scanner).config).set_scan_identifier_NULL((*config_templ).scan_identifier_NULL() as guint);
    (*(*scanner).config).set_scan_symbols((*config_templ).scan_symbols() as guint);
    (*(*scanner).config).set_scan_binary((*config_templ).scan_binary() as guint);
    (*(*scanner).config).set_scan_octal((*config_templ).scan_octal() as guint);
    (*(*scanner).config).set_scan_float((*config_templ).scan_float() as guint);
    (*(*scanner).config).set_scan_hex((*config_templ).scan_hex() as guint);
    (*(*scanner).config).set_scan_hex_dollar((*config_templ).scan_hex_dollar() as guint);
    (*(*scanner).config).set_scan_string_sq((*config_templ).scan_string_sq() as guint);
    (*(*scanner).config).set_scan_string_dq((*config_templ).scan_string_dq() as guint);
    (*(*scanner).config).set_numbers_2_int((*config_templ).numbers_2_int() as guint);
    (*(*scanner).config).set_int_2_float((*config_templ).int_2_float() as guint);
    (*(*scanner).config).set_identifier_2_string((*config_templ).identifier_2_string() as guint);
    (*(*scanner).config).set_char_2_token((*config_templ).char_2_token() as guint);
    (*(*scanner).config).set_symbol_2_token((*config_templ).symbol_2_token() as guint);
    (*(*scanner).config).set_scope_0_fallback((*config_templ).scope_0_fallback() as guint);
    (*(*scanner).config).set_store_int64((*config_templ).store_int64() as guint);
    (*scanner).token = G_TOKEN_NONE;
    (*scanner).value.v_int64 = 0 as guint64;
    (*scanner).line = 1 as guint;
    (*scanner).position = 0 as guint;
    (*scanner).next_token = G_TOKEN_NONE;
    (*scanner).next_value.v_int64 = 0 as guint64;
    (*scanner).next_line = 1 as guint;
    (*scanner).next_position = 0 as guint;
    (*scanner).symbol_table = g_hash_table_new(
        Some(safe_c2rust_g_scanner_key_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            safe_c2rust_g_scanner_key_equal
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
    );
    (*scanner).input_fd = -(1 as ::core::ffi::c_int) as gint;
    (*scanner).text = ::core::ptr::null::<gchar>();
    (*scanner).text_end = ::core::ptr::null::<gchar>();
    (*scanner).buffer = ::core::ptr::null_mut::<gchar>();
    (*scanner).scope_id = 0 as guint;
    (*scanner).msg_handler = Some(
        safe_c2rust_g_scanner_msg_handler
            as unsafe extern "C" fn(*mut GScanner, *mut gchar, gboolean) -> (),
    ) as GScannerMsgFunc;
    return scanner;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_scanner_free_value(
    mut token_p: *mut GTokenType,
    mut value_p: *mut GTokenValue,
) {
    match *token_p as ::core::ffi::c_uint {
        264 | 266 | 267 | 268 | 269 => {
            g_free((*value_p).v_string as gpointer);
        }
        _ => {}
    }
    *token_p = G_TOKEN_NONE;
}
unsafe extern "C" fn safe_c2rust_g_scanner_destroy_symbol_table_entry(
    mut _key: gpointer,
    mut _value: gpointer,
    mut _data: gpointer,
) {
    let mut key: *mut GScannerKey = _key as *mut GScannerKey;
    g_free((*key).symbol as gpointer);
    g_free(key as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_destroy(mut scanner: *mut GScanner) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_clear(&raw mut (*scanner).qdata);
    g_hash_table_foreach(
        (*scanner).symbol_table,
        Some(
            safe_c2rust_g_scanner_destroy_symbol_table_entry
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        NULL_0,
    );
    g_hash_table_destroy((*scanner).symbol_table);
    safe_c2rust_g_scanner_free_value(&raw mut (*scanner).token, &raw mut (*scanner).value);
    safe_c2rust_g_scanner_free_value(
        &raw mut (*scanner).next_token,
        &raw mut (*scanner).next_value,
    );
    g_free((*scanner).config as gpointer);
    g_free((*scanner).buffer as gpointer);
    g_free(scanner as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_scanner_msg_handler(
    mut scanner: *mut GScanner,
    mut message: *mut gchar,
    mut is_error: gboolean,
) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    fprintf(
        safe_c2rust_stderr,
        b"%s:%d: \0" as *const u8 as *const ::core::ffi::c_char,
        if !(*scanner).input_name.is_null() {
            (*scanner).input_name as *const ::core::ffi::c_char
        } else {
            b"<memory>\0" as *const u8 as *const ::core::ffi::c_char
        },
        (*scanner).line,
    );
    if is_error != 0 {
        fprintf(
            safe_c2rust_stderr,
            b"error: \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        safe_c2rust_stderr,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        message,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_error(
    mut scanner: *mut GScanner,
    mut format: *const gchar,
    mut args: ...
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*scanner).parse_errors = (*scanner).parse_errors.wrapping_add(1);
    if (*scanner).msg_handler.is_some() {
        let mut args_0: ::core::ffi::VaList;
        let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
        args_0 = args.clone();
        string = g_strdup_vprintf(format, args_0.clone());
        (*scanner).msg_handler.expect("non-null function pointer")(scanner, string, TRUE);
        g_free(string as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_warn(
    mut scanner: *mut GScanner,
    mut format: *const gchar,
    mut args: ...
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*scanner).msg_handler.is_some() {
        let mut args_0: ::core::ffi::VaList;
        let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
        args_0 = args.clone();
        string = g_strdup_vprintf(format, args_0.clone());
        (*scanner).msg_handler.expect("non-null function pointer")(scanner, string, FALSE);
        g_free(string as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_scanner_key_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    let mut key1: *const GScannerKey = v1 as *const GScannerKey;
    let mut key2: *const GScannerKey = v2 as *const GScannerKey;
    return ((*key1).scope_id == (*key2).scope_id
        && strcmp((*key1).symbol, (*key2).symbol) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_scanner_key_hash(mut v: gconstpointer) -> guint {
    let mut key: *const GScannerKey = v as *const GScannerKey;
    let mut c: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut h: guint = 0;
    h = (*key).scope_id;
    c = (*key).symbol;
    while *c != 0 {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_sub(h)
            .wrapping_add(*c as guint);
        c = c.offset(1);
    }
    return h;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_scanner_lookup_internal(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
    mut symbol: *const gchar,
) -> *mut GScannerKey {
    let mut key_p: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    let mut key: GScannerKey = _GScannerKey {
        scope_id: 0,
        symbol: ::core::ptr::null_mut::<gchar>(),
        value: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    key.scope_id = scope_id;
    if (*(*scanner).config).case_sensitive() == 0 {
        let mut d: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut c: *const gchar = ::core::ptr::null::<gchar>();
        key.symbol = ({
            let mut __n: gsize =
                strlen(symbol as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
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
        }) as *mut gchar;
        d = key.symbol;
        c = symbol;
        while *c != 0 {
            *d = ((*c as guchar as ::core::ffi::c_int >= 'A' as i32
                && *c as guchar as ::core::ffi::c_int <= 'Z' as i32)
                as ::core::ffi::c_int
                * ('a' as i32 - 'A' as i32)
                | (*c as guchar as ::core::ffi::c_int >= 192 as ::core::ffi::c_int
                    && *c as guchar as ::core::ffi::c_int <= 214 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    * (224 as ::core::ffi::c_int - 192 as ::core::ffi::c_int)
                | (*c as guchar as ::core::ffi::c_int >= 216 as ::core::ffi::c_int
                    && *c as guchar as ::core::ffi::c_int <= 222 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    * (248 as ::core::ffi::c_int - 216 as ::core::ffi::c_int)
                | *c as guchar as ::core::ffi::c_int) as guchar as gchar;
            c = c.offset(1);
            d = d.offset(1);
        }
        *d = 0 as gchar;
        key_p = g_hash_table_lookup((*scanner).symbol_table, &raw mut key as gconstpointer)
            as *mut GScannerKey;
        g_free(key.symbol as gpointer);
    } else {
        key.symbol = symbol as *mut gchar;
        key_p = g_hash_table_lookup((*scanner).symbol_table, &raw mut key as gconstpointer)
            as *mut GScannerKey;
    }
    return key_p;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_scope_add_symbol(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
    mut symbol: *const gchar,
    mut value: gpointer,
) {
    let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !symbol.is_null() {
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
            b"symbol != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    key = safe_c2rust_g_scanner_lookup_internal(scanner, scope_id, symbol);
    if key.is_null() {
        key = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GScannerKey>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GScannerKey;
        (*key).scope_id = scope_id;
        (*key).symbol =
            safe_c2rust_g_strdup_inline(symbol as *const ::core::ffi::c_char) as *mut gchar;
        (*key).value = value;
        if (*(*scanner).config).case_sensitive() == 0 {
            let mut c: *mut gchar = ::core::ptr::null_mut::<gchar>();
            c = (*key).symbol;
            while *c as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                *c = ((*c as guchar as ::core::ffi::c_int >= 'A' as i32
                    && *c as guchar as ::core::ffi::c_int <= 'Z' as i32)
                    as ::core::ffi::c_int
                    * ('a' as i32 - 'A' as i32)
                    | (*c as guchar as ::core::ffi::c_int >= 192 as ::core::ffi::c_int
                        && *c as guchar as ::core::ffi::c_int <= 214 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        * (224 as ::core::ffi::c_int - 192 as ::core::ffi::c_int)
                    | (*c as guchar as ::core::ffi::c_int >= 216 as ::core::ffi::c_int
                        && *c as guchar as ::core::ffi::c_int <= 222 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        * (248 as ::core::ffi::c_int - 216 as ::core::ffi::c_int)
                    | *c as guchar as ::core::ffi::c_int) as guchar as gchar;
                c = c.offset(1);
            }
        }
        g_hash_table_add((*scanner).symbol_table, key as gpointer);
    } else {
        (*key).value = value;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_scope_remove_symbol(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
    mut symbol: *const gchar,
) {
    let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !symbol.is_null() {
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
            b"symbol != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    key = safe_c2rust_g_scanner_lookup_internal(scanner, scope_id, symbol);
    if !key.is_null() {
        g_hash_table_remove((*scanner).symbol_table, key as gconstpointer);
        g_free((*key).symbol as gpointer);
        g_free(key as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_lookup_symbol(
    mut scanner: *mut GScanner,
    mut symbol: *const gchar,
) -> gpointer {
    let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    let mut scope_id: guint = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if symbol.is_null() {
        return NULL_0;
    }
    scope_id = (*scanner).scope_id;
    key = safe_c2rust_g_scanner_lookup_internal(scanner, scope_id, symbol);
    if key.is_null()
        && scope_id != 0
        && (*(*scanner).config).scope_0_fallback() as ::core::ffi::c_int != 0
    {
        key = safe_c2rust_g_scanner_lookup_internal(scanner, 0 as guint, symbol);
    }
    if !key.is_null() {
        return (*key).value;
    } else {
        return NULL_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_scope_lookup_symbol(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
    mut symbol: *const gchar,
) -> gpointer {
    let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if symbol.is_null() {
        return NULL_0;
    }
    key = safe_c2rust_g_scanner_lookup_internal(scanner, scope_id, symbol);
    if !key.is_null() {
        return (*key).value;
    } else {
        return NULL_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_set_scope(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
) -> guint {
    let mut old_scope_id: guint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    old_scope_id = (*scanner).scope_id;
    (*scanner).scope_id = scope_id;
    return old_scope_id;
}
unsafe extern "C" fn safe_c2rust_g_scanner_foreach_internal(
    mut _key: gpointer,
    mut _value: gpointer,
    mut _user_data: gpointer,
) {
    let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
    let mut d: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    let mut func: GHFunc = None;
    let mut user_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut scope_id: *mut guint = ::core::ptr::null_mut::<guint>();
    d = _user_data as *mut gpointer;
    func = ::core::mem::transmute::<gpointer, GHFunc>(*d.offset(0 as ::core::ffi::c_int as isize));
    user_data = *d.offset(1 as ::core::ffi::c_int as isize);
    scope_id = *d.offset(2 as ::core::ffi::c_int as isize) as *mut guint;
    key = _value as *mut GScannerKey;
    if (*key).scope_id == *scope_id {
        func.expect("non-null function pointer")(
            (*key).symbol as gpointer,
            (*key).value,
            user_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_scope_foreach_symbol(
    mut scanner: *mut GScanner,
    mut scope_id: guint,
    mut func: GHFunc,
    mut user_data: gpointer,
) {
    let mut d: [gpointer; 3] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 3];
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    d[0 as ::core::ffi::c_int as usize] = ::core::mem::transmute::<GHFunc, gpointer>(func);
    d[1 as ::core::ffi::c_int as usize] = user_data;
    d[2 as ::core::ffi::c_int as usize] = &raw mut scope_id as gpointer;
    g_hash_table_foreach(
        (*scanner).symbol_table,
        Some(
            safe_c2rust_g_scanner_foreach_internal
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        &raw mut d as *mut gpointer as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_peek_next_token(
    mut scanner: *mut GScanner,
) -> GTokenType {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TOKEN_EOF;
    }
    if (*scanner).next_token as ::core::ffi::c_uint
        == G_TOKEN_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*scanner).next_line = (*scanner).line;
        (*scanner).next_position = (*scanner).position;
        safe_c2rust_g_scanner_get_token_i(
            scanner,
            &raw mut (*scanner).next_token,
            &raw mut (*scanner).next_value,
            &raw mut (*scanner).next_line,
            &raw mut (*scanner).next_position,
        );
    }
    return (*scanner).next_token;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_get_next_token(
    mut scanner: *mut GScanner,
) -> GTokenType {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TOKEN_EOF;
    }
    if (*scanner).next_token as ::core::ffi::c_uint
        != G_TOKEN_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_scanner_free_value(&raw mut (*scanner).token, &raw mut (*scanner).value);
        (*scanner).token = (*scanner).next_token;
        (*scanner).value = (*scanner).next_value;
        (*scanner).line = (*scanner).next_line;
        (*scanner).position = (*scanner).next_position;
        (*scanner).next_token = G_TOKEN_NONE;
    } else {
        safe_c2rust_g_scanner_get_token_i(
            scanner,
            &raw mut (*scanner).token,
            &raw mut (*scanner).value,
            &raw mut (*scanner).line,
            &raw mut (*scanner).position,
        );
    }
    return (*scanner).token;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_cur_token(mut scanner: *mut GScanner) -> GTokenType {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TOKEN_EOF;
    }
    return (*scanner).token;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_cur_value(
    mut scanner: *mut GScanner,
) -> GTokenValue {
    let mut v: GTokenValue = _GTokenValue {
        v_symbol: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    v.v_int64 = 0 as guint64;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return v;
    }
    v = (*scanner).value;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_cur_line(mut scanner: *mut GScanner) -> guint {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*scanner).line;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_cur_position(mut scanner: *mut GScanner) -> guint {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*scanner).position;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_eof(mut scanner: *mut GScanner) -> gboolean {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return ((*scanner).token as ::core::ffi::c_uint
        == G_TOKEN_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*scanner).token as ::core::ffi::c_uint
            == G_TOKEN_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_input_file(
    mut scanner: *mut GScanner,
    mut input_fd: gint,
) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if input_fd >= 0 as ::core::ffi::c_int {
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
            b"input_fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*scanner).input_fd >= 0 as ::core::ffi::c_int {
        safe_c2rust_g_scanner_sync_file_offset(scanner);
    }
    (*scanner).token = G_TOKEN_NONE;
    (*scanner).value.v_int64 = 0 as guint64;
    (*scanner).line = 1 as guint;
    (*scanner).position = 0 as guint;
    (*scanner).next_token = G_TOKEN_NONE;
    (*scanner).input_fd = input_fd;
    (*scanner).text = ::core::ptr::null::<gchar>();
    (*scanner).text_end = ::core::ptr::null::<gchar>();
    if (*scanner).buffer.is_null() {
        (*scanner).buffer = ({
            let mut __n: gsize = (4000 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        }) as *mut gchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_input_text(
    mut scanner: *mut GScanner,
    mut text: *const gchar,
    mut text_len: guint,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if text_len != 0 {
        if ({
            let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
            if !text.is_null() {
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
                b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    } else {
        text = ::core::ptr::null::<gchar>();
    }
    if (*scanner).input_fd >= 0 as ::core::ffi::c_int {
        safe_c2rust_g_scanner_sync_file_offset(scanner);
    }
    (*scanner).token = G_TOKEN_NONE;
    (*scanner).value.v_int64 = 0 as guint64;
    (*scanner).line = 1 as guint;
    (*scanner).position = 0 as guint;
    (*scanner).next_token = G_TOKEN_NONE;
    (*scanner).input_fd = -(1 as ::core::ffi::c_int) as gint;
    (*scanner).text = text;
    (*scanner).text_end = text.offset(text_len as isize);
    if !(*scanner).buffer.is_null() {
        g_free((*scanner).buffer as gpointer);
        (*scanner).buffer = ::core::ptr::null_mut::<gchar>();
    }
}
unsafe extern "C" fn safe_c2rust_g_scanner_peek_next_char(mut scanner: *mut GScanner) -> guchar {
    if (*scanner).text < (*scanner).text_end {
        return *(*scanner).text as guchar;
    } else if (*scanner).input_fd >= 0 as ::core::ffi::c_int {
        let mut count: gint = 0;
        let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
        buffer = (*scanner).buffer;
        loop {
            count = read(
                (*scanner).input_fd as ::core::ffi::c_int,
                buffer as *mut ::core::ffi::c_void,
                READ_BUFFER_SIZE as size_t,
            ) as gint;
            if !(count == -(1 as ::core::ffi::c_int)
                && (*__errno_location() == EINTR || *__errno_location() == EAGAIN))
            {
                break;
            }
        }
        if count < 1 as ::core::ffi::c_int {
            (*scanner).input_fd = -(1 as ::core::ffi::c_int) as gint;
            return 0 as guchar;
        } else {
            (*scanner).text = buffer;
            (*scanner).text_end = buffer.offset(count as isize);
            return *buffer as guchar;
        }
    } else {
        return 0 as guchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_sync_file_offset(mut scanner: *mut GScanner) {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*scanner).input_fd >= 0 as ::core::ffi::c_int && (*scanner).text_end > (*scanner).text {
        let mut buffered: gint = 0;
        buffered = (*scanner).text_end.offset_from((*scanner).text) as ::core::ffi::c_long as gint;
        if lseek(
            (*scanner).input_fd as ::core::ffi::c_int,
            -buffered as __off64_t,
            SEEK_CUR,
        ) >= 0 as __off64_t
        {
            (*scanner).text = ::core::ptr::null::<gchar>();
            (*scanner).text_end = ::core::ptr::null::<gchar>();
        } else {
            *__errno_location() = 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_scanner_get_char(
    mut scanner: *mut GScanner,
    mut line_p: *mut guint,
    mut position_p: *mut guint,
) -> guchar {
    let mut fchar: guchar = 0;
    if (*scanner).text < (*scanner).text_end {
        let fresh1 = (*scanner).text;
        (*scanner).text = (*scanner).text.offset(1);
        fchar = *fresh1 as guchar;
    } else if (*scanner).input_fd >= 0 as ::core::ffi::c_int {
        let mut count: gint = 0;
        let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
        buffer = (*scanner).buffer;
        loop {
            count = read(
                (*scanner).input_fd as ::core::ffi::c_int,
                buffer as *mut ::core::ffi::c_void,
                READ_BUFFER_SIZE as size_t,
            ) as gint;
            if !(count == -(1 as ::core::ffi::c_int)
                && (*__errno_location() == EINTR || *__errno_location() == EAGAIN))
            {
                break;
            }
        }
        if count < 1 as ::core::ffi::c_int {
            (*scanner).input_fd = -(1 as ::core::ffi::c_int) as gint;
            fchar = 0 as guchar;
        } else {
            (*scanner).text = buffer.offset(1 as ::core::ffi::c_int as isize);
            (*scanner).text_end = buffer.offset(count as isize);
            fchar = *buffer as guchar;
            if fchar == 0 {
                safe_c2rust_g_scanner_sync_file_offset(scanner);
                (*scanner).text_end = (*scanner).text;
                (*scanner).input_fd = -(1 as ::core::ffi::c_int) as gint;
            }
        }
    } else {
        fchar = 0 as guchar;
    }
    if fchar as ::core::ffi::c_int == '\n' as i32 {
        *position_p = 0 as guint;
        *line_p = (*line_p).wrapping_add(1);
    } else if fchar != 0 {
        *position_p = (*position_p).wrapping_add(1);
    }
    return fchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_scanner_unexp_token(
    mut scanner: *mut GScanner,
    mut expected_token: GTokenType,
    mut identifier_spec: *const gchar,
    mut symbol_spec: *const gchar,
    mut symbol_name: *const gchar,
    mut message: *const gchar,
    mut is_error: gint,
) {
    let mut token_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut token_string_len: guint = 0;
    let mut expected_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut expected_string_len: guint = 0;
    let mut message_prefix: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut print_unexp: gboolean = 0;
    let mut msg_handler: Option<unsafe extern "C" fn(*mut GScanner, *const gchar, ...) -> ()> =
        None;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !scanner.is_null() {
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
            b"scanner != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if is_error != 0 {
        msg_handler = Some(
            safe_c2rust_g_scanner_error
                as unsafe extern "C" fn(*mut GScanner, *const gchar, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*mut GScanner, *const gchar, ...) -> ()>;
    } else {
        msg_handler = Some(
            safe_c2rust_g_scanner_warn
                as unsafe extern "C" fn(*mut GScanner, *const gchar, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*mut GScanner, *const gchar, ...) -> ()>;
    }
    if identifier_spec.is_null() {
        identifier_spec =
            b"identifier\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if symbol_spec.is_null() {
        symbol_spec = b"symbol\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    token_string_len = 56 as guint;
    token_string = ({
        let mut __n: gsize = token_string_len.wrapping_add(1 as guint) as gsize;
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
    }) as *mut gchar;
    expected_string_len = 64 as guint;
    expected_string = ({
        let mut __n: gsize = expected_string_len.wrapping_add(1 as guint) as gsize;
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
    }) as *mut gchar;
    print_unexp = TRUE as gboolean;
    let mut current_block_57: u64;
    match (*scanner).token as ::core::ffi::c_uint {
        0 => {
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"end of file\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block_57 = 13707613154239713890;
        }
        265 => {
            current_block_57 = 4946879549817297424;
        }
        257 => {
            print_unexp = FALSE as gboolean;
            expected_token = G_TOKEN_NONE;
            match (*scanner).value.v_error {
                1 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: unexpected end of file\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                2 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: unterminated string constant\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                3 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: unterminated comment\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                4 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: non digit in constant\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                6 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: invalid radix for floating constant\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                7 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: malformed floating constant\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                5 => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: digit is beyond radix\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                0 | _ => {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"scanner: unknown error\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
            current_block_57 = 13707613154239713890;
        }
        258 => {
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"character '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                (*scanner).value.v_char as ::core::ffi::c_int,
            );
            current_block_57 = 13707613154239713890;
        }
        266 | 267 => {
            if expected_token as ::core::ffi::c_uint
                == G_TOKEN_IDENTIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
                || expected_token as ::core::ffi::c_uint
                    == G_TOKEN_IDENTIFIER_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                print_unexp = FALSE as gboolean;
            }
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"%s%s '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                if print_unexp != 0 {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"invalid \0" as *const u8 as *const ::core::ffi::c_char
                },
                identifier_spec,
                if (*scanner).token as ::core::ffi::c_uint
                    == G_TOKEN_IDENTIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*scanner).value.v_string as *const gchar
                } else {
                    b"null\0" as *const u8 as *const gchar
                },
            );
            current_block_57 = 13707613154239713890;
        }
        259 | 260 | 261 | 262 => {
            if (*(*scanner).config).store_int64() != 0 {
                snprintf(
                    token_string as *mut ::core::ffi::c_char,
                    token_string_len as size_t,
                    b"number '%lu'\0" as *const u8 as *const ::core::ffi::c_char,
                    (*scanner).value.v_int64,
                );
            } else {
                snprintf(
                    token_string as *mut ::core::ffi::c_char,
                    token_string_len as size_t,
                    b"number '%lu'\0" as *const u8 as *const ::core::ffi::c_char,
                    (*scanner).value.v_int,
                );
            }
            current_block_57 = 13707613154239713890;
        }
        263 => {
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"number '%.3f'\0" as *const u8 as *const ::core::ffi::c_char,
                (*scanner).value.v_float,
            );
            current_block_57 = 13707613154239713890;
        }
        264 => {
            if expected_token as ::core::ffi::c_uint
                == G_TOKEN_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                print_unexp = FALSE as gboolean;
            }
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"%s%sstring constant \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if print_unexp != 0 {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"invalid \0" as *const u8 as *const ::core::ffi::c_char
                },
                if *(*scanner)
                    .value
                    .v_string
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    b"empty \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*scanner).value.v_string,
            );
            *token_string.offset(token_string_len.wrapping_sub(2 as guint) as isize) =
                '"' as i32 as gchar;
            *token_string.offset(token_string_len.wrapping_sub(1 as guint) as isize) = 0 as gchar;
            current_block_57 = 13707613154239713890;
        }
        268 | 269 => {
            snprintf(
                token_string as *mut ::core::ffi::c_char,
                token_string_len as size_t,
                b"comment\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block_57 = 13707613154239713890;
        }
        256 => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gscanner.c\0" as *const u8 as *const ::core::ffi::c_char,
                1491 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        _ => {
            if (*scanner).token as ::core::ffi::c_uint >= 1 as ::core::ffi::c_uint
                && (*scanner).token as ::core::ffi::c_uint <= 255 as ::core::ffi::c_uint
            {
                if (*scanner).token as ::core::ffi::c_uint >= ' ' as i32 as ::core::ffi::c_uint
                    && (*scanner).token as ::core::ffi::c_uint <= '~' as i32 as ::core::ffi::c_uint
                    || !strchr(
                        (*(*scanner).config).cset_identifier_first,
                        (*scanner).token as ::core::ffi::c_int,
                    )
                    .is_null()
                    || !strchr(
                        (*(*scanner).config).cset_identifier_nth,
                        (*scanner).token as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"character '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                        (*scanner).token as ::core::ffi::c_uint,
                    );
                } else {
                    snprintf(
                        token_string as *mut ::core::ffi::c_char,
                        token_string_len as size_t,
                        b"character '\\%o'\0" as *const u8 as *const ::core::ffi::c_char,
                        (*scanner).token as ::core::ffi::c_uint,
                    );
                }
                current_block_57 = 13707613154239713890;
            } else if (*(*scanner).config).symbol_2_token() == 0 {
                snprintf(
                    token_string as *mut ::core::ffi::c_char,
                    token_string_len as size_t,
                    b"(unknown) token <%d>\0" as *const u8 as *const ::core::ffi::c_char,
                    (*scanner).token as ::core::ffi::c_uint,
                );
                current_block_57 = 13707613154239713890;
            } else {
                current_block_57 = 4946879549817297424;
            }
        }
    }
    match current_block_57 {
        4946879549817297424 => {
            if expected_token as ::core::ffi::c_uint
                == G_TOKEN_SYMBOL as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*scanner).config).symbol_2_token() as ::core::ffi::c_int != 0
                    && expected_token as ::core::ffi::c_uint
                        > G_TOKEN_LAST as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                print_unexp = FALSE as gboolean;
            }
            if !symbol_name.is_null() {
                snprintf(
                    token_string as *mut ::core::ffi::c_char,
                    token_string_len as size_t,
                    b"%s%s '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    if print_unexp != 0 {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"invalid \0" as *const u8 as *const ::core::ffi::c_char
                    },
                    symbol_spec,
                    symbol_name,
                );
            } else {
                snprintf(
                    token_string as *mut ::core::ffi::c_char,
                    token_string_len as size_t,
                    b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if print_unexp != 0 {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"invalid \0" as *const u8 as *const ::core::ffi::c_char
                    },
                    symbol_spec,
                );
            }
        }
        _ => {}
    }
    let mut need_valid: gboolean = 0;
    let mut tstring: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut current_block_84: u64;
    match expected_token as ::core::ffi::c_uint {
        0 => {
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"end of file\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block_84 = 9500030526577190060;
        }
        265 => {
            current_block_84 = 12820061367173135591;
        }
        258 => {
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%scharacter\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint
                    == G_TOKEN_CHAR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            current_block_84 = 9500030526577190060;
        }
        259 => {
            tstring = b"binary\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%snumber (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        260 => {
            tstring = b"octal\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%snumber (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        261 => {
            tstring = b"integer\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%snumber (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        262 => {
            tstring = b"hexadecimal\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%snumber (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        263 => {
            tstring = b"float\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%snumber (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        264 => {
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%sstring constant\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint
                    == G_TOKEN_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            current_block_84 = 9500030526577190060;
        }
        266 | 267 => {
            need_valid = ((*scanner).token as ::core::ffi::c_uint
                == G_TOKEN_IDENTIFIER_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*scanner).token as ::core::ffi::c_uint
                    == G_TOKEN_IDENTIFIER as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int as gboolean;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                if need_valid != 0 {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                identifier_spec,
            );
            current_block_84 = 9500030526577190060;
        }
        268 => {
            tstring = b"single-line\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%scomment (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        269 => {
            tstring = b"multi-line\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%scomment (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                if (*scanner).token as ::core::ffi::c_uint == expected_token as ::core::ffi::c_uint
                {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                tstring,
            );
            current_block_84 = 9500030526577190060;
        }
        256 | 257 => {
            current_block_84 = 9500030526577190060;
        }
        _ => {
            if expected_token as ::core::ffi::c_uint >= 1 as ::core::ffi::c_uint
                && expected_token as ::core::ffi::c_uint <= 255 as ::core::ffi::c_uint
            {
                if expected_token as ::core::ffi::c_uint >= ' ' as i32 as ::core::ffi::c_uint
                    && expected_token as ::core::ffi::c_uint <= '~' as i32 as ::core::ffi::c_uint
                    || !strchr(
                        (*(*scanner).config).cset_identifier_first,
                        expected_token as ::core::ffi::c_int,
                    )
                    .is_null()
                    || !strchr(
                        (*(*scanner).config).cset_identifier_nth,
                        expected_token as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    snprintf(
                        expected_string as *mut ::core::ffi::c_char,
                        expected_string_len as size_t,
                        b"character '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                        expected_token as ::core::ffi::c_uint,
                    );
                } else {
                    snprintf(
                        expected_string as *mut ::core::ffi::c_char,
                        expected_string_len as size_t,
                        b"character '\\%o'\0" as *const u8 as *const ::core::ffi::c_char,
                        expected_token as ::core::ffi::c_uint,
                    );
                }
                current_block_84 = 9500030526577190060;
            } else if (*(*scanner).config).symbol_2_token() == 0 {
                snprintf(
                    expected_string as *mut ::core::ffi::c_char,
                    expected_string_len as size_t,
                    b"(unknown) token <%d>\0" as *const u8 as *const ::core::ffi::c_char,
                    expected_token as ::core::ffi::c_uint,
                );
                current_block_84 = 9500030526577190060;
            } else {
                current_block_84 = 12820061367173135591;
            }
        }
    }
    match current_block_84 {
        12820061367173135591 => {
            need_valid = ((*scanner).token as ::core::ffi::c_uint
                == G_TOKEN_SYMBOL as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*scanner).config).symbol_2_token() as ::core::ffi::c_int != 0
                    && (*scanner).token as ::core::ffi::c_uint
                        > G_TOKEN_LAST as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int as gboolean;
            snprintf(
                expected_string as *mut ::core::ffi::c_char,
                expected_string_len as size_t,
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                if need_valid != 0 {
                    b"valid \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                symbol_spec,
            );
        }
        _ => {}
    }
    if !message.is_null()
        && *message.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        message_prefix = b" - \0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    } else {
        message_prefix = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
        message = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if expected_token as ::core::ffi::c_uint
        == G_TOKEN_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        msg_handler.expect("non-null function pointer")(
            scanner,
            b"failure around %s%s%s\0" as *const u8 as *const gchar,
            token_string,
            message_prefix,
            message,
        );
    } else if expected_token as ::core::ffi::c_uint
        == G_TOKEN_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if print_unexp != 0 {
            msg_handler.expect("non-null function pointer")(
                scanner,
                b"unexpected %s%s%s\0" as *const u8 as *const gchar,
                token_string,
                message_prefix,
                message,
            );
        } else {
            msg_handler.expect("non-null function pointer")(
                scanner,
                b"%s%s%s\0" as *const u8 as *const gchar,
                token_string,
                message_prefix,
                message,
            );
        }
    } else if print_unexp != 0 {
        msg_handler.expect("non-null function pointer")(
            scanner,
            b"unexpected %s, expected %s%s%s\0" as *const u8 as *const gchar,
            token_string,
            expected_string,
            message_prefix,
            message,
        );
    } else {
        msg_handler.expect("non-null function pointer")(
            scanner,
            b"%s, expected %s%s%s\0" as *const u8 as *const gchar,
            token_string,
            expected_string,
            message_prefix,
            message,
        );
    }
    g_free(token_string as gpointer);
    g_free(expected_string as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_scanner_get_token_i(
    mut scanner: *mut GScanner,
    mut token_p: *mut GTokenType,
    mut value_p: *mut GTokenValue,
    mut line_p: *mut guint,
    mut position_p: *mut guint,
) {
    loop {
        safe_c2rust_g_scanner_free_value(token_p, value_p);
        safe_c2rust_g_scanner_get_token_ll(scanner, token_p, value_p, line_p, position_p);
        if !(*token_p as ::core::ffi::c_uint > 0 as ::core::ffi::c_uint
            && (*token_p as ::core::ffi::c_uint) < 256 as ::core::ffi::c_uint
            && !strchr(
                (*(*scanner).config).cset_skip_characters,
                *token_p as ::core::ffi::c_int,
            )
            .is_null()
            || *token_p as ::core::ffi::c_uint
                == G_TOKEN_CHAR as ::core::ffi::c_int as ::core::ffi::c_uint
                && !strchr(
                    (*(*scanner).config).cset_skip_characters,
                    (*value_p).v_char as ::core::ffi::c_int,
                )
                .is_null()
            || *token_p as ::core::ffi::c_uint
                == G_TOKEN_COMMENT_MULTI as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*scanner).config).skip_comment_multi() as ::core::ffi::c_int != 0
            || *token_p as ::core::ffi::c_uint
                == G_TOKEN_COMMENT_SINGLE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*scanner).config).skip_comment_single() as ::core::ffi::c_int != 0)
        {
            break;
        }
    }
    match *token_p as ::core::ffi::c_uint {
        266 => {
            if (*(*scanner).config).identifier_2_string() != 0 {
                *token_p = G_TOKEN_STRING;
            }
        }
        265 => {
            if (*(*scanner).config).symbol_2_token() != 0 {
                *token_p = (*value_p).v_symbol as size_t as GTokenType;
            }
        }
        259 | 260 | 262 => {
            if (*(*scanner).config).numbers_2_int() != 0 {
                *token_p = G_TOKEN_INT;
            }
        }
        _ => {}
    }
    if *token_p as ::core::ffi::c_uint == G_TOKEN_INT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*scanner).config).int_2_float() as ::core::ffi::c_int != 0
    {
        *token_p = G_TOKEN_FLOAT;
        if (*(*scanner).config).store_int64() != 0 {
            let mut temp: gint64 = (*value_p).v_int64 as gint64;
            (*value_p).v_float = temp as gdouble;
        } else {
            let mut temp_0: gint = (*value_p).v_int as gint;
            (*value_p).v_float = temp_0 as gdouble;
        }
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_scanner_get_token_ll(
    mut scanner: *mut GScanner,
    mut token_p: *mut GTokenType,
    mut value_p: *mut GTokenValue,
    mut line_p: *mut guint,
    mut position_p: *mut guint,
) {
    let mut current_block: u64;
    let mut config: *mut GScannerConfig = ::core::ptr::null_mut::<GScannerConfig>();
    let mut token: GTokenType = G_TOKEN_EOF;
    let mut in_comment_multi: gboolean = 0;
    let mut in_comment_single: gboolean = 0;
    let mut in_string_sq: gboolean = 0;
    let mut in_string_dq: gboolean = 0;
    let mut gstring: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut value: GTokenValue = _GTokenValue {
        v_symbol: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut ch: guchar = 0;
    config = (*scanner).config;
    (*value_p).v_int64 = 0 as guint64;
    if (*scanner).text >= (*scanner).text_end && (*scanner).input_fd < 0 as ::core::ffi::c_int
        || (*scanner).token as ::core::ffi::c_uint
            == G_TOKEN_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *token_p = G_TOKEN_EOF;
        return;
    }
    in_comment_multi = FALSE as gboolean;
    in_comment_single = FALSE as gboolean;
    in_string_sq = FALSE as gboolean;
    in_string_dq = FALSE as gboolean;
    gstring = ::core::ptr::null_mut::<GString>();
    loop {
        let mut dotted_float: gboolean = FALSE;
        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
        value.v_int64 = 0 as guint64;
        token = G_TOKEN_NONE;
        if (*config).scan_identifier() as ::core::ffi::c_int != 0
            && ch as ::core::ffi::c_int != 0
            && !strchr((*config).cset_identifier_first, ch as ::core::ffi::c_int).is_null()
        {
            current_block = 10226750860685273701;
        } else {
            match ch as ::core::ffi::c_int {
                0 => {
                    token = G_TOKEN_EOF;
                    *position_p = (*position_p).wrapping_add(1);
                    current_block = 6950536787749910113;
                }
                47 => {
                    if (*config).scan_comment_multi() == 0
                        || safe_c2rust_g_scanner_peek_next_char(scanner) as ::core::ffi::c_int
                            != '*' as i32
                    {
                        current_block = 145651165234646754;
                    } else {
                        safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        token = G_TOKEN_COMMENT_MULTI;
                        in_comment_multi = TRUE as gboolean;
                        gstring = g_string_new(::core::ptr::null::<gchar>());
                        loop {
                            ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                            if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                break;
                            }
                            if ch as ::core::ffi::c_int == '*' as i32
                                && safe_c2rust_g_scanner_peek_next_char(scanner)
                                    as ::core::ffi::c_int
                                    == '/' as i32
                            {
                                safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                                in_comment_multi = FALSE as gboolean;
                                break;
                            } else {
                                gstring =
                                    safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                            }
                        }
                        ch = 0 as guchar;
                        current_block = 6950536787749910113;
                    }
                }
                39 => {
                    if (*config).scan_string_sq() == 0 {
                        current_block = 145651165234646754;
                    } else {
                        token = G_TOKEN_STRING;
                        in_string_sq = TRUE as gboolean;
                        gstring = g_string_new(::core::ptr::null::<gchar>());
                        loop {
                            ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                            if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                break;
                            }
                            if ch as ::core::ffi::c_int == '\'' as i32 {
                                in_string_sq = FALSE as gboolean;
                                break;
                            } else {
                                gstring =
                                    safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                            }
                        }
                        ch = 0 as guchar;
                        current_block = 6950536787749910113;
                    }
                }
                34 => {
                    if (*config).scan_string_dq() == 0 {
                        current_block = 145651165234646754;
                    } else {
                        token = G_TOKEN_STRING;
                        in_string_dq = TRUE as gboolean;
                        gstring = g_string_new(::core::ptr::null::<gchar>());
                        loop {
                            ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                            if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                break;
                            }
                            if ch as ::core::ffi::c_int == '"' as i32 {
                                in_string_dq = FALSE as gboolean;
                                break;
                            } else if ch as ::core::ffi::c_int == '\\' as i32 {
                                ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                                let mut i: guint = 0;
                                let mut fchar: guint = 0;
                                match ch as ::core::ffi::c_int {
                                    0 => {}
                                    92 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\\' as i32 as gchar,
                                        );
                                    }
                                    110 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\n' as i32 as gchar,
                                        );
                                    }
                                    116 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\t' as i32 as gchar,
                                        );
                                    }
                                    114 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\r' as i32 as gchar,
                                        );
                                    }
                                    98 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\u{8}' as i32 as gchar,
                                        );
                                    }
                                    102 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            '\u{c}' as i32 as gchar,
                                        );
                                    }
                                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                                        i = (ch as ::core::ffi::c_int - '0' as i32) as guint;
                                        fchar =
                                            safe_c2rust_g_scanner_peek_next_char(scanner) as guint;
                                        if fchar >= '0' as i32 as guint
                                            && fchar <= '7' as i32 as guint
                                        {
                                            ch = safe_c2rust_g_scanner_get_char(
                                                scanner, line_p, position_p,
                                            );
                                            i = i
                                                .wrapping_mul(8 as guint)
                                                .wrapping_add(ch as guint)
                                                .wrapping_sub('0' as i32 as guint);
                                            fchar = safe_c2rust_g_scanner_peek_next_char(scanner)
                                                as guint;
                                            if fchar >= '0' as i32 as guint
                                                && fchar <= '7' as i32 as guint
                                            {
                                                ch = safe_c2rust_g_scanner_get_char(
                                                    scanner, line_p, position_p,
                                                );
                                                i = i
                                                    .wrapping_mul(8 as guint)
                                                    .wrapping_add(ch as guint)
                                                    .wrapping_sub('0' as i32 as guint);
                                            }
                                        }
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring, i as gchar,
                                        );
                                    }
                                    _ => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            ch as gchar,
                                        );
                                    }
                                }
                            } else {
                                gstring =
                                    safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                            }
                        }
                        ch = 0 as guchar;
                        current_block = 6950536787749910113;
                    }
                }
                46 => {
                    if (*config).scan_float() == 0 {
                        current_block = 145651165234646754;
                    } else {
                        token = G_TOKEN_FLOAT;
                        dotted_float = TRUE as gboolean;
                        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        current_block = 2424750145044873408;
                    }
                }
                36 => {
                    if (*config).scan_hex_dollar() == 0 {
                        current_block = 145651165234646754;
                    } else {
                        token = G_TOKEN_HEX;
                        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        current_block = 2424750145044873408;
                    }
                }
                48 => {
                    if (*config).scan_octal() != 0 {
                        token = G_TOKEN_OCTAL;
                    } else {
                        token = G_TOKEN_INT;
                    }
                    ch = safe_c2rust_g_scanner_peek_next_char(scanner);
                    if (*config).scan_hex() as ::core::ffi::c_int != 0
                        && (ch as ::core::ffi::c_int == 'x' as i32
                            || ch as ::core::ffi::c_int == 'X' as i32)
                    {
                        token = G_TOKEN_HEX;
                        safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        if ch as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            token = G_TOKEN_ERROR;
                            value.v_error = G_ERR_UNEXP_EOF as ::core::ffi::c_int as guint;
                            *position_p = (*position_p).wrapping_add(1);
                            current_block = 6950536787749910113;
                        } else if safe_c2rust_g_scanner_char_2_num(ch, 16 as guchar)
                            < 0 as ::core::ffi::c_int
                        {
                            token = G_TOKEN_ERROR;
                            value.v_error = G_ERR_DIGIT_RADIX as ::core::ffi::c_int as guint;
                            ch = 0 as guchar;
                            current_block = 6950536787749910113;
                        } else {
                            current_block = 2424750145044873408;
                        }
                    } else if (*config).scan_binary() as ::core::ffi::c_int != 0
                        && (ch as ::core::ffi::c_int == 'b' as i32
                            || ch as ::core::ffi::c_int == 'B' as i32)
                    {
                        token = G_TOKEN_BINARY;
                        safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        if ch as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            token = G_TOKEN_ERROR;
                            value.v_error = G_ERR_UNEXP_EOF as ::core::ffi::c_int as guint;
                            *position_p = (*position_p).wrapping_add(1);
                            current_block = 6950536787749910113;
                        } else if safe_c2rust_g_scanner_char_2_num(ch, 10 as guchar)
                            < 0 as ::core::ffi::c_int
                        {
                            token = G_TOKEN_ERROR;
                            value.v_error = G_ERR_NON_DIGIT_IN_CONST as ::core::ffi::c_int as guint;
                            ch = 0 as guchar;
                            current_block = 6950536787749910113;
                        } else {
                            current_block = 2424750145044873408;
                        }
                    } else {
                        ch = '0' as i32 as guchar;
                        current_block = 2424750145044873408;
                    }
                }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 2424750145044873408;
                }
                _ => {
                    current_block = 145651165234646754;
                }
            }
            match current_block {
                6950536787749910113 => {}
                _ => match current_block {
                    145651165234646754 => {
                        if !(*config).cpair_comment_single.is_null()
                            && ch as ::core::ffi::c_int
                                == *(*config)
                                    .cpair_comment_single
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                        {
                            token = G_TOKEN_COMMENT_SINGLE;
                            in_comment_single = TRUE as gboolean;
                            gstring = g_string_new(::core::ptr::null::<gchar>());
                            ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                            while ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                if ch as ::core::ffi::c_int
                                    == *(*config)
                                        .cpair_comment_single
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                {
                                    in_comment_single = FALSE as gboolean;
                                    ch = 0 as guchar;
                                    break;
                                } else {
                                    gstring =
                                        safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                                    ch =
                                        safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                                }
                            }
                            if in_comment_single != 0
                                && *(*config)
                                    .cpair_comment_single
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32
                            {
                                in_comment_single = FALSE as gboolean;
                            }
                            current_block = 13077018311583886556;
                        } else if (*config).scan_identifier() as ::core::ffi::c_int != 0
                            && ch as ::core::ffi::c_int != 0
                            && !strchr((*config).cset_identifier_first, ch as ::core::ffi::c_int)
                                .is_null()
                        {
                            current_block = 10226750860685273701;
                        } else {
                            current_block = 13077018311583886556;
                        }
                    }
                    _ => {
                        let mut in_number: gboolean = TRUE;
                        let mut endptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
                        if token as ::core::ffi::c_uint
                            == G_TOKEN_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            token = G_TOKEN_INT;
                        }
                        gstring = g_string_new(if dotted_float != 0 {
                            b"0.\0" as *const u8 as *const gchar
                        } else {
                            b"\0" as *const u8 as *const gchar
                        });
                        gstring = safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                        loop {
                            let mut is_E: gboolean = 0;
                            is_E = (token as ::core::ffi::c_uint
                                == G_TOKEN_FLOAT as ::core::ffi::c_int as ::core::ffi::c_uint
                                && (ch as ::core::ffi::c_int == 'e' as i32
                                    || ch as ::core::ffi::c_int == 'E' as i32))
                                as ::core::ffi::c_int
                                as gboolean;
                            ch = safe_c2rust_g_scanner_peek_next_char(scanner);
                            if safe_c2rust_g_scanner_char_2_num(ch, 36 as guchar)
                                >= 0 as ::core::ffi::c_int
                                || (*config).scan_float() as ::core::ffi::c_int != 0
                                    && ch as ::core::ffi::c_int == '.' as i32
                                || is_E != 0
                                    && (ch as ::core::ffi::c_int == '+' as i32
                                        || ch as ::core::ffi::c_int == '-' as i32)
                            {
                                ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                                match ch as ::core::ffi::c_int {
                                    46 => {
                                        if token as ::core::ffi::c_uint
                                            != G_TOKEN_INT as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            && token as ::core::ffi::c_uint
                                                != G_TOKEN_OCTAL as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                        {
                                            value.v_error = (if token as ::core::ffi::c_uint
                                                == G_TOKEN_FLOAT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                G_ERR_FLOAT_MALFORMED as ::core::ffi::c_int
                                            } else {
                                                G_ERR_FLOAT_RADIX as ::core::ffi::c_int
                                            })
                                                as guint;
                                            token = G_TOKEN_ERROR;
                                            in_number = FALSE as gboolean;
                                        } else {
                                            token = G_TOKEN_FLOAT;
                                            gstring = safe_c2rust_g_string_append_c_inline(
                                                gstring,
                                                ch as gchar,
                                            );
                                        }
                                    }
                                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                        gstring = safe_c2rust_g_string_append_c_inline(
                                            gstring,
                                            ch as gchar,
                                        );
                                    }
                                    45 | 43 => {
                                        if token as ::core::ffi::c_uint
                                            != G_TOKEN_FLOAT as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            token = G_TOKEN_ERROR;
                                            value.v_error = G_ERR_NON_DIGIT_IN_CONST
                                                as ::core::ffi::c_int
                                                as guint;
                                            in_number = FALSE as gboolean;
                                        } else {
                                            gstring = safe_c2rust_g_string_append_c_inline(
                                                gstring,
                                                ch as gchar,
                                            );
                                        }
                                    }
                                    101 | 69 => {
                                        if token as ::core::ffi::c_uint
                                            != G_TOKEN_HEX as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            && (*config).scan_float() == 0
                                            || token as ::core::ffi::c_uint
                                                != G_TOKEN_HEX as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                                && token as ::core::ffi::c_uint
                                                    != G_TOKEN_OCTAL as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && token as ::core::ffi::c_uint
                                                    != G_TOKEN_FLOAT as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && token as ::core::ffi::c_uint
                                                    != G_TOKEN_INT as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                        {
                                            token = G_TOKEN_ERROR;
                                            value.v_error = G_ERR_NON_DIGIT_IN_CONST
                                                as ::core::ffi::c_int
                                                as guint;
                                            in_number = FALSE as gboolean;
                                        } else {
                                            if token as ::core::ffi::c_uint
                                                != G_TOKEN_HEX as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                token = G_TOKEN_FLOAT;
                                            }
                                            gstring = safe_c2rust_g_string_append_c_inline(
                                                gstring,
                                                ch as gchar,
                                            );
                                        }
                                    }
                                    _ => {
                                        if token as ::core::ffi::c_uint
                                            != G_TOKEN_HEX as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            token = G_TOKEN_ERROR;
                                            value.v_error = G_ERR_NON_DIGIT_IN_CONST
                                                as ::core::ffi::c_int
                                                as guint;
                                            in_number = FALSE as gboolean;
                                        } else {
                                            gstring = safe_c2rust_g_string_append_c_inline(
                                                gstring,
                                                ch as gchar,
                                            );
                                        }
                                    }
                                }
                            } else {
                                in_number = FALSE as gboolean;
                            }
                            if !(in_number != 0) {
                                break;
                            }
                        }
                        endptr = ::core::ptr::null_mut::<gchar>();
                        if token as ::core::ffi::c_uint
                            == G_TOKEN_FLOAT as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            value.v_float = g_strtod((*gstring).str_0, &raw mut endptr);
                        } else {
                            let mut ui64: guint64 = 0 as guint64;
                            match token as ::core::ffi::c_uint {
                                259 => {
                                    ui64 = g_ascii_strtoull(
                                        (*gstring).str_0,
                                        &raw mut endptr,
                                        2 as guint,
                                    );
                                }
                                260 => {
                                    ui64 = g_ascii_strtoull(
                                        (*gstring).str_0,
                                        &raw mut endptr,
                                        8 as guint,
                                    );
                                }
                                261 => {
                                    ui64 = g_ascii_strtoull(
                                        (*gstring).str_0,
                                        &raw mut endptr,
                                        10 as guint,
                                    );
                                }
                                262 => {
                                    ui64 = g_ascii_strtoull(
                                        (*gstring).str_0,
                                        &raw mut endptr,
                                        16 as guint,
                                    );
                                }
                                _ => {}
                            }
                            if (*(*scanner).config).store_int64() != 0 {
                                value.v_int64 = ui64;
                            } else {
                                value.v_int = ui64 as gulong;
                            }
                        }
                        if !endptr.is_null() && *endptr as ::core::ffi::c_int != 0 {
                            token = G_TOKEN_ERROR;
                            if *endptr as ::core::ffi::c_int == 'e' as i32
                                || *endptr as ::core::ffi::c_int == 'E' as i32
                            {
                                value.v_error =
                                    G_ERR_NON_DIGIT_IN_CONST as ::core::ffi::c_int as guint;
                            } else {
                                value.v_error = G_ERR_DIGIT_RADIX as ::core::ffi::c_int as guint;
                            }
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
                        gstring = ::core::ptr::null_mut::<GString>();
                        ch = 0 as guchar;
                        current_block = 6950536787749910113;
                    }
                },
            }
        }
        match current_block {
            10226750860685273701 => {
                if !(*config).cset_identifier_nth.is_null()
                    && ch as ::core::ffi::c_int != 0
                    && !strchr(
                        (*config).cset_identifier_nth,
                        safe_c2rust_g_scanner_peek_next_char(scanner) as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    token = G_TOKEN_IDENTIFIER;
                    gstring = g_string_new(::core::ptr::null::<gchar>());
                    gstring = safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                    loop {
                        ch = safe_c2rust_g_scanner_get_char(scanner, line_p, position_p);
                        gstring = safe_c2rust_g_string_append_c_inline(gstring, ch as gchar);
                        ch = safe_c2rust_g_scanner_peek_next_char(scanner);
                        if !(ch as ::core::ffi::c_int != 0
                            && !strchr((*config).cset_identifier_nth, ch as ::core::ffi::c_int)
                                .is_null())
                        {
                            break;
                        }
                    }
                    ch = 0 as guchar;
                } else if (*config).scan_identifier_1char() != 0 {
                    token = G_TOKEN_IDENTIFIER;
                    value.v_identifier = ({
                        let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
                        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
                        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                        if __s == 1 as gsize {
                            __p = g_malloc0(__n);
                        } else if 0 != 0
                            && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s))
                        {
                            __p = g_malloc0(__n.wrapping_mul(__s));
                        } else {
                            __p = g_malloc0_n(__n, __s);
                        }
                        __p
                    }) as *mut gchar;
                    *value.v_identifier.offset(0 as ::core::ffi::c_int as isize) = ch as gchar;
                    ch = 0 as guchar;
                }
                current_block = 13077018311583886556;
            }
            _ => {}
        }
        match current_block {
            13077018311583886556 => {
                if ch != 0 {
                    if (*config).char_2_token() != 0 {
                        token = ch as GTokenType;
                    } else {
                        token = G_TOKEN_CHAR;
                        value.v_char = ch;
                    }
                    ch = 0 as guchar;
                }
            }
            _ => {}
        }
        if ({
            let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
            if ch as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && token as ::core::ffi::c_uint
                    != G_TOKEN_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _g_boolean_var_35 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_35 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_35
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gscanner.c\0" as *const u8 as *const ::core::ffi::c_char,
                2182 as ::core::ffi::c_int,
                G_STRFUNC,
                b"ch == 0 && token != G_TOKEN_NONE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    if in_comment_multi != 0 || in_comment_single != 0 || in_string_sq != 0 || in_string_dq != 0 {
        token = G_TOKEN_ERROR;
        if !gstring.is_null() {
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
            gstring = ::core::ptr::null_mut::<GString>();
        }
        *position_p = (*position_p).wrapping_add(1);
        if in_comment_multi != 0 || in_comment_single != 0 {
            value.v_error = G_ERR_UNEXP_EOF_IN_COMMENT as ::core::ffi::c_int as guint;
        } else {
            value.v_error = G_ERR_UNEXP_EOF_IN_STRING as ::core::ffi::c_int as guint;
        }
    }
    if !gstring.is_null() {
        value.v_string = if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(gstring, 0 as gboolean)
            } else {
                g_string_free_and_steal(gstring)
            }
        } else {
            g_string_free(gstring, 0 as gboolean)
        };
        gstring = ::core::ptr::null_mut::<GString>();
    }
    if token as ::core::ffi::c_uint
        == G_TOKEN_IDENTIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*config).scan_symbols() != 0 {
            let mut key: *mut GScannerKey = ::core::ptr::null_mut::<GScannerKey>();
            let mut scope_id: guint = 0;
            scope_id = (*scanner).scope_id;
            key = safe_c2rust_g_scanner_lookup_internal(scanner, scope_id, value.v_identifier);
            if key.is_null()
                && scope_id != 0
                && (*(*scanner).config).scope_0_fallback() as ::core::ffi::c_int != 0
            {
                key =
                    safe_c2rust_g_scanner_lookup_internal(scanner, 0 as guint, value.v_identifier);
            }
            if !key.is_null() {
                g_free(value.v_identifier as gpointer);
                token = G_TOKEN_SYMBOL;
                value.v_symbol = (*key).value;
            }
        }
        if token as ::core::ffi::c_uint
            == G_TOKEN_IDENTIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*config).scan_identifier_NULL() as ::core::ffi::c_int != 0
            && strlen(value.v_identifier) == 4 as size_t
        {
            let mut null_upper: *mut gchar =
                b"NULL\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            let mut null_lower: *mut gchar =
                b"null\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            if (*(*scanner).config).case_sensitive() != 0 {
                if *value.v_identifier.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == *null_upper.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    && *value.v_identifier.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_upper.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    && *value.v_identifier.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_upper.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    && *value.v_identifier.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_upper.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                {
                    token = G_TOKEN_IDENTIFIER_NULL;
                }
            } else if (*value.v_identifier.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                == *null_upper.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                || *value.v_identifier.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == *null_lower.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                && (*value.v_identifier.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == *null_upper.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    || *value.v_identifier.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_lower.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)
                && (*value.v_identifier.offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == *null_upper.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    || *value.v_identifier.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_lower.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)
                && (*value.v_identifier.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == *null_upper.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    || *value.v_identifier.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == *null_lower.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)
            {
                token = G_TOKEN_IDENTIFIER_NULL;
            }
        }
    }
    *token_p = token;
    *value_p = value;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
unsafe extern "C" fn run_static_initializers() {
    safe_c2rust_g_scanner_config_template = {
        let mut init = _GScannerConfig {
            case_sensitive_skip_comment_multi_skip_comment_single_scan_comment_multi_scan_identifier_scan_identifier_1char_scan_identifier_NULL_scan_symbols_scan_binary_scan_octal_scan_float_scan_hex_scan_hex_dollar_scan_string_sq_scan_string_dq_numbers_2_int_int_2_float_identifier_2_string_char_2_token_symbol_2_token_scope_0_fallback_store_int64: [0; 3],
            c2rust_padding: [0; 1],
            cset_skip_characters: b" \t\r\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            cset_identifier_first: b"abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
                as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            cset_identifier_nth: b"abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\xF3\xF4\xF5\xF6\xF8\xF9\xFA\xFB\xFC\xFD\xFE\xFF\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\xD2\xD3\xD4\xD5\xD6\xD8\xD9\xDA\xDB\xDC\xDD\xDE\0"
                as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            cpair_comment_single: b"#\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            padding_dummy: 0 as guint,
        };
        init.set_case_sensitive(FALSE as guint);
        init.set_skip_comment_multi(TRUE as guint);
        init.set_skip_comment_single(TRUE as guint);
        init.set_scan_comment_multi(TRUE as guint);
        init.set_scan_identifier(TRUE as guint);
        init.set_scan_identifier_1char(FALSE as guint);
        init.set_scan_identifier_NULL(FALSE as guint);
        init.set_scan_symbols(TRUE as guint);
        init.set_scan_binary(FALSE as guint);
        init.set_scan_octal(TRUE as guint);
        init.set_scan_float(TRUE as guint);
        init.set_scan_hex(TRUE as guint);
        init.set_scan_hex_dollar(FALSE as guint);
        init.set_scan_string_sq(TRUE as guint);
        init.set_scan_string_dq(TRUE as guint);
        init.set_numbers_2_int(TRUE as guint);
        init.set_int_2_float(FALSE as guint);
        init.set_identifier_2_string(FALSE as guint);
        init.set_char_2_token(TRUE as guint);
        init.set_symbol_2_token(FALSE as guint);
        init.set_scope_0_fallback(FALSE as guint);
        init.set_store_int64(FALSE as guint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
