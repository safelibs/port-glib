extern "C" {
    pub type _GHashTable;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_strchug(string: *mut gchar) -> *mut gchar;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_private_set_alloc0(key: *mut GPrivate, size: gsize) -> gpointer;
    fn _g_locale_charset_raw() -> *const ::core::ffi::c_char;
    fn _g_locale_charset_unalias(codeset: *const ::core::ffi::c_char)
        -> *const ::core::ffi::c_char;
    fn _g_locale_get_charset_aliases() -> *const ::core::ffi::c_char;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GCharsetCache = _GCharsetCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCharsetCache {
    pub is_utf8: gboolean,
    pub raw: *mut gchar,
    pub charset: *mut gchar,
}
pub type GPrivate = _GPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GLanguageNamesCache = _GLanguageNamesCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLanguageNamesCache {
    pub languages: *mut gchar,
    pub language_names: *mut *mut gchar,
}
pub type GHashTable = _GHashTable;
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
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
pub type FILE = _IO_FILE;
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
pub const COMPONENT_MODIFIER: C2RustUnnamed_0 = 4;
pub const COMPONENT_TERRITORY: C2RustUnnamed_0 = 2;
pub const COMPONENT_CODESET: C2RustUnnamed_0 = 1;
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
pub type nl_item = ::core::ffi::c_int;
pub const CODESET: C2RustUnnamed = 14;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _NL_NUM: C2RustUnnamed = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed = 327684;
pub const __NOSTR: C2RustUnnamed = 327683;
pub const __YESSTR: C2RustUnnamed = 327682;
pub const __NOEXPR: C2RustUnnamed = 327681;
pub const __YESEXPR: C2RustUnnamed = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed = 65539;
pub const __GROUPING: C2RustUnnamed = 65538;
pub const THOUSEP: C2RustUnnamed = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed = 65537;
pub const RADIXCHAR: C2RustUnnamed = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed = 262149;
pub const __MON_GROUPING: C2RustUnnamed = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed = 15;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed = 131195;
pub const __ALTMON_12: C2RustUnnamed = 131194;
pub const __ALTMON_11: C2RustUnnamed = 131193;
pub const __ALTMON_10: C2RustUnnamed = 131192;
pub const __ALTMON_9: C2RustUnnamed = 131191;
pub const __ALTMON_8: C2RustUnnamed = 131190;
pub const __ALTMON_7: C2RustUnnamed = 131189;
pub const __ALTMON_6: C2RustUnnamed = 131188;
pub const __ALTMON_5: C2RustUnnamed = 131187;
pub const __ALTMON_4: C2RustUnnamed = 131186;
pub const __ALTMON_3: C2RustUnnamed = 131185;
pub const __ALTMON_2: C2RustUnnamed = 131184;
pub const __ALTMON_1: C2RustUnnamed = 131183;
pub const _NL_W_DATE_FMT: C2RustUnnamed = 131181;
pub const _DATE_FMT: C2RustUnnamed = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed = 131167;
pub const _NL_WT_FMT: C2RustUnnamed = 131166;
pub const _NL_WD_FMT: C2RustUnnamed = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed = 131164;
pub const _NL_WPM_STR: C2RustUnnamed = 131163;
pub const _NL_WAM_STR: C2RustUnnamed = 131162;
pub const _NL_WMON_12: C2RustUnnamed = 131161;
pub const _NL_WMON_11: C2RustUnnamed = 131160;
pub const _NL_WMON_10: C2RustUnnamed = 131159;
pub const _NL_WMON_9: C2RustUnnamed = 131158;
pub const _NL_WMON_8: C2RustUnnamed = 131157;
pub const _NL_WMON_7: C2RustUnnamed = 131156;
pub const _NL_WMON_6: C2RustUnnamed = 131155;
pub const _NL_WMON_5: C2RustUnnamed = 131154;
pub const _NL_WMON_4: C2RustUnnamed = 131153;
pub const _NL_WMON_3: C2RustUnnamed = 131152;
pub const _NL_WMON_2: C2RustUnnamed = 131151;
pub const _NL_WMON_1: C2RustUnnamed = 131150;
pub const _NL_WABMON_12: C2RustUnnamed = 131149;
pub const _NL_WABMON_11: C2RustUnnamed = 131148;
pub const _NL_WABMON_10: C2RustUnnamed = 131147;
pub const _NL_WABMON_9: C2RustUnnamed = 131146;
pub const _NL_WABMON_8: C2RustUnnamed = 131145;
pub const _NL_WABMON_7: C2RustUnnamed = 131144;
pub const _NL_WABMON_6: C2RustUnnamed = 131143;
pub const _NL_WABMON_5: C2RustUnnamed = 131142;
pub const _NL_WABMON_4: C2RustUnnamed = 131141;
pub const _NL_WABMON_3: C2RustUnnamed = 131140;
pub const _NL_WABMON_2: C2RustUnnamed = 131139;
pub const _NL_WABMON_1: C2RustUnnamed = 131138;
pub const _NL_WDAY_7: C2RustUnnamed = 131137;
pub const _NL_WDAY_6: C2RustUnnamed = 131136;
pub const _NL_WDAY_5: C2RustUnnamed = 131135;
pub const _NL_WDAY_4: C2RustUnnamed = 131134;
pub const _NL_WDAY_3: C2RustUnnamed = 131133;
pub const _NL_WDAY_2: C2RustUnnamed = 131132;
pub const _NL_WDAY_1: C2RustUnnamed = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed = 131122;
pub const ERA_T_FMT: C2RustUnnamed = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed = 131120;
pub const ALT_DIGITS: C2RustUnnamed = 131119;
pub const ERA_D_FMT: C2RustUnnamed = 131118;
pub const __ERA_YEAR: C2RustUnnamed = 131117;
pub const ERA: C2RustUnnamed = 131116;
pub const T_FMT_AMPM: C2RustUnnamed = 131115;
pub const T_FMT: C2RustUnnamed = 131114;
pub const D_FMT: C2RustUnnamed = 131113;
pub const D_T_FMT: C2RustUnnamed = 131112;
pub const PM_STR: C2RustUnnamed = 131111;
pub const AM_STR: C2RustUnnamed = 131110;
pub const MON_12: C2RustUnnamed = 131109;
pub const MON_11: C2RustUnnamed = 131108;
pub const MON_10: C2RustUnnamed = 131107;
pub const MON_9: C2RustUnnamed = 131106;
pub const MON_8: C2RustUnnamed = 131105;
pub const MON_7: C2RustUnnamed = 131104;
pub const MON_6: C2RustUnnamed = 131103;
pub const MON_5: C2RustUnnamed = 131102;
pub const MON_4: C2RustUnnamed = 131101;
pub const MON_3: C2RustUnnamed = 131100;
pub const MON_2: C2RustUnnamed = 131099;
pub const MON_1: C2RustUnnamed = 131098;
pub const ABMON_12: C2RustUnnamed = 131097;
pub const ABMON_11: C2RustUnnamed = 131096;
pub const ABMON_10: C2RustUnnamed = 131095;
pub const ABMON_9: C2RustUnnamed = 131094;
pub const ABMON_8: C2RustUnnamed = 131093;
pub const ABMON_7: C2RustUnnamed = 131092;
pub const ABMON_6: C2RustUnnamed = 131091;
pub const ABMON_5: C2RustUnnamed = 131090;
pub const ABMON_4: C2RustUnnamed = 131089;
pub const ABMON_3: C2RustUnnamed = 131088;
pub const ABMON_2: C2RustUnnamed = 131087;
pub const ABMON_1: C2RustUnnamed = 131086;
pub const DAY_7: C2RustUnnamed = 131085;
pub const DAY_6: C2RustUnnamed = 131084;
pub const DAY_5: C2RustUnnamed = 131083;
pub const DAY_4: C2RustUnnamed = 131082;
pub const DAY_3: C2RustUnnamed = 131081;
pub const DAY_2: C2RustUnnamed = 131080;
pub const DAY_1: C2RustUnnamed = 131079;
pub const ABDAY_7: C2RustUnnamed = 131078;
pub const ABDAY_6: C2RustUnnamed = 131077;
pub const ABDAY_5: C2RustUnnamed = 131076;
pub const ABDAY_4: C2RustUnnamed = 131075;
pub const ABDAY_3: C2RustUnnamed = 131074;
pub const ABDAY_2: C2RustUnnamed = 131073;
pub const ABDAY_1: C2RustUnnamed = 131072;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
static mut safe_c2rust_g__aliases_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_get_alias_hash() -> *mut GHashTable {
    static mut safe_c2rust_alias_hash: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    let mut aliases: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    g_mutex_lock(&raw mut safe_c2rust_g__aliases_lock);
    if safe_c2rust_alias_hash.is_null() {
        safe_c2rust_alias_hash = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        aliases = _g_locale_get_charset_aliases();
        while *aliases as ::core::ffi::c_int != '\0' as i32 {
            let mut canonical: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut alias: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut alias_array: *mut *const ::core::ffi::c_char =
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
            let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            alias = aliases;
            aliases = aliases.offset(strlen(aliases).wrapping_add(1 as size_t) as isize);
            canonical = aliases;
            aliases = aliases.offset(strlen(aliases).wrapping_add(1 as size_t) as isize);
            alias_array = g_hash_table_lookup(safe_c2rust_alias_hash, canonical as gconstpointer)
                as *mut *const ::core::ffi::c_char;
            if !alias_array.is_null() {
                while !(*alias_array.offset(count as isize)).is_null() {
                    count += 1;
                }
            }
            alias_array = ({
                let mut __n: gsize = (count + 2 as ::core::ffi::c_int) as gsize;
                let mut __s: gsize = ::core::mem::size_of::<*const ::core::ffi::c_char>() as gsize;
                let mut __p: gpointer = alias_array as gpointer;
                if __s == 1 as gsize {
                    __p = g_realloc(__p, __n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_realloc(__p, __n.wrapping_mul(__s));
                } else {
                    __p = g_realloc_n(__p, __n, __s);
                }
                __p
            }) as *mut *const ::core::ffi::c_char;
            let ref mut fresh1 = *alias_array.offset(count as isize);
            *fresh1 = alias;
            let ref mut fresh2 = *alias_array.offset((count + 1 as ::core::ffi::c_int) as isize);
            *fresh2 = ::core::ptr::null::<::core::ffi::c_char>();
            g_hash_table_insert(
                safe_c2rust_alias_hash,
                canonical as *mut ::core::ffi::c_char as gpointer,
                alias_array as gpointer,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__aliases_lock);
    return safe_c2rust_alias_hash;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_charset_get_aliases(
    mut canonical_name: *const ::core::ffi::c_char,
) -> *mut *const ::core::ffi::c_char {
    let mut alias_hash: *mut GHashTable = safe_c2rust_get_alias_hash();
    return g_hash_table_lookup(alias_hash, canonical_name as gconstpointer)
        as *mut *const ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_utf8_get_charset_internal(
    mut raw_data: *const ::core::ffi::c_char,
    mut a: *mut *const ::core::ffi::c_char,
) -> gboolean {
    let mut charset: *const ::core::ffi::c_char =
        g_getenv(b"CHARSET\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char;
    if !charset.is_null() && *charset as ::core::ffi::c_int != 0 {
        *a = charset;
        if !charset.is_null()
            && !strstr(
                charset,
                b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            return TRUE;
        } else {
            return FALSE;
        }
    }
    g_mutex_lock(&raw mut safe_c2rust_g__aliases_lock);
    charset = _g_locale_charset_unalias(raw_data);
    g_mutex_unlock(&raw mut safe_c2rust_g__aliases_lock);
    if !charset.is_null() && *charset as ::core::ffi::c_int != 0 {
        *a = charset;
        if !charset.is_null()
            && !strstr(
                charset,
                b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            return TRUE;
        } else {
            return FALSE;
        }
    }
    *a = b"US-ASCII\0" as *const u8 as *const ::core::ffi::c_char;
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_charset_cache_free(mut data: gpointer) {
    let mut cache: *mut GCharsetCache = data as *mut GCharsetCache;
    g_free((*cache).raw as gpointer);
    g_free((*cache).charset as gpointer);
    g_free(cache as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_charset(
    mut charset: *mut *const ::core::ffi::c_char,
) -> gboolean {
    static mut safe_c2rust_cache_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL,
            notify: Some(safe_c2rust_charset_cache_free as unsafe extern "C" fn(gpointer) -> ()),
            future: [NULL, NULL],
        }
    };
    let mut cache: *mut GCharsetCache =
        g_private_get(&raw mut safe_c2rust_cache_private) as *mut GCharsetCache;
    let mut raw: *const gchar = ::core::ptr::null::<gchar>();
    if cache.is_null() {
        cache = g_private_set_alloc0(
            &raw mut safe_c2rust_cache_private,
            ::core::mem::size_of::<GCharsetCache>() as gsize,
        ) as *mut GCharsetCache;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__aliases_lock);
    raw = _g_locale_charset_raw() as *const gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__aliases_lock);
    if (*cache).raw.is_null()
        || strcmp((*cache).raw, raw as *const ::core::ffi::c_char) != 0 as ::core::ffi::c_int
    {
        let mut new_charset: *const gchar = ::core::ptr::null::<gchar>();
        g_free((*cache).raw as gpointer);
        g_free((*cache).charset as gpointer);
        (*cache).raw = safe_c2rust_g_strdup_inline(raw as *const ::core::ffi::c_char) as *mut gchar;
        (*cache).is_utf8 = safe_c2rust_g_utf8_get_charset_internal(
            raw as *const ::core::ffi::c_char,
            &raw mut new_charset,
        );
        (*cache).charset =
            safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !charset.is_null() {
        *charset = (*cache).charset;
    }
    return (*cache).is_utf8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_get_time_charset(
    mut charset: *mut *const ::core::ffi::c_char,
) -> gboolean {
    static mut safe_c2rust_cache_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL,
            notify: Some(safe_c2rust_charset_cache_free as unsafe extern "C" fn(gpointer) -> ()),
            future: [NULL, NULL],
        }
    };
    let mut cache: *mut GCharsetCache =
        g_private_get(&raw mut safe_c2rust_cache_private) as *mut GCharsetCache;
    let mut raw: *const gchar = ::core::ptr::null::<gchar>();
    if cache.is_null() {
        cache = g_private_set_alloc0(
            &raw mut safe_c2rust_cache_private,
            ::core::mem::size_of::<GCharsetCache>() as gsize,
        ) as *mut GCharsetCache;
    }
    raw = nl_langinfo(_NL_TIME_CODESET as ::core::ffi::c_int as nl_item);
    if (*cache).raw.is_null()
        || strcmp((*cache).raw, raw as *const ::core::ffi::c_char) != 0 as ::core::ffi::c_int
    {
        let mut new_charset: *const gchar = ::core::ptr::null::<gchar>();
        g_free((*cache).raw as gpointer);
        g_free((*cache).charset as gpointer);
        (*cache).raw = safe_c2rust_g_strdup_inline(raw as *const ::core::ffi::c_char) as *mut gchar;
        (*cache).is_utf8 = safe_c2rust_g_utf8_get_charset_internal(
            raw as *const ::core::ffi::c_char,
            &raw mut new_charset,
        );
        (*cache).charset =
            safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !charset.is_null() {
        *charset = (*cache).charset;
    }
    return (*cache).is_utf8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_get_ctype_charset(
    mut charset: *mut *const ::core::ffi::c_char,
) -> gboolean {
    static mut safe_c2rust_cache_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL,
            notify: Some(safe_c2rust_charset_cache_free as unsafe extern "C" fn(gpointer) -> ()),
            future: [NULL, NULL],
        }
    };
    let mut cache: *mut GCharsetCache =
        g_private_get(&raw mut safe_c2rust_cache_private) as *mut GCharsetCache;
    let mut raw: *const gchar = ::core::ptr::null::<gchar>();
    if cache.is_null() {
        cache = g_private_set_alloc0(
            &raw mut safe_c2rust_cache_private,
            ::core::mem::size_of::<GCharsetCache>() as gsize,
        ) as *mut GCharsetCache;
    }
    raw = nl_langinfo(CODESET as ::core::ffi::c_int as nl_item);
    if (*cache).raw.is_null()
        || strcmp((*cache).raw, raw as *const ::core::ffi::c_char) != 0 as ::core::ffi::c_int
    {
        let mut new_charset: *const gchar = ::core::ptr::null::<gchar>();
        g_free((*cache).raw as gpointer);
        g_free((*cache).charset as gpointer);
        (*cache).raw = safe_c2rust_g_strdup_inline(raw as *const ::core::ffi::c_char) as *mut gchar;
        (*cache).is_utf8 = safe_c2rust_g_utf8_get_charset_internal(
            raw as *const ::core::ffi::c_char,
            &raw mut new_charset,
        );
        (*cache).charset =
            safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !charset.is_null() {
        *charset = (*cache).charset;
    }
    return (*cache).is_utf8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_codeset() -> *mut gchar {
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    safe_c2rust_g_get_charset(&raw mut charset);
    return safe_c2rust_g_strdup_inline(charset as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_console_charset(
    mut charset: *mut *const ::core::ffi::c_char,
) -> gboolean {
    return safe_c2rust_g_get_charset(charset);
}
unsafe extern "C" fn safe_c2rust_read_aliases(
    mut file: *const gchar,
    mut alias_table: *mut GHashTable,
) {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    fp = fopen(
        file as *const ::core::ffi::c_char,
        b"re\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return;
    }
    while !fgets(
        &raw mut buf as *mut ::core::ffi::c_char,
        256 as ::core::ffi::c_int,
        fp,
    )
    .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_strchomp(g_strchug(&raw mut buf as *mut gchar));
        if buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
            || buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32
        {
            continue;
        }
        p = &raw mut buf as *mut ::core::ffi::c_char;
        q = ::core::ptr::null_mut::<::core::ffi::c_char>();
        while *p != 0 {
            if *p as ::core::ffi::c_int == '\t' as i32
                || *p as ::core::ffi::c_int == ' ' as i32
                || *p as ::core::ffi::c_int == ':' as i32
            {
                *p = '\0' as i32 as ::core::ffi::c_char;
                q = p.offset(1 as ::core::ffi::c_int as isize);
                while *q as ::core::ffi::c_int == '\t' as i32
                    || *q as ::core::ffi::c_int == ' ' as i32
                {
                    q = q.offset(1);
                }
                break;
            } else {
                p = p.offset(1);
            }
        }
        if q.is_null() || *q as ::core::ffi::c_int == '\0' as i32 {
            continue;
        }
        p = q;
        while *p != 0 {
            if *p as ::core::ffi::c_int == '\t' as i32 || *p as ::core::ffi::c_int == ' ' as i32 {
                *p = '\0' as i32 as ::core::ffi::c_char;
                break;
            } else {
                p = p.offset(1);
            }
        }
        if g_hash_table_lookup(
            alias_table,
            &raw mut buf as *mut ::core::ffi::c_char as gconstpointer,
        )
        .is_null()
        {
            g_hash_table_insert(
                alias_table,
                safe_c2rust_g_strdup_inline(&raw mut buf as *mut ::core::ffi::c_char) as gpointer,
                safe_c2rust_g_strdup_inline(q) as gpointer,
            );
        }
    }
    fclose(fp);
}
unsafe extern "C" fn safe_c2rust_unalias_lang(
    mut lang: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut safe_c2rust_alias_table: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_alias_table;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
            let mut gapg_temp_atomic: *mut *mut GHashTable = &raw mut safe_c2rust_alias_table;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_alias_table as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut table: *mut GHashTable = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        safe_c2rust_read_aliases(
            b"/usr/share/locale/locale.alias\0" as *const u8 as *const gchar,
            table,
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_alias_table = table;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_alias_table as *mut ::core::ffi::c_void,
            table as guintptr as gpointer,
        );
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        p = g_hash_table_lookup(safe_c2rust_alias_table, lang as gconstpointer)
            as *mut ::core::ffi::c_char;
        if !(!p.is_null() && strcmp(p, lang) != 0 as ::core::ffi::c_int) {
            break;
        }
        lang = p;
        let fresh0 = i;
        i = i + 1;
        if fresh0 == 30 as ::core::ffi::c_int {
            static mut safe_c2rust_said_before: gboolean = FALSE;
            if safe_c2rust_said_before == 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Too many alias levels for a locale, may indicate a loop\0" as *const u8
                        as *const gchar,
                );
            }
            safe_c2rust_said_before = TRUE as gboolean;
            return lang;
        }
    }
    return lang;
}
unsafe extern "C" fn safe_c2rust_explode_locale(
    mut locale: *const gchar,
    mut language: *mut *mut gchar,
    mut territory: *mut *mut gchar,
    mut codeset: *mut *mut gchar,
    mut modifier: *mut *mut gchar,
) -> guint {
    let mut uscore_pos: *const gchar = ::core::ptr::null::<gchar>();
    let mut at_pos: *const gchar = ::core::ptr::null::<gchar>();
    let mut dot_pos: *const gchar = ::core::ptr::null::<gchar>();
    let mut mask: guint = 0 as guint;
    uscore_pos = strchr(locale as *const ::core::ffi::c_char, '_' as i32);
    dot_pos = strchr(
        if !uscore_pos.is_null() {
            uscore_pos as *const ::core::ffi::c_char
        } else {
            locale as *const ::core::ffi::c_char
        },
        '.' as i32,
    );
    at_pos = strchr(
        if !dot_pos.is_null() {
            dot_pos as *const ::core::ffi::c_char
        } else if !uscore_pos.is_null() {
            uscore_pos as *const ::core::ffi::c_char
        } else {
            locale as *const ::core::ffi::c_char
        },
        '@' as i32,
    );
    if !at_pos.is_null() {
        mask |= COMPONENT_MODIFIER as ::core::ffi::c_int as guint;
        *modifier = safe_c2rust_g_strdup_inline(at_pos as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        at_pos = locale.offset(strlen(locale as *const ::core::ffi::c_char) as isize);
    }
    if !dot_pos.is_null() {
        mask |= COMPONENT_CODESET as ::core::ffi::c_int as guint;
        *codeset = g_strndup(
            dot_pos,
            at_pos.offset_from(dot_pos) as ::core::ffi::c_long as gsize,
        );
    } else {
        dot_pos = at_pos;
    }
    if !uscore_pos.is_null() {
        mask |= COMPONENT_TERRITORY as ::core::ffi::c_int as guint;
        *territory = g_strndup(
            uscore_pos,
            dot_pos.offset_from(uscore_pos) as ::core::ffi::c_long as gsize,
        );
    } else {
        uscore_pos = dot_pos;
    }
    *language = g_strndup(
        locale,
        uscore_pos.offset_from(locale) as ::core::ffi::c_long as gsize,
    );
    return mask;
}
unsafe extern "C" fn safe_c2rust_append_locale_variants(
    mut array: *mut GPtrArray,
    mut locale: *const gchar,
) {
    let mut language: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut territory: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut codeset: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut modifier: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut mask: guint = 0;
    let mut i: guint = 0;
    let mut j: guint = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !locale.is_null() {
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
            b"locale != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    mask = safe_c2rust_explode_locale(
        locale,
        &raw mut language,
        &raw mut territory,
        &raw mut codeset,
        &raw mut modifier,
    );
    j = 0 as guint;
    while j <= mask {
        i = mask.wrapping_sub(j);
        if i & !mask == 0 as guint {
            let mut val: *mut gchar = g_strconcat(
                language,
                if i & COMPONENT_TERRITORY as ::core::ffi::c_int as guint != 0 {
                    territory as *const gchar
                } else {
                    b"\0" as *const u8 as *const gchar
                },
                if i & COMPONENT_CODESET as ::core::ffi::c_int as guint != 0 {
                    codeset as *const gchar
                } else {
                    b"\0" as *const u8 as *const gchar
                },
                if i & COMPONENT_MODIFIER as ::core::ffi::c_int as guint != 0 {
                    modifier as *const gchar
                } else {
                    b"\0" as *const u8 as *const gchar
                },
                NULL,
            );
            g_ptr_array_add(array, val as gpointer);
        }
        j = j.wrapping_add(1);
    }
    g_free(language as gpointer);
    if mask & COMPONENT_CODESET as ::core::ffi::c_int as guint != 0 {
        g_free(codeset as gpointer);
    }
    if mask & COMPONENT_TERRITORY as ::core::ffi::c_int as guint != 0 {
        g_free(territory as gpointer);
    }
    if mask & COMPONENT_MODIFIER as ::core::ffi::c_int as guint != 0 {
        g_free(modifier as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_locale_variants(
    mut locale: *const gchar,
) -> *mut *mut gchar {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !locale.is_null() {
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
            b"locale != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    array = g_ptr_array_sized_new(8 as guint);
    safe_c2rust_append_locale_variants(array, locale);
    g_ptr_array_add(array, NULL);
    return g_ptr_array_free(array, FALSE) as *mut *mut gchar;
}
unsafe extern "C" fn safe_c2rust_guess_category_value(
    mut category_name: *const gchar,
) -> *const gchar {
    let mut retval: *const gchar = ::core::ptr::null::<gchar>();
    retval = g_getenv(b"LANGUAGE\0" as *const u8 as *const gchar);
    if !retval.is_null()
        && *retval.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        return retval;
    }
    retval = g_getenv(b"LC_ALL\0" as *const u8 as *const gchar);
    if !retval.is_null()
        && *retval.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        return retval;
    }
    retval = g_getenv(category_name);
    if !retval.is_null()
        && *retval.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        return retval;
    }
    retval = g_getenv(b"LANG\0" as *const u8 as *const gchar);
    if !retval.is_null()
        && *retval.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        return retval;
    }
    return ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn safe_c2rust_language_names_cache_free(mut data: gpointer) {
    let mut cache: *mut GLanguageNamesCache = data as *mut GLanguageNamesCache;
    g_free((*cache).languages as gpointer);
    g_strfreev((*cache).language_names);
    g_free(cache as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_language_names() -> *const *const gchar {
    return safe_c2rust_g_get_language_names_with_category(
        b"LC_MESSAGES\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_language_names_with_category(
    mut category_name: *const gchar,
) -> *const *const gchar {
    static mut safe_c2rust_cache_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL,
            notify: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GHashTable) -> ()>,
                GDestroyNotify,
            >(Some(
                g_hash_table_unref as unsafe extern "C" fn(*mut GHashTable) -> (),
            )),
            future: [NULL, NULL],
        }
    };
    let mut cache: *mut GHashTable =
        g_private_get(&raw mut safe_c2rust_cache_private) as *mut GHashTable;
    let mut languages: *const gchar = ::core::ptr::null::<gchar>();
    let mut name_cache: *mut GLanguageNamesCache = ::core::ptr::null_mut::<GLanguageNamesCache>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !category_name.is_null() {
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
            b"category_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<*const gchar>();
    }
    if cache.is_null() {
        cache = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(safe_c2rust_language_names_cache_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_private_set(&raw mut safe_c2rust_cache_private, cache as gpointer);
    }
    languages = safe_c2rust_guess_category_value(category_name);
    if languages.is_null() {
        languages = b"C\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    name_cache =
        g_hash_table_lookup(cache, category_name as gconstpointer) as *mut GLanguageNamesCache;
    if !(!name_cache.is_null()
        && !(*name_cache).languages.is_null()
        && strcmp(
            (*name_cache).languages,
            languages as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
    {
        let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
        let mut alist: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut a: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        g_hash_table_remove(cache, category_name as gconstpointer);
        array = g_ptr_array_sized_new(8 as guint);
        alist = g_strsplit(languages, b":\0" as *const u8 as *const gchar, 0 as gint);
        a = alist;
        while !(*a).is_null() {
            safe_c2rust_append_locale_variants(array, safe_c2rust_unalias_lang(*a));
            a = a.offset(1);
        }
        g_strfreev(alist);
        g_ptr_array_add(
            array,
            safe_c2rust_g_strdup_inline(b"C\0" as *const u8 as *const ::core::ffi::c_char)
                as gpointer,
        );
        g_ptr_array_add(array, NULL);
        name_cache = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GLanguageNamesCache>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut GLanguageNamesCache;
        (*name_cache).languages =
            safe_c2rust_g_strdup_inline(languages as *const ::core::ffi::c_char) as *mut gchar;
        (*name_cache).language_names = g_ptr_array_free(array, FALSE) as *mut *mut gchar;
        g_hash_table_insert(
            cache,
            safe_c2rust_g_strdup_inline(category_name as *const ::core::ffi::c_char) as gpointer,
            name_cache as gpointer,
        );
    }
    return (*name_cache).language_names as *const *const gchar;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"append_locale_variants\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
