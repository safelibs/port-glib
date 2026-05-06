extern "C" {
    pub type pcre2_real_general_context_8;
    pub type pcre2_real_compile_context_8;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_match_data_8;
    pub type pcre2_real_jit_stack_8;
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
    fn pcre2_set_newline_8(_: *mut pcre2_compile_context_8, _: uint32_t) -> ::core::ffi::c_int;
    fn pcre2_jit_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> ::core::ffi::c_int;
    fn pcre2_get_ovector_count_8(_: *mut pcre2_match_data_8) -> uint32_t;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> ::core::ffi::c_int;
    fn pcre2_dfa_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
        _: *mut ::core::ffi::c_int,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn pcre2_match_data_create_from_pattern_8(
        _: *const pcre2_code_8,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_match_data_create_8(
        _: uint32_t,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_pattern_info_8(
        _: *const pcre2_code_8,
        _: uint32_t,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pcre2_code_free_8(_: *mut pcre2_code_8);
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut ::core::ffi::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_match_context_free_8(_: *mut pcre2_match_context_8);
    fn pcre2_match_context_create_8(_: *mut pcre2_general_context_8) -> *mut pcre2_match_context_8;
    fn pcre2_get_error_message_8(
        _: ::core::ffi::c_int,
        _: *mut PCRE2_UCHAR8,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn pcre2_set_bsr_8(_: *mut pcre2_compile_context_8, _: uint32_t) -> ::core::ffi::c_int;
    fn pcre2_substring_nametable_scan_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: *mut PCRE2_SPTR8,
        _: *mut PCRE2_SPTR8,
    ) -> ::core::ffi::c_int;
    fn pcre2_compile_context_free_8(_: *mut pcre2_compile_context_8);
    fn pcre2_compile_context_create_8(
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_compile_context_8;
    fn pcre2_substring_number_from_name_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
    ) -> ::core::ffi::c_int;
    fn pcre2_jit_compile_8(_: *mut pcre2_code_8, _: uint32_t) -> ::core::ffi::c_int;
    fn pcre2_get_ovector_pointer_8(_: *mut pcre2_match_data_8) -> *mut size_t;
    fn pcre2_jit_stack_create_8(
        _: size_t,
        _: size_t,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_jit_stack_8;
    fn pcre2_config_8(_: uint32_t, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn pcre2_jit_stack_assign_8(
        _: *mut pcre2_match_context_8,
        _: pcre2_jit_callback_8,
        _: *mut ::core::ffi::c_void,
    );
    fn pcre2_jit_stack_free_8(_: *mut pcre2_jit_stack_8);
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_unichar_toupper(c: gunichar) -> gunichar;
    fn g_unichar_tolower(c: gunichar) -> gunichar;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_pointer_to_offset(str: *const gchar, pos: *const gchar) -> glong;
    fn g_utf8_prev_char(p: *const gchar) -> *mut gchar;
    fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_digit_value(c: gchar) -> gint;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_unichar(string: *mut GString, wc: gunichar) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
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
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type PCRE2_UCHAR8 = uint8_t;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_code_8 = pcre2_real_code_8;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type pcre2_jit_stack_8 = pcre2_real_jit_stack_8;
pub type pcre2_jit_callback_8 =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut pcre2_jit_stack_8>;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_REGEX_ERROR_CHARACTER_VALUE_TOO_LARGE: C2RustUnnamed_0 = 176;
pub const G_REGEX_ERROR_NAME_TOO_LONG: C2RustUnnamed_0 = 175;
pub const G_REGEX_ERROR_TOO_MANY_FORWARD_REFERENCES: C2RustUnnamed_0 = 172;
pub const G_REGEX_ERROR_NOT_SUPPORTED_IN_CLASS: C2RustUnnamed_0 = 171;
pub const G_REGEX_ERROR_MISSING_NAME: C2RustUnnamed_0 = 169;
pub const G_REGEX_ERROR_INVALID_CONTROL_CHAR: C2RustUnnamed_0 = 168;
pub const G_REGEX_ERROR_BACKTRACKING_CONTROL_VERB_ARGUMENT_REQUIRED: C2RustUnnamed_0 = 166;
pub const G_REGEX_ERROR_EXTRA_SUBPATTERN_NAME: C2RustUnnamed_0 = 165;
pub const G_REGEX_ERROR_INVALID_DATA_CHARACTER: C2RustUnnamed_0 = 164;
pub const G_REGEX_ERROR_MISSING_DIGIT: C2RustUnnamed_0 = 163;
pub const G_REGEX_ERROR_MISSING_SUBPATTERN_NAME: C2RustUnnamed_0 = 162;
pub const G_REGEX_ERROR_NUMBER_TOO_BIG: C2RustUnnamed_0 = 161;
pub const G_REGEX_ERROR_UNKNOWN_BACKTRACKING_CONTROL_VERB: C2RustUnnamed_0 = 160;
pub const G_REGEX_ERROR_BACKTRACKING_CONTROL_VERB_ARGUMENT_FORBIDDEN: C2RustUnnamed_0 = 159;
pub const G_REGEX_ERROR_INVALID_RELATIVE_REFERENCE: C2RustUnnamed_0 = 158;
pub const G_REGEX_ERROR_MISSING_BACK_REFERENCE: C2RustUnnamed_0 = 157;
pub const G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS: C2RustUnnamed_0 = 156;
pub const G_REGEX_ERROR_DEFINE_REPETION: C2RustUnnamed_0 = 155;
pub const G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE: C2RustUnnamed_0 = 154;
pub const G_REGEX_ERROR_INVALID_OCTAL_VALUE: C2RustUnnamed_0 = 151;
pub const G_REGEX_ERROR_TOO_MANY_SUBPATTERNS: C2RustUnnamed_0 = 149;
pub const G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG: C2RustUnnamed_0 = 148;
pub const G_REGEX_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_0 = 147;
pub const G_REGEX_ERROR_MALFORMED_PROPERTY: C2RustUnnamed_0 = 146;
pub const G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME: C2RustUnnamed_0 = 143;
pub const G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR: C2RustUnnamed_0 = 142;
pub const G_REGEX_ERROR_INFINITE_LOOP: C2RustUnnamed_0 = 140;
pub const G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND: C2RustUnnamed_0 = 136;
pub const G_REGEX_ERROR_INVALID_CONDITION: C2RustUnnamed_0 = 135;
pub const G_REGEX_ERROR_HEX_CODE_TOO_LARGE: C2RustUnnamed_0 = 134;
pub const G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED: C2RustUnnamed_0 = 131;
pub const G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME: C2RustUnnamed_0 = 130;
pub const G_REGEX_ERROR_ASSERTION_EXPECTED: C2RustUnnamed_0 = 128;
pub const G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES: C2RustUnnamed_0 = 127;
pub const G_REGEX_ERROR_MALFORMED_CONDITION: C2RustUnnamed_0 = 126;
pub const G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND: C2RustUnnamed_0 = 125;
pub const G_REGEX_ERROR_MEMORY_ERROR: C2RustUnnamed_0 = 121;
pub const G_REGEX_ERROR_EXPRESSION_TOO_LARGE: C2RustUnnamed_0 = 120;
pub const G_REGEX_ERROR_UNTERMINATED_COMMENT: C2RustUnnamed_0 = 118;
pub const G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE: C2RustUnnamed_0 = 115;
pub const G_REGEX_ERROR_UNMATCHED_PARENTHESIS: C2RustUnnamed_0 = 114;
pub const G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS: C2RustUnnamed_0 = 113;
pub const G_REGEX_ERROR_UNRECOGNIZED_CHARACTER: C2RustUnnamed_0 = 112;
pub const G_REGEX_ERROR_NOTHING_TO_REPEAT: C2RustUnnamed_0 = 109;
pub const G_REGEX_ERROR_RANGE_OUT_OF_ORDER: C2RustUnnamed_0 = 108;
pub const G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS: C2RustUnnamed_0 = 107;
pub const G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS: C2RustUnnamed_0 = 106;
pub const G_REGEX_ERROR_QUANTIFIER_TOO_BIG: C2RustUnnamed_0 = 105;
pub const G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER: C2RustUnnamed_0 = 104;
pub const G_REGEX_ERROR_UNRECOGNIZED_ESCAPE: C2RustUnnamed_0 = 103;
pub const G_REGEX_ERROR_MISSING_CONTROL_CHAR: C2RustUnnamed_0 = 102;
pub const G_REGEX_ERROR_STRAY_BACKSLASH: C2RustUnnamed_0 = 101;
pub const G_REGEX_ERROR_INTERNAL: C2RustUnnamed_0 = 4;
pub const G_REGEX_ERROR_MATCH: C2RustUnnamed_0 = 3;
pub const G_REGEX_ERROR_REPLACE: C2RustUnnamed_0 = 2;
pub const G_REGEX_ERROR_OPTIMIZE: C2RustUnnamed_0 = 1;
pub const G_REGEX_ERROR_COMPILE: C2RustUnnamed_0 = 0;
pub type GRegexCompileFlags = ::core::ffi::c_uint;
pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
pub type GRegexMatchFlags = ::core::ffi::c_uint;
pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRegex {
    pub ref_count: gint,
    pub pattern: *mut gchar,
    pub pcre_re: *mut pcre2_code_8,
    pub compile_opts: uint32_t,
    pub orig_compile_opts: GRegexCompileFlags,
    pub match_opts: uint32_t,
    pub orig_match_opts: GRegexMatchFlags,
    pub jit_options: uint32_t,
    pub jit_status: JITStatus,
}
pub type JITStatus = ::core::ffi::c_uint;
pub const JIT_STATUS_DISABLED: JITStatus = 2;
pub const JIT_STATUS_ENABLED: JITStatus = 1;
pub const JIT_STATUS_DEFAULT: JITStatus = 0;
pub type GRegex = _GRegex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMatchInfo {
    pub ref_count: gint,
    pub regex: *mut GRegex,
    pub match_opts: uint32_t,
    pub matches: gint,
    pub n_subpatterns: uint32_t,
    pub pos: gint,
    pub n_offsets: uint32_t,
    pub offsets: *mut gint,
    pub workspace: *mut gint,
    pub n_workspace: size_t,
    pub string: *const gchar,
    pub string_len: gssize,
    pub match_context: *mut pcre2_match_context_8,
    pub match_data: *mut pcre2_match_data_8,
    pub jit_stack: *mut pcre2_jit_stack_8,
}
pub type GMatchInfo = _GMatchInfo;
pub type GRegexEvalCallback =
    Option<unsafe extern "C" fn(*const GMatchInfo, *mut GString, gpointer) -> gboolean>;
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
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type InterpolationData = _InterpolationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _InterpolationData {
    pub text: *mut gchar,
    pub type_0: gint,
    pub num: gint,
    pub c: gchar,
    pub change_case: ChangeCase,
}
pub type ChangeCase = ::core::ffi::c_uint;
pub const CHANGE_CASE_UPPER_MASK: ChangeCase = 10;
pub const CHANGE_CASE_LOWER_MASK: ChangeCase = 20;
pub const CHANGE_CASE_SINGLE_MASK: ChangeCase = 24;
pub const CHANGE_CASE_LOWER_SINGLE: ChangeCase = 16;
pub const CHANGE_CASE_UPPER_SINGLE: ChangeCase = 8;
pub const CHANGE_CASE_LOWER: ChangeCase = 4;
pub const CHANGE_CASE_UPPER: ChangeCase = 2;
pub const CHANGE_CASE_NONE: ChangeCase = 1;
pub const REPL_TYPE_CHANGE_CASE: C2RustUnnamed_1 = 4;
pub const REPL_TYPE_SYMBOLIC_REFERENCE: C2RustUnnamed_1 = 2;
pub const REPL_TYPE_NUMERIC_REFERENCE: C2RustUnnamed_1 = 3;
pub const REPL_TYPE_CHARACTER: C2RustUnnamed_1 = 1;
pub const REPL_TYPE_STRING: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"get_pcre2_error_string\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const PCRE2_ANCHORED: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const PCRE2_NO_UTF_CHECK: ::core::ffi::c_uint = 0x40000000 as ::core::ffi::c_uint;
pub const PCRE2_ENDANCHORED: ::core::ffi::c_uint = 0x20000000 as ::core::ffi::c_uint;
pub const PCRE2_ALLOW_EMPTY_CLASS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PCRE2_ALT_BSUX: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const PCRE2_AUTO_CALLOUT: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PCRE2_CASELESS: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PCRE2_DOLLAR_ENDONLY: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PCRE2_DOTALL: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const PCRE2_DUPNAMES: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const PCRE2_EXTENDED: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const PCRE2_FIRSTLINE: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const PCRE2_MATCH_UNSET_BACKREF: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const PCRE2_MULTILINE: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const PCRE2_NEVER_UCP: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const PCRE2_NEVER_UTF: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const PCRE2_NO_AUTO_CAPTURE: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const PCRE2_NO_AUTO_POSSESS: ::core::ffi::c_uint = 0x4000 as ::core::ffi::c_uint;
pub const PCRE2_NO_DOTSTAR_ANCHOR: ::core::ffi::c_uint = 0x8000 as ::core::ffi::c_uint;
pub const PCRE2_NO_START_OPTIMIZE: ::core::ffi::c_uint = 0x10000 as ::core::ffi::c_uint;
pub const PCRE2_UCP: ::core::ffi::c_uint = 0x20000 as ::core::ffi::c_uint;
pub const PCRE2_UNGREEDY: ::core::ffi::c_uint = 0x40000 as ::core::ffi::c_uint;
pub const PCRE2_UTF: ::core::ffi::c_uint = 0x80000 as ::core::ffi::c_uint;
pub const PCRE2_NEVER_BACKSLASH_C: ::core::ffi::c_uint = 0x100000 as ::core::ffi::c_uint;
pub const PCRE2_ALT_CIRCUMFLEX: ::core::ffi::c_uint = 0x200000 as ::core::ffi::c_uint;
pub const PCRE2_ALT_VERBNAMES: ::core::ffi::c_uint = 0x400000 as ::core::ffi::c_uint;
pub const PCRE2_USE_OFFSET_LIMIT: ::core::ffi::c_uint = 0x800000 as ::core::ffi::c_uint;
pub const PCRE2_EXTENDED_MORE: ::core::ffi::c_uint = 0x1000000 as ::core::ffi::c_uint;
pub const PCRE2_LITERAL: ::core::ffi::c_uint = 0x2000000 as ::core::ffi::c_uint;
pub const PCRE2_MATCH_INVALID_UTF: ::core::ffi::c_uint = 0x4000000 as ::core::ffi::c_uint;
pub const PCRE2_JIT_COMPLETE: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PCRE2_JIT_PARTIAL_SOFT: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const PCRE2_JIT_PARTIAL_HARD: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PCRE2_NOTBOL: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PCRE2_NOTEOL: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const PCRE2_NOTEMPTY: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PCRE2_NOTEMPTY_ATSTART: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PCRE2_PARTIAL_SOFT: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PCRE2_PARTIAL_HARD: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const PCRE2_NO_JIT: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const PCRE2_COPY_MATCHED_SUBJECT: ::core::ffi::c_uint = 0x4000 as ::core::ffi::c_uint;
pub const PCRE2_NEWLINE_CR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PCRE2_NEWLINE_LF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PCRE2_NEWLINE_CRLF: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PCRE2_NEWLINE_ANY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PCRE2_NEWLINE_ANYCRLF: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PCRE2_BSR_UNICODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PCRE2_BSR_ANYCRLF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PCRE2_ERROR_END_BACKSLASH: ::core::ffi::c_int = 101;
pub const PCRE2_ERROR_END_BACKSLASH_C: ::core::ffi::c_int = 102;
pub const PCRE2_ERROR_UNKNOWN_ESCAPE: ::core::ffi::c_int = 103;
pub const PCRE2_ERROR_QUANTIFIER_OUT_OF_ORDER: ::core::ffi::c_int = 104;
pub const PCRE2_ERROR_QUANTIFIER_TOO_BIG: ::core::ffi::c_int = 105;
pub const PCRE2_ERROR_MISSING_SQUARE_BRACKET: ::core::ffi::c_int = 106;
pub const PCRE2_ERROR_ESCAPE_INVALID_IN_CLASS: ::core::ffi::c_int = 107;
pub const PCRE2_ERROR_CLASS_RANGE_ORDER: ::core::ffi::c_int = 108;
pub const PCRE2_ERROR_QUANTIFIER_INVALID: ::core::ffi::c_int = 109;
pub const PCRE2_ERROR_INTERNAL_UNEXPECTED_REPEAT: ::core::ffi::c_int = 110;
pub const PCRE2_ERROR_INVALID_AFTER_PARENS_QUERY: ::core::ffi::c_int = 111;
pub const PCRE2_ERROR_POSIX_CLASS_NOT_IN_CLASS: ::core::ffi::c_int = 112;
pub const PCRE2_ERROR_POSIX_NO_SUPPORT_COLLATING: ::core::ffi::c_int = 113;
pub const PCRE2_ERROR_MISSING_CLOSING_PARENTHESIS: ::core::ffi::c_int = 114;
pub const PCRE2_ERROR_BAD_SUBPATTERN_REFERENCE: ::core::ffi::c_int = 115;
pub const PCRE2_ERROR_NULL_PATTERN: ::core::ffi::c_int = 116;
pub const PCRE2_ERROR_BAD_OPTIONS: ::core::ffi::c_int = 117;
pub const PCRE2_ERROR_MISSING_COMMENT_CLOSING: ::core::ffi::c_int = 118;
pub const PCRE2_ERROR_PARENTHESES_NEST_TOO_DEEP: ::core::ffi::c_int = 119;
pub const PCRE2_ERROR_PATTERN_TOO_LARGE: ::core::ffi::c_int = 120;
pub const PCRE2_ERROR_HEAP_FAILED: ::core::ffi::c_int = 121;
pub const PCRE2_ERROR_UNMATCHED_CLOSING_PARENTHESIS: ::core::ffi::c_int = 122;
pub const PCRE2_ERROR_INTERNAL_CODE_OVERFLOW: ::core::ffi::c_int = 123;
pub const PCRE2_ERROR_MISSING_CONDITION_CLOSING: ::core::ffi::c_int = 124;
pub const PCRE2_ERROR_LOOKBEHIND_NOT_FIXED_LENGTH: ::core::ffi::c_int = 125;
pub const PCRE2_ERROR_ZERO_RELATIVE_REFERENCE: ::core::ffi::c_int = 126;
pub const PCRE2_ERROR_TOO_MANY_CONDITION_BRANCHES: ::core::ffi::c_int = 127;
pub const PCRE2_ERROR_CONDITION_ASSERTION_EXPECTED: ::core::ffi::c_int = 128;
pub const PCRE2_ERROR_BAD_RELATIVE_REFERENCE: ::core::ffi::c_int = 129;
pub const PCRE2_ERROR_UNKNOWN_POSIX_CLASS: ::core::ffi::c_int = 130;
pub const PCRE2_ERROR_INTERNAL_STUDY_ERROR: ::core::ffi::c_int = 131;
pub const PCRE2_ERROR_UNICODE_NOT_SUPPORTED: ::core::ffi::c_int = 132;
pub const PCRE2_ERROR_PARENTHESES_STACK_CHECK: ::core::ffi::c_int = 133;
pub const PCRE2_ERROR_CODE_POINT_TOO_BIG: ::core::ffi::c_int = 134;
pub const PCRE2_ERROR_LOOKBEHIND_TOO_COMPLICATED: ::core::ffi::c_int = 135;
pub const PCRE2_ERROR_LOOKBEHIND_INVALID_BACKSLASH_C: ::core::ffi::c_int = 136;
pub const PCRE2_ERROR_UNSUPPORTED_ESCAPE_SEQUENCE: ::core::ffi::c_int = 137;
pub const PCRE2_ERROR_CALLOUT_NUMBER_TOO_BIG: ::core::ffi::c_int = 138;
pub const PCRE2_ERROR_MISSING_CALLOUT_CLOSING: ::core::ffi::c_int = 139;
pub const PCRE2_ERROR_ESCAPE_INVALID_IN_VERB: ::core::ffi::c_int = 140;
pub const PCRE2_ERROR_UNRECOGNIZED_AFTER_QUERY_P: ::core::ffi::c_int = 141;
pub const PCRE2_ERROR_MISSING_NAME_TERMINATOR: ::core::ffi::c_int = 142;
pub const PCRE2_ERROR_DUPLICATE_SUBPATTERN_NAME: ::core::ffi::c_int = 143;
pub const PCRE2_ERROR_INVALID_SUBPATTERN_NAME: ::core::ffi::c_int = 144;
pub const PCRE2_ERROR_UNICODE_PROPERTIES_UNAVAILABLE: ::core::ffi::c_int = 145;
pub const PCRE2_ERROR_MALFORMED_UNICODE_PROPERTY: ::core::ffi::c_int = 146;
pub const PCRE2_ERROR_UNKNOWN_UNICODE_PROPERTY: ::core::ffi::c_int = 147;
pub const PCRE2_ERROR_SUBPATTERN_NAME_TOO_LONG: ::core::ffi::c_int = 148;
pub const PCRE2_ERROR_TOO_MANY_NAMED_SUBPATTERNS: ::core::ffi::c_int = 149;
pub const PCRE2_ERROR_CLASS_INVALID_RANGE: ::core::ffi::c_int = 150;
pub const PCRE2_ERROR_OCTAL_BYTE_TOO_BIG: ::core::ffi::c_int = 151;
pub const PCRE2_ERROR_INTERNAL_OVERRAN_WORKSPACE: ::core::ffi::c_int = 152;
pub const PCRE2_ERROR_INTERNAL_MISSING_SUBPATTERN: ::core::ffi::c_int = 153;
pub const PCRE2_ERROR_DEFINE_TOO_MANY_BRANCHES: ::core::ffi::c_int = 154;
pub const PCRE2_ERROR_BACKSLASH_O_MISSING_BRACE: ::core::ffi::c_int = 155;
pub const PCRE2_ERROR_INTERNAL_UNKNOWN_NEWLINE: ::core::ffi::c_int = 156;
pub const PCRE2_ERROR_BACKSLASH_G_SYNTAX: ::core::ffi::c_int = 157;
pub const PCRE2_ERROR_PARENS_QUERY_R_MISSING_CLOSING: ::core::ffi::c_int = 158;
pub const PCRE2_ERROR_VERB_ARGUMENT_NOT_ALLOWED: ::core::ffi::c_int = 159;
pub const PCRE2_ERROR_VERB_UNKNOWN: ::core::ffi::c_int = 160;
pub const PCRE2_ERROR_SUBPATTERN_NUMBER_TOO_BIG: ::core::ffi::c_int = 161;
pub const PCRE2_ERROR_SUBPATTERN_NAME_EXPECTED: ::core::ffi::c_int = 162;
pub const PCRE2_ERROR_INTERNAL_PARSED_OVERFLOW: ::core::ffi::c_int = 163;
pub const PCRE2_ERROR_INVALID_OCTAL: ::core::ffi::c_int = 164;
pub const PCRE2_ERROR_SUBPATTERN_NAMES_MISMATCH: ::core::ffi::c_int = 165;
pub const PCRE2_ERROR_MARK_MISSING_ARGUMENT: ::core::ffi::c_int = 166;
pub const PCRE2_ERROR_INVALID_HEXADECIMAL: ::core::ffi::c_int = 167;
pub const PCRE2_ERROR_BACKSLASH_C_SYNTAX: ::core::ffi::c_int = 168;
pub const PCRE2_ERROR_BACKSLASH_K_SYNTAX: ::core::ffi::c_int = 169;
pub const PCRE2_ERROR_INTERNAL_BAD_CODE_LOOKBEHINDS: ::core::ffi::c_int = 170;
pub const PCRE2_ERROR_BACKSLASH_N_IN_CLASS: ::core::ffi::c_int = 171;
pub const PCRE2_ERROR_CALLOUT_STRING_TOO_LONG: ::core::ffi::c_int = 172;
pub const PCRE2_ERROR_UNICODE_DISALLOWED_CODE_POINT: ::core::ffi::c_int = 173;
pub const PCRE2_ERROR_UTF_IS_DISABLED: ::core::ffi::c_int = 174;
pub const PCRE2_ERROR_UCP_IS_DISABLED: ::core::ffi::c_int = 175;
pub const PCRE2_ERROR_VERB_NAME_TOO_LONG: ::core::ffi::c_int = 176;
pub const PCRE2_ERROR_BACKSLASH_U_CODE_POINT_TOO_BIG: ::core::ffi::c_int = 177;
pub const PCRE2_ERROR_MISSING_OCTAL_OR_HEX_DIGITS: ::core::ffi::c_int = 178;
pub const PCRE2_ERROR_VERSION_CONDITION_SYNTAX: ::core::ffi::c_int = 179;
pub const PCRE2_ERROR_INTERNAL_BAD_CODE_AUTO_POSSESS: ::core::ffi::c_int = 180;
pub const PCRE2_ERROR_CALLOUT_NO_STRING_DELIMITER: ::core::ffi::c_int = 181;
pub const PCRE2_ERROR_CALLOUT_BAD_STRING_DELIMITER: ::core::ffi::c_int = 182;
pub const PCRE2_ERROR_BACKSLASH_C_CALLER_DISABLED: ::core::ffi::c_int = 183;
pub const PCRE2_ERROR_QUERY_BARJX_NEST_TOO_DEEP: ::core::ffi::c_int = 184;
pub const PCRE2_ERROR_BACKSLASH_C_LIBRARY_DISABLED: ::core::ffi::c_int = 185;
pub const PCRE2_ERROR_PATTERN_TOO_COMPLICATED: ::core::ffi::c_int = 186;
pub const PCRE2_ERROR_LOOKBEHIND_TOO_LONG: ::core::ffi::c_int = 187;
pub const PCRE2_ERROR_PATTERN_STRING_TOO_LONG: ::core::ffi::c_int = 188;
pub const PCRE2_ERROR_INTERNAL_BAD_CODE: ::core::ffi::c_int = 189;
pub const PCRE2_ERROR_INTERNAL_BAD_CODE_IN_SKIP: ::core::ffi::c_int = 190;
pub const PCRE2_ERROR_NO_SURROGATES_IN_UTF16: ::core::ffi::c_int = 191;
pub const PCRE2_ERROR_BAD_LITERAL_OPTIONS: ::core::ffi::c_int = 192;
pub const PCRE2_ERROR_NOMATCH: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PCRE2_ERROR_PARTIAL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const PCRE2_ERROR_BADMAGIC: ::core::ffi::c_int = -31;
pub const PCRE2_ERROR_BADOFFSET: ::core::ffi::c_int = -33;
pub const PCRE2_ERROR_BADOPTION: ::core::ffi::c_int = -34;
pub const PCRE2_ERROR_BADUTFOFFSET: ::core::ffi::c_int = -36;
pub const PCRE2_ERROR_CALLOUT: ::core::ffi::c_int = -37;
pub const PCRE2_ERROR_DFA_RECURSE: ::core::ffi::c_int = -39;
pub const PCRE2_ERROR_DFA_UCOND: ::core::ffi::c_int = -40;
pub const PCRE2_ERROR_DFA_UITEM: ::core::ffi::c_int = -42;
pub const PCRE2_ERROR_DFA_WSSIZE: ::core::ffi::c_int = -43;
pub const PCRE2_ERROR_INTERNAL: ::core::ffi::c_int = -44;
pub const PCRE2_ERROR_JIT_BADOPTION: ::core::ffi::c_int = -45;
pub const PCRE2_ERROR_JIT_STACKLIMIT: ::core::ffi::c_int = -(46 as ::core::ffi::c_int);
pub const PCRE2_ERROR_MATCHLIMIT: ::core::ffi::c_int = -47;
pub const PCRE2_ERROR_NOMEMORY: ::core::ffi::c_int = -48;
pub const PCRE2_ERROR_NOSUBSTRING: ::core::ffi::c_int = -(49 as ::core::ffi::c_int);
pub const PCRE2_ERROR_NULL: ::core::ffi::c_int = -51;
pub const PCRE2_ERROR_RECURSELOOP: ::core::ffi::c_int = -52;
pub const PCRE2_ERROR_RECURSIONLIMIT: ::core::ffi::c_int = -53;
pub const PCRE2_INFO_ALLOPTIONS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PCRE2_INFO_BACKREFMAX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PCRE2_INFO_BSR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PCRE2_INFO_CAPTURECOUNT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PCRE2_INFO_HASCRORLF: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PCRE2_INFO_JCHANGED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PCRE2_INFO_MAXLOOKBEHIND: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const PCRE2_INFO_NEWLINE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PCRE2_CONFIG_UNICODE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PCRE2_ZERO_TERMINATED: size_t = !(0 as ::core::ffi::c_int as size_t);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
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
pub const G_REGEX_PCRE_GENERIC_MASK: ::core::ffi::c_uint =
    PCRE2_ANCHORED | PCRE2_NO_UTF_CHECK | PCRE2_ENDANCHORED;
pub const G_REGEX_PCRE2_COMPILE_MASK: ::core::ffi::c_uint = PCRE2_ALLOW_EMPTY_CLASS
    | PCRE2_ALT_BSUX
    | PCRE2_AUTO_CALLOUT
    | PCRE2_CASELESS
    | PCRE2_DOLLAR_ENDONLY
    | PCRE2_DOTALL
    | PCRE2_DUPNAMES
    | PCRE2_EXTENDED
    | PCRE2_FIRSTLINE
    | PCRE2_MATCH_UNSET_BACKREF
    | PCRE2_MULTILINE
    | PCRE2_NEVER_UCP
    | PCRE2_NEVER_UTF
    | PCRE2_NO_AUTO_CAPTURE
    | PCRE2_NO_AUTO_POSSESS
    | PCRE2_NO_DOTSTAR_ANCHOR
    | PCRE2_NO_START_OPTIMIZE
    | PCRE2_UCP
    | PCRE2_UNGREEDY
    | PCRE2_UTF
    | PCRE2_NEVER_BACKSLASH_C
    | PCRE2_ALT_CIRCUMFLEX
    | PCRE2_ALT_VERBNAMES
    | PCRE2_USE_OFFSET_LIMIT
    | PCRE2_EXTENDED_MORE
    | PCRE2_LITERAL
    | PCRE2_MATCH_INVALID_UTF
    | G_REGEX_PCRE_GENERIC_MASK;
pub const G_REGEX_COMPILE_NONPCRE_MASK: ::core::ffi::c_uint = 0x80000 as ::core::ffi::c_uint;
pub const G_REGEX_PCRE2_MATCH_MASK: ::core::ffi::c_uint = PCRE2_NOTBOL
    | PCRE2_NOTEOL
    | PCRE2_NOTEMPTY
    | PCRE2_NOTEMPTY_ATSTART
    | PCRE2_PARTIAL_SOFT
    | PCRE2_PARTIAL_HARD
    | PCRE2_NO_JIT
    | PCRE2_COPY_MATCHED_SUBJECT
    | G_REGEX_PCRE_GENERIC_MASK;
pub const G_REGEX_PCRE2_JIT_UNSUPPORTED_OPTIONS: ::core::ffi::c_uint =
    PCRE2_ANCHORED | PCRE2_ENDANCHORED;
unsafe extern "C" fn safe_c2rust_get_pcre2_compile_options(
    mut compile_flags: GRegexCompileFlags,
) -> uint32_t {
    let mut pcre2_flags: uint32_t = 0 as uint32_t;
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_CASELESS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_CASELESS) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_MULTILINE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_MULTILINE) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_DOTALL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_DOTALL) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_EXTENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_EXTENDED) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_ANCHORED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_ANCHORED) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_DOLLAR_ENDONLY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_DOLLAR_ENDONLY) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_UNGREEDY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_UNGREEDY) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_RAW as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_UTF) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_NO_AUTO_CAPTURE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NO_AUTO_CAPTURE) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_FIRSTLINE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_FIRSTLINE) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_DUPNAMES as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_DUPNAMES) as uint32_t;
    }
    return pcre2_flags & G_REGEX_PCRE2_COMPILE_MASK as uint32_t;
}
unsafe extern "C" fn safe_c2rust_get_pcre2_match_options(
    mut match_flags: GRegexMatchFlags,
    mut compile_flags: GRegexCompileFlags,
) -> uint32_t {
    let mut pcre2_flags: uint32_t = 0 as uint32_t;
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_ANCHORED) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NOTBOL) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NOTEOL) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NOTEMPTY) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_PARTIAL_SOFT) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_PARTIAL_HARD) as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NOTEMPTY_ATSTART) as uint32_t;
    }
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_RAW as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pcre2_flags = (pcre2_flags as ::core::ffi::c_uint | PCRE2_NO_UTF_CHECK) as uint32_t;
    }
    return pcre2_flags & G_REGEX_PCRE2_MATCH_MASK as uint32_t;
}
unsafe extern "C" fn safe_c2rust_g_regex_compile_flags_from_pcre2(
    mut pcre2_flags: uint32_t,
) -> GRegexCompileFlags {
    let mut compile_flags: GRegexCompileFlags = G_REGEX_DEFAULT;
    if pcre2_flags & PCRE2_CASELESS as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_CASELESS as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_MULTILINE as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_MULTILINE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_DOTALL as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_DOTALL as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_EXTENDED as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_EXTENDED as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_ANCHORED as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_ANCHORED as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_DOLLAR_ENDONLY as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_DOLLAR_ENDONLY as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_UNGREEDY as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_UNGREEDY as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_UTF as uint32_t == 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_RAW as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_NO_AUTO_CAPTURE as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_NO_AUTO_CAPTURE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_FIRSTLINE as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_FIRSTLINE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_DUPNAMES as uint32_t != 0 {
        compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
            compile_flags as ::core::ffi::c_uint
                | G_REGEX_DUPNAMES as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return (compile_flags as ::core::ffi::c_uint
        & (G_REGEX_DEFAULT as ::core::ffi::c_int
            | G_REGEX_CASELESS as ::core::ffi::c_int
            | G_REGEX_MULTILINE as ::core::ffi::c_int
            | G_REGEX_DOTALL as ::core::ffi::c_int
            | G_REGEX_EXTENDED as ::core::ffi::c_int
            | G_REGEX_ANCHORED as ::core::ffi::c_int
            | G_REGEX_DOLLAR_ENDONLY as ::core::ffi::c_int
            | G_REGEX_UNGREEDY as ::core::ffi::c_int
            | G_REGEX_RAW as ::core::ffi::c_int
            | G_REGEX_NO_AUTO_CAPTURE as ::core::ffi::c_int
            | G_REGEX_OPTIMIZE as ::core::ffi::c_int
            | G_REGEX_FIRSTLINE as ::core::ffi::c_int
            | G_REGEX_DUPNAMES as ::core::ffi::c_int
            | G_REGEX_NEWLINE_CR as ::core::ffi::c_int
            | G_REGEX_NEWLINE_LF as ::core::ffi::c_int
            | G_REGEX_NEWLINE_CRLF as ::core::ffi::c_int
            | G_REGEX_NEWLINE_ANYCRLF as ::core::ffi::c_int
            | G_REGEX_BSR_ANYCRLF as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as GRegexCompileFlags;
}
unsafe extern "C" fn safe_c2rust_g_regex_match_flags_from_pcre2(
    mut pcre2_flags: uint32_t,
) -> GRegexMatchFlags {
    let mut match_flags: GRegexMatchFlags = G_REGEX_MATCH_DEFAULT;
    if pcre2_flags & PCRE2_ANCHORED as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_NOTBOL as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_NOTEOL as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_NOTEMPTY as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_PARTIAL_SOFT as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_PARTIAL_HARD as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if pcre2_flags & PCRE2_NOTEMPTY_ATSTART as uint32_t != 0 {
        match_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexMatchFlags>(
            match_flags as ::core::ffi::c_uint
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return (match_flags as ::core::ffi::c_uint
        & (G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
            | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
            | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
            | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
            | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
            | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
            | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
            | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
            | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
            | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
            | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
            as ::core::ffi::c_uint) as GRegexMatchFlags;
}
unsafe extern "C" fn safe_c2rust_get_pcre2_newline_compile_options(
    mut compile_flags: GRegexCompileFlags,
) -> uint32_t {
    compile_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
        compile_flags as ::core::ffi::c_uint
            & (G_REGEX_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_NEWLINE_ANYCRLF as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
    );
    match compile_flags as ::core::ffi::c_uint {
        1048576 => return PCRE2_NEWLINE_CR as uint32_t,
        2097152 => return PCRE2_NEWLINE_LF as uint32_t,
        3145728 => return PCRE2_NEWLINE_CRLF as uint32_t,
        5242880 => return PCRE2_NEWLINE_ANYCRLF as uint32_t,
        _ => {
            if compile_flags as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
                return 0 as uint32_t;
            }
            return PCRE2_NEWLINE_ANY as uint32_t;
        }
    };
}
unsafe extern "C" fn safe_c2rust_get_pcre2_newline_match_options(
    mut match_flags: GRegexMatchFlags,
) -> uint32_t {
    match match_flags as ::core::ffi::c_uint
        & (G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
            | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        1048576 => return PCRE2_NEWLINE_CR as uint32_t,
        2097152 => return PCRE2_NEWLINE_LF as uint32_t,
        3145728 => return PCRE2_NEWLINE_CRLF as uint32_t,
        4194304 => return PCRE2_NEWLINE_ANY as uint32_t,
        5242880 => return PCRE2_NEWLINE_ANYCRLF as uint32_t,
        _ => return 0 as uint32_t,
    };
}
unsafe extern "C" fn safe_c2rust_get_pcre2_bsr_compile_options(
    mut compile_flags: GRegexCompileFlags,
) -> uint32_t {
    if compile_flags as ::core::ffi::c_uint
        & G_REGEX_BSR_ANYCRLF as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return PCRE2_BSR_ANYCRLF as uint32_t;
    }
    return PCRE2_BSR_UNICODE as uint32_t;
}
unsafe extern "C" fn safe_c2rust_get_pcre2_bsr_match_options(
    mut match_flags: GRegexMatchFlags,
) -> uint32_t {
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return PCRE2_BSR_ANYCRLF as uint32_t;
    }
    if match_flags as ::core::ffi::c_uint
        & G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return PCRE2_BSR_UNICODE as uint32_t;
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn safe_c2rust_get_pcre2_error_string(
    mut errcode: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut error_msg: [PCRE2_UCHAR8; 2048] = [0; 2048];
    let mut err_length: ::core::ffi::c_int = 0;
    err_length = pcre2_get_error_message_8(
        errcode,
        &raw mut error_msg as *mut PCRE2_UCHAR8,
        (::core::mem::size_of::<[PCRE2_UCHAR8; 2048]>() as size_t)
            .wrapping_div(::core::mem::size_of::<PCRE2_UCHAR8>() as size_t),
    );
    if err_length <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if (err_length as size_t)
            < (::core::mem::size_of::<[PCRE2_UCHAR8; 2048]>() as usize)
                .wrapping_div(::core::mem::size_of::<PCRE2_UCHAR8>() as usize)
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
            b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
            467 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(size_t) err_length < G_N_ELEMENTS (error_msg)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return g_memdup2(
        &raw mut error_msg as *mut PCRE2_UCHAR8 as gconstpointer,
        (err_length + 1 as ::core::ffi::c_int) as gsize,
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_translate_match_error(mut errcode: gint) -> *const gchar {
    match errcode {
        PCRE2_ERROR_NULL => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"A NULL argument was passed to PCRE\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_BADOPTION => return b"bad options\0" as *const u8 as *const gchar,
        PCRE2_ERROR_BADMAGIC => {
            return glib_gettext(b"corrupted object\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_NOMEMORY => {
            return glib_gettext(b"out of memory\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_INTERNAL => {
            return glib_gettext(b"internal error\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_DFA_UITEM => {
            return glib_gettext(
                b"the pattern contains items not supported for partial matching\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_DFA_UCOND => {
            return glib_gettext(
                b"back references as conditions are not supported for partial matching\0"
                    as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_DFA_RECURSE | PCRE2_ERROR_RECURSIONLIMIT => {
            return glib_gettext(b"recursion limit reached\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_BADOFFSET => {
            return glib_gettext(b"bad offset\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_RECURSELOOP => {
            return glib_gettext(b"recursion loop\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_JIT_BADOPTION => {
            return glib_gettext(
                b"matching mode is requested that was not compiled for JIT\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_NOMATCH
        | PCRE2_ERROR_NOSUBSTRING
        | PCRE2_ERROR_MATCHLIMIT
        | PCRE2_ERROR_CALLOUT
        | PCRE2_ERROR_BADUTFOFFSET
        | PCRE2_ERROR_PARTIAL
        | PCRE2_ERROR_DFA_WSSIZE
        | _ => {}
    }
    return ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn safe_c2rust_get_match_error_message(
    mut errcode: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut msg: *const ::core::ffi::c_char =
        safe_c2rust_translate_match_error(errcode as gint) as *const ::core::ffi::c_char;
    let mut error_string: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !msg.is_null() {
        return safe_c2rust_g_strdup_inline(msg);
    }
    error_string = safe_c2rust_get_pcre2_error_string(errcode);
    if !error_string.is_null() {
        return error_string;
    }
    return safe_c2rust_g_strdup_inline(
        glib_gettext(b"unknown error\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_translate_compile_error(
    mut errcode: *mut gint,
    mut errmsg: *mut *const gchar,
) {
    let mut original_errcode: gint = *errcode;
    *errcode = -(1 as ::core::ffi::c_int) as gint;
    *errmsg = ::core::ptr::null::<gchar>();
    match original_errcode {
        PCRE2_ERROR_END_BACKSLASH => {
            *errcode = G_REGEX_ERROR_STRAY_BACKSLASH as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"\\ at end of pattern\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_END_BACKSLASH_C => {
            *errcode = G_REGEX_ERROR_MISSING_CONTROL_CHAR as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"\\c at end of pattern\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_UNKNOWN_ESCAPE | PCRE2_ERROR_UNSUPPORTED_ESCAPE_SEQUENCE => {
            *errcode = G_REGEX_ERROR_UNRECOGNIZED_ESCAPE as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"unrecognized character following \\\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_QUANTIFIER_OUT_OF_ORDER => {
            *errcode = G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"numbers out of order in {} quantifier\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_QUANTIFIER_TOO_BIG => {
            *errcode = G_REGEX_ERROR_QUANTIFIER_TOO_BIG as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"number too big in {} quantifier\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_MISSING_SQUARE_BRACKET => {
            *errcode = G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"missing terminating ] for character class\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_ESCAPE_INVALID_IN_CLASS => {
            *errcode =
                G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"invalid escape sequence in character class\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_CLASS_RANGE_ORDER => {
            *errcode = G_REGEX_ERROR_RANGE_OUT_OF_ORDER as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"range out of order in character class\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_QUANTIFIER_INVALID | PCRE2_ERROR_INTERNAL_UNEXPECTED_REPEAT => {
            *errcode = G_REGEX_ERROR_NOTHING_TO_REPEAT as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"nothing to repeat\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_INVALID_AFTER_PARENS_QUERY => {
            *errcode = G_REGEX_ERROR_UNRECOGNIZED_CHARACTER as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"unrecognized character after (? or (?-\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_POSIX_CLASS_NOT_IN_CLASS => {
            *errcode = G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"POSIX named classes are supported only within a class\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_POSIX_NO_SUPPORT_COLLATING => {
            *errcode =
                G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"POSIX collating elements are not supported\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_MISSING_CLOSING_PARENTHESIS
        | PCRE2_ERROR_UNMATCHED_CLOSING_PARENTHESIS
        | PCRE2_ERROR_PARENS_QUERY_R_MISSING_CLOSING => {
            *errcode = G_REGEX_ERROR_UNMATCHED_PARENTHESIS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"missing terminating )\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_BAD_SUBPATTERN_REFERENCE => {
            *errcode = G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"reference to non-existent subpattern\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_MISSING_COMMENT_CLOSING => {
            *errcode = G_REGEX_ERROR_UNTERMINATED_COMMENT as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"missing ) after comment\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_PATTERN_TOO_LARGE => {
            *errcode = G_REGEX_ERROR_EXPRESSION_TOO_LARGE as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"regular expression is too large\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_MISSING_CONDITION_CLOSING => {
            *errcode = G_REGEX_ERROR_MALFORMED_CONDITION as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"malformed number or name after (?(\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_LOOKBEHIND_NOT_FIXED_LENGTH => {
            *errcode = G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"lookbehind assertion is not fixed length\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_TOO_MANY_CONDITION_BRANCHES => {
            *errcode = G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"conditional group contains more than two branches\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_CONDITION_ASSERTION_EXPECTED => {
            *errcode = G_REGEX_ERROR_ASSERTION_EXPECTED as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"assertion expected after (?(\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_BAD_RELATIVE_REFERENCE => {
            *errcode = G_REGEX_ERROR_INVALID_RELATIVE_REFERENCE as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"a numbered reference must not be zero\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_UNKNOWN_POSIX_CLASS => {
            *errcode = G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"unknown POSIX class name\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_CODE_POINT_TOO_BIG | PCRE2_ERROR_INVALID_HEXADECIMAL => {
            *errcode = G_REGEX_ERROR_HEX_CODE_TOO_LARGE as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"character value in \\x{...} sequence is too large\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_LOOKBEHIND_INVALID_BACKSLASH_C => {
            *errcode = G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"\\C not allowed in lookbehind assertion\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_MISSING_NAME_TERMINATOR => {
            *errcode =
                G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"missing terminator in subpattern name\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_DUPLICATE_SUBPATTERN_NAME => {
            *errcode = G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"two named subpatterns have the same name\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_MALFORMED_UNICODE_PROPERTY => {
            *errcode = G_REGEX_ERROR_MALFORMED_PROPERTY as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"malformed \\P or \\p sequence\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_UNKNOWN_UNICODE_PROPERTY => {
            *errcode = G_REGEX_ERROR_UNKNOWN_PROPERTY as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"unknown property name after \\P or \\p\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_SUBPATTERN_NAME_TOO_LONG => {
            *errcode = G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"subpattern name is too long (maximum 32 characters)\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_TOO_MANY_NAMED_SUBPATTERNS => {
            *errcode = G_REGEX_ERROR_TOO_MANY_SUBPATTERNS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"too many named subpatterns (maximum 10,000)\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_OCTAL_BYTE_TOO_BIG => {
            *errcode = G_REGEX_ERROR_INVALID_OCTAL_VALUE as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"octal value is greater than \\377\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_DEFINE_TOO_MANY_BRANCHES => {
            *errcode = G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"DEFINE group contains more than one branch\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_INTERNAL_UNKNOWN_NEWLINE => {
            *errcode = G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"inconsistent NEWLINE options\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_BACKSLASH_G_SYNTAX => {
            *errcode = G_REGEX_ERROR_MISSING_BACK_REFERENCE as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"\\g is not followed by a braced, angle-bracketed, or quoted name or number, or by a plain number\0"
                    as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_VERB_ARGUMENT_NOT_ALLOWED => {
            *errcode = G_REGEX_ERROR_BACKTRACKING_CONTROL_VERB_ARGUMENT_FORBIDDEN
                as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"an argument is not allowed for (*ACCEPT), (*FAIL), or (*COMMIT)\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_VERB_UNKNOWN => {
            *errcode =
                G_REGEX_ERROR_UNKNOWN_BACKTRACKING_CONTROL_VERB as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"(*VERB) not recognized\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_SUBPATTERN_NUMBER_TOO_BIG => {
            *errcode = G_REGEX_ERROR_NUMBER_TOO_BIG as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"number is too big\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_SUBPATTERN_NAME_EXPECTED => {
            *errcode = G_REGEX_ERROR_MISSING_SUBPATTERN_NAME as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"missing subpattern name after (?&\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_SUBPATTERN_NAMES_MISMATCH => {
            *errcode = G_REGEX_ERROR_EXTRA_SUBPATTERN_NAME as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"different names for subpatterns of the same number are not allowed\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_MARK_MISSING_ARGUMENT => {
            *errcode = G_REGEX_ERROR_BACKTRACKING_CONTROL_VERB_ARGUMENT_REQUIRED
                as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"(*MARK) must have an argument\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_BACKSLASH_C_SYNTAX => {
            *errcode = G_REGEX_ERROR_INVALID_CONTROL_CHAR as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"\\c must be followed by an ASCII character\0" as *const u8 as *const gchar,
            );
        }
        PCRE2_ERROR_BACKSLASH_K_SYNTAX => {
            *errcode = G_REGEX_ERROR_MISSING_NAME as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"\\k is not followed by a braced, angle-bracketed, or quoted name\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_BACKSLASH_N_IN_CLASS => {
            *errcode = G_REGEX_ERROR_NOT_SUPPORTED_IN_CLASS as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"\\N is not supported in a class\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_VERB_NAME_TOO_LONG => {
            *errcode = G_REGEX_ERROR_NAME_TOO_LONG as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"name is too long in (*MARK), (*PRUNE), (*SKIP), or (*THEN)\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_INTERNAL_CODE_OVERFLOW => {
            *errcode = G_REGEX_ERROR_INTERNAL as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"code overflow\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_UNRECOGNIZED_AFTER_QUERY_P => {
            *errcode = G_REGEX_ERROR_UNRECOGNIZED_CHARACTER as ::core::ffi::c_int as gint;
            *errmsg =
                glib_gettext(b"unrecognized character after (?P\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_INTERNAL_OVERRAN_WORKSPACE => {
            *errcode = G_REGEX_ERROR_INTERNAL as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(b"overran compiling workspace\0" as *const u8 as *const gchar);
        }
        PCRE2_ERROR_INTERNAL_MISSING_SUBPATTERN => {
            *errcode = G_REGEX_ERROR_INTERNAL as ::core::ffi::c_int as gint;
            *errmsg = glib_gettext(
                b"previously-checked referenced subpattern not found\0" as *const u8
                    as *const gchar,
            );
        }
        PCRE2_ERROR_HEAP_FAILED
        | PCRE2_ERROR_INTERNAL_PARSED_OVERFLOW
        | PCRE2_ERROR_UNICODE_NOT_SUPPORTED
        | PCRE2_ERROR_UNICODE_DISALLOWED_CODE_POINT
        | PCRE2_ERROR_NO_SURROGATES_IN_UTF16
        | PCRE2_ERROR_INTERNAL_BAD_CODE_LOOKBEHINDS
        | PCRE2_ERROR_UNICODE_PROPERTIES_UNAVAILABLE
        | PCRE2_ERROR_INTERNAL_STUDY_ERROR
        | PCRE2_ERROR_UTF_IS_DISABLED
        | PCRE2_ERROR_UCP_IS_DISABLED
        | PCRE2_ERROR_INTERNAL_BAD_CODE_AUTO_POSSESS
        | PCRE2_ERROR_BACKSLASH_C_LIBRARY_DISABLED
        | PCRE2_ERROR_INTERNAL_BAD_CODE
        | PCRE2_ERROR_INTERNAL_BAD_CODE_IN_SKIP => {
            *errcode = G_REGEX_ERROR_INTERNAL as ::core::ffi::c_int as gint;
        }
        PCRE2_ERROR_INVALID_SUBPATTERN_NAME
        | PCRE2_ERROR_CLASS_INVALID_RANGE
        | PCRE2_ERROR_ZERO_RELATIVE_REFERENCE
        | PCRE2_ERROR_PARENTHESES_STACK_CHECK
        | PCRE2_ERROR_LOOKBEHIND_TOO_COMPLICATED
        | PCRE2_ERROR_CALLOUT_NUMBER_TOO_BIG
        | PCRE2_ERROR_MISSING_CALLOUT_CLOSING
        | PCRE2_ERROR_ESCAPE_INVALID_IN_VERB
        | PCRE2_ERROR_NULL_PATTERN
        | PCRE2_ERROR_BAD_OPTIONS
        | PCRE2_ERROR_PARENTHESES_NEST_TOO_DEEP
        | PCRE2_ERROR_BACKSLASH_O_MISSING_BRACE
        | PCRE2_ERROR_INVALID_OCTAL
        | PCRE2_ERROR_CALLOUT_STRING_TOO_LONG
        | PCRE2_ERROR_BACKSLASH_U_CODE_POINT_TOO_BIG
        | PCRE2_ERROR_MISSING_OCTAL_OR_HEX_DIGITS
        | PCRE2_ERROR_VERSION_CONDITION_SYNTAX
        | PCRE2_ERROR_CALLOUT_NO_STRING_DELIMITER
        | PCRE2_ERROR_CALLOUT_BAD_STRING_DELIMITER
        | PCRE2_ERROR_BACKSLASH_C_CALLER_DISABLED
        | PCRE2_ERROR_QUERY_BARJX_NEST_TOO_DEEP
        | PCRE2_ERROR_PATTERN_TOO_COMPLICATED
        | PCRE2_ERROR_LOOKBEHIND_TOO_LONG
        | PCRE2_ERROR_PATTERN_STRING_TOO_LONG
        | PCRE2_ERROR_BAD_LITERAL_OPTIONS
        | _ => {
            *errcode = G_REGEX_ERROR_COMPILE as ::core::ffi::c_int as gint;
        }
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if *errcode != -(1 as ::core::ffi::c_int) {
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
            b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
            804 as ::core::ffi::c_int,
            G_STRFUNC,
            b"*errcode != -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_match_info_new(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gint,
    mut start_position: gint,
    mut match_options: GRegexMatchFlags,
    mut is_dfa: gboolean,
) -> *mut GMatchInfo {
    let mut match_info: *mut GMatchInfo = ::core::ptr::null_mut::<GMatchInfo>();
    if string_len < 0 as ::core::ffi::c_int {
        string_len = strlen(string as *const ::core::ffi::c_char) as gint;
    }
    match_info = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GMatchInfo>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GMatchInfo;
    (*match_info).ref_count = 1 as ::core::ffi::c_int as gint;
    (*match_info).regex = safe_c2rust_g_regex_ref(regex as *mut GRegex);
    (*match_info).string = string;
    (*match_info).string_len = string_len as gssize;
    (*match_info).matches = PCRE2_ERROR_NOMATCH as gint;
    (*match_info).pos = start_position;
    (*match_info).match_opts =
        safe_c2rust_get_pcre2_match_options(match_options, (*regex).orig_compile_opts);
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_CAPTURECOUNT as uint32_t,
        &raw mut (*match_info).n_subpatterns as *mut ::core::ffi::c_void,
    );
    (*match_info).match_context =
        pcre2_match_context_create_8(::core::ptr::null_mut::<pcre2_general_context_8>());
    if is_dfa != 0 {
        (*match_info).n_workspace = 100 as size_t;
        (*match_info).workspace = ({
            let mut __n: gsize = (*match_info).n_workspace as gsize;
            let mut __s: gsize = ::core::mem::size_of::<gint>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut gint;
    }
    (*match_info).n_offsets = 2 as uint32_t;
    (*match_info).offsets = ({
        let mut __n: gsize = (*match_info).n_offsets as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut gint;
    *(*match_info)
        .offsets
        .offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int) as gint;
    *(*match_info)
        .offsets
        .offset(1 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int) as gint;
    (*match_info).match_data = pcre2_match_data_create_from_pattern_8(
        (*(*match_info).regex).pcre_re,
        ::core::ptr::null_mut::<pcre2_general_context_8>(),
    );
    return match_info;
}
unsafe extern "C" fn safe_c2rust_recalc_match_offsets(
    mut match_info: *mut GMatchInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ovector: *mut size_t = ::core::ptr::null_mut::<size_t>();
    let mut ovector_size: uint32_t = 0 as uint32_t;
    let mut pre_n_offset: uint32_t = 0;
    let mut i: uint32_t = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !((*match_info).matches < -(1 as ::core::ffi::c_int)
            && (*match_info).matches != -(2 as ::core::ffi::c_int))
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
            b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
            867 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!IS_PCRE2_ERROR (match_info->matches)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*match_info).matches == PCRE2_ERROR_PARTIAL {
        ovector_size = 1 as uint32_t;
    } else if (*match_info).matches > 0 as ::core::ffi::c_int {
        ovector_size = (*match_info).matches as uint32_t;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ovector_size != 0 as uint32_t {
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
            b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
            874 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ovector_size != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if pcre2_get_ovector_count_8((*match_info).match_data) < ovector_size {
        g_set_error(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_MATCH as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error while matching regular expression %s: %s\0" as *const u8 as *const gchar,
            ),
            (*(*match_info).regex).pattern,
            glib_gettext(b"code overflow\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    pre_n_offset = (*match_info).n_offsets;
    (*match_info).n_offsets = ovector_size.wrapping_mul(2 as uint32_t);
    ovector = pcre2_get_ovector_pointer_8((*match_info).match_data);
    if (*match_info).n_offsets != pre_n_offset {
        (*match_info).offsets = g_realloc_n(
            (*match_info).offsets as gpointer,
            (*match_info).n_offsets as gsize,
            ::core::mem::size_of::<gint>() as gsize,
        ) as *mut gint;
    }
    i = 0 as uint32_t;
    while i < (*match_info).n_offsets {
        *(*match_info).offsets.offset(i as isize) =
            *ovector.offset(i as isize) as ::core::ffi::c_int as gint;
        i = i.wrapping_add(1);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_enable_jit_with_match_options(
    mut match_info: *mut GMatchInfo,
    mut match_options: uint32_t,
) -> JITStatus {
    let mut retval: gint = 0;
    let mut old_jit_options: uint32_t = 0;
    let mut new_jit_options: uint32_t = 0;
    if (*(*match_info).regex).orig_compile_opts as ::core::ffi::c_uint
        & G_REGEX_OPTIMIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        return JIT_STATUS_DISABLED;
    }
    if (*(*match_info).regex).jit_status as ::core::ffi::c_uint
        == JIT_STATUS_DISABLED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return JIT_STATUS_DISABLED;
    }
    if match_options & G_REGEX_PCRE2_JIT_UNSUPPORTED_OPTIONS as uint32_t != 0 {
        return JIT_STATUS_DISABLED;
    }
    old_jit_options = (*(*match_info).regex).jit_options;
    new_jit_options = old_jit_options | PCRE2_JIT_COMPLETE as uint32_t;
    if match_options & PCRE2_PARTIAL_HARD as uint32_t != 0 {
        new_jit_options =
            (new_jit_options as ::core::ffi::c_uint | PCRE2_JIT_PARTIAL_HARD) as uint32_t;
    }
    if match_options & PCRE2_PARTIAL_SOFT as uint32_t != 0 {
        new_jit_options =
            (new_jit_options as ::core::ffi::c_uint | PCRE2_JIT_PARTIAL_SOFT) as uint32_t;
    }
    if new_jit_options == old_jit_options {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if (*(*match_info).regex).jit_status as ::core::ffi::c_uint
                != JIT_STATUS_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
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
                b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
                929 as ::core::ffi::c_int,
                G_STRFUNC,
                b"match_info->regex->jit_status != JIT_STATUS_DEFAULT\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return (*(*match_info).regex).jit_status;
    }
    retval = pcre2_jit_compile_8((*(*match_info).regex).pcre_re, new_jit_options) as gint;
    if retval == 0 as ::core::ffi::c_int {
        (*(*match_info).regex).jit_status = JIT_STATUS_ENABLED;
        (*(*match_info).regex).jit_options = new_jit_options;
        (*match_info).jit_stack = pcre2_jit_stack_create_8(
            ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as size_t,
            ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as size_t,
            ::core::ptr::null_mut::<pcre2_general_context_8>(),
        );
        pcre2_jit_stack_assign_8(
            (*match_info).match_context,
            None,
            (*match_info).jit_stack as *mut ::core::ffi::c_void,
        );
    } else {
        (*(*match_info).regex).jit_status = JIT_STATUS_DISABLED;
        match retval {
            PCRE2_ERROR_NOMEMORY => {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"JIT compilation was requested with G_REGEX_OPTIMIZE, but JIT was unable to allocate executable memory for the compiler. Falling back to interpretive code.\0"
                        as *const u8 as *const gchar,
                );
            }
            PCRE2_ERROR_JIT_BADOPTION => {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"JIT compilation was requested with G_REGEX_OPTIMIZE, but JIT support is not available. Falling back to interpretive code.\0"
                        as *const u8 as *const gchar,
                );
            }
            _ => {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"JIT compilation was requested with G_REGEX_OPTIMIZE, but request for JIT support had unexpectedly failed (error %d). Falling back to interpretive code.\0"
                        as *const u8 as *const gchar,
                    retval,
                );
            }
        }
    }
    return (*(*match_info).regex).jit_status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_get_regex(
    mut match_info: *const GMatchInfo,
) -> *mut GRegex {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    return (*match_info).regex;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_get_string(
    mut match_info: *const GMatchInfo,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*match_info).string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_ref(
    mut match_info: *mut GMatchInfo,
) -> *mut GMatchInfo {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMatchInfo>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*match_info).ref_count;
        (*match_info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut (*match_info).ref_count,
        1 as ::core::ffi::c_int,
    );
    return match_info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_unref(mut match_info: *mut GMatchInfo) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*match_info).ref_count;
            (*match_info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*match_info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_regex_unref((*match_info).regex);
        if !(*match_info).match_context.is_null() {
            pcre2_match_context_free_8((*match_info).match_context);
        }
        if !(*match_info).jit_stack.is_null() {
            pcre2_jit_stack_free_8((*match_info).jit_stack);
        }
        if !(*match_info).match_data.is_null() {
            pcre2_match_data_free_8((*match_info).match_data);
        }
        g_free((*match_info).offsets as gpointer);
        g_free((*match_info).workspace as gpointer);
        g_free(match_info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_free(mut match_info: *mut GMatchInfo) {
    if match_info.is_null() {
        return;
    }
    safe_c2rust_g_match_info_unref(match_info);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_next(
    mut match_info: *mut GMatchInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut jit_status: JITStatus = JIT_STATUS_DEFAULT;
    let mut prev_match_start: gint = 0;
    let mut prev_match_end: gint = 0;
    let mut opts: uint32_t = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*match_info).pos >= 0 as ::core::ffi::c_int {
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
            b"match_info->pos >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    prev_match_start = *(*match_info)
        .offsets
        .offset(0 as ::core::ffi::c_int as isize);
    prev_match_end = *(*match_info)
        .offsets
        .offset(1 as ::core::ffi::c_int as isize);
    if (*match_info).pos as gssize > (*match_info).string_len {
        (*match_info).pos = -(1 as ::core::ffi::c_int) as gint;
        (*match_info).matches = PCRE2_ERROR_NOMATCH as gint;
        return FALSE;
    }
    opts = (*(*match_info).regex).match_opts | (*match_info).match_opts;
    jit_status = safe_c2rust_enable_jit_with_match_options(match_info, opts);
    if jit_status as ::core::ffi::c_uint
        == JIT_STATUS_ENABLED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*match_info).matches = pcre2_jit_match_8(
            (*(*match_info).regex).pcre_re,
            (*match_info).string as PCRE2_SPTR8,
            (*match_info).string_len as size_t,
            (*match_info).pos as size_t,
            opts,
            (*match_info).match_data,
            (*match_info).match_context,
        ) as gint;
        if (*match_info).matches == PCRE2_ERROR_JIT_STACKLIMIT {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"PCRE2 JIT stack limit reached, falling back to non-optimized matching.\0"
                    as *const u8 as *const gchar,
            );
            opts = (opts as ::core::ffi::c_uint | PCRE2_NO_JIT) as uint32_t;
            jit_status = JIT_STATUS_DISABLED;
        }
    }
    if jit_status as ::core::ffi::c_uint
        != JIT_STATUS_ENABLED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*match_info).matches = pcre2_match_8(
            (*(*match_info).regex).pcre_re,
            (*match_info).string as PCRE2_SPTR8,
            (*match_info).string_len as size_t,
            (*match_info).pos as size_t,
            opts,
            (*match_info).match_data,
            (*match_info).match_context,
        ) as gint;
    }
    if (*match_info).matches < PCRE2_ERROR_NOMATCH && (*match_info).matches != PCRE2_ERROR_PARTIAL {
        let mut error_msg: *mut gchar =
            safe_c2rust_get_match_error_message((*match_info).matches as ::core::ffi::c_int)
                as *mut gchar;
        g_set_error(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_MATCH as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error while matching regular expression %s: %s\0" as *const u8 as *const gchar,
            ),
            (*(*match_info).regex).pattern,
            error_msg,
        );
        let mut _pp: *mut *mut gchar = &raw mut error_msg;
        let mut _ptr: *mut gchar = *_pp;
        *_pp = ::core::ptr::null_mut::<gchar>();
        if !_ptr.is_null() {
            g_free(_ptr as gpointer);
        }
        return FALSE;
    } else if (*match_info).matches == 0 as ::core::ffi::c_int {
        (*match_info).n_offsets = (*match_info).n_offsets.wrapping_mul(2 as uint32_t);
        (*match_info).offsets = g_realloc_n(
            (*match_info).offsets as gpointer,
            (*match_info).n_offsets as gsize,
            ::core::mem::size_of::<gint>() as gsize,
        ) as *mut gint;
        pcre2_match_data_free_8((*match_info).match_data);
        (*match_info).match_data = pcre2_match_data_create_8(
            (*match_info).n_offsets,
            ::core::ptr::null_mut::<pcre2_general_context_8>(),
        );
        return safe_c2rust_g_match_info_next(match_info, error);
    } else if (*match_info).matches == PCRE2_ERROR_NOMATCH {
        (*match_info).pos = -(1 as ::core::ffi::c_int) as gint;
        return FALSE;
    } else if safe_c2rust_recalc_match_offsets(match_info, error) == 0 {
        return FALSE;
    }
    if (*match_info).pos
        == *(*match_info)
            .offsets
            .offset(1 as ::core::ffi::c_int as isize)
    {
        if (*match_info).pos as gssize > (*match_info).string_len {
            (*match_info).pos = -(1 as ::core::ffi::c_int) as gint;
            (*match_info).matches = PCRE2_ERROR_NOMATCH as gint;
            return FALSE;
        }
        (*match_info).pos =
            (if (*(*match_info).regex).compile_opts & G_REGEX_RAW as ::core::ffi::c_int as uint32_t
                != 0
            {
                ((*match_info).string.offset((*match_info).pos as isize) as *const gchar)
                    .offset(1 as ::core::ffi::c_int as isize)
            } else {
                ((*match_info).string.offset((*match_info).pos as isize) as *const gchar).offset(
                    *safe_c2rust_g_utf8_skip.offset(
                        *((*match_info).string.offset((*match_info).pos as isize) as *const gchar
                            as *const guchar) as isize,
                    ) as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char as *const gchar
            })
            .offset_from((*match_info).string) as ::core::ffi::c_long as gint;
    } else {
        (*match_info).pos = *(*match_info)
            .offsets
            .offset(1 as ::core::ffi::c_int as isize);
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*match_info).matches < 0 as ::core::ffi::c_int
            || (*match_info).matches as uint32_t
                <= (*match_info).n_subpatterns.wrapping_add(1 as uint32_t)
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
            b"../original/glib/gregex.c\0" as *const u8 as *const ::core::ffi::c_char,
            1203 as ::core::ffi::c_int,
            G_STRFUNC,
            b"match_info->matches < 0 || (uint32_t) match_info->matches <= match_info->n_subpatterns + 1\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*match_info).matches >= 0 as ::core::ffi::c_int
        && prev_match_start
            == *(*match_info)
                .offsets
                .offset(0 as ::core::ffi::c_int as isize)
        && prev_match_end
            == *(*match_info)
                .offsets
                .offset(1 as ::core::ffi::c_int as isize)
    {
        return safe_c2rust_g_match_info_next(match_info, error);
    }
    return ((*match_info).matches >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_matches(
    mut match_info: *const GMatchInfo,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*match_info).matches >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_get_match_count(
    mut match_info: *const GMatchInfo,
) -> gint {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if (*match_info).matches == PCRE2_ERROR_NOMATCH {
        return 0 as gint;
    } else if (*match_info).matches < PCRE2_ERROR_NOMATCH {
        return -(1 as gint);
    } else {
        return (*match_info).matches;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_is_partial_match(
    mut match_info: *const GMatchInfo,
) -> gboolean {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*match_info).matches == PCRE2_ERROR_PARTIAL) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_expand_references(
    mut match_info: *const GMatchInfo,
    mut string_to_expand: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut result: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !string_to_expand.is_null() {
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
            b"string_to_expand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    list = safe_c2rust_split_replacement(string_to_expand, &raw mut tmp_error);
    if !tmp_error.is_null() {
        g_propagate_error(error, tmp_error);
        return ::core::ptr::null_mut::<gchar>();
    }
    if match_info.is_null() && safe_c2rust_interpolation_list_needs_match(list) != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"String '%s' contains references to the match, can't expand references without GMatchInfo object\0"
                as *const u8 as *const gchar,
            string_to_expand,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_string_sized_new(strlen(string_to_expand as *const ::core::ffi::c_char) as gsize);
    safe_c2rust_interpolate_replacement(match_info, result, list as gpointer);
    g_list_free_full(
        list,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut InterpolationData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_free_interpolation_data
                as unsafe extern "C" fn(*mut InterpolationData) -> (),
        )),
    );
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(result, 0 as gboolean)
        } else {
            g_string_free_and_steal(result)
        }
    } else {
        g_string_free(result, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_fetch(
    mut match_info: *const GMatchInfo,
    mut match_num: gint,
) -> *mut gchar {
    let mut match_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut start: gint = 0;
    let mut end: gint = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if match_num >= 0 as ::core::ffi::c_int {
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
            b"match_num >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_g_match_info_fetch_pos(match_info, match_num, &raw mut start, &raw mut end) == 0
    {
        match_0 = ::core::ptr::null_mut::<gchar>();
    } else if start == -(1 as ::core::ffi::c_int) {
        match_0 = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    } else {
        match_0 = g_strndup(
            (*match_info).string.offset(start as isize) as *const gchar,
            (end - start) as gsize,
        );
    }
    return match_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_fetch_pos(
    mut match_info: *const GMatchInfo,
    mut match_num: gint,
    mut start_pos: *mut gint,
    mut end_pos: *mut gint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if match_num >= 0 as ::core::ffi::c_int {
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
            b"match_num >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*match_info).matches < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if match_num as uint32_t
        >= (if (*match_info).n_subpatterns.wrapping_add(1 as uint32_t)
            > (*match_info).matches as uint32_t
        {
            (*match_info).n_subpatterns.wrapping_add(1 as uint32_t)
        } else {
            (*match_info).matches as uint32_t
        })
    {
        return FALSE;
    }
    if !start_pos.is_null() {
        *start_pos = (if match_num < (*match_info).matches {
            *(*match_info)
                .offsets
                .offset((2 as gint * match_num) as isize) as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        }) as gint;
    }
    if !end_pos.is_null() {
        *end_pos = (if match_num < (*match_info).matches {
            *(*match_info).offsets.offset(
                (2 as ::core::ffi::c_int * match_num as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        }) as gint;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_matched_substring_number(
    mut match_info: *const GMatchInfo,
    mut name: *const gchar,
) -> gint {
    let mut entrysize: gint = 0;
    let mut first: PCRE2_SPTR8 = ::core::ptr::null::<PCRE2_UCHAR8>();
    let mut last: PCRE2_SPTR8 = ::core::ptr::null::<PCRE2_UCHAR8>();
    let mut entry: *mut guchar = ::core::ptr::null_mut::<guchar>();
    if (*(*match_info).regex).compile_opts & PCRE2_DUPNAMES as uint32_t == 0 {
        return pcre2_substring_number_from_name_8(
            (*(*match_info).regex).pcre_re,
            name as PCRE2_SPTR8,
        ) as gint;
    }
    entrysize = pcre2_substring_nametable_scan_8(
        (*(*match_info).regex).pcre_re,
        name as PCRE2_SPTR8,
        &raw mut first,
        &raw mut last,
    ) as gint;
    if entrysize <= 0 as ::core::ffi::c_int {
        return entrysize;
    }
    entry = first as *mut guchar;
    while entry <= last as *mut guchar {
        let mut n: guint = (((*entry.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *entry.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as guint;
        if (n as uint32_t).wrapping_mul(2 as uint32_t) < (*match_info).n_offsets
            && *(*match_info)
                .offsets
                .offset(n.wrapping_mul(2 as guint) as isize)
                >= 0 as ::core::ffi::c_int
        {
            return n as gint;
        }
        entry = entry.offset(entrysize as isize);
    }
    return ((*first.offset(0 as ::core::ffi::c_int as isize) as gint) << 8 as ::core::ffi::c_int)
        + *first.offset(1 as ::core::ffi::c_int as isize) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_fetch_named(
    mut match_info: *const GMatchInfo,
    mut name: *const gchar,
) -> *mut gchar {
    let mut num: gint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    num = safe_c2rust_get_matched_substring_number(match_info, name);
    if num < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<gchar>();
    } else {
        return safe_c2rust_g_match_info_fetch(match_info, num);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_fetch_named_pos(
    mut match_info: *const GMatchInfo,
    mut name: *const gchar,
    mut start_pos: *mut gint,
    mut end_pos: *mut gint,
) -> gboolean {
    let mut num: gint = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    num = safe_c2rust_get_matched_substring_number(match_info, name);
    if num < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    return safe_c2rust_g_match_info_fetch_pos(match_info, num, start_pos, end_pos);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_match_info_fetch_all(
    mut match_info: *const GMatchInfo,
) -> *mut *mut gchar {
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !match_info.is_null() {
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
            b"match_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if (*match_info).matches < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    result = ({
        let mut __n: gsize =
            ((*match_info).matches as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*match_info).matches {
        let ref mut fresh5 = *result.offset(i as isize);
        *fresh5 = safe_c2rust_g_match_info_fetch(match_info, i);
        i += 1;
    }
    let ref mut fresh6 = *result.offset(i as isize);
    *fresh6 = ::core::ptr::null_mut::<gchar>();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-regex-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_ref(mut regex: *mut GRegex) -> *mut GRegex {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*regex).ref_count;
        (*regex).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*regex).ref_count, 1 as ::core::ffi::c_int);
    return regex;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_unref(mut regex: *mut GRegex) {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*regex).ref_count;
            (*regex).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*regex).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*regex).pattern as gpointer);
        if !(*regex).pcre_re.is_null() {
            pcre2_code_free_8((*regex).pcre_re);
        }
        g_free(regex as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_new(
    mut pattern: *const gchar,
    mut compile_options: GRegexCompileFlags,
    mut match_options: GRegexMatchFlags,
    mut error: *mut *mut GError,
) -> *mut GRegex {
    let mut regex: *mut GRegex = ::core::ptr::null_mut::<GRegex>();
    let mut re: *mut pcre2_code_8 = ::core::ptr::null_mut::<pcre2_code_8>();
    static mut safe_c2rust_initialised: gsize = 0 as gsize;
    let mut pcre_compile_options: uint32_t = 0;
    let mut pcre_match_options: uint32_t = 0;
    let mut newline_options: uint32_t = 0;
    let mut bsr_options: uint32_t = 0;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !pattern.is_null() {
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
            b"pattern != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if compile_options as ::core::ffi::c_uint
            & !(G_REGEX_DEFAULT as ::core::ffi::c_int
                | G_REGEX_CASELESS as ::core::ffi::c_int
                | G_REGEX_MULTILINE as ::core::ffi::c_int
                | G_REGEX_DOTALL as ::core::ffi::c_int
                | G_REGEX_EXTENDED as ::core::ffi::c_int
                | G_REGEX_ANCHORED as ::core::ffi::c_int
                | G_REGEX_DOLLAR_ENDONLY as ::core::ffi::c_int
                | G_REGEX_UNGREEDY as ::core::ffi::c_int
                | G_REGEX_RAW as ::core::ffi::c_int
                | G_REGEX_NO_AUTO_CAPTURE as ::core::ffi::c_int
                | G_REGEX_OPTIMIZE as ::core::ffi::c_int
                | G_REGEX_FIRSTLINE as ::core::ffi::c_int
                | G_REGEX_DUPNAMES as ::core::ffi::c_int
                | G_REGEX_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_JAVASCRIPT_COMPAT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
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
            b"(compile_options & ~(G_REGEX_COMPILE_MASK | G_REGEX_JAVASCRIPT_COMPAT)) == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
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
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialised;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut supports_utf8: ::core::ffi::c_int = 0;
        pcre2_config_8(
            PCRE2_CONFIG_UNICODE as uint32_t,
            &raw mut supports_utf8 as *mut ::core::ffi::c_void,
        );
        if supports_utf8 == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                glib_gettext(
                    b"PCRE library is compiled without UTF8 support\0" as *const u8 as *const gchar,
                ),
            );
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised = (if supports_utf8 != 0 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void,
            (if supports_utf8 != 0 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as gsize,
        );
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if safe_c2rust_initialised != 1 as gsize {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
        g_set_error_literal(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_COMPILE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"PCRE library is compiled with incompatible options\0" as *const u8
                    as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    pcre_compile_options = safe_c2rust_get_pcre2_compile_options(compile_options);
    pcre_match_options = safe_c2rust_get_pcre2_match_options(match_options, compile_options);
    newline_options = safe_c2rust_get_pcre2_newline_match_options(match_options);
    if newline_options == 0 as uint32_t {
        newline_options = safe_c2rust_get_pcre2_newline_compile_options(compile_options);
    }
    if newline_options == 0 as uint32_t {
        g_set_error(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS as ::core::ffi::c_int as gint,
            b"Invalid newline flags\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GRegex>();
    }
    bsr_options = safe_c2rust_get_pcre2_bsr_match_options(match_options);
    if bsr_options == 0 {
        bsr_options = safe_c2rust_get_pcre2_bsr_compile_options(compile_options);
    }
    re = safe_c2rust_regex_compile(
        pattern,
        pcre_compile_options,
        newline_options,
        bsr_options,
        error,
    );
    if re.is_null() {
        return ::core::ptr::null_mut::<GRegex>();
    }
    pcre_compile_options |= safe_c2rust_get_pcre2_inline_compile_options(re, pcre_compile_options);
    regex = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRegex>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRegex;
    (*regex).ref_count = 1 as ::core::ffi::c_int as gint;
    (*regex).pattern =
        safe_c2rust_g_strdup_inline(pattern as *const ::core::ffi::c_char) as *mut gchar;
    (*regex).pcre_re = re;
    (*regex).compile_opts = pcre_compile_options;
    (*regex).orig_compile_opts = compile_options;
    (*regex).match_opts = pcre_match_options;
    (*regex).orig_match_opts = match_options;
    return regex;
}
unsafe extern "C" fn safe_c2rust_regex_compile(
    mut pattern: *const gchar,
    mut compile_options: uint32_t,
    mut newline_options: uint32_t,
    mut bsr_options: uint32_t,
    mut error: *mut *mut GError,
) -> *mut pcre2_code_8 {
    let mut re: *mut pcre2_code_8 = ::core::ptr::null_mut::<pcre2_code_8>();
    let mut context: *mut pcre2_compile_context_8 =
        ::core::ptr::null_mut::<pcre2_compile_context_8>();
    let mut errmsg: *const gchar = ::core::ptr::null::<gchar>();
    let mut erroffset: size_t = 0;
    let mut errcode: gint = 0;
    context = pcre2_compile_context_create_8(::core::ptr::null_mut::<pcre2_general_context_8>());
    if pcre2_set_newline_8(context, newline_options) != 0 as ::core::ffi::c_int {
        g_set_error(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS as ::core::ffi::c_int as gint,
            b"Invalid newline flags\0" as *const u8 as *const gchar,
        );
        pcre2_compile_context_free_8(context);
        return ::core::ptr::null_mut::<pcre2_code_8>();
    }
    if pcre2_set_bsr_8(context, bsr_options) != 0 as ::core::ffi::c_int {
        g_set_error(
            error,
            safe_c2rust_g_regex_error_quark(),
            G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS as ::core::ffi::c_int as gint,
            b"Invalid BSR flags\0" as *const u8 as *const gchar,
        );
        pcre2_compile_context_free_8(context);
        return ::core::ptr::null_mut::<pcre2_code_8>();
    }
    if compile_options & PCRE2_UTF as uint32_t != 0 {
        compile_options = (compile_options as ::core::ffi::c_uint | PCRE2_NO_UTF_CHECK) as uint32_t;
    }
    compile_options = (compile_options as ::core::ffi::c_uint | PCRE2_UCP) as uint32_t;
    re = pcre2_compile_8(
        pattern as PCRE2_SPTR8,
        PCRE2_ZERO_TERMINATED,
        compile_options,
        &raw mut errcode,
        &raw mut erroffset,
        context,
    );
    pcre2_compile_context_free_8(context);
    if re.is_null() {
        let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut offset_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut pcre2_errmsg: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut original_errcode: ::core::ffi::c_int = 0;
        original_errcode = errcode as ::core::ffi::c_int;
        safe_c2rust_translate_compile_error(&raw mut errcode, &raw mut errmsg);
        if errmsg.is_null() {
            errmsg = glib_gettext(b"unknown error\0" as *const u8 as *const gchar);
            pcre2_errmsg = safe_c2rust_get_pcre2_error_string(original_errcode) as *mut gchar;
        }
        erroffset =
            g_utf8_pointer_to_offset(pattern, pattern.offset(erroffset as isize) as *const gchar)
                as size_t;
        offset_str = g_strdup_printf(b"%lu\0" as *const u8 as *const gchar, erroffset);
        tmp_error = g_error_new(
            safe_c2rust_g_regex_error_quark(),
            errcode,
            glib_gettext(
                b"Error while compiling regular expression \xE2\x80\x98%s\xE2\x80\x99 at char %s: %s\0"
                    as *const u8 as *const gchar,
            ),
            pattern,
            offset_str,
            if !pcre2_errmsg.is_null() { pcre2_errmsg as *const gchar } else { errmsg },
        );
        g_propagate_error(error, tmp_error);
        g_free(offset_str as gpointer);
        let mut _pp: *mut *mut gchar = &raw mut pcre2_errmsg;
        let mut _ptr: *mut gchar = *_pp;
        *_pp = ::core::ptr::null_mut::<gchar>();
        if !_ptr.is_null() {
            g_free(_ptr as gpointer);
        }
        return ::core::ptr::null_mut::<pcre2_code_8>();
    }
    return re;
}
unsafe extern "C" fn safe_c2rust_get_pcre2_inline_compile_options(
    mut re: *mut pcre2_code_8,
    mut compile_options: uint32_t,
) -> uint32_t {
    let mut pcre_compile_options: uint32_t = 0;
    let mut nonpcre_compile_options: uint32_t = 0;
    nonpcre_compile_options = compile_options & G_REGEX_COMPILE_NONPCRE_MASK as uint32_t;
    pcre2_pattern_info_8(
        re,
        PCRE2_INFO_ALLOPTIONS as uint32_t,
        &raw mut pcre_compile_options as *mut ::core::ffi::c_void,
    );
    compile_options = pcre_compile_options & G_REGEX_PCRE2_COMPILE_MASK as uint32_t;
    compile_options |= nonpcre_compile_options;
    if compile_options & PCRE2_DUPNAMES as uint32_t == 0 {
        let mut jchanged: uint32_t = 0 as uint32_t;
        pcre2_pattern_info_8(
            re,
            PCRE2_INFO_JCHANGED as uint32_t,
            &raw mut jchanged as *mut ::core::ffi::c_void,
        );
        if jchanged != 0 {
            compile_options = (compile_options as ::core::ffi::c_uint | PCRE2_DUPNAMES) as uint32_t;
        }
    }
    return compile_options;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_pattern(mut regex: *const GRegex) -> *const gchar {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*regex).pattern;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_max_backref(mut regex: *const GRegex) -> gint {
    let mut value: uint32_t = 0;
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_BACKREFMAX as uint32_t,
        &raw mut value as *mut ::core::ffi::c_void,
    );
    return value as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_capture_count(mut regex: *const GRegex) -> gint {
    let mut value: uint32_t = 0;
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_CAPTURECOUNT as uint32_t,
        &raw mut value as *mut ::core::ffi::c_void,
    );
    return value as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_has_cr_or_lf(
    mut regex: *const GRegex,
) -> gboolean {
    let mut value: uint32_t = 0;
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_HASCRORLF as uint32_t,
        &raw mut value as *mut ::core::ffi::c_void,
    );
    return (value != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_max_lookbehind(mut regex: *const GRegex) -> gint {
    let mut max_lookbehind: uint32_t = 0;
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_MAXLOOKBEHIND as uint32_t,
        &raw mut max_lookbehind as *mut ::core::ffi::c_void,
    );
    return max_lookbehind as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_compile_flags(
    mut regex: *const GRegex,
) -> GRegexCompileFlags {
    let mut extra_flags: GRegexCompileFlags = G_REGEX_DEFAULT;
    let mut info_value: uint32_t = 0;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_REGEX_DEFAULT;
    }
    extra_flags = ((*regex).orig_compile_opts as ::core::ffi::c_uint
        & G_REGEX_OPTIMIZE as ::core::ffi::c_int as ::core::ffi::c_uint)
        as GRegexCompileFlags;
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_NEWLINE as uint32_t,
        &raw mut info_value as *mut ::core::ffi::c_void,
    );
    match info_value {
        5 => {
            extra_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
                extra_flags as ::core::ffi::c_uint
                    | G_REGEX_NEWLINE_ANYCRLF as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        3 => {
            extra_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
                extra_flags as ::core::ffi::c_uint
                    | G_REGEX_NEWLINE_CRLF as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        2 => {
            extra_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
                extra_flags as ::core::ffi::c_uint
                    | G_REGEX_NEWLINE_LF as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        1 => {
            extra_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
                extra_flags as ::core::ffi::c_uint
                    | G_REGEX_NEWLINE_CR as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        _ => {}
    }
    pcre2_pattern_info_8(
        (*regex).pcre_re,
        PCRE2_INFO_BSR as uint32_t,
        &raw mut info_value as *mut ::core::ffi::c_void,
    );
    match info_value {
        2 => {
            extra_flags = ::core::mem::transmute::<::core::ffi::c_uint, GRegexCompileFlags>(
                extra_flags as ::core::ffi::c_uint
                    | G_REGEX_BSR_ANYCRLF as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        _ => {}
    }
    return (safe_c2rust_g_regex_compile_flags_from_pcre2((*regex).compile_opts)
        as ::core::ffi::c_uint
        | extra_flags as ::core::ffi::c_uint) as GRegexCompileFlags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_match_flags(
    mut regex: *const GRegex,
) -> GRegexMatchFlags {
    let mut flags: uint32_t = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_REGEX_MATCH_DEFAULT;
    }
    flags = safe_c2rust_g_regex_match_flags_from_pcre2((*regex).match_opts) as uint32_t;
    flags = (flags as ::core::ffi::c_uint
        | (*regex).orig_match_opts as ::core::ffi::c_uint
            & (G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as uint32_t;
    flags = (flags as ::core::ffi::c_uint
        | (*regex).orig_match_opts as ::core::ffi::c_uint
            & (G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as uint32_t;
    return flags as GRegexMatchFlags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_match_simple(
    mut pattern: *const gchar,
    mut string: *const gchar,
    mut compile_options: GRegexCompileFlags,
    mut match_options: GRegexMatchFlags,
) -> gboolean {
    let mut regex: *mut GRegex = ::core::ptr::null_mut::<GRegex>();
    let mut result: gboolean = 0;
    regex = safe_c2rust_g_regex_new(
        pattern,
        compile_options,
        G_REGEX_MATCH_DEFAULT,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if regex.is_null() {
        return FALSE;
    }
    result = safe_c2rust_g_regex_match_full(
        regex,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        0 as gint,
        match_options,
        ::core::ptr::null_mut::<*mut GMatchInfo>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    safe_c2rust_g_regex_unref(regex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_match(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut match_options: GRegexMatchFlags,
    mut match_info: *mut *mut GMatchInfo,
) -> gboolean {
    return safe_c2rust_g_regex_match_full(
        regex,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        0 as gint,
        match_options,
        match_info,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_match_full(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut match_options: GRegexMatchFlags,
    mut match_info: *mut *mut GMatchInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GMatchInfo = ::core::ptr::null_mut::<GMatchInfo>();
    let mut match_ok: gboolean = 0;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !regex.is_null() {
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
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if start_position >= 0 as ::core::ffi::c_int {
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
            b"start_position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
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
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    info = safe_c2rust_match_info_new(
        regex,
        string,
        string_len as gint,
        start_position,
        match_options,
        FALSE,
    );
    match_ok = safe_c2rust_g_match_info_next(info, error);
    if !match_info.is_null() {
        *match_info = info;
    } else {
        safe_c2rust_g_match_info_free(info);
    }
    return match_ok;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_match_all(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut match_options: GRegexMatchFlags,
    mut match_info: *mut *mut GMatchInfo,
) -> gboolean {
    return safe_c2rust_g_regex_match_all_full(
        regex,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        0 as gint,
        match_options,
        match_info,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_match_all_full(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut match_options: GRegexMatchFlags,
    mut match_info: *mut *mut GMatchInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GMatchInfo = ::core::ptr::null_mut::<GMatchInfo>();
    let mut done: gboolean = 0;
    let mut pcre_re: *mut pcre2_code_8 = ::core::ptr::null_mut::<pcre2_code_8>();
    let mut retval: gboolean = 0;
    let mut newline_options: uint32_t = 0;
    let mut bsr_options: uint32_t = 0;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !regex.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
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
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if start_position >= 0 as ::core::ffi::c_int {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"start_position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    newline_options = safe_c2rust_get_pcre2_newline_match_options(match_options);
    if newline_options == 0 {
        newline_options = safe_c2rust_get_pcre2_newline_compile_options((*regex).orig_compile_opts);
    }
    bsr_options = safe_c2rust_get_pcre2_bsr_match_options(match_options);
    if bsr_options == 0 {
        bsr_options = safe_c2rust_get_pcre2_bsr_compile_options((*regex).orig_compile_opts);
    }
    pcre_re = safe_c2rust_regex_compile(
        (*regex).pattern,
        (*regex).compile_opts | PCRE2_NO_AUTO_POSSESS as uint32_t,
        newline_options,
        bsr_options,
        error,
    );
    if pcre_re.is_null() {
        return FALSE;
    }
    info = safe_c2rust_match_info_new(
        regex,
        string,
        string_len as gint,
        start_position,
        match_options,
        TRUE,
    );
    done = FALSE as gboolean;
    while done == 0 {
        done = TRUE as gboolean;
        (*info).matches = pcre2_dfa_match_8(
            pcre_re,
            (*info).string as PCRE2_SPTR8,
            (*info).string_len as size_t,
            (*info).pos as size_t,
            (*regex).match_opts | (*info).match_opts,
            (*info).match_data,
            (*info).match_context,
            (*info).workspace as *mut ::core::ffi::c_int,
            (*info).n_workspace,
        ) as gint;
        if (*info).matches == PCRE2_ERROR_DFA_WSSIZE {
            (*info).n_workspace = (*info).n_workspace.wrapping_mul(2 as size_t);
            (*info).workspace = g_realloc_n(
                (*info).workspace as gpointer,
                (*info).n_workspace as gsize,
                ::core::mem::size_of::<gint>() as gsize,
            ) as *mut gint;
            done = FALSE as gboolean;
        } else if (*info).matches == 0 as ::core::ffi::c_int {
            (*info).n_offsets = (*info).n_offsets.wrapping_mul(2 as uint32_t);
            (*info).offsets = g_realloc_n(
                (*info).offsets as gpointer,
                (*info).n_offsets as gsize,
                ::core::mem::size_of::<gint>() as gsize,
            ) as *mut gint;
            pcre2_match_data_free_8((*info).match_data);
            (*info).match_data = pcre2_match_data_create_8(
                (*info).n_offsets,
                ::core::ptr::null_mut::<pcre2_general_context_8>(),
            );
            done = FALSE as gboolean;
        } else if (*info).matches < PCRE2_ERROR_NOMATCH && (*info).matches != PCRE2_ERROR_PARTIAL {
            let mut error_msg: *mut gchar =
                safe_c2rust_get_match_error_message((*info).matches as ::core::ffi::c_int)
                    as *mut gchar;
            g_set_error(
                error,
                safe_c2rust_g_regex_error_quark(),
                G_REGEX_ERROR_MATCH as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error while matching regular expression %s: %s\0" as *const u8
                        as *const gchar,
                ),
                (*regex).pattern,
                error_msg,
            );
            let mut _pp: *mut *mut gchar = &raw mut error_msg;
            let mut _ptr: *mut gchar = *_pp;
            *_pp = ::core::ptr::null_mut::<gchar>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
        } else if (*info).matches != PCRE2_ERROR_NOMATCH {
            if safe_c2rust_recalc_match_offsets(info, error) == 0 {
                (*info).matches = PCRE2_ERROR_NOMATCH as gint;
            }
        }
    }
    pcre2_code_free_8(pcre_re);
    (*info).pos = -(1 as ::core::ffi::c_int) as gint;
    retval = ((*info).matches >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    if !match_info.is_null() {
        *match_info = info;
    } else {
        safe_c2rust_g_match_info_free(info);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_get_string_number(
    mut regex: *const GRegex,
    mut name: *const gchar,
) -> gint {
    let mut num: gint = 0;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !regex.is_null() {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    num = pcre2_substring_number_from_name_8((*regex).pcre_re, name as PCRE2_SPTR8) as gint;
    if num == PCRE2_ERROR_NOSUBSTRING {
        num = -(1 as ::core::ffi::c_int) as gint;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_split_simple(
    mut pattern: *const gchar,
    mut string: *const gchar,
    mut compile_options: GRegexCompileFlags,
    mut match_options: GRegexMatchFlags,
) -> *mut *mut gchar {
    let mut regex: *mut GRegex = ::core::ptr::null_mut::<GRegex>();
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    regex = safe_c2rust_g_regex_new(
        pattern,
        compile_options,
        G_REGEX_MATCH_DEFAULT,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if regex.is_null() {
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    result = safe_c2rust_g_regex_split_full(
        regex,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        0 as gint,
        match_options,
        0 as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    safe_c2rust_g_regex_unref(regex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_split(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut match_options: GRegexMatchFlags,
) -> *mut *mut gchar {
    return safe_c2rust_g_regex_split_full(
        regex,
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        0 as gint,
        match_options,
        0 as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_split_full(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut match_options: GRegexMatchFlags,
    mut max_tokens: gint,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut match_info: *mut GMatchInfo = ::core::ptr::null_mut::<GMatchInfo>();
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: gint = 0;
    let mut token_count: gint = 0;
    let mut match_ok: gboolean = 0;
    let mut last_separator_end: gint = 0;
    let mut last_match_is_empty: gboolean = 0;
    let mut string_list: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !regex.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if start_position >= 0 as ::core::ffi::c_int {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"start_position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if max_tokens <= 0 as ::core::ffi::c_int {
        max_tokens = G_MAXINT as gint;
    }
    if string_len < 0 as gssize {
        string_len = strlen(string as *const ::core::ffi::c_char) as gssize;
    }
    if string_len - start_position as gssize == 0 as gssize {
        return ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
    }
    if max_tokens == 1 as ::core::ffi::c_int {
        string_list = ({
            let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        let ref mut fresh1 = *string_list.offset(0 as ::core::ffi::c_int as isize);
        *fresh1 = g_strndup(
            string.offset(start_position as isize) as *const gchar,
            (string_len - start_position as gssize) as gsize,
        );
        return string_list;
    }
    list = ::core::ptr::null_mut::<GList>();
    token_count = 0 as ::core::ffi::c_int as gint;
    last_separator_end = start_position;
    last_match_is_empty = FALSE as gboolean;
    match_ok = safe_c2rust_g_regex_match_full(
        regex,
        string,
        string_len,
        start_position,
        match_options,
        &raw mut match_info,
        &raw mut tmp_error,
    );
    while tmp_error.is_null() {
        if match_ok != 0 {
            last_match_is_empty = (*(*match_info)
                .offsets
                .offset(0 as ::core::ffi::c_int as isize)
                == *(*match_info)
                    .offsets
                    .offset(1 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_int as gboolean;
            if last_separator_end
                != *(*match_info)
                    .offsets
                    .offset(1 as ::core::ffi::c_int as isize)
            {
                let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut match_count: gint = 0;
                token = g_strndup(
                    string.offset(last_separator_end as isize),
                    (*(*match_info)
                        .offsets
                        .offset(0 as ::core::ffi::c_int as isize)
                        - last_separator_end) as gsize,
                );
                list = g_list_prepend(list, token as gpointer);
                token_count += 1;
                match_count = safe_c2rust_g_match_info_get_match_count(match_info);
                if match_count > 1 as ::core::ffi::c_int {
                    i = 1 as ::core::ffi::c_int as gint;
                    while i < match_count {
                        list = g_list_prepend(
                            list,
                            safe_c2rust_g_match_info_fetch(match_info, i) as gpointer,
                        );
                        i += 1;
                    }
                }
            }
            if token_count >= max_tokens as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                if last_match_is_empty != 0 {
                    (*match_info).pos = (if (*regex).compile_opts
                        & G_REGEX_RAW as ::core::ffi::c_int as uint32_t
                        != 0
                    {
                        (string.offset((*match_info).pos as isize) as *const gchar)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                    } else {
                        g_utf8_prev_char(string.offset((*match_info).pos as isize) as *const gchar)
                            as *const gchar
                    })
                    .offset_from(string)
                        as ::core::ffi::c_long as gint;
                }
                if string_len > (*match_info).pos as gssize {
                    let mut token_1: *mut gchar = g_strndup(
                        string.offset((*match_info).pos as isize),
                        (string_len - (*match_info).pos as gssize) as gsize,
                    );
                    list = g_list_prepend(list, token_1 as gpointer);
                }
                break;
            } else {
                last_separator_end = (*match_info).pos;
                if last_match_is_empty != 0 {
                    last_separator_end = (if (*regex).compile_opts
                        & G_REGEX_RAW as ::core::ffi::c_int as uint32_t
                        != 0
                    {
                        (string.offset(last_separator_end as isize) as *const gchar)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                    } else {
                        g_utf8_prev_char(string.offset(last_separator_end as isize) as *const gchar)
                            as *const gchar
                    })
                    .offset_from(string)
                        as ::core::ffi::c_long as gint;
                }
                match_ok = safe_c2rust_g_match_info_next(match_info, &raw mut tmp_error);
            }
        } else {
            if last_match_is_empty == 0 {
                let mut token_0: *mut gchar = g_strndup(
                    string.offset(last_separator_end as isize),
                    ((*match_info).string_len - last_separator_end as gssize) as gsize,
                );
                list = g_list_prepend(list, token_0 as gpointer);
            }
            break;
        }
    }
    safe_c2rust_g_match_info_free(match_info);
    if !tmp_error.is_null() {
        g_propagate_error(error, tmp_error);
        g_list_free_full(list, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    string_list = ({
        let mut __n: gsize = g_list_length(list).wrapping_add(1 as guint) as gsize;
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
    i = 0 as ::core::ffi::c_int as gint;
    last = g_list_last(list);
    while !last.is_null() {
        let fresh2 = i;
        i = i + 1;
        let ref mut fresh3 = *string_list.offset(fresh2 as isize);
        *fresh3 = (*last).data as *mut gchar;
        last = if !last.is_null() {
            (*last).prev
        } else {
            ::core::ptr::null_mut::<GList>()
        };
    }
    let ref mut fresh4 = *string_list.offset(i as isize);
    *fresh4 = ::core::ptr::null_mut::<gchar>();
    g_list_free(list);
    return string_list;
}
unsafe extern "C" fn safe_c2rust_free_interpolation_data(mut data: *mut InterpolationData) {
    g_free((*data).text as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_expand_escape(
    mut replacement: *const gchar,
    mut p: *const gchar,
    mut data: *mut InterpolationData,
    mut error: *mut *mut GError,
) -> *const gchar {
    let mut current_block: u64;
    let mut q: *const gchar = ::core::ptr::null::<gchar>();
    let mut r: *const gchar = ::core::ptr::null::<gchar>();
    let mut x: gint = 0;
    let mut d: gint = 0;
    let mut h: gint = 0;
    let mut i: gint = 0;
    let mut error_detail: *const gchar = ::core::ptr::null::<gchar>();
    let mut base: gint = 0 as gint;
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    p = p.offset(1);
    match *p as ::core::ffi::c_int {
        116 => {
            p = p.offset(1);
            (*data).c = '\t' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        110 => {
            p = p.offset(1);
            (*data).c = '\n' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        118 => {
            p = p.offset(1);
            (*data).c = '\u{b}' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        114 => {
            p = p.offset(1);
            (*data).c = '\r' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        102 => {
            p = p.offset(1);
            (*data).c = '\u{c}' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        97 => {
            p = p.offset(1);
            (*data).c = '\u{7}' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        98 => {
            p = p.offset(1);
            (*data).c = '\u{8}' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        92 => {
            p = p.offset(1);
            (*data).c = '\\' as i32 as gchar;
            (*data).type_0 = REPL_TYPE_CHARACTER as ::core::ffi::c_int as gint;
            current_block = 14913924298693586572;
        }
        120 => {
            p = p.offset(1);
            x = 0 as ::core::ffi::c_int as gint;
            if *p as ::core::ffi::c_int == '{' as i32 {
                p = p.offset(1);
                loop {
                    h = g_ascii_xdigit_value(*p);
                    if h < 0 as ::core::ffi::c_int {
                        error_detail = glib_gettext(
                            b"hexadecimal digit or \xE2\x80\x9C}\xE2\x80\x9D expected\0"
                                as *const u8 as *const gchar,
                        );
                        current_block = 3035155727157373046;
                        break;
                    } else {
                        x = x * 16 as gint + h;
                        p = p.offset(1);
                        if !(*p as ::core::ffi::c_int != '}' as i32) {
                            current_block = 652864300344834934;
                            break;
                        }
                    }
                }
                match current_block {
                    3035155727157373046 => {}
                    _ => {
                        p = p.offset(1);
                        current_block = 3934796541983872331;
                    }
                }
            } else {
                i = 0 as ::core::ffi::c_int as gint;
                loop {
                    if !(i < 2 as ::core::ffi::c_int) {
                        current_block = 3934796541983872331;
                        break;
                    }
                    h = g_ascii_xdigit_value(*p);
                    if h < 0 as ::core::ffi::c_int {
                        error_detail = glib_gettext(
                            b"hexadecimal digit expected\0" as *const u8 as *const gchar,
                        );
                        current_block = 3035155727157373046;
                        break;
                    } else {
                        x = x * 16 as gint + h;
                        p = p.offset(1);
                        i += 1;
                    }
                }
            }
            match current_block {
                3035155727157373046 => {}
                _ => {
                    (*data).type_0 = REPL_TYPE_STRING as ::core::ffi::c_int as gint;
                    (*data).text = ({
                        let mut __n: gsize = 8 as ::core::ffi::c_int as gsize;
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
                    g_unichar_to_utf8(x as gunichar, (*data).text);
                    current_block = 14913924298693586572;
                }
            }
        }
        108 => {
            p = p.offset(1);
            (*data).type_0 = REPL_TYPE_CHANGE_CASE as ::core::ffi::c_int as gint;
            (*data).change_case = CHANGE_CASE_LOWER_SINGLE;
            current_block = 14913924298693586572;
        }
        117 => {
            p = p.offset(1);
            (*data).type_0 = REPL_TYPE_CHANGE_CASE as ::core::ffi::c_int as gint;
            (*data).change_case = CHANGE_CASE_UPPER_SINGLE;
            current_block = 14913924298693586572;
        }
        76 => {
            p = p.offset(1);
            (*data).type_0 = REPL_TYPE_CHANGE_CASE as ::core::ffi::c_int as gint;
            (*data).change_case = CHANGE_CASE_LOWER;
            current_block = 14913924298693586572;
        }
        85 => {
            p = p.offset(1);
            (*data).type_0 = REPL_TYPE_CHANGE_CASE as ::core::ffi::c_int as gint;
            (*data).change_case = CHANGE_CASE_UPPER;
            current_block = 14913924298693586572;
        }
        69 => {
            p = p.offset(1);
            (*data).type_0 = REPL_TYPE_CHANGE_CASE as ::core::ffi::c_int as gint;
            (*data).change_case = CHANGE_CASE_NONE;
            current_block = 14913924298693586572;
        }
        103 => {
            p = p.offset(1);
            if *p as ::core::ffi::c_int != '<' as i32 {
                error_detail = glib_gettext(
                    b"missing \xE2\x80\x9C<\xE2\x80\x9D in symbolic reference\0" as *const u8
                        as *const gchar,
                );
                current_block = 3035155727157373046;
            } else {
                q = p.offset(1 as ::core::ffi::c_int as isize);
                loop {
                    p = p.offset(1);
                    if *p == 0 {
                        error_detail = glib_gettext(
                            b"unfinished symbolic reference\0" as *const u8 as *const gchar,
                        );
                        current_block = 3035155727157373046;
                        break;
                    } else if !(*p as ::core::ffi::c_int != '>' as i32) {
                        current_block = 9437013279121998969;
                        break;
                    }
                }
                match current_block {
                    3035155727157373046 => {}
                    _ => {
                        if p.offset_from(q) as ::core::ffi::c_long == 0 as ::core::ffi::c_long {
                            error_detail = glib_gettext(
                                b"zero-length symbolic reference\0" as *const u8 as *const gchar,
                            );
                            current_block = 3035155727157373046;
                        } else {
                            if *safe_c2rust_g_ascii_table.offset(*q as guchar as isize)
                                as ::core::ffi::c_int
                                & G_ASCII_DIGIT as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                            {
                                x = 0 as ::core::ffi::c_int as gint;
                                loop {
                                    h = g_ascii_digit_value(*q);
                                    if h < 0 as ::core::ffi::c_int {
                                        error_detail = glib_gettext(
                                            b"digit expected\0" as *const u8 as *const gchar,
                                        );
                                        p = q;
                                        current_block = 3035155727157373046;
                                        break;
                                    } else {
                                        x = x * 10 as gint + h;
                                        q = q.offset(1);
                                        if !(q != p) {
                                            current_block = 7158658067966855297;
                                            break;
                                        }
                                    }
                                }
                                match current_block {
                                    3035155727157373046 => {}
                                    _ => {
                                        (*data).num = x;
                                        (*data).type_0 = REPL_TYPE_NUMERIC_REFERENCE
                                            as ::core::ffi::c_int
                                            as gint;
                                        current_block = 17167606947040001567;
                                    }
                                }
                            } else {
                                r = q;
                                loop {
                                    if !(*safe_c2rust_g_ascii_table.offset(*r as guchar as isize)
                                        as ::core::ffi::c_int
                                        & G_ASCII_ALNUM as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int)
                                    {
                                        error_detail = glib_gettext(
                                            b"illegal symbolic reference\0" as *const u8
                                                as *const gchar,
                                        );
                                        p = r;
                                        current_block = 3035155727157373046;
                                        break;
                                    } else {
                                        r = r.offset(1);
                                        if !(r != p) {
                                            current_block = 17485376261910781866;
                                            break;
                                        }
                                    }
                                }
                                match current_block {
                                    3035155727157373046 => {}
                                    _ => {
                                        (*data).text = g_strndup(
                                            q,
                                            p.offset_from(q) as ::core::ffi::c_long as gsize,
                                        );
                                        (*data).type_0 = REPL_TYPE_SYMBOLIC_REFERENCE
                                            as ::core::ffi::c_int
                                            as gint;
                                        current_block = 17167606947040001567;
                                    }
                                }
                            }
                            match current_block {
                                3035155727157373046 => {}
                                _ => {
                                    p = p.offset(1);
                                    current_block = 14913924298693586572;
                                }
                            }
                        }
                    }
                }
            }
        }
        48 => {
            if g_ascii_digit_value(
                *(p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char),
            ) >= 0 as ::core::ffi::c_int
            {
                base = 8 as ::core::ffi::c_int as gint;
                p = p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
            current_block = 4207629312478840922;
        }
        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            current_block = 4207629312478840922;
        }
        0 => {
            error_detail = glib_gettext(
                b"stray final \xE2\x80\x9C\\\xE2\x80\x9D\0" as *const u8 as *const gchar,
            );
            current_block = 3035155727157373046;
        }
        _ => {
            error_detail = glib_gettext(b"unknown escape sequence\0" as *const u8 as *const gchar);
            current_block = 3035155727157373046;
        }
    }
    match current_block {
        4207629312478840922 => {
            x = 0 as ::core::ffi::c_int as gint;
            d = 0 as ::core::ffi::c_int as gint;
            i = 0 as ::core::ffi::c_int as gint;
            while i < 3 as ::core::ffi::c_int {
                h = g_ascii_digit_value(*p);
                if h < 0 as ::core::ffi::c_int {
                    break;
                }
                if h > 7 as ::core::ffi::c_int {
                    if base == 8 as ::core::ffi::c_int {
                        break;
                    }
                    base = 10 as ::core::ffi::c_int as gint;
                }
                if i == 2 as ::core::ffi::c_int && base == 10 as ::core::ffi::c_int {
                    break;
                }
                x = x * 8 as gint + h;
                d = d * 10 as gint + h;
                p = p.offset(1);
                i += 1;
            }
            if base == 8 as ::core::ffi::c_int || i == 3 as ::core::ffi::c_int {
                (*data).type_0 = REPL_TYPE_STRING as ::core::ffi::c_int as gint;
                (*data).text = ({
                    let mut __n: gsize = 8 as ::core::ffi::c_int as gsize;
                    let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
                    let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    if __s == 1 as gsize {
                        __p = g_malloc0(__n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_malloc0(__n.wrapping_mul(__s));
                    } else {
                        __p = g_malloc0_n(__n, __s);
                    }
                    __p
                }) as *mut gchar;
                g_unichar_to_utf8(x as gunichar, (*data).text);
            } else {
                (*data).type_0 = REPL_TYPE_NUMERIC_REFERENCE as ::core::ffi::c_int as gint;
                (*data).num = d;
            }
        }
        3035155727157373046 => {
            tmp_error = g_error_new(
                safe_c2rust_g_regex_error_quark(),
                G_REGEX_ERROR_REPLACE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error while parsing replacement text \xE2\x80\x9C%s\xE2\x80\x9D at char %lu: %s\0"
                        as *const u8 as *const gchar,
                ),
                replacement,
                p.offset_from(replacement) as ::core::ffi::c_long as gulong,
                error_detail,
            );
            g_propagate_error(error, tmp_error);
            return ::core::ptr::null::<gchar>();
        }
        _ => {}
    }
    return p;
}
unsafe extern "C" fn safe_c2rust_split_replacement(
    mut replacement: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut data: *mut InterpolationData = ::core::ptr::null_mut::<InterpolationData>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut start: *const gchar = ::core::ptr::null::<gchar>();
    p = replacement;
    start = p;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '\\' as i32 {
            data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<InterpolationData>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut InterpolationData;
            p = safe_c2rust_expand_escape(replacement, p, data, error);
            start = p;
            if p.is_null() {
                g_list_free_full(
                    list,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut InterpolationData) -> ()>,
                        GDestroyNotify,
                    >(Some(
                        safe_c2rust_free_interpolation_data
                            as unsafe extern "C" fn(*mut InterpolationData) -> (),
                    )),
                );
                safe_c2rust_free_interpolation_data(data);
                return ::core::ptr::null_mut::<GList>();
            }
            list = g_list_prepend(list, data as gpointer);
        } else {
            p = p.offset(1);
            if *p as ::core::ffi::c_int == '\\' as i32 || *p as ::core::ffi::c_int == '\0' as i32 {
                if p.offset_from(start) as ::core::ffi::c_long > 0 as ::core::ffi::c_long {
                    data = ({
                        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                        let mut __s: gsize = ::core::mem::size_of::<InterpolationData>() as gsize;
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
                    }) as *mut InterpolationData;
                    (*data).text =
                        g_strndup(start, p.offset_from(start) as ::core::ffi::c_long as gsize);
                    (*data).type_0 = REPL_TYPE_STRING as ::core::ffi::c_int as gint;
                    list = g_list_prepend(list, data as gpointer);
                }
            }
        }
    }
    return g_list_reverse(list);
}
unsafe extern "C" fn safe_c2rust_string_append(
    mut string: *mut GString,
    mut text: *const gchar,
    mut change_case: *mut ChangeCase,
) {
    let mut c: gunichar = 0;
    if *text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    if *change_case as ::core::ffi::c_uint
        == CHANGE_CASE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = text as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_62
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
                string,
                text as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else if *change_case as ::core::ffi::c_uint
        & CHANGE_CASE_SINGLE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        c = g_utf8_get_char(text);
        g_string_append_unichar(
            string,
            if *change_case as ::core::ffi::c_uint
                & CHANGE_CASE_LOWER_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                g_unichar_tolower(c)
            } else {
                g_unichar_toupper(c)
            },
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = text.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(text as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                )
                    as *mut ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_63
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
                string,
                text.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(text as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        *change_case = CHANGE_CASE_NONE;
    } else {
        while *text as ::core::ffi::c_int != '\0' as i32 {
            c = g_utf8_get_char(text);
            g_string_append_unichar(
                string,
                if *change_case as ::core::ffi::c_uint
                    & CHANGE_CASE_LOWER_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    g_unichar_tolower(c)
                } else {
                    g_unichar_toupper(c)
                },
            );
            text = text.offset(
                *safe_c2rust_g_utf8_skip.offset(*(text as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
    };
}
unsafe extern "C" fn safe_c2rust_interpolate_replacement(
    mut match_info: *const GMatchInfo,
    mut result: *mut GString,
    mut data: gpointer,
) -> gboolean {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut idata: *mut InterpolationData = ::core::ptr::null_mut::<InterpolationData>();
    let mut match_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut change_case: ChangeCase = CHANGE_CASE_NONE;
    list = data as *mut GList;
    while !list.is_null() {
        idata = (*list).data as *mut InterpolationData;
        match (*idata).type_0 {
            0 => {
                safe_c2rust_string_append(result, (*idata).text, &raw mut change_case);
            }
            1 => {
                safe_c2rust_g_string_append_c_inline(
                    result,
                    (if change_case as ::core::ffi::c_uint
                        & CHANGE_CASE_LOWER_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                    {
                        g_unichar_tolower((*idata).c as gunichar)
                    } else {
                        g_unichar_toupper((*idata).c as gunichar)
                    }) as gchar,
                );
                if change_case as ::core::ffi::c_uint
                    & CHANGE_CASE_SINGLE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    change_case = CHANGE_CASE_NONE;
                }
            }
            3 => {
                match_0 = safe_c2rust_g_match_info_fetch(match_info, (*idata).num);
                if !match_0.is_null() {
                    safe_c2rust_string_append(result, match_0, &raw mut change_case);
                    g_free(match_0 as gpointer);
                }
            }
            2 => {
                match_0 = safe_c2rust_g_match_info_fetch_named(match_info, (*idata).text);
                if !match_0.is_null() {
                    safe_c2rust_string_append(result, match_0, &raw mut change_case);
                    g_free(match_0 as gpointer);
                }
            }
            4 => {
                change_case = (*idata).change_case;
            }
            _ => {}
        }
        list = (*list).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_interpolation_list_needs_match(mut list: *mut GList) -> gboolean {
    while !list.is_null() {
        let mut data: *mut InterpolationData = (*list).data as *mut InterpolationData;
        if (*data).type_0 == REPL_TYPE_SYMBOLIC_REFERENCE as ::core::ffi::c_int
            || (*data).type_0 == REPL_TYPE_NUMERIC_REFERENCE as ::core::ffi::c_int
        {
            return TRUE;
        }
        list = (*list).next;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_replace(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut replacement: *const gchar,
    mut match_options: GRegexMatchFlags,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !regex.is_null() {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if start_position >= 0 as ::core::ffi::c_int {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"start_position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !replacement.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"replacement != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    list = safe_c2rust_split_replacement(replacement, &raw mut tmp_error);
    if !tmp_error.is_null() {
        g_propagate_error(error, tmp_error);
        return ::core::ptr::null_mut::<gchar>();
    }
    result = safe_c2rust_g_regex_replace_eval(
        regex,
        string,
        string_len,
        start_position,
        match_options,
        Some(
            safe_c2rust_interpolate_replacement
                as unsafe extern "C" fn(*const GMatchInfo, *mut GString, gpointer) -> gboolean,
        ),
        list as gpointer,
        &raw mut tmp_error,
    );
    if !tmp_error.is_null() {
        g_propagate_error(error, tmp_error);
    }
    g_list_free_full(
        list,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut InterpolationData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_free_interpolation_data
                as unsafe extern "C" fn(*mut InterpolationData) -> (),
        )),
    );
    return result;
}
unsafe extern "C" fn safe_c2rust_literal_replacement(
    mut match_info: *const GMatchInfo,
    mut result: *mut GString,
    mut data: gpointer,
) -> gboolean {
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = data as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                result,
                __val,
                if ({
                    let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_70 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_70 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_70
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
            result,
            data as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_replace_literal(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut replacement: *const gchar,
    mut match_options: GRegexMatchFlags,
    mut error: *mut *mut GError,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !replacement.is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"replacement != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_regex_replace_eval(
        regex,
        string,
        string_len,
        start_position,
        match_options,
        Some(
            safe_c2rust_literal_replacement
                as unsafe extern "C" fn(*const GMatchInfo, *mut GString, gpointer) -> gboolean,
        ),
        replacement as gpointer,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_replace_eval(
    mut regex: *const GRegex,
    mut string: *const gchar,
    mut string_len: gssize,
    mut start_position: gint,
    mut match_options: GRegexMatchFlags,
    mut eval: GRegexEvalCallback,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut match_info: *mut GMatchInfo = ::core::ptr::null_mut::<GMatchInfo>();
    let mut result: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut str_pos: gint = 0 as gint;
    let mut done: gboolean = FALSE;
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if !regex.is_null() {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"regex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if start_position >= 0 as ::core::ffi::c_int {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"start_position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if eval.is_some() {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"eval != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if match_options as ::core::ffi::c_uint
            & !(G_REGEX_MATCH_DEFAULT as ::core::ffi::c_int
                | G_REGEX_MATCH_ANCHORED as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTBOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEOL as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CR as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_LF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_CRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_NEWLINE_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANYCRLF as ::core::ffi::c_int
                | G_REGEX_MATCH_BSR_ANY as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_SOFT as ::core::ffi::c_int
                | G_REGEX_MATCH_PARTIAL_HARD as ::core::ffi::c_int
                | G_REGEX_MATCH_NOTEMPTY_ATSTART as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(match_options & ~G_REGEX_MATCH_MASK) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if string_len < 0 as gssize {
        string_len = strlen(string as *const ::core::ffi::c_char) as gssize;
    }
    result = g_string_sized_new(string_len as gsize);
    safe_c2rust_g_regex_match_full(
        regex,
        string,
        string_len,
        start_position,
        match_options,
        &raw mut match_info,
        &raw mut tmp_error,
    );
    while done == 0 && safe_c2rust_g_match_info_matches(match_info) != 0 {
        safe_c2rust_g_string_append_len_inline(
            result,
            string.offset(str_pos as isize),
            (*(*match_info)
                .offsets
                .offset(0 as ::core::ffi::c_int as isize)
                - str_pos) as gssize,
        );
        done = Some(eval.expect("non-null function pointer")).expect("non-null function pointer")(
            match_info, result, user_data,
        );
        str_pos = *(*match_info)
            .offsets
            .offset(1 as ::core::ffi::c_int as isize);
        safe_c2rust_g_match_info_next(match_info, &raw mut tmp_error);
    }
    safe_c2rust_g_match_info_free(match_info);
    if !tmp_error.is_null() {
        g_propagate_error(error, tmp_error);
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(result, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(result);
            };
        } else {
            g_string_free(result, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return ::core::ptr::null_mut::<gchar>();
    }
    safe_c2rust_g_string_append_len_inline(
        result,
        string.offset(str_pos as isize),
        string_len - str_pos as gssize,
    );
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(result, 0 as gboolean)
        } else {
            g_string_free_and_steal(result)
        }
    } else {
        g_string_free(result, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_check_replacement(
    mut replacement: *const gchar,
    mut has_references: *mut gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp: *mut GError = ::core::ptr::null_mut::<GError>();
    list = safe_c2rust_split_replacement(replacement, &raw mut tmp);
    if !tmp.is_null() {
        g_propagate_error(error, tmp);
        return FALSE;
    }
    if !has_references.is_null() {
        *has_references = safe_c2rust_interpolation_list_needs_match(list);
    }
    g_list_free_full(
        list,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut InterpolationData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_free_interpolation_data
                as unsafe extern "C" fn(*mut InterpolationData) -> (),
        )),
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_regex_escape_nul(
    mut string: *const gchar,
    mut length: gint,
) -> *mut gchar {
    let mut escaped: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut piece_start: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    let mut backslashes: gint = 0;
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if length < 0 as ::core::ffi::c_int {
        return safe_c2rust_g_strdup_inline(string as *const ::core::ffi::c_char) as *mut gchar;
    }
    end = string.offset(length as isize);
    piece_start = string;
    p = piece_start;
    escaped = g_string_sized_new((length as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize);
    backslashes = 0 as ::core::ffi::c_int as gint;
    while p < end {
        match *p as ::core::ffi::c_int {
            0 => {
                if p != piece_start {
                    safe_c2rust_g_string_append_len_inline(
                        escaped,
                        piece_start as *const ::core::ffi::c_char,
                        p.offset_from(piece_start) as gssize,
                    );
                }
                if backslashes as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    safe_c2rust_g_string_append_c_inline(escaped, '\\' as i32 as gchar);
                }
                safe_c2rust_g_string_append_c_inline(escaped, 'x' as i32 as gchar);
                safe_c2rust_g_string_append_c_inline(escaped, '0' as i32 as gchar);
                safe_c2rust_g_string_append_c_inline(escaped, '0' as i32 as gchar);
                p = p.offset(1);
                piece_start = p;
                backslashes = 0 as ::core::ffi::c_int as gint;
            }
            92 => {
                backslashes += 1;
                p = p.offset(1);
            }
            _ => {
                backslashes = 0 as ::core::ffi::c_int as gint;
                p = p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
        }
    }
    if piece_start < end {
        safe_c2rust_g_string_append_len_inline(
            escaped,
            piece_start as *const ::core::ffi::c_char,
            end.offset_from(piece_start) as gssize,
        );
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
pub unsafe extern "C" fn safe_c2rust_g_regex_escape_string(
    mut string: *const gchar,
    mut length: gint,
) -> *mut gchar {
    let mut escaped: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut piece_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if length < 0 as ::core::ffi::c_int {
        length = strlen(string as *const ::core::ffi::c_char) as gint;
    }
    end = string.offset(length as isize) as *const ::core::ffi::c_char;
    piece_start = string as *const ::core::ffi::c_char;
    p = piece_start;
    escaped = g_string_sized_new((length as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize);
    while p < end {
        match *p as ::core::ffi::c_int {
            0 | 92 | 124 | 40 | 41 | 91 | 93 | 123 | 125 | 94 | 36 | 42 | 43 | 63 | 46 => {
                if p != piece_start {
                    safe_c2rust_g_string_append_len_inline(
                        escaped,
                        piece_start,
                        p.offset_from(piece_start) as gssize,
                    );
                }
                safe_c2rust_g_string_append_c_inline(escaped, '\\' as i32 as gchar);
                if *p as ::core::ffi::c_int == '\0' as i32 {
                    safe_c2rust_g_string_append_c_inline(escaped, '0' as i32 as gchar);
                } else {
                    safe_c2rust_g_string_append_c_inline(escaped, *p);
                }
                p = p.offset(1);
                piece_start = p;
            }
            _ => {
                p = p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
        }
    }
    if piece_start < end {
        safe_c2rust_g_string_append_len_inline(
            escaped,
            piece_start,
            end.offset_from(piece_start) as gssize,
        );
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
