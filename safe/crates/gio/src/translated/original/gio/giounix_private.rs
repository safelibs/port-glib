extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn epoll_create1(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn epoll_ctl(
        __epfd: ::core::ffi::c_int,
        __op: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __event: *mut epoll_event,
    ) -> ::core::ffi::c_int;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const EPOLL_CLOEXEC: C2RustUnnamed = 524288;
pub type EPOLL_EVENTS = ::core::ffi::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
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
pub const EPOLL_CTL_ADD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_fd_is_pollable(mut fd: ::core::ffi::c_int) -> gboolean {
    let mut efd: ::core::ffi::c_int = 0;
    let mut ev: epoll_event = epoll_event {
        events: 0 as uint32_t,
        data: epoll_data {
            ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    };
    let mut add_succeeded: gboolean = 0;
    efd = epoll_create1(EPOLL_CLOEXEC as ::core::ffi::c_int);
    if efd == -(1 as ::core::ffi::c_int) {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"epoll_create1 () failed: %s\0" as *const u8 as *const gchar,
            g_strerror(*__errno_location()),
        );
        loop {}
    }
    ev.events = EPOLLIN as ::core::ffi::c_int as uint32_t;
    add_succeeded = (epoll_ctl(efd, EPOLL_CTL_ADD, fd, &raw mut ev) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as gboolean;
    close(efd);
    return add_succeeded;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
