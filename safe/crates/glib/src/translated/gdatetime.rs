extern "C" {
    pub type _GTimeZone;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn nl_langinfo(__item: nl_item) -> *mut ::core::ffi::c_char;
    fn _g_get_time_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_ptr_array_ref(array: *mut GPtrArray) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_time_zone_new_identifier(identifier: *const gchar) -> *mut GTimeZone;
    fn g_time_zone_new_utc() -> *mut GTimeZone;
    fn g_time_zone_new_local() -> *mut GTimeZone;
    fn g_time_zone_ref(tz: *mut GTimeZone) -> *mut GTimeZone;
    fn g_time_zone_unref(tz: *mut GTimeZone);
    fn g_time_zone_find_interval(tz: *mut GTimeZone, type_0: GTimeType, time_: gint64) -> gint;
    fn g_time_zone_adjust_time(tz: *mut GTimeZone, type_0: GTimeType, time_: *mut gint64) -> gint;
    fn g_time_zone_get_abbreviation(tz: *mut GTimeZone, interval: gint) -> *const gchar;
    fn g_time_zone_get_offset(tz: *mut GTimeZone, interval: gint) -> gint32;
    fn g_time_zone_is_dst(tz: *mut GTimeZone, interval: gint) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_stpcpy(dest: *mut gchar, src: *const ::core::ffi::c_char) -> *mut gchar;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_strup(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
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
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn _g_time_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn _g_ctype_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn _g_era_date_compare(date1: *const GEraDate, date2: *const GEraDate) -> ::core::ffi::c_int;
    fn _g_era_description_parse(desc: *const ::core::ffi::c_char) -> *mut GPtrArray;
    fn _g_era_description_segment_ref(
        segment: *mut GEraDescriptionSegment,
    ) -> *mut GEraDescriptionSegment;
    fn _g_era_description_segment_unref(segment: *mut GEraDescriptionSegment);
    fn glib_pgettext(msgctxtid: *const gchar, msgidoffset: gsize) -> *const gchar;
}
pub type size_t = usize;
pub type nl_item = ::core::ffi::c_int;
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
pub const CODESET: C2RustUnnamed = 14;
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
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
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
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gintptr = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
pub type gatomicrefcount = gint;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GTimeZone = _GTimeZone;
pub type GTimeType = ::core::ffi::c_uint;
pub const G_TIME_TYPE_UNIVERSAL: GTimeType = 2;
pub const G_TIME_TYPE_DAYLIGHT: GTimeType = 1;
pub const G_TIME_TYPE_STANDARD: GTimeType = 0;
pub type GTimeSpan = gint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDateTime {
    pub usec: guint64,
    pub tz: *mut GTimeZone,
    pub interval: gint,
    pub days: gint32,
    pub ref_count: gint,
}
pub type GDateTime = _GDateTime;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type gunichar = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GEraDescriptionSegment {
    pub ref_count: gatomicrefcount,
    pub direction_multiplier: ::core::ffi::c_int,
    pub offset: guint64,
    pub start_date: GEraDate,
    pub end_date: GEraDate,
    pub era_name: *mut ::core::ffi::c_char,
    pub era_format: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GEraDate {
    pub type_0: C2RustUnnamed_0,
    pub year: ::core::ffi::c_int,
    pub month: ::core::ffi::c_int,
    pub day: ::core::ffi::c_int,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_ERA_DATE_MINUS_INFINITY: C2RustUnnamed_0 = 2;
pub const G_ERA_DATE_PLUS_INFINITY: C2RustUnnamed_0 = 1;
pub const G_ERA_DATE_SET: C2RustUnnamed_0 = 0;
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
pub const LC_CTYPE: ::core::ffi::c_int = __LC_CTYPE;
pub const LC_TIME: ::core::ffi::c_int = __LC_TIME;
pub const G_MININT64: gint64 = -G_MAXINT64 - 1 as ::core::ffi::c_long;
pub const G_MAXINT64: ::core::ffi::c_long = 0x7fffffffffffffff as ::core::ffi::c_long;
pub const G_MAXUINT64: ::core::ffi::c_ulong = 0xffffffffffffffff as ::core::ffi::c_ulong;
pub const G_TIME_SPAN_SECOND: ::core::ffi::c_long = 1000000 as ::core::ffi::c_long;
pub const UNIX_EPOCH_START: ::core::ffi::c_int = 719163 as ::core::ffi::c_int;
pub const DAYS_IN_4YEARS: ::core::ffi::c_int = 1461 as ::core::ffi::c_int;
pub const DAYS_IN_100YEARS: ::core::ffi::c_int = 36524 as ::core::ffi::c_int;
pub const DAYS_IN_400YEARS: ::core::ffi::c_int = 146097 as ::core::ffi::c_int;
pub const USEC_PER_SECOND: ::core::ffi::c_long = 1000000 as ::core::ffi::c_long;
pub const USEC_PER_MINUTE: ::core::ffi::c_long = 60000000 as ::core::ffi::c_long;
pub const USEC_PER_HOUR: ::core::ffi::c_long = 3600000000 as ::core::ffi::c_long;
pub const USEC_PER_DAY: ::core::ffi::c_long = 86400000000 as ::core::ffi::c_long;
pub const SEC_PER_DAY: ::core::ffi::c_long = 86400 as ::core::ffi::c_long;
pub const SECS_PER_MINUTE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const SECS_PER_HOUR: ::core::ffi::c_int = 60 as ::core::ffi::c_int * SECS_PER_MINUTE;
static mut safe_c2rust_days_in_months: [[guint16; 13]; 2] = [
    [
        0 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        28 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
    ],
    [
        0 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        29 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        30 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
    ],
];
static mut safe_c2rust_days_in_year: [[guint16; 13]; 2] = [
    [
        0 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        59 as ::core::ffi::c_int as guint16,
        90 as ::core::ffi::c_int as guint16,
        120 as ::core::ffi::c_int as guint16,
        151 as ::core::ffi::c_int as guint16,
        181 as ::core::ffi::c_int as guint16,
        212 as ::core::ffi::c_int as guint16,
        243 as ::core::ffi::c_int as guint16,
        273 as ::core::ffi::c_int as guint16,
        304 as ::core::ffi::c_int as guint16,
        334 as ::core::ffi::c_int as guint16,
        365 as ::core::ffi::c_int as guint16,
    ],
    [
        0 as ::core::ffi::c_int as guint16,
        31 as ::core::ffi::c_int as guint16,
        60 as ::core::ffi::c_int as guint16,
        91 as ::core::ffi::c_int as guint16,
        121 as ::core::ffi::c_int as guint16,
        152 as ::core::ffi::c_int as guint16,
        182 as ::core::ffi::c_int as guint16,
        213 as ::core::ffi::c_int as guint16,
        244 as ::core::ffi::c_int as guint16,
        274 as ::core::ffi::c_int as guint16,
        305 as ::core::ffi::c_int as guint16,
        335 as ::core::ffi::c_int as guint16,
        366 as ::core::ffi::c_int as guint16,
    ],
];
pub const GET_AMPM_IS_LOCALE: ::core::ffi::c_int = TRUE;
static mut safe_c2rust_weekday_item: [[gint; 7]; 2] = [
    [
        ABDAY_2 as ::core::ffi::c_int,
        ABDAY_3 as ::core::ffi::c_int,
        ABDAY_4 as ::core::ffi::c_int,
        ABDAY_5 as ::core::ffi::c_int,
        ABDAY_6 as ::core::ffi::c_int,
        ABDAY_7 as ::core::ffi::c_int,
        ABDAY_1 as ::core::ffi::c_int,
    ],
    [
        DAY_2 as ::core::ffi::c_int,
        DAY_3 as ::core::ffi::c_int,
        DAY_4 as ::core::ffi::c_int,
        DAY_5 as ::core::ffi::c_int,
        DAY_6 as ::core::ffi::c_int,
        DAY_7 as ::core::ffi::c_int,
        DAY_1 as ::core::ffi::c_int,
    ],
];
static mut safe_c2rust_month_item: [[gint; 12]; 2] = [
    [
        ABMON_1 as ::core::ffi::c_int,
        ABMON_2 as ::core::ffi::c_int,
        ABMON_3 as ::core::ffi::c_int,
        ABMON_4 as ::core::ffi::c_int,
        ABMON_5 as ::core::ffi::c_int,
        ABMON_6 as ::core::ffi::c_int,
        ABMON_7 as ::core::ffi::c_int,
        ABMON_8 as ::core::ffi::c_int,
        ABMON_9 as ::core::ffi::c_int,
        ABMON_10 as ::core::ffi::c_int,
        ABMON_11 as ::core::ffi::c_int,
        ABMON_12 as ::core::ffi::c_int,
    ],
    [
        MON_1 as ::core::ffi::c_int,
        MON_2 as ::core::ffi::c_int,
        MON_3 as ::core::ffi::c_int,
        MON_4 as ::core::ffi::c_int,
        MON_5 as ::core::ffi::c_int,
        MON_6 as ::core::ffi::c_int,
        MON_7 as ::core::ffi::c_int,
        MON_8 as ::core::ffi::c_int,
        MON_9 as ::core::ffi::c_int,
        MON_10 as ::core::ffi::c_int,
        MON_11 as ::core::ffi::c_int,
        MON_12 as ::core::ffi::c_int,
    ],
];
pub const WEEKDAY_ABBR_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const WEEKDAY_FULL_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const MONTH_ABBR_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const MONTH_FULL_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const MONTH_FULL_WITH_DAY_IS_LOCALE: ::core::ffi::c_int = MONTH_FULL_IS_LOCALE;
static mut safe_c2rust_alt_month_item: [gint; 12] = [
    __ALTMON_1 as ::core::ffi::c_int,
    __ALTMON_2 as ::core::ffi::c_int,
    __ALTMON_3 as ::core::ffi::c_int,
    __ALTMON_4 as ::core::ffi::c_int,
    __ALTMON_5 as ::core::ffi::c_int,
    __ALTMON_6 as ::core::ffi::c_int,
    __ALTMON_7 as ::core::ffi::c_int,
    __ALTMON_8 as ::core::ffi::c_int,
    __ALTMON_9 as ::core::ffi::c_int,
    __ALTMON_10 as ::core::ffi::c_int,
    __ALTMON_11 as ::core::ffi::c_int,
    __ALTMON_12 as ::core::ffi::c_int,
];
pub const MONTH_FULL_STANDALONE_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const MONTH_ABBR_WITH_DAY_IS_LOCALE: ::core::ffi::c_int = MONTH_ABBR_IS_LOCALE;
static mut safe_c2rust_ab_alt_month_item: [gint; 12] = [
    _NL_ABALTMON_1 as ::core::ffi::c_int,
    _NL_ABALTMON_2 as ::core::ffi::c_int,
    _NL_ABALTMON_3 as ::core::ffi::c_int,
    _NL_ABALTMON_4 as ::core::ffi::c_int,
    _NL_ABALTMON_5 as ::core::ffi::c_int,
    _NL_ABALTMON_6 as ::core::ffi::c_int,
    _NL_ABALTMON_7 as ::core::ffi::c_int,
    _NL_ABALTMON_8 as ::core::ffi::c_int,
    _NL_ABALTMON_9 as ::core::ffi::c_int,
    _NL_ABALTMON_10 as ::core::ffi::c_int,
    _NL_ABALTMON_11 as ::core::ffi::c_int,
    _NL_ABALTMON_12 as ::core::ffi::c_int,
];
pub const MONTH_ABBR_STANDALONE_IS_LOCALE: ::core::ffi::c_int = TRUE;
pub const ERA_DESCRIPTION_IS_LOCALE: ::core::ffi::c_int = TRUE;
unsafe extern "C" fn safe_c2rust_get_fallback_ampm(mut hour: gint) -> *const gchar {
    if hour < 12 as ::core::ffi::c_int {
        return glib_pgettext(
            b"GDateTime\x04AM\0" as *const u8 as *const gchar,
            (strlen(b"GDateTime\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                .wrapping_add(1 as gsize),
        );
    } else {
        return glib_pgettext(
            b"GDateTime\x04PM\0" as *const u8 as *const gchar,
            (strlen(b"GDateTime\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                .wrapping_add(1 as gsize),
        );
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_ymd_to_days(
    mut year: gint,
    mut month: gint,
    mut day: gint,
) -> gint {
    let mut days: gint64 = 0;
    days = (year as gint64 - 1 as gint64) * 365 as gint64
        + ((year as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int)
            as gint64
        - ((year as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int)
            as gint64
        + ((year as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 400 as ::core::ffi::c_int)
            as gint64;
    days += safe_c2rust_days_in_year[0 as ::core::ffi::c_int as usize]
        [(month as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] as gint64;
    if year as ::core::ffi::c_int % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        && month > 2 as ::core::ffi::c_int
    {
        day += 1;
    }
    days += day as gint64;
    return days as gint;
}
unsafe extern "C" fn safe_c2rust_g_date_time_get_week_number(
    mut datetime: *mut GDateTime,
    mut week_number: *mut gint,
    mut day_of_week: *mut gint,
    mut day_of_year: *mut gint,
) {
    let mut a: gint = 0;
    let mut b: gint = 0;
    let mut c: gint = 0;
    let mut d: gint = 0;
    let mut e: gint = 0;
    let mut f: gint = 0;
    let mut g: gint = 0;
    let mut n: gint = 0;
    let mut s: gint = 0;
    let mut month: gint = -(1 as gint);
    let mut day: gint = -(1 as gint);
    let mut year: gint = -(1 as gint);
    safe_c2rust_g_date_time_get_ymd(datetime, &raw mut year, &raw mut month, &raw mut day);
    if month <= 2 as ::core::ffi::c_int {
        a = (safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as gint;
        b = (a as ::core::ffi::c_int / 4 as ::core::ffi::c_int
            - a as ::core::ffi::c_int / 100 as ::core::ffi::c_int
            + a as ::core::ffi::c_int / 400 as ::core::ffi::c_int) as gint;
        c = ((a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
            - (a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int
            + (a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 400 as ::core::ffi::c_int)
            as gint;
        s = b - c;
        e = 0 as ::core::ffi::c_int as gint;
        f = (day as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            + 31 as ::core::ffi::c_int * (month as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            as gint;
    } else {
        a = year;
        b = (a as ::core::ffi::c_int / 4 as ::core::ffi::c_int
            - a as ::core::ffi::c_int / 100 as ::core::ffi::c_int
            + a as ::core::ffi::c_int / 400 as ::core::ffi::c_int) as gint;
        c = ((a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
            - (a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int
            + (a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 400 as ::core::ffi::c_int)
            as gint;
        s = b - c;
        e = (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
        f = day + (153 as gint * (month - 3 as gint) + 2 as gint) / 5 as gint + 58 as gint + s;
    }
    g = ((a as ::core::ffi::c_int + b as ::core::ffi::c_int) % 7 as ::core::ffi::c_int) as gint;
    d = ((f as ::core::ffi::c_int + g as ::core::ffi::c_int - e as ::core::ffi::c_int)
        % 7 as ::core::ffi::c_int) as gint;
    n = f + 3 as gint - d;
    if !week_number.is_null() {
        if n < 0 as ::core::ffi::c_int {
            *week_number = (53 as ::core::ffi::c_int
                - (g as ::core::ffi::c_int - s as ::core::ffi::c_int) / 5 as ::core::ffi::c_int)
                as gint;
        } else if n > 364 as gint + s {
            *week_number = 1 as ::core::ffi::c_int as gint;
        } else {
            *week_number = (n as ::core::ffi::c_int / 7 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as gint;
        }
    }
    if !day_of_week.is_null() {
        *day_of_week = (d as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
    }
    if !day_of_year.is_null() {
        *day_of_year = (f as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
    }
}
unsafe extern "C" fn safe_c2rust_g_date_time_alloc(mut tz: *mut GTimeZone) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    datetime = ({
        let mut __s: gsize = ::core::mem::size_of::<GDateTime>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GDateTime;
    (*datetime).tz = g_time_zone_ref(tz);
    (*datetime).ref_count = 1 as ::core::ffi::c_int as gint;
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_ref(
    mut datetime: *mut GDateTime,
) -> *mut GDateTime {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*datetime).ref_count > 0 as ::core::ffi::c_int {
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
            b"datetime->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*datetime).ref_count;
        (*datetime).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*datetime).ref_count, 1 as ::core::ffi::c_int);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_unref(mut datetime: *mut GDateTime) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*datetime).ref_count > 0 as ::core::ffi::c_int {
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
            b"datetime->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*datetime).ref_count;
            (*datetime).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*datetime).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_time_zone_unref((*datetime).tz);
        g_slice_free1(
            ::core::mem::size_of::<GDateTime>() as gsize,
            datetime as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_date_time_to_instant(mut datetime: *mut GDateTime) -> gint64 {
    let mut offset: gint64 = 0;
    offset = g_time_zone_get_offset((*datetime).tz, (*datetime).interval) as gint64;
    offset *= USEC_PER_SECOND;
    return (((*datetime).days as ::core::ffi::c_long * USEC_PER_DAY) as guint64)
        .wrapping_add((*datetime).usec)
        .wrapping_sub(offset as guint64) as gint64;
}
unsafe extern "C" fn safe_c2rust_g_date_time_from_instant(
    mut tz: *mut GTimeZone,
    mut instant: gint64,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut offset: gint64 = 0;
    if instant < 0 as gint64 || instant > 1000000000000000000 as ::core::ffi::c_long {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    datetime = safe_c2rust_g_date_time_alloc(tz);
    (*datetime).interval = g_time_zone_find_interval(
        tz,
        G_TIME_TYPE_UNIVERSAL,
        instant / USEC_PER_SECOND - UNIX_EPOCH_START as gint64 * SEC_PER_DAY,
    );
    offset = g_time_zone_get_offset((*datetime).tz, (*datetime).interval) as gint64;
    offset *= USEC_PER_SECOND;
    instant += offset;
    (*datetime).days = (instant as ::core::ffi::c_long / USEC_PER_DAY) as gint32;
    (*datetime).usec = (instant as ::core::ffi::c_long % USEC_PER_DAY) as guint64;
    if (*datetime).days < 1 as ::core::ffi::c_int
        || (3652059 as ::core::ffi::c_int) < (*datetime).days
    {
        safe_c2rust_g_date_time_unref(datetime);
        datetime = ::core::ptr::null_mut::<GDateTime>();
    }
    return datetime;
}
unsafe extern "C" fn safe_c2rust_g_date_time_deal_with_date_change(
    mut datetime: *mut GDateTime,
) -> gboolean {
    let mut was_dst: GTimeType = G_TIME_TYPE_STANDARD;
    let mut full_time: gint64 = 0;
    let mut usec: gint64 = 0;
    if (*datetime).days < 1 as ::core::ffi::c_int
        || (*datetime).days > 3652059 as ::core::ffi::c_int
    {
        return FALSE;
    }
    was_dst = g_time_zone_is_dst((*datetime).tz, (*datetime).interval) as GTimeType;
    full_time = (((*datetime).days as ::core::ffi::c_long * USEC_PER_DAY) as guint64)
        .wrapping_add((*datetime).usec) as gint64;
    usec = (full_time as ::core::ffi::c_long % USEC_PER_SECOND) as gint64;
    full_time /= USEC_PER_SECOND;
    full_time -= UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY;
    (*datetime).interval = g_time_zone_adjust_time((*datetime).tz, was_dst, &raw mut full_time);
    full_time += UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY;
    full_time *= USEC_PER_SECOND;
    full_time += usec;
    (*datetime).days = (full_time as ::core::ffi::c_long / USEC_PER_DAY) as gint32;
    (*datetime).usec = (full_time as ::core::ffi::c_long % USEC_PER_DAY) as guint64;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_date_time_replace_days(
    mut datetime: *mut GDateTime,
    mut days: gint,
) -> *mut GDateTime {
    let mut new: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    new = safe_c2rust_g_date_time_alloc((*datetime).tz);
    (*new).interval = (*datetime).interval;
    (*new).usec = (*datetime).usec;
    (*new).days = days as gint32;
    if safe_c2rust_g_date_time_deal_with_date_change(new) == 0 {
        safe_c2rust_g_date_time_unref(new);
        new = ::core::ptr::null_mut::<GDateTime>();
    }
    return new;
}
unsafe extern "C" fn safe_c2rust_g_date_time_new_from_timeval(
    mut tz: *mut GTimeZone,
    mut tv: *const GTimeVal,
) -> *mut GDateTime {
    let mut tv_sec: gint64 = (*tv).tv_sec as gint64;
    if tv_sec > G_MAXINT64 - 1 as ::core::ffi::c_long
        || !(tv_sec + 1 as gint64
            <= 0x7fffffffffffffff as ::core::ffi::c_long / USEC_PER_SECOND
                - UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY)
    {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_from_instant(
        tz,
        (*tv).tv_usec as gint64
            + ((*tv).tv_sec + UNIX_EPOCH_START as gint64 * SEC_PER_DAY) * USEC_PER_SECOND,
    );
}
unsafe extern "C" fn safe_c2rust_g_date_time_new_from_unix(
    mut tz: *mut GTimeZone,
    mut usecs: gint64,
) -> *mut GDateTime {
    if !(usecs
        <= 0x7fffffffffffffff as ::core::ffi::c_long
            - UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY * USEC_PER_SECOND)
    {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_from_instant(
        tz,
        usecs + UNIX_EPOCH_START as gint64 * SEC_PER_DAY * USEC_PER_SECOND,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_now(mut tz: *mut GTimeZone) -> *mut GDateTime {
    let mut now_us: gint64 = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !tz.is_null() {
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
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    now_us = g_get_real_time();
    return safe_c2rust_g_date_time_new_from_unix(tz, now_us);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_now_local() -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut local: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    local = g_time_zone_new_local();
    datetime = safe_c2rust_g_date_time_new_now(local);
    g_time_zone_unref(local);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_now_utc() -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut utc: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    utc = g_time_zone_new_utc();
    datetime = safe_c2rust_g_date_time_new_now(utc);
    g_time_zone_unref(utc);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_unix_local(
    mut t: gint64,
) -> *mut GDateTime {
    if t > G_MAXINT64 / USEC_PER_SECOND || t < G_MININT64 / USEC_PER_SECOND {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_new_from_unix_local_usec(t * USEC_PER_SECOND);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_unix_local_usec(
    mut usecs: gint64,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut local: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    local = g_time_zone_new_local();
    datetime = safe_c2rust_g_date_time_new_from_unix(local, usecs);
    g_time_zone_unref(local);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_unix_utc(
    mut t: gint64,
) -> *mut GDateTime {
    if t > G_MAXINT64 / USEC_PER_SECOND || t < G_MININT64 / USEC_PER_SECOND {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_new_from_unix_utc_usec(t * USEC_PER_SECOND);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_unix_utc_usec(
    mut usecs: gint64,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut utc: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    utc = g_time_zone_new_utc();
    datetime = safe_c2rust_g_date_time_new_from_unix(utc, usecs);
    g_time_zone_unref(utc);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_timeval_local(
    mut tv: *const GTimeVal,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut local: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    local = g_time_zone_new_local();
    datetime = safe_c2rust_g_date_time_new_from_timeval(local, tv);
    g_time_zone_unref(local);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_timeval_utc(
    mut tv: *const GTimeVal,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut utc: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    utc = g_time_zone_new_utc();
    datetime = safe_c2rust_g_date_time_new_from_timeval(utc, tv);
    g_time_zone_unref(utc);
    return datetime;
}
unsafe extern "C" fn safe_c2rust_get_iso8601_int(
    mut text: *const gchar,
    mut length: gsize,
    mut value: *mut gint,
) -> gboolean {
    let mut i: gsize = 0;
    let mut v: guint = 0 as guint;
    if length < 1 as gsize || length > 4 as gsize {
        return FALSE;
    }
    i = 0 as gsize;
    while i < length {
        let c: gchar = *text.offset(i as isize);
        if (c as ::core::ffi::c_int) < '0' as i32 || c as ::core::ffi::c_int > '9' as i32 {
            return FALSE;
        }
        v = v
            .wrapping_mul(10 as guint)
            .wrapping_add((c as ::core::ffi::c_int - '0' as i32) as guint);
        i = i.wrapping_add(1);
    }
    *value = v as gint;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_iso8601_seconds(
    mut text: *const gchar,
    mut length: gsize,
    mut value: *mut gdouble,
) -> gboolean {
    let mut i: gsize = 0;
    let mut divisor: guint64 = 1 as guint64;
    let mut v: guint64 = 0 as guint64;
    if length < 2 as gsize {
        return FALSE;
    }
    i = 0 as gsize;
    while i < 2 as gsize {
        let c: gchar = *text.offset(i as isize);
        if (c as ::core::ffi::c_int) < '0' as i32 || c as ::core::ffi::c_int > '9' as i32 {
            return FALSE;
        }
        v = v
            .wrapping_mul(10 as guint64)
            .wrapping_add((c as ::core::ffi::c_int - '0' as i32) as guint64);
        i = i.wrapping_add(1);
    }
    if length > 2 as gsize
        && !(*text.offset(i as isize) as ::core::ffi::c_int == '.' as i32
            || *text.offset(i as isize) as ::core::ffi::c_int == ',' as i32)
    {
        return FALSE;
    }
    if v as ::core::ffi::c_double >= 60.0f64 && v as ::core::ffi::c_double <= 61.0f64 {
        v = 59.0f64 as guint64;
    }
    i = i.wrapping_add(1);
    if i == length {
        return FALSE;
    }
    while i < length {
        let c_0: gchar = *text.offset(i as isize);
        if (c_0 as ::core::ffi::c_int) < '0' as i32
            || c_0 as ::core::ffi::c_int > '9' as i32
            || v > G_MAXUINT64
                .wrapping_sub((c_0 as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_ulong)
                .wrapping_div(10 as ::core::ffi::c_ulong)
            || divisor > G_MAXUINT64.wrapping_div(10 as ::core::ffi::c_ulong)
        {
            return FALSE;
        }
        v = v
            .wrapping_mul(10 as guint64)
            .wrapping_add((c_0 as ::core::ffi::c_int - '0' as i32) as guint64);
        divisor = divisor.wrapping_mul(10 as guint64);
        i = i.wrapping_add(1);
    }
    *value = v as gdouble / divisor as gdouble;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_date_time_new_ordinal(
    mut tz: *mut GTimeZone,
    mut year: gint,
    mut ordinal_day: gint,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ordinal_day < 1 as ::core::ffi::c_int
        || ordinal_day
            > (if year as ::core::ffi::c_int % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int)
            {
                366 as ::core::ffi::c_int
            } else {
                365 as ::core::ffi::c_int
            })
    {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    dt = safe_c2rust_g_date_time_new(tz, year, 1 as gint, 1 as gint, hour, minute, seconds);
    if dt.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    (*dt).days += ordinal_day as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    return dt;
}
unsafe extern "C" fn safe_c2rust_g_date_time_new_week(
    mut tz: *mut GTimeZone,
    mut year: gint,
    mut week: gint,
    mut week_day: gint,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut p: gint64 = 0;
    let mut max_week: gint = 0;
    let mut jan4_week_day: gint = 0;
    let mut ordinal_day: gint = 0;
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    p = ((year as ::core::ffi::c_int * 365 as ::core::ffi::c_int
        + year as ::core::ffi::c_int / 4 as ::core::ffi::c_int
        - year as ::core::ffi::c_int / 100 as ::core::ffi::c_int
        + year as ::core::ffi::c_int / 400 as ::core::ffi::c_int)
        % 7 as ::core::ffi::c_int) as gint64;
    max_week = (if p == 4 as gint64 {
        53 as ::core::ffi::c_int
    } else {
        52 as ::core::ffi::c_int
    }) as gint;
    if week < 1 as ::core::ffi::c_int
        || week > max_week
        || week_day < 1 as ::core::ffi::c_int
        || week_day > 7 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    dt = safe_c2rust_g_date_time_new(
        tz,
        year,
        1 as gint,
        4 as gint,
        0 as gint,
        0 as gint,
        0 as ::core::ffi::c_int as gdouble,
    );
    if dt.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    safe_c2rust_g_date_time_get_week_number(
        dt,
        ::core::ptr::null_mut::<gint>(),
        &raw mut jan4_week_day,
        ::core::ptr::null_mut::<gint>(),
    );
    safe_c2rust_g_date_time_unref(dt);
    ordinal_day =
        (week as ::core::ffi::c_int * 7 as ::core::ffi::c_int + week_day as ::core::ffi::c_int
            - (jan4_week_day as ::core::ffi::c_int + 3 as ::core::ffi::c_int)) as gint;
    if ordinal_day < 0 as ::core::ffi::c_int {
        year -= 1;
        ordinal_day += if year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            366 as ::core::ffi::c_int
        } else {
            365 as ::core::ffi::c_int
        };
    } else if ordinal_day
        > (if year as ::core::ffi::c_int % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            366 as ::core::ffi::c_int
        } else {
            365 as ::core::ffi::c_int
        })
    {
        ordinal_day -= if year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            366 as ::core::ffi::c_int
        } else {
            365 as ::core::ffi::c_int
        };
        year += 1;
    }
    return safe_c2rust_g_date_time_new_ordinal(tz, year, ordinal_day, hour, minute, seconds);
}
unsafe extern "C" fn safe_c2rust_parse_iso8601_date(
    mut text: *const gchar,
    mut length: gsize,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
    mut tz: *mut GTimeZone,
) -> *mut GDateTime {
    if length == 10 as gsize
        && *text.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        && *text.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
    {
        let mut year: ::core::ffi::c_int = 0;
        let mut month: ::core::ffi::c_int = 0;
        let mut day: ::core::ffi::c_int = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(5 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut month,
            ) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(8 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut day,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new(
            tz,
            year as gint,
            month as gint,
            day as gint,
            hour,
            minute,
            seconds,
        );
    } else if length == 8 as gsize
        && *text.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
    {
        let mut year_0: gint = 0;
        let mut ordinal_day: gint = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year_0) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(5 as ::core::ffi::c_int as isize),
                3 as gsize,
                &raw mut ordinal_day,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new_ordinal(tz, year_0, ordinal_day, hour, minute, seconds);
    } else if length == 10 as gsize
        && *text.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        && *text.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'W' as i32
        && *text.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
    {
        let mut year_1: gint = 0;
        let mut week: gint = 0;
        let mut week_day: gint = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year_1) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(6 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut week,
            ) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(9 as ::core::ffi::c_int as isize),
                1 as gsize,
                &raw mut week_day,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new_week(tz, year_1, week, week_day, hour, minute, seconds);
    } else if length == 8 as gsize
        && *text.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'W' as i32
    {
        let mut year_2: gint = 0;
        let mut week_0: gint = 0;
        let mut week_day_0: gint = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year_2) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(5 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut week_0,
            ) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(7 as ::core::ffi::c_int as isize),
                1 as gsize,
                &raw mut week_day_0,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new_week(
            tz, year_2, week_0, week_day_0, hour, minute, seconds,
        );
    } else if length == 8 as gsize {
        let mut year_3: ::core::ffi::c_int = 0;
        let mut month_0: ::core::ffi::c_int = 0;
        let mut day_0: ::core::ffi::c_int = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year_3) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(4 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut month_0,
            ) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(6 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut day_0,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new(
            tz,
            year_3 as gint,
            month_0 as gint,
            day_0 as gint,
            hour,
            minute,
            seconds,
        );
    } else if length == 7 as gsize {
        let mut year_4: gint = 0;
        let mut ordinal_day_0: gint = 0;
        if safe_c2rust_get_iso8601_int(text, 4 as gsize, &raw mut year_4) == 0
            || safe_c2rust_get_iso8601_int(
                text.offset(4 as ::core::ffi::c_int as isize),
                3 as gsize,
                &raw mut ordinal_day_0,
            ) == 0
        {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        return safe_c2rust_g_date_time_new_ordinal(
            tz,
            year_4,
            ordinal_day_0,
            hour,
            minute,
            seconds,
        );
    } else {
        return ::core::ptr::null_mut::<GDateTime>();
    };
}
unsafe extern "C" fn safe_c2rust_parse_iso8601_timezone(
    mut text: *const gchar,
    mut length: gsize,
    mut tz_offset: *mut size_t,
) -> *mut GTimeZone {
    let mut tz_length: size_t = 0;
    let mut offset_hours: gint = 0;
    let mut offset_minutes: gint = 0;
    let mut offset_sign: gint = 1 as gint;
    let mut tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut tz_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if length > 0 as gsize
        && *text.offset(length.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
            == 'Z' as i32
    {
        *tz_offset = length.wrapping_sub(1 as gsize) as size_t;
        return g_time_zone_new_utc();
    }
    tz_length = 1 as size_t;
    while tz_length <= length as size_t {
        if *text.offset((length as size_t).wrapping_sub(tz_length) as isize) as ::core::ffi::c_int
            == '+' as i32
            || *text.offset((length as size_t).wrapping_sub(tz_length) as isize)
                as ::core::ffi::c_int
                == '-' as i32
        {
            offset_sign = (if *text.offset((length as size_t).wrapping_sub(tz_length) as isize)
                as ::core::ffi::c_int
                == '-' as i32
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as gint;
            break;
        } else {
            tz_length = tz_length.wrapping_add(1);
        }
    }
    if tz_length > length as size_t {
        return ::core::ptr::null_mut::<GTimeZone>();
    }
    tz_start =
        text.offset(length as isize).offset(-(tz_length as isize)) as *const ::core::ffi::c_char;
    if tz_length == 6 as size_t
        && *tz_start.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
    {
        if safe_c2rust_get_iso8601_int(
            tz_start.offset(1 as ::core::ffi::c_int as isize),
            2 as gsize,
            &raw mut offset_hours,
        ) == 0
            || safe_c2rust_get_iso8601_int(
                tz_start.offset(4 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut offset_minutes,
            ) == 0
        {
            return ::core::ptr::null_mut::<GTimeZone>();
        }
    } else if tz_length == 5 as size_t {
        if safe_c2rust_get_iso8601_int(
            tz_start.offset(1 as ::core::ffi::c_int as isize),
            2 as gsize,
            &raw mut offset_hours,
        ) == 0
            || safe_c2rust_get_iso8601_int(
                tz_start.offset(3 as ::core::ffi::c_int as isize),
                2 as gsize,
                &raw mut offset_minutes,
            ) == 0
        {
            return ::core::ptr::null_mut::<GTimeZone>();
        }
    } else if tz_length == 3 as size_t {
        if safe_c2rust_get_iso8601_int(
            tz_start.offset(1 as ::core::ffi::c_int as isize),
            2 as gsize,
            &raw mut offset_hours,
        ) == 0
        {
            return ::core::ptr::null_mut::<GTimeZone>();
        }
        offset_minutes = 0 as ::core::ffi::c_int as gint;
    } else {
        return ::core::ptr::null_mut::<GTimeZone>();
    }
    *tz_offset = tz_start.offset_from(text) as ::core::ffi::c_long as size_t;
    tz = g_time_zone_new_identifier(tz_start as *const gchar);
    if tz.is_null()
        || g_time_zone_get_offset(tz, 0 as gint)
            != offset_sign as ::core::ffi::c_int
                * (offset_hours as ::core::ffi::c_int * 3600 as ::core::ffi::c_int
                    + offset_minutes as ::core::ffi::c_int * 60 as ::core::ffi::c_int)
    {
        let mut _pp: *mut *mut GTimeZone = &raw mut tz;
        let mut _ptr: *mut GTimeZone = *_pp;
        *_pp = ::core::ptr::null_mut::<GTimeZone>();
        if !_ptr.is_null() {
            g_time_zone_unref(_ptr as *mut GTimeZone);
        }
        return ::core::ptr::null_mut::<GTimeZone>();
    }
    return tz;
}
unsafe extern "C" fn safe_c2rust_parse_iso8601_time(
    mut text: *const gchar,
    mut length: gsize,
    mut hour: *mut gint,
    mut minute: *mut gint,
    mut seconds: *mut gdouble,
    mut tz: *mut *mut GTimeZone,
) -> gboolean {
    let mut tz_offset: size_t = 0 as size_t;
    *tz = safe_c2rust_parse_iso8601_timezone(text, length, &raw mut tz_offset);
    if !(*tz).is_null() {
        length = tz_offset as gsize;
    }
    if length >= 8 as gsize
        && *text.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
        && *text.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
    {
        return (safe_c2rust_get_iso8601_int(text, 2 as gsize, hour) != 0
            && safe_c2rust_get_iso8601_int(
                text.offset(3 as ::core::ffi::c_int as isize),
                2 as gsize,
                minute,
            ) != 0
            && safe_c2rust_get_iso8601_seconds(
                text.offset(6 as ::core::ffi::c_int as isize),
                length.wrapping_sub(6 as gsize),
                seconds,
            ) != 0) as ::core::ffi::c_int;
    } else if length >= 6 as gsize {
        return (safe_c2rust_get_iso8601_int(text, 2 as gsize, hour) != 0
            && safe_c2rust_get_iso8601_int(
                text.offset(2 as ::core::ffi::c_int as isize),
                2 as gsize,
                minute,
            ) != 0
            && safe_c2rust_get_iso8601_seconds(
                text.offset(4 as ::core::ffi::c_int as isize),
                length.wrapping_sub(4 as gsize),
                seconds,
            ) != 0) as ::core::ffi::c_int;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_from_iso8601(
    mut text: *const gchar,
    mut default_tz: *mut GTimeZone,
) -> *mut GDateTime {
    let mut length: size_t = 0;
    let mut date_length: size_t = 0 as size_t;
    let mut date_length_set: gboolean = FALSE;
    let mut hour: gint = 0 as gint;
    let mut minute: gint = 0 as gint;
    let mut seconds: gdouble = 0.0f64;
    let mut tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !text.is_null() {
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
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    length = 0 as size_t;
    while *text.offset(length as isize) as ::core::ffi::c_int != '\0' as i32 {
        if date_length_set == 0
            && (*text.offset(length as isize) as ::core::ffi::c_int == 'T' as i32
                || *text.offset(length as isize) as ::core::ffi::c_int == 't' as i32
                || *text.offset(length as isize) as ::core::ffi::c_int == ' ' as i32)
        {
            date_length = length;
            date_length_set = TRUE as gboolean;
        }
        length = length.wrapping_add(1);
    }
    if date_length_set == 0 {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if !(safe_c2rust_parse_iso8601_time(
        text.offset(date_length as isize)
            .offset(1 as ::core::ffi::c_int as isize),
        (length as gsize).wrapping_sub((date_length as gsize).wrapping_add(1 as gsize)),
        &raw mut hour,
        &raw mut minute,
        &raw mut seconds,
        &raw mut tz,
    ) == 0)
    {
        if tz.is_null() && default_tz.is_null() {
            return ::core::ptr::null_mut::<GDateTime>();
        }
        datetime = safe_c2rust_parse_iso8601_date(
            text,
            date_length as gsize,
            hour,
            minute,
            seconds,
            if !tz.is_null() { tz } else { default_tz },
        );
    }
    if !tz.is_null() {
        g_time_zone_unref(tz);
    }
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new(
    mut tz: *mut GTimeZone,
    mut year: gint,
    mut month: gint,
    mut day: gint,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut full_time: gint64 = 0;
    let mut usec: gint64 = 0;
    let mut usecd: gdouble = 0.;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !tz.is_null() {
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
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if year < 1 as ::core::ffi::c_int
        || year > 9999 as ::core::ffi::c_int
        || month < 1 as ::core::ffi::c_int
        || month > 12 as ::core::ffi::c_int
        || day < 1 as ::core::ffi::c_int
        || day
            > safe_c2rust_days_in_months[(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int))
                as ::core::ffi::c_int as usize][month as usize] as ::core::ffi::c_int
        || hour < 0 as ::core::ffi::c_int
        || hour > 23 as ::core::ffi::c_int
        || minute < 0 as ::core::ffi::c_int
        || minute > 59 as ::core::ffi::c_int
        || seconds.is_nan() as i32 != 0
        || seconds < 0.0f64
        || seconds >= 60.0f64
    {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    datetime = safe_c2rust_g_date_time_alloc(tz);
    (*datetime).days = safe_c2rust_ymd_to_days(year, month, day) as gint32;
    (*datetime).usec = (hour as gint64 * USEC_PER_HOUR
        + minute as gint64 * USEC_PER_MINUTE
        + (seconds * USEC_PER_SECOND as gdouble) as gint64) as guint64;
    full_time = (SEC_PER_DAY
        * (safe_c2rust_ymd_to_days(year, month, day) as ::core::ffi::c_int - UNIX_EPOCH_START)
            as ::core::ffi::c_long
        + (SECS_PER_HOUR * hour) as ::core::ffi::c_long
        + (SECS_PER_MINUTE * minute) as ::core::ffi::c_long
        + seconds as ::core::ffi::c_int as ::core::ffi::c_long) as gint64;
    (*datetime).interval =
        g_time_zone_adjust_time((*datetime).tz, G_TIME_TYPE_STANDARD, &raw mut full_time);
    ::core::ptr::write_volatile(
        &mut usec as *mut gint64,
        (seconds * USEC_PER_SECOND as gdouble) as gint64,
    );
    ::core::ptr::write_volatile(
        &mut usecd as *mut gdouble,
        ((usec + 1 as gint64) as ::core::ffi::c_double * 1e-6f64) as gdouble,
    );
    if usecd <= seconds {
        ::core::ptr::write_volatile(
            &mut usec as *mut gint64,
            ::core::ptr::read_volatile::<gint64>(&usec as *const gint64) + 1,
        );
    }
    full_time += UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY;
    (*datetime).days = (full_time as ::core::ffi::c_long / SEC_PER_DAY) as gint32;
    (*datetime).usec =
        (full_time as ::core::ffi::c_long % SEC_PER_DAY * USEC_PER_SECOND) as guint64;
    (*datetime).usec = (*datetime)
        .usec
        .wrapping_add((usec as ::core::ffi::c_long % USEC_PER_SECOND) as guint64);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_local(
    mut year: gint,
    mut month: gint,
    mut day: gint,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut local: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    local = g_time_zone_new_local();
    datetime = safe_c2rust_g_date_time_new(local, year, month, day, hour, minute, seconds);
    g_time_zone_unref(local);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_new_utc(
    mut year: gint,
    mut month: gint,
    mut day: gint,
    mut hour: gint,
    mut minute: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut datetime: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut utc: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    utc = g_time_zone_new_utc();
    datetime = safe_c2rust_g_date_time_new(utc, year, month, day, hour, minute, seconds);
    g_time_zone_unref(utc);
    return datetime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add(
    mut datetime: *mut GDateTime,
    mut timespan: GTimeSpan,
) -> *mut GDateTime {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_from_instant(
        (*datetime).tz,
        timespan as gint64 + safe_c2rust_g_date_time_to_instant(datetime),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_years(
    mut datetime: *mut GDateTime,
    mut years: gint,
) -> *mut GDateTime {
    let mut year: gint = 0;
    let mut month: gint = 0;
    let mut day: gint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if years < -(10000 as ::core::ffi::c_int) || years > 10000 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    safe_c2rust_g_date_time_get_ymd(datetime, &raw mut year, &raw mut month, &raw mut day);
    year += years;
    if month == 2 as ::core::ffi::c_int
        && day == 29 as ::core::ffi::c_int
        && !(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int))
    {
        day = 28 as ::core::ffi::c_int as gint;
    }
    return safe_c2rust_g_date_time_replace_days(
        datetime,
        safe_c2rust_ymd_to_days(year, month, day),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_months(
    mut datetime: *mut GDateTime,
    mut months: gint,
) -> *mut GDateTime {
    let mut year: gint = 0;
    let mut month: gint = 0;
    let mut day: gint = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    safe_c2rust_g_date_time_get_ymd(datetime, &raw mut year, &raw mut month, &raw mut day);
    if months < -(120000 as ::core::ffi::c_int) || months > 120000 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    year += months as ::core::ffi::c_int / 12 as ::core::ffi::c_int;
    month += months as ::core::ffi::c_int % 12 as ::core::ffi::c_int;
    if month < 1 as ::core::ffi::c_int {
        month += 12 as ::core::ffi::c_int;
        year -= 1;
    } else if month > 12 as ::core::ffi::c_int {
        month -= 12 as ::core::ffi::c_int;
        year += 1;
    }
    day = (if day
        < safe_c2rust_days_in_months[(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as usize][month as usize] as ::core::ffi::c_int
    {
        day as ::core::ffi::c_int
    } else {
        safe_c2rust_days_in_months[(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as usize][month as usize] as ::core::ffi::c_int
    }) as gint;
    return safe_c2rust_g_date_time_replace_days(
        datetime,
        safe_c2rust_ymd_to_days(year, month, day),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_weeks(
    mut datetime: *mut GDateTime,
    mut weeks: gint,
) -> *mut GDateTime {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_add_days(datetime, weeks * 7 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_days(
    mut datetime: *mut GDateTime,
    mut days: gint,
) -> *mut GDateTime {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if days < -(3660000 as ::core::ffi::c_int) || days > 3660000 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_replace_days(datetime, (*datetime).days as gint + days);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_hours(
    mut datetime: *mut GDateTime,
    mut hours: gint,
) -> *mut GDateTime {
    return safe_c2rust_g_date_time_add(datetime, hours as GTimeSpan * USEC_PER_HOUR);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_minutes(
    mut datetime: *mut GDateTime,
    mut minutes: gint,
) -> *mut GDateTime {
    return safe_c2rust_g_date_time_add(datetime, minutes as GTimeSpan * USEC_PER_MINUTE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_seconds(
    mut datetime: *mut GDateTime,
    mut seconds: gdouble,
) -> *mut GDateTime {
    return safe_c2rust_g_date_time_add(
        datetime,
        (seconds * USEC_PER_SECOND as gdouble) as GTimeSpan,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_add_full(
    mut datetime: *mut GDateTime,
    mut years: gint,
    mut months: gint,
    mut days: gint,
    mut hours: gint,
    mut minutes: gint,
    mut seconds: gdouble,
) -> *mut GDateTime {
    let mut year: gint = 0;
    let mut month: gint = 0;
    let mut day: gint = 0;
    let mut full_time: gint64 = 0;
    let mut new: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut interval: gint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    safe_c2rust_g_date_time_get_ymd(datetime, &raw mut year, &raw mut month, &raw mut day);
    months += years as ::core::ffi::c_int * 12 as ::core::ffi::c_int;
    if months < -(120000 as ::core::ffi::c_int) || months > 120000 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if days < -(3660000 as ::core::ffi::c_int) || days > 3660000 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    year += months as ::core::ffi::c_int / 12 as ::core::ffi::c_int;
    month += months as ::core::ffi::c_int % 12 as ::core::ffi::c_int;
    if month < 1 as ::core::ffi::c_int {
        month += 12 as ::core::ffi::c_int;
        year -= 1;
    } else if month > 12 as ::core::ffi::c_int {
        month -= 12 as ::core::ffi::c_int;
        year += 1;
    }
    day = (if day
        < safe_c2rust_days_in_months[(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as usize][month as usize] as ::core::ffi::c_int
    {
        day as ::core::ffi::c_int
    } else {
        safe_c2rust_days_in_months[(year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && !(year as ::core::ffi::c_int % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as usize][month as usize] as ::core::ffi::c_int
    }) as gint;
    full_time = (*datetime)
        .usec
        .wrapping_div(USEC_PER_SECOND as guint64)
        .wrapping_add(
            (SEC_PER_DAY
                * (safe_c2rust_ymd_to_days(year, month, day) as ::core::ffi::c_int
                    + days as ::core::ffi::c_int
                    - UNIX_EPOCH_START) as ::core::ffi::c_long) as guint64,
        ) as gint64;
    interval = g_time_zone_adjust_time(
        (*datetime).tz,
        g_time_zone_is_dst((*datetime).tz, (*datetime).interval) as GTimeType,
        &raw mut full_time,
    );
    full_time -= g_time_zone_get_offset((*datetime).tz, interval) as gint64;
    full_time += UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY;
    full_time = ((full_time as ::core::ffi::c_long * USEC_PER_SECOND) as guint64)
        .wrapping_add((*datetime).usec.wrapping_rem(USEC_PER_SECOND as guint64))
        as gint64;
    full_time += (hours as gint64 * USEC_PER_HOUR
        + minutes as gint64 * USEC_PER_MINUTE
        + (seconds * USEC_PER_SECOND as gdouble) as gint64) as ::core::ffi::c_long;
    interval = g_time_zone_find_interval(
        (*datetime).tz,
        G_TIME_TYPE_UNIVERSAL,
        full_time / USEC_PER_SECOND - UNIX_EPOCH_START as gint64 * SEC_PER_DAY,
    );
    full_time +=
        USEC_PER_SECOND * g_time_zone_get_offset((*datetime).tz, interval) as ::core::ffi::c_long;
    new = safe_c2rust_g_date_time_alloc((*datetime).tz);
    (*new).interval = interval;
    (*new).days = (full_time as ::core::ffi::c_long / USEC_PER_DAY) as gint32;
    (*new).usec = (full_time as ::core::ffi::c_long % USEC_PER_DAY) as guint64;
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_compare(
    mut dt1: gconstpointer,
    mut dt2: gconstpointer,
) -> gint {
    let mut difference: gint64 = 0;
    difference =
        safe_c2rust_g_date_time_difference(dt1 as *mut GDateTime, dt2 as *mut GDateTime) as gint64;
    if difference < 0 as gint64 {
        return -(1 as gint);
    } else if difference > 0 as gint64 {
        return 1 as gint;
    } else {
        return 0 as gint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_difference(
    mut end: *mut GDateTime,
    mut begin: *mut GDateTime,
) -> GTimeSpan {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !begin.is_null() {
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
            b"begin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GTimeSpan;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !end.is_null() {
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
            b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GTimeSpan;
    }
    return safe_c2rust_g_date_time_to_instant(end) as GTimeSpan
        - safe_c2rust_g_date_time_to_instant(begin) as GTimeSpan;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_hash(mut datetime: gconstpointer) -> guint {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return safe_c2rust_g_date_time_to_instant(datetime as *mut GDateTime) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_equal(
    mut dt1: gconstpointer,
    mut dt2: gconstpointer,
) -> gboolean {
    return (safe_c2rust_g_date_time_difference(dt1 as *mut GDateTime, dt2 as *mut GDateTime)
        == 0 as GTimeSpan) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_ymd(
    mut datetime: *mut GDateTime,
    mut year: *mut gint,
    mut month: *mut gint,
    mut day: *mut gint,
) {
    let mut the_year: gint = 0;
    let mut the_month: gint = 0;
    let mut the_day: gint = 0;
    let mut remaining_days: gint = 0;
    let mut y100_cycles: gint = 0;
    let mut y4_cycles: gint = 0;
    let mut y1_cycles: gint = 0;
    let mut preceding: gint = 0;
    let mut leap: gboolean = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    remaining_days = (*datetime).days as gint;
    remaining_days -= 1;
    the_year = (remaining_days as ::core::ffi::c_int / DAYS_IN_400YEARS * 400 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as gint;
    remaining_days = (remaining_days as ::core::ffi::c_int % DAYS_IN_400YEARS) as gint;
    y100_cycles = (remaining_days as ::core::ffi::c_int / DAYS_IN_100YEARS) as gint;
    remaining_days = (remaining_days as ::core::ffi::c_int % DAYS_IN_100YEARS) as gint;
    the_year += y100_cycles as ::core::ffi::c_int * 100 as ::core::ffi::c_int;
    y4_cycles = (remaining_days as ::core::ffi::c_int / DAYS_IN_4YEARS) as gint;
    remaining_days = (remaining_days as ::core::ffi::c_int % DAYS_IN_4YEARS) as gint;
    the_year += y4_cycles as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
    y1_cycles = (remaining_days as ::core::ffi::c_int / 365 as ::core::ffi::c_int) as gint;
    the_year += y1_cycles;
    remaining_days = (remaining_days as ::core::ffi::c_int % 365 as ::core::ffi::c_int) as gint;
    if y1_cycles == 4 as ::core::ffi::c_int || y100_cycles == 4 as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if remaining_days == 0 as ::core::ffi::c_int {
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
                b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
                2241 as ::core::ffi::c_int,
                G_STRFUNC,
                b"remaining_days == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        the_year -= 1;
        the_month = 12 as ::core::ffi::c_int as gint;
        the_day = 31 as ::core::ffi::c_int as gint;
    } else {
        leap = (y1_cycles == 3 as ::core::ffi::c_int
            && (y4_cycles != 24 as ::core::ffi::c_int || y100_cycles == 3 as ::core::ffi::c_int))
            as ::core::ffi::c_int as gboolean;
        if ({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if leap
                == (the_year as ::core::ffi::c_int % 4 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && !(the_year as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && the_year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int)) as ::core::ffi::c_int
            {
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
                b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
                2254 as ::core::ffi::c_int,
                G_STRFUNC,
                b"leap == GREGORIAN_LEAP(the_year)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        the_month = (remaining_days as ::core::ffi::c_int + 50 as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int) as gint;
        preceding = (safe_c2rust_days_in_year[0 as ::core::ffi::c_int as usize]
            [(the_month as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            + (the_month > 2 as ::core::ffi::c_int && leap != 0) as ::core::ffi::c_int)
            as gint;
        if preceding > remaining_days {
            the_month -= 1 as ::core::ffi::c_int;
            preceding -= if leap != 0 {
                safe_c2rust_days_in_months[1 as ::core::ffi::c_int as usize][the_month as usize]
                    as ::core::ffi::c_int
            } else {
                safe_c2rust_days_in_months[0 as ::core::ffi::c_int as usize][the_month as usize]
                    as ::core::ffi::c_int
            };
        }
        remaining_days -= preceding;
        if ({
            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
            if 0 as ::core::ffi::c_int <= remaining_days {
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
                b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
                2267 as ::core::ffi::c_int,
                G_STRFUNC,
                b"0 <= remaining_days\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        the_day = (remaining_days as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
    }
    if !year.is_null() {
        *year = the_year;
    }
    if !month.is_null() {
        *month = the_month;
    }
    if !day.is_null() {
        *day = the_day;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_year(mut datetime: *mut GDateTime) -> gint {
    let mut year: gint = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    safe_c2rust_g_date_time_get_ymd(
        datetime,
        &raw mut year,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
    );
    return year;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_month(mut datetime: *mut GDateTime) -> gint {
    let mut month: gint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    safe_c2rust_g_date_time_get_ymd(
        datetime,
        ::core::ptr::null_mut::<gint>(),
        &raw mut month,
        ::core::ptr::null_mut::<gint>(),
    );
    return month;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_day_of_month(
    mut datetime: *mut GDateTime,
) -> gint {
    let mut day_of_year: gint = 0;
    let mut i: gint = 0;
    let mut is_leap: guint = 0;
    let mut last: guint16 = 0 as guint16;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    is_leap = (if safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
        % 4 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && !(safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
            % 100 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
                % 400 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint;
    safe_c2rust_g_date_time_get_week_number(
        datetime,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        &raw mut day_of_year,
    );
    i = 1 as ::core::ffi::c_int as gint;
    while i <= 12 as ::core::ffi::c_int {
        if safe_c2rust_days_in_year[is_leap as usize][i as usize] as ::core::ffi::c_int
            >= day_of_year
        {
            return day_of_year - last as gint;
        }
        last = safe_c2rust_days_in_year[is_leap as usize][i as usize];
        i += 1;
    }
    g_warn_message(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
        2356 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_week_numbering_year(
    mut datetime: *mut GDateTime,
) -> gint {
    let mut year: gint = -(1 as gint);
    let mut month: gint = -(1 as gint);
    let mut day: gint = -(1 as gint);
    let mut weekday: gint = 0;
    safe_c2rust_g_date_time_get_ymd(datetime, &raw mut year, &raw mut month, &raw mut day);
    weekday = safe_c2rust_g_date_time_get_day_of_week(datetime);
    if month == 1 as ::core::ffi::c_int && day - weekday <= -(4 as ::core::ffi::c_int) {
        return year - 1 as gint;
    } else if month == 12 as ::core::ffi::c_int && day - weekday >= 28 as ::core::ffi::c_int {
        return year + 1 as gint;
    } else {
        return year;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_week_of_year(
    mut datetime: *mut GDateTime,
) -> gint {
    let mut weeknum: gint = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    safe_c2rust_g_date_time_get_week_number(
        datetime,
        &raw mut weeknum,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
    );
    return weeknum;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_day_of_week(
    mut datetime: *mut GDateTime,
) -> gint {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return ((*datetime).days as gint - 1 as gint) % 7 as gint + 1 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_day_of_year(
    mut datetime: *mut GDateTime,
) -> gint {
    let mut doy: gint = 0 as gint;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    safe_c2rust_g_date_time_get_week_number(
        datetime,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        &raw mut doy,
    );
    return doy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_hour(mut datetime: *mut GDateTime) -> gint {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*datetime).usec.wrapping_div(USEC_PER_HOUR as guint64) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_minute(mut datetime: *mut GDateTime) -> gint {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*datetime)
        .usec
        .wrapping_rem(USEC_PER_HOUR as guint64)
        .wrapping_div(USEC_PER_MINUTE as guint64) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_second(mut datetime: *mut GDateTime) -> gint {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*datetime)
        .usec
        .wrapping_rem(USEC_PER_MINUTE as guint64)
        .wrapping_div(USEC_PER_SECOND as guint64) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_microsecond(
    mut datetime: *mut GDateTime,
) -> gint {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*datetime).usec.wrapping_rem(USEC_PER_SECOND as guint64) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_seconds(
    mut datetime: *mut GDateTime,
) -> gdouble {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    return (*datetime).usec.wrapping_rem(USEC_PER_MINUTE as guint64) as gdouble / 1000000.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_unix(mut datetime: *mut GDateTime) -> gint64 {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    return safe_c2rust_g_date_time_to_instant(datetime) / USEC_PER_SECOND
        - UNIX_EPOCH_START as gint64 * SEC_PER_DAY;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_unix_usec(
    mut datetime: *mut GDateTime,
) -> gint64 {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    return safe_c2rust_g_date_time_to_instant(datetime)
        - UNIX_EPOCH_START as gint64 * SEC_PER_DAY * USEC_PER_SECOND;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_timeval(
    mut datetime: *mut GDateTime,
    mut tv: *mut GTimeVal,
) -> gboolean {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    (*tv).tv_sec = (safe_c2rust_g_date_time_to_instant(datetime) as ::core::ffi::c_long
        / USEC_PER_SECOND
        - UNIX_EPOCH_START as ::core::ffi::c_long * SEC_PER_DAY) as glong;
    (*tv).tv_usec = (*datetime).usec.wrapping_rem(USEC_PER_SECOND as guint64) as glong;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_utc_offset(
    mut datetime: *mut GDateTime,
) -> GTimeSpan {
    let mut offset: gint = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GTimeSpan;
    }
    offset = g_time_zone_get_offset((*datetime).tz, (*datetime).interval) as gint;
    return offset as GTimeSpan * USEC_PER_SECOND;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_timezone(
    mut datetime: *mut GDateTime,
) -> *mut GTimeZone {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTimeZone>();
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !(*datetime).tz.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
            2736 as ::core::ffi::c_int,
            G_STRFUNC,
            b"datetime->tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*datetime).tz;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_get_timezone_abbreviation(
    mut datetime: *mut GDateTime,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return g_time_zone_get_abbreviation((*datetime).tz, (*datetime).interval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_is_daylight_savings(
    mut datetime: *mut GDateTime,
) -> gboolean {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_time_zone_is_dst((*datetime).tz, (*datetime).interval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_timezone(
    mut datetime: *mut GDateTime,
    mut tz: *mut GTimeZone,
) -> *mut GDateTime {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !tz.is_null() {
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
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return safe_c2rust_g_date_time_from_instant(tz, safe_c2rust_g_date_time_to_instant(datetime));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_local(
    mut datetime: *mut GDateTime,
) -> *mut GDateTime {
    let mut new: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut local: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    local = g_time_zone_new_local();
    new = safe_c2rust_g_date_time_to_timezone(datetime, local);
    g_time_zone_unref(local);
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_to_utc(
    mut datetime: *mut GDateTime,
) -> *mut GDateTime {
    let mut new: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut utc: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    utc = g_time_zone_new_utc();
    new = safe_c2rust_g_date_time_to_timezone(datetime, utc);
    g_time_zone_unref(utc);
    return new;
}
unsafe extern "C" fn safe_c2rust_format_z(
    mut outstr: *mut GString,
    mut offset: gint,
    mut colons: guint,
) -> gboolean {
    let mut hours: gint = 0;
    let mut minutes: gint = 0;
    let mut seconds: gint = 0;
    let mut sign: gchar = (if offset >= 0 as ::core::ffi::c_int {
        '+' as i32
    } else {
        '-' as i32
    }) as gchar;
    offset = if offset < 0 as ::core::ffi::c_int {
        -offset
    } else {
        offset
    };
    hours = (offset as ::core::ffi::c_int / 3600 as ::core::ffi::c_int) as gint;
    minutes = (offset as ::core::ffi::c_int / 60 as ::core::ffi::c_int % 60 as ::core::ffi::c_int)
        as gint;
    seconds = (offset as ::core::ffi::c_int % 60 as ::core::ffi::c_int) as gint;
    match colons {
        0 => {
            g_string_append_printf(
                outstr,
                b"%c%02d%02d\0" as *const u8 as *const gchar,
                sign as ::core::ffi::c_int,
                hours,
                minutes,
            );
        }
        1 => {
            g_string_append_printf(
                outstr,
                b"%c%02d:%02d\0" as *const u8 as *const gchar,
                sign as ::core::ffi::c_int,
                hours,
                minutes,
            );
        }
        2 => {
            g_string_append_printf(
                outstr,
                b"%c%02d:%02d:%02d\0" as *const u8 as *const gchar,
                sign as ::core::ffi::c_int,
                hours,
                minutes,
                seconds,
            );
        }
        3 => {
            g_string_append_printf(
                outstr,
                b"%c%02d\0" as *const u8 as *const gchar,
                sign as ::core::ffi::c_int,
                hours,
            );
            if minutes != 0 as ::core::ffi::c_int || seconds != 0 as ::core::ffi::c_int {
                g_string_append_printf(outstr, b":%02d\0" as *const u8 as *const gchar, minutes);
                if seconds != 0 as ::core::ffi::c_int {
                    g_string_append_printf(
                        outstr,
                        b":%02d\0" as *const u8 as *const gchar,
                        seconds,
                    );
                }
            }
        }
        _ => return FALSE,
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_initialize_alt_digits() -> *const *const gchar {
    let mut i: guint = 0;
    let mut digit_len: gsize = 0;
    let mut digit: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut locale_digit: *const gchar = ::core::ptr::null::<gchar>();
    static mut safe_c2rust_buffer: [gchar; 50] = [0; 50];
    let mut buffer_end: *mut gchar = &raw mut safe_c2rust_buffer as *mut gchar;
    static mut safe_c2rust_alt_digits: [*const gchar; 10] = [::core::ptr::null::<gchar>(); 10];
    i = 0 as guint;
    while i != 10 as guint {
        locale_digit = nl_langinfo(
            (_NL_CTYPE_OUTDIGIT0_MB as ::core::ffi::c_int as guint).wrapping_add(i) as nl_item,
        );
        if g_strcmp0(
            locale_digit as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<*const gchar>();
        }
        digit = _g_ctype_locale_to_utf8(
            locale_digit,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            &raw mut digit_len,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if digit.is_null() {
            return ::core::ptr::null::<*const gchar>();
        }
        if ({
            let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
            if digit_len
                < (&raw mut safe_c2rust_buffer as *mut gchar)
                    .offset(::core::mem::size_of::<[gchar; 50]>() as usize as isize)
                    .offset_from(buffer_end) as ::core::ffi::c_long as gsize
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
                2962 as ::core::ffi::c_int,
                G_STRFUNC,
                b"digit_len < (gsize) (buffer + sizeof (buffer) - buffer_end)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_alt_digits[i as usize] = buffer_end;
        buffer_end = g_stpcpy(buffer_end, digit);
        buffer_end = buffer_end.offset(1 as ::core::ffi::c_int as isize);
        g_free(digit as gpointer);
        i = i.wrapping_add(1);
    }
    return &raw mut safe_c2rust_alt_digits as *mut *const gchar;
}
unsafe extern "C" fn safe_c2rust_date_time_lookup_era(
    mut datetime: *mut GDateTime,
    mut locale_is_utf8: gboolean,
) -> *mut GEraDescriptionSegment {
    static mut safe_c2rust_era_mutex: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    static mut safe_c2rust_static_era_description: *mut GPtrArray =
        ::core::ptr::null::<GPtrArray>() as *mut GPtrArray;
    static mut safe_c2rust_static_era_description_locale: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut current_lc_time: *const ::core::ffi::c_char =
        setlocale(LC_TIME, ::core::ptr::null::<::core::ffi::c_char>());
    let mut local_era_description: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut datetime_date: GEraDate = GEraDate {
        type_0: G_ERA_DATE_SET,
        year: 0,
        month: 0,
        day: 0,
    };
    g_mutex_lock(&raw mut safe_c2rust_era_mutex);
    if safe_c2rust_static_era_description_locale != current_lc_time {
        let mut era_description_str: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut era_description_str_len: size_t = 0;
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        era_description_str = nl_langinfo(ERA as ::core::ffi::c_int as nl_item);
        if !era_description_str.is_null() {
            let mut n_entries: ::core::ffi::c_int =
                nl_langinfo(_NL_TIME_ERA_NUM_ENTRIES as ::core::ffi::c_int as nl_item) as gintptr
                    as ::core::ffi::c_int;
            let mut s: *const ::core::ffi::c_char = era_description_str;
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < n_entries {
                let mut next_semicolon: *const ::core::ffi::c_char = strchr(s, ';' as i32);
                let mut next_nul: *const ::core::ffi::c_char = strchr(s, '\0' as i32);
                if !next_semicolon.is_null() && next_semicolon < next_nul {
                    s = next_semicolon.offset(1 as ::core::ffi::c_int as isize);
                } else {
                    s = next_nul.offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
            }
            era_description_str_len = strlen(s)
                .wrapping_add(s.offset_from(era_description_str) as ::core::ffi::c_long as size_t);
            tmp = g_memdup2(
                era_description_str as gconstpointer,
                (era_description_str_len as gsize).wrapping_add(1 as gsize),
            ) as *mut ::core::ffi::c_char;
            era_description_str = tmp;
            s = era_description_str;
            let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i_0 < n_entries {
                let mut next_nul_0: *mut ::core::ffi::c_char = strchr(s, '\0' as i32);
                if next_nul_0.offset_from(era_description_str) as ::core::ffi::c_long as size_t
                    >= era_description_str_len
                {
                    break;
                }
                *next_nul_0 = ';' as i32 as ::core::ffi::c_char;
                s = next_nul_0.offset(1 as ::core::ffi::c_int as isize);
                i_0 += 1;
            }
            if locale_is_utf8 == 0 && FALSE == 0 {
                let mut tmp2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp2 = g_locale_to_utf8(
                    era_description_str as *const gchar,
                    -(1 as ::core::ffi::c_int) as gssize,
                    ::core::ptr::null_mut::<gsize>(),
                    ::core::ptr::null_mut::<gsize>(),
                    ::core::ptr::null_mut::<*mut GError>(),
                ) as *mut ::core::ffi::c_char;
                era_description_str = tmp2;
                g_free(tmp as gpointer);
                tmp = safe_c2rust_g_steal_pointer(&raw mut tmp2 as gpointer)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            let mut _pp: *mut *mut GPtrArray = &raw mut safe_c2rust_static_era_description;
            let mut _ptr: *mut GPtrArray = *_pp;
            *_pp = ::core::ptr::null_mut::<GPtrArray>();
            if !_ptr.is_null() {
                g_ptr_array_unref(_ptr as *mut GPtrArray);
            }
            if !era_description_str.is_null() {
                safe_c2rust_static_era_description = _g_era_description_parse(era_description_str);
            }
            if safe_c2rust_static_era_description.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Could not parse ERA description: %s\0" as *const u8 as *const gchar,
                    era_description_str,
                );
            }
        } else {
            let mut _pp_0: *mut *mut GPtrArray = &raw mut safe_c2rust_static_era_description;
            let mut _ptr_0: *mut GPtrArray = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<GPtrArray>();
            if !_ptr_0.is_null() {
                g_ptr_array_unref(_ptr_0 as *mut GPtrArray);
            }
        }
        g_free(tmp as gpointer);
        safe_c2rust_static_era_description_locale = current_lc_time;
    }
    if safe_c2rust_static_era_description.is_null() {
        g_mutex_unlock(&raw mut safe_c2rust_era_mutex);
        return ::core::ptr::null_mut::<GEraDescriptionSegment>();
    }
    local_era_description = g_ptr_array_ref(safe_c2rust_static_era_description);
    g_mutex_unlock(&raw mut safe_c2rust_era_mutex);
    datetime_date.type_0 = G_ERA_DATE_SET;
    datetime_date.year = safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int;
    datetime_date.month = safe_c2rust_g_date_time_get_month(datetime) as ::core::ffi::c_int;
    datetime_date.day = safe_c2rust_g_date_time_get_day_of_month(datetime) as ::core::ffi::c_int;
    let mut i_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i_1 < (*local_era_description).len {
        let mut segment: *mut GEraDescriptionSegment =
            *(*local_era_description).pdata.offset(i_1 as isize) as *mut GEraDescriptionSegment;
        if _g_era_date_compare(&raw mut (*segment).start_date, &raw mut datetime_date)
            <= 0 as ::core::ffi::c_int
            && _g_era_date_compare(&raw mut datetime_date, &raw mut (*segment).end_date)
                <= 0 as ::core::ffi::c_int
            || _g_era_date_compare(&raw mut (*segment).end_date, &raw mut datetime_date)
                <= 0 as ::core::ffi::c_int
                && _g_era_date_compare(&raw mut datetime_date, &raw mut (*segment).start_date)
                    <= 0 as ::core::ffi::c_int
        {
            g_ptr_array_unref(local_era_description);
            return _g_era_description_segment_ref(segment);
        }
        i_1 = i_1.wrapping_add(1);
    }
    g_ptr_array_unref(local_era_description);
    return ::core::ptr::null_mut::<GEraDescriptionSegment>();
}
unsafe extern "C" fn safe_c2rust_format_number(
    mut str: *mut GString,
    mut use_alt_digits: gboolean,
    mut pad: *const gchar,
    mut width: gint,
    mut number: guint32,
) {
    let mut ascii_digits: [*const gchar; 10] = [
        b"0\0" as *const u8 as *const ::core::ffi::c_char,
        b"1\0" as *const u8 as *const ::core::ffi::c_char,
        b"2\0" as *const u8 as *const ::core::ffi::c_char,
        b"3\0" as *const u8 as *const ::core::ffi::c_char,
        b"4\0" as *const u8 as *const ::core::ffi::c_char,
        b"5\0" as *const u8 as *const ::core::ffi::c_char,
        b"6\0" as *const u8 as *const ::core::ffi::c_char,
        b"7\0" as *const u8 as *const ::core::ffi::c_char,
        b"8\0" as *const u8 as *const ::core::ffi::c_char,
        b"9\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut digits: *const *const gchar = &raw mut ascii_digits as *mut *const gchar;
    let mut tmp: [*const gchar; 10] = [::core::ptr::null::<gchar>(); 10];
    let mut i: gint = 0 as gint;
    static mut safe_c2rust_alt_digits_mutex: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if width <= 10 as ::core::ffi::c_int {
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
            b"width <= 10\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if use_alt_digits != 0 {
        static mut safe_c2rust_alt_digits: *const *const gchar =
            ::core::ptr::null::<*const gchar>();
        static mut safe_c2rust_alt_digits_locale: *mut ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
        let mut current_ctype_locale: *const ::core::ffi::c_char =
            setlocale(LC_CTYPE, ::core::ptr::null::<::core::ffi::c_char>());
        g_mutex_lock(&raw mut safe_c2rust_alt_digits_mutex);
        if g_strcmp0(safe_c2rust_alt_digits_locale, current_ctype_locale) != 0 as ::core::ffi::c_int
        {
            safe_c2rust_alt_digits = safe_c2rust_initialize_alt_digits();
            if safe_c2rust_alt_digits.is_null() {
                safe_c2rust_alt_digits = &raw mut ascii_digits as *mut *const gchar;
            }
            g_free(safe_c2rust_alt_digits_locale as gpointer);
            safe_c2rust_alt_digits_locale = safe_c2rust_g_strdup_inline(current_ctype_locale);
        }
        digits = safe_c2rust_alt_digits;
    }
    loop {
        let fresh1 = i;
        i = i + 1;
        tmp[fresh1 as usize] = *digits.offset(number.wrapping_rem(10 as guint32) as isize);
        number = number.wrapping_div(10 as guint32);
        if !(number != 0) {
            break;
        }
    }
    while !pad.is_null() && i < width {
        let fresh2 = i;
        i = i + 1;
        tmp[fresh2 as usize] = if *pad as ::core::ffi::c_int == '0' as i32 {
            *digits.offset(0 as ::core::ffi::c_int as isize)
        } else {
            pad
        };
    }
    if use_alt_digits != 0 {
        g_mutex_unlock(&raw mut safe_c2rust_alt_digits_mutex);
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if i <= 10 as ::core::ffi::c_int {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
            3165 as ::core::ffi::c_int,
            G_STRFUNC,
            b"i <= 10\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    while i != 0 {
        if 0 != 0 {
            ({
                i -= 1;
                let __val: *const ::core::ffi::c_char = tmp[i as usize];
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_52
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
            i -= 1;
            safe_c2rust_g_string_append_len_inline(
                str,
                tmp[i as usize],
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
}
unsafe extern "C" fn safe_c2rust_format_ampm(
    mut datetime: *mut GDateTime,
    mut outstr: *mut GString,
    mut locale_is_utf8: gboolean,
    mut uppercase: gboolean,
) -> gboolean {
    let mut ampm: *const gchar = ::core::ptr::null::<gchar>();
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ampm_dup: *mut gchar = ::core::ptr::null_mut::<gchar>();
    ampm = if safe_c2rust_g_date_time_get_hour(datetime) < 12 as ::core::ffi::c_int {
        nl_langinfo(AM_STR as ::core::ffi::c_int as nl_item)
    } else {
        nl_langinfo(PM_STR as ::core::ffi::c_int as nl_item)
    };
    if ampm.is_null()
        || *ampm.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
    {
        ampm = safe_c2rust_get_fallback_ampm(safe_c2rust_g_date_time_get_hour(datetime));
    }
    if locale_is_utf8 == 0 && FALSE == 0 {
        tmp = g_locale_to_utf8(
            ampm,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        ampm = tmp;
        if tmp.is_null() {
            return FALSE;
        }
    }
    if uppercase != 0 {
        ampm_dup = g_utf8_strup(ampm, -(1 as ::core::ffi::c_int) as gssize);
    } else {
        ampm_dup = g_utf8_strdown(ampm, -(1 as ::core::ffi::c_int) as gssize);
    }
    g_free(tmp as gpointer);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = ampm_dup;
            safe_c2rust_g_string_append_len_inline(
                outstr,
                __val,
                if ({
                    let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_53 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_53 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_53
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
            outstr,
            ampm_dup,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_free(ampm_dup as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_date_time_format_locale(
    mut datetime: *mut GDateTime,
    mut locale_format: *const gchar,
    mut outstr: *mut GString,
    mut locale_is_utf8: gboolean,
) -> gboolean {
    let mut utf8_format: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut success: gboolean = 0;
    if locale_is_utf8 != 0 {
        return safe_c2rust_g_date_time_format_utf8(
            datetime,
            locale_format,
            outstr,
            locale_is_utf8,
        );
    }
    utf8_format = _g_time_locale_to_utf8(
        locale_format,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if utf8_format.is_null() {
        return FALSE;
    }
    success = safe_c2rust_g_date_time_format_utf8(datetime, utf8_format, outstr, locale_is_utf8);
    g_free(utf8_format as gpointer);
    return success;
}
#[inline]
unsafe extern "C" fn safe_c2rust_string_append(
    mut string: *mut GString,
    mut s: *const gchar,
    mut do_strup: gboolean,
    mut s_is_utf8: gboolean,
) -> gboolean {
    let mut utf8: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut utf8_len: gsize = 0;
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if s_is_utf8 != 0 {
        if do_strup != 0 {
            tmp = g_utf8_strup(s, -(1 as ::core::ffi::c_int) as gssize) as *mut ::core::ffi::c_char;
            s = tmp;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = s as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_54
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
                s as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        utf8 = _g_time_locale_to_utf8(
            s,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            &raw mut utf8_len,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if utf8.is_null() {
            return FALSE;
        }
        if do_strup != 0 {
            tmp = g_utf8_strup(utf8, utf8_len as gssize) as *mut ::core::ffi::c_char;
            g_free(utf8 as gpointer);
            utf8 = safe_c2rust_g_steal_pointer(&raw mut tmp as gpointer) as *mut ::core::ffi::c_char
                as *mut gchar;
        }
        safe_c2rust_g_string_append_len_inline(string, utf8, utf8_len as gssize);
        g_free(utf8 as gpointer);
    }
    g_free(tmp as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_date_time_format_utf8(
    mut datetime: *mut GDateTime,
    mut utf8_format: *const gchar,
    mut outstr: *mut GString,
    mut locale_is_utf8: gboolean,
) -> gboolean {
    let mut current_block: u64;
    let mut len: guint = 0;
    let mut colons: guint = 0;
    let mut c: gunichar = 0;
    let mut alt_digits: gboolean = FALSE;
    let mut alt_era: gboolean = FALSE;
    let mut pad_set: gboolean = FALSE;
    let mut mod_case: gboolean = FALSE;
    let mut name_is_utf8: gboolean = 0;
    let mut pad: *const gchar = b"\0" as *const u8 as *const gchar;
    let mut mod_0: *const gchar = b"\0" as *const u8 as *const gchar;
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut tz: *const gchar = ::core::ptr::null::<gchar>();
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    's_27: while *utf8_format != 0 {
        len = strcspn(
            utf8_format as *const ::core::ffi::c_char,
            b"%\0" as *const u8 as *const ::core::ffi::c_char,
        ) as guint;
        if len != 0 {
            safe_c2rust_g_string_append_len_inline(
                outstr,
                utf8_format as *const ::core::ffi::c_char,
                len as gssize,
            );
        }
        utf8_format = utf8_format.offset(len as isize);
        if *utf8_format == 0 {
            break;
        }
        if ({
            let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
            if *utf8_format as ::core::ffi::c_int == '%' as i32 {
                _g_boolean_var_55 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_55 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_55
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gdatetime.c\0" as *const u8 as *const ::core::ffi::c_char,
                3303 as ::core::ffi::c_int,
                G_STRFUNC,
                b"*utf8_format == '%'\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        utf8_format = utf8_format.offset(1);
        if *utf8_format == 0 {
            break;
        }
        colons = 0 as guint;
        alt_digits = FALSE as gboolean;
        alt_era = FALSE as gboolean;
        pad_set = FALSE as gboolean;
        mod_case = FALSE as gboolean;
        loop {
            c = g_utf8_get_char(utf8_format);
            utf8_format = utf8_format.offset(
                *safe_c2rust_g_utf8_skip.offset(*(utf8_format as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            match c {
                97 => {
                    name = nl_langinfo(
                        safe_c2rust_weekday_item[0 as ::core::ffi::c_int as usize]
                            [(safe_c2rust_g_date_time_get_day_of_week(datetime)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize],
                    );
                    if g_strcmp0(
                        name as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    name_is_utf8 =
                        (locale_is_utf8 != 0 || FALSE != 0) as ::core::ffi::c_int as gboolean;
                    if safe_c2rust_string_append(outstr, name, mod_case, name_is_utf8) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                65 => {
                    name = nl_langinfo(
                        safe_c2rust_weekday_item[1 as ::core::ffi::c_int as usize]
                            [(safe_c2rust_g_date_time_get_day_of_week(datetime)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize],
                    );
                    if g_strcmp0(
                        name as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    name_is_utf8 =
                        (locale_is_utf8 != 0 || FALSE != 0) as ::core::ffi::c_int as gboolean;
                    if safe_c2rust_string_append(outstr, name, mod_case, name_is_utf8) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                98 => {
                    name = if alt_digits != 0 {
                        nl_langinfo(
                            safe_c2rust_ab_alt_month_item[(safe_c2rust_g_date_time_get_month(
                                datetime,
                            )
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                                as usize],
                        )
                    } else {
                        nl_langinfo(
                            safe_c2rust_month_item[0 as ::core::ffi::c_int as usize]
                                [(safe_c2rust_g_date_time_get_month(datetime) as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as usize],
                        )
                    };
                    if g_strcmp0(
                        name as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    name_is_utf8 = (locale_is_utf8 != 0
                        || (alt_digits != 0 && FALSE != 0 || alt_digits == 0 && FALSE != 0))
                        as ::core::ffi::c_int as gboolean;
                    if safe_c2rust_string_append(outstr, name, mod_case, name_is_utf8) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                66 => {
                    name = if alt_digits != 0 {
                        nl_langinfo(
                            safe_c2rust_alt_month_item[(safe_c2rust_g_date_time_get_month(datetime)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                                as usize],
                        )
                    } else {
                        nl_langinfo(
                            safe_c2rust_month_item[1 as ::core::ffi::c_int as usize]
                                [(safe_c2rust_g_date_time_get_month(datetime) as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as usize],
                        )
                    };
                    if g_strcmp0(
                        name as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    name_is_utf8 = (locale_is_utf8 != 0
                        || (alt_digits != 0 && FALSE != 0 || alt_digits == 0 && FALSE != 0))
                        as ::core::ffi::c_int as gboolean;
                    if safe_c2rust_string_append(outstr, name, mod_case, name_is_utf8) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                99 => {
                    let mut subformat: *const ::core::ffi::c_char = if alt_era != 0 {
                        nl_langinfo(ERA_D_T_FMT as ::core::ffi::c_int as nl_item)
                    } else {
                        nl_langinfo(D_T_FMT as ::core::ffi::c_int as nl_item)
                    };
                    if alt_era != 0
                        && g_strcmp0(subformat, b"\0" as *const u8 as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                    {
                        subformat = nl_langinfo(D_T_FMT as ::core::ffi::c_int as nl_item);
                    }
                    if g_strcmp0(subformat, b"\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    if safe_c2rust_g_date_time_format_locale(
                        datetime,
                        subformat as *const gchar,
                        outstr,
                        locale_is_utf8,
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                67 => {
                    if alt_era != 0 {
                        current_block = 3160140712158701372;
                        break;
                    } else {
                        current_block = 5141539773904409130;
                        break;
                    }
                }
                100 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_day_of_month(datetime) as guint32,
                    );
                    continue 's_27;
                }
                101 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"\xE2\x80\x87\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_day_of_month(datetime) as guint32,
                    );
                    continue 's_27;
                }
                102 => {
                    g_string_append_printf(
                        outstr,
                        b"%06lu\0" as *const u8 as *const gchar,
                        (*datetime).usec.wrapping_rem(G_TIME_SPAN_SECOND as guint64),
                    );
                    continue 's_27;
                }
                70 => {
                    g_string_append_printf(
                        outstr,
                        b"%d-%02d-%02d\0" as *const u8 as *const gchar,
                        safe_c2rust_g_date_time_get_year(datetime),
                        safe_c2rust_g_date_time_get_month(datetime),
                        safe_c2rust_g_date_time_get_day_of_month(datetime),
                    );
                    continue 's_27;
                }
                103 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        (safe_c2rust_g_date_time_get_week_numbering_year(datetime)
                            as ::core::ffi::c_int
                            % 100 as ::core::ffi::c_int) as guint32,
                    );
                    continue 's_27;
                }
                71 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            ::core::ptr::null::<gchar>()
                        },
                        0 as gint,
                        safe_c2rust_g_date_time_get_week_numbering_year(datetime) as guint32,
                    );
                    continue 's_27;
                }
                104 => {
                    name = if alt_digits != 0 {
                        nl_langinfo(
                            safe_c2rust_ab_alt_month_item[(safe_c2rust_g_date_time_get_month(
                                datetime,
                            )
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                                as usize],
                        )
                    } else {
                        nl_langinfo(
                            safe_c2rust_month_item[0 as ::core::ffi::c_int as usize]
                                [(safe_c2rust_g_date_time_get_month(datetime) as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as usize],
                        )
                    };
                    if g_strcmp0(
                        name as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    name_is_utf8 = (locale_is_utf8 != 0
                        || (alt_digits != 0 && FALSE != 0 || alt_digits == 0 && FALSE != 0))
                        as ::core::ffi::c_int as gboolean;
                    if safe_c2rust_string_append(outstr, name, mod_case, name_is_utf8) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                72 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_hour(datetime) as guint32,
                    );
                    continue 's_27;
                }
                73 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        ((safe_c2rust_g_date_time_get_hour(datetime) as ::core::ffi::c_int
                            + 11 as ::core::ffi::c_int)
                            % 12 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as guint32,
                    );
                    continue 's_27;
                }
                106 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        3 as gint,
                        safe_c2rust_g_date_time_get_day_of_year(datetime) as guint32,
                    );
                    continue 's_27;
                }
                107 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"\xE2\x80\x87\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_hour(datetime) as guint32,
                    );
                    continue 's_27;
                }
                108 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"\xE2\x80\x87\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        ((safe_c2rust_g_date_time_get_hour(datetime) as ::core::ffi::c_int
                            + 11 as ::core::ffi::c_int)
                            % 12 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as guint32,
                    );
                    continue 's_27;
                }
                109 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_month(datetime) as guint32,
                    );
                    continue 's_27;
                }
                77 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_minute(datetime) as guint32,
                    );
                    continue 's_27;
                }
                110 => {
                    safe_c2rust_g_string_append_c_inline(outstr, '\n' as i32 as gchar);
                    continue 's_27;
                }
                79 => {
                    alt_digits = TRUE as gboolean;
                }
                69 => {
                    alt_era = TRUE as gboolean;
                }
                112 => {
                    if safe_c2rust_format_ampm(
                        datetime,
                        outstr,
                        locale_is_utf8,
                        if mod_case != 0
                            && g_strcmp0(
                                mod_0 as *const ::core::ffi::c_char,
                                b"#\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            FALSE
                        } else {
                            TRUE
                        },
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                80 => {
                    if safe_c2rust_format_ampm(
                        datetime,
                        outstr,
                        locale_is_utf8,
                        if mod_case != 0
                            && g_strcmp0(
                                mod_0 as *const ::core::ffi::c_char,
                                b"^\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            TRUE
                        } else {
                            FALSE
                        },
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                114 => {
                    if g_strcmp0(
                        nl_langinfo(T_FMT_AMPM as ::core::ffi::c_int as nl_item),
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    if safe_c2rust_g_date_time_format_locale(
                        datetime,
                        nl_langinfo(T_FMT_AMPM as ::core::ffi::c_int as nl_item),
                        outstr,
                        locale_is_utf8,
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                82 => {
                    g_string_append_printf(
                        outstr,
                        b"%02d:%02d\0" as *const u8 as *const gchar,
                        safe_c2rust_g_date_time_get_hour(datetime),
                        safe_c2rust_g_date_time_get_minute(datetime),
                    );
                    continue 's_27;
                }
                115 => {
                    g_string_append_printf(
                        outstr,
                        b"%li\0" as *const u8 as *const gchar,
                        safe_c2rust_g_date_time_to_unix(datetime),
                    );
                    continue 's_27;
                }
                83 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_second(datetime) as guint32,
                    );
                    continue 's_27;
                }
                116 => {
                    safe_c2rust_g_string_append_c_inline(outstr, '\t' as i32 as gchar);
                    continue 's_27;
                }
                84 => {
                    g_string_append_printf(
                        outstr,
                        b"%02d:%02d:%02d\0" as *const u8 as *const gchar,
                        safe_c2rust_g_date_time_get_hour(datetime),
                        safe_c2rust_g_date_time_get_minute(datetime),
                        safe_c2rust_g_date_time_get_second(datetime),
                    );
                    continue 's_27;
                }
                117 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        ::core::ptr::null::<gchar>(),
                        0 as gint,
                        safe_c2rust_g_date_time_get_day_of_week(datetime) as guint32,
                    );
                    continue 's_27;
                }
                86 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        safe_c2rust_g_date_time_get_week_of_year(datetime) as guint32,
                    );
                    continue 's_27;
                }
                119 => {
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        ::core::ptr::null::<gchar>(),
                        0 as gint,
                        (safe_c2rust_g_date_time_get_day_of_week(datetime) as ::core::ffi::c_int
                            % 7 as ::core::ffi::c_int) as guint32,
                    );
                    continue 's_27;
                }
                120 => {
                    let mut subformat_0: *const ::core::ffi::c_char = if alt_era != 0 {
                        nl_langinfo(ERA_D_FMT as ::core::ffi::c_int as nl_item)
                    } else {
                        nl_langinfo(D_FMT as ::core::ffi::c_int as nl_item)
                    };
                    if alt_era != 0
                        && g_strcmp0(
                            subformat_0,
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        subformat_0 = nl_langinfo(D_FMT as ::core::ffi::c_int as nl_item);
                    }
                    if g_strcmp0(
                        subformat_0,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    if safe_c2rust_g_date_time_format_locale(
                        datetime,
                        subformat_0 as *const gchar,
                        outstr,
                        locale_is_utf8,
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                88 => {
                    let mut subformat_1: *const ::core::ffi::c_char = if alt_era != 0 {
                        nl_langinfo(ERA_T_FMT as ::core::ffi::c_int as nl_item)
                    } else {
                        nl_langinfo(T_FMT as ::core::ffi::c_int as nl_item)
                    };
                    if alt_era != 0
                        && g_strcmp0(
                            subformat_1,
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        subformat_1 = nl_langinfo(T_FMT as ::core::ffi::c_int as nl_item);
                    }
                    if g_strcmp0(
                        subformat_1,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return FALSE;
                    }
                    if safe_c2rust_g_date_time_format_locale(
                        datetime,
                        subformat_1 as *const gchar,
                        outstr,
                        locale_is_utf8,
                    ) == 0
                    {
                        return FALSE;
                    }
                    continue 's_27;
                }
                121 => {
                    if alt_era != 0 {
                        current_block = 900943123863005455;
                        break;
                    } else {
                        current_block = 8533724845731836612;
                        break;
                    }
                }
                89 => {
                    if alt_era != 0 {
                        current_block = 16667286137552459707;
                        break;
                    } else {
                        current_block = 5916212523694105379;
                        break;
                    }
                }
                122 => {
                    let mut offset: gint64 = 0;
                    offset = (safe_c2rust_g_date_time_get_utc_offset(datetime)
                        as ::core::ffi::c_long
                        / USEC_PER_SECOND) as gint64;
                    if safe_c2rust_format_z(outstr, offset as gint, colons) == 0 {
                        return FALSE;
                    }
                    continue 's_27;
                }
                90 => {
                    tz = safe_c2rust_g_date_time_get_timezone_abbreviation(datetime);
                    if mod_case != 0
                        && g_strcmp0(
                            mod_0 as *const ::core::ffi::c_char,
                            b"#\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        tmp = g_utf8_strdown(tz, -(1 as ::core::ffi::c_int) as gssize)
                            as *mut ::core::ffi::c_char;
                        tz = tmp;
                    }
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                tz as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                outstr,
                                __val,
                                if ({
                                    let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_57 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_57 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_57
                                }) as ::core::ffi::c_long
                                    != 0
                                {
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
                        safe_c2rust_g_string_append_len_inline(
                            outstr,
                            tz as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(tmp as gpointer);
                    continue 's_27;
                }
                37 => {
                    safe_c2rust_g_string_append_c_inline(outstr, '%' as i32 as gchar);
                    continue 's_27;
                }
                45 => {
                    pad_set = TRUE as gboolean;
                    pad = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                95 => {
                    pad_set = TRUE as gboolean;
                    pad = b" \0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                48 => {
                    pad_set = TRUE as gboolean;
                    pad = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                58 => {
                    if *utf8_format as ::core::ffi::c_int != 0
                        && *utf8_format as ::core::ffi::c_int != 'z' as i32
                        && *utf8_format as ::core::ffi::c_int != ':' as i32
                    {
                        return FALSE;
                    }
                    colons = colons.wrapping_add(1);
                }
                94 => {
                    mod_case = TRUE as gboolean;
                    mod_0 = b"^\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                35 => {
                    mod_case = TRUE as gboolean;
                    mod_0 = b"#\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                }
                _ => return FALSE,
            }
        }
        match current_block {
            900943123863005455 => {
                let mut era_0: *mut GEraDescriptionSegment =
                    safe_c2rust_date_time_lookup_era(datetime, locale_is_utf8);
                if !era_0.is_null() {
                    let mut delta: ::core::ffi::c_int = safe_c2rust_g_date_time_get_year(datetime)
                        as ::core::ffi::c_int
                        - (*era_0).start_date.year;
                    if (safe_c2rust_g_date_time_get_year(datetime) < 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        != ((*era_0).start_date.year < 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    {
                        delta -= 1 as ::core::ffi::c_int;
                    }
                    safe_c2rust_format_number(
                        outstr,
                        alt_digits,
                        if pad_set != 0 {
                            pad
                        } else {
                            b"0\0" as *const u8 as *const gchar
                        },
                        2 as gint,
                        (*era_0)
                            .offset
                            .wrapping_add((delta * (*era_0).direction_multiplier) as guint64)
                            as guint32,
                    );
                    _g_era_description_segment_unref(era_0);
                    continue;
                } else {
                    current_block = 8533724845731836612;
                }
            }
            16667286137552459707 => {
                let mut era_1: *mut GEraDescriptionSegment =
                    safe_c2rust_date_time_lookup_era(datetime, locale_is_utf8);
                if !era_1.is_null() {
                    if safe_c2rust_g_date_time_format_utf8(
                        datetime,
                        (*era_1).era_format,
                        outstr,
                        locale_is_utf8,
                    ) == 0
                    {
                        _g_era_description_segment_unref(era_1);
                        return FALSE;
                    }
                    _g_era_description_segment_unref(era_1);
                    continue;
                } else {
                    current_block = 5916212523694105379;
                }
            }
            3160140712158701372 => {
                let mut era: *mut GEraDescriptionSegment =
                    safe_c2rust_date_time_lookup_era(datetime, locale_is_utf8);
                if !era.is_null() {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = (*era).era_name;
                            safe_c2rust_g_string_append_len_inline(
                                outstr,
                                __val,
                                if ({
                                    let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_56 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_56 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_56
                                }) as ::core::ffi::c_long
                                    != 0
                                {
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
                        safe_c2rust_g_string_append_len_inline(
                            outstr,
                            (*era).era_name,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    _g_era_description_segment_unref(era);
                    continue;
                } else {
                    current_block = 5141539773904409130;
                }
            }
            _ => {}
        }
        match current_block {
            8533724845731836612 => {
                safe_c2rust_format_number(
                    outstr,
                    alt_digits,
                    if pad_set != 0 {
                        pad
                    } else {
                        b"0\0" as *const u8 as *const gchar
                    },
                    2 as gint,
                    (safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
                        % 100 as ::core::ffi::c_int) as guint32,
                );
            }
            5916212523694105379 => {
                safe_c2rust_format_number(
                    outstr,
                    alt_digits,
                    ::core::ptr::null::<gchar>(),
                    0 as gint,
                    safe_c2rust_g_date_time_get_year(datetime) as guint32,
                );
            }
            _ => {
                safe_c2rust_format_number(
                    outstr,
                    alt_digits,
                    if pad_set != 0 {
                        pad
                    } else {
                        b"0\0" as *const u8 as *const gchar
                    },
                    2 as gint,
                    (safe_c2rust_g_date_time_get_year(datetime) as ::core::ffi::c_int
                        / 100 as ::core::ffi::c_int) as guint32,
                );
            }
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_format(
    mut datetime: *mut GDateTime,
    mut format: *const gchar,
) -> *mut gchar {
    let mut outstr: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    let mut time_is_utf8_compatible: gboolean = (_g_get_time_charset(&raw mut charset) != 0
        || g_strcmp0(
            b"ASCII\0" as *const u8 as *const ::core::ffi::c_char,
            charset as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || g_strcmp0(
            b"ANSI_X3.4-1968\0" as *const u8 as *const ::core::ffi::c_char,
            charset as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if g_utf8_validate(
            format,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
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
            b"g_utf8_validate (format, -1, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    outstr = g_string_sized_new(
        (strlen(format as *const ::core::ffi::c_char) as gsize).wrapping_mul(2 as gsize),
    );
    if safe_c2rust_g_date_time_format_utf8(datetime, format, outstr, time_is_utf8_compatible) == 0 {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(outstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(outstr);
            };
        } else {
            g_string_free(outstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return ::core::ptr::null_mut::<gchar>();
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(outstr, 0 as gboolean)
        } else {
            g_string_free_and_steal(outstr)
        }
    } else {
        g_string_free(outstr, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_time_format_iso8601(
    mut datetime: *mut GDateTime,
) -> *mut gchar {
    let mut outstr: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut main_date: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut offset: gint64 = 0;
    let mut format: *mut gchar =
        b"%C%y-%m-%dT%H:%M:%S\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !datetime.is_null() {
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
            b"datetime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if (*datetime).usec.wrapping_rem(G_TIME_SPAN_SECOND as guint64) != 0 as guint64 {
        format =
            b"%C%y-%m-%dT%H:%M:%S.%f\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
    }
    main_date = safe_c2rust_g_date_time_format(datetime, format);
    outstr = g_string_new(main_date);
    g_free(main_date as gpointer);
    offset = safe_c2rust_g_date_time_get_utc_offset(datetime) as gint64;
    if offset == 0 as gint64 {
        safe_c2rust_g_string_append_c_inline(outstr, 'Z' as i32 as gchar);
    } else {
        let mut time_zone: *mut gchar =
            safe_c2rust_g_date_time_format(datetime, b"%:::z\0" as *const u8 as *const gchar);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = time_zone;
                safe_c2rust_g_string_append_len_inline(
                    outstr,
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
                outstr,
                time_zone,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(time_zone as gpointer);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(outstr, 0 as gboolean)
        } else {
            g_string_free_and_steal(outstr)
        }
    } else {
        g_string_free(outstr, 0 as gboolean)
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_0 as gpointer;
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
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_date_time_unref\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
