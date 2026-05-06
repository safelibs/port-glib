use ::c2rust_bitfields;
extern "C" {
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn timegm(__tp: *mut tm) -> time_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_get_monotonic_time() -> gint64;
}
pub type guint16 = ::core::ffi::c_ushort;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GTimer {
    pub start: guint64,
    pub end: guint64,
    #[bitfield(name = "active", ty = "guint", bits = "0..=0")]
    pub active: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GTimer = _GTimer;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_new() -> *mut GTimer {
    let mut timer: *mut GTimer = ::core::ptr::null_mut::<GTimer>();
    timer = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GTimer>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GTimer;
    (*timer).set_active(TRUE as guint as guint);
    (*timer).start = g_get_monotonic_time() as guint64;
    return timer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_destroy(mut timer: *mut GTimer) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free(timer as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_start(mut timer: *mut GTimer) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*timer).set_active(TRUE as guint as guint);
    (*timer).start = g_get_monotonic_time() as guint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_stop(mut timer: *mut GTimer) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*timer).set_active(FALSE as guint as guint);
    (*timer).end = g_get_monotonic_time() as guint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_reset(mut timer: *mut GTimer) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*timer).start = g_get_monotonic_time() as guint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_continue(mut timer: *mut GTimer) {
    let mut elapsed: guint64 = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*timer).active() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
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
            b"timer->active == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    elapsed = (*timer).end.wrapping_sub((*timer).start);
    (*timer).start = g_get_monotonic_time() as guint64;
    (*timer).start = (*timer).start.wrapping_sub(elapsed);
    (*timer).set_active(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_elapsed(
    mut timer: *mut GTimer,
    mut microseconds: *mut gulong,
) -> gdouble {
    let mut total: gdouble = 0.;
    let mut elapsed: gint64 = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    if (*timer).active() != 0 {
        (*timer).end = g_get_monotonic_time() as guint64;
    }
    elapsed = (*timer).end.wrapping_sub((*timer).start) as gint64;
    total = (elapsed as ::core::ffi::c_double / 1e6f64) as gdouble;
    if !microseconds.is_null() {
        *microseconds = (elapsed % 1000000 as gint64) as gulong;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timer_is_active(mut timer: *mut GTimer) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !timer.is_null() {
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
            b"timer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*timer).active() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_usleep(mut microseconds: gulong) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if microseconds == 0 as gulong {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        return;
    }
    let mut request: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut remaining: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    request.tv_sec = microseconds.wrapping_div(G_USEC_PER_SEC as gulong) as __time_t;
    request.tv_nsec = (1000 as gulong)
        .wrapping_mul(microseconds.wrapping_rem(G_USEC_PER_SEC as gulong))
        as __syscall_slong_t;
    while nanosleep(&raw mut request, &raw mut remaining) == -(1 as ::core::ffi::c_int)
        && *__errno_location() == EINTR
    {
        request = remaining;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_val_add(
    mut time_: *mut GTimeVal,
    mut microseconds: glong,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !time_.is_null() && (*time_).tv_usec >= 0 as glong && (*time_).tv_usec < 1000000 as glong
        {
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
            b"time_ != NULL && time_->tv_usec >= 0 && time_->tv_usec < G_USEC_PER_SEC\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if microseconds >= 0 as glong {
        (*time_).tv_usec += microseconds % G_USEC_PER_SEC as glong;
        (*time_).tv_sec += microseconds / G_USEC_PER_SEC as glong;
        if (*time_).tv_usec >= G_USEC_PER_SEC as glong {
            (*time_).tv_usec -= G_USEC_PER_SEC as glong;
            (*time_).tv_sec += 1;
        }
    } else {
        microseconds *= -(1 as ::core::ffi::c_int) as glong;
        (*time_).tv_usec -= microseconds % G_USEC_PER_SEC as glong;
        (*time_).tv_sec -= microseconds / G_USEC_PER_SEC as glong;
        if (*time_).tv_usec < 0 as glong {
            (*time_).tv_usec += G_USEC_PER_SEC as glong;
            (*time_).tv_sec -= 1;
        }
    };
}
unsafe extern "C" fn safe_c2rust_mktime_utc(mut tm: *mut tm) -> time_t {
    let mut retval: time_t = 0;
    retval = timegm(tm);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_val_from_iso8601(
    mut iso_date: *const gchar,
    mut time_: *mut GTimeVal,
) -> gboolean {
    let mut tm: tm = tm {
        tm_sec: 0 as ::core::ffi::c_int,
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
    let mut val: ::core::ffi::c_long = 0;
    let mut mday: ::core::ffi::c_long = 0;
    let mut mon: ::core::ffi::c_long = 0;
    let mut year: ::core::ffi::c_long = 0;
    let mut hour: ::core::ffi::c_long = 0;
    let mut min: ::core::ffi::c_long = 0;
    let mut sec: ::core::ffi::c_long = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !iso_date.is_null() {
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
            b"iso_date != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !time_.is_null() {
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
            b"time_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    while *safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        iso_date = iso_date.offset(1);
    }
    if *iso_date as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    if !(*safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_DIGIT as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
        && *iso_date as ::core::ffi::c_int != '+' as i32
    {
        return FALSE;
    }
    val = strtoul(
        iso_date as *const ::core::ffi::c_char,
        &raw mut iso_date as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_long;
    if *iso_date as ::core::ffi::c_int == '-' as i32 {
        year = val;
        iso_date = iso_date.offset(1);
        mon = strtoul(
            iso_date as *const ::core::ffi::c_char,
            &raw mut iso_date as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
        let fresh0 = iso_date;
        iso_date = iso_date.offset(1);
        if *fresh0 as ::core::ffi::c_int != '-' as i32 {
            return FALSE;
        }
        mday = strtoul(
            iso_date as *const ::core::ffi::c_char,
            &raw mut iso_date as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
    } else {
        mday = val % 100 as ::core::ffi::c_long;
        mon = val % 10000 as ::core::ffi::c_long / 100 as ::core::ffi::c_long;
        year = val / 10000 as ::core::ffi::c_long;
    }
    if year < 1900 as ::core::ffi::c_long || year > G_MAXINT as ::core::ffi::c_long {
        return FALSE;
    }
    if mon < 1 as ::core::ffi::c_long || mon > 12 as ::core::ffi::c_long {
        return FALSE;
    }
    if mday < 1 as ::core::ffi::c_long || mday > 31 as ::core::ffi::c_long {
        return FALSE;
    }
    tm.tm_mday = mday as ::core::ffi::c_int;
    tm.tm_mon = (mon - 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
    tm.tm_year = (year - 1900 as ::core::ffi::c_long) as ::core::ffi::c_int;
    if *iso_date as ::core::ffi::c_int != 'T' as i32 {
        return FALSE;
    }
    iso_date = iso_date.offset(1);
    if !(*safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_DIGIT as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        return FALSE;
    }
    val = strtoul(
        iso_date as *const ::core::ffi::c_char,
        &raw mut iso_date as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_long;
    if *iso_date as ::core::ffi::c_int == ':' as i32 {
        hour = val;
        iso_date = iso_date.offset(1);
        min = strtoul(
            iso_date as *const ::core::ffi::c_char,
            &raw mut iso_date as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
        let fresh1 = iso_date;
        iso_date = iso_date.offset(1);
        if *fresh1 as ::core::ffi::c_int != ':' as i32 {
            return FALSE;
        }
        sec = strtoul(
            iso_date as *const ::core::ffi::c_char,
            &raw mut iso_date as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
    } else {
        sec = val % 100 as ::core::ffi::c_long;
        min = val % 10000 as ::core::ffi::c_long / 100 as ::core::ffi::c_long;
        hour = val / 10000 as ::core::ffi::c_long;
    }
    if hour > 23 as ::core::ffi::c_long {
        return FALSE;
    }
    if min > 59 as ::core::ffi::c_long {
        return FALSE;
    }
    if sec > 61 as ::core::ffi::c_long {
        return FALSE;
    }
    tm.tm_hour = hour as ::core::ffi::c_int;
    tm.tm_min = min as ::core::ffi::c_int;
    tm.tm_sec = sec as ::core::ffi::c_int;
    (*time_).tv_usec = 0 as glong;
    if *iso_date as ::core::ffi::c_int == ',' as i32
        || *iso_date as ::core::ffi::c_int == '.' as i32
    {
        let mut mul: glong = 100000 as glong;
        while mul >= 1 as glong && {
            iso_date = iso_date.offset(1);
            *safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_DIGIT as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        } {
            (*time_).tv_usec += (*iso_date as ::core::ffi::c_int - '0' as i32) as glong * mul;
            mul /= 10 as glong;
        }
        while *safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_DIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            iso_date = iso_date.offset(1);
        }
    }
    if *iso_date as ::core::ffi::c_int == 'Z' as i32 {
        iso_date = iso_date.offset(1);
        (*time_).tv_sec = safe_c2rust_mktime_utc(&raw mut tm) as glong;
    } else if *iso_date as ::core::ffi::c_int == '+' as i32
        || *iso_date as ::core::ffi::c_int == '-' as i32
    {
        let mut sign: gint = if *iso_date as ::core::ffi::c_int == '+' as i32 {
            -(1 as gint)
        } else {
            1 as gint
        };
        val = strtoul(
            iso_date.offset(1 as ::core::ffi::c_int as isize),
            &raw mut iso_date as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
        if *iso_date as ::core::ffi::c_int == ':' as i32 {
            hour = val;
            min = strtoul(
                iso_date.offset(1 as ::core::ffi::c_int as isize),
                &raw mut iso_date as *mut *mut ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_long;
        } else {
            hour = val / 100 as ::core::ffi::c_long;
            min = val % 100 as ::core::ffi::c_long;
        }
        if hour > 99 as ::core::ffi::c_long {
            return FALSE;
        }
        if min > 59 as ::core::ffi::c_long {
            return FALSE;
        }
        (*time_).tv_sec = (safe_c2rust_mktime_utc(&raw mut tm)
            + 60 as gint64 * (60 as ::core::ffi::c_long * hour + min) * sign as gint64)
            as glong;
    } else {
        tm.tm_isdst = -(1 as ::core::ffi::c_int);
        (*time_).tv_sec = mktime(&raw mut tm) as glong;
    }
    while *safe_c2rust_g_ascii_table.offset(*iso_date as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        iso_date = iso_date.offset(1);
    }
    return (*iso_date as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_val_to_iso8601(mut time_: *mut GTimeVal) -> *mut gchar {
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tm: *mut tm = ::core::ptr::null_mut::<tm>();
    let mut tm_: tm = tm {
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
    let mut secs: time_t = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !time_.is_null() && (*time_).tv_usec >= 0 as glong && (*time_).tv_usec < 1000000 as glong
        {
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
            b"time_ != NULL && time_->tv_usec >= 0 && time_->tv_usec < G_USEC_PER_SEC\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    secs = (*time_).tv_sec as time_t;
    tm = gmtime_r(&raw mut secs, &raw mut tm_);
    if tm.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    if (*time_).tv_usec != 0 as glong {
        retval = g_strdup_printf(
            b"%4d-%02d-%02dT%02d:%02d:%02d.%06ldZ\0" as *const u8 as *const gchar,
            (*tm).tm_year + 1900 as ::core::ffi::c_int,
            (*tm).tm_mon + 1 as ::core::ffi::c_int,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
            (*time_).tv_usec,
        );
    } else {
        retval = g_strdup_printf(
            b"%4d-%02d-%02dT%02d:%02d:%02dZ\0" as *const u8 as *const gchar,
            (*tm).tm_year + 1900 as ::core::ffi::c_int,
            (*tm).tm_mon + 1 as ::core::ffi::c_int,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
        );
    }
    return retval;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_timer_destroy\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
