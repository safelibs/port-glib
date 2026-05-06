use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    pub type __locale_data;
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GMarkupParseContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GRegex;
    pub type _GWakeup;
    pub type _GvdbTable;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_get_user_data_dir() -> *const gchar;
    fn g_get_system_data_dirs() -> *const *const gchar;
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_dir_open(path: *const gchar, flags: guint, error: *mut *mut GError) -> *mut GDir;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_dgettext(domain: *const gchar, msgid: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_steal_all(hash_table: *mut GHashTable);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_get_keys_as_array(
        hash_table: *mut GHashTable,
        length: *mut guint,
    ) -> *mut gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_iter_remove(iter: *mut GHashTableIter);
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
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
    fn g_markup_parse_context_new(
        parser: *const GMarkupParser,
        flags: GMarkupParseFlags,
        user_data: gpointer,
        user_data_dnotify: GDestroyNotify,
    ) -> *mut GMarkupParseContext;
    fn g_markup_parse_context_free(context: *mut GMarkupParseContext);
    fn g_markup_parse_context_parse(
        context: *mut GMarkupParseContext,
        text: *const gchar,
        text_len: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_markup_parse_context_end_parse(
        context: *mut GMarkupParseContext,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_is_container(value: *mut GVariant) -> gboolean;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_new_array(
        child_type: *const GVariantType,
        children: *const *mut GVariant,
        n_children: gsize,
    ) -> *mut GVariant;
    fn g_variant_lookup_value(
        dictionary: *mut GVariant,
        key: *const gchar,
        expected_type: *const GVariantType,
    ) -> *mut GVariant;
    fn g_variant_get_fixed_array(
        value: *mut GVariant,
        n_elements: *mut gsize,
        element_size: gsize,
    ) -> gconstpointer;
    fn g_variant_iter_new(value: *mut GVariant) -> *mut GVariantIter;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next_value(iter: *mut GVariantIter) -> *mut GVariant;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_variant_parse(
        type_0: *const GVariantType,
        text: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_variant_compare(one: gconstpointer, two: gconstpointer) -> gint;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_regex_new(
        pattern: *const gchar,
        compile_options: GRegexCompileFlags,
        match_options: GRegexMatchFlags,
        error: *mut *mut GError,
    ) -> *mut GRegex;
    fn g_regex_split(
        regex: *const GRegex,
        string: *const gchar,
        match_options: GRegexMatchFlags,
    ) -> *mut *mut gchar;
    fn g_regex_replace_literal(
        regex: *const GRegex,
        string: *const gchar,
        string_len: gssize,
        start_position: gint,
        replacement: *const gchar,
        match_options: GRegexMatchFlags,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn gvdb_table_new(
        filename: *const gchar,
        trusted: gboolean,
        error: *mut *mut GError,
    ) -> *mut GvdbTable;
    fn gvdb_table_free(table: *mut GvdbTable);
    fn gvdb_table_get_raw_value(table: *mut GvdbTable, key: *const gchar) -> *mut GVariant;
    fn gvdb_table_list(table: *mut GvdbTable, key: *const gchar) -> *mut *mut gchar;
    fn gvdb_table_get_table(table: *mut GvdbTable, key: *const gchar) -> *mut GvdbTable;
    fn gvdb_table_has_value(table: *mut GvdbTable, key: *const gchar) -> gboolean;
    fn bind_textdomain_codeset(
        __domainname: *const ::core::ffi::c_char,
        __codeset: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn newlocale(
        __category_mask: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn freelocale(__dataset: locale_t);
    fn uselocale(__dataset: locale_t) -> locale_t;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const ::core::ffi::c_ushort,
    pub __ctype_tolower: *const ::core::ffi::c_int,
    pub __ctype_toupper: *const ::core::ffi::c_int,
    pub __names: [*const ::core::ffi::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GData = _GData;
pub type GDir = _GDir;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GMarkupParseFlags = ::core::ffi::c_uint;
pub const G_MARKUP_IGNORE_QUALIFIED: GMarkupParseFlags = 8;
pub const G_MARKUP_PREFIX_ERROR_POSITION: GMarkupParseFlags = 4;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: GMarkupParseFlags = 2;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: GMarkupParseFlags = 1;
pub const G_MARKUP_DEFAULT_FLAGS: GMarkupParseFlags = 0;
pub type GMarkupParseContext = _GMarkupParseContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMarkupParser {
    pub start_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            *mut *const gchar,
            *mut *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub end_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub text: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub passthrough: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub error: Option<unsafe extern "C" fn(*mut GMarkupParseContext, *mut GError, gpointer) -> ()>,
}
pub type GMarkupParser = _GMarkupParser;
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
pub type GRegex = _GRegex;
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
pub type GType = gsize;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsSchemaSource {
    pub parent: *mut GSettingsSchemaSource,
    pub directory: *mut gchar,
    pub table: *mut GvdbTable,
    pub text_tables: *mut *mut GHashTable,
    pub ref_count: gint,
}
pub type GvdbTable = _GvdbTable;
pub type GSettingsSchemaSource = _GSettingsSchemaSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsSchema {
    pub source: *mut GSettingsSchemaSource,
    pub gettext_domain: *const gchar,
    pub path: *const gchar,
    pub items: *mut GQuark,
    pub n_items: gint,
    pub table: *mut GvdbTable,
    pub id: *mut gchar,
    pub extends: *mut GSettingsSchema,
    pub ref_count: gint,
}
pub type GSettingsSchema = _GSettingsSchema;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GSettingsSchemaKey {
    pub schema: *mut GSettingsSchema,
    pub name: *const gchar,
    #[bitfield(name = "is_flags", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_enum", ty = "guint", bits = "1..=1")]
    pub is_flags_is_enum: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub strinfo: *const guint32,
    pub strinfo_length: gsize,
    pub unparsed: *const gchar,
    pub lc_char: gchar,
    pub type_0: *const GVariantType,
    pub minimum: *mut GVariant,
    pub maximum: *mut GVariant,
    pub default_value: *mut GVariant,
    pub desktop_overrides: *mut GVariant,
    pub ref_count: gint,
}
pub type GSettingsSchemaKey = _GSettingsSchemaKey;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GSettingsSchemaSource) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GSettingsSchemaSource) -> *mut GSettingsSchemaSource>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GSettingsSchemaSource) -> *mut GSettingsSchemaSource>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GSettingsSchema) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GSettingsSchema) -> *mut GSettingsSchema>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GSettingsSchema) -> *mut GSettingsSchema>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GSettingsSchemaKey) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GSettingsSchemaKey) -> *mut GSettingsSchemaKey>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GSettingsSchemaKey) -> *mut GSettingsSchemaKey>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TextTableParseInfo {
    pub summaries: *mut GHashTable,
    pub descriptions: *mut GHashTable,
    pub gettext_domain: *mut GSList,
    pub schema_id: *mut GSList,
    pub key_name: *mut GSList,
    pub string: *mut GString,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_SEARCHPATH_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b":\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
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
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const STRINFO_MAX_WORDS: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_strinfo_string_to_words(
    mut string: *const gchar,
    mut words: *mut guint32,
    mut alias: gboolean,
) -> guint {
    let mut n_words: guint = 0;
    let mut size: gsize = 0;
    size = strlen(string as *const ::core::ffi::c_char) as gsize;
    n_words = (if 2 as gsize > size.wrapping_add(6 as gsize) >> 2 as ::core::ffi::c_int {
        2 as gsize
    } else {
        size.wrapping_add(6 as gsize) >> 2 as ::core::ffi::c_int
    }) as guint;
    if n_words > STRINFO_MAX_WORDS as guint {
        return FALSE as guint;
    }
    *words.offset(0 as ::core::ffi::c_int as isize) = (if alias != 0 {
        0xfe as ::core::ffi::c_int
    } else {
        0xff as ::core::ffi::c_int
    }) as guint32;
    *words.offset(n_words.wrapping_sub(1 as guint) as isize) = ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = 0xff as ::core::ffi::c_int as guint32;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh8 = &mut __v;
            let fresh9;
            let fresh10 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh8,
                fresh10) => fresh9, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
        }
        __v
    });
    memcpy(
        (words as *mut gchar).offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        (size as size_t).wrapping_add(1 as size_t),
    );
    return n_words;
}
unsafe extern "C" fn safe_c2rust_strinfo_scan(
    mut strinfo: *const guint32,
    mut length: guint,
    mut words: *const guint32,
    mut n_words: guint,
) -> gint {
    let mut i: guint = 0 as guint;
    if length < n_words {
        return -(1 as gint);
    }
    while i <= length.wrapping_sub(n_words) {
        let mut j: guint = 0 as guint;
        j = 0 as guint;
        while j < n_words {
            if *strinfo.offset(i.wrapping_add(j) as isize) != *words.offset(j as isize) {
                break;
            }
            j = j.wrapping_add(1);
        }
        if j == n_words {
            return i as gint;
        }
        i = i.wrapping_add(if j != 0 { j } else { 1 as guint });
    }
    return -(1 as gint);
}
unsafe extern "C" fn safe_c2rust_strinfo_find_string(
    mut strinfo: *const guint32,
    mut length: guint,
    mut string: *const gchar,
    mut alias: gboolean,
) -> gint {
    let mut words: [guint32; 17] = [0; 17];
    let mut n_words: guint = 0;
    if length == 0 as guint {
        return -(1 as gint);
    }
    n_words = safe_c2rust_strinfo_string_to_words(string, &raw mut words as *mut guint32, alias);
    return safe_c2rust_strinfo_scan(
        strinfo.offset(1 as ::core::ffi::c_int as isize),
        length.wrapping_sub(1 as guint),
        &raw mut words as *mut guint32,
        n_words,
    );
}
unsafe extern "C" fn safe_c2rust_strinfo_find_integer(
    mut strinfo: *const guint32,
    mut length: guint,
    mut value: guint32,
) -> gint {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < length {
        if *strinfo.offset(i as isize) == value {
            let mut charinfo: *const guchar =
                strinfo.offset(i as isize) as *const guint32 as *const guchar;
            if (i == 0 as guint
                || *charinfo.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0xff as ::core::ffi::c_int)
                && *charinfo.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0xff as ::core::ffi::c_int
            {
                return i as gint;
            }
        }
        i = i.wrapping_add(1);
    }
    return -(1 as gint);
}
unsafe extern "C" fn safe_c2rust_strinfo_is_string_valid(
    mut strinfo: *const guint32,
    mut length: guint,
    mut string: *const gchar,
) -> gboolean {
    return (safe_c2rust_strinfo_find_string(strinfo, length, string, FALSE)
        != -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_strinfo_enum_from_string(
    mut strinfo: *const guint32,
    mut length: guint,
    mut string: *const gchar,
    mut result: *mut guint,
) -> gboolean {
    let mut index: gint = 0;
    index = safe_c2rust_strinfo_find_string(strinfo, length, string, FALSE);
    if index < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    *result = *strinfo.offset(index as isize) as guint;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_strinfo_string_from_enum(
    mut strinfo: *const guint32,
    mut length: guint,
    mut value: guint,
) -> *const gchar {
    let mut index: gint = 0;
    index = safe_c2rust_strinfo_find_integer(strinfo, length, value as guint32);
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<gchar>();
    }
    return (strinfo.offset((index as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *const guint32 as *const gchar)
        .offset(1 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn safe_c2rust_strinfo_string_from_alias(
    mut strinfo: *const guint32,
    mut length: guint,
    mut alias: *const gchar,
) -> *const gchar {
    let mut index: gint = 0;
    index = safe_c2rust_strinfo_find_string(strinfo, length, alias, TRUE);
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<gchar>();
    }
    return (strinfo.offset((*strinfo.offset(index as isize)).wrapping_add(1 as guint32) as isize)
        as *const guint32 as *const gchar)
        .offset(1 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn safe_c2rust_strinfo_enumerate(
    mut strinfo: *const guint32,
    mut length: guint,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut ptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    ptr = strinfo as gpointer as *const gchar;
    end = ptr.offset((4 as guint).wrapping_mul(length) as isize);
    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_STRING_ARRAY);
    while ptr < end {
        if *ptr as ::core::ffi::c_int == -1i32 {
            g_variant_builder_add(
                &raw mut builder,
                b"s\0" as *const u8 as *const gchar,
                ptr.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        ptr = memchr(
            ptr as *const ::core::ffi::c_void,
            -1i32,
            end.offset_from(ptr) as ::core::ffi::c_long as size_t,
        ) as *const gchar;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if !ptr.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/strinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                285 as ::core::ffi::c_int,
                G_STRFUNC,
                b"ptr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ptr = ptr.offset(5 as ::core::ffi::c_int as isize);
    }
    return g_variant_builder_end(&raw mut builder);
}
pub const LC_TIME: ::core::ffi::c_int = __LC_TIME;
pub const LC_MESSAGES_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_MESSAGES;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_schema_source_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_settings_schema_source_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GSettingsSchemaSource\0" as *const u8 as *const gchar),
        C2RustUnnamed_2 {
            do_copy_type: Some(
                safe_c2rust_g_settings_schema_source_ref
                    as unsafe extern "C" fn(
                        *mut GSettingsSchemaSource,
                    ) -> *mut GSettingsSchemaSource,
            ),
        },
        C2RustUnnamed_1 {
            do_free_type: Some(
                safe_c2rust_g_settings_schema_source_unref
                    as unsafe extern "C" fn(*mut GSettingsSchemaSource) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_settings_schema_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_4, C2RustUnnamed_3) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_4, C2RustUnnamed_3) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GSettingsSchema\0" as *const u8 as *const gchar),
        C2RustUnnamed_4 {
            do_copy_type: Some(
                safe_c2rust_g_settings_schema_ref
                    as unsafe extern "C" fn(*mut GSettingsSchema) -> *mut GSettingsSchema,
            ),
        },
        C2RustUnnamed_3 {
            do_free_type: Some(
                safe_c2rust_g_settings_schema_unref
                    as unsafe extern "C" fn(*mut GSettingsSchema) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_schema_get_type_once();
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
static mut safe_c2rust_schema_sources: *mut GSettingsSchemaSource =
    ::core::ptr::null::<GSettingsSchemaSource>() as *mut GSettingsSchemaSource;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_ref(
    mut source: *mut GSettingsSchemaSource,
) -> *mut GSettingsSchemaSource {
    if 0 as ::core::ffi::c_int != 0 {
        (*source).ref_count;
        (*source).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*source).ref_count, 1 as ::core::ffi::c_int);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_unref(
    mut source: *mut GSettingsSchemaSource,
) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*source).ref_count;
            (*source).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*source).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if source == safe_c2rust_schema_sources {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"g_settings_schema_source_unref() called too many times on the default schema source\0"
                    as *const u8 as *const gchar,
            );
            loop {}
        }
        if !(*source).parent.is_null() {
            safe_c2rust_g_settings_schema_source_unref((*source).parent);
        }
        gvdb_table_free((*source).table);
        g_free((*source).directory as gpointer);
        if !(*source).text_tables.is_null() {
            g_hash_table_unref(
                *(*source)
                    .text_tables
                    .offset(0 as ::core::ffi::c_int as isize),
            );
            g_hash_table_unref(
                *(*source)
                    .text_tables
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            g_free((*source).text_tables as gpointer);
        }
        g_slice_free1(
            ::core::mem::size_of::<GSettingsSchemaSource>() as gsize,
            source as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_new_from_directory(
    mut directory: *const gchar,
    mut parent: *mut GSettingsSchemaSource,
    mut trusted: gboolean,
    mut error: *mut *mut GError,
) -> *mut GSettingsSchemaSource {
    let mut source: *mut GSettingsSchemaSource = ::core::ptr::null_mut::<GSettingsSchemaSource>();
    let mut table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
    let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    filename = g_build_filename(
        directory,
        b"gschemas.compiled\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    table = gvdb_table_new(filename, trusted, error);
    g_free(filename as gpointer);
    if table.is_null() {
        return ::core::ptr::null_mut::<GSettingsSchemaSource>();
    }
    source = g_slice_alloc(::core::mem::size_of::<GSettingsSchemaSource>() as gsize)
        as *mut GSettingsSchemaSource;
    (*source).directory =
        safe_c2rust_g_strdup_inline(directory as *const ::core::ffi::c_char) as *mut gchar;
    (*source).parent = if !parent.is_null() {
        safe_c2rust_g_settings_schema_source_ref(parent)
    } else {
        ::core::ptr::null_mut::<GSettingsSchemaSource>()
    };
    (*source).text_tables = ::core::ptr::null_mut::<*mut GHashTable>();
    (*source).table = table;
    (*source).ref_count = 1 as ::core::ffi::c_int as gint;
    return source;
}
unsafe extern "C" fn safe_c2rust_try_prepend_dir(mut directory: *const gchar) {
    let mut source: *mut GSettingsSchemaSource = ::core::ptr::null_mut::<GSettingsSchemaSource>();
    source = safe_c2rust_g_settings_schema_source_new_from_directory(
        directory,
        safe_c2rust_schema_sources,
        TRUE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !source.is_null() {
        safe_c2rust_schema_sources = source;
    }
}
unsafe extern "C" fn safe_c2rust_try_prepend_data_dir(mut directory: *const gchar) {
    let mut dirname: *mut gchar = g_build_filename(
        directory,
        b"glib-2.0\0" as *const u8 as *const ::core::ffi::c_char,
        b"schemas\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    safe_c2rust_try_prepend_dir(dirname);
    g_free(dirname as gpointer);
}
unsafe extern "C" fn safe_c2rust_initialise_schema_sources() {
    static mut safe_c2rust_initialised: gsize = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_initialised;
            } else {
            };
            (({
                let mut gapg_temp_newval: gsize = 0;
                let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialised;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) == 0
                && g_once_init_enter(&raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void)
                    != 0) as ::core::ffi::c_int
        }) != 0
        {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        let mut is_setuid: gboolean = (*glib__private__())
            .g_check_setuid
            .expect("non-null function pointer")();
        let mut dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
        let mut path: *const gchar = ::core::ptr::null::<gchar>();
        let mut extra_schema_dirs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut i: gint = 0;
        dirs = g_get_system_data_dirs();
        i = 0 as ::core::ffi::c_int as gint;
        while !(*dirs.offset(i as isize)).is_null() {
            i += 1;
        }
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            safe_c2rust_try_prepend_data_dir(*dirs.offset(i as isize));
        }
        safe_c2rust_try_prepend_data_dir(g_get_user_data_dir());
        if is_setuid == 0 && {
            path = g_getenv(b"GSETTINGS_SCHEMA_DIR\0" as *const u8 as *const gchar);
            !path.is_null()
        } {
            extra_schema_dirs = g_strsplit(
                path,
                G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
                0 as gint,
            );
            i = 0 as ::core::ffi::c_int as gint;
            while !(*extra_schema_dirs.offset(i as isize)).is_null() {
                i += 1;
            }
            loop {
                let fresh1 = i;
                i = i - 1;
                if !(fresh1 != 0) {
                    break;
                }
                safe_c2rust_try_prepend_dir(*extra_schema_dirs.offset(i as isize));
            }
            g_strfreev(extra_schema_dirs);
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_get_default(
) -> *mut GSettingsSchemaSource {
    safe_c2rust_initialise_schema_sources();
    return safe_c2rust_schema_sources;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_lookup(
    mut source: *mut GSettingsSchemaSource,
    mut schema_id: *const gchar,
    mut recursive: gboolean,
) -> *mut GSettingsSchema {
    let mut schema: *mut GSettingsSchema = ::core::ptr::null_mut::<GSettingsSchema>();
    let mut table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
    let mut extends: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsSchema>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !schema_id.is_null() {
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
            b"schema_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsSchema>();
    }
    table = gvdb_table_get_table((*source).table, schema_id);
    if table.is_null() && recursive != 0 {
        source = (*source).parent;
        while !source.is_null() {
            table = gvdb_table_get_table((*source).table, schema_id);
            if !table.is_null() {
                break;
            }
            source = (*source).parent;
        }
    }
    if table.is_null() {
        return ::core::ptr::null_mut::<GSettingsSchema>();
    }
    schema = ({
        let mut __s: gsize = ::core::mem::size_of::<GSettingsSchema>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSettingsSchema;
    (*schema).source = safe_c2rust_g_settings_schema_source_ref(source);
    (*schema).ref_count = 1 as ::core::ffi::c_int as gint;
    (*schema).id =
        safe_c2rust_g_strdup_inline(schema_id as *const ::core::ffi::c_char) as *mut gchar;
    (*schema).table = table;
    (*schema).path =
        safe_c2rust_g_settings_schema_get_string(schema, b".path\0" as *const u8 as *const gchar);
    (*schema).gettext_domain = safe_c2rust_g_settings_schema_get_string(
        schema,
        b".gettext-domain\0" as *const u8 as *const gchar,
    );
    if !(*schema).gettext_domain.is_null() {
        bind_textdomain_codeset(
            (*schema).gettext_domain as *const ::core::ffi::c_char,
            b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    extends = safe_c2rust_g_settings_schema_get_string(
        schema,
        b".extends\0" as *const u8 as *const gchar,
    );
    if !extends.is_null() {
        (*schema).extends = safe_c2rust_g_settings_schema_source_lookup(source, extends, TRUE);
        if (*schema).extends.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Schema '%s' extends schema '%s' but we could not find it\0" as *const u8
                    as *const gchar,
                schema_id,
                extends,
            );
        }
    }
    return schema;
}
unsafe extern "C" fn safe_c2rust_get_attribute_value(mut list: *mut GSList) -> *const gchar {
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    node = list;
    while !node.is_null() {
        if !(*node).data.is_null() {
            return (*node).data as *const gchar;
        }
        node = (*node).next;
    }
    return ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn safe_c2rust_pop_attribute_value(mut list: *mut *mut GSList) {
    let mut top: *mut gchar = ::core::ptr::null_mut::<gchar>();
    top = (**list).data as *mut gchar;
    *list = g_slist_remove(*list, top as gconstpointer);
    g_free(top as gpointer);
}
unsafe extern "C" fn safe_c2rust_push_attribute_value(
    mut list: *mut *mut GSList,
    mut value: *const gchar,
) {
    *list = g_slist_prepend(
        *list,
        safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_start_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut info: *mut TextTableParseInfo = user_data as *mut TextTableParseInfo;
    let mut gettext_domain: *const gchar = ::core::ptr::null::<gchar>();
    let mut schema_id: *const gchar = ::core::ptr::null::<gchar>();
    let mut key_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*attribute_names.offset(i as isize)).is_null() {
        if strcmp(
            *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
            b"gettext-domain\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            gettext_domain = *attribute_values.offset(i as isize);
        } else if strcmp(
            *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
            b"id\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            schema_id = *attribute_values.offset(i as isize);
        } else if strcmp(
            *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            key_name = *attribute_values.offset(i as isize);
        }
        i += 1;
    }
    safe_c2rust_push_attribute_value(&raw mut (*info).gettext_domain, gettext_domain);
    safe_c2rust_push_attribute_value(&raw mut (*info).schema_id, schema_id);
    safe_c2rust_push_attribute_value(&raw mut (*info).key_name, key_name);
    if !(*info).string.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    (*info).string,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal((*info).string);
            };
        } else {
            g_string_free(
                (*info).string,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        (*info).string = ::core::ptr::null_mut::<GString>();
    }
    if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"summary\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            element_name as *const ::core::ffi::c_char,
            b"description\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        (*info).string = g_string_new(::core::ptr::null::<gchar>());
    }
}
unsafe extern "C" fn safe_c2rust_normalise_whitespace(mut orig: *const gchar) -> *mut gchar {
    static mut safe_c2rust_cleanup: [*mut GRegex; 3] =
        [::core::ptr::null::<GRegex>() as *mut GRegex; 3];
    static mut safe_c2rust_splitter: *mut GRegex = ::core::ptr::null::<GRegex>() as *mut GRegex;
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gint = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_splitter;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GRegex = ::core::ptr::null_mut::<GRegex>();
            let mut gapg_temp_atomic: *mut *mut GRegex = &raw mut safe_c2rust_splitter;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_splitter as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut s: *mut GRegex = ::core::ptr::null_mut::<GRegex>();
        safe_c2rust_cleanup[0 as ::core::ffi::c_int as usize] = g_regex_new(
            b"^\\s+\0" as *const u8 as *const gchar,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_cleanup[1 as ::core::ffi::c_int as usize] = g_regex_new(
            b"\\s+$\0" as *const u8 as *const gchar,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_cleanup[2 as ::core::ffi::c_int as usize] = g_regex_new(
            b"\\s+\0" as *const u8 as *const gchar,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        s = g_regex_new(
            b"\\n\\s*\\n+\0" as *const u8 as *const gchar,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_splitter = s;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_splitter as *mut ::core::ffi::c_void,
            s as guintptr as gpointer,
        );
    }
    lines = g_regex_split(safe_c2rust_splitter, orig, G_REGEX_MATCH_DEFAULT);
    i = 0 as ::core::ffi::c_int as gint;
    while !(*lines.offset(i as isize)).is_null() {
        let mut a: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut b: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut c: *mut gchar = ::core::ptr::null_mut::<gchar>();
        a = g_regex_replace_literal(
            safe_c2rust_cleanup[0 as ::core::ffi::c_int as usize],
            *lines.offset(i as isize),
            -(1 as ::core::ffi::c_int) as gssize,
            0 as gint,
            b"\0" as *const u8 as *const gchar,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        b = g_regex_replace_literal(
            safe_c2rust_cleanup[1 as ::core::ffi::c_int as usize],
            a,
            -(1 as ::core::ffi::c_int) as gssize,
            0 as gint,
            b"\0" as *const u8 as *const gchar,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        c = g_regex_replace_literal(
            safe_c2rust_cleanup[2 as ::core::ffi::c_int as usize],
            b,
            -(1 as ::core::ffi::c_int) as gssize,
            0 as gint,
            b" \0" as *const u8 as *const gchar,
            G_REGEX_MATCH_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_free(*lines.offset(i as isize) as gpointer);
        g_free(a as gpointer);
        g_free(b as gpointer);
        let ref mut fresh13 = *lines.offset(i as isize);
        *fresh13 = c;
        i += 1;
    }
    result = g_strjoinv(b"\n\n\0" as *const u8 as *const gchar, lines);
    g_strfreev(lines);
    return result;
}
unsafe extern "C" fn safe_c2rust_end_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut info: *mut TextTableParseInfo = user_data as *mut TextTableParseInfo;
    safe_c2rust_pop_attribute_value(&raw mut (*info).gettext_domain);
    safe_c2rust_pop_attribute_value(&raw mut (*info).schema_id);
    safe_c2rust_pop_attribute_value(&raw mut (*info).key_name);
    if !(*info).string.is_null() {
        let mut source_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut gettext_domain: *const gchar = ::core::ptr::null::<gchar>();
        let mut schema_id: *const gchar = ::core::ptr::null::<gchar>();
        let mut key_name: *const gchar = ::core::ptr::null::<gchar>();
        gettext_domain = safe_c2rust_get_attribute_value((*info).gettext_domain);
        schema_id = safe_c2rust_get_attribute_value((*info).schema_id);
        key_name = safe_c2rust_get_attribute_value((*info).key_name);
        if strcmp(
            element_name as *const ::core::ffi::c_char,
            b"summary\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            source_table = (*info).summaries;
        } else if strcmp(
            element_name as *const ::core::ffi::c_char,
            b"description\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            source_table = (*info).descriptions;
        }
        if !source_table.is_null() && !schema_id.is_null() && !key_name.is_null() {
            let mut schema_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
            let mut normalised: *mut gchar = ::core::ptr::null_mut::<gchar>();
            schema_table =
                g_hash_table_lookup(source_table, schema_id as gconstpointer) as *mut GHashTable;
            if schema_table.is_null() {
                schema_table = g_hash_table_new_full(
                    Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                    Some(
                        g_str_equal
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                    ),
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                );
                g_hash_table_insert(
                    source_table,
                    safe_c2rust_g_strdup_inline(schema_id as *const ::core::ffi::c_char)
                        as gpointer,
                    schema_table as gpointer,
                );
            }
            normalised = safe_c2rust_normalise_whitespace((*(*info).string).str_0);
            if !gettext_domain.is_null()
                && *normalised.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            {
                let mut translated: *mut gchar = ::core::ptr::null_mut::<gchar>();
                translated = safe_c2rust_g_strdup_inline(
                    g_dgettext(gettext_domain, normalised) as *const ::core::ffi::c_char
                ) as *mut gchar;
                g_free(normalised as gpointer);
                normalised = translated;
            }
            g_hash_table_insert(
                schema_table,
                safe_c2rust_g_strdup_inline(key_name as *const ::core::ffi::c_char) as gpointer,
                normalised as gpointer,
            );
        }
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    (*info).string,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal((*info).string);
            };
        } else {
            g_string_free(
                (*info).string,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        (*info).string = ::core::ptr::null_mut::<GString>();
    }
}
unsafe extern "C" fn safe_c2rust_text(
    mut context: *mut GMarkupParseContext,
    mut text: *const gchar,
    mut text_len: gsize,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut info: *mut TextTableParseInfo = user_data as *mut TextTableParseInfo;
    if !(*info).string.is_null() {
        safe_c2rust_g_string_append_len_inline(
            (*info).string,
            text as *const ::core::ffi::c_char,
            text_len as gssize,
        );
    }
}
unsafe extern "C" fn safe_c2rust_parse_into_text_tables(
    mut directory: *const gchar,
    mut summaries: *mut GHashTable,
    mut descriptions: *mut GHashTable,
) {
    let mut parser: GMarkupParser = _GMarkupParser {
        start_element: Some(
            safe_c2rust_start_element
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    *mut *const gchar,
                    *mut *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        end_element: Some(
            safe_c2rust_end_element
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        text: Some(
            safe_c2rust_text
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gsize,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        passthrough: None,
        error: None,
    };
    let mut info: TextTableParseInfo = TextTableParseInfo {
        summaries: summaries,
        descriptions: descriptions,
        gettext_domain: ::core::ptr::null_mut::<GSList>(),
        schema_id: ::core::ptr::null_mut::<GSList>(),
        key_name: ::core::ptr::null_mut::<GSList>(),
        string: ::core::ptr::null_mut::<GString>(),
    };
    let mut basename: *const gchar = ::core::ptr::null::<gchar>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    dir = g_dir_open(
        directory,
        0 as guint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    loop {
        basename = g_dir_read_name(dir);
        if basename.is_null() {
            break;
        }
        let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut size: gsize = 0;
        filename = g_build_filename(directory, basename, NULL_1);
        if g_file_get_contents(
            filename,
            &raw mut contents,
            &raw mut size,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != 0
        {
            let mut context: *mut GMarkupParseContext =
                ::core::ptr::null_mut::<GMarkupParseContext>();
            context = g_markup_parse_context_new(
                &raw mut parser,
                G_MARKUP_TREAT_CDATA_AS_TEXT,
                &raw mut info as gpointer,
                None,
            );
            if g_markup_parse_context_parse(
                context,
                contents,
                size as gssize,
                ::core::ptr::null_mut::<*mut GError>(),
            ) != 0
            {
                g_markup_parse_context_end_parse(context, ::core::ptr::null_mut::<*mut GError>());
            }
            g_markup_parse_context_free(context);
            g_slist_free_full(
                info.gettext_domain,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            g_slist_free_full(
                info.schema_id,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            g_slist_free_full(
                info.key_name,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            info.gettext_domain = ::core::ptr::null_mut::<GSList>();
            info.schema_id = ::core::ptr::null_mut::<GSList>();
            info.key_name = ::core::ptr::null_mut::<GSList>();
            if !info.string.is_null() {
                if 0 != 0 {
                    if 0 as ::core::ffi::c_int == 0 {
                        g_string_free(
                            info.string,
                            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                        );
                    } else {
                        g_string_free_and_steal(info.string);
                    };
                } else {
                    g_string_free(
                        info.string,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                };
                info.string = ::core::ptr::null_mut::<GString>();
            }
            g_free(contents as gpointer);
        }
        g_free(filename as gpointer);
    }
    g_dir_close(dir);
}
unsafe extern "C" fn safe_c2rust_g_settings_schema_source_get_text_tables(
    mut source: *mut GSettingsSchemaSource,
) -> *mut *mut GHashTable {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*source).text_tables;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut *mut GHashTable =
                ::core::ptr::null_mut::<*mut GHashTable>();
            let mut gapg_temp_atomic: *mut *mut *mut GHashTable = &raw mut (*source).text_tables;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut (*source).text_tables as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut text_tables: *mut *mut GHashTable = ::core::ptr::null_mut::<*mut GHashTable>();
        text_tables = ({
            let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut GHashTable>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut GHashTable;
        let ref mut fresh11 = *text_tables.offset(0 as ::core::ffi::c_int as isize);
        *fresh11 = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GHashTable) -> ()>,
                GDestroyNotify,
            >(Some(
                g_hash_table_unref as unsafe extern "C" fn(*mut GHashTable) -> (),
            )),
        );
        let ref mut fresh12 = *text_tables.offset(1 as ::core::ffi::c_int as isize);
        *fresh12 = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GHashTable) -> ()>,
                GDestroyNotify,
            >(Some(
                g_hash_table_unref as unsafe extern "C" fn(*mut GHashTable) -> (),
            )),
        );
        if !(*source).directory.is_null() {
            safe_c2rust_parse_into_text_tables(
                (*source).directory,
                *text_tables.offset(0 as ::core::ffi::c_int as isize),
                *text_tables.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        if 0 as ::core::ffi::c_int != 0 {
            (*source).text_tables = text_tables;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut (*source).text_tables as *mut ::core::ffi::c_void,
            text_tables as guintptr as gpointer,
        );
    }
    return (*source).text_tables;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_source_list_schemas(
    mut source: *mut GSettingsSchemaSource,
    mut recursive: gboolean,
    mut non_relocatable: *mut *mut *mut gchar,
    mut relocatable: *mut *mut *mut gchar,
) {
    let mut single: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut reloc: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut s: *mut GSettingsSchemaSource = ::core::ptr::null_mut::<GSettingsSchemaSource>();
    single = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    reloc = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    s = source;
    while !s.is_null() {
        let mut list: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut i: gint = 0;
        list = gvdb_table_list((*s).table, b"\0" as *const u8 as *const gchar);
        if !list.is_null() {
            i = 0 as ::core::ffi::c_int as gint;
            while !(*list.offset(i as isize)).is_null() {
                if g_hash_table_contains(single, *list.offset(i as isize) as gconstpointer) == 0
                    && g_hash_table_contains(reloc, *list.offset(i as isize) as gconstpointer) == 0
                {
                    let mut schema: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    let mut table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
                    schema = safe_c2rust_g_strdup_inline(*list.offset(i as isize)) as *mut gchar;
                    table = gvdb_table_get_table((*s).table, *list.offset(i as isize));
                    if ({
                        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                        if !table.is_null() {
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
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettingsschema.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            815 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if gvdb_table_has_value(table, b".path\0" as *const u8 as *const gchar) != 0 {
                        g_hash_table_add(single, schema as gpointer);
                    } else {
                        g_hash_table_add(reloc, schema as gpointer);
                    }
                    gvdb_table_free(table);
                }
                i += 1;
            }
            g_strfreev(list);
            if recursive == 0 {
                break;
            }
        }
        s = (*s).parent;
    }
    if !non_relocatable.is_null() {
        *non_relocatable = g_hash_table_get_keys_as_array(single, ::core::ptr::null_mut::<guint>())
            as *mut *mut gchar;
        g_hash_table_steal_all(single);
    }
    if !relocatable.is_null() {
        *relocatable = g_hash_table_get_keys_as_array(reloc, ::core::ptr::null_mut::<guint>())
            as *mut *mut gchar;
        g_hash_table_steal_all(reloc);
    }
    g_hash_table_unref(single);
    g_hash_table_unref(reloc);
}
static mut safe_c2rust_non_relocatable_schema_list: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_relocatable_schema_list: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_schema_lists_initialised: gsize = 0;
unsafe extern "C" fn safe_c2rust_ensure_schema_lists() {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_schema_lists_initialised;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_schema_lists_initialised;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &raw mut safe_c2rust_schema_lists_initialised as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_initialise_schema_sources();
        safe_c2rust_g_settings_schema_source_list_schemas(
            safe_c2rust_schema_sources,
            TRUE,
            &raw mut safe_c2rust_non_relocatable_schema_list,
            &raw mut safe_c2rust_relocatable_schema_list,
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_schema_lists_initialised =
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_schema_lists_initialised as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_list_schemas() -> *const *const gchar {
    safe_c2rust_ensure_schema_lists();
    return safe_c2rust_non_relocatable_schema_list as *mut *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_list_relocatable_schemas() -> *const *const gchar {
    safe_c2rust_ensure_schema_lists();
    return safe_c2rust_relocatable_schema_list as *mut *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_ref(
    mut schema: *mut GSettingsSchema,
) -> *mut GSettingsSchema {
    if 0 as ::core::ffi::c_int != 0 {
        (*schema).ref_count;
        (*schema).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*schema).ref_count, 1 as ::core::ffi::c_int);
    return schema;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_unref(mut schema: *mut GSettingsSchema) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*schema).ref_count;
            (*schema).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*schema).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if !(*schema).extends.is_null() {
            safe_c2rust_g_settings_schema_unref((*schema).extends);
        }
        safe_c2rust_g_settings_schema_source_unref((*schema).source);
        gvdb_table_free((*schema).table);
        g_free((*schema).items as gpointer);
        g_free((*schema).id as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<GSettingsSchema>() as gsize,
            schema as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_string(
    mut schema: *mut GSettingsSchema,
    mut key: *const gchar,
) -> *const gchar {
    let mut result: *const gchar = ::core::ptr::null::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = gvdb_table_get_raw_value((*schema).table, key);
    if !value.is_null() {
        result = g_variant_get_string(value, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(value);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_child_schema(
    mut schema: *mut GSettingsSchema,
    mut name: *const gchar,
) -> *mut GSettingsSchema {
    let mut child_id: *const gchar = ::core::ptr::null::<gchar>();
    let mut child_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    child_name = g_strconcat(
        name,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    child_id = safe_c2rust_g_settings_schema_get_string(schema, child_name);
    g_free(child_name as gpointer);
    if child_id.is_null() {
        return ::core::ptr::null_mut::<GSettingsSchema>();
    }
    return safe_c2rust_g_settings_schema_source_lookup((*schema).source, child_id, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_value(
    mut schema: *mut GSettingsSchema,
    mut key: *const gchar,
) -> *mut GVariantIter {
    let mut s: *mut GSettingsSchema = schema;
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !schema.is_null() {
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
            b"schema != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantIter>();
    }
    s = schema;
    while !s.is_null() {
        value = gvdb_table_get_raw_value((*s).table, key);
        if !value.is_null() {
            break;
        }
        s = (*s).extends;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if value.is_null()
            || g_variant_is_of_type(
                value,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) == 0
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Settings schema '%s' does not contain a key named '%s'\0" as *const u8
                as *const gchar,
            (*schema).id,
            key,
        );
        loop {}
    }
    iter = g_variant_iter_new(value);
    g_variant_unref(value);
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_path(
    mut schema: *mut GSettingsSchema,
) -> *const gchar {
    return (*schema).path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_gettext_domain(
    mut schema: *mut GSettingsSchema,
) -> *const gchar {
    return (*schema).gettext_domain;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_has_key(
    mut schema: *mut GSettingsSchema,
    mut key: *const gchar,
) -> gboolean {
    return gvdb_table_has_value((*schema).table, key);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_list_children(
    mut schema: *mut GSettingsSchema,
) -> *mut *mut gchar {
    let mut keys: *const GQuark = ::core::ptr::null::<GQuark>();
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n_keys: gint = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !schema.is_null() {
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
            b"schema != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    keys = safe_c2rust_g_settings_schema_list(schema, &raw mut n_keys);
    strv = ({
        let mut __n: gsize = (n_keys as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
    j = 0 as ::core::ffi::c_int as gint;
    i = j;
    while i < n_keys {
        let mut key: *const gchar = g_quark_to_string(*keys.offset(i as isize));
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = key as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_18
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(key, b"/\0" as *const u8 as *const gchar)
        } != 0
        {
            let mut length: gsize = strlen(key as *const ::core::ffi::c_char) as gsize;
            let ref mut fresh6 = *strv.offset(j as isize);
            *fresh6 = g_memdup2(key as gconstpointer, length) as *mut gchar;
            *(*strv.offset(j as isize)).offset(length.wrapping_sub(1 as gsize) as isize) =
                '\0' as i32 as gchar;
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh7 = *strv.offset(j as isize);
    *fresh7 = ::core::ptr::null_mut::<gchar>();
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_list_keys(
    mut schema: *mut GSettingsSchema,
) -> *mut *mut gchar {
    let mut keys: *const GQuark = ::core::ptr::null::<GQuark>();
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n_keys: gint = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !schema.is_null() {
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
            b"schema != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    keys = safe_c2rust_g_settings_schema_list(schema, &raw mut n_keys);
    strv = ({
        let mut __n: gsize = (n_keys as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
    j = 0 as ::core::ffi::c_int as gint;
    i = j;
    while i < n_keys {
        let mut key: *const gchar = g_quark_to_string(*keys.offset(i as isize));
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = key as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_20
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(key, b"/\0" as *const u8 as *const gchar)
        } == 0
        {
            let fresh2 = j;
            j = j + 1;
            let ref mut fresh3 = *strv.offset(fresh2 as isize);
            *fresh3 = safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
        }
        i += 1;
    }
    let ref mut fresh4 = *strv.offset(j as isize);
    *fresh4 = ::core::ptr::null_mut::<gchar>();
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_list(
    mut schema: *mut GSettingsSchema,
    mut n_items: *mut gint,
) -> *const GQuark {
    if (*schema).items.is_null() {
        let mut s: *mut GSettingsSchema = ::core::ptr::null_mut::<GSettingsSchema>();
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut items: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut name: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut len: gint = 0;
        let mut i: gint = 0;
        items = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            None,
        );
        s = schema;
        while !s.is_null() {
            let mut list: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            list = gvdb_table_list((*s).table, b"\0" as *const u8 as *const gchar);
            if !list.is_null() {
                i = 0 as ::core::ffi::c_int as gint;
                while !(*list.offset(i as isize)).is_null() {
                    g_hash_table_add(items, *list.offset(i as isize) as gpointer);
                    i += 1;
                }
                g_free(list as gpointer);
            }
            s = (*s).extends;
        }
        g_hash_table_iter_init(&raw mut iter, items);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut name,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            if !(if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = name as *const ::core::ffi::c_char;
                    let __suffix: *const ::core::ffi::c_char =
                        b"/\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                        if __str.is_null() || __suffix.is_null() {
                            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_21
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __suffix_len: size_t = strlen(
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __suffix_len {
                            __result = (memcmp(
                                __str
                                    .offset(__str_len as isize)
                                    .offset(-(__suffix_len as isize))
                                    as *const ::core::ffi::c_void,
                                __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __suffix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                g_str_has_suffix(name as *const gchar, b"/\0" as *const u8 as *const gchar)
            } != 0)
            {
                continue;
            }
            let mut source: *mut GSettingsSchemaSource =
                ::core::ptr::null_mut::<GSettingsSchemaSource>();
            let mut child_schema: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut child_table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
            child_schema = gvdb_table_get_raw_value((*schema).table, name as *const gchar);
            if child_schema.is_null() {
                continue;
            }
            child_table = ::core::ptr::null_mut::<GvdbTable>();
            source = (*schema).source;
            while !source.is_null() {
                child_table = gvdb_table_get_table(
                    (*source).table,
                    g_variant_get_string(child_schema, ::core::ptr::null_mut::<gsize>()),
                );
                if !child_table.is_null() {
                    break;
                }
                source = (*source).parent;
            }
            g_variant_unref(child_schema);
            if child_table.is_null() {
                g_hash_table_iter_remove(&raw mut iter);
            } else {
                if gvdb_table_has_value(child_table, b".path\0" as *const u8 as *const gchar) != 0 {
                    let mut path: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    let mut expected: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    let mut same: gboolean = 0;
                    path = gvdb_table_get_raw_value(
                        child_table,
                        b".path\0" as *const u8 as *const gchar,
                    );
                    expected = g_strconcat((*schema).path, name, NULL_1);
                    same = (strcmp(
                        expected as *const ::core::ffi::c_char,
                        g_variant_get_string(path, ::core::ptr::null_mut::<gsize>())
                            as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as gboolean;
                    g_variant_unref(path);
                    g_free(expected as gpointer);
                    if same == 0 {
                        g_hash_table_iter_remove(&raw mut iter);
                    }
                }
                gvdb_table_free(child_table);
            }
        }
        len = g_hash_table_size(items) as gint;
        (*schema).items = ({
            let mut __n: gsize = len as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GQuark>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GQuark;
        i = 0 as ::core::ffi::c_int as gint;
        g_hash_table_iter_init(&raw mut iter, items);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut name,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            let fresh5 = i;
            i = i + 1;
            *(*schema).items.offset(fresh5 as isize) = g_quark_from_string(name as *const gchar);
        }
        (*schema).n_items = i;
        if ({
            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
            if i == len {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettingsschema.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1239 as ::core::ffi::c_int,
                G_STRFUNC,
                b"i == len\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_hash_table_unref(items);
    }
    *n_items = (*schema).n_items;
    return (*schema).items;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_id(
    mut schema: *mut GSettingsSchema,
) -> *const gchar {
    return (*schema).id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_endian_fixup(mut value: *mut *mut GVariant) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_init(
    mut key: *mut GSettingsSchemaKey,
    mut schema: *mut GSettingsSchema,
    mut name: *const gchar,
) {
    let mut current_block: u64;
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut code: guchar = 0;
    memset(
        key as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GSettingsSchemaKey>() as size_t,
    );
    iter = safe_c2rust_g_settings_schema_get_value(schema, name);
    (*key).schema = safe_c2rust_g_settings_schema_ref(schema);
    (*key).default_value = g_variant_iter_next_value(iter);
    safe_c2rust_endian_fixup(&raw mut (*key).default_value);
    (*key).type_0 = g_variant_get_type((*key).default_value);
    (*key).name = g_intern_string(name);
    while g_variant_iter_next(
        iter,
        b"(y*)\0" as *const u8 as *const gchar,
        &raw mut code,
        &raw mut data,
    ) != 0
    {
        match code as ::core::ffi::c_int {
            108 => {
                g_variant_get(
                    data,
                    b"(y&s)\0" as *const u8 as *const gchar,
                    &raw mut (*key).lc_char,
                    &raw mut (*key).unparsed,
                );
                current_block = 2370887241019905314;
            }
            101 => {
                (*key).set_is_enum(TRUE as guint as guint);
                current_block = 53785787823454161;
            }
            102 => {
                (*key).set_is_flags(TRUE as guint as guint);
                current_block = 53785787823454161;
            }
            99 => {
                current_block = 53785787823454161;
            }
            114 => {
                g_variant_get(
                    data,
                    b"(**)\0" as *const u8 as *const gchar,
                    &raw mut (*key).minimum,
                    &raw mut (*key).maximum,
                );
                safe_c2rust_endian_fixup(&raw mut (*key).minimum);
                safe_c2rust_endian_fixup(&raw mut (*key).maximum);
                current_block = 2370887241019905314;
            }
            100 => {
                g_variant_get(
                    data,
                    b"@a{sv}\0" as *const u8 as *const gchar,
                    &raw mut (*key).desktop_overrides,
                );
                safe_c2rust_endian_fixup(&raw mut (*key).desktop_overrides);
                current_block = 2370887241019905314;
            }
            _ => {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"unknown schema extension '%c'\0" as *const u8 as *const gchar,
                    code as ::core::ffi::c_int,
                );
                current_block = 2370887241019905314;
            }
        }
        match current_block {
            53785787823454161 => {
                (*key).strinfo = g_variant_get_fixed_array(
                    data,
                    &raw mut (*key).strinfo_length,
                    ::core::mem::size_of::<guint32>() as gsize,
                ) as *const guint32;
            }
            _ => {}
        }
        g_variant_unref(data);
    }
    g_variant_iter_free(iter);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_clear(mut key: *mut GSettingsSchemaKey) {
    if !(*key).minimum.is_null() {
        g_variant_unref((*key).minimum);
    }
    if !(*key).maximum.is_null() {
        g_variant_unref((*key).maximum);
    }
    if !(*key).desktop_overrides.is_null() {
        g_variant_unref((*key).desktop_overrides);
    }
    g_variant_unref((*key).default_value);
    safe_c2rust_g_settings_schema_unref((*key).schema);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_type_check(
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_variant_is_of_type(value, (*key).type_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_range_fixup(
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> *mut GVariant {
    let mut target: *const gchar = ::core::ptr::null::<gchar>();
    if safe_c2rust_g_settings_schema_key_range_check(key, value) != 0 {
        return g_variant_ref(value);
    }
    if (*key).strinfo.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if g_variant_is_container(value) != 0 {
        let mut builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
        let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_iter_init(&raw mut iter, value);
        g_variant_builder_init(&raw mut builder, g_variant_get_type(value));
        loop {
            child = g_variant_iter_next_value(&raw mut iter);
            if child.is_null() {
                break;
            }
            let mut fixed: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            fixed = safe_c2rust_g_settings_schema_key_range_fixup(key, child);
            g_variant_unref(child);
            if fixed.is_null() {
                g_variant_builder_clear(&raw mut builder);
                return ::core::ptr::null_mut::<GVariant>();
            }
            g_variant_builder_add_value(&raw mut builder, fixed);
            g_variant_unref(fixed);
        }
        return g_variant_ref_sink(g_variant_builder_end(&raw mut builder));
    }
    target = safe_c2rust_strinfo_string_from_alias(
        (*key).strinfo,
        (*key).strinfo_length as guint,
        g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
    );
    return if !target.is_null() {
        g_variant_ref_sink(g_variant_new_string(target))
    } else {
        ::core::ptr::null_mut::<GVariant>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_translated_default(
    mut key: *mut GSettingsSchemaKey,
) -> *mut GVariant {
    let mut translated: *const gchar = ::core::ptr::null::<gchar>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut domain: *const gchar = ::core::ptr::null::<gchar>();
    let mut lc_time: *const gchar = ::core::ptr::null::<gchar>();
    let mut old_locale: locale_t = ::core::ptr::null_mut::<__locale_struct>();
    let mut locale: locale_t = ::core::ptr::null_mut::<__locale_struct>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    domain = safe_c2rust_g_settings_schema_get_gettext_domain((*key).schema);
    if (*key).lc_char as ::core::ffi::c_int == '\0' as i32 {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if (*key).lc_char as ::core::ffi::c_int == 't' as i32 {
        lc_time = setlocale(LC_TIME, ::core::ptr::null::<::core::ffi::c_char>());
        if !lc_time.is_null() {
            locale = newlocale(
                LC_MESSAGES_MASK,
                lc_time as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<__locale_struct>(),
            );
            if !locale.is_null() {
                old_locale = uselocale(locale);
                translated = g_dgettext(domain, (*key).unparsed);
                uselocale(old_locale);
                freelocale(locale);
            }
        }
    }
    if translated.is_null() {
        translated = g_dgettext(domain, (*key).unparsed);
    }
    if translated == (*key).unparsed {
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = g_variant_parse(
        (*key).type_0,
        translated,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut error,
    );
    if value.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Failed to parse translated string '%s' for key '%s' in schema '%s': %s\0" as *const u8
                as *const gchar,
            translated,
            (*key).name,
            safe_c2rust_g_settings_schema_get_id((*key).schema),
            (*error).message,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Using untranslated default instead.\0" as *const u8 as *const gchar,
        );
        g_error_free(error);
    } else if safe_c2rust_g_settings_schema_key_range_check(key, value) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Translated default '%s' for key '%s' in schema '%s' is outside of valid range\0"
                as *const u8 as *const gchar,
            (*key).unparsed,
            (*key).name,
            safe_c2rust_g_settings_schema_get_id((*key).schema),
        );
        g_variant_unref(value);
        value = ::core::ptr::null_mut::<GVariant>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_per_desktop_default(
    mut key: *mut GSettingsSchemaKey,
) -> *mut GVariant {
    static mut safe_c2rust_current_desktops: *const *const gchar =
        ::core::ptr::null::<*const gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut i: gint = 0;
    if (*key).desktop_overrides.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_current_desktops;
        } else {
        };
        (({
            let mut gapg_temp_newval: *const *const gchar = ::core::ptr::null::<*const gchar>();
            let mut gapg_temp_atomic: *mut *const *const gchar =
                &raw mut safe_c2rust_current_desktops;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_current_desktops as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut xdg_current_desktop: *const gchar =
            g_getenv(b"XDG_CURRENT_DESKTOP\0" as *const u8 as *const gchar);
        let mut tmp: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        if !xdg_current_desktop.is_null()
            && *xdg_current_desktop.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
        {
            tmp = g_strsplit(
                xdg_current_desktop,
                G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
                -(1 as gint),
            );
        } else {
            tmp = ({
                let mut __n: gsize = (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_current_desktops = tmp as *mut *const gchar;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_current_desktops as *mut ::core::ffi::c_void,
            tmp as *mut *const gchar as guintptr as gpointer,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    while value.is_null() && !(*safe_c2rust_current_desktops.offset(i as isize)).is_null() {
        value = g_variant_lookup_value(
            (*key).desktop_overrides,
            *safe_c2rust_current_desktops.offset(i as isize),
            ::core::ptr::null::<GVariantType>(),
        );
        i += 1;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_to_enum(
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> gint {
    let mut it_worked: gboolean = 0;
    let mut result: guint = 0;
    it_worked = safe_c2rust_strinfo_enum_from_string(
        (*key).strinfo,
        (*key).strinfo_length as guint,
        g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
        &raw mut result,
    );
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if it_worked != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettingsschema.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1524 as ::core::ffi::c_int,
            G_STRFUNC,
            b"it_worked\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return result as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_from_enum(
    mut key: *mut GSettingsSchemaKey,
    mut value: gint,
) -> *mut GVariant {
    let mut string: *const gchar = ::core::ptr::null::<gchar>();
    string = safe_c2rust_strinfo_string_from_enum(
        (*key).strinfo,
        (*key).strinfo_length as guint,
        value as guint,
    );
    if string.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new_string(string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_to_flags(
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> guint {
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut flag: *const gchar = ::core::ptr::null::<gchar>();
    let mut result: guint = 0;
    result = 0 as guint;
    g_variant_iter_init(&raw mut iter, value);
    while g_variant_iter_next(
        &raw mut iter,
        b"&s\0" as *const u8 as *const gchar,
        &raw mut flag,
    ) != 0
    {
        let mut it_worked: gboolean = 0;
        let mut flag_value: guint = 0;
        it_worked = safe_c2rust_strinfo_enum_from_string(
            (*key).strinfo,
            (*key).strinfo_length as guint,
            flag,
            &raw mut flag_value,
        );
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if it_worked != 0 {
                _g_boolean_var_25 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_25 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_25
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettingsschema.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1561 as ::core::ffi::c_int,
                G_STRFUNC,
                b"it_worked\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        result |= flag_value;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_from_flags(
    mut key: *mut GSettingsSchemaKey,
    mut value: guint,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut i: gint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    i = 0 as ::core::ffi::c_int as gint;
    while i < 32 as ::core::ffi::c_int {
        if value as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i != 0 {
            let mut string: *const gchar = ::core::ptr::null::<gchar>();
            string = safe_c2rust_strinfo_string_from_enum(
                (*key).strinfo,
                (*key).strinfo_length as guint,
                (1 as guint) << i,
            );
            if string.is_null() {
                g_variant_builder_clear(&raw mut builder);
                return ::core::ptr::null_mut::<GVariant>();
            }
            g_variant_builder_add(
                &raw mut builder,
                b"s\0" as *const u8 as *const gchar,
                string,
            );
        }
        i += 1;
    }
    return g_variant_builder_end(&raw mut builder);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_schema_key_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_6, C2RustUnnamed_5) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_6, C2RustUnnamed_5) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GSettingsSchemaKey\0" as *const u8 as *const gchar),
        C2RustUnnamed_6 {
            do_copy_type: Some(
                safe_c2rust_g_settings_schema_key_ref
                    as unsafe extern "C" fn(*mut GSettingsSchemaKey) -> *mut GSettingsSchemaKey,
            ),
        },
        C2RustUnnamed_5 {
            do_free_type: Some(
                safe_c2rust_g_settings_schema_key_unref
                    as unsafe extern "C" fn(*mut GSettingsSchemaKey) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_ref(
    mut key: *mut GSettingsSchemaKey,
) -> *mut GSettingsSchemaKey {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsSchemaKey>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*key).ref_count;
        (*key).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*key).ref_count, 1 as ::core::ffi::c_int);
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_unref(mut key: *mut GSettingsSchemaKey) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*key).ref_count;
            (*key).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(&raw mut (*key).ref_count, 1 as ::core::ffi::c_int)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_settings_schema_key_clear(key);
        g_slice_free1(
            ::core::mem::size_of::<GSettingsSchemaKey>() as gsize,
            key as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_get_key(
    mut schema: *mut GSettingsSchema,
    mut name: *const gchar,
) -> *mut GSettingsSchemaKey {
    let mut key: *mut GSettingsSchemaKey = ::core::ptr::null_mut::<GSettingsSchemaKey>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !schema.is_null() {
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
            b"schema != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsSchemaKey>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsSchemaKey>();
    }
    key = g_slice_alloc(::core::mem::size_of::<GSettingsSchemaKey>() as gsize)
        as *mut GSettingsSchemaKey;
    safe_c2rust_g_settings_schema_key_init(key, schema, name);
    (*key).ref_count = 1 as ::core::ffi::c_int as gint;
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_name(
    mut key: *mut GSettingsSchemaKey,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*key).name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_summary(
    mut key: *mut GSettingsSchemaKey,
) -> *const gchar {
    let mut text_tables: *mut *mut GHashTable = ::core::ptr::null_mut::<*mut GHashTable>();
    let mut summaries: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    text_tables = safe_c2rust_g_settings_schema_source_get_text_tables((*(*key).schema).source);
    summaries = g_hash_table_lookup(
        *text_tables.offset(0 as ::core::ffi::c_int as isize),
        (*(*key).schema).id as gconstpointer,
    ) as *mut GHashTable;
    return (if !summaries.is_null() {
        g_hash_table_lookup(summaries, (*key).name as gconstpointer)
    } else {
        NULL_1
    }) as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_description(
    mut key: *mut GSettingsSchemaKey,
) -> *const gchar {
    let mut text_tables: *mut *mut GHashTable = ::core::ptr::null_mut::<*mut GHashTable>();
    let mut descriptions: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    text_tables = safe_c2rust_g_settings_schema_source_get_text_tables((*(*key).schema).source);
    descriptions = g_hash_table_lookup(
        *text_tables.offset(1 as ::core::ffi::c_int as isize),
        (*(*key).schema).id as gconstpointer,
    ) as *mut GHashTable;
    return (if !descriptions.is_null() {
        g_hash_table_lookup(descriptions, (*key).name as gconstpointer)
    } else {
        NULL_1
    }) as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_value_type(
    mut key: *mut GSettingsSchemaKey,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return (*key).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_default_value(
    mut key: *mut GSettingsSchemaKey,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = safe_c2rust_g_settings_schema_key_get_translated_default(key);
    if value.is_null() {
        value = safe_c2rust_g_settings_schema_key_get_per_desktop_default(key);
    }
    if value.is_null() {
        value = g_variant_ref((*key).default_value);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_get_range(
    mut key: *mut GSettingsSchemaKey,
) -> *mut GVariant {
    let mut type_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut range: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !(*key).minimum.is_null() {
        range = g_variant_new(
            b"(**)\0" as *const u8 as *const gchar,
            (*key).minimum,
            (*key).maximum,
        );
        type_0 = b"range\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    } else if !(*key).strinfo.is_null() {
        range = safe_c2rust_strinfo_enumerate((*key).strinfo, (*key).strinfo_length as guint);
        type_0 = (if (*key).is_flags() as ::core::ffi::c_int != 0 {
            b"flags\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"enum\0" as *const u8 as *const ::core::ffi::c_char
        }) as *const gchar;
    } else {
        range = g_variant_new_array(
            (*key).type_0,
            ::core::ptr::null::<*mut GVariant>(),
            0 as gsize,
        );
        type_0 = b"type\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    return g_variant_ref_sink(g_variant_new(
        b"(sv)\0" as *const u8 as *const gchar,
        type_0,
        range,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_schema_key_range_check(
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> gboolean {
    if (*key).minimum.is_null() && (*key).strinfo.is_null() {
        return TRUE;
    }
    if g_variant_is_container(value) != 0 {
        let mut ok: gboolean = TRUE;
        let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
        let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_iter_init(&raw mut iter, value);
        while ok != 0 && {
            child = g_variant_iter_next_value(&raw mut iter);
            !child.is_null()
        } {
            ok = safe_c2rust_g_settings_schema_key_range_check(key, child);
            g_variant_unref(child);
        }
        return ok;
    }
    if !(*key).minimum.is_null() {
        return (g_variant_compare((*key).minimum as gconstpointer, value as gconstpointer)
            <= 0 as ::core::ffi::c_int
            && g_variant_compare(value as gconstpointer, (*key).maximum as gconstpointer)
                <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    return safe_c2rust_strinfo_is_string_valid(
        (*key).strinfo,
        (*key).strinfo_length as guint,
        g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
