extern "C" {
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type gint = ::core::ffi::c_int;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_poll(
    mut fds: *mut GPollFD,
    mut nfds: guint,
    mut timeout: gint,
) -> gint {
    return poll(
        fds as *mut pollfd,
        nfds as nfds_t,
        timeout as ::core::ffi::c_int,
    ) as gint;
}
