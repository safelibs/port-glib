extern "C" {
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_unix_open_pipe(fds: *mut gint, flags: gint, error: *mut *mut GError) -> gboolean;
    fn g_unix_set_fd_nonblocking(fd: gint, nonblock: gboolean, error: *mut *mut GError)
        -> gboolean;
    fn eventfd(__count: ::core::ffi::c_uint, __flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type gpointer = *mut ::core::ffi::c_void;
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
pub struct _GWakeup {
    pub fds: [gint; 2],
}
pub type GWakeup = _GWakeup;
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GQuark = guint32;
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
pub const EFD_NONBLOCK: C2RustUnnamed_0 = 2048;
pub const EFD_CLOEXEC: C2RustUnnamed_0 = 524288;
pub const G_IO_IN: C2RustUnnamed = 1;
pub type ssize_t = __ssize_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_IO_NVAL: C2RustUnnamed = 32;
pub const G_IO_HUP: C2RustUnnamed = 16;
pub const G_IO_ERR: C2RustUnnamed = 8;
pub const G_IO_PRI: C2RustUnnamed = 2;
pub const G_IO_OUT: C2RustUnnamed = 4;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const EFD_SEMAPHORE: C2RustUnnamed_0 = 1;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_wakeup_new() -> *mut GWakeup {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut wakeup: *mut GWakeup = ::core::ptr::null_mut::<GWakeup>();
    wakeup = g_slice_alloc(::core::mem::size_of::<GWakeup>() as gsize) as *mut GWakeup;
    (*wakeup).fds[0 as ::core::ffi::c_int as usize] = eventfd(
        0 as ::core::ffi::c_uint,
        EFD_CLOEXEC as ::core::ffi::c_int | EFD_NONBLOCK as ::core::ffi::c_int,
    ) as gint;
    if (*wakeup).fds[0 as ::core::ffi::c_int as usize] != -(1 as ::core::ffi::c_int) {
        (*wakeup).fds[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int) as gint;
        return wakeup;
    }
    if g_unix_open_pipe(
        &raw mut (*wakeup).fds as *mut gint,
        O_CLOEXEC | O_NONBLOCK,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Creating pipes for GWakeup: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        loop {}
    }
    if g_unix_set_fd_nonblocking(
        (*wakeup).fds[0 as ::core::ffi::c_int as usize],
        TRUE,
        &raw mut error,
    ) == 0
        || g_unix_set_fd_nonblocking(
            (*wakeup).fds[1 as ::core::ffi::c_int as usize],
            TRUE,
            &raw mut error,
        ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Set pipes non-blocking for GWakeup: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        loop {}
    }
    return wakeup;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_wakeup_get_pollfd(
    mut wakeup: *mut GWakeup,
    mut poll_fd: *mut GPollFD,
) {
    (*poll_fd).fd = (*wakeup).fds[0 as ::core::ffi::c_int as usize];
    (*poll_fd).events = G_IO_IN as ::core::ffi::c_int as gushort;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_wakeup_acknowledge(mut wakeup: *mut GWakeup) {
    let mut res: ::core::ffi::c_int = 0;
    if (*wakeup).fds[1 as ::core::ffi::c_int as usize] == -(1 as ::core::ffi::c_int) {
        let mut value: uint64_t = 0;
        loop {
            res = read(
                (*wakeup).fds[0 as ::core::ffi::c_int as usize],
                &raw mut value as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<uint64_t>() as size_t,
            ) as ::core::ffi::c_int;
            if !(({
                let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
                if res == -(1 as ::core::ffi::c_int)
                    && *__errno_location() == 4 as ::core::ffi::c_int
                {
                    _g_boolean_var_8 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_8 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_8
            }) as ::core::ffi::c_long
                != 0)
            {
                break;
            }
        }
    } else {
        let mut value_0: uint8_t = 0;
        loop {
            res = read(
                (*wakeup).fds[0 as ::core::ffi::c_int as usize],
                &raw mut value_0 as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<uint8_t>() as size_t,
            ) as ::core::ffi::c_int;
            if !(res as usize == ::core::mem::size_of::<uint8_t>() as usize
                || ({
                    let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
                    if res == -(1 as ::core::ffi::c_int)
                        && *__errno_location() == 4 as ::core::ffi::c_int
                    {
                        _g_boolean_var_9 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_9 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_9
                }) as ::core::ffi::c_long
                    != 0)
            {
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_wakeup_signal(mut wakeup: *mut GWakeup) {
    let mut res: ::core::ffi::c_int = 0;
    if (*wakeup).fds[1 as ::core::ffi::c_int as usize] == -(1 as ::core::ffi::c_int) {
        let mut one: uint64_t = 1 as uint64_t;
        loop {
            res = write(
                (*wakeup).fds[0 as ::core::ffi::c_int as usize],
                &raw mut one as *const ::core::ffi::c_void,
                ::core::mem::size_of::<uint64_t>() as size_t,
            ) as ::core::ffi::c_int;
            if !(({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if res == -(1 as ::core::ffi::c_int)
                    && *__errno_location() == 4 as ::core::ffi::c_int
                {
                    _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_10
            }) as ::core::ffi::c_long
                != 0)
            {
                break;
            }
        }
    } else {
        let mut one_0: uint8_t = 1 as uint8_t;
        loop {
            res = write(
                (*wakeup).fds[1 as ::core::ffi::c_int as usize],
                &raw mut one_0 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<uint8_t>() as size_t,
            ) as ::core::ffi::c_int;
            if !(({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if res == -(1 as ::core::ffi::c_int)
                    && *__errno_location() == 4 as ::core::ffi::c_int
                {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0)
            {
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_wakeup_free(mut wakeup: *mut GWakeup) {
    close((*wakeup).fds[0 as ::core::ffi::c_int as usize]);
    if (*wakeup).fds[1 as ::core::ffi::c_int as usize] != -(1 as ::core::ffi::c_int) {
        close((*wakeup).fds[1 as ::core::ffi::c_int as usize]);
    }
    g_slice_free1(
        ::core::mem::size_of::<GWakeup>() as gsize,
        wakeup as gpointer,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
