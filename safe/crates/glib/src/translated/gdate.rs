use ::c2rust_bitfields;
extern "C" {
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_error_free(error: *mut GError);
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_locale_from_utf8(
        utf8string: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_utf8_find_prev_char(str: *const gchar, p: *const gchar) -> *mut gchar;
    fn g_utf8_validate_len(str: *const gchar, max_len: gsize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_casefold(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_normalize(str: *const gchar, len: gssize, mode: GNormalizeMode) -> *mut gchar;
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
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
pub type GQuark = guint32;
pub type GTime = gint32;
pub type GDateYear = guint16;
pub type GDateDay = guint8;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GDate {
    #[bitfield(name = "julian_days", ty = "guint", bits = "0..=31")]
    #[bitfield(name = "julian", ty = "guint", bits = "32..=32")]
    #[bitfield(name = "dmy", ty = "guint", bits = "33..=33")]
    #[bitfield(name = "day", ty = "guint", bits = "34..=39")]
    #[bitfield(name = "month", ty = "guint", bits = "40..=43")]
    #[bitfield(name = "year", ty = "guint", bits = "44..=59")]
    pub julian_days_julian_dmy_day_month_year: [u8; 8],
}
pub type GDate = _GDate;
pub type GDateDMY = ::core::ffi::c_uint;
pub const G_DATE_YEAR: GDateDMY = 2;
pub const G_DATE_MONTH: GDateDMY = 1;
pub const G_DATE_DAY: GDateDMY = 0;
pub type GDateWeekday = ::core::ffi::c_uint;
pub const G_DATE_SUNDAY: GDateWeekday = 7;
pub const G_DATE_SATURDAY: GDateWeekday = 6;
pub const G_DATE_FRIDAY: GDateWeekday = 5;
pub const G_DATE_THURSDAY: GDateWeekday = 4;
pub const G_DATE_WEDNESDAY: GDateWeekday = 3;
pub const G_DATE_TUESDAY: GDateWeekday = 2;
pub const G_DATE_MONDAY: GDateWeekday = 1;
pub const G_DATE_BAD_WEEKDAY: GDateWeekday = 0;
pub type GDateMonth = ::core::ffi::c_uint;
pub const G_DATE_DECEMBER: GDateMonth = 12;
pub const G_DATE_NOVEMBER: GDateMonth = 11;
pub const G_DATE_OCTOBER: GDateMonth = 10;
pub const G_DATE_SEPTEMBER: GDateMonth = 9;
pub const G_DATE_AUGUST: GDateMonth = 8;
pub const G_DATE_JULY: GDateMonth = 7;
pub const G_DATE_JUNE: GDateMonth = 6;
pub const G_DATE_MAY: GDateMonth = 5;
pub const G_DATE_APRIL: GDateMonth = 4;
pub const G_DATE_MARCH: GDateMonth = 3;
pub const G_DATE_FEBRUARY: GDateMonth = 2;
pub const G_DATE_JANUARY: GDateMonth = 1;
pub const G_DATE_BAD_MONTH: GDateMonth = 0;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GDateParseTokens = _GDateParseTokens;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDateParseTokens {
    pub num_ints: gint,
    pub n: [gint; 3],
    pub month: guint,
}
pub type GNormalizeMode = ::core::ffi::c_uint;
pub const G_NORMALIZE_NFKC: GNormalizeMode = 3;
pub const G_NORMALIZE_ALL_COMPOSE: GNormalizeMode = 3;
pub const G_NORMALIZE_NFKD: GNormalizeMode = 2;
pub const G_NORMALIZE_ALL: GNormalizeMode = 2;
pub const G_NORMALIZE_NFC: GNormalizeMode = 1;
pub const G_NORMALIZE_DEFAULT_COMPOSE: GNormalizeMode = 1;
pub const G_NORMALIZE_NFD: GNormalizeMode = 0;
pub const G_NORMALIZE_DEFAULT: GNormalizeMode = 0;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LC_TIME: ::core::ffi::c_int = __LC_TIME;
pub const G_DATE_BAD_JULIAN: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const G_DATE_BAD_DAY: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const G_DATE_BAD_YEAR: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_new() -> *mut GDate {
    let mut d: *mut GDate = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDate>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GDate;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_new_dmy(
    mut day: GDateDay,
    mut m: GDateMonth,
    mut y: GDateYear,
) -> *mut GDate {
    let mut d: *mut GDate = ::core::ptr::null_mut::<GDate>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_dmy(day, m, y) != 0 {
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
            b"g_date_valid_dmy (day, m, y)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDate>();
    }
    d = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDate>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GDate;
    (*d).set_julian(FALSE as guint as guint);
    (*d).set_dmy(TRUE as guint as guint);
    (*d).set_month(m as guint as guint);
    (*d).set_day(day as guint as guint);
    (*d).set_year(y as guint as guint);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"../original/glib/gdate.c\0" as *const u8 as *const ::core::ffi::c_char,
            302 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_new_julian(mut julian_day: guint32) -> *mut GDate {
    let mut d: *mut GDate = ::core::ptr::null_mut::<GDate>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_julian(julian_day) != 0 {
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
            b"g_date_valid_julian (julian_day)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDate>();
    }
    d = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDate>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GDate;
    (*d).set_julian(TRUE as guint as guint);
    (*d).set_dmy(FALSE as guint as guint);
    (*d).set_julian_days(julian_day as guint as guint);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"../original/glib/gdate.c\0" as *const u8 as *const ::core::ffi::c_char,
            333 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_free(mut date: *mut GDate) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !date.is_null() {
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
            b"date != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free(date as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_copy(mut date: *const GDate) -> *mut GDate {
    let mut res: *mut GDate = ::core::ptr::null_mut::<GDate>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !date.is_null() {
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
            b"date != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDate>();
    }
    if safe_c2rust_g_date_valid(date) != 0 {
        res = safe_c2rust_g_date_new_julian(safe_c2rust_g_date_get_julian(date));
    } else {
        res = safe_c2rust_g_date_new();
        *res = *date;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid(mut d: *const GDate) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*d).julian() as ::core::ffi::c_int != 0 || (*d).dmy() as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
static mut safe_c2rust_days_in_months: [[guint8; 13]; 2] = [
    [
        0 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        28 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
    ],
    [
        0 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        29 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
        30 as ::core::ffi::c_int as guint8,
        31 as ::core::ffi::c_int as guint8,
    ],
];
static mut safe_c2rust_days_in_year: [[guint16; 14]; 2] = [
    [
        0 as ::core::ffi::c_int as guint16,
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_month(mut m: GDateMonth) -> gboolean {
    return (m as gint > G_DATE_BAD_MONTH as ::core::ffi::c_int
        && (m as gint) < 13 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_year(mut y: GDateYear) -> gboolean {
    return (y as ::core::ffi::c_uint > G_DATE_BAD_YEAR) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_day(mut d: GDateDay) -> gboolean {
    return (d as ::core::ffi::c_uint > G_DATE_BAD_DAY
        && (d as ::core::ffi::c_int) < 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_weekday(mut w: GDateWeekday) -> gboolean {
    return (w as gint > G_DATE_BAD_WEEKDAY as ::core::ffi::c_int
        && (w as gint) < 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_julian(mut j: guint32) -> gboolean {
    return (j > G_DATE_BAD_JULIAN) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_valid_dmy(
    mut d: GDateDay,
    mut m: GDateMonth,
    mut y: GDateYear,
) -> gboolean {
    return (m as ::core::ffi::c_uint
        > G_DATE_BAD_MONTH as ::core::ffi::c_int as ::core::ffi::c_uint
        && (m as ::core::ffi::c_uint) < 13 as ::core::ffi::c_uint
        && d as ::core::ffi::c_uint > G_DATE_BAD_DAY
        && y as ::core::ffi::c_uint > G_DATE_BAD_YEAR
        && d as ::core::ffi::c_int
            <= (if safe_c2rust_g_date_is_leap_year(y) != 0 {
                safe_c2rust_days_in_months[1 as ::core::ffi::c_int as usize][m as usize]
                    as ::core::ffi::c_int
            } else {
                safe_c2rust_days_in_months[0 as ::core::ffi::c_int as usize][m as usize]
                    as ::core::ffi::c_int
            })) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_date_update_julian(mut const_d: *const GDate) {
    let mut d: *mut GDate = const_d as *mut GDate;
    let mut year: GDateYear = 0;
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
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
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*d).julian() == 0 {
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
            b"!d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_dmy(
            (*d).day() as GDateDay,
            (*d).month() as GDateMonth,
            (*d).year() as GDateYear,
        ) != 0
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
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_dmy (d->day, d->month, d->year)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    year = ((*d).year() as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as GDateYear;
    (*d).set_julian_days(
        (year as ::core::ffi::c_uint).wrapping_mul(365 as ::core::ffi::c_uint) as guint as guint,
    );
    let mut rhs = (*d).julian_days() + {
        year = (year as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as GDateYear;
        year as ::core::ffi::c_uint
    } as guint;
    (*d).set_julian_days(rhs);
    let mut rhs_0 = (*d).julian_days() - {
        year = (year as ::core::ffi::c_int / 25 as ::core::ffi::c_int) as GDateYear;
        year as ::core::ffi::c_uint
    } as guint;
    (*d).set_julian_days(rhs_0);
    (*d).set_julian_days(
        (*d).julian_days()
            + (year as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
                as guint,
    );
    idx = (if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    (*d).set_julian_days(
        (*d).julian_days()
            + (safe_c2rust_days_in_year[idx as usize][(*d).month() as usize] as ::core::ffi::c_int
                + (*d).day() as ::core::ffi::c_int) as ::core::ffi::c_uint as guint,
    );
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_julian((*d).julian_days() as guint32) != 0 {
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
            b"g_date_valid_julian (d->julian_days)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_julian(TRUE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_date_update_dmy(mut const_d: *const GDate) {
    let mut d: *mut GDate = const_d as *mut GDate;
    let mut y: GDateYear = 0;
    let mut m: GDateMonth = G_DATE_BAD_MONTH;
    let mut day: GDateDay = 0;
    let mut A: guint32 = 0;
    let mut B: guint32 = 0;
    let mut C: guint32 = 0;
    let mut D: guint32 = 0;
    let mut E: guint32 = 0;
    let mut M: guint32 = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*d).dmy() == 0 {
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
            b"!d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_julian((*d).julian_days() as guint32) != 0 {
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
            b"g_date_valid_julian (d->julian_days)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    A = ((*d).julian_days() as ::core::ffi::c_uint)
        .wrapping_add(1721425 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(32045 as ::core::ffi::c_uint) as guint32;
    B = (4 as guint32)
        .wrapping_mul(A.wrapping_add(36524 as guint32))
        .wrapping_div(146097 as ::core::ffi::c_int as guint32)
        .wrapping_sub(1 as guint32);
    C = A.wrapping_sub(
        (146097 as ::core::ffi::c_int as guint32)
            .wrapping_mul(B)
            .wrapping_div(4 as guint32),
    );
    D = (4 as guint32)
        .wrapping_mul(C.wrapping_add(365 as guint32))
        .wrapping_div(1461 as guint32)
        .wrapping_sub(1 as guint32);
    E = C.wrapping_sub((1461 as guint32).wrapping_mul(D).wrapping_div(4 as guint32));
    M = (5 as guint32)
        .wrapping_mul(E.wrapping_sub(1 as guint32))
        .wrapping_add(2 as guint32)
        .wrapping_div(153 as guint32);
    m = M
        .wrapping_add(3 as guint32)
        .wrapping_sub((12 as guint32).wrapping_mul(M.wrapping_div(10 as guint32)))
        as GDateMonth;
    day = E.wrapping_sub(
        (153 as guint32)
            .wrapping_mul(M)
            .wrapping_add(2 as guint32)
            .wrapping_div(5 as guint32),
    ) as GDateDay;
    y = (100 as guint32)
        .wrapping_mul(B)
        .wrapping_add(D)
        .wrapping_sub(4800 as guint32)
        .wrapping_add(M.wrapping_div(10 as guint32)) as GDateYear;
    (*d).set_month(m as guint as guint);
    (*d).set_day(day as guint as guint);
    (*d).set_year(y as guint as guint);
    (*d).set_dmy(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_weekday(mut d: *const GDate) -> GDateWeekday {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATE_BAD_WEEKDAY;
    }
    if (*d).julian() == 0 {
        safe_c2rust_g_date_update_julian(d);
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATE_BAD_WEEKDAY;
    }
    return ((*d).julian_days() as ::core::ffi::c_uint)
        .wrapping_sub(1 as ::core::ffi::c_uint)
        .wrapping_rem(7 as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint) as GDateWeekday;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_month(mut d: *const GDate) -> GDateMonth {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATE_BAD_MONTH;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATE_BAD_MONTH;
    }
    return (*d).month() as GDateMonth;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_year(mut d: *const GDate) -> GDateYear {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GDateYear;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GDateYear;
    }
    return (*d).year() as GDateYear;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_day(mut d: *const GDate) -> GDateDay {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GDateDay;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GDateDay;
    }
    return (*d).day() as GDateDay;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_julian(mut d: *const GDate) -> guint32 {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if (*d).julian() == 0 {
        safe_c2rust_g_date_update_julian(d);
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return (*d).julian_days() as guint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_day_of_year(mut d: *const GDate) -> guint {
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    idx = (if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    return (safe_c2rust_days_in_year[idx as usize][(*d).month() as usize] as ::core::ffi::c_int
        + (*d).day() as ::core::ffi::c_int) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_monday_week_of_year(mut d: *const GDate) -> guint {
    let mut wd: GDateWeekday = G_DATE_BAD_WEEKDAY;
    let mut day: guint = 0;
    let mut first: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    safe_c2rust_g_date_clear(&raw mut first, 1 as guint);
    safe_c2rust_g_date_set_dmy(
        &raw mut first,
        1 as GDateDay,
        G_DATE_JANUARY,
        (*d).year() as GDateYear,
    );
    wd = (safe_c2rust_g_date_get_weekday(&raw mut first) as ::core::ffi::c_uint)
        .wrapping_sub(1 as ::core::ffi::c_uint) as GDateWeekday;
    day = safe_c2rust_g_date_get_day_of_year(d).wrapping_sub(1 as guint);
    return day
        .wrapping_add(wd as guint)
        .wrapping_div(7 as guint)
        .wrapping_add(
            (if wd as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as guint,
        );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_sunday_week_of_year(mut d: *const GDate) -> guint {
    let mut wd: GDateWeekday = G_DATE_BAD_WEEKDAY;
    let mut day: guint = 0;
    let mut first: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    safe_c2rust_g_date_clear(&raw mut first, 1 as guint);
    safe_c2rust_g_date_set_dmy(
        &raw mut first,
        1 as GDateDay,
        G_DATE_JANUARY,
        (*d).year() as GDateYear,
    );
    wd = safe_c2rust_g_date_get_weekday(&raw mut first);
    if wd as ::core::ffi::c_uint == 7 as ::core::ffi::c_uint {
        wd = G_DATE_BAD_WEEKDAY;
    }
    day = safe_c2rust_g_date_get_day_of_year(d).wrapping_sub(1 as guint);
    return day
        .wrapping_add(wd as guint)
        .wrapping_div(7 as guint)
        .wrapping_add(
            (if wd as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as guint,
        );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_iso8601_week_of_year(mut d: *const GDate) -> guint {
    let mut j: guint = 0;
    let mut d4: guint = 0;
    let mut L: guint = 0;
    let mut d1: guint = 0;
    let mut w: guint = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*d).julian() == 0 {
        safe_c2rust_g_date_update_julian(d);
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    j = ((*d).julian_days() as ::core::ffi::c_uint)
        .wrapping_add(1721425 as ::core::ffi::c_int as ::core::ffi::c_uint) as guint;
    d4 = j
        .wrapping_add(31741 as guint)
        .wrapping_sub(j.wrapping_rem(7 as guint))
        .wrapping_rem(146097 as ::core::ffi::c_int as guint)
        .wrapping_rem(36524 as guint)
        .wrapping_rem(1461 as guint);
    L = d4.wrapping_div(1460 as guint);
    d1 = d4
        .wrapping_sub(L)
        .wrapping_rem(365 as guint)
        .wrapping_add(L);
    w = d1.wrapping_div(7 as guint).wrapping_add(1 as guint);
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_days_between(
    mut d1: *const GDate,
    mut d2: *const GDate,
) -> gint {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d1) != 0 {
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
            b"g_date_valid (d1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d2) != 0 {
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
            b"g_date_valid (d2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return safe_c2rust_g_date_get_julian(d2) as gint - safe_c2rust_g_date_get_julian(d1) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_clear(mut d: *mut GDate, mut ndates: guint) {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ndates != 0 as guint {
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
            b"ndates != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        d as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (ndates as size_t).wrapping_mul(::core::mem::size_of::<GDate>() as size_t),
    );
}
static mut safe_c2rust_g__g_date_global_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_long_month_names: [*mut gchar; 13] = [
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
];
static mut safe_c2rust_long_month_names_alternative: [*mut gchar; 13] = [
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
];
static mut safe_c2rust_short_month_names: [*mut gchar; 13] = [
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
];
static mut safe_c2rust_short_month_names_alternative: [*mut gchar; 13] = [
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
    ::core::ptr::null::<gchar>() as *mut gchar,
];
static mut safe_c2rust_current_locale: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_dmy_order: [GDateDMY; 3] = [G_DATE_DAY, G_DATE_MONTH, G_DATE_YEAR];
static mut safe_c2rust_twodigit_start_year: GDateYear = 1930 as GDateYear;
static mut safe_c2rust_using_twodigit_years: gboolean = FALSE;
static mut safe_c2rust_locale_era_adjust: gint = 0 as gint;
#[inline]
unsafe extern "C" fn safe_c2rust_update_month_match(
    mut longest: *mut gsize,
    mut haystack: *const gchar,
    mut needle: *const gchar,
) -> gboolean {
    let mut length: gsize = 0;
    if needle.is_null() {
        return FALSE;
    }
    length = strlen(needle as *const ::core::ffi::c_char) as gsize;
    if *longest >= length {
        return FALSE;
    }
    if strstr(
        haystack as *const ::core::ffi::c_char,
        needle as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return FALSE;
    }
    *longest = length;
    return TRUE;
}
pub const NUM_LEN: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_date_fill_parse_tokens(
    mut str: *const gchar,
    mut pt: *mut GDateParseTokens,
) {
    let mut num: [[gchar; 11]; 4] = [[0; 11]; 4];
    let mut i: gint = 0;
    let mut s: *const guchar = ::core::ptr::null::<guchar>();
    num[3 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    num[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
        num[3 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
    num[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
        num[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
    num[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
        num[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
    s = str as *const guchar;
    (*pt).num_ints = 0 as ::core::ffi::c_int as gint;
    while *s as ::core::ffi::c_int != 0 && (*pt).num_ints < 4 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int as gint;
        while *s as ::core::ffi::c_int != 0
            && *safe_c2rust_g_ascii_table.offset(*s as isize) as ::core::ffi::c_int
                & G_ASCII_DIGIT as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            && i < NUM_LEN
        {
            num[(*pt).num_ints as usize][i as usize] = *s as gchar;
            s = s.offset(1);
            i += 1;
        }
        if i > 0 as ::core::ffi::c_int {
            num[(*pt).num_ints as usize][i as usize] = '\0' as i32 as gchar;
            (*pt).num_ints += 1;
        }
        if *s as ::core::ffi::c_int == '\0' as i32 {
            break;
        }
        s = s.offset(1);
    }
    (*pt).n[0 as ::core::ffi::c_int as usize] = (if (*pt).num_ints > 0 as ::core::ffi::c_int {
        safe_c2rust_atoi(
            &raw mut *(&raw mut num as *mut [gchar; 11]).offset(0 as ::core::ffi::c_int as isize)
                as *mut gchar,
        )
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    (*pt).n[1 as ::core::ffi::c_int as usize] = (if (*pt).num_ints > 1 as ::core::ffi::c_int {
        safe_c2rust_atoi(
            &raw mut *(&raw mut num as *mut [gchar; 11]).offset(1 as ::core::ffi::c_int as isize)
                as *mut gchar,
        )
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    (*pt).n[2 as ::core::ffi::c_int as usize] = (if (*pt).num_ints > 2 as ::core::ffi::c_int {
        safe_c2rust_atoi(
            &raw mut *(&raw mut num as *mut [gchar; 11]).offset(2 as ::core::ffi::c_int as isize)
                as *mut gchar,
        )
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    (*pt).month = G_DATE_BAD_MONTH as ::core::ffi::c_int as guint;
    if (*pt).num_ints < 3 as ::core::ffi::c_int {
        let mut longest: gsize = 0 as gsize;
        let mut casefold: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut normalized: *mut gchar = ::core::ptr::null_mut::<gchar>();
        casefold = g_utf8_casefold(str, -(1 as ::core::ffi::c_int) as gssize);
        normalized = g_utf8_normalize(
            casefold,
            -(1 as ::core::ffi::c_int) as gssize,
            G_NORMALIZE_ALL,
        );
        g_free(casefold as gpointer);
        i = 1 as ::core::ffi::c_int as gint;
        while i < 13 as ::core::ffi::c_int {
            if safe_c2rust_update_month_match(
                &raw mut longest,
                normalized,
                safe_c2rust_long_month_names[i as usize],
            ) != 0
            {
                (*pt).month = i as guint;
            }
            if safe_c2rust_update_month_match(
                &raw mut longest,
                normalized,
                safe_c2rust_long_month_names_alternative[i as usize],
            ) != 0
            {
                (*pt).month = i as guint;
            }
            if safe_c2rust_update_month_match(
                &raw mut longest,
                normalized,
                safe_c2rust_short_month_names[i as usize],
            ) != 0
            {
                (*pt).month = i as guint;
            }
            if safe_c2rust_update_month_match(
                &raw mut longest,
                normalized,
                safe_c2rust_short_month_names_alternative[i as usize],
            ) != 0
            {
                (*pt).month = i as guint;
            }
            i += 1;
        }
        g_free(normalized as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_date_prepare_to_parse(
    mut str: *const gchar,
    mut pt: *mut GDateParseTokens,
) {
    let mut locale: *const gchar = setlocale(LC_TIME, ::core::ptr::null::<::core::ffi::c_char>());
    let mut recompute_localeinfo: gboolean = FALSE;
    let mut d: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !locale.is_null() {
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
            b"locale != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_date_clear(&raw mut d, 1 as guint);
    if safe_c2rust_current_locale.is_null()
        || strcmp(
            locale as *const ::core::ffi::c_char,
            safe_c2rust_current_locale,
        ) != 0 as ::core::ffi::c_int
    {
        recompute_localeinfo = TRUE as gboolean;
    }
    if recompute_localeinfo != 0 {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut testpt: GDateParseTokens = _GDateParseTokens {
            num_ints: 0,
            n: [0; 3],
            month: 0,
        };
        let mut buf: [gchar; 128] = [0; 128];
        g_free(safe_c2rust_current_locale as gpointer);
        safe_c2rust_current_locale =
            safe_c2rust_g_strdup_inline(locale as *const ::core::ffi::c_char) as *mut gchar;
        safe_c2rust_short_month_names[0 as ::core::ffi::c_int as usize] =
            b"Error\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
        safe_c2rust_long_month_names[0 as ::core::ffi::c_int as usize] =
            b"Error\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
        while i < 13 as ::core::ffi::c_int {
            let mut casefold: *mut gchar = ::core::ptr::null_mut::<gchar>();
            safe_c2rust_g_date_set_dmy(
                &raw mut d,
                1 as GDateDay,
                i as GDateMonth,
                1976 as GDateYear,
            );
            if ({
                let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
                if safe_c2rust_g_date_valid(&raw mut d) != 0 {
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
                    b"g_date_valid (&d)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
            safe_c2rust_g_date_strftime(
                &raw mut buf as *mut gchar,
                127 as gsize,
                b"%b\0" as *const u8 as *const gchar,
                &raw mut d,
            );
            casefold = g_utf8_casefold(
                &raw mut buf as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
            g_free(safe_c2rust_short_month_names[i as usize] as gpointer);
            safe_c2rust_short_month_names[i as usize] = g_utf8_normalize(
                casefold,
                -(1 as ::core::ffi::c_int) as gssize,
                G_NORMALIZE_ALL,
            );
            g_free(casefold as gpointer);
            safe_c2rust_g_date_strftime(
                &raw mut buf as *mut gchar,
                127 as gsize,
                b"%B\0" as *const u8 as *const gchar,
                &raw mut d,
            );
            casefold = g_utf8_casefold(
                &raw mut buf as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
            g_free(safe_c2rust_long_month_names[i as usize] as gpointer);
            safe_c2rust_long_month_names[i as usize] = g_utf8_normalize(
                casefold,
                -(1 as ::core::ffi::c_int) as gssize,
                G_NORMALIZE_ALL,
            );
            g_free(casefold as gpointer);
            safe_c2rust_g_date_strftime(
                &raw mut buf as *mut gchar,
                127 as gsize,
                b"%Ob\0" as *const u8 as *const gchar,
                &raw mut d,
            );
            casefold = g_utf8_casefold(
                &raw mut buf as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
            g_free(safe_c2rust_short_month_names_alternative[i as usize] as gpointer);
            safe_c2rust_short_month_names_alternative[i as usize] = g_utf8_normalize(
                casefold,
                -(1 as ::core::ffi::c_int) as gssize,
                G_NORMALIZE_ALL,
            );
            g_free(casefold as gpointer);
            safe_c2rust_g_date_strftime(
                &raw mut buf as *mut gchar,
                127 as gsize,
                b"%OB\0" as *const u8 as *const gchar,
                &raw mut d,
            );
            casefold = g_utf8_casefold(
                &raw mut buf as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
            g_free(safe_c2rust_long_month_names_alternative[i as usize] as gpointer);
            safe_c2rust_long_month_names_alternative[i as usize] = g_utf8_normalize(
                casefold,
                -(1 as ::core::ffi::c_int) as gssize,
                G_NORMALIZE_ALL,
            );
            g_free(casefold as gpointer);
            i += 1;
        }
        safe_c2rust_g_date_set_dmy(&raw mut d, 4 as GDateDay, G_DATE_JULY, 1976 as GDateYear);
        safe_c2rust_g_date_strftime(
            &raw mut buf as *mut gchar,
            127 as gsize,
            b"%x\0" as *const u8 as *const gchar,
            &raw mut d,
        );
        safe_c2rust_g_date_fill_parse_tokens(&raw mut buf as *mut gchar, &raw mut testpt);
        safe_c2rust_using_twodigit_years = FALSE as gboolean;
        safe_c2rust_locale_era_adjust = 0 as ::core::ffi::c_int as gint;
        safe_c2rust_dmy_order[0 as ::core::ffi::c_int as usize] = G_DATE_DAY;
        safe_c2rust_dmy_order[1 as ::core::ffi::c_int as usize] = G_DATE_MONTH;
        safe_c2rust_dmy_order[2 as ::core::ffi::c_int as usize] = G_DATE_YEAR;
        i = 0 as ::core::ffi::c_int;
        while i < testpt.num_ints {
            let mut current_block_60: u64;
            match testpt.n[i as usize] {
                7 => {
                    safe_c2rust_dmy_order[i as usize] = G_DATE_MONTH;
                    current_block_60 = 16738040538446813684;
                }
                4 => {
                    safe_c2rust_dmy_order[i as usize] = G_DATE_DAY;
                    current_block_60 = 16738040538446813684;
                }
                76 => {
                    safe_c2rust_using_twodigit_years = TRUE as gboolean;
                    current_block_60 = 13252443776852304843;
                }
                1976 => {
                    current_block_60 = 13252443776852304843;
                }
                _ => {
                    safe_c2rust_locale_era_adjust = 1976 as gint - testpt.n[i as usize];
                    safe_c2rust_dmy_order[i as usize] = G_DATE_YEAR;
                    current_block_60 = 16738040538446813684;
                }
            }
            match current_block_60 {
                13252443776852304843 => {
                    safe_c2rust_dmy_order[i as usize] = G_DATE_YEAR;
                }
                _ => {}
            }
            i += 1;
        }
    }
    safe_c2rust_g_date_fill_parse_tokens(str, pt);
}
unsafe extern "C" fn safe_c2rust_convert_twodigit_year(mut y: guint) -> guint {
    if safe_c2rust_using_twodigit_years != 0 && y < 100 as guint {
        let mut two: guint = (safe_c2rust_twodigit_start_year as ::core::ffi::c_int
            % 100 as ::core::ffi::c_int) as guint;
        let mut century: guint = (safe_c2rust_twodigit_start_year as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int) as guint;
        if y < two {
            century = century.wrapping_add(100 as guint);
        }
        y = y.wrapping_add(century);
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_parse(mut d: *mut GDate, mut str: *const gchar) {
    let mut pt: GDateParseTokens = _GDateParseTokens {
        num_ints: 0,
        n: [0; 3],
        month: 0,
    };
    let mut m: guint = G_DATE_BAD_MONTH as ::core::ffi::c_int as guint;
    let mut day: guint = G_DATE_BAD_DAY;
    let mut y: guint = G_DATE_BAD_YEAR;
    let mut str_len: gsize = 0;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_date_clear(d, 1 as guint);
    str_len = strlen(str as *const ::core::ffi::c_char) as gsize;
    if str_len > 200 as gsize {
        return;
    }
    if g_utf8_validate_len(str, str_len, ::core::ptr::null_mut::<*const gchar>()) == 0 {
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_date_global_lock);
    safe_c2rust_g_date_prepare_to_parse(str, &raw mut pt);
    if pt.num_ints == 4 as ::core::ffi::c_int {
        g_mutex_unlock(&raw mut safe_c2rust_g__g_date_global_lock);
        return;
    }
    if pt.num_ints > 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if ({
            let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
            if pt.num_ints < 4 as ::core::ffi::c_int {
                _g_boolean_var_50 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_50 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_50
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gdate.c\0" as *const u8 as *const ::core::ffi::c_char,
                1284 as ::core::ffi::c_int,
                G_STRFUNC,
                b"pt.num_ints < 4\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        while i < pt.num_ints && j < 3 as ::core::ffi::c_int {
            match safe_c2rust_dmy_order[j as usize] as ::core::ffi::c_uint {
                1 => {
                    if pt.num_ints == 2 as ::core::ffi::c_int
                        && pt.month != G_DATE_BAD_MONTH as ::core::ffi::c_int as guint
                    {
                        m = pt.month;
                        j += 1;
                        continue;
                    } else {
                        m = pt.n[i as usize] as guint;
                    }
                }
                0 => {
                    if pt.num_ints == 2 as ::core::ffi::c_int
                        && pt.month == G_DATE_BAD_MONTH as ::core::ffi::c_int as guint
                    {
                        day = 1 as guint;
                        j += 1;
                        continue;
                    } else {
                        day = pt.n[i as usize] as guint;
                    }
                }
                2 => {
                    y = pt.n[i as usize] as guint;
                    if safe_c2rust_locale_era_adjust != 0 as ::core::ffi::c_int {
                        y = y.wrapping_add(safe_c2rust_locale_era_adjust as guint);
                    }
                    y = safe_c2rust_convert_twodigit_year(y);
                }
                _ => {}
            }
            i += 1;
            j += 1;
        }
        if pt.num_ints == 3 as ::core::ffi::c_int
            && safe_c2rust_g_date_valid_dmy(day as GDateDay, m as GDateMonth, y as GDateYear) == 0
        {
            y = pt.n[0 as ::core::ffi::c_int as usize] as guint;
            m = pt.n[1 as ::core::ffi::c_int as usize] as guint;
            day = pt.n[2 as ::core::ffi::c_int as usize] as guint;
            if safe_c2rust_using_twodigit_years != 0 && y < 100 as guint {
                y = G_DATE_BAD_YEAR as guint;
            }
        } else if pt.num_ints == 2 as ::core::ffi::c_int {
            if m == G_DATE_BAD_MONTH as ::core::ffi::c_int as guint
                && pt.month != G_DATE_BAD_MONTH as ::core::ffi::c_int as guint
            {
                m = pt.month;
            }
        }
    } else if pt.num_ints == 1 as ::core::ffi::c_int {
        if pt.month != G_DATE_BAD_MONTH as ::core::ffi::c_int as guint {
            m = pt.month;
            day = 1 as guint;
            y = pt.n[0 as ::core::ffi::c_int as usize] as guint;
        } else {
            m = (pt.n[0 as ::core::ffi::c_int as usize] / 100 as ::core::ffi::c_int
                % 100 as ::core::ffi::c_int) as guint;
            day = (pt.n[0 as ::core::ffi::c_int as usize] % 100 as ::core::ffi::c_int) as guint;
            y = (pt.n[0 as ::core::ffi::c_int as usize] / 10000 as ::core::ffi::c_int) as guint;
            y = safe_c2rust_convert_twodigit_year(y);
        }
    }
    if y < 8000 as guint
        && safe_c2rust_g_date_valid_dmy(day as GDateDay, m as GDateMonth, y as GDateYear) != 0
    {
        (*d).set_month(m as guint);
        (*d).set_day(day as guint);
        (*d).set_year(y as guint);
        (*d).set_dmy(TRUE as guint as guint);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_date_global_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_localtime(
    mut timet: time_t,
    mut out_tm: *mut tm,
) -> gboolean {
    let mut success: gboolean = TRUE;
    if localtime_r(&raw mut timet, out_tm).is_null() {
        success = FALSE as gboolean;
    }
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_time_t(mut date: *mut GDate, mut timet: time_t) {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !date.is_null() {
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
            b"date != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    success = safe_c2rust__g_localtime(timet, &raw mut tm);
    if success == 0 {
        tm.tm_mon = 0 as ::core::ffi::c_int;
        tm.tm_mday = 1 as ::core::ffi::c_int;
        tm.tm_year = 100 as ::core::ffi::c_int;
    }
    (*date).set_julian(FALSE as guint as guint);
    (*date).set_month((tm.tm_mon + 1 as ::core::ffi::c_int) as guint as guint);
    (*date).set_day(tm.tm_mday as guint as guint);
    (*date).set_year((tm.tm_year + 1900 as ::core::ffi::c_int) as guint as guint);
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_dmy(
            (*date).day() as GDateDay,
            (*date).month() as GDateMonth,
            (*date).year() as GDateYear,
        ) != 0
        {
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
            b"g_date_valid_dmy (date->day, date->month, date->year)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*date).set_dmy(TRUE as guint as guint);
    if success == 0 {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            b"g_date_set_time\0" as *const u8 as *const ::core::ffi::c_char,
            b"localtime() == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_time(mut date: *mut GDate, mut time_: GTime) {
    safe_c2rust_g_date_set_time_t(date, time_ as time_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_time_val(
    mut date: *mut GDate,
    mut timeval: *mut GTimeVal,
) {
    safe_c2rust_g_date_set_time_t(date, (*timeval).tv_sec);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_month(mut d: *mut GDate, mut m: GDateMonth) {
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_month(m) != 0 {
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
            b"g_date_valid_month (m)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).julian() as ::core::ffi::c_int != 0 && (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    (*d).set_julian(FALSE as guint as guint);
    (*d).set_month(m as guint as guint);
    if safe_c2rust_g_date_valid_dmy(
        (*d).day() as GDateDay,
        (*d).month() as GDateMonth,
        (*d).year() as GDateYear,
    ) != 0
    {
        (*d).set_dmy(TRUE as guint as guint);
    } else {
        (*d).set_dmy(FALSE as guint as guint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_day(mut d: *mut GDate, mut day: GDateDay) {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_day(day) != 0 {
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
            b"g_date_valid_day (day)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).julian() as ::core::ffi::c_int != 0 && (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    (*d).set_julian(FALSE as guint as guint);
    (*d).set_day(day as guint as guint);
    if safe_c2rust_g_date_valid_dmy(
        (*d).day() as GDateDay,
        (*d).month() as GDateMonth,
        (*d).year() as GDateYear,
    ) != 0
    {
        (*d).set_dmy(TRUE as guint as guint);
    } else {
        (*d).set_dmy(FALSE as guint as guint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_year(mut d: *mut GDate, mut y: GDateYear) {
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_year(y) != 0 {
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
            b"g_date_valid_year (y)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).julian() as ::core::ffi::c_int != 0 && (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    (*d).set_julian(FALSE as guint as guint);
    (*d).set_year(y as guint as guint);
    if safe_c2rust_g_date_valid_dmy(
        (*d).day() as GDateDay,
        (*d).month() as GDateMonth,
        (*d).year() as GDateYear,
    ) != 0
    {
        (*d).set_dmy(TRUE as guint as guint);
    } else {
        (*d).set_dmy(FALSE as guint as guint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_dmy(
    mut d: *mut GDate,
    mut day: GDateDay,
    mut m: GDateMonth,
    mut y: GDateYear,
) {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_dmy(day, m, y) != 0 {
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
            b"g_date_valid_dmy (day, m, y)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_julian(FALSE as guint as guint);
    (*d).set_month(m as guint as guint);
    (*d).set_day(day as guint as guint);
    (*d).set_year(y as guint as guint);
    (*d).set_dmy(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_set_julian(mut d: *mut GDate, mut j: guint32) {
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !d.is_null() {
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
            b"d != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_julian(j) != 0 {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_julian (j)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_julian_days(j as guint as guint);
    (*d).set_julian(TRUE as guint as guint);
    (*d).set_dmy(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_is_first_of_month(mut d: *const GDate) -> gboolean {
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*d).day() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_is_last_of_month(mut d: *const GDate) -> gboolean {
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if (*d).dmy() != 0 {
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
            b"d->dmy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    idx = (if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    if (*d).day() as ::core::ffi::c_int
        == safe_c2rust_days_in_months[idx as usize][(*d).month() as usize] as ::core::ffi::c_int
    {
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_add_days(mut d: *mut GDate, mut ndays: guint) {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).julian() == 0 {
        safe_c2rust_g_date_update_julian(d);
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if ndays <= (0xffffffff as ::core::ffi::c_uint).wrapping_sub((*d).julian_days()) {
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
            b"ndays <= G_MAXUINT32 - d->julian_days\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_julian_days((*d).julian_days() + ndays as guint);
    (*d).set_dmy(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_subtract_days(mut d: *mut GDate, mut ndays: guint) {
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).julian() == 0 {
        safe_c2rust_g_date_update_julian(d);
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if (*d).julian() != 0 {
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
            b"d->julian\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if (*d).julian_days() > ndays {
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
            b"d->julian_days > ndays\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_julian_days((*d).julian_days() - ndays as guint);
    (*d).set_dmy(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_add_months(mut d: *mut GDate, mut nmonths: guint) {
    let mut years: guint = 0;
    let mut months: guint = 0;
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
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
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if nmonths
            <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(
                    ((*d).month() as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                )
        {
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
            b"nmonths <= G_MAXUINT - (d->month - 1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    nmonths = nmonths
        .wrapping_add(((*d).month() as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as guint);
    years = nmonths.wrapping_div(12 as guint);
    months = nmonths.wrapping_rem(12 as guint);
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if years
            <= (0xffff as ::core::ffi::c_int as guint16 as ::core::ffi::c_int
                - (*d).year() as ::core::ffi::c_int) as guint
        {
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
            b"years <= (guint) (G_MAXUINT16 - d->year)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_month(months.wrapping_add(1 as guint) as guint);
    (*d).set_year((*d).year() + years as guint);
    idx = (if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    if (*d).day() as ::core::ffi::c_int
        > safe_c2rust_days_in_months[idx as usize][(*d).month() as usize] as ::core::ffi::c_int
    {
        (*d).set_day(
            safe_c2rust_days_in_months[idx as usize][(*d).month() as usize] as guint as guint,
        );
    }
    (*d).set_julian(FALSE as guint as guint);
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_subtract_months(mut d: *mut GDate, mut nmonths: guint) {
    let mut years: guint = 0;
    let mut months: guint = 0;
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
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
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
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
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    years = nmonths.wrapping_div(12 as guint);
    months = nmonths.wrapping_rem(12 as guint);
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if (*d).year() > years {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"d->year > years\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_year((*d).year() - years as guint);
    if (*d).month() > months {
        (*d).set_month((*d).month() - months as guint);
    } else {
        months = months.wrapping_sub((*d).month());
        (*d).set_month((12 as guint).wrapping_sub(months) as guint);
        (*d).set_year((*d).year() - 1 as ::core::ffi::c_int as guint);
    }
    idx = (if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    if (*d).day() as ::core::ffi::c_int
        > safe_c2rust_days_in_months[idx as usize][(*d).month() as usize] as ::core::ffi::c_int
    {
        (*d).set_day(
            safe_c2rust_days_in_months[idx as usize][(*d).month() as usize] as guint as guint,
        );
    }
    (*d).set_julian(FALSE as guint as guint);
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_add_years(mut d: *mut GDate, mut nyears: guint) {
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if nyears
            <= (0xffff as ::core::ffi::c_int as guint16 as ::core::ffi::c_int
                - (*d).year() as ::core::ffi::c_int) as guint
        {
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"nyears <= (guint) (G_MAXUINT16 - d->year)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_year((*d).year() + nyears as guint);
    if (*d).month() as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        && (*d).day() as ::core::ffi::c_int == 29 as ::core::ffi::c_int
    {
        if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) == 0 {
            (*d).set_day(28 as guint as guint);
        }
    }
    (*d).set_julian(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_subtract_years(mut d: *mut GDate, mut nyears: guint) {
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if (*d).year() > nyears {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"d->year > nyears\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*d).set_year((*d).year() - nyears as guint);
    if (*d).month() as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        && (*d).day() as ::core::ffi::c_int == 29 as ::core::ffi::c_int
    {
        if safe_c2rust_g_date_is_leap_year((*d).year() as GDateYear) == 0 {
            (*d).set_day(28 as guint as guint);
        }
    }
    (*d).set_julian(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_is_leap_year(mut year: GDateYear) -> gboolean {
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_year(year) != 0 {
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_year (year)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (year as ::core::ffi::c_int % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && year as ::core::ffi::c_int % 100 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        || year as ::core::ffi::c_int % 400 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_days_in_month(
    mut month: GDateMonth,
    mut year: GDateYear,
) -> guint8 {
    let mut idx: gint = 0;
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_year(year) != 0 {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_year (year)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint8;
    }
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_month(month) != 0 {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_month (month)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint8;
    }
    idx = (if safe_c2rust_g_date_is_leap_year(year) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    return safe_c2rust_days_in_months[idx as usize][month as usize];
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_monday_weeks_in_year(
    mut year: GDateYear,
) -> guint8 {
    let mut d: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_year(year) != 0 {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_year (year)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint8;
    }
    safe_c2rust_g_date_clear(&raw mut d, 1 as guint);
    safe_c2rust_g_date_set_dmy(&raw mut d, 1 as GDateDay, G_DATE_JANUARY, year);
    if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
        == G_DATE_MONDAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 53 as guint8;
    }
    safe_c2rust_g_date_set_dmy(&raw mut d, 31 as GDateDay, G_DATE_DECEMBER, year);
    if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
        == G_DATE_MONDAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 53 as guint8;
    }
    if safe_c2rust_g_date_is_leap_year(year) != 0 {
        safe_c2rust_g_date_set_dmy(&raw mut d, 2 as GDateDay, G_DATE_JANUARY, year);
        if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
            == G_DATE_MONDAY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 53 as guint8;
        }
        safe_c2rust_g_date_set_dmy(&raw mut d, 30 as GDateDay, G_DATE_DECEMBER, year);
        if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
            == G_DATE_MONDAY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 53 as guint8;
        }
    }
    return 52 as guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_get_sunday_weeks_in_year(
    mut year: GDateYear,
) -> guint8 {
    let mut d: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid_year(year) != 0 {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid_year (year)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint8;
    }
    safe_c2rust_g_date_clear(&raw mut d, 1 as guint);
    safe_c2rust_g_date_set_dmy(&raw mut d, 1 as GDateDay, G_DATE_JANUARY, year);
    if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
        == G_DATE_SUNDAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 53 as guint8;
    }
    safe_c2rust_g_date_set_dmy(&raw mut d, 31 as GDateDay, G_DATE_DECEMBER, year);
    if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
        == G_DATE_SUNDAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 53 as guint8;
    }
    if safe_c2rust_g_date_is_leap_year(year) != 0 {
        safe_c2rust_g_date_set_dmy(&raw mut d, 2 as GDateDay, G_DATE_JANUARY, year);
        if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
            == G_DATE_SUNDAY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 53 as guint8;
        }
        safe_c2rust_g_date_set_dmy(&raw mut d, 30 as GDateDay, G_DATE_DECEMBER, year);
        if safe_c2rust_g_date_get_weekday(&raw mut d) as ::core::ffi::c_uint
            == G_DATE_SUNDAY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 53 as guint8;
        }
    }
    return 52 as guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_compare(
    mut lhs: *const GDate,
    mut rhs: *const GDate,
) -> gint {
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if !lhs.is_null() {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"lhs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if !rhs.is_null() {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rhs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(lhs) != 0 {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (lhs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(rhs) != 0 {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (rhs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    while FALSE == 0 {
        if (*lhs).julian() as ::core::ffi::c_int != 0 && (*rhs).julian() as ::core::ffi::c_int != 0
        {
            if (*lhs).julian_days() < (*rhs).julian_days() {
                return -(1 as gint);
            } else if (*lhs).julian_days() > (*rhs).julian_days() {
                return 1 as gint;
            } else {
                return 0 as gint;
            }
        } else if (*lhs).dmy() as ::core::ffi::c_int != 0 && (*rhs).dmy() as ::core::ffi::c_int != 0
        {
            if ((*lhs).year() as ::core::ffi::c_int) < (*rhs).year() as ::core::ffi::c_int {
                return -(1 as gint);
            } else if (*lhs).year() as ::core::ffi::c_int > (*rhs).year() as ::core::ffi::c_int {
                return 1 as gint;
            } else if ((*lhs).month() as ::core::ffi::c_int) < (*rhs).month() as ::core::ffi::c_int
            {
                return -(1 as gint);
            } else if (*lhs).month() as ::core::ffi::c_int > (*rhs).month() as ::core::ffi::c_int {
                return 1 as gint;
            } else if ((*lhs).day() as ::core::ffi::c_int) < (*rhs).day() as ::core::ffi::c_int {
                return -(1 as gint);
            } else if (*lhs).day() as ::core::ffi::c_int > (*rhs).day() as ::core::ffi::c_int {
                return 1 as gint;
            } else {
                return 0 as gint;
            }
        } else {
            if (*lhs).julian() == 0 {
                safe_c2rust_g_date_update_julian(lhs);
            }
            if (*rhs).julian() == 0 {
                safe_c2rust_g_date_update_julian(rhs);
            }
            if ({
                let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
                if (*lhs).julian() != 0 {
                    _g_boolean_var_97 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_97 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_97
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_return_if_fail_warning(
                    G_LOG_DOMAIN.as_ptr(),
                    G_STRFUNC,
                    b"lhs->julian\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as gint;
            }
            if ({
                let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
                if (*rhs).julian() != 0 {
                    _g_boolean_var_98 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_98 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_98
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_return_if_fail_warning(
                    G_LOG_DOMAIN.as_ptr(),
                    G_STRFUNC,
                    b"rhs->julian\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as gint;
            }
        }
    }
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_to_struct_tm(mut d: *const GDate, mut tm: *mut tm) {
    let mut day: GDateWeekday = G_DATE_BAD_WEEKDAY;
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if !tm.is_null() {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"tm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*d).dmy() == 0 {
        safe_c2rust_g_date_update_dmy(d);
    }
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if (*d).dmy() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"d->dmy != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        tm as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>() as size_t,
    );
    (*tm).tm_mday = (*d).day() as ::core::ffi::c_int;
    (*tm).tm_mon = (*d).month() as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    (*tm).tm_year = (*d).year() as ::core::ffi::c_int - 1900 as ::core::ffi::c_int;
    day = safe_c2rust_g_date_get_weekday(d);
    if day as ::core::ffi::c_uint == 7 as ::core::ffi::c_uint {
        day = G_DATE_BAD_WEEKDAY;
    }
    (*tm).tm_wday = day as ::core::ffi::c_int;
    (*tm).tm_yday =
        safe_c2rust_g_date_get_day_of_year(d).wrapping_sub(1 as guint) as ::core::ffi::c_int;
    (*tm).tm_isdst = -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_clamp(
    mut date: *mut GDate,
    mut min_date: *const GDate,
    mut max_date: *const GDate,
) {
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(date) != 0 {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (date)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !min_date.is_null() {
        if ({
            let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
            if safe_c2rust_g_date_valid(min_date) != 0 {
                _g_boolean_var_103 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_103 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_103
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"g_date_valid (min_date)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !max_date.is_null() {
        if ({
            let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
            if safe_c2rust_g_date_valid(max_date) != 0 {
                _g_boolean_var_104 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_104 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_104
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"g_date_valid (max_date)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !min_date.is_null() && !max_date.is_null() {
        if ({
            let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
            if safe_c2rust_g_date_compare(min_date, max_date) <= 0 as ::core::ffi::c_int {
                _g_boolean_var_105 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_105 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_105
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"g_date_compare (min_date, max_date) <= 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !min_date.is_null() && safe_c2rust_g_date_compare(date, min_date) < 0 as ::core::ffi::c_int {
        *date = *min_date;
    }
    if !max_date.is_null() && safe_c2rust_g_date_compare(max_date, date) < 0 as ::core::ffi::c_int {
        *date = *max_date;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_order(mut date1: *mut GDate, mut date2: *mut GDate) {
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(date1) != 0 {
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (date1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(date2) != 0 {
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (date2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_date_compare(date1, date2) > 0 as ::core::ffi::c_int {
        let mut tmp: GDate = *date1;
        *date1 = *date2;
        *date2 = tmp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_date_strftime(
    mut s: *mut gchar,
    mut slen: gsize,
    mut format: *const gchar,
    mut d: *const GDate,
) -> gsize {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut locale_format_len: gsize = 0 as gsize;
    let mut locale_format: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmplen: gsize = 0;
    let mut tmpbuf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmpbufsize: gsize = 0;
    let mut convlen: gsize = 0 as gsize;
    let mut convbuf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut retval: gsize = 0;
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if safe_c2rust_g_date_valid(d) != 0 {
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_date_valid (d)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if slen > 0 as gsize {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"slen > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if !s.is_null() {
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    safe_c2rust_g_date_to_struct_tm(d, &raw mut tm);
    locale_format = g_locale_from_utf8(
        format,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut locale_format_len,
        &raw mut error,
    );
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"../original/glib/gdate.c:2688Error converting format to locale encoding: %s\0"
                as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
        return 0 as gsize;
    }
    tmpbufsize = if 128 as gsize > locale_format_len.wrapping_mul(2 as gsize) {
        128 as gsize
    } else {
        locale_format_len.wrapping_mul(2 as gsize)
    };
    while FALSE == 0 {
        tmpbuf = g_malloc(tmpbufsize) as *mut gchar;
        *tmpbuf.offset(0 as ::core::ffi::c_int as isize) = '\u{1}' as i32 as gchar;
        tmplen = strftime(
            tmpbuf as *mut ::core::ffi::c_char,
            tmpbufsize as size_t,
            locale_format,
            &raw mut tm,
        ) as gsize;
        if !(tmplen == 0 as gsize
            && *tmpbuf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32)
        {
            break;
        }
        g_free(tmpbuf as gpointer);
        tmpbufsize = tmpbufsize.wrapping_mul(2 as gsize);
        if tmpbufsize > 65536 as gsize {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/gdate.c:2713Maximum buffer size for g_date_strftime exceeded: giving up\0"
                    as *const u8 as *const gchar,
            );
            g_free(locale_format as gpointer);
            *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
            return 0 as gsize;
        }
    }
    g_free(locale_format as gpointer);
    convbuf = g_locale_to_utf8(
        tmpbuf,
        tmplen as gssize,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut convlen,
        &raw mut error,
    );
    g_free(tmpbuf as gpointer);
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"../original/glib/gdate.c:2730Error converting results of strftime to UTF-8: %s\0"
                as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        if ({
            let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
            if convbuf.is_null() {
                _g_boolean_var_112 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_112 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_112
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gdate.c\0" as *const u8 as *const ::core::ffi::c_char,
                2733 as ::core::ffi::c_int,
                G_STRFUNC,
                b"convbuf == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
        return 0 as gsize;
    }
    if slen <= convlen {
        let mut end: *mut gchar = g_utf8_find_prev_char(convbuf, convbuf.offset(slen as isize));
        if ({
            let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
            if !end.is_null() {
                _g_boolean_var_113 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_113 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_113
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gdate.c\0" as *const u8 as *const ::core::ffi::c_char,
                2744 as ::core::ffi::c_int,
                G_STRFUNC,
                b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        convlen = end.offset_from(convbuf) as ::core::ffi::c_long as gsize;
        retval = 0 as gsize;
    } else {
        retval = convlen;
    }
    memcpy(
        s as *mut ::core::ffi::c_void,
        convbuf as *const ::core::ffi::c_void,
        convlen as size_t,
    );
    *s.offset(convlen as isize) = '\0' as i32 as gchar;
    g_free(convbuf as gpointer);
    return retval;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_date_new_dmy\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
