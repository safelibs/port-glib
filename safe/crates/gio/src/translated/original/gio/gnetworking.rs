extern "C" {
    fn getservbyname_r(
        __name: *const ::core::ffi::c_char,
        __proto: *const ::core::ffi::c_char,
        __result_buf: *mut servent,
        __buf: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut ::core::ffi::c_char,
    pub s_aliases: *mut *mut ::core::ffi::c_char,
    pub s_port: ::core::ffi::c_int,
    pub s_proto: *mut ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_networking_init() {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_getservbyname_ntohs(
    mut name: *const ::core::ffi::c_char,
    mut proto: *const ::core::ffi::c_char,
    mut out_port: *mut guint16,
) -> gboolean {
    let mut result: *mut servent = ::core::ptr::null_mut::<servent>();
    let mut result_buf: servent = servent {
        s_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        s_aliases: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        s_port: 0,
        s_proto: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut r: ::core::ffi::c_int = 0;
    r = getservbyname_r(
        name,
        proto,
        &raw mut result_buf,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
        &raw mut result,
    );
    if r != 0 as ::core::ffi::c_int || result != &raw mut result_buf {
        result = ::core::ptr::null_mut::<servent>();
    }
    if result.is_null() {
        return FALSE;
    }
    *out_port = (((*result).s_port as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
        as guint16 as ::core::ffi::c_int
        | (((*result).s_port as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as guint16 as ::core::ffi::c_int) as guint16;
    return TRUE;
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
