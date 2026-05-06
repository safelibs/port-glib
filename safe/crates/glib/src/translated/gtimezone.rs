use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    pub type _GHashTable;
    pub type _GBytes;
    pub type _GMappedFile;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_array_sized_new(
        zero_terminated: gboolean,
        clear_: gboolean,
        element_size: guint,
        reserved_size: guint,
    ) -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_mapped_file_new(
        filename: *const gchar,
        writable: gboolean,
        error: *mut *mut GError,
    ) -> *mut GMappedFile;
    fn g_mapped_file_get_length(file: *mut GMappedFile) -> gsize;
    fn g_mapped_file_get_contents(file: *mut GMappedFile) -> *mut gchar;
    fn g_mapped_file_ref(file: *mut GMappedFile) -> *mut GMappedFile;
    fn g_mapped_file_unref(file: *mut GMappedFile);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_file_error_quark() -> GQuark;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_read_link(filename: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_canonicalize_filename(filename: *const gchar, relative_to: *const gchar) -> *mut gchar;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_date_get_weekday(date: *const GDate) -> GDateWeekday;
    fn g_date_get_month(date: *const GDate) -> GDateMonth;
    fn g_date_get_day(date: *const GDate) -> GDateDay;
    fn g_date_get_julian(date: *const GDate) -> guint32;
    fn g_date_clear(date: *mut GDate, n_dates: guint);
    fn g_date_set_dmy(date: *mut GDate, day: GDateDay, month: GDateMonth, y: GDateYear);
    fn g_date_set_julian(date: *mut GDate, julian_date: guint32);
    fn g_date_add_days(date: *mut GDate, n_days: guint);
    fn g_date_is_leap_year(year: GDateYear) -> gboolean;
    fn g_date_get_days_in_month(month: GDateMonth, year: GDateYear) -> guint8;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeZone {
    pub name: *mut gchar,
    pub t_info: *mut GArray,
    pub transitions: *mut GArray,
    pub ref_count: gint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GTimeZone = _GTimeZone;
pub type GTimeType = ::core::ffi::c_uint;
pub const G_TIME_TYPE_UNIVERSAL: GTimeType = 2;
pub const G_TIME_TYPE_DAYLIGHT: GTimeType = 1;
pub const G_TIME_TYPE_STANDARD: GTimeType = 0;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GHashTable = _GHashTable;
pub type GBytes = _GBytes;
pub type GMappedFile = _GMappedFile;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionInfo {
    pub gmt_offset: gint32,
    pub is_dst: gboolean,
    pub abbrev: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Transition {
    pub time: gint64,
    pub info_index: gint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gint32_be {
    pub bytes: [gchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gint64_be {
    pub bytes: [gchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tzhead {
    pub tzh_magic: [gchar; 4],
    pub tzh_version: gchar,
    pub tzh_reserved: [guchar; 15],
    pub tzh_ttisgmtcnt: guint32_be,
    pub tzh_ttisstdcnt: guint32_be,
    pub tzh_leapcnt: guint32_be,
    pub tzh_timecnt: guint32_be,
    pub tzh_typecnt: guint32_be,
    pub tzh_charcnt: guint32_be,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct guint32_be {
    pub bytes: [gchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttinfo {
    pub tt_gmtoff: gint32_be,
    pub tt_isdst: guint8,
    pub tt_abbrind: guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimeZoneRule {
    pub start_year: guint,
    pub std_offset: gint32,
    pub dlt_offset: gint32,
    pub dlt_start: TimeZoneDate,
    pub dlt_end: TimeZoneDate,
    pub std_name: [gchar; 33],
    pub dlt_name: [gchar; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimeZoneDate {
    pub year: gint,
    pub mon: gint,
    pub mday: gint,
    pub wday: gint,
    pub week: gint,
    pub offset: gint32,
}
pub type GDateDay = guint8;
pub type GDate = _GDate;
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
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub type GDateYear = guint16;
pub type GDateWeekday = ::core::ffi::c_uint;
pub const G_DATE_SUNDAY: GDateWeekday = 7;
pub const G_DATE_SATURDAY: GDateWeekday = 6;
pub const G_DATE_FRIDAY: GDateWeekday = 5;
pub const G_DATE_THURSDAY: GDateWeekday = 4;
pub const G_DATE_WEDNESDAY: GDateWeekday = 3;
pub const G_DATE_TUESDAY: GDateWeekday = 2;
pub const G_DATE_MONDAY: GDateWeekday = 1;
pub const G_DATE_BAD_WEEKDAY: GDateWeekday = 0;
pub const G_FILE_ERROR_INVAL: C2RustUnnamed_0 = 17;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: C2RustUnnamed_0 = 24;
pub const G_FILE_ERROR_NOSYS: C2RustUnnamed_0 = 23;
pub const G_FILE_ERROR_PERM: C2RustUnnamed_0 = 22;
pub const G_FILE_ERROR_IO: C2RustUnnamed_0 = 21;
pub const G_FILE_ERROR_INTR: C2RustUnnamed_0 = 20;
pub const G_FILE_ERROR_AGAIN: C2RustUnnamed_0 = 19;
pub const G_FILE_ERROR_PIPE: C2RustUnnamed_0 = 18;
pub const G_FILE_ERROR_BADF: C2RustUnnamed_0 = 16;
pub const G_FILE_ERROR_NFILE: C2RustUnnamed_0 = 15;
pub const G_FILE_ERROR_MFILE: C2RustUnnamed_0 = 14;
pub const G_FILE_ERROR_NOMEM: C2RustUnnamed_0 = 13;
pub const G_FILE_ERROR_NOSPC: C2RustUnnamed_0 = 12;
pub const G_FILE_ERROR_LOOP: C2RustUnnamed_0 = 11;
pub const G_FILE_ERROR_FAULT: C2RustUnnamed_0 = 10;
pub const G_FILE_ERROR_TXTBSY: C2RustUnnamed_0 = 9;
pub const G_FILE_ERROR_ROFS: C2RustUnnamed_0 = 8;
pub const G_FILE_ERROR_NODEV: C2RustUnnamed_0 = 7;
pub const G_FILE_ERROR_NXIO: C2RustUnnamed_0 = 6;
pub const G_FILE_ERROR_NOTDIR: C2RustUnnamed_0 = 5;
pub const G_FILE_ERROR_NOENT: C2RustUnnamed_0 = 4;
pub const G_FILE_ERROR_NAMETOOLONG: C2RustUnnamed_0 = 3;
pub const G_FILE_ERROR_ACCES: C2RustUnnamed_0 = 2;
pub const G_FILE_ERROR_ISDIR: C2RustUnnamed_0 = 1;
pub const G_FILE_ERROR_EXIST: C2RustUnnamed_0 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MININT64: gint64 = -G_MAXINT64 - 1 as ::core::ffi::c_long;
pub const G_MAXINT64: ::core::ffi::c_long = 0x7fffffffffffffff as ::core::ffi::c_long;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
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
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
#[inline]
unsafe extern "C" fn safe_c2rust_gint64_from_be(be: gint64_be) -> gint64 {
    let mut tmp: gint64 = 0;
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_void,
        &raw const be as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gint64>() as size_t,
    );
    return ({
        let mut __v: guint64 = 0;
        let mut __x: guint64 = tmp as guint64;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                | (__x & 0xff0000000000 as ::core::ffi::c_ulong) >> 24 as ::core::ffi::c_int
                | (__x & 0xff000000000000 as ::core::ffi::c_ulong) >> 40 as ::core::ffi::c_int
                | (__x & 0xff00000000000000 as ::core::ffi::c_ulong) >> 56 as ::core::ffi::c_int;
        } else {
            let fresh4 = &mut __v;
            let fresh5;
            let fresh6 = __x;
            asm!(
                "bswapq {0}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh4,
                fresh6) => fresh5, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        }
        __v
    }) as gint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gint32_from_be(be: gint32_be) -> gint32 {
    let mut tmp: gint32 = 0;
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_void,
        &raw const be as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gint32>() as size_t,
    );
    return ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = tmp as guint32;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh1 = &mut __v;
            let fresh2;
            let fresh3 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh1,
                fresh3) => fresh2, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        }
        __v
    }) as gint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust_guint32_from_be(be: guint32_be) -> guint32 {
    let mut tmp: guint32 = 0;
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_void,
        &raw const be as *const ::core::ffi::c_void,
        ::core::mem::size_of::<guint32>() as size_t,
    );
    return ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = tmp;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh7 = &mut __v;
            let fresh8;
            let fresh9 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh7,
                fresh9) => fresh8, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
        }
        __v
    });
}
pub const NAME_SIZE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
static mut safe_c2rust_g__time_zones_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_time_zones: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_g__tz_default_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_tz_default: *mut GTimeZone =
    ::core::ptr::null::<GTimeZone>() as *mut GTimeZone;
static mut safe_c2rust_g__tz_local_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_tz_local: *mut GTimeZone =
    ::core::ptr::null::<GTimeZone>() as *mut GTimeZone;
pub const MIN_TZYEAR: ::core::ffi::c_int = 1916 as ::core::ffi::c_int;
pub const MAX_TZYEAR: ::core::ffi::c_int = 2999 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_unref(mut tz: *mut GTimeZone) {
    let mut current_block: u64;
    let mut ref_count: ::core::ffi::c_int = 0;
    loop {
        ref_count = ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*tz).ref_count;
                (*tz).ref_count;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*tz).ref_count);
            gaig_temp
        }) as ::core::ffi::c_int;
        if ({
            let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
            if ref_count > 0 as ::core::ffi::c_int {
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
                b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                227 as ::core::ffi::c_int,
                G_STRFUNC,
                b"ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ref_count == 1 as ::core::ffi::c_int {
            if (*tz).name.is_null() {
                current_block = 1841672684692190573;
                break;
            }
            g_mutex_lock(&raw mut safe_c2rust_g__time_zones_lock);
            if ({
                let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
                if ({
                    let mut gaig_temp: gint = 0;
                    if 0 as ::core::ffi::c_int != 0 {
                        (*tz).ref_count;
                        (*tz).ref_count;
                    } else {
                    };
                    *&raw mut gaig_temp =
                        crate::translated::compat::atomic_load_seqcst(&raw mut (*tz).ref_count);
                    gaig_temp
                }) != 1 as ::core::ffi::c_int
                {
                    _g_boolean_var_9 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_9 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_9
            }) as ::core::ffi::c_long
                != 0
            {
                g_mutex_unlock(&raw mut safe_c2rust_g__time_zones_lock);
            } else {
                if !safe_c2rust_time_zones.is_null() {
                    g_hash_table_remove(safe_c2rust_time_zones, (*tz).name as gconstpointer);
                }
                g_mutex_unlock(&raw mut safe_c2rust_g__time_zones_lock);
                current_block = 1841672684692190573;
                break;
            }
        } else if !(({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if ({
                let mut gaicae_oldval: gint = ref_count as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*tz).ref_count;
                } else {
                };
                let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*tz).ref_count,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    ref_count - 1 as ::core::ffi::c_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh0.0;
                if fresh0.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {
                _g_boolean_var_10 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_10 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_10
        }) as ::core::ffi::c_long
            != 0)
        {
            current_block = 15925075030174552612;
            break;
        }
    }
    match current_block {
        1841672684692190573 => {
            if !(*tz).t_info.is_null() {
                let mut idx: guint = 0;
                idx = 0 as guint;
                while idx < (*(*tz).t_info).len {
                    let mut info: *mut TransitionInfo =
                        ((*(*tz).t_info).data as *mut ::core::ffi::c_void as *mut TransitionInfo)
                            .offset(idx as isize) as *mut TransitionInfo;
                    g_free((*info).abbrev as gpointer);
                    idx = idx.wrapping_add(1);
                }
                g_array_free((*tz).t_info, TRUE);
            }
            if !(*tz).transitions.is_null() {
                g_array_free((*tz).transitions, TRUE);
            }
            g_free((*tz).name as gpointer);
            g_slice_free1(::core::mem::size_of::<GTimeZone>() as gsize, tz as gpointer);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_ref(mut tz: *mut GTimeZone) -> *mut GTimeZone {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*tz).ref_count > 0 as ::core::ffi::c_int {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tz->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*tz).ref_count;
        (*tz).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*tz).ref_count, 1 as ::core::ffi::c_int);
    return tz;
}
unsafe extern "C" fn safe_c2rust_parse_time(
    mut time_: *const gchar,
    mut offset: *mut gint32,
    mut rfc8536: gboolean,
) -> gboolean {
    if (*time_ as ::core::ffi::c_int) < '0' as i32 || ('9' as i32) < *time_ as ::core::ffi::c_int {
        return FALSE;
    }
    let fresh13 = time_;
    time_ = time_.offset(1);
    *offset = (60 as ::core::ffi::c_int
        * 60 as ::core::ffi::c_int
        * (*fresh13 as ::core::ffi::c_int - '0' as i32)) as gint32;
    if *time_ as ::core::ffi::c_int == '\0' as i32 {
        return TRUE;
    }
    if *time_ as ::core::ffi::c_int != ':' as i32 {
        if (*time_ as ::core::ffi::c_int) < '0' as i32
            || ('9' as i32) < *time_ as ::core::ffi::c_int
        {
            return FALSE;
        }
        *offset *= 10 as ::core::ffi::c_int;
        let fresh14 = time_;
        time_ = time_.offset(1);
        *offset += 60 as ::core::ffi::c_int
            * 60 as ::core::ffi::c_int
            * (*fresh14 as ::core::ffi::c_int - '0' as i32);
        if rfc8536 != 0 {
            if '0' as i32 <= *time_ as ::core::ffi::c_int
                && *time_ as ::core::ffi::c_int <= '9' as i32
            {
                *offset *= 10 as ::core::ffi::c_int;
                let fresh15 = time_;
                time_ = time_.offset(1);
                *offset += 60 as ::core::ffi::c_int
                    * 60 as ::core::ffi::c_int
                    * (*fresh15 as ::core::ffi::c_int - '0' as i32);
            }
            if *offset
                > 167 as ::core::ffi::c_int * 60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
            {
                return FALSE;
            }
        } else if *offset
            > 24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
        {
            return FALSE;
        }
        if *time_ as ::core::ffi::c_int == '\0' as i32 {
            return TRUE;
        }
    }
    if *time_ as ::core::ffi::c_int == ':' as i32 {
        time_ = time_.offset(1);
    } else if rfc8536 != 0 {
        return FALSE;
    }
    if (*time_ as ::core::ffi::c_int) < '0' as i32 || ('5' as i32) < *time_ as ::core::ffi::c_int {
        return FALSE;
    }
    let fresh16 = time_;
    time_ = time_.offset(1);
    *offset += 10 as ::core::ffi::c_int
        * 60 as ::core::ffi::c_int
        * (*fresh16 as ::core::ffi::c_int - '0' as i32);
    if (*time_ as ::core::ffi::c_int) < '0' as i32 || ('9' as i32) < *time_ as ::core::ffi::c_int {
        return FALSE;
    }
    let fresh17 = time_;
    time_ = time_.offset(1);
    *offset += 60 as ::core::ffi::c_int * (*fresh17 as ::core::ffi::c_int - '0' as i32);
    if *time_ as ::core::ffi::c_int == '\0' as i32 {
        return TRUE;
    }
    if *time_ as ::core::ffi::c_int == ':' as i32 {
        time_ = time_.offset(1);
    } else if rfc8536 != 0 {
        return FALSE;
    }
    if (*time_ as ::core::ffi::c_int) < '0' as i32 || ('5' as i32) < *time_ as ::core::ffi::c_int {
        return FALSE;
    }
    let fresh18 = time_;
    time_ = time_.offset(1);
    *offset += 10 as ::core::ffi::c_int * (*fresh18 as ::core::ffi::c_int - '0' as i32);
    if (*time_ as ::core::ffi::c_int) < '0' as i32 || ('9' as i32) < *time_ as ::core::ffi::c_int {
        return FALSE;
    }
    let fresh19 = time_;
    time_ = time_.offset(1);
    *offset += *fresh19 as ::core::ffi::c_int - '0' as i32;
    return (*time_ as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_parse_constant_offset(
    mut name: *const gchar,
    mut offset: *mut gint32,
    mut rfc8536: gboolean,
) -> gboolean {
    if rfc8536 == 0
        && g_strcmp0(
            name as *const ::core::ffi::c_char,
            b"UTC\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *offset = 0 as ::core::ffi::c_int as gint32;
        return TRUE;
    }
    if *name as ::core::ffi::c_int >= '0' as i32 && '9' as i32 >= *name as ::core::ffi::c_int {
        return safe_c2rust_parse_time(name, offset, rfc8536);
    }
    let fresh12 = name;
    name = name.offset(1);
    match *fresh12 as ::core::ffi::c_int {
        90 => {
            *offset = 0 as ::core::ffi::c_int as gint32;
            return (rfc8536 == 0 && *name == 0) as ::core::ffi::c_int;
        }
        43 => return safe_c2rust_parse_time(name, offset, rfc8536),
        45 => {
            if safe_c2rust_parse_time(name, offset, rfc8536) != 0 {
                *offset = -*offset;
                return TRUE;
            } else {
                return FALSE;
            }
        }
        _ => return FALSE,
    };
}
unsafe extern "C" fn safe_c2rust_zone_for_constant_offset(
    mut gtz: *mut GTimeZone,
    mut name: *const gchar,
) {
    let mut offset: gint32 = 0;
    let mut info: TransitionInfo = TransitionInfo {
        gmt_offset: 0,
        is_dst: 0,
        abbrev: ::core::ptr::null_mut::<gchar>(),
    };
    if name.is_null() || safe_c2rust_parse_constant_offset(name, &raw mut offset, FALSE) == 0 {
        return;
    }
    info.gmt_offset = offset;
    info.is_dst = FALSE as gboolean;
    info.abbrev = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*gtz).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*gtz).t_info = g_array_sized_new(
        FALSE,
        TRUE,
        ::core::mem::size_of::<TransitionInfo>() as guint,
        1 as guint,
    );
    g_array_append_vals((*gtz).t_info, &raw mut info as gconstpointer, 1 as guint);
    (*gtz).transitions = ::core::ptr::null_mut::<GArray>();
}
unsafe extern "C" fn safe_c2rust_zone_info_base_dir() -> *const gchar {
    if g_file_test(
        b"/usr/share/zoneinfo\0" as *const u8 as *const gchar,
        G_FILE_TEST_IS_DIR,
    ) != 0
    {
        return b"/usr/share/zoneinfo\0" as *const u8 as *const gchar;
    } else if g_file_test(
        b"/usr/share/lib/zoneinfo\0" as *const u8 as *const gchar,
        G_FILE_TEST_IS_DIR,
    ) != 0
    {
        return b"/usr/share/lib/zoneinfo\0" as *const u8 as *const gchar;
    }
    return b"/usr/share/zoneinfo\0" as *const u8 as *const gchar;
}
unsafe extern "C" fn safe_c2rust_zone_identifier_unix() -> *mut gchar {
    let mut current_block: u64;
    let mut resolved_identifier: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut prefix_len: gsize = 0 as gsize;
    let mut canonical_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut read_link_err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut tzdir: *const gchar = ::core::ptr::null::<gchar>();
    let mut not_a_symlink_to_zoneinfo: gboolean = FALSE;
    let mut file_status: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    resolved_identifier = g_file_read_link(
        b"/etc/localtime\0" as *const u8 as *const gchar,
        &raw mut read_link_err,
    );
    if !resolved_identifier.is_null() {
        if g_path_is_absolute(resolved_identifier) == 0 {
            let mut absolute_resolved_identifier: *mut gchar = g_build_filename(
                b"/etc\0" as *const u8 as *const gchar,
                resolved_identifier,
                NULL,
            );
            g_free(resolved_identifier as gpointer);
            resolved_identifier =
                safe_c2rust_g_steal_pointer(&raw mut absolute_resolved_identifier as gpointer)
                    as *mut gchar as *mut gchar;
        }
        if lstat(resolved_identifier, &raw mut file_status) == 0 as ::core::ffi::c_int {
            if file_status.st_mode & S_IFMT as __mode_t != S_IFREG as __mode_t {
                let mut _pp: *mut *mut gchar = &raw mut resolved_identifier;
                let mut _ptr: *mut gchar = *_pp;
                *_pp = ::core::ptr::null_mut::<gchar>();
                if !_ptr.is_null() {
                    g_free(_ptr as gpointer);
                }
                not_a_symlink_to_zoneinfo = TRUE as gboolean;
            }
        } else {
            let mut _pp_0: *mut *mut gchar = &raw mut resolved_identifier;
            let mut _ptr_0: *mut gchar = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<gchar>();
            if !_ptr_0.is_null() {
                g_free(_ptr_0 as gpointer);
            }
        }
    } else {
        not_a_symlink_to_zoneinfo = g_error_matches(
            read_link_err,
            g_file_error_quark(),
            G_FILE_ERROR_INVAL as ::core::ffi::c_int as gint,
        );
        g_clear_error(&raw mut read_link_err);
    }
    if resolved_identifier.is_null() {
        if not_a_symlink_to_zoneinfo != 0
            && (g_file_get_contents(
                b"/var/db/zoneinfo\0" as *const u8 as *const gchar,
                &raw mut resolved_identifier,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) != 0
                || g_file_get_contents(
                    b"/etc/timezone\0" as *const u8 as *const gchar,
                    &raw mut resolved_identifier,
                    ::core::ptr::null_mut::<gsize>(),
                    ::core::ptr::null_mut::<*mut GError>(),
                ) != 0)
        {
            g_strchomp(resolved_identifier);
            current_block = 6417057564578538666;
        } else {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if resolved_identifier.is_null() {
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
                    b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                    599 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"resolved_identifier == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            current_block = 934197306813476288;
        }
    } else {
        canonical_path =
            g_canonicalize_filename(resolved_identifier, b"/etc\0" as *const u8 as *const gchar);
        g_free(resolved_identifier as gpointer);
        resolved_identifier = safe_c2rust_g_steal_pointer(&raw mut canonical_path as gpointer)
            as *mut gchar as *mut gchar;
        current_block = 6417057564578538666;
    }
    match current_block {
        6417057564578538666 => {
            tzdir = g_getenv(b"TZDIR\0" as *const u8 as *const gchar);
            if tzdir.is_null() {
                tzdir = safe_c2rust_zone_info_base_dir();
            }
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = resolved_identifier;
                    let __prefix: *const ::core::ffi::c_char = tzdir as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_13
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                g_str_has_prefix(resolved_identifier, tzdir)
            } != 0
            {
                prefix_len = strlen(tzdir as *const ::core::ffi::c_char) as gsize;
                while *resolved_identifier.offset(prefix_len as isize) as ::core::ffi::c_int
                    == '/' as i32
                {
                    prefix_len = prefix_len.wrapping_add(1);
                }
            }
            if prefix_len > 0 as gsize {
                memmove(
                    resolved_identifier as *mut ::core::ffi::c_void,
                    resolved_identifier.offset(prefix_len as isize) as *const ::core::ffi::c_void,
                    strlen(resolved_identifier)
                        .wrapping_sub(prefix_len as size_t)
                        .wrapping_add(1 as size_t),
                );
            }
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if !resolved_identifier.is_null() {
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
                    b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                    627 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"resolved_identifier != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    g_free(canonical_path as gpointer);
    return resolved_identifier;
}
unsafe extern "C" fn safe_c2rust_zone_info_unix(
    mut identifier: *const gchar,
    mut resolved_identifier: *const gchar,
) -> *mut GBytes {
    let mut current_block: u64;
    let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut file: *mut GMappedFile = ::core::ptr::null_mut::<GMappedFile>();
    let mut zoneinfo: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut tzdir: *const gchar = ::core::ptr::null::<gchar>();
    tzdir = g_getenv(b"TZDIR\0" as *const u8 as *const gchar);
    if tzdir.is_null() {
        tzdir = safe_c2rust_zone_info_base_dir();
    }
    if !identifier.is_null() {
        if *identifier as ::core::ffi::c_int == ':' as i32 {
            identifier = identifier.offset(1);
        }
        if g_path_is_absolute(identifier) != 0 {
            filename =
                safe_c2rust_g_strdup_inline(identifier as *const ::core::ffi::c_char) as *mut gchar;
        } else {
            filename = g_build_filename(tzdir, identifier, NULL);
        }
        current_block = 2968425633554183086;
    } else if resolved_identifier.is_null() {
        current_block = 18137788381642338949;
    } else {
        filename = safe_c2rust_g_strdup_inline(
            b"/etc/localtime\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut gchar;
        current_block = 2968425633554183086;
    }
    match current_block {
        2968425633554183086 => {
            file = g_mapped_file_new(filename, FALSE, ::core::ptr::null_mut::<*mut GError>());
            if !file.is_null() {
                zoneinfo = g_bytes_new_with_free_func(
                    g_mapped_file_get_contents(file) as gconstpointer,
                    g_mapped_file_get_length(file),
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut GMappedFile) -> ()>,
                        GDestroyNotify,
                    >(Some(
                        g_mapped_file_unref as unsafe extern "C" fn(*mut GMappedFile) -> (),
                    )),
                    g_mapped_file_ref(file) as gpointer,
                );
                g_mapped_file_unref(file);
            }
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if !resolved_identifier.is_null() {
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
                    b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                    680 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"resolved_identifier != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    g_free(filename as gpointer);
    return zoneinfo;
}
unsafe extern "C" fn safe_c2rust_init_zone_from_iana_info(
    mut gtz: *mut GTimeZone,
    mut zoneinfo: *mut GBytes,
    mut identifier: *mut gchar,
) {
    let mut size: gsize = 0;
    let mut index: guint = 0;
    let mut time_count: guint32 = 0;
    let mut type_count: guint32 = 0;
    let mut tz_transitions: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut tz_type_index: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut tz_ttinfo: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut tz_abbrs: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut timesize: gsize = ::core::mem::size_of::<gint32>() as gsize;
    let mut header_data: gconstpointer = g_bytes_get_data(zoneinfo, &raw mut size);
    let mut data: *const gchar = header_data as *const gchar;
    let mut header: *const tzhead = header_data as *const tzhead;
    let mut footertz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut extra_time_count: guint = 0 as guint;
    let mut extra_type_count: guint = 0 as guint;
    let mut last_explicit_transition_time: gint64 = 0 as gint64;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if size as usize >= ::core::mem::size_of::<tzhead>() as usize
            && memcmp(
                header as *const ::core::ffi::c_void,
                b"TZif\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
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
            b"size >= sizeof (struct tzhead) && memcmp (header, \"TZif\", 4) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*header).tzh_version as ::core::ffi::c_int >= '2' as i32 {
        header = (header.offset(1 as ::core::ffi::c_int as isize) as *const gchar)
            .offset(safe_c2rust_guint32_from_be((*header).tzh_ttisgmtcnt) as isize)
            .offset(safe_c2rust_guint32_from_be((*header).tzh_ttisstdcnt) as isize)
            .offset(
                (8 as guint32).wrapping_mul(safe_c2rust_guint32_from_be((*header).tzh_leapcnt))
                    as isize,
            )
            .offset(
                (5 as guint32).wrapping_mul(safe_c2rust_guint32_from_be((*header).tzh_timecnt))
                    as isize,
            )
            .offset(
                (6 as guint32).wrapping_mul(safe_c2rust_guint32_from_be((*header).tzh_typecnt))
                    as isize,
            )
            .offset(safe_c2rust_guint32_from_be((*header).tzh_charcnt) as isize)
            as *const tzhead;
        timesize = ::core::mem::size_of::<gint64>() as usize as gsize;
    }
    time_count = safe_c2rust_guint32_from_be((*header).tzh_timecnt);
    type_count = safe_c2rust_guint32_from_be((*header).tzh_typecnt);
    if (*header).tzh_version as ::core::ffi::c_int >= '2' as i32 {
        let mut footer: *const gchar = (header.offset(1 as ::core::ffi::c_int as isize)
            as *const gchar)
            .offset(safe_c2rust_guint32_from_be((*header).tzh_ttisgmtcnt) as isize)
            .offset(safe_c2rust_guint32_from_be((*header).tzh_ttisstdcnt) as isize)
            .offset(
                (12 as guint32).wrapping_mul(safe_c2rust_guint32_from_be((*header).tzh_leapcnt))
                    as isize,
            )
            .offset((9 as guint32).wrapping_mul(time_count) as isize)
            .offset((6 as guint32).wrapping_mul(type_count) as isize)
            .offset(safe_c2rust_guint32_from_be((*header).tzh_charcnt) as isize);
        let mut footerlast: *const gchar = ::core::ptr::null::<gchar>();
        let mut footerlen: size_t = 0;
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if footer
                <= data
                    .offset(size as isize)
                    .offset(-(2 as ::core::ffi::c_int as isize))
                && *footer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
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
                b"footer <= data + size - 2 && footer[0] == '\\n'\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        footerlast = memchr(
            footer.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            '\n' as i32,
            data.offset(size as isize)
                .offset_from(footer.offset(1 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_long as size_t,
        ) as *const gchar;
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if !footerlast.is_null() {
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
                b"footerlast\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        footerlen = footerlast
            .offset(1 as ::core::ffi::c_int as isize)
            .offset_from(footer) as ::core::ffi::c_long as size_t;
        if footerlen != 2 as size_t {
            footertz = safe_c2rust_parse_footertz(footer, footerlen);
            if ({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if !footertz.is_null() {
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
                    b"footertz\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
            extra_type_count = (*(*footertz).t_info).len;
            extra_time_count = (*(*footertz).transitions).len;
        }
    }
    tz_transitions =
        (header as *mut guint8).offset(::core::mem::size_of::<tzhead>() as usize as isize);
    tz_type_index = tz_transitions.offset(timesize.wrapping_mul(time_count as gsize) as isize);
    tz_ttinfo = tz_type_index.offset(time_count as isize);
    tz_abbrs = tz_ttinfo.offset(
        (::core::mem::size_of::<ttinfo>() as usize).wrapping_mul(type_count as usize) as isize,
    );
    (*gtz).name =
        safe_c2rust_g_steal_pointer(&raw mut identifier as gpointer) as *mut gchar as *mut gchar;
    (*gtz).t_info = g_array_sized_new(
        FALSE,
        TRUE,
        ::core::mem::size_of::<TransitionInfo>() as guint,
        (type_count as guint).wrapping_add(extra_type_count),
    );
    (*gtz).transitions = g_array_sized_new(
        FALSE,
        TRUE,
        ::core::mem::size_of::<Transition>() as guint,
        (time_count as guint).wrapping_add(extra_time_count),
    );
    index = 0 as guint;
    while index < type_count {
        let mut t_info: TransitionInfo = TransitionInfo {
            gmt_offset: 0,
            is_dst: 0,
            abbrev: ::core::ptr::null_mut::<gchar>(),
        };
        let mut info: ttinfo = *(tz_ttinfo as *mut ttinfo).offset(index as isize);
        t_info.gmt_offset = safe_c2rust_gint32_from_be(info.tt_gmtoff);
        t_info.is_dst = (if info.tt_isdst as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }) as gboolean;
        t_info.abbrev = safe_c2rust_g_strdup_inline(
            tz_abbrs.offset(info.tt_abbrind as isize) as *mut guint8 as *mut gchar
        ) as *mut gchar;
        g_array_append_vals((*gtz).t_info, &raw mut t_info as gconstpointer, 1 as guint);
        index = index.wrapping_add(1);
    }
    index = 0 as guint;
    while index < time_count {
        let mut trans: Transition = Transition {
            time: 0,
            info_index: 0,
        };
        if (*header).tzh_version as ::core::ffi::c_int >= '2' as i32 {
            trans.time = safe_c2rust_gint64_from_be(
                *(tz_transitions as *mut gint64_be).offset(index as isize),
            );
        } else {
            trans.time = safe_c2rust_gint32_from_be(
                *(tz_transitions as *mut gint32_be).offset(index as isize),
            ) as gint64;
        }
        last_explicit_transition_time = trans.time;
        trans.info_index = *tz_type_index.offset(index as isize) as gint;
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if trans.info_index >= 0 as ::core::ffi::c_int {
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
                b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                781 as ::core::ffi::c_int,
                G_STRFUNC,
                b"trans.info_index >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if (trans.info_index as guint) < (*(*gtz).t_info).len {
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
                b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                782 as ::core::ffi::c_int,
                G_STRFUNC,
                b"(guint) trans.info_index < gtz->t_info->len\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_array_append_vals(
            (*gtz).transitions,
            &raw mut trans as gconstpointer,
            1 as guint,
        );
        index = index.wrapping_add(1);
    }
    if !footertz.is_null() {
        index = 0 as guint;
        while index < extra_type_count {
            let mut t_info_0: TransitionInfo = TransitionInfo {
                gmt_offset: 0,
                is_dst: 0,
                abbrev: ::core::ptr::null_mut::<gchar>(),
            };
            let mut footer_t_info: *mut TransitionInfo =
                ((*(*footertz).t_info).data as *mut ::core::ffi::c_void as *mut TransitionInfo)
                    .offset(index as isize) as *mut TransitionInfo;
            t_info_0.gmt_offset = (*footer_t_info).gmt_offset;
            t_info_0.is_dst = (*footer_t_info).is_dst;
            t_info_0.abbrev =
                safe_c2rust_g_steal_pointer(&raw mut (*footer_t_info).abbrev as gpointer)
                    as *mut gchar as *mut gchar;
            g_array_append_vals(
                (*gtz).t_info,
                &raw mut t_info_0 as gconstpointer,
                1 as guint,
            );
            index = index.wrapping_add(1);
        }
        index = 0 as guint;
        while index < extra_time_count {
            let mut footer_transition: *mut Transition =
                ((*(*footertz).transitions).data as *mut ::core::ffi::c_void as *mut Transition)
                    .offset(index as isize) as *mut Transition;
            if time_count <= 0 as guint32
                || last_explicit_transition_time < (*footer_transition).time
            {
                let mut trans_0: Transition = Transition {
                    time: 0,
                    info_index: 0,
                };
                trans_0.time = (*footer_transition).time;
                trans_0.info_index =
                    type_count.wrapping_add((*footer_transition).info_index as guint32) as gint;
                g_array_append_vals(
                    (*gtz).transitions,
                    &raw mut trans_0 as gconstpointer,
                    1 as guint,
                );
            }
            index = index.wrapping_add(1);
        }
        safe_c2rust_g_time_zone_unref(footertz);
    }
}
unsafe extern "C" fn safe_c2rust_find_relative_date(mut buffer: *mut TimeZoneDate) {
    let mut wday: guint = 0;
    let mut date: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    g_date_clear(&raw mut date, 1 as guint);
    wday = (*buffer).wday as guint;
    if (*buffer).mon == 13 as ::core::ffi::c_int || (*buffer).mon == 14 as ::core::ffi::c_int {
        g_date_set_dmy(
            &raw mut date,
            1 as GDateDay,
            G_DATE_JANUARY,
            (*buffer).year as GDateYear,
        );
        if wday >= 59 as guint
            && (*buffer).mon == 13 as ::core::ffi::c_int
            && g_date_is_leap_year((*buffer).year as GDateYear) != 0
        {
            g_date_add_days(&raw mut date, wday);
        } else {
            g_date_add_days(&raw mut date, wday.wrapping_sub(1 as guint));
        }
        (*buffer).mon = g_date_get_month(&raw mut date) as ::core::ffi::c_int as gint;
        (*buffer).mday = g_date_get_day(&raw mut date) as ::core::ffi::c_int as gint;
        (*buffer).wday = 0 as ::core::ffi::c_int as gint;
    } else {
        let mut days: guint = 0;
        let mut days_in_month: guint =
            g_date_get_days_in_month((*buffer).mon as GDateMonth, (*buffer).year as GDateYear)
                as guint;
        let mut first_wday: GDateWeekday = G_DATE_BAD_WEEKDAY;
        g_date_set_dmy(
            &raw mut date,
            1 as GDateDay,
            (*buffer).mon as GDateMonth,
            (*buffer).year as GDateYear,
        );
        first_wday = g_date_get_weekday(&raw mut date);
        if first_wday as guint > wday {
            (*buffer).week += 1;
        }
        days = ((7 as ::core::ffi::c_int
            * ((*buffer).week as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            as ::core::ffi::c_uint)
            .wrapping_add(wday as ::core::ffi::c_uint)
            .wrapping_sub(first_wday as ::core::ffi::c_uint) as guint;
        while days >= days_in_month {
            days = days.wrapping_sub(7 as guint);
        }
        g_date_add_days(&raw mut date, days);
        (*buffer).mday = g_date_get_day(&raw mut date) as gint;
    };
}
unsafe extern "C" fn safe_c2rust_boundary_for_year(
    mut boundary: *mut TimeZoneDate,
    mut year: gint,
    mut offset: gint32,
) -> gint64 {
    let mut buffer: TimeZoneDate = TimeZoneDate {
        year: 0,
        mon: 0,
        mday: 0,
        wday: 0,
        week: 0,
        offset: 0,
    };
    let mut date: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    let unix_epoch_start: guint64 = 719163 as guint64;
    let seconds_per_day: guint64 = 86400 as guint64;
    if (*boundary).mon == 0 {
        return 0 as gint64;
    }
    buffer = *boundary;
    if (*boundary).year == 0 as ::core::ffi::c_int {
        buffer.year = year;
        if buffer.wday != 0 {
            safe_c2rust_find_relative_date(&raw mut buffer);
        }
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if buffer.year == year {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1203 as ::core::ffi::c_int,
            G_STRFUNC,
            b"buffer.year == year\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_date_clear(&raw mut date, 1 as guint);
    g_date_set_dmy(
        &raw mut date,
        buffer.mday as GDateDay,
        buffer.mon as GDateMonth,
        buffer.year as GDateYear,
    );
    return (g_date_get_julian(&raw mut date) as guint64)
        .wrapping_sub(unix_epoch_start)
        .wrapping_mul(seconds_per_day)
        .wrapping_add(buffer.offset as guint64)
        .wrapping_sub(offset as guint64) as gint64;
}
unsafe extern "C" fn safe_c2rust_fill_transition_info_from_rule(
    mut info: *mut TransitionInfo,
    mut rule: *mut TimeZoneRule,
    mut is_dst: gboolean,
) {
    let mut offset: gint = if is_dst != 0 {
        (*rule).dlt_offset as gint
    } else {
        (*rule).std_offset as gint
    };
    let mut name: *mut gchar = if is_dst != 0 {
        &raw mut (*rule).dlt_name as *mut gchar
    } else {
        &raw mut (*rule).std_name as *mut gchar
    };
    (*info).gmt_offset = offset as gint32;
    (*info).is_dst = is_dst;
    if !name.is_null() {
        (*info).abbrev = safe_c2rust_g_strdup_inline(name) as *mut gchar;
    } else {
        (*info).abbrev = g_strdup_printf(
            b"%+03d%02d\0" as *const u8 as *const gchar,
            offset / 3600 as ::core::ffi::c_int,
            abs(offset as ::core::ffi::c_int / 60 as ::core::ffi::c_int) % 60 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn safe_c2rust_init_zone_from_rules(
    mut gtz: *mut GTimeZone,
    mut rules: *mut TimeZoneRule,
    mut rules_num: guint,
    mut identifier: *mut gchar,
) {
    let mut type_count: guint = 0 as guint;
    let mut trans_count: guint = 0 as guint;
    let mut info_index: guint = 0 as guint;
    let mut ri: guint = 0;
    let mut skip_first_std_trans: gboolean = TRUE;
    let mut last_offset: gint32 = 0;
    type_count = 0 as guint;
    trans_count = 0 as guint;
    ri = 0 as guint;
    while ri < rules_num.wrapping_sub(1 as guint) {
        if (*rules.offset(ri as isize)).dlt_start.mon != 0
            || (*rules.offset(ri as isize)).dlt_end.mon != 0
        {
            let mut rulespan: guint = (*rules.offset(ri.wrapping_add(1 as guint) as isize))
                .start_year
                .wrapping_sub((*rules.offset(ri as isize)).start_year);
            let mut transitions: guint =
                (if (*rules.offset(ri as isize)).dlt_start.mon > 0 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as guint;
            transitions = transitions.wrapping_add(
                (if (*rules.offset(ri as isize)).dlt_end.mon > 0 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as guint,
            );
            type_count = type_count.wrapping_add(
                (if (*rules.offset(ri as isize)).dlt_start.mon > 0 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as guint,
            );
            trans_count = trans_count.wrapping_add(transitions.wrapping_mul(rulespan));
        } else {
            type_count = type_count.wrapping_add(1);
        }
        ri = ri.wrapping_add(1);
    }
    (*gtz).name =
        safe_c2rust_g_steal_pointer(&raw mut identifier as gpointer) as *mut gchar as *mut gchar;
    (*gtz).t_info = g_array_sized_new(
        FALSE,
        TRUE,
        ::core::mem::size_of::<TransitionInfo>() as guint,
        type_count,
    );
    (*gtz).transitions = g_array_sized_new(
        FALSE,
        TRUE,
        ::core::mem::size_of::<Transition>() as guint,
        trans_count,
    );
    last_offset = (*rules.offset(0 as ::core::ffi::c_int as isize)).std_offset;
    ri = 0 as guint;
    while ri < rules_num.wrapping_sub(1 as guint) {
        if ((*rules.offset(ri as isize)).std_offset != 0
            || (*rules.offset(ri as isize)).dlt_offset != 0)
            && (*rules.offset(ri as isize)).dlt_start.mon == 0 as ::core::ffi::c_int
            && (*rules.offset(ri as isize)).dlt_end.mon == 0 as ::core::ffi::c_int
        {
            let mut std_info: TransitionInfo = TransitionInfo {
                gmt_offset: 0,
                is_dst: 0,
                abbrev: ::core::ptr::null_mut::<gchar>(),
            };
            safe_c2rust_fill_transition_info_from_rule(
                &raw mut std_info,
                rules.offset(ri as isize) as *mut TimeZoneRule,
                FALSE,
            );
            g_array_append_vals(
                (*gtz).t_info,
                &raw mut std_info as gconstpointer,
                1 as guint,
            );
            if ri > 0 as guint
                && ((*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                    .dlt_start
                    .mon
                    > 12 as ::core::ffi::c_int
                    && (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                        .dlt_start
                        .wday
                        > (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                            .dlt_end
                            .wday
                    || (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                        .dlt_start
                        .mon
                        > (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                            .dlt_end
                            .mon)
            {
                let mut year: guint = (*rules.offset(ri as isize)).start_year;
                let mut std_time: gint64 = safe_c2rust_boundary_for_year(
                    &raw mut (*rules.offset(ri as isize)).dlt_end,
                    year as gint,
                    last_offset,
                );
                let mut std_trans: Transition = Transition {
                    time: std_time,
                    info_index: info_index as gint,
                };
                g_array_append_vals(
                    (*gtz).transitions,
                    &raw mut std_trans as gconstpointer,
                    1 as guint,
                );
            }
            last_offset = (*rules.offset(ri as isize)).std_offset;
            info_index = info_index.wrapping_add(1);
            skip_first_std_trans = TRUE as gboolean;
        } else {
            let start_year: guint = (*rules.offset(ri as isize)).start_year;
            let end_year: guint = (*rules.offset(ri.wrapping_add(1 as guint) as isize)).start_year;
            let mut dlt_first: gboolean = 0;
            let mut year_0: guint = 0;
            let mut std_info_0: TransitionInfo = TransitionInfo {
                gmt_offset: 0,
                is_dst: 0,
                abbrev: ::core::ptr::null_mut::<gchar>(),
            };
            let mut dlt_info: TransitionInfo = TransitionInfo {
                gmt_offset: 0,
                is_dst: 0,
                abbrev: ::core::ptr::null_mut::<gchar>(),
            };
            if (*rules.offset(ri as isize)).dlt_start.mon > 12 as ::core::ffi::c_int {
                dlt_first = ((*rules.offset(ri as isize)).dlt_start.wday
                    > (*rules.offset(ri as isize)).dlt_end.wday)
                    as ::core::ffi::c_int as gboolean;
            } else {
                dlt_first = ((*rules.offset(ri as isize)).dlt_start.mon
                    > (*rules.offset(ri as isize)).dlt_end.mon)
                    as ::core::ffi::c_int as gboolean;
            }
            safe_c2rust_fill_transition_info_from_rule(
                &raw mut std_info_0,
                rules.offset(ri as isize) as *mut TimeZoneRule,
                FALSE,
            );
            safe_c2rust_fill_transition_info_from_rule(
                &raw mut dlt_info,
                rules.offset(ri as isize) as *mut TimeZoneRule,
                TRUE,
            );
            g_array_append_vals(
                (*gtz).t_info,
                &raw mut std_info_0 as gconstpointer,
                1 as guint,
            );
            g_array_append_vals(
                (*gtz).t_info,
                &raw mut dlt_info as gconstpointer,
                1 as guint,
            );
            year_0 = start_year;
            while year_0 < end_year {
                let mut dlt_offset: gint32 = if dlt_first != 0 {
                    last_offset
                } else {
                    (*rules.offset(ri as isize)).dlt_offset
                };
                let mut std_offset: gint32 = if dlt_first != 0 {
                    (*rules.offset(ri as isize)).std_offset
                } else {
                    last_offset
                };
                let mut std_time_0: gint64 = safe_c2rust_boundary_for_year(
                    &raw mut (*rules.offset(ri as isize)).dlt_end,
                    year_0 as gint,
                    dlt_offset,
                );
                let mut dlt_time: gint64 = safe_c2rust_boundary_for_year(
                    &raw mut (*rules.offset(ri as isize)).dlt_start,
                    year_0 as gint,
                    std_offset,
                );
                let mut std_trans_0: Transition = Transition {
                    time: std_time_0,
                    info_index: info_index as gint,
                };
                let mut dlt_trans: Transition = Transition {
                    time: dlt_time,
                    info_index: info_index.wrapping_add(1 as guint) as gint,
                };
                last_offset = if dlt_first != 0 {
                    (*rules.offset(ri as isize)).dlt_offset
                } else {
                    (*rules.offset(ri as isize)).std_offset
                };
                if dlt_first != 0 {
                    if skip_first_std_trans != 0 {
                        skip_first_std_trans = FALSE as gboolean;
                    } else if std_time_0 != 0 {
                        g_array_append_vals(
                            (*gtz).transitions,
                            &raw mut std_trans_0 as gconstpointer,
                            1 as guint,
                        );
                    }
                    if dlt_time != 0 {
                        g_array_append_vals(
                            (*gtz).transitions,
                            &raw mut dlt_trans as gconstpointer,
                            1 as guint,
                        );
                    }
                } else {
                    if dlt_time != 0 {
                        g_array_append_vals(
                            (*gtz).transitions,
                            &raw mut dlt_trans as gconstpointer,
                            1 as guint,
                        );
                    }
                    if std_time_0 != 0 {
                        g_array_append_vals(
                            (*gtz).transitions,
                            &raw mut std_trans_0 as gconstpointer,
                            1 as guint,
                        );
                    }
                }
                year_0 = year_0.wrapping_add(1);
            }
            info_index = info_index.wrapping_add(2 as guint);
        }
        ri = ri.wrapping_add(1);
    }
    if ri > 0 as guint
        && ((*rules.offset(ri.wrapping_sub(1 as guint) as isize))
            .dlt_start
            .mon
            > 12 as ::core::ffi::c_int
            && (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                .dlt_start
                .wday
                > (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                    .dlt_end
                    .wday
            || (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                .dlt_start
                .mon
                > (*rules.offset(ri.wrapping_sub(1 as guint) as isize))
                    .dlt_end
                    .mon)
    {
        let mut info: TransitionInfo = TransitionInfo {
            gmt_offset: 0,
            is_dst: 0,
            abbrev: ::core::ptr::null_mut::<gchar>(),
        };
        let mut year_1: guint = (*rules.offset(ri as isize)).start_year;
        let mut trans: Transition = Transition {
            time: 0,
            info_index: 0,
        };
        safe_c2rust_fill_transition_info_from_rule(
            &raw mut info,
            rules.offset(ri.wrapping_sub(1 as guint) as isize) as *mut TimeZoneRule,
            FALSE,
        );
        g_array_append_vals((*gtz).t_info, &raw mut info as gconstpointer, 1 as guint);
        trans.time = safe_c2rust_boundary_for_year(
            &raw mut (*rules.offset(ri.wrapping_sub(1 as guint) as isize)).dlt_end,
            year_1 as gint,
            last_offset,
        );
        trans.info_index = info_index as gint;
        g_array_append_vals(
            (*gtz).transitions,
            &raw mut trans as gconstpointer,
            1 as guint,
        );
    }
}
unsafe extern "C" fn safe_c2rust_parse_mwd_boundary(
    mut pos: *mut *mut gchar,
    mut boundary: *mut TimeZoneDate,
) -> gboolean {
    let mut month: gint = 0;
    let mut week: gint = 0;
    let mut day: gint = 0;
    if **pos as ::core::ffi::c_int == '\0' as i32
        || (**pos as ::core::ffi::c_int) < '0' as i32
        || ('9' as i32) < **pos as ::core::ffi::c_int
    {
        return FALSE;
    }
    let fresh21 = *pos;
    *pos = (*pos).offset(1);
    month = (*fresh21 as ::core::ffi::c_int - '0' as i32) as gint;
    if month == 1 as ::core::ffi::c_int
        && **pos as ::core::ffi::c_int >= '0' as i32
        && '2' as i32 >= **pos as ::core::ffi::c_int
        || month == 0 as ::core::ffi::c_int
            && **pos as ::core::ffi::c_int >= '0' as i32
            && '9' as i32 >= **pos as ::core::ffi::c_int
    {
        month *= 10 as ::core::ffi::c_int;
        let fresh22 = *pos;
        *pos = (*pos).offset(1);
        month += *fresh22 as ::core::ffi::c_int - '0' as i32;
    }
    let fresh23 = *pos;
    *pos = (*pos).offset(1);
    if *fresh23 as ::core::ffi::c_int != '.' as i32 || month == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if **pos as ::core::ffi::c_int == '\0' as i32
        || (**pos as ::core::ffi::c_int) < '1' as i32
        || ('5' as i32) < **pos as ::core::ffi::c_int
    {
        return FALSE;
    }
    let fresh24 = *pos;
    *pos = (*pos).offset(1);
    week = (*fresh24 as ::core::ffi::c_int - '0' as i32) as gint;
    let fresh25 = *pos;
    *pos = (*pos).offset(1);
    if *fresh25 as ::core::ffi::c_int != '.' as i32 {
        return FALSE;
    }
    if **pos as ::core::ffi::c_int == '\0' as i32
        || (**pos as ::core::ffi::c_int) < '0' as i32
        || ('6' as i32) < **pos as ::core::ffi::c_int
    {
        return FALSE;
    }
    let fresh26 = *pos;
    *pos = (*pos).offset(1);
    day = (*fresh26 as ::core::ffi::c_int - '0' as i32) as gint;
    if day == 0 {
        day += 7 as ::core::ffi::c_int;
    }
    (*boundary).year = 0 as ::core::ffi::c_int as gint;
    (*boundary).mon = month;
    (*boundary).week = week;
    (*boundary).wday = day;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_julian_boundary(
    mut pos: *mut *mut gchar,
    mut boundary: *mut TimeZoneDate,
    mut ignore_leap: gboolean,
) -> gboolean {
    let mut day: gint = 0 as gint;
    let mut date: GDate = _GDate {
        julian_days_julian_dmy_day_month_year: [0; 8],
    };
    while **pos as ::core::ffi::c_int >= '0' as i32 && '9' as i32 >= **pos as ::core::ffi::c_int {
        day *= 10 as ::core::ffi::c_int;
        let fresh20 = *pos;
        *pos = (*pos).offset(1);
        day += *fresh20 as ::core::ffi::c_int - '0' as i32;
    }
    if ignore_leap != 0 {
        if day < 1 as ::core::ffi::c_int || (365 as ::core::ffi::c_int) < day {
            return FALSE;
        }
        if day >= 59 as ::core::ffi::c_int {
            day += 1;
        }
    } else {
        if day < 0 as ::core::ffi::c_int || (365 as ::core::ffi::c_int) < day {
            return FALSE;
        }
        day += 1;
    }
    g_date_clear(&raw mut date, 1 as guint);
    g_date_set_julian(&raw mut date, day as guint32);
    (*boundary).year = 0 as ::core::ffi::c_int as gint;
    (*boundary).mon = g_date_get_month(&raw mut date) as ::core::ffi::c_int as gint;
    (*boundary).mday = g_date_get_day(&raw mut date) as ::core::ffi::c_int as gint;
    (*boundary).wday = 0 as ::core::ffi::c_int as gint;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_tz_boundary(
    mut identifier: *const gchar,
    mut boundary: *mut TimeZoneDate,
) -> gboolean {
    let mut pos: *mut gchar = ::core::ptr::null_mut::<gchar>();
    pos = identifier as *mut gchar;
    if *pos as ::core::ffi::c_int == 'M' as i32 {
        pos = pos.offset(1);
        if safe_c2rust_parse_mwd_boundary(&raw mut pos, boundary) == 0 {
            return FALSE;
        }
    } else if *pos as ::core::ffi::c_int == 'J' as i32 {
        pos = pos.offset(1);
        if safe_c2rust_parse_julian_boundary(&raw mut pos, boundary, TRUE) == 0 {
            return FALSE;
        }
    } else if *pos as ::core::ffi::c_int >= '0' as i32 && '9' as i32 >= *pos as ::core::ffi::c_int {
        if safe_c2rust_parse_julian_boundary(&raw mut pos, boundary, FALSE) == 0 {
            return FALSE;
        }
    } else {
        return FALSE;
    }
    if *pos as ::core::ffi::c_int == '/' as i32 {
        return safe_c2rust_parse_constant_offset(
            pos.offset(1 as ::core::ffi::c_int as isize),
            &raw mut (*boundary).offset,
            TRUE,
        );
    } else {
        (*boundary).offset = (2 as ::core::ffi::c_int
            * 60 as ::core::ffi::c_int
            * 60 as ::core::ffi::c_int) as gint32;
        return (*pos as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn safe_c2rust_create_ruleset_from_rule(
    mut rules: *mut *mut TimeZoneRule,
    mut rule: *mut TimeZoneRule,
) -> guint {
    *rules = ({
        let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<TimeZoneRule>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut TimeZoneRule;
    (*(*rules).offset(0 as ::core::ffi::c_int as isize)).start_year = MIN_TZYEAR as guint;
    (*(*rules).offset(1 as ::core::ffi::c_int as isize)).start_year = MAX_TZYEAR as guint;
    (*(*rules).offset(0 as ::core::ffi::c_int as isize)).std_offset = -(*rule).std_offset;
    (*(*rules).offset(0 as ::core::ffi::c_int as isize)).dlt_offset = -(*rule).dlt_offset;
    (*(*rules).offset(0 as ::core::ffi::c_int as isize)).dlt_start = (*rule).dlt_start;
    (*(*rules).offset(0 as ::core::ffi::c_int as isize)).dlt_end = (*rule).dlt_end;
    strcpy(
        &raw mut (*(*rules).offset(0 as ::core::ffi::c_int as isize)).std_name
            as *mut ::core::ffi::c_char,
        &raw mut (*rule).std_name as *mut gchar,
    );
    strcpy(
        &raw mut (*(*rules).offset(0 as ::core::ffi::c_int as isize)).dlt_name
            as *mut ::core::ffi::c_char,
        &raw mut (*rule).dlt_name as *mut gchar,
    );
    return 2 as guint;
}
unsafe extern "C" fn safe_c2rust_parse_offset(
    mut pos: *mut *mut gchar,
    mut target: *mut gint32,
) -> gboolean {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut target_pos: *mut gchar = *pos;
    let mut ret: gboolean = 0;
    while **pos as ::core::ffi::c_int == '+' as i32
        || **pos as ::core::ffi::c_int == '-' as i32
        || **pos as ::core::ffi::c_int == ':' as i32
        || **pos as ::core::ffi::c_int >= '0' as i32 && '9' as i32 >= **pos as ::core::ffi::c_int
    {
        *pos = (*pos).offset(1);
    }
    buffer = g_strndup(
        target_pos,
        (*pos).offset_from(target_pos) as ::core::ffi::c_long as gsize,
    );
    ret = safe_c2rust_parse_constant_offset(buffer, target, FALSE);
    g_free(buffer as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_identifier_boundary(
    mut pos: *mut *mut gchar,
    mut target: *mut TimeZoneDate,
) -> gboolean {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut target_pos: *mut gchar = *pos;
    let mut ret: gboolean = 0;
    while **pos as ::core::ffi::c_int != ',' as i32 && **pos as ::core::ffi::c_int != '\0' as i32 {
        *pos = (*pos).offset(1);
    }
    buffer = g_strndup(
        target_pos,
        (*pos).offset_from(target_pos) as ::core::ffi::c_long as gsize,
    );
    ret = safe_c2rust_parse_tz_boundary(buffer, target);
    g_free(buffer as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_set_tz_name(
    mut pos: *mut *mut gchar,
    mut buffer: *mut gchar,
    mut size: guint,
) -> gboolean {
    let mut quoted: gboolean = (**pos as ::core::ffi::c_int == '<' as i32) as ::core::ffi::c_int;
    let mut name_pos: *mut gchar = *pos;
    let mut len: guint = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if size != 0 as guint {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1583 as ::core::ffi::c_int,
            G_STRFUNC,
            b"size != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if quoted != 0 {
        name_pos = name_pos.offset(1);
        loop {
            *pos = (*pos).offset(1);
            if !(*safe_c2rust_g_ascii_table.offset(**pos as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_ALNUM as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || **pos as ::core::ffi::c_int == '-' as i32
                || **pos as ::core::ffi::c_int == '+' as i32)
            {
                break;
            }
        }
        if **pos as ::core::ffi::c_int != '>' as i32 {
            return FALSE;
        }
    } else {
        while *safe_c2rust_g_ascii_table.offset(**pos as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALPHA as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            *pos = (*pos).offset(1);
        }
    }
    if ((*pos).offset_from(name_pos) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
        return FALSE;
    }
    memset(
        buffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        size as size_t,
    );
    len = if (*pos).offset_from(name_pos) as ::core::ffi::c_long as guint
        > size.wrapping_sub(1 as guint)
    {
        size.wrapping_sub(1 as guint)
    } else {
        (*pos).offset_from(name_pos) as ::core::ffi::c_long as guint
    };
    strncpy(buffer as *mut ::core::ffi::c_char, name_pos, len as size_t);
    *pos = (*pos).offset(quoted as isize);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_identifier_boundaries(
    mut pos: *mut *mut gchar,
    mut tzr: *mut TimeZoneRule,
) -> gboolean {
    let fresh10 = *pos;
    *pos = (*pos).offset(1);
    if *fresh10 as ::core::ffi::c_int != ',' as i32 {
        return FALSE;
    }
    if safe_c2rust_parse_identifier_boundary(pos, &raw mut (*tzr).dlt_start) == 0 || {
        let fresh11 = *pos;
        *pos = (*pos).offset(1);
        *fresh11 as ::core::ffi::c_int != ',' as i32
    } {
        return FALSE;
    }
    if safe_c2rust_parse_identifier_boundary(pos, &raw mut (*tzr).dlt_end) == 0 {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_rules_from_identifier(
    mut identifier: *const gchar,
    mut rules: *mut *mut TimeZoneRule,
) -> guint {
    let mut pos: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tzr: TimeZoneRule = TimeZoneRule {
        start_year: 0,
        std_offset: 0,
        dlt_offset: 0,
        dlt_start: TimeZoneDate {
            year: 0,
            mon: 0,
            mday: 0,
            wday: 0,
            week: 0,
            offset: 0,
        },
        dlt_end: TimeZoneDate {
            year: 0,
            mon: 0,
            mday: 0,
            wday: 0,
            week: 0,
            offset: 0,
        },
        std_name: [0; 33],
        dlt_name: [0; 33],
    };
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !rules.is_null() {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1639 as ::core::ffi::c_int,
            G_STRFUNC,
            b"rules != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *rules = ::core::ptr::null_mut::<TimeZoneRule>();
    if identifier.is_null() {
        return 0 as guint;
    }
    pos = identifier as *mut gchar;
    memset(
        &raw mut tzr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TimeZoneRule>() as size_t,
    );
    if safe_c2rust_set_tz_name(
        &raw mut pos,
        &raw mut tzr.std_name as *mut gchar,
        NAME_SIZE as guint,
    ) == 0
        || safe_c2rust_parse_offset(&raw mut pos, &raw mut tzr.std_offset) == 0
    {
        return 0 as guint;
    }
    if *pos as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return safe_c2rust_create_ruleset_from_rule(rules, &raw mut tzr);
    }
    if safe_c2rust_set_tz_name(
        &raw mut pos,
        &raw mut tzr.dlt_name as *mut gchar,
        NAME_SIZE as guint,
    ) == 0
    {
        return 0 as guint;
    }
    safe_c2rust_parse_offset(&raw mut pos, &raw mut tzr.dlt_offset);
    if tzr.dlt_offset == 0 as ::core::ffi::c_int {
        tzr.dlt_offset =
            (tzr.std_offset as ::core::ffi::c_int - 3600 as ::core::ffi::c_int) as gint32;
    }
    if *pos as ::core::ffi::c_int == '\0' as i32 {
        return 0 as guint;
    }
    if safe_c2rust_parse_identifier_boundaries(&raw mut pos, &raw mut tzr) == 0 {
        return 0 as guint;
    }
    return safe_c2rust_create_ruleset_from_rule(rules, &raw mut tzr);
}
unsafe extern "C" fn safe_c2rust_parse_footertz(
    mut footer: *const gchar,
    mut footerlen: size_t,
) -> *mut GTimeZone {
    let mut tzstring: *mut gchar = g_strndup(
        footer.offset(1 as ::core::ffi::c_int as isize),
        (footerlen as gsize).wrapping_sub(2 as gsize),
    );
    let mut footertz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut rules: *mut TimeZoneRule = ::core::ptr::null_mut::<TimeZoneRule>();
    let mut rules_num: guint = safe_c2rust_rules_from_identifier(tzstring, &raw mut rules);
    g_free(tzstring as gpointer);
    if rules_num > 1 as guint {
        footertz = ({
            let mut __s: gsize = ::core::mem::size_of::<GTimeZone>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut GTimeZone;
        safe_c2rust_init_zone_from_rules(
            footertz,
            rules,
            rules_num,
            ::core::ptr::null_mut::<gchar>(),
        );
        (*footertz).ref_count += 1;
    }
    g_free(rules as gpointer);
    return footertz;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_new(
    mut identifier: *const gchar,
) -> *mut GTimeZone {
    let mut tz: *mut GTimeZone = safe_c2rust_g_time_zone_new_identifier(identifier);
    if tz.is_null() {
        tz = safe_c2rust_g_time_zone_new_utc();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !tz.is_null() {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1752 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_g_steal_pointer(&raw mut tz as gpointer) as *mut GTimeZone;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_new_identifier(
    mut identifier: *const gchar,
) -> *mut GTimeZone {
    let mut tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut rules: *mut TimeZoneRule = ::core::ptr::null_mut::<TimeZoneRule>();
    let mut rules_num: gint = 0;
    let mut resolved_identifier: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !identifier.is_null() {
        g_mutex_lock(&raw mut safe_c2rust_g__time_zones_lock);
        if safe_c2rust_time_zones.is_null() {
            safe_c2rust_time_zones = g_hash_table_new(
                Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            );
        }
        tz = g_hash_table_lookup(safe_c2rust_time_zones, identifier as gconstpointer)
            as *mut GTimeZone;
        if !tz.is_null() {
            if 0 as ::core::ffi::c_int != 0 {
                (*tz).ref_count;
                (*tz).ref_count;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut (*tz).ref_count,
                1 as ::core::ffi::c_int,
            );
            g_mutex_unlock(&raw mut safe_c2rust_g__time_zones_lock);
            return tz;
        } else {
            resolved_identifier =
                safe_c2rust_g_strdup_inline(identifier as *const ::core::ffi::c_char) as *mut gchar;
        }
    } else {
        g_mutex_lock(&raw mut safe_c2rust_g__tz_default_lock);
        resolved_identifier = safe_c2rust_zone_identifier_unix();
        if !safe_c2rust_tz_default.is_null() {
            if !(resolved_identifier.is_null()
                && strcmp(
                    (*safe_c2rust_tz_default).name as *const ::core::ffi::c_char,
                    b"UTC\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
                && g_strcmp0((*safe_c2rust_tz_default).name, resolved_identifier)
                    != 0 as ::core::ffi::c_int
            {
                let mut _pp: *mut *mut GTimeZone = &raw mut safe_c2rust_tz_default;
                let mut _ptr: *mut GTimeZone = *_pp;
                *_pp = ::core::ptr::null_mut::<GTimeZone>();
                if !_ptr.is_null() {
                    safe_c2rust_g_time_zone_unref(_ptr as *mut GTimeZone);
                }
            } else {
                tz = safe_c2rust_g_time_zone_ref(safe_c2rust_tz_default);
                g_mutex_unlock(&raw mut safe_c2rust_g__tz_default_lock);
                g_free(resolved_identifier as gpointer);
                return tz;
            }
        }
    }
    tz = ({
        let mut __s: gsize = ::core::mem::size_of::<GTimeZone>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GTimeZone;
    (*tz).ref_count = 0 as ::core::ffi::c_int as gint;
    safe_c2rust_zone_for_constant_offset(tz, identifier);
    if (*tz).t_info.is_null() && {
        rules_num = safe_c2rust_rules_from_identifier(identifier, &raw mut rules) as gint;
        rules_num != 0
    } {
        safe_c2rust_init_zone_from_rules(
            tz,
            rules,
            rules_num as guint,
            safe_c2rust_g_steal_pointer(&raw mut resolved_identifier as gpointer) as *mut gchar,
        );
        g_free(rules as gpointer);
    }
    if (*tz).t_info.is_null() {
        let mut zoneinfo: *mut GBytes = safe_c2rust_zone_info_unix(identifier, resolved_identifier);
        if !zoneinfo.is_null() {
            safe_c2rust_init_zone_from_iana_info(
                tz,
                zoneinfo,
                safe_c2rust_g_steal_pointer(&raw mut resolved_identifier as gpointer) as *mut gchar,
            );
            g_bytes_unref(zoneinfo);
        }
    }
    g_free(resolved_identifier as gpointer);
    if (*tz).t_info.is_null() {
        g_slice_free1(::core::mem::size_of::<GTimeZone>() as gsize, tz as gpointer);
        if !identifier.is_null() {
            g_mutex_unlock(&raw mut safe_c2rust_g__time_zones_lock);
        } else {
            g_mutex_unlock(&raw mut safe_c2rust_g__tz_default_lock);
        }
        return ::core::ptr::null_mut::<GTimeZone>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !(*tz).name.is_null() {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1959 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tz->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !(*tz).t_info.is_null() {
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            1960 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tz->t_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !identifier.is_null() {
        g_hash_table_insert(
            safe_c2rust_time_zones,
            (*tz).name as gpointer,
            tz as gpointer,
        );
    } else if !(*tz).name.is_null() {
        if 0 as ::core::ffi::c_int != 0 {
            (*tz).ref_count;
            (*tz).ref_count;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(&raw mut (*tz).ref_count, 1 as ::core::ffi::c_int);
        safe_c2rust_tz_default = tz;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*tz).ref_count;
        (*tz).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*tz).ref_count, 1 as ::core::ffi::c_int);
    if !identifier.is_null() {
        g_mutex_unlock(&raw mut safe_c2rust_g__time_zones_lock);
    } else {
        g_mutex_unlock(&raw mut safe_c2rust_g__tz_default_lock);
    }
    return tz;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_new_utc() -> *mut GTimeZone {
    static mut safe_c2rust_utc: *mut GTimeZone = ::core::ptr::null::<GTimeZone>() as *mut GTimeZone;
    static mut safe_c2rust_initialised: gsize = 0;
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
        safe_c2rust_utc =
            safe_c2rust_g_time_zone_new_identifier(b"UTC\0" as *const u8 as *const gchar);
        if ({
            let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
            if !safe_c2rust_utc.is_null() {
                _g_boolean_var_28 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_28 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_28
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
                2005 as ::core::ffi::c_int,
                G_STRFUNC,
                b"utc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
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
    return safe_c2rust_g_time_zone_ref(safe_c2rust_utc);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_new_local() -> *mut GTimeZone {
    let mut tzenv: *const gchar = g_getenv(b"TZ\0" as *const u8 as *const gchar);
    let mut tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    g_mutex_lock(&raw mut safe_c2rust_g__tz_local_lock);
    if !safe_c2rust_tz_local.is_null()
        && g_strcmp0(
            safe_c2rust_g_time_zone_get_identifier(safe_c2rust_tz_local)
                as *const ::core::ffi::c_char,
            tzenv as *const ::core::ffi::c_char,
        ) != 0
    {
        let mut _pp: *mut *mut GTimeZone = &raw mut safe_c2rust_tz_local;
        let mut _ptr: *mut GTimeZone = *_pp;
        *_pp = ::core::ptr::null_mut::<GTimeZone>();
        if !_ptr.is_null() {
            safe_c2rust_g_time_zone_unref(_ptr as *mut GTimeZone);
        }
    }
    if safe_c2rust_tz_local.is_null() {
        safe_c2rust_tz_local = safe_c2rust_g_time_zone_new_identifier(tzenv);
    }
    if safe_c2rust_tz_local.is_null() {
        safe_c2rust_tz_local = safe_c2rust_g_time_zone_new_utc();
    }
    tz = safe_c2rust_g_time_zone_ref(safe_c2rust_tz_local);
    g_mutex_unlock(&raw mut safe_c2rust_g__tz_local_lock);
    return tz;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_new_offset(mut seconds: gint32) -> *mut GTimeZone {
    let mut tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut identifier: *mut gchar = ::core::ptr::null_mut::<gchar>();
    identifier = g_strdup_printf(
        b"%c%02u:%02u:%02u\0" as *const u8 as *const gchar,
        if seconds >= 0 as ::core::ffi::c_int {
            '+' as i32
        } else {
            '-' as i32
        },
        (if seconds < 0 as ::core::ffi::c_int {
            -(seconds as ::core::ffi::c_int)
        } else {
            seconds as ::core::ffi::c_int
        }) / 60 as ::core::ffi::c_int
            / 60 as ::core::ffi::c_int,
        (if seconds < 0 as ::core::ffi::c_int {
            -(seconds as ::core::ffi::c_int)
        } else {
            seconds as ::core::ffi::c_int
        }) / 60 as ::core::ffi::c_int
            % 60 as ::core::ffi::c_int,
        (if seconds < 0 as ::core::ffi::c_int {
            -(seconds as ::core::ffi::c_int)
        } else {
            seconds as ::core::ffi::c_int
        }) % 60 as ::core::ffi::c_int,
    );
    tz = safe_c2rust_g_time_zone_new_identifier(identifier);
    if tz.is_null() {
        tz = safe_c2rust_g_time_zone_new_utc();
    } else if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_g_time_zone_get_offset(tz, 0 as gint) == seconds {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            2093 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_time_zone_get_offset (tz, 0) == seconds\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !tz.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            2095 as ::core::ffi::c_int,
            G_STRFUNC,
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_free(identifier as gpointer);
    return tz;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_info(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> *const TransitionInfo {
    let mut index: guint = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !(*tz).t_info.is_null() {
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
            b"tz->t_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<TransitionInfo>();
    }
    if interval != 0 && !(*tz).transitions.is_null() && interval <= (*(*tz).transitions).len {
        index = (*((*(*tz).transitions).data as *mut ::core::ffi::c_void as *mut Transition)
            .offset(interval.wrapping_sub(1 as guint) as isize))
        .info_index as guint;
    } else {
        index = 0 as guint;
        while index < (*(*tz).t_info).len {
            let mut tzinfo: *mut TransitionInfo =
                ((*(*tz).t_info).data as *mut ::core::ffi::c_void as *mut TransitionInfo)
                    .offset(index as isize) as *mut TransitionInfo;
            if (*tzinfo).is_dst == 0 {
                return tzinfo;
            }
            index = index.wrapping_add(1);
        }
        index = 0 as guint;
    }
    return ((*(*tz).t_info).data as *mut ::core::ffi::c_void as *mut TransitionInfo)
        .offset(index as isize) as *mut TransitionInfo;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_start(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gint64 {
    if interval == 0 || (*tz).transitions.is_null() || (*(*tz).transitions).len == 0 as guint {
        return G_MININT64;
    }
    if interval > (*(*tz).transitions).len {
        interval = (*(*tz).transitions).len;
    }
    return (*((*(*tz).transitions).data as *mut ::core::ffi::c_void as *mut Transition)
        .offset(interval.wrapping_sub(1 as guint) as isize))
    .time;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_end(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gint64 {
    if !(*tz).transitions.is_null() && interval < (*(*tz).transitions).len {
        let mut lim: gint64 = (*((*(*tz).transitions).data as *mut ::core::ffi::c_void
            as *mut Transition)
            .offset(interval as isize))
        .time;
        return lim - (lim != G_MININT64) as ::core::ffi::c_int as gint64;
    }
    return G_MAXINT64;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_offset(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gint32 {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !(*tz).t_info.is_null() {
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
            b"tz->t_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    return (*safe_c2rust_interval_info(tz, interval)).gmt_offset;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_isdst(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !(*tz).t_info.is_null() {
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
            b"tz->t_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*safe_c2rust_interval_info(tz, interval)).is_dst;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_abbrev(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !(*tz).t_info.is_null() {
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
            b"tz->t_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*safe_c2rust_interval_info(tz, interval)).abbrev;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_local_start(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gint64 {
    if interval != 0 {
        return safe_c2rust_interval_start(tz, interval)
            + safe_c2rust_interval_offset(tz, interval) as gint64;
    }
    return G_MININT64;
}
#[inline]
unsafe extern "C" fn safe_c2rust_interval_local_end(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gint64 {
    if !(*tz).transitions.is_null() && interval < (*(*tz).transitions).len {
        return safe_c2rust_interval_end(tz, interval)
            + safe_c2rust_interval_offset(tz, interval) as gint64;
    }
    return G_MAXINT64;
}
unsafe extern "C" fn safe_c2rust_interval_valid(
    mut tz: *mut GTimeZone,
    mut interval: guint,
) -> gboolean {
    if (*tz).transitions.is_null() {
        return (interval == 0 as guint) as ::core::ffi::c_int;
    }
    return (interval <= (*(*tz).transitions).len) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_adjust_time(
    mut tz: *mut GTimeZone,
    mut type_0: GTimeType,
    mut time_: *mut gint64,
) -> gint {
    let mut i: guint = 0;
    let mut intervals: guint = 0;
    let mut interval_is_dst: gboolean = 0;
    if (*tz).transitions.is_null() {
        return 0 as gint;
    }
    intervals = (*(*tz).transitions).len;
    i = 0 as guint;
    while i <= intervals {
        if *time_ <= safe_c2rust_interval_end(tz, i) {
            break;
        }
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if safe_c2rust_interval_start(tz, i) <= *time_ && *time_ <= safe_c2rust_interval_end(tz, i)
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
            b"../original/glib/gtimezone.c\0" as *const u8 as *const ::core::ffi::c_char,
            2260 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interval_start (tz, i) <= *time_ && *time_ <= interval_end (tz, i)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if type_0 as ::core::ffi::c_uint
        != G_TIME_TYPE_UNIVERSAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if *time_ < safe_c2rust_interval_local_start(tz, i) {
            i = i.wrapping_sub(1);
            if *time_ > safe_c2rust_interval_local_end(tz, i) {
                i = i.wrapping_add(1);
                *time_ = safe_c2rust_interval_local_start(tz, i);
            }
        } else if *time_ > safe_c2rust_interval_local_end(tz, i) {
            i = i.wrapping_add(1);
            if *time_ < safe_c2rust_interval_local_start(tz, i) {
                *time_ = safe_c2rust_interval_local_start(tz, i);
            }
        } else {
            interval_is_dst = safe_c2rust_interval_isdst(tz, i);
            if interval_is_dst != 0
                && type_0 as ::core::ffi::c_uint
                    != G_TIME_TYPE_DAYLIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
                || interval_is_dst == 0
                    && type_0 as ::core::ffi::c_uint
                        == G_TIME_TYPE_DAYLIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if i != 0
                    && *time_ <= safe_c2rust_interval_local_end(tz, i.wrapping_sub(1 as guint))
                {
                    i = i.wrapping_sub(1);
                } else if i < intervals
                    && *time_ >= safe_c2rust_interval_local_start(tz, i.wrapping_add(1 as guint))
                {
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    return i as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_find_interval(
    mut tz: *mut GTimeZone,
    mut type_0: GTimeType,
    mut time_: gint64,
) -> gint {
    let mut i: guint = 0;
    let mut intervals: guint = 0;
    let mut interval_is_dst: gboolean = 0;
    if (*tz).transitions.is_null() {
        return 0 as gint;
    }
    intervals = (*(*tz).transitions).len;
    i = 0 as guint;
    while i <= intervals {
        if time_ <= safe_c2rust_interval_end(tz, i) {
            break;
        }
        i = i.wrapping_add(1);
    }
    if type_0 as ::core::ffi::c_uint
        == G_TIME_TYPE_UNIVERSAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return i as gint;
    }
    if time_ < safe_c2rust_interval_local_start(tz, i) {
        i = i.wrapping_sub(1);
        if time_ > safe_c2rust_interval_local_end(tz, i) {
            return -(1 as gint);
        }
    } else if time_ > safe_c2rust_interval_local_end(tz, i) {
        i = i.wrapping_add(1);
        if time_ < safe_c2rust_interval_local_start(tz, i) {
            return -(1 as gint);
        }
    } else {
        interval_is_dst = safe_c2rust_interval_isdst(tz, i);
        if interval_is_dst != 0
            && type_0 as ::core::ffi::c_uint
                != G_TIME_TYPE_DAYLIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
            || interval_is_dst == 0
                && type_0 as ::core::ffi::c_uint
                    == G_TIME_TYPE_DAYLIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if i != 0 && time_ <= safe_c2rust_interval_local_end(tz, i.wrapping_sub(1 as guint)) {
                i = i.wrapping_sub(1);
            } else if i < intervals
                && time_ >= safe_c2rust_interval_local_start(tz, i.wrapping_add(1 as guint))
            {
                i = i.wrapping_add(1);
            }
        }
    }
    return i as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_get_abbreviation(
    mut tz: *mut GTimeZone,
    mut interval: gint,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if safe_c2rust_interval_valid(tz, interval as guint) != 0 {
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
            b"interval_valid (tz, (guint)interval)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_interval_abbrev(tz, interval as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_get_offset(
    mut tz: *mut GTimeZone,
    mut interval: gint,
) -> gint32 {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if safe_c2rust_interval_valid(tz, interval as guint) != 0 {
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
            b"interval_valid (tz, (guint)interval)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    return safe_c2rust_interval_offset(tz, interval as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_is_dst(
    mut tz: *mut GTimeZone,
    mut interval: gint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if safe_c2rust_interval_valid(tz, interval as guint) != 0 {
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
            b"interval_valid (tz, interval)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*tz).transitions.is_null() {
        return FALSE;
    }
    return safe_c2rust_interval_isdst(tz, interval as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_time_zone_get_identifier(
    mut tz: *mut GTimeZone,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !tz.is_null() {
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
            b"tz != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*tz).name;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_time_zone_new_identifier\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
