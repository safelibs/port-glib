extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_in6;
    pub type sockaddr_in;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type size_t = usize;
pub type socklen_t = __socklen_t;
pub type __socklen_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
unsafe extern "C" fn safe_c2rust_str_has_prefix(
    mut str: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (strncmp(str, prefix, strlen(prefix)) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_fd_is_journal(
    mut output_fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut addr: C2RustUnnamed = C2RustUnnamed {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut addr_len: socklen_t = 0;
    let mut err: ::core::ffi::c_int = 0;
    if output_fd < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed>() as size_t,
    );
    addr_len = ::core::mem::size_of::<C2RustUnnamed>() as socklen_t;
    err = getpeername(
        output_fd,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut addr.sa,
        },
        &raw mut addr_len,
    );
    if err == 0 as ::core::ffi::c_int && addr.storage.ss_family as ::core::ffi::c_int == AF_UNIX {
        return (safe_c2rust_str_has_prefix(
            &raw mut addr.un.sun_path as *mut ::core::ffi::c_char,
            b"/run/systemd/journal/\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
            || safe_c2rust_str_has_prefix(
                &raw mut addr.un.sun_path as *mut ::core::ffi::c_char,
                b"/run/systemd/journal.\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0) as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
