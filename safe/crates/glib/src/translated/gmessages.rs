use ::core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GVariantType;
    pub type _GVariant;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn abort() -> !;
    fn mkostemp(
        __template: *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut safe_c2rust_stdout: *mut FILE;
    static mut safe_c2rust_stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn getpid() -> __pid_t;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sendmsg(
        __fd: ::core::ffi::c_int,
        __message: *const msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn g_get_console_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn g_error_free(error: *mut GError);
    fn g_convert_with_fallback(
        str: *const gchar,
        len: gssize,
        to_codeset: *const gchar,
        from_codeset: *const gchar,
        fallback: *const gchar,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_sized_new(
        zero_terminated: gboolean,
        clear_: gboolean,
        element_size: guint,
        reserved_size: guint,
    ) -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_get_prgname() -> *const gchar;
    fn g_snprintf(string: *mut gchar, n: gulong, format: *const gchar, ...) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strlcpy(dest: *mut gchar, src: *const gchar, dest_size: gsize) -> gsize;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_new_len(init: *const gchar, len: gssize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_insert(string: *mut GString, pos: gssize, val: *const gchar) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_erase(string: *mut GString, pos: gssize, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_get_fixed_array(
        value: *mut GVariant,
        n_elements: *mut gsize,
        element_size: gsize,
    ) -> gconstpointer;
    fn g_variant_print(value: *mut GVariant, type_annotate: gboolean) -> *mut gchar;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    static mut safe_c2rust_g_log_always_fatal: GLogLevelFlags;
    static mut safe_c2rust_g_log_msg_prefix: GLogLevelFlags;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_rw_lock_writer_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_writer_unlock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_unlock(rw_lock: *mut GRWLock);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_get_real_time() -> gint64;
    fn g_pattern_match_simple(pattern: *const gchar, string: *const gchar) -> gboolean;
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_test_subprocess() -> gboolean;
    fn g_assertion_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_thread_n_created() -> guint;
    fn _g_localtime(timet: time_t, tm: *mut tm) -> gboolean;
    fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    fn _g_fd_is_journal(output_fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type time_t = __time_t;
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
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ::core::ffi::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::core::ffi::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::core::ffi::c_int,
    pub cmsg_type: ::core::ffi::c_int,
    pub __cmsg_data: [::core::ffi::c_uchar; 0],
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCM_PIDFD: C2RustUnnamed_0 = 4;
pub const SCM_SECURITY: C2RustUnnamed_0 = 3;
pub const SCM_CREDENTIALS: C2RustUnnamed_0 = 2;
pub const SCM_RIGHTS: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
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
pub type GVariantType = _GVariantType;
pub type gunichar = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
pub type GLogFunc =
    Option<unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> ()>;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GLogHandler = _GLogHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLogHandler {
    pub id: guint,
    pub log_level: GLogLevelFlags,
    pub log_func: GLogFunc,
    pub data: gpointer,
    pub destroy: GDestroyNotify,
    pub next: *mut GLogHandler,
}
pub type GLogDomain = _GLogDomain;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLogDomain {
    pub log_domain: *mut gchar,
    pub fatal_mask: GLogLevelFlags,
    pub handlers: *mut GLogHandler,
    pub next: *mut GLogDomain,
}
pub type GPrivate = _GPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub type GTestLogFatalFunc =
    Option<unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> gboolean>;
pub type GLogField = _GLogField;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLogField {
    pub key: *const gchar,
    pub value: gconstpointer,
    pub length: gssize,
}
pub type GLogWriterOutput = ::core::ffi::c_uint;
pub const G_LOG_WRITER_UNHANDLED: GLogWriterOutput = 0;
pub const G_LOG_WRITER_HANDLED: GLogWriterOutput = 1;
pub type GLogWriterFunc = Option<
    unsafe extern "C" fn(GLogLevelFlags, *const GLogField, gsize, gpointer) -> GLogWriterOutput,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub cmsghdr: cmsghdr,
    pub buf: [guint8; 24],
}
pub type GRWLock = _GRWLock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub lock: GRWLock,
    pub domains: *mut gchar,
    pub domains_set: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestExpectedMessage {
    pub log_domain: *mut gchar,
    pub log_level: GLogLevelFlags,
    pub pattern: *mut gchar,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GPrintFunc = Option<unsafe extern "C" fn(*const gchar) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const EMSGSIZE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const ENOBUFS: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXLONG: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const GLIB_SIZEOF_LONG: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXSSIZE: ::core::ffi::c_long = G_MAXLONG;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_log_set_handler_full\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_bit_nth_msf_impl(mut mask: gulong, mut nth_bit: gint) -> gint {
    if nth_bit < 0 as ::core::ffi::c_int
        || ({
            let mut _g_boolean_var_1: ::core::ffi::c_int = 0;
            if nth_bit > 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int {
                _g_boolean_var_1 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_1 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_1
        }) as ::core::ffi::c_long
            != 0
    {
        nth_bit = (GLIB_SIZEOF_LONG * 8 as ::core::ffi::c_int) as gint;
    }
    while nth_bit > 0 as ::core::ffi::c_int {
        nth_bit -= 1;
        if mask as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << nth_bit != 0 {
            return nth_bit;
        }
    }
    return -(1 as gint);
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
pub const G_LOG_LEVEL_USER_SHIFT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LOG_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_AUTH: ::core::ffi::c_int = (4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
static mut safe_c2rust_g_messages_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_g_log_domains: *mut GLogDomain =
    ::core::ptr::null::<GLogDomain>() as *mut GLogDomain;
static mut safe_c2rust_glib_print_func: GPrintFunc =
    unsafe { Some(safe_c2rust_g_default_print_func as unsafe extern "C" fn(*const gchar) -> ()) };
static mut safe_c2rust_glib_printerr_func: GPrintFunc = unsafe {
    Some(safe_c2rust_g_default_printerr_func as unsafe extern "C" fn(*const gchar) -> ())
};
static mut safe_c2rust_g_log_depth: GPrivate = _GPrivate {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    notify: None,
    future: [::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void; 2],
};
static mut safe_c2rust_g_log_structured_depth: GPrivate = _GPrivate {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    notify: None,
    future: [::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void; 2],
};
static mut safe_c2rust_default_log_func: GLogFunc = unsafe {
    Some(
        safe_c2rust_g_log_default_handler
            as unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> (),
    )
};
static mut safe_c2rust_default_log_data: gpointer = NULL_1;
static mut safe_c2rust_fatal_log_func: GTestLogFatalFunc = None;
static mut safe_c2rust_fatal_log_data: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static mut safe_c2rust_log_writer_func: GLogWriterFunc = unsafe {
    Some(
        safe_c2rust_g_log_writer_default
            as unsafe extern "C" fn(
                GLogLevelFlags,
                *const GLogField,
                gsize,
                gpointer,
            ) -> GLogWriterOutput,
    )
};
static mut safe_c2rust_log_writer_user_data: gpointer = NULL_1;
static mut safe_c2rust_log_writer_user_data_free: GDestroyNotify = None;
static mut safe_c2rust_g_log_debug_enabled: gboolean = FALSE;
unsafe extern "C" fn safe_c2rust__g_log_abort(mut breakpoint: gboolean) {
    let mut debugger_present: gboolean = 0;
    if g_test_subprocess() != 0 {
        _exit(1 as ::core::ffi::c_int);
    }
    debugger_present = TRUE as gboolean;
    if debugger_present != 0 && breakpoint != 0 {
        asm!("int $03\n", options(preserves_flags, att_syntax));
    } else {
        abort();
    };
}
unsafe extern "C" fn safe_c2rust_write_string(mut stream: *mut FILE, mut string: *const gchar) {
    fputs(string as *const ::core::ffi::c_char, stream) == EOF;
}
unsafe extern "C" fn safe_c2rust_write_string_sized(
    mut stream: *mut FILE,
    mut string: *const gchar,
    mut length: gssize,
) {
    if length < 0 as gssize {
        safe_c2rust_write_string(stream, string);
    } else {
        (fwrite(
            string as *const ::core::ffi::c_void,
            1 as size_t,
            length as size_t,
            stream,
        ) as size_t)
            < length as size_t;
    };
}
unsafe extern "C" fn safe_c2rust_g_log_find_domain_L(
    mut log_domain: *const gchar,
) -> *mut GLogDomain {
    let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
    domain = safe_c2rust_g_log_domains;
    while !domain.is_null() {
        if strcmp(
            (*domain).log_domain,
            log_domain as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return domain;
        }
        domain = (*domain).next;
    }
    return ::core::ptr::null_mut::<GLogDomain>();
}
unsafe extern "C" fn safe_c2rust_g_log_domain_new_L(
    mut log_domain: *const gchar,
) -> *mut GLogDomain {
    let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
    domain = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GLogDomain>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GLogDomain;
    (*domain).log_domain =
        safe_c2rust_g_strdup_inline(log_domain as *const ::core::ffi::c_char) as *mut gchar;
    (*domain).fatal_mask = (G_LOG_FLAG_RECURSION as ::core::ffi::c_int
        | G_LOG_LEVEL_ERROR as ::core::ffi::c_int) as GLogLevelFlags;
    (*domain).handlers = ::core::ptr::null_mut::<GLogHandler>();
    (*domain).next = safe_c2rust_g_log_domains;
    safe_c2rust_g_log_domains = domain;
    return domain;
}
unsafe extern "C" fn safe_c2rust_g_log_domain_check_free_L(mut domain: *mut GLogDomain) {
    if (*domain).fatal_mask as ::core::ffi::c_int
        == G_LOG_FLAG_RECURSION as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int
        && (*domain).handlers.is_null()
    {
        let mut last: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
        let mut work: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
        last = ::core::ptr::null_mut::<GLogDomain>();
        work = safe_c2rust_g_log_domains;
        while !work.is_null() {
            if work == domain {
                if !last.is_null() {
                    (*last).next = (*domain).next;
                } else {
                    safe_c2rust_g_log_domains = (*domain).next;
                }
                g_free((*domain).log_domain as gpointer);
                g_free(domain as gpointer);
                break;
            } else {
                last = work;
                work = (*last).next;
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_log_domain_get_handler_L(
    mut domain: *mut GLogDomain,
    mut log_level: GLogLevelFlags,
    mut data: *mut gpointer,
) -> GLogFunc {
    if !domain.is_null() && log_level as ::core::ffi::c_int != 0 {
        let mut handler: *mut GLogHandler = ::core::ptr::null_mut::<GLogHandler>();
        handler = (*domain).handlers;
        while !handler.is_null() {
            if (*handler).log_level as ::core::ffi::c_int & log_level as ::core::ffi::c_int
                == log_level as ::core::ffi::c_int
            {
                *data = (*handler).data;
                return (*handler).log_func;
            }
            handler = (*handler).next;
        }
    }
    *data = safe_c2rust_default_log_data;
    return safe_c2rust_default_log_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_always_fatal(
    mut fatal_mask: GLogLevelFlags,
) -> GLogLevelFlags {
    let mut old_mask: GLogLevelFlags = 0 as GLogLevelFlags;
    fatal_mask = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        fatal_mask as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << G_LOG_LEVEL_USER_SHIFT) - 1 as ::core::ffi::c_int,
    );
    fatal_mask = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        fatal_mask as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int,
    );
    fatal_mask = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        fatal_mask as ::core::ffi::c_int & !(G_LOG_FLAG_FATAL as ::core::ffi::c_int),
    );
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    old_mask = safe_c2rust_g_log_always_fatal;
    safe_c2rust_g_log_always_fatal = fatal_mask;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    return old_mask;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_fatal_mask(
    mut log_domain: *const gchar,
    mut fatal_mask: GLogLevelFlags,
) -> GLogLevelFlags {
    let mut old_flags: GLogLevelFlags = 0 as GLogLevelFlags;
    let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
    if log_domain.is_null() {
        log_domain = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    fatal_mask = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        fatal_mask as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int,
    );
    fatal_mask = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        fatal_mask as ::core::ffi::c_int & !(G_LOG_FLAG_FATAL as ::core::ffi::c_int),
    );
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    domain = safe_c2rust_g_log_find_domain_L(log_domain);
    if domain.is_null() {
        domain = safe_c2rust_g_log_domain_new_L(log_domain);
    }
    old_flags = (*domain).fatal_mask;
    (*domain).fatal_mask = fatal_mask;
    safe_c2rust_g_log_domain_check_free_L(domain);
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    return old_flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_handler(
    mut log_domain: *const gchar,
    mut log_levels: GLogLevelFlags,
    mut log_func: GLogFunc,
    mut user_data: gpointer,
) -> guint {
    return safe_c2rust_g_log_set_handler_full(log_domain, log_levels, log_func, user_data, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_handler_full(
    mut log_domain: *const gchar,
    mut log_levels: GLogLevelFlags,
    mut log_func: GLogFunc,
    mut user_data: gpointer,
    mut destroy: GDestroyNotify,
) -> guint {
    static mut safe_c2rust_handler_id: guint = 0 as guint;
    let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
    let mut handler: *mut GLogHandler = ::core::ptr::null_mut::<GLogHandler>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if log_levels as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(log_levels & G_LOG_LEVEL_MASK) != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if log_func.is_some() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"log_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if log_domain.is_null() {
        log_domain = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    handler = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GLogHandler>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GLogHandler;
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    domain = safe_c2rust_g_log_find_domain_L(log_domain);
    if domain.is_null() {
        domain = safe_c2rust_g_log_domain_new_L(log_domain);
    }
    safe_c2rust_handler_id = safe_c2rust_handler_id.wrapping_add(1);
    (*handler).id = safe_c2rust_handler_id;
    (*handler).log_level = log_levels;
    (*handler).log_func = log_func;
    (*handler).data = user_data;
    (*handler).destroy = destroy;
    (*handler).next = (*domain).handlers;
    (*domain).handlers = handler;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    return safe_c2rust_handler_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_default_handler(
    mut log_func: GLogFunc,
    mut user_data: gpointer,
) -> GLogFunc {
    let mut old_log_func: GLogFunc = None;
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    old_log_func = safe_c2rust_default_log_func;
    safe_c2rust_default_log_func = log_func;
    safe_c2rust_default_log_data = user_data;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    return old_log_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_set_fatal_handler(
    mut log_func: GTestLogFatalFunc,
    mut user_data: gpointer,
) {
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    safe_c2rust_fatal_log_func = log_func;
    safe_c2rust_fatal_log_data = user_data;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_remove_handler(
    mut log_domain: *const gchar,
    mut handler_id: guint,
) {
    let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if handler_id > 0 as guint {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"handler_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if log_domain.is_null() {
        log_domain = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    domain = safe_c2rust_g_log_find_domain_L(log_domain);
    if !domain.is_null() {
        let mut work: *mut GLogHandler = ::core::ptr::null_mut::<GLogHandler>();
        let mut last: *mut GLogHandler = ::core::ptr::null_mut::<GLogHandler>();
        last = ::core::ptr::null_mut::<GLogHandler>();
        work = (*domain).handlers;
        while !work.is_null() {
            if (*work).id == handler_id {
                if !last.is_null() {
                    (*last).next = (*work).next;
                } else {
                    (*domain).handlers = (*work).next;
                }
                safe_c2rust_g_log_domain_check_free_L(domain);
                g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
                if (*work).destroy.is_some() {
                    (*work).destroy.expect("non-null function pointer")((*work).data);
                }
                g_free(work as gpointer);
                return;
            }
            last = work;
            work = (*last).next;
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    safe_c2rust_g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s: could not find handler with id '%d' for domain \"%s\"\0" as *const u8 as *const gchar,
        b"../original/glib/gmessages.c:893\0" as *const u8 as *const ::core::ffi::c_char,
        handler_id,
        log_domain,
    );
}
unsafe extern "C" fn safe_c2rust_strdup_convert(
    mut string: *const gchar,
    mut charset: *const gchar,
) -> *mut gchar {
    if g_utf8_validate(
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        let mut gstring: *mut GString =
            g_string_new(b"[Invalid UTF-8] \0" as *const u8 as *const gchar);
        let mut p: *mut guchar = ::core::ptr::null_mut::<guchar>();
        p = string as *mut guchar;
        while *p != 0 {
            if !((*p as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
                && *p as ::core::ffi::c_int != '\t' as i32
                && *p as ::core::ffi::c_int != '\n' as i32
                && *p as ::core::ffi::c_int != '\r' as i32
                || *p as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
                || *p as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && (*p as ::core::ffi::c_int) < 0xa0 as ::core::ffi::c_int)
                && !(*p as ::core::ffi::c_int == '\r' as i32
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != '\n' as i32)
                && (*p as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
            {
                safe_c2rust_g_string_append_c_inline(gstring, *p as gchar);
            } else {
                g_string_append_printf(
                    gstring,
                    b"\\x%02x\0" as *const u8 as *const gchar,
                    *p as guint,
                );
            }
            p = p.offset(1);
        }
        return if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(gstring, 0 as gboolean)
            } else {
                g_string_free_and_steal(gstring)
            }
        } else {
            g_string_free(gstring, 0 as gboolean)
        };
    } else {
        let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut result: *mut gchar = g_convert_with_fallback(
            string,
            -(1 as ::core::ffi::c_int) as gssize,
            charset,
            b"UTF-8\0" as *const u8 as *const gchar,
            b"?\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            &raw mut err,
        );
        if !result.is_null() {
            return result;
        } else {
            static mut safe_c2rust_warned: gboolean = FALSE;
            if safe_c2rust_warned == 0 {
                safe_c2rust_warned = TRUE as gboolean;
                fprintf(
                    safe_c2rust_stderr,
                    b"GLib: Cannot convert message: %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*err).message,
                );
            }
            g_error_free(err);
            return safe_c2rust_g_strdup_inline(string as *const ::core::ffi::c_char) as *mut gchar;
        }
    };
}
pub const FORMAT_UNSIGNED_BUFSIZE: ::core::ffi::c_int =
    GLIB_SIZEOF_LONG * 3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_format_unsigned(
    mut buf: *mut gchar,
    mut num: gulong,
    mut radix: guint,
) {
    let mut tmp: gulong = 0;
    let mut c: gchar = 0;
    let mut i: gint = 0;
    let mut n: gint = 0;
    if radix != 8 as guint && radix != 10 as guint && radix != 16 as guint {
        *buf = '\0' as i32 as gchar;
        return;
    }
    if num == 0 {
        let fresh1 = buf;
        buf = buf.offset(1);
        *fresh1 = '0' as i32 as gchar;
        *buf = '\0' as i32 as gchar;
        return;
    }
    if radix == 16 as guint {
        let fresh2 = buf;
        buf = buf.offset(1);
        *fresh2 = '0' as i32 as gchar;
        let fresh3 = buf;
        buf = buf.offset(1);
        *fresh3 = 'x' as i32 as gchar;
    } else if radix == 8 as guint {
        let fresh4 = buf;
        buf = buf.offset(1);
        *fresh4 = '0' as i32 as gchar;
    }
    n = 0 as ::core::ffi::c_int as gint;
    tmp = num;
    while tmp != 0 {
        tmp = tmp.wrapping_div(radix as gulong);
        n += 1;
    }
    i = n;
    if n > FORMAT_UNSIGNED_BUFSIZE - 3 as ::core::ffi::c_int {
        *buf = '\0' as i32 as gchar;
        return;
    }
    while num != 0 {
        i -= 1;
        c = num.wrapping_rem(radix as gulong) as gchar;
        if (c as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            *buf.offset(i as isize) = (c as ::core::ffi::c_int + '0' as i32) as gchar;
        } else {
            *buf.offset(i as isize) =
                (c as ::core::ffi::c_int + 'a' as i32 - 10 as ::core::ffi::c_int) as gchar;
        }
        num = num.wrapping_div(radix as gulong);
    }
    *buf.offset(n as isize) = '\0' as i32 as gchar;
}
static mut safe_c2rust_gmessages_use_stderr: gboolean = FALSE;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_default_set_use_stderr(mut use_stderr: gboolean) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_thread_n_created() == 0 as guint {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_thread_n_created () == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_gmessages_use_stderr = use_stderr;
}
unsafe extern "C" fn safe_c2rust_mklevel_prefix(
    mut level_prefix: *mut gchar,
    mut log_level: GLogLevelFlags,
    mut use_color: gboolean,
) -> *mut FILE {
    strcpy(
        level_prefix as *mut ::core::ffi::c_char,
        safe_c2rust_log_level_to_color(log_level, use_color) as *const ::core::ffi::c_char,
    );
    match log_level as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int {
        4 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"ERROR\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        8 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"CRITICAL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        16 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"WARNING\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        32 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"Message\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        64 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"INFO\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        128 => {
            strcat(
                level_prefix as *mut ::core::ffi::c_char,
                b"DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {
            if log_level as u64 != 0 {
                strcat(
                    level_prefix as *mut ::core::ffi::c_char,
                    b"LOG-\0" as *const u8 as *const ::core::ffi::c_char,
                );
                safe_c2rust_format_unsigned(
                    level_prefix.offset(4 as ::core::ffi::c_int as isize),
                    (log_level as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int)
                        as gulong,
                    16 as guint,
                );
            } else {
                strcat(
                    level_prefix as *mut ::core::ffi::c_char,
                    b"LOG\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    strcat(
        level_prefix as *mut ::core::ffi::c_char,
        safe_c2rust_color_reset(use_color) as *const ::core::ffi::c_char,
    );
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        strcat(
            level_prefix as *mut ::core::ffi::c_char,
            b" (recursed)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if log_level as ::core::ffi::c_int
        & (G_LOG_LEVEL_ERROR as ::core::ffi::c_int
            | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int
            | G_LOG_LEVEL_WARNING as ::core::ffi::c_int)
        != 0
    {
        strcat(
            level_prefix as *mut ::core::ffi::c_char,
            b" **\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_log_level_to_file(log_level);
}
static mut safe_c2rust_expected_messages: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_logv(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) {
    let mut was_fatal: gboolean = (log_level as ::core::ffi::c_int
        & G_LOG_FLAG_FATAL as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut was_recursion: gboolean = (log_level as ::core::ffi::c_int
        & G_LOG_FLAG_RECURSION as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut buffer: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut msg_alloc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: gint = 0;
    log_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
        log_level as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int,
    );
    if log_level as u64 == 0 {
        return;
    }
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        let mut size: gsize = 0;
        size = vsnprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            1024 as size_t,
            format as *const ::core::ffi::c_char,
            args.clone(),
        ) as gsize;
        msg = &raw mut buffer as *mut ::core::ffi::c_char;
    } else {
        msg = safe_c2rust_format_string(
            format as *const ::core::ffi::c_char,
            args.clone(),
            &raw mut msg_alloc,
        );
    }
    if !safe_c2rust_expected_messages.is_null() {
        let mut expected: *mut GTestExpectedMessage =
            (*safe_c2rust_expected_messages).data as *mut GTestExpectedMessage;
        if g_strcmp0(
            (*expected).log_domain,
            log_domain as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && log_level as ::core::ffi::c_int & (*expected).log_level as ::core::ffi::c_int
                == (*expected).log_level as ::core::ffi::c_int
            && g_pattern_match_simple((*expected).pattern, msg as *const gchar) != 0
        {
            safe_c2rust_expected_messages =
                g_slist_delete_link(safe_c2rust_expected_messages, safe_c2rust_expected_messages);
            g_free((*expected).log_domain as gpointer);
            g_free((*expected).pattern as gpointer);
            g_free(expected as gpointer);
            g_free(msg_alloc as gpointer);
            return;
        } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_DEBUG as ::core::ffi::c_int
            != G_LOG_LEVEL_DEBUG as ::core::ffi::c_int
        {
            let mut level_prefix: [gchar; 59] = [0; 59];
            let mut expected_message: *mut gchar = ::core::ptr::null_mut::<gchar>();
            safe_c2rust_mklevel_prefix(
                &raw mut level_prefix as *mut gchar,
                (*expected).log_level,
                FALSE,
            );
            expected_message = g_strdup_printf(
                b"Did not see expected message %s-%s: %s\0" as *const u8 as *const gchar,
                if !(*expected).log_domain.is_null() {
                    (*expected).log_domain as *const gchar
                } else {
                    b"**\0" as *const u8 as *const gchar
                },
                &raw mut level_prefix as *mut gchar,
                (*expected).pattern,
            );
            safe_c2rust_g_log_default_handler(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                expected_message,
                NULL_1,
            );
            g_free(expected_message as gpointer);
            log_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
                log_level as ::core::ffi::c_int | G_LOG_FLAG_FATAL as ::core::ffi::c_int,
            );
        }
    }
    i = safe_c2rust_g_bit_nth_msf_impl(log_level as gulong, -(1 as gint));
    while i >= 0 as ::core::ffi::c_int {
        let mut test_level: GLogLevelFlags = 0 as GLogLevelFlags;
        test_level = ((1 as ::core::ffi::c_long) << i) as GLogLevelFlags;
        if log_level as ::core::ffi::c_int & test_level as ::core::ffi::c_int != 0 {
            let mut domain: *mut GLogDomain = ::core::ptr::null_mut::<GLogDomain>();
            let mut log_func: GLogFunc = None;
            let mut domain_fatal_mask: GLogLevelFlags = 0 as GLogLevelFlags;
            let mut data: gpointer = NULL_1;
            let mut masquerade_fatal: gboolean = FALSE;
            let mut depth: guint = 0;
            if was_fatal != 0 {
                test_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
                    test_level as ::core::ffi::c_int | G_LOG_FLAG_FATAL as ::core::ffi::c_int,
                );
            }
            if was_recursion != 0 {
                test_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
                    test_level as ::core::ffi::c_int | G_LOG_FLAG_RECURSION as ::core::ffi::c_int,
                );
            }
            g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
            depth = g_private_get(&raw mut safe_c2rust_g_log_depth) as gulong as guint;
            domain = safe_c2rust_g_log_find_domain_L(if !log_domain.is_null() {
                log_domain
            } else {
                b"\0" as *const u8 as *const gchar
            });
            if depth != 0 {
                test_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
                    test_level as ::core::ffi::c_int | G_LOG_FLAG_RECURSION as ::core::ffi::c_int,
                );
            }
            depth = depth.wrapping_add(1);
            domain_fatal_mask = (if !domain.is_null() {
                (*domain).fatal_mask as ::core::ffi::c_int
            } else {
                G_LOG_FLAG_RECURSION as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int
            }) as GLogLevelFlags;
            if (domain_fatal_mask as ::core::ffi::c_int
                | safe_c2rust_g_log_always_fatal as ::core::ffi::c_int)
                & test_level as ::core::ffi::c_int
                != 0
            {
                test_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
                    test_level as ::core::ffi::c_int | G_LOG_FLAG_FATAL as ::core::ffi::c_int,
                );
            }
            if test_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
                log_func = Some(
                    safe_c2rust__g_log_fallback_handler
                        as unsafe extern "C" fn(
                            *const gchar,
                            GLogLevelFlags,
                            *const gchar,
                            gpointer,
                        ) -> (),
                ) as GLogFunc;
            } else {
                log_func =
                    safe_c2rust_g_log_domain_get_handler_L(domain, test_level, &raw mut data);
            }
            domain = ::core::ptr::null_mut::<GLogDomain>();
            g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
            g_private_set(
                &raw mut safe_c2rust_g_log_depth,
                depth as gulong as gpointer,
            );
            log_func.expect("non-null function pointer")(
                log_domain,
                test_level,
                msg as *const gchar,
                data,
            );
            if test_level as ::core::ffi::c_int & G_LOG_FLAG_FATAL as ::core::ffi::c_int != 0
                && test_level as ::core::ffi::c_int & G_LOG_LEVEL_ERROR as ::core::ffi::c_int == 0
            {
                masquerade_fatal = (safe_c2rust_fatal_log_func.is_some()
                    && safe_c2rust_fatal_log_func.expect("non-null function pointer")(
                        log_domain,
                        test_level,
                        msg as *const gchar,
                        safe_c2rust_fatal_log_data,
                    ) == 0) as ::core::ffi::c_int as gboolean;
            }
            if test_level as ::core::ffi::c_int & G_LOG_FLAG_FATAL as ::core::ffi::c_int != 0
                && masquerade_fatal == 0
            {
                safe_c2rust__g_log_abort(
                    (test_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int
                        == 0) as ::core::ffi::c_int,
                );
            }
            depth = depth.wrapping_sub(1);
            g_private_set(
                &raw mut safe_c2rust_g_log_depth,
                depth as gulong as gpointer,
            );
        }
        i = safe_c2rust_g_bit_nth_msf_impl(log_level as gulong, i);
    }
    g_free(msg_alloc as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut format: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    safe_c2rust_g_logv(log_domain, log_level, format, args_0.clone());
}
unsafe extern "C" fn safe_c2rust_log_level_to_priority(
    mut log_level: GLogLevelFlags,
) -> *const gchar {
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_ERROR as ::core::ffi::c_int != 0 {
        return b"3\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int != 0 {
        return b"4\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_WARNING as ::core::ffi::c_int != 0 {
        return b"4\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int != 0 {
        return b"5\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_INFO as ::core::ffi::c_int != 0 {
        return b"6\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_DEBUG as ::core::ffi::c_int != 0 {
        return b"7\0" as *const u8 as *const gchar;
    }
    return b"5\0" as *const u8 as *const gchar;
}
unsafe extern "C" fn safe_c2rust_str_to_syslog_facility(
    mut syslog_facility_str: *const gchar,
) -> ::core::ffi::c_int {
    let mut syslog_facility: ::core::ffi::c_int = LOG_USER;
    if g_strcmp0(
        syslog_facility_str as *const ::core::ffi::c_char,
        b"auth\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        syslog_facility = LOG_AUTH;
    } else if g_strcmp0(
        syslog_facility_str as *const ::core::ffi::c_char,
        b"daemon\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        syslog_facility = LOG_DAEMON;
    }
    return syslog_facility;
}
#[inline]
unsafe extern "C" fn safe_c2rust_log_level_to_file(mut log_level: GLogLevelFlags) -> *mut FILE {
    if safe_c2rust_gmessages_use_stderr != 0 {
        return safe_c2rust_stderr;
    }
    if log_level as ::core::ffi::c_int
        & (G_LOG_LEVEL_ERROR as ::core::ffi::c_int
            | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int
            | G_LOG_LEVEL_WARNING as ::core::ffi::c_int
            | G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int)
        != 0
    {
        return safe_c2rust_stderr;
    } else {
        return safe_c2rust_stdout;
    };
}
unsafe extern "C" fn safe_c2rust_log_level_to_color(
    mut log_level: GLogLevelFlags,
    mut use_color: gboolean,
) -> *const gchar {
    if use_color == 0 {
        return b"\0" as *const u8 as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_ERROR as ::core::ffi::c_int != 0 {
        return b"\x1B[1;31m\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int != 0 {
        return b"\x1B[1;35m\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_WARNING as ::core::ffi::c_int != 0 {
        return b"\x1B[1;33m\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int != 0 {
        return b"\x1B[1;32m\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_INFO as ::core::ffi::c_int != 0 {
        return b"\x1B[1;32m\0" as *const u8 as *const gchar;
    } else if log_level as ::core::ffi::c_int & G_LOG_LEVEL_DEBUG as ::core::ffi::c_int != 0 {
        return b"\x1B[1;32m\0" as *const u8 as *const gchar;
    }
    return b"\0" as *const u8 as *const gchar;
}
unsafe extern "C" fn safe_c2rust_color_reset(mut use_color: gboolean) -> *const gchar {
    if use_color == 0 {
        return b"\0" as *const u8 as *const gchar;
    }
    return b"\x1B[0m\0" as *const u8 as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_structured(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    let mut buffer: [gchar; 1025] = [0; 1025];
    let mut message_allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut format: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut message: *const gchar = ::core::ptr::null::<gchar>();
    let mut p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut n_fields: gsize = 0;
    let mut i: gsize = 0;
    let mut stack_fields: [GLogField; 16] = [_GLogField {
        key: ::core::ptr::null::<gchar>(),
        value: ::core::ptr::null::<::core::ffi::c_void>(),
        length: 0,
    }; 16];
    let mut fields: *mut GLogField = &raw mut stack_fields as *mut GLogField;
    let mut fields_allocated: *mut GLogField = ::core::ptr::null_mut::<GLogField>();
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    args_0 = args.clone();
    n_fields = 2 as gsize;
    if !log_domain.is_null() {
        n_fields = n_fields.wrapping_add(1);
    }
    p = args_0.arg::<*mut gchar>() as gpointer;
    i = n_fields;
    while strcmp(
        p as *const ::core::ffi::c_char,
        b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        let mut field: GLogField = _GLogField {
            key: ::core::ptr::null::<gchar>(),
            value: ::core::ptr::null::<::core::ffi::c_void>(),
            length: 0,
        };
        let mut key: *const gchar = p as *const gchar;
        let mut value: gconstpointer = args_0.arg::<gpointer>() as gconstpointer;
        field.key = key;
        field.value = value;
        field.length = -(1 as ::core::ffi::c_int) as gssize;
        if i < 16 as gsize {
            stack_fields[i as usize] = field;
        } else if !(log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int
            != 0)
        {
            if i == 16 as gsize {
                array = g_array_sized_new(
                    FALSE,
                    FALSE,
                    ::core::mem::size_of::<GLogField>() as guint,
                    32 as guint,
                );
                g_array_append_vals(
                    array,
                    &raw mut stack_fields as *mut GLogField as gconstpointer,
                    16 as guint,
                );
            }
            g_array_append_vals(array, &raw mut field as gconstpointer, 1 as guint);
        }
        p = args_0.arg::<*mut gchar>() as gpointer;
        i = i.wrapping_add(1);
    }
    n_fields = i;
    if !array.is_null() {
        fields_allocated = g_array_free(array, FALSE) as *mut GLogField;
        fields = fields_allocated;
    }
    format = args_0.arg::<*mut gchar>();
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        let mut size: gsize = 0;
        size = vsnprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 1025]>() as size_t,
            format,
            args_0.clone(),
        ) as gsize;
        message = &raw mut buffer as *mut gchar;
    } else {
        message = safe_c2rust_format_string(format, args_0.clone(), &raw mut message_allocated)
            as *const gchar;
    }
    let ref mut fresh12 = (*fields.offset(0 as ::core::ffi::c_int as isize)).key;
    *fresh12 = b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    let ref mut fresh13 = (*fields.offset(0 as ::core::ffi::c_int as isize)).value;
    *fresh13 = message as gconstpointer;
    (*fields.offset(0 as ::core::ffi::c_int as isize)).length =
        -(1 as ::core::ffi::c_int) as gssize;
    let ref mut fresh14 = (*fields.offset(1 as ::core::ffi::c_int as isize)).key;
    *fresh14 = b"PRIORITY\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    let ref mut fresh15 = (*fields.offset(1 as ::core::ffi::c_int as isize)).value;
    *fresh15 = safe_c2rust_log_level_to_priority(log_level) as gconstpointer;
    (*fields.offset(1 as ::core::ffi::c_int as isize)).length =
        -(1 as ::core::ffi::c_int) as gssize;
    if !log_domain.is_null() {
        let ref mut fresh16 = (*fields.offset(2 as ::core::ffi::c_int as isize)).key;
        *fresh16 = b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        let ref mut fresh17 = (*fields.offset(2 as ::core::ffi::c_int as isize)).value;
        *fresh17 = log_domain as gconstpointer;
        (*fields.offset(2 as ::core::ffi::c_int as isize)).length =
            -(1 as ::core::ffi::c_int) as gssize;
    }
    safe_c2rust_g_log_structured_array(log_level, fields, n_fields);
    g_free(fields_allocated as gpointer);
    g_free(message_allocated as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_variant(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut fields: *mut GVariant,
) {
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut fields_array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut field: GLogField = _GLogField {
        key: ::core::ptr::null::<gchar>(),
        value: ::core::ptr::null::<::core::ffi::c_void>(),
        length: 0,
    };
    let mut values_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut print_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(
            fields,
            b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) != 0
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
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (fields, G_VARIANT_TYPE_VARDICT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    print_list = ::core::ptr::null_mut::<GSList>();
    values_list = print_list;
    fields_array = g_array_new(FALSE, FALSE, ::core::mem::size_of::<GLogField>() as guint);
    field.key = b"PRIORITY\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    field.value = safe_c2rust_log_level_to_priority(log_level) as gconstpointer;
    field.length = -(1 as ::core::ffi::c_int) as gssize;
    g_array_append_vals(fields_array, &raw mut field as gconstpointer, 1 as guint);
    if !log_domain.is_null() {
        field.key = b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        field.value = log_domain as gconstpointer;
        field.length = -(1 as ::core::ffi::c_int) as gssize;
        g_array_append_vals(fields_array, &raw mut field as gconstpointer, 1 as guint);
    }
    g_variant_iter_init(&raw mut iter, fields);
    while g_variant_iter_next(
        &raw mut iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        &raw mut value,
    ) != 0
    {
        let mut defer_unref: gboolean = TRUE;
        field.key = key;
        field.length = -(1 as ::core::ffi::c_int) as gssize;
        if g_variant_is_of_type(value, G_VARIANT_TYPE_STRING) != 0 {
            field.value =
                g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()) as gconstpointer;
        } else if g_variant_is_of_type(value, G_VARIANT_TYPE_BYTESTRING) != 0 {
            let mut s: gsize = 0;
            field.value = g_variant_get_fixed_array(
                value,
                &raw mut s,
                ::core::mem::size_of::<guchar>() as gsize,
            );
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if s <= 9223372036854775807 as ::core::ffi::c_long as gsize {
                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_14
            }) as ::core::ffi::c_long
                != 0
            {
                field.length = s as gssize;
            } else {
                fprintf(
                    safe_c2rust_stderr,
                    b"Byte array too large (%lu bytes) passed to g_log_variant(). Truncating to 9223372036854775807L bytes.\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    s,
                );
                field.length = G_MAXSSIZE as gssize;
            }
        } else {
            let mut s_0: *mut ::core::ffi::c_char =
                g_variant_print(value, FALSE) as *mut ::core::ffi::c_char;
            field.value = s_0 as gconstpointer;
            print_list = g_slist_prepend(print_list, s_0 as gpointer);
            defer_unref = FALSE as gboolean;
        }
        g_array_append_vals(fields_array, &raw mut field as gconstpointer, 1 as guint);
        if ({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if defer_unref != 0 {
                _g_boolean_var_15 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_15 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_15
        }) as ::core::ffi::c_long
            != 0
        {
            values_list = g_slist_prepend(values_list, value as gpointer);
        } else {
            g_variant_unref(value);
        }
    }
    safe_c2rust_g_log_structured_array(
        log_level,
        (*fields_array).data as *mut GLogField,
        (*fields_array).len as gsize,
    );
    g_array_free(fields_array, TRUE);
    g_slist_free_full(
        values_list,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    g_slist_free_full(
        print_list,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_structured_array(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
) {
    let mut writer_func: GLogWriterFunc = None;
    let mut writer_user_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut recursion: gboolean = 0;
    let mut depth: guint = 0;
    if n_fields == 0 as gsize {
        return;
    }
    depth = g_private_get(&raw mut safe_c2rust_g_log_structured_depth) as gulong as guint;
    recursion = (depth > 0 as guint) as ::core::ffi::c_int as gboolean;
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    writer_func = (if recursion != 0 {
        Some(
            safe_c2rust__g_log_writer_fallback
                as unsafe extern "C" fn(
                    GLogLevelFlags,
                    *const GLogField,
                    gsize,
                    gpointer,
                ) -> GLogWriterOutput,
        )
    } else {
        safe_c2rust_log_writer_func
            as Option<
                unsafe extern "C" fn(
                    GLogLevelFlags,
                    *const GLogField,
                    gsize,
                    gpointer,
                ) -> GLogWriterOutput,
            >
    }) as GLogWriterFunc;
    writer_user_data = safe_c2rust_log_writer_user_data;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
    depth = depth.wrapping_add(1);
    g_private_set(
        &raw mut safe_c2rust_g_log_structured_depth,
        depth as gulong as gpointer,
    );
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if writer_func.is_some() {
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
            b"../original/glib/gmessages.c\0" as *const u8 as *const ::core::ffi::c_char,
            1858 as ::core::ffi::c_int,
            G_STRFUNC,
            b"writer_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    writer_func.expect("non-null function pointer")(log_level, fields, n_fields, writer_user_data);
    depth = depth.wrapping_sub(1);
    g_private_set(
        &raw mut safe_c2rust_g_log_structured_depth,
        depth as gulong as gpointer,
    );
    if log_level as ::core::ffi::c_int
        & (G_LOG_FLAG_RECURSION as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int)
        != 0
    {
        safe_c2rust__g_log_abort(
            (log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int == 0)
                as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_structured_standard(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut file: *const gchar,
    mut line: *const gchar,
    mut func: *const gchar,
    mut message_format: *const gchar,
    mut args: ...
) {
    let mut fields: [GLogField; 6] = [
        _GLogField {
            key: b"PRIORITY\0" as *const u8 as *const gchar,
            value: safe_c2rust_log_level_to_priority(log_level) as gconstpointer,
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
        _GLogField {
            key: b"CODE_FILE\0" as *const u8 as *const gchar,
            value: file as gconstpointer,
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
        _GLogField {
            key: b"CODE_LINE\0" as *const u8 as *const gchar,
            value: line as gconstpointer,
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
        _GLogField {
            key: b"CODE_FUNC\0" as *const u8 as *const gchar,
            value: func as gconstpointer,
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
        _GLogField {
            key: b"MESSAGE\0" as *const u8 as *const gchar,
            value: ::core::ptr::null::<::core::ffi::c_void>(),
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
        _GLogField {
            key: b"GLIB_DOMAIN\0" as *const u8 as *const gchar,
            value: log_domain as gconstpointer,
            length: -(1 as ::core::ffi::c_int) as gssize,
        },
    ];
    let mut n_fields: gsize = 0;
    let mut message_allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut buffer: [gchar; 1025] = [0; 1025];
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        let mut size: gsize = 0;
        size = vsnprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 1025]>() as size_t,
            message_format as *const ::core::ffi::c_char,
            args_0.clone(),
        ) as gsize;
        fields[4 as ::core::ffi::c_int as usize].value =
            &raw mut buffer as *mut gchar as gconstpointer;
    } else {
        fields[4 as ::core::ffi::c_int as usize].value = safe_c2rust_format_string(
            message_format as *const ::core::ffi::c_char,
            args_0.clone(),
            &raw mut message_allocated,
        ) as gconstpointer;
    }
    n_fields = (::core::mem::size_of::<[GLogField; 6]>() as usize)
        .wrapping_div(::core::mem::size_of::<GLogField>() as usize)
        .wrapping_sub(
            (if log_domain.is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as usize,
        ) as gsize;
    safe_c2rust_g_log_structured_array(log_level, &raw mut fields as *mut GLogField, n_fields);
    g_free(message_allocated as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_writer_func(
    mut func: GLogWriterFunc,
    mut user_data: gpointer,
    mut user_data_free: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if func.is_some() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g_messages_lock);
    if safe_c2rust_log_writer_func
        != Some(
            safe_c2rust_g_log_writer_default
                as unsafe extern "C" fn(
                    GLogLevelFlags,
                    *const GLogField,
                    gsize,
                    gpointer,
                ) -> GLogWriterOutput,
        )
    {
        g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
        safe_c2rust_g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_log_set_writer_func() called multiple times\0" as *const u8 as *const gchar,
        );
        loop {}
    }
    safe_c2rust_log_writer_func = func;
    safe_c2rust_log_writer_user_data = user_data;
    safe_c2rust_log_writer_user_data_free = user_data_free;
    g_mutex_unlock(&raw mut safe_c2rust_g_messages_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_supports_color(mut output_fd: gint) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if output_fd >= 0 as ::core::ffi::c_int {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"output_fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return isatty(output_fd as ::core::ffi::c_int) as gboolean;
}
static mut safe_c2rust_syslog_opened: gboolean = FALSE;
static mut safe_c2rust_journal_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn safe_c2rust_open_journal() {
    safe_c2rust_journal_fd = socket(
        AF_UNIX,
        SOCK_DGRAM as ::core::ffi::c_int | SOCK_CLOEXEC as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if safe_c2rust_journal_fd < 0 as ::core::ffi::c_int {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_is_journald(mut output_fd: gint) -> gboolean {
    return _g_fd_is_journal(output_fd as ::core::ffi::c_int) as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_format_fields(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut use_color: gboolean,
) -> *mut gchar {
    let mut i: gsize = 0;
    let mut message: *const gchar = ::core::ptr::null::<gchar>();
    let mut log_domain: *const gchar = ::core::ptr::null::<gchar>();
    let mut message_length: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut log_domain_length: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut level_prefix: [gchar; 59] = [0; 59];
    let mut gstring: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut now: gint64 = 0;
    let mut now_secs: time_t = 0;
    let mut now_tm: tm = tm {
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
    let mut time_buf: [gchar; 128] = [0; 128];
    i = 0 as gsize;
    while (message.is_null() || log_domain.is_null()) && i < n_fields {
        let mut field: *const GLogField = fields.offset(i as isize) as *const GLogField;
        if g_strcmp0(
            (*field).key as *const ::core::ffi::c_char,
            b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            message = (*field).value as *const gchar;
            message_length = (*field).length;
        } else if g_strcmp0(
            (*field).key as *const ::core::ffi::c_char,
            b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            log_domain = (*field).value as *const gchar;
            log_domain_length = (*field).length;
        }
        i = i.wrapping_add(1);
    }
    safe_c2rust_mklevel_prefix(&raw mut level_prefix as *mut gchar, log_level, use_color);
    gstring = g_string_new(::core::ptr::null::<gchar>());
    if log_level as ::core::ffi::c_int
        & (G_LOG_LEVEL_ERROR as ::core::ffi::c_int
            | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int
            | G_LOG_LEVEL_WARNING as ::core::ffi::c_int)
        != 0
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    gstring,
                    __val,
                    if ({
                        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_19
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
                gstring,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if log_domain.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"** \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    gstring,
                    __val,
                    if ({
                        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_20
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
                gstring,
                b"** \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if safe_c2rust_g_log_msg_prefix as ::core::ffi::c_int
        & (log_level as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int)
        == log_level as ::core::ffi::c_int & G_LOG_LEVEL_MASK as ::core::ffi::c_int
    {
        let mut prg_name: *const gchar = g_get_prgname();
        let mut pid: gulong = getpid() as gulong;
        if prg_name.is_null() {
            g_string_append_printf(
                gstring,
                b"(process:%lu): \0" as *const u8 as *const gchar,
                pid,
            );
        } else {
            g_string_append_printf(
                gstring,
                b"(%s:%lu): \0" as *const u8 as *const gchar,
                prg_name,
                pid,
            );
        }
    }
    if !log_domain.is_null() {
        safe_c2rust_g_string_append_len_inline(
            gstring,
            log_domain as *const ::core::ffi::c_char,
            log_domain_length,
        );
        safe_c2rust_g_string_append_c_inline(gstring, '-' as i32 as gchar);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = &raw mut level_prefix as *mut gchar;
            safe_c2rust_g_string_append_len_inline(
                gstring,
                __val,
                if ({
                    let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_21
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
            gstring,
            &raw mut level_prefix as *mut gchar,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b": \0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                gstring,
                __val,
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
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
            gstring,
            b": \0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    now = g_get_real_time();
    now_secs = now / 1000000 as gint64;
    if _g_localtime(now_secs, &raw mut now_tm) != 0 {
        strftime(
            &raw mut time_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 128]>() as size_t,
            b"%H:%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut now_tm,
        );
    } else {
        strcpy(
            &raw mut time_buf as *mut ::core::ffi::c_char,
            b"(error)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_string_append_printf(
        gstring,
        b"%s%s.%03d%s: \0" as *const u8 as *const gchar,
        if use_color != 0 {
            b"\x1B[34m\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        &raw mut time_buf as *mut gchar,
        (now / 1000 as gint64 % 1000 as gint64) as gint,
        safe_c2rust_color_reset(use_color),
    );
    if message.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"(NULL) message\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    gstring,
                    __val,
                    if ({
                        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_23
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
                gstring,
                b"(NULL) message\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        let mut msg: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut charset: *const gchar = ::core::ptr::null::<gchar>();
        msg = g_string_new_len(message, message_length);
        safe_c2rust_escape_string(msg);
        if g_get_console_charset(&raw mut charset) != 0 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = (*msg).str_0;
                    safe_c2rust_g_string_append_len_inline(
                        gstring,
                        __val,
                        if ({
                            let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_24
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
                    gstring,
                    (*msg).str_0,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            let mut lstring: *mut gchar = safe_c2rust_strdup_convert((*msg).str_0, charset);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = lstring;
                    safe_c2rust_g_string_append_len_inline(
                        gstring,
                        __val,
                        if ({
                            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_25
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
                    gstring,
                    lstring,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            g_free(lstring as gpointer);
        }
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
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(gstring, 0 as gboolean)
        } else {
            g_string_free_and_steal(gstring)
        }
    } else {
        g_string_free(gstring, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_syslog(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut user_data: gpointer,
) -> GLogWriterOutput {
    let mut i: gsize = 0;
    let mut message: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut log_domain: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut syslog_facility: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut syslog_level: ::core::ffi::c_int = 0;
    let mut message_length: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut log_domain_length: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut gstring: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !fields.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fields != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if n_fields > 0 as gsize {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_fields > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if safe_c2rust_syslog_opened == 0 {
        openlog(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        safe_c2rust_syslog_opened = TRUE as gboolean;
    }
    i = 0 as gsize;
    while i < n_fields {
        let mut field: *const GLogField = fields.offset(i as isize) as *const GLogField;
        if g_strcmp0(
            (*field).key as *const ::core::ffi::c_char,
            b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            message = (*field).value as *const ::core::ffi::c_char;
            message_length = (*field).length;
        } else if g_strcmp0(
            (*field).key as *const ::core::ffi::c_char,
            b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            log_domain = (*field).value as *const ::core::ffi::c_char;
            log_domain_length = (*field).length;
        } else if g_strcmp0(
            (*field).key as *const ::core::ffi::c_char,
            b"SYSLOG_FACILITY\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            syslog_facility = safe_c2rust_str_to_syslog_facility((*field).value as *const gchar);
        }
        i = i.wrapping_add(1);
    }
    gstring = g_string_new(::core::ptr::null::<gchar>());
    if !log_domain.is_null() {
        safe_c2rust_g_string_append_len_inline(gstring, log_domain, log_domain_length);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b": \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    gstring,
                    __val,
                    if ({
                        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_28
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
                gstring,
                b": \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    safe_c2rust_g_string_append_len_inline(gstring, message, message_length);
    syslog_level = safe_c2rust_atoi(
        safe_c2rust_log_level_to_priority(log_level) as *const ::core::ffi::c_char
    );
    syslog(
        syslog_level | syslog_facility,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*gstring).str_0,
    );
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
    return G_LOG_WRITER_HANDLED;
}
unsafe extern "C" fn safe_c2rust_journal_sendv(
    mut iov: *mut iovec,
    mut iovlen: gsize,
) -> ::core::ffi::c_int {
    let mut buf_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut mh: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut control: C2RustUnnamed_1 = C2RustUnnamed_1 {
        cmsghdr: cmsghdr {
            cmsg_len: 0,
            cmsg_level: 0,
            cmsg_type: 0,
            __cmsg_data: [],
        },
    };
    let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
    let mut path: [::core::ffi::c_char; 24] = ::core::mem::transmute::<
        [u8; 24],
        [::core::ffi::c_char; 24],
    >(*b"/dev/shm/journal.XXXXXX\0");
    if safe_c2rust_journal_fd < 0 as ::core::ffi::c_int {
        safe_c2rust_open_journal();
    }
    if safe_c2rust_journal_fd < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        &raw mut sa as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>() as size_t,
    );
    sa.sun_family = AF_UNIX as sa_family_t;
    if g_strlcpy(
        &raw mut sa.sun_path as *mut gchar,
        b"/run/systemd/journal/socket\0" as *const u8 as *const gchar,
        ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as gsize,
    ) as usize
        >= ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize
    {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        &raw mut mh as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<msghdr>() as size_t,
    );
    mh.msg_name = &raw mut sa as *mut ::core::ffi::c_void;
    mh.msg_namelen = (2 as size_t)
        .wrapping_add(strlen(&raw mut sa.sun_path as *mut ::core::ffi::c_char))
        as socklen_t;
    mh.msg_iov = iov;
    mh.msg_iovlen = iovlen as size_t;
    loop {
        if sendmsg(
            safe_c2rust_journal_fd,
            &raw mut mh,
            MSG_NOSIGNAL as ::core::ffi::c_int,
        ) >= 0 as ssize_t
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*__errno_location() == EINTR) {
            break;
        }
    }
    if *__errno_location() != EMSGSIZE && *__errno_location() != ENOBUFS {
        return -(1 as ::core::ffi::c_int);
    }
    buf_fd = mkostemp(
        &raw mut path as *mut ::core::ffi::c_char,
        O_CLOEXEC | O_RDWR,
    );
    if buf_fd < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if unlink(&raw mut path as *mut ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
        close(buf_fd);
        return -(1 as ::core::ffi::c_int);
    }
    if writev(buf_fd, iov, iovlen as ::core::ffi::c_int) < 0 as ssize_t {
        close(buf_fd);
        return -(1 as ::core::ffi::c_int);
    }
    mh.msg_iov = ::core::ptr::null_mut::<iovec>();
    mh.msg_iovlen = 0 as size_t;
    memset(
        &raw mut control as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed_1>() as size_t,
    );
    mh.msg_control = &raw mut control as *mut ::core::ffi::c_void;
    mh.msg_controllen = ::core::mem::size_of::<C2RustUnnamed_1>() as usize as size_t;
    cmsg = if mh.msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
        mh.msg_control as *mut cmsghdr
    } else {
        ::core::ptr::null_mut::<cmsghdr>()
    };
    (*cmsg).cmsg_level = SOL_SOCKET;
    (*cmsg).cmsg_type = SCM_RIGHTS as ::core::ffi::c_int;
    (*cmsg).cmsg_len = ((::core::mem::size_of::<cmsghdr>() as usize)
        .wrapping_add(::core::mem::size_of::<size_t>() as usize)
        .wrapping_sub(1 as usize)
        & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
    .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        as size_t;
    memcpy(
        &raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut buf_fd as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    );
    mh.msg_controllen = (*cmsg).cmsg_len;
    loop {
        if sendmsg(
            safe_c2rust_journal_fd,
            &raw mut mh,
            MSG_NOSIGNAL as ::core::ffi::c_int,
        ) >= 0 as ssize_t
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*__errno_location() == EINTR) {
            break;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_journald(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut user_data: gpointer,
) -> GLogWriterOutput {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let equals: ::core::ffi::c_char = '=' as i32 as ::core::ffi::c_char;
    let newline: ::core::ffi::c_char = '\n' as i32 as ::core::ffi::c_char;
    let mut i: gsize = 0;
    let mut k: gsize = 0;
    let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
    let mut v: *mut iovec = ::core::ptr::null_mut::<iovec>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut retval: gint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !fields.is_null() {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fields != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if n_fields > 0 as gsize {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_fields > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<iovec>() as usize)
            .wrapping_mul(5 as usize)
            .wrapping_mul(n_fields as usize) as usize,
    ));
    iov = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut iovec;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (32 as gsize).wrapping_mul(n_fields) as usize,
    ));
    buf = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut ::core::ffi::c_char;
    k = 0 as gsize;
    v = iov;
    i = 0 as gsize;
    while i < n_fields {
        let mut length: guint64 = 0;
        let mut binary: gboolean = 0;
        if (*fields.offset(i as isize)).length < 0 as gssize {
            length =
                strlen((*fields.offset(i as isize)).value as *const ::core::ffi::c_char) as guint64;
            binary = (strchr(
                (*fields.offset(i as isize)).value as *const ::core::ffi::c_char,
                '\n' as i32,
            ) != NULL_1 as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                as gboolean;
        } else {
            length = (*fields.offset(i as isize)).length as guint64;
            binary = TRUE as gboolean;
        }
        if binary != 0 {
            let mut nstr: guint64 = 0;
            let ref mut fresh5 = (*v.offset(0 as ::core::ffi::c_int as isize)).iov_base;
            *fresh5 = (*fields.offset(i as isize)).key as gpointer as *mut ::core::ffi::c_void;
            (*v.offset(0 as ::core::ffi::c_int as isize)).iov_len =
                strlen((*fields.offset(i as isize)).key as *const ::core::ffi::c_char);
            let ref mut fresh6 = (*v.offset(1 as ::core::ffi::c_int as isize)).iov_base;
            *fresh6 = &raw const newline as gpointer as *mut ::core::ffi::c_void;
            (*v.offset(1 as ::core::ffi::c_int as isize)).iov_len = 1 as size_t;
            nstr = length;
            memcpy(
                buf.offset(k as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                &raw mut nstr as *const ::core::ffi::c_void,
                ::core::mem::size_of::<guint64>() as size_t,
            );
            let ref mut fresh7 = (*v.offset(2 as ::core::ffi::c_int as isize)).iov_base;
            *fresh7 =
                buf.offset(k as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*v.offset(2 as ::core::ffi::c_int as isize)).iov_len =
                ::core::mem::size_of::<guint64>() as usize as size_t;
            v = v.offset(3 as ::core::ffi::c_int as isize);
            k = (k as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<guint64>() as usize as ::core::ffi::c_ulong)
                as gsize as gsize;
        } else {
            let ref mut fresh8 = (*v.offset(0 as ::core::ffi::c_int as isize)).iov_base;
            *fresh8 = (*fields.offset(i as isize)).key as gpointer as *mut ::core::ffi::c_void;
            (*v.offset(0 as ::core::ffi::c_int as isize)).iov_len =
                strlen((*fields.offset(i as isize)).key as *const ::core::ffi::c_char);
            let ref mut fresh9 = (*v.offset(1 as ::core::ffi::c_int as isize)).iov_base;
            *fresh9 = &raw const equals as gpointer as *mut ::core::ffi::c_void;
            (*v.offset(1 as ::core::ffi::c_int as isize)).iov_len = 1 as size_t;
            v = v.offset(2 as ::core::ffi::c_int as isize);
        }
        let ref mut fresh10 = (*v.offset(0 as ::core::ffi::c_int as isize)).iov_base;
        *fresh10 = (*fields.offset(i as isize)).value as gpointer as *mut ::core::ffi::c_void;
        (*v.offset(0 as ::core::ffi::c_int as isize)).iov_len = length as size_t;
        let ref mut fresh11 = (*v.offset(1 as ::core::ffi::c_int as isize)).iov_base;
        *fresh11 = &raw const newline as gpointer as *mut ::core::ffi::c_void;
        (*v.offset(1 as ::core::ffi::c_int as isize)).iov_len = 1 as size_t;
        v = v.offset(2 as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
    retval =
        safe_c2rust_journal_sendv(iov, v.offset_from(iov) as ::core::ffi::c_long as gsize) as gint;
    return (if retval == 0 as ::core::ffi::c_int {
        G_LOG_WRITER_HANDLED as ::core::ffi::c_int
    } else {
        G_LOG_WRITER_UNHANDLED as ::core::ffi::c_int
    }) as GLogWriterOutput;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_standard_streams(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut user_data: gpointer,
) -> GLogWriterOutput {
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !fields.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fields != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if n_fields > 0 as gsize {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_fields > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    stream = safe_c2rust_log_level_to_file(log_level);
    if stream.is_null() || fileno(stream) < 0 as ::core::ffi::c_int {
        return G_LOG_WRITER_UNHANDLED;
    }
    out = safe_c2rust_g_log_writer_format_fields(
        log_level,
        fields,
        n_fields,
        safe_c2rust_g_log_writer_supports_color(fileno(stream) as gint),
    );
    fprintf(
        stream,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        out,
    );
    fflush(stream);
    g_free(out as gpointer);
    return G_LOG_WRITER_HANDLED;
}
unsafe extern "C" fn safe_c2rust_log_is_old_api(
    mut fields: *const GLogField,
    mut n_fields: gsize,
) -> gboolean {
    return (n_fields >= 1 as gsize
        && g_strcmp0(
            (*fields.offset(0 as ::core::ffi::c_int as isize)).key as *const ::core::ffi::c_char,
            b"GLIB_OLD_LOG_API\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            (*fields.offset(0 as ::core::ffi::c_int as isize)).value as *const ::core::ffi::c_char,
            b"1\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_domain_found(
    mut domains: *const gchar,
    mut log_domain: *const ::core::ffi::c_char,
) -> gboolean {
    let mut len: guint = 0;
    let mut found: *const gchar = ::core::ptr::null::<gchar>();
    len = strlen(log_domain) as guint;
    found = strstr(domains as *const ::core::ffi::c_char, log_domain);
    while !found.is_null() {
        if (found == domains
            || *found.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == ' ' as i32)
            && (*found.offset(len as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *found.offset(len as isize) as ::core::ffi::c_int == ' ' as i32)
        {
            return TRUE;
        }
        found = strstr(found.offset(1 as ::core::ffi::c_int as isize), log_domain);
    }
    return FALSE;
}
static mut safe_c2rust_g_log_global: C2RustUnnamed_2 = C2RustUnnamed_2 {
    lock: _GRWLock {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        i: [0; 2],
    },
    domains: ::core::ptr::null::<gchar>() as *mut gchar,
    domains_set: 0,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_default_set_debug_domains(
    mut domains: *const *const gchar,
) {
    g_rw_lock_writer_lock(&raw mut safe_c2rust_g_log_global.lock);
    g_free(safe_c2rust_g_log_global.domains as gpointer);
    safe_c2rust_g_log_global.domains = if !domains.is_null() {
        g_strjoinv(
            b" \0" as *const u8 as *const gchar,
            domains as *mut *mut gchar,
        )
    } else {
        ::core::ptr::null_mut::<gchar>()
    };
    safe_c2rust_g_log_global.domains_set = TRUE as gboolean;
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_g_log_global.lock);
}
unsafe extern "C" fn safe_c2rust_should_drop_message(
    mut log_level: GLogLevelFlags,
    mut log_domain: *const ::core::ffi::c_char,
    mut fields: *const GLogField,
    mut n_fields: gsize,
) -> gboolean {
    if log_level as ::core::ffi::c_int
        & (G_LOG_LEVEL_ERROR as ::core::ffi::c_int
            | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int
            | G_LOG_LEVEL_WARNING as ::core::ffi::c_int
            | G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int)
        == 0
        && log_level as ::core::ffi::c_int >> G_LOG_LEVEL_USER_SHIFT == 0
        && safe_c2rust_g_log_get_debug_enabled() == 0
    {
        let mut i: gsize = 0;
        g_rw_lock_reader_lock(&raw mut safe_c2rust_g_log_global.lock);
        if ({
            let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
            if safe_c2rust_g_log_global.domains_set == 0 {
                _g_boolean_var_33 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_33 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_33
        }) as ::core::ffi::c_long
            != 0
        {
            safe_c2rust_g_log_global.domains =
                safe_c2rust_g_strdup_inline(g_getenv(
                    b"G_MESSAGES_DEBUG\0" as *const u8 as *const gchar,
                ) as *const ::core::ffi::c_char) as *mut gchar;
            safe_c2rust_g_log_global.domains_set = TRUE as gboolean;
        }
        if log_level as ::core::ffi::c_int
            & (G_LOG_LEVEL_INFO as ::core::ffi::c_int | G_LOG_LEVEL_DEBUG as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
            || safe_c2rust_g_log_global.domains.is_null()
        {
            g_rw_lock_reader_unlock(&raw mut safe_c2rust_g_log_global.lock);
            return TRUE;
        }
        if log_domain.is_null() {
            i = 0 as gsize;
            while i < n_fields {
                if g_strcmp0(
                    (*fields.offset(i as isize)).key as *const ::core::ffi::c_char,
                    b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    log_domain = (*fields.offset(i as isize)).value as *const ::core::ffi::c_char;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        if strcmp(
            safe_c2rust_g_log_global.domains,
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && (log_domain.is_null()
                || safe_c2rust_domain_found(safe_c2rust_g_log_global.domains, log_domain) == 0)
        {
            g_rw_lock_reader_unlock(&raw mut safe_c2rust_g_log_global.lock);
            return TRUE;
        }
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_g_log_global.lock);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_default_would_drop(
    mut log_level: GLogLevelFlags,
    mut log_domain: *const ::core::ffi::c_char,
) -> gboolean {
    return safe_c2rust_should_drop_message(
        log_level,
        log_domain,
        ::core::ptr::null::<GLogField>(),
        0 as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_writer_default(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut user_data: gpointer,
) -> GLogWriterOutput {
    static mut safe_c2rust_initialized: gsize = 0 as gsize;
    static mut safe_c2rust_stderr_is_journal: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !fields.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fields != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if n_fields > 0 as gsize {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_fields > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_LOG_WRITER_UNHANDLED;
    }
    if safe_c2rust_should_drop_message(
        log_level,
        ::core::ptr::null::<::core::ffi::c_char>(),
        fields,
        n_fields,
    ) != 0
    {
        return G_LOG_WRITER_HANDLED;
    }
    if log_level as ::core::ffi::c_int & safe_c2rust_g_log_always_fatal as ::core::ffi::c_int != 0
        && safe_c2rust_log_is_old_api(fields, n_fields) == 0
    {
        log_level = ::core::mem::transmute::<::core::ffi::c_int, GLogLevelFlags>(
            log_level as ::core::ffi::c_int | G_LOG_FLAG_FATAL as ::core::ffi::c_int,
        );
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_stderr_is_journal =
            safe_c2rust_g_log_writer_is_journald(fileno(safe_c2rust_stderr) as gint);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
    if !(safe_c2rust_stderr_is_journal != 0
        && safe_c2rust_g_log_writer_journald(log_level, fields, n_fields, user_data)
            as ::core::ffi::c_uint
            == G_LOG_WRITER_HANDLED as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if !(safe_c2rust_g_log_writer_standard_streams(log_level, fields, n_fields, user_data)
            as ::core::ffi::c_uint
            == G_LOG_WRITER_HANDLED as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            return G_LOG_WRITER_UNHANDLED;
        }
    }
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_FATAL as ::core::ffi::c_int != 0 {
        safe_c2rust__g_log_abort(
            (log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int == 0)
                as ::core::ffi::c_int,
        );
    }
    return G_LOG_WRITER_HANDLED;
}
unsafe extern "C" fn safe_c2rust__g_log_writer_fallback(
    mut log_level: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut user_data: gpointer,
) -> GLogWriterOutput {
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: gsize = 0;
    stream = safe_c2rust_log_level_to_file(log_level);
    i = 0 as gsize;
    while i < n_fields {
        let mut field: *const GLogField = fields.offset(i as isize) as *const GLogField;
        if !(strcmp(
            (*field).key as *const ::core::ffi::c_char,
            b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"MESSAGE_ID\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"PRIORITY\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"CODE_FILE\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"CODE_LINE\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"CODE_FUNC\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"ERRNO\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"SYSLOG_FACILITY\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"SYSLOG_IDENTIFIER\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"SYSLOG_PID\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*field).key as *const ::core::ffi::c_char,
                b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            safe_c2rust_write_string(stream, (*field).key);
            safe_c2rust_write_string(stream, b"=\0" as *const u8 as *const gchar);
            safe_c2rust_write_string_sized(stream, (*field).value as *const gchar, (*field).length);
        }
        i = i.wrapping_add(1);
    }
    let mut pid_string: [gchar; 27] = [0; 27];
    safe_c2rust_format_unsigned(
        &raw mut pid_string as *mut gchar,
        getpid() as gulong,
        10 as guint,
    );
    safe_c2rust_write_string(stream, b"_PID=\0" as *const u8 as *const gchar);
    safe_c2rust_write_string(stream, &raw mut pid_string as *mut gchar);
    return G_LOG_WRITER_HANDLED;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_get_debug_enabled() -> gboolean {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_log_debug_enabled;
            safe_c2rust_g_log_debug_enabled;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut safe_c2rust_g_log_debug_enabled as *mut gint,
        );
        gaig_temp
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_set_debug_enabled(mut enabled: gboolean) {
    let mut gais_temp: gint = enabled;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_g_log_debug_enabled;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut safe_c2rust_g_log_debug_enabled as *mut gint,
        *&raw mut gais_temp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_return_if_fail_warning(
    mut log_domain: *const ::core::ffi::c_char,
    mut pretty_function: *const ::core::ffi::c_char,
    mut expression: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_log(
        log_domain as *const gchar,
        G_LOG_LEVEL_CRITICAL,
        b"%s: assertion '%s' failed\0" as *const u8 as *const gchar,
        pretty_function,
        expression,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_warn_message(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut warnexpr: *const ::core::ffi::c_char,
) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lstr: [::core::ffi::c_char; 32] = [0; 32];
    g_snprintf(
        &raw mut lstr as *mut gchar,
        32 as gulong,
        b"%d\0" as *const u8 as *const gchar,
        line,
    );
    if !warnexpr.is_null() {
        s = g_strconcat(
            b"(\0" as *const u8 as *const gchar,
            file,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut lstr as *mut ::core::ffi::c_char,
            b"):\0" as *const u8 as *const ::core::ffi::c_char,
            func,
            if *func.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
                b":\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            b" runtime check failed: (\0" as *const u8 as *const ::core::ffi::c_char,
            warnexpr,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_1,
        ) as *mut ::core::ffi::c_char;
    } else {
        s = g_strconcat(
            b"(\0" as *const u8 as *const gchar,
            file,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut lstr as *mut ::core::ffi::c_char,
            b"):\0" as *const u8 as *const ::core::ffi::c_char,
            func,
            if *func.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
                b":\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            b"code should not be reached\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_1,
        ) as *mut ::core::ffi::c_char;
    }
    safe_c2rust_g_log(
        domain as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s\0" as *const u8 as *const gchar,
        s,
    );
    g_free(s as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assert_warning(
    mut log_domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
    mut pretty_function: *const ::core::ffi::c_char,
    mut expression: *const ::core::ffi::c_char,
) -> ! {
    if !expression.is_null() {
        safe_c2rust_g_log(
            log_domain as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"file %s: line %d (%s): assertion failed: (%s)\0" as *const u8 as *const gchar,
            file,
            line,
            pretty_function,
            expression,
        );
    } else {
        safe_c2rust_g_log(
            log_domain as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            file,
            line,
            pretty_function,
        );
    }
    safe_c2rust__g_log_abort(FALSE);
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_expect_message(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut pattern: *const gchar,
) {
    let mut expected: *mut GTestExpectedMessage = ::core::ptr::null_mut::<GTestExpectedMessage>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if log_level as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"log_level != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
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
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pattern != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !(log_level as ::core::ffi::c_int) & G_LOG_LEVEL_ERROR as ::core::ffi::c_int != 0 {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"~log_level & G_LOG_LEVEL_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    expected = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GTestExpectedMessage>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GTestExpectedMessage;
    (*expected).log_domain =
        safe_c2rust_g_strdup_inline(log_domain as *const ::core::ffi::c_char) as *mut gchar;
    (*expected).log_level = log_level;
    (*expected).pattern =
        safe_c2rust_g_strdup_inline(pattern as *const ::core::ffi::c_char) as *mut gchar;
    safe_c2rust_expected_messages =
        g_slist_append(safe_c2rust_expected_messages, expected as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_assert_expected_messages_internal(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
) {
    if !safe_c2rust_expected_messages.is_null() {
        let mut expected: *mut GTestExpectedMessage =
            ::core::ptr::null_mut::<GTestExpectedMessage>();
        let mut level_prefix: [gchar; 59] = [0; 59];
        let mut message: *mut gchar = ::core::ptr::null_mut::<gchar>();
        expected = (*safe_c2rust_expected_messages).data as *mut GTestExpectedMessage;
        safe_c2rust_mklevel_prefix(
            &raw mut level_prefix as *mut gchar,
            (*expected).log_level,
            FALSE,
        );
        message = g_strdup_printf(
            b"Did not see expected message %s-%s: %s\0" as *const u8 as *const gchar,
            if !(*expected).log_domain.is_null() {
                (*expected).log_domain as *const gchar
            } else {
                b"**\0" as *const u8 as *const gchar
            },
            &raw mut level_prefix as *mut gchar,
            (*expected).pattern,
        );
        g_assertion_message(G_LOG_DOMAIN.as_ptr(), file, line, func, message);
        g_free(message as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_log_fallback_handler(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut message: *const gchar,
    mut unused_data: gpointer,
) {
    let mut level_prefix: [gchar; 59] = [0; 59];
    let mut pid_string: [gchar; 27] = [0; 27];
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    stream = safe_c2rust_mklevel_prefix(&raw mut level_prefix as *mut gchar, log_level, FALSE);
    if message.is_null() {
        message = b"(NULL) message\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    safe_c2rust_format_unsigned(
        &raw mut pid_string as *mut gchar,
        getpid() as gulong,
        10 as guint,
    );
    if !log_domain.is_null() {
        safe_c2rust_write_string(stream, b"\n\0" as *const u8 as *const gchar);
    } else {
        safe_c2rust_write_string(stream, b"\n** \0" as *const u8 as *const gchar);
    }
    safe_c2rust_write_string(stream, b"(process:\0" as *const u8 as *const gchar);
    safe_c2rust_write_string(stream, &raw mut pid_string as *mut gchar);
    safe_c2rust_write_string(stream, b"): \0" as *const u8 as *const gchar);
    if !log_domain.is_null() {
        safe_c2rust_write_string(stream, log_domain);
        safe_c2rust_write_string(stream, b"-\0" as *const u8 as *const gchar);
    }
    safe_c2rust_write_string(stream, &raw mut level_prefix as *mut gchar);
    safe_c2rust_write_string(stream, b": \0" as *const u8 as *const gchar);
    safe_c2rust_write_string(stream, message);
    safe_c2rust_write_string(stream, b"\n\0" as *const u8 as *const gchar);
}
unsafe extern "C" fn safe_c2rust_escape_string(mut string: *mut GString) {
    let mut p: *const ::core::ffi::c_char = (*string).str_0;
    let mut wc: gunichar = 0;
    while p < (*string).str_0.offset((*string).len as isize) as *const ::core::ffi::c_char {
        let mut safe: gboolean = 0;
        wc = g_utf8_get_char_validated(p as *const gchar, -(1 as ::core::ffi::c_int) as gssize);
        if wc == -(1 as ::core::ffi::c_int) as gunichar
            || wc == -(2 as ::core::ffi::c_int) as gunichar
        {
            let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut pos: guint = 0;
            pos = p.offset_from((*string).str_0) as ::core::ffi::c_long as guint;
            tmp = g_strdup_printf(
                b"\\x%02x\0" as *const u8 as *const gchar,
                *p as guchar as guint,
            );
            g_string_erase(string, pos as gssize, 1 as gssize);
            g_string_insert(string, pos as gssize, tmp);
            p = (*string)
                .str_0
                .offset(pos.wrapping_add(4 as guint) as isize);
            g_free(tmp as gpointer);
        } else {
            if wc == '\r' as i32 as gunichar {
                safe = (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32) as ::core::ffi::c_int as gboolean;
            } else {
                safe = !(wc < 0x20 as gunichar
                    && wc != '\t' as i32 as gunichar
                    && wc != '\n' as i32 as gunichar
                    && wc != '\r' as i32 as gunichar
                    || wc == 0x7f as gunichar
                    || wc >= 0x80 as gunichar && wc < 0xa0 as gunichar)
                    as ::core::ffi::c_int as gboolean;
            }
            if safe == 0 {
                let mut tmp_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut pos_0: guint = 0;
                pos_0 = p.offset_from((*string).str_0) as ::core::ffi::c_long as guint;
                tmp_0 = g_strdup_printf(b"\\u%04x\0" as *const u8 as *const gchar, wc);
                g_string_erase(
                    string,
                    pos_0 as gssize,
                    (p.offset(
                        *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as *mut ::core::ffi::c_char)
                        .offset_from(p) as gssize,
                );
                g_string_insert(string, pos_0 as gssize, tmp_0);
                g_free(tmp_0 as gpointer);
                p = (*string)
                    .str_0
                    .offset(pos_0.wrapping_add(6 as guint) as isize);
            } else {
                p = p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_log_default_handler(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut message: *const gchar,
    mut unused_data: gpointer,
) {
    let mut fields: [GLogField; 4] = [_GLogField {
        key: ::core::ptr::null::<gchar>(),
        value: ::core::ptr::null::<::core::ffi::c_void>(),
        length: 0,
    }; 4];
    let mut n_fields: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        safe_c2rust__g_log_fallback_handler(log_domain, log_level, message, unused_data);
        return;
    }
    fields[0 as ::core::ffi::c_int as usize].key =
        b"GLIB_OLD_LOG_API\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    fields[0 as ::core::ffi::c_int as usize].value =
        b"1\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer;
    fields[0 as ::core::ffi::c_int as usize].length = -(1 as ::core::ffi::c_int) as gssize;
    n_fields += 1;
    fields[1 as ::core::ffi::c_int as usize].key =
        b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    fields[1 as ::core::ffi::c_int as usize].value = message as gconstpointer;
    fields[1 as ::core::ffi::c_int as usize].length = -(1 as ::core::ffi::c_int) as gssize;
    n_fields += 1;
    fields[2 as ::core::ffi::c_int as usize].key =
        b"PRIORITY\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    fields[2 as ::core::ffi::c_int as usize].value =
        safe_c2rust_log_level_to_priority(log_level) as gconstpointer;
    fields[2 as ::core::ffi::c_int as usize].length = -(1 as ::core::ffi::c_int) as gssize;
    n_fields += 1;
    if !log_domain.is_null() {
        fields[3 as ::core::ffi::c_int as usize].key =
            b"GLIB_DOMAIN\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        fields[3 as ::core::ffi::c_int as usize].value = log_domain as gconstpointer;
        fields[3 as ::core::ffi::c_int as usize].length = -(1 as ::core::ffi::c_int) as gssize;
        n_fields += 1;
    }
    safe_c2rust_g_log_structured_array(
        (log_level as ::core::ffi::c_int & !(G_LOG_FLAG_FATAL as ::core::ffi::c_int))
            as GLogLevelFlags,
        &raw mut fields as *mut GLogField,
        n_fields as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_print_handler(mut func: GPrintFunc) -> GPrintFunc {
    return ::core::mem::transmute::<gpointer, GPrintFunc>(
        ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_glib_print_func;
            } else {
            };
            ::core::mem::transmute::<GPrintFunc, gpointer>(crate::translated::compat::atomic_xchg_seqcst(
                &raw mut safe_c2rust_glib_print_func,
                if func.is_some() {
                    func as Option<unsafe extern "C" fn(*const gchar) -> ()>
                } else {
                    Some(
                        safe_c2rust_g_default_print_func
                            as unsafe extern "C" fn(*const gchar) -> (),
                    )
                },
            ))
        }),
    );
}
unsafe extern "C" fn safe_c2rust_print_string(mut stream: *mut FILE, mut string: *const gchar) {
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    let mut ret: ::core::ffi::c_int = 0;
    if g_get_console_charset(&raw mut charset) != 0 {
        ret = fputs(string as *const ::core::ffi::c_char, stream);
    } else {
        let mut converted_string: *mut gchar = safe_c2rust_strdup_convert(string, charset);
        ret = fputs(converted_string, stream);
        g_free(converted_string as gpointer);
    }
    if ret == EOF {
        return;
    }
    fflush(stream);
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_format_string(
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
    mut out_allocated_string: *mut *mut ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if strchr(format, '%' as i32).is_null() {
        *out_allocated_string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return format;
    } else {
        *out_allocated_string =
            g_strdup_vprintf(format as *const gchar, args.clone()) as *mut ::core::ffi::c_char;
        return *out_allocated_string;
    };
}
unsafe extern "C" fn safe_c2rust_g_default_print_func(mut string: *const gchar) {
    safe_c2rust_print_string(safe_c2rust_stdout, string);
}
unsafe extern "C" fn safe_c2rust_g_default_printerr_func(mut string: *const gchar) {
    safe_c2rust_print_string(safe_c2rust_stderr, string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_print(mut format: *const gchar, mut args: ...) {
    let mut args_0: ::core::ffi::VaList;
    let mut string: *const gchar = ::core::ptr::null::<gchar>();
    let mut free_me: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut local_glib_print_func: GPrintFunc = None;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_0 = args.clone();
    string = safe_c2rust_format_string(
        format as *const ::core::ffi::c_char,
        args_0.clone(),
        &raw mut free_me,
    ) as *const gchar;
    local_glib_print_func = ({
        let mut gapg_temp_newval: GPrintFunc = None;
        let mut gapg_temp_atomic: *mut GPrintFunc = &raw mut safe_c2rust_glib_print_func;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as GPrintFunc;
    local_glib_print_func.expect("non-null function pointer")(string);
    g_free(free_me as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_printerr_handler(mut func: GPrintFunc) -> GPrintFunc {
    return ::core::mem::transmute::<gpointer, GPrintFunc>(
        ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_glib_printerr_func;
            } else {
            };
            ::core::mem::transmute::<GPrintFunc, gpointer>(crate::translated::compat::atomic_xchg_seqcst(
                &raw mut safe_c2rust_glib_printerr_func,
                if func.is_some() {
                    func as Option<unsafe extern "C" fn(*const gchar) -> ()>
                } else {
                    Some(
                        safe_c2rust_g_default_printerr_func
                            as unsafe extern "C" fn(*const gchar) -> (),
                    )
                },
            ))
        }),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_printerr(mut format: *const gchar, mut args: ...) {
    let mut args_0: ::core::ffi::VaList;
    let mut string: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut free_me: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut local_glib_printerr_func: GPrintFunc = None;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !format.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_0 = args.clone();
    string = safe_c2rust_format_string(
        format as *const ::core::ffi::c_char,
        args_0.clone(),
        &raw mut free_me,
    );
    local_glib_printerr_func = ({
        let mut gapg_temp_newval: GPrintFunc = None;
        let mut gapg_temp_atomic: *mut GPrintFunc = &raw mut safe_c2rust_glib_printerr_func;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as GPrintFunc;
    local_glib_printerr_func.expect("non-null function pointer")(string as *const gchar);
    g_free(free_me as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_printf_string_upper_bound(
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gsize {
    let mut c: gchar = 0;
    let mut count: ::core::ffi::c_int = vsnprintf(
        &raw mut c,
        1 as size_t,
        format as *const ::core::ffi::c_char,
        args.clone(),
    );
    if count < 0 as ::core::ffi::c_int {
        return 0 as gsize;
    }
    return (count + 1 as ::core::ffi::c_int) as gsize;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
