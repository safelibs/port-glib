use ::c2rust_bitfields;
extern "C" {
    pub type _GIConv;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_add_poll(source: *mut GSource, fd: *mut GPollFD);
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_io_channel_init(channel: *mut GIOChannel);
    fn g_io_channel_ref(channel: *mut GIOChannel) -> *mut GIOChannel;
    fn g_io_channel_unref(channel: *mut GIOChannel);
    fn g_io_channel_get_buffer_condition(channel: *mut GIOChannel) -> GIOCondition;
    fn g_io_channel_error_quark() -> GQuark;
    fn g_io_channel_error_from_errno(en: gint) -> GIOChannelError;
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
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GIConv = *mut _GIConv;
pub type GFileError = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: GFileError = 24;
pub const G_FILE_ERROR_NOSYS: GFileError = 23;
pub const G_FILE_ERROR_PERM: GFileError = 22;
pub const G_FILE_ERROR_IO: GFileError = 21;
pub const G_FILE_ERROR_INTR: GFileError = 20;
pub const G_FILE_ERROR_AGAIN: GFileError = 19;
pub const G_FILE_ERROR_PIPE: GFileError = 18;
pub const G_FILE_ERROR_INVAL: GFileError = 17;
pub const G_FILE_ERROR_BADF: GFileError = 16;
pub const G_FILE_ERROR_NFILE: GFileError = 15;
pub const G_FILE_ERROR_MFILE: GFileError = 14;
pub const G_FILE_ERROR_NOMEM: GFileError = 13;
pub const G_FILE_ERROR_NOSPC: GFileError = 12;
pub const G_FILE_ERROR_LOOP: GFileError = 11;
pub const G_FILE_ERROR_FAULT: GFileError = 10;
pub const G_FILE_ERROR_TXTBSY: GFileError = 9;
pub const G_FILE_ERROR_ROFS: GFileError = 8;
pub const G_FILE_ERROR_NODEV: GFileError = 7;
pub const G_FILE_ERROR_NXIO: GFileError = 6;
pub const G_FILE_ERROR_NOTDIR: GFileError = 5;
pub const G_FILE_ERROR_NOENT: GFileError = 4;
pub const G_FILE_ERROR_NAMETOOLONG: GFileError = 3;
pub const G_FILE_ERROR_ACCES: GFileError = 2;
pub const G_FILE_ERROR_ISDIR: GFileError = 1;
pub const G_FILE_ERROR_EXIST: GFileError = 0;
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GIOChannel {
    pub ref_count: gint,
    pub funcs: *mut GIOFuncs,
    pub encoding: *mut gchar,
    pub read_cd: GIConv,
    pub write_cd: GIConv,
    pub line_term: *mut gchar,
    pub line_term_len: guint,
    pub buf_size: gsize,
    pub read_buf: *mut GString,
    pub encoded_read_buf: *mut GString,
    pub write_buf: *mut GString,
    pub partial_write_buf: [gchar; 6],
    #[bitfield(name = "use_buffer", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "do_encode", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "close_on_unref", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "is_readable", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "is_writeable", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "is_seekable", ty = "guint", bits = "5..=5")]
    pub use_buffer_do_encode_close_on_unref_is_readable_is_writeable_is_seekable: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub reserved1: gpointer,
    pub reserved2: gpointer,
}
pub type GIOFuncs = _GIOFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOFuncs {
    pub io_read: Option<
        unsafe extern "C" fn(
            *mut GIOChannel,
            *mut gchar,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GIOStatus,
    >,
    pub io_write: Option<
        unsafe extern "C" fn(
            *mut GIOChannel,
            *const gchar,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GIOStatus,
    >,
    pub io_seek: Option<
        unsafe extern "C" fn(*mut GIOChannel, gint64, GSeekType, *mut *mut GError) -> GIOStatus,
    >,
    pub io_close: Option<unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus>,
    pub io_create_watch:
        Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource>,
    pub io_free: Option<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
    pub io_set_flags:
        Option<unsafe extern "C" fn(*mut GIOChannel, GIOFlags, *mut *mut GError) -> GIOStatus>,
    pub io_get_flags: Option<unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags>,
}
pub type GIOChannel = _GIOChannel;
pub type GIOFlags = ::core::ffi::c_uint;
pub const G_IO_FLAG_SET_MASK: GIOFlags = 3;
pub const G_IO_FLAG_GET_MASK: GIOFlags = 31;
pub const G_IO_FLAG_MASK: GIOFlags = 31;
pub const G_IO_FLAG_IS_SEEKABLE: GIOFlags = 16;
pub const G_IO_FLAG_IS_WRITEABLE: GIOFlags = 8;
pub const G_IO_FLAG_IS_WRITABLE: GIOFlags = 8;
pub const G_IO_FLAG_IS_READABLE: GIOFlags = 4;
pub const G_IO_FLAG_NONBLOCK: GIOFlags = 2;
pub const G_IO_FLAG_APPEND: GIOFlags = 1;
pub const G_IO_FLAG_NONE: GIOFlags = 0;
pub type GIOStatus = ::core::ffi::c_uint;
pub const G_IO_STATUS_AGAIN: GIOStatus = 3;
pub const G_IO_STATUS_EOF: GIOStatus = 2;
pub const G_IO_STATUS_NORMAL: GIOStatus = 1;
pub const G_IO_STATUS_ERROR: GIOStatus = 0;
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
pub type GIOChannelError = ::core::ffi::c_uint;
pub const G_IO_CHANNEL_ERROR_FAILED: GIOChannelError = 8;
pub const G_IO_CHANNEL_ERROR_PIPE: GIOChannelError = 7;
pub const G_IO_CHANNEL_ERROR_OVERFLOW: GIOChannelError = 6;
pub const G_IO_CHANNEL_ERROR_NXIO: GIOChannelError = 5;
pub const G_IO_CHANNEL_ERROR_NOSPC: GIOChannelError = 4;
pub const G_IO_CHANNEL_ERROR_ISDIR: GIOChannelError = 3;
pub const G_IO_CHANNEL_ERROR_IO: GIOChannelError = 2;
pub const G_IO_CHANNEL_ERROR_INVAL: GIOChannelError = 1;
pub const G_IO_CHANNEL_ERROR_FBIG: GIOChannelError = 0;
pub type GIOFunc =
    Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>;
pub type GIOUnixChannel = _GIOUnixChannel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOUnixChannel {
    pub channel: GIOChannel,
    pub fd: gint,
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
pub type GIOUnixWatch = _GIOUnixWatch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOUnixWatch {
    pub source: GSource,
    pub pollfd: GPollFD,
    pub channel: *mut GIOChannel,
    pub condition: GIOCondition,
}
pub const MODE_PLUS: C2RustUnnamed = 8;
pub const MODE_A_PLUS: C2RustUnnamed = 12;
pub const MODE_W_PLUS: C2RustUnnamed = 10;
pub const MODE_R_PLUS: C2RustUnnamed = 9;
pub const MODE_A: C2RustUnnamed = 4;
pub const MODE_W: C2RustUnnamed = 2;
pub const MODE_R: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4;
pub const EAGAIN: ::core::ffi::c_int = 11;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_io_unix_get_flags\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IWGRP: ::core::ffi::c_int = S_IWUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IWOTH: ::core::ffi::c_int = S_IWGRP >> 3 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SSIZE_MAX: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[no_mangle]
pub static mut safe_c2rust_g_io_watch_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: Some(
            safe_c2rust_g_io_unix_prepare
                as unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
        ),
        check: Some(safe_c2rust_g_io_unix_check as unsafe extern "C" fn(*mut GSource) -> gboolean),
        dispatch: Some(
            safe_c2rust_g_io_unix_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: Some(safe_c2rust_g_io_unix_finalize as unsafe extern "C" fn(*mut GSource) -> ()),
        closure_callback: None,
        closure_marshal: None,
    }
};
static mut safe_c2rust_unix_channel_funcs: GIOFuncs = unsafe {
    _GIOFuncs {
        io_read: Some(
            safe_c2rust_g_io_unix_read
                as unsafe extern "C" fn(
                    *mut GIOChannel,
                    *mut gchar,
                    gsize,
                    *mut gsize,
                    *mut *mut GError,
                ) -> GIOStatus,
        ),
        io_write: Some(
            safe_c2rust_g_io_unix_write
                as unsafe extern "C" fn(
                    *mut GIOChannel,
                    *const gchar,
                    gsize,
                    *mut gsize,
                    *mut *mut GError,
                ) -> GIOStatus,
        ),
        io_seek: Some(
            safe_c2rust_g_io_unix_seek
                as unsafe extern "C" fn(
                    *mut GIOChannel,
                    gint64,
                    GSeekType,
                    *mut *mut GError,
                ) -> GIOStatus,
        ),
        io_close: Some(
            safe_c2rust_g_io_unix_close
                as unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus,
        ),
        io_create_watch: Some(
            safe_c2rust_g_io_unix_create_watch
                as unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource,
        ),
        io_free: Some(safe_c2rust_g_io_unix_free as unsafe extern "C" fn(*mut GIOChannel) -> ()),
        io_set_flags: Some(
            safe_c2rust_g_io_unix_set_flags
                as unsafe extern "C" fn(*mut GIOChannel, GIOFlags, *mut *mut GError) -> GIOStatus,
        ),
        io_get_flags: Some(
            safe_c2rust_g_io_unix_get_flags as unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags,
        ),
    }
};
unsafe extern "C" fn safe_c2rust_g_io_unix_prepare(
    mut source: *mut GSource,
    mut timeout: *mut gint,
) -> gboolean {
    let mut watch: *mut GIOUnixWatch = source as *mut GIOUnixWatch;
    let mut buffer_condition: GIOCondition = g_io_channel_get_buffer_condition((*watch).channel);
    return ((*watch).condition as ::core::ffi::c_uint & buffer_condition as ::core::ffi::c_uint
        == (*watch).condition as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_check(mut source: *mut GSource) -> gboolean {
    let mut watch: *mut GIOUnixWatch = source as *mut GIOUnixWatch;
    let mut buffer_condition: GIOCondition = g_io_channel_get_buffer_condition((*watch).channel);
    let mut poll_condition: GIOCondition = (*watch).pollfd.revents as GIOCondition;
    return ((poll_condition as ::core::ffi::c_uint | buffer_condition as ::core::ffi::c_uint)
        & (*watch).condition as ::core::ffi::c_uint) as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut func: GIOFunc = ::core::mem::transmute::<GSourceFunc, GIOFunc>(callback);
    let mut watch: *mut GIOUnixWatch = source as *mut GIOUnixWatch;
    let mut buffer_condition: GIOCondition = g_io_channel_get_buffer_condition((*watch).channel);
    if func.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"IO watch dispatched without callback. You must call g_source_connect().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*watch).channel,
        (((*watch).pollfd.revents as ::core::ffi::c_uint | buffer_condition as ::core::ffi::c_uint)
            & (*watch).condition as ::core::ffi::c_uint) as GIOCondition,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_io_unix_finalize(mut source: *mut GSource) {
    let mut watch: *mut GIOUnixWatch = source as *mut GIOUnixWatch;
    g_io_channel_unref((*watch).channel);
}
unsafe extern "C" fn safe_c2rust_g_io_unix_read(
    mut channel: *mut GIOChannel,
    mut buf: *mut gchar,
    mut count: gsize,
    mut bytes_read: *mut gsize,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    let mut result: gssize = 0;
    if count > SSIZE_MAX as gsize {
        count = SSIZE_MAX as gsize;
    }
    loop {
        result = read(
            (*unix_channel).fd as ::core::ffi::c_int,
            buf as *mut ::core::ffi::c_void,
            count as size_t,
        ) as gssize;
        if result < 0 as gssize {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            *bytes_read = 0 as gsize;
            match errsv {
                EINTR => {}
                EAGAIN => return G_IO_STATUS_AGAIN,
                _ => {
                    g_set_error_literal(
                        err,
                        g_io_channel_error_quark(),
                        g_io_channel_error_from_errno(errsv as gint) as gint,
                        g_strerror(errsv as gint),
                    );
                    return G_IO_STATUS_ERROR;
                }
            }
        } else {
            *bytes_read = result as gsize;
            return (if result > 0 as gssize {
                G_IO_STATUS_NORMAL as ::core::ffi::c_int
            } else {
                G_IO_STATUS_EOF as ::core::ffi::c_int
            }) as GIOStatus;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_io_unix_write(
    mut channel: *mut GIOChannel,
    mut buf: *const gchar,
    mut count: gsize,
    mut bytes_written: *mut gsize,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    let mut result: gssize = 0;
    loop {
        result = write(
            (*unix_channel).fd as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_void,
            count as size_t,
        ) as gssize;
        if result < 0 as gssize {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            *bytes_written = 0 as gsize;
            match errsv {
                EINTR => {}
                EAGAIN => return G_IO_STATUS_AGAIN,
                _ => {
                    g_set_error_literal(
                        err,
                        g_io_channel_error_quark(),
                        g_io_channel_error_from_errno(errsv as gint) as gint,
                        g_strerror(errsv as gint),
                    );
                    return G_IO_STATUS_ERROR;
                }
            }
        } else {
            *bytes_written = result as gsize;
            return G_IO_STATUS_NORMAL;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_io_unix_seek(
    mut channel: *mut GIOChannel,
    mut offset: gint64,
    mut type_0: GSeekType,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    let mut whence: ::core::ffi::c_int = 0;
    let mut tmp_offset: off_t = 0;
    let mut result: off_t = 0;
    match type_0 as ::core::ffi::c_uint {
        1 => {
            whence = SEEK_SET;
        }
        0 => {
            whence = SEEK_CUR;
        }
        2 => {
            whence = SEEK_END;
        }
        _ => {
            whence = -(1 as ::core::ffi::c_int);
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giounix.c\0" as *const u8 as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    tmp_offset = offset as off_t;
    if tmp_offset != offset {
        g_set_error_literal(
            err,
            g_io_channel_error_quark(),
            g_io_channel_error_from_errno(EINVAL) as gint,
            g_strerror(EINVAL),
        );
        return G_IO_STATUS_ERROR;
    }
    result = lseek(
        (*unix_channel).fd as ::core::ffi::c_int,
        tmp_offset as __off64_t,
        whence,
    ) as off_t;
    if result < 0 as off_t {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error_literal(
            err,
            g_io_channel_error_quark(),
            g_io_channel_error_from_errno(errsv as gint) as gint,
            g_strerror(errsv as gint),
        );
        return G_IO_STATUS_ERROR;
    }
    return G_IO_STATUS_NORMAL;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_close(
    mut channel: *mut GIOChannel,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    if close((*unix_channel).fd as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error_literal(
            err,
            g_io_channel_error_quark(),
            g_io_channel_error_from_errno(errsv as gint) as gint,
            g_strerror(errsv as gint),
        );
        return G_IO_STATUS_ERROR;
    }
    return G_IO_STATUS_NORMAL;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_free(mut channel: *mut GIOChannel) {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    g_free(unix_channel as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_io_unix_create_watch(
    mut channel: *mut GIOChannel,
    mut condition: GIOCondition,
) -> *mut GSource {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut watch: *mut GIOUnixWatch = ::core::ptr::null_mut::<GIOUnixWatch>();
    source = g_source_new(
        &raw mut safe_c2rust_g_io_watch_funcs,
        ::core::mem::size_of::<GIOUnixWatch>() as guint,
    );
    g_source_set_static_name(
        source,
        b"GIOChannel (Unix)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    watch = source as *mut GIOUnixWatch;
    (*watch).channel = channel;
    g_io_channel_ref(channel);
    (*watch).condition = condition;
    (*watch).pollfd.fd = (*unix_channel).fd;
    (*watch).pollfd.events = condition as gushort;
    g_source_add_poll(source, &raw mut (*watch).pollfd);
    return source;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_set_flags(
    mut channel: *mut GIOChannel,
    mut flags: GIOFlags,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut fcntl_flags: glong = 0;
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    fcntl_flags = 0 as glong;
    if flags as ::core::ffi::c_uint & G_IO_FLAG_APPEND as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        fcntl_flags |= O_APPEND as glong;
    }
    if flags as ::core::ffi::c_uint
        & G_IO_FLAG_NONBLOCK as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        fcntl_flags |= O_NONBLOCK as glong;
    }
    if fcntl(
        (*unix_channel).fd as ::core::ffi::c_int,
        F_SETFL,
        fcntl_flags,
    ) == -(1 as ::core::ffi::c_int)
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error_literal(
            err,
            g_io_channel_error_quark(),
            g_io_channel_error_from_errno(errsv as gint) as gint,
            g_strerror(errsv as gint),
        );
        return G_IO_STATUS_ERROR;
    }
    return G_IO_STATUS_NORMAL;
}
unsafe extern "C" fn safe_c2rust_g_io_unix_get_flags(mut channel: *mut GIOChannel) -> GIOFlags {
    let mut flags: GIOFlags = G_IO_FLAG_NONE;
    let mut fcntl_flags: glong = 0;
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    fcntl_flags = fcntl((*unix_channel).fd as ::core::ffi::c_int, F_GETFL) as glong;
    if fcntl_flags == -(1 as ::core::ffi::c_int) as glong {
        let mut err: ::core::ffi::c_int = *__errno_location();
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"../original/glib/giounix.c:414Error while getting flags for FD: %s (%d)\0"
                as *const u8 as *const gchar,
            g_strerror(err as gint),
            err,
        );
        return G_IO_FLAG_NONE;
    }
    if fcntl_flags & O_APPEND as glong != 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GIOFlags>(
            flags as ::core::ffi::c_uint
                | G_IO_FLAG_APPEND as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if fcntl_flags & O_NONBLOCK as glong != 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GIOFlags>(
            flags as ::core::ffi::c_uint
                | G_IO_FLAG_NONBLOCK as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    match fcntl_flags & (O_RDONLY | O_WRONLY | O_RDWR) as glong {
        0 => {
            (*channel).set_is_readable(TRUE as guint as guint);
            (*channel).set_is_writeable(FALSE as guint as guint);
        }
        1 => {
            (*channel).set_is_readable(FALSE as guint as guint);
            (*channel).set_is_writeable(TRUE as guint as guint);
        }
        2 => {
            (*channel).set_is_readable(TRUE as guint as guint);
            (*channel).set_is_writeable(TRUE as guint as guint);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giounix.c\0" as *const u8 as *const ::core::ffi::c_char,
                443 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_new_file(
    mut filename: *const gchar,
    mut mode: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GIOChannel {
    let mut fid: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut create_mode: mode_t = 0;
    let mut channel: *mut GIOChannel = ::core::ptr::null_mut::<GIOChannel>();
    let mut mode_num: C2RustUnnamed = 0 as C2RustUnnamed;
    let mut buffer: stat = stat {
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
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOChannel>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !mode.is_null() {
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
            b"mode != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOChannel>();
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOChannel>();
    }
    match *mode.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        114 => {
            mode_num = MODE_R;
        }
        119 => {
            mode_num = MODE_W;
        }
        97 => {
            mode_num = MODE_A;
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Invalid GIOFileMode %s.\0" as *const u8 as *const gchar,
                mode,
            );
            return ::core::ptr::null_mut::<GIOChannel>();
        }
    }
    let mut current_block_30: u64;
    match *mode.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        0 => {
            current_block_30 = 15897653523371991391;
        }
        43 => {
            if *mode.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
                mode_num = ::core::mem::transmute::<::core::ffi::c_uint, C2RustUnnamed>(
                    mode_num as ::core::ffi::c_uint
                        | MODE_PLUS as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                current_block_30 = 15897653523371991391;
            } else {
                current_block_30 = 8292779500306085187;
            }
        }
        _ => {
            current_block_30 = 8292779500306085187;
        }
    }
    match current_block_30 {
        15897653523371991391 => {}
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Invalid GIOFileMode %s.\0" as *const u8 as *const gchar,
                mode,
            );
            return ::core::ptr::null_mut::<GIOChannel>();
        }
    }
    match mode_num as ::core::ffi::c_uint {
        1 => {
            flags = O_RDONLY;
        }
        2 => {
            flags = O_WRONLY | O_TRUNC | O_CREAT;
        }
        4 => {
            flags = O_WRONLY | O_APPEND | O_CREAT;
        }
        9 => {
            flags = O_RDWR;
        }
        10 => {
            flags = O_RDWR | O_TRUNC | O_CREAT;
        }
        12 => {
            flags = O_RDWR | O_APPEND | O_CREAT;
        }
        8 | _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giounix.c\0" as *const u8 as *const ::core::ffi::c_char,
                526 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    create_mode = (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH) as mode_t;
    fid = open(
        filename as *const ::core::ffi::c_char,
        flags | O_CLOEXEC,
        create_mode,
    );
    if fid == -(1 as ::core::ffi::c_int) {
        let mut err: ::core::ffi::c_int = *__errno_location();
        g_set_error_literal(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(err as gint) as gint,
            g_strerror(err as gint),
        );
        return NULL as *mut GIOChannel;
    }
    if fstat(fid, &raw mut buffer) == -(1 as ::core::ffi::c_int) {
        let mut err_0: ::core::ffi::c_int = *__errno_location();
        close(fid);
        g_set_error_literal(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(err_0 as gint) as gint,
            g_strerror(err_0 as gint),
        );
        return NULL as *mut GIOChannel;
    }
    channel = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GIOUnixChannel>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GIOUnixChannel as *mut GIOChannel;
    (*channel).set_is_seekable(
        (buffer.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            || buffer.st_mode & __S_IFMT as __mode_t == 0o20000 as __mode_t
            || buffer.st_mode & __S_IFMT as __mode_t == 0o60000 as __mode_t)
            as ::core::ffi::c_int as guint as guint,
    );
    match mode_num as ::core::ffi::c_uint {
        1 => {
            (*channel).set_is_readable(TRUE as guint as guint);
            (*channel).set_is_writeable(FALSE as guint as guint);
        }
        2 | 4 => {
            (*channel).set_is_readable(FALSE as guint as guint);
            (*channel).set_is_writeable(TRUE as guint as guint);
        }
        9 | 10 | 12 => {
            (*channel).set_is_readable(TRUE as guint as guint);
            (*channel).set_is_writeable(TRUE as guint as guint);
        }
        8 | _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giounix.c\0" as *const u8 as *const ::core::ffi::c_char,
                576 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    g_io_channel_init(channel);
    (*channel).set_close_on_unref(TRUE as guint as guint);
    (*channel).funcs = &raw mut safe_c2rust_unix_channel_funcs;
    (*(channel as *mut GIOUnixChannel)).fd = fid as gint;
    return channel;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_unix_new(mut fd: gint) -> *mut GIOChannel {
    let mut buffer: stat = stat {
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
    let mut unix_channel: *mut GIOUnixChannel = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GIOUnixChannel>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GIOUnixChannel;
    let mut channel: *mut GIOChannel = unix_channel as *mut GIOChannel;
    g_io_channel_init(channel);
    (*channel).funcs = &raw mut safe_c2rust_unix_channel_funcs;
    (*unix_channel).fd = fd;
    if fstat((*unix_channel).fd as ::core::ffi::c_int, &raw mut buffer) == 0 as ::core::ffi::c_int {
        (*channel).set_is_seekable(
            (buffer.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                || buffer.st_mode & __S_IFMT as __mode_t == 0o20000 as __mode_t
                || buffer.st_mode & __S_IFMT as __mode_t == 0o60000 as __mode_t)
                as ::core::ffi::c_int as guint as guint,
        );
    } else {
        (*channel).set_is_seekable(FALSE as guint as guint);
    }
    safe_c2rust_g_io_unix_get_flags(channel);
    return channel;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_unix_get_fd(
    mut channel: *mut GIOChannel,
) -> gint {
    let mut unix_channel: *mut GIOUnixChannel = channel as *mut GIOUnixChannel;
    return (*unix_channel).fd;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
