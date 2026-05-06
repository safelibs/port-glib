extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    static safe_c2rust__gio_xdg_type_unknown: [::core::ffi::c_char; 0];
    static safe_c2rust__gio_xdg_type_textplain: [::core::ffi::c_char; 0];
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type xdg_unichar_t = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
static mut safe_c2rust__xdg_utf8_skip_data: [::core::ffi::c_char; 256] = [
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    3 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    4 as ::core::ffi::c_int as ::core::ffi::c_char,
    5 as ::core::ffi::c_int as ::core::ffi::c_char,
    5 as ::core::ffi::c_int as ::core::ffi::c_char,
    5 as ::core::ffi::c_int as ::core::ffi::c_char,
    5 as ::core::ffi::c_int as ::core::ffi::c_char,
    6 as ::core::ffi::c_int as ::core::ffi::c_char,
    6 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut safe_c2rust___gio_xdg_utf8_skip: *const ::core::ffi::c_char =
    unsafe { &raw const safe_c2rust__xdg_utf8_skip_data as *const ::core::ffi::c_char };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_utf8_to_ucs4(
    mut source: *const ::core::ffi::c_char,
) -> xdg_unichar_t {
    let mut ucs32: xdg_unichar_t = 0;
    if *source as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 {
        ucs32 = *source as xdg_unichar_t;
    } else {
        let mut bytelength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut result: xdg_unichar_t = 0;
        if *source as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int == 0 {
            ucs32 = *source as xdg_unichar_t;
        } else {
            if *source as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int == 0 {
                let fresh0 = source;
                source = source.offset(1);
                result =
                    (*fresh0 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength = 2 as ::core::ffi::c_int;
            } else if *source as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int == 0 {
                let fresh1 = source;
                source = source.offset(1);
                result =
                    (*fresh1 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength = 3 as ::core::ffi::c_int;
            } else if *source as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 {
                let fresh2 = source;
                source = source.offset(1);
                result =
                    (*fresh2 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength = 4 as ::core::ffi::c_int;
            } else if *source as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
                let fresh3 = source;
                source = source.offset(1);
                result =
                    (*fresh3 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength = 5 as ::core::ffi::c_int;
            } else if *source as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 {
                let fresh4 = source;
                source = source.offset(1);
                result =
                    (*fresh4 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength = 6 as ::core::ffi::c_int;
            } else {
                let fresh5 = source;
                source = source.offset(1);
                result = *fresh5 as xdg_unichar_t;
                bytelength = 1 as ::core::ffi::c_int;
            }
            bytelength -= 1;
            while bytelength > 0 as ::core::ffi::c_int {
                result <<= 6 as ::core::ffi::c_int;
                let fresh6 = source;
                source = source.offset(1);
                result |=
                    (*fresh6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as xdg_unichar_t;
                bytelength -= 1;
            }
            ucs32 = result;
        }
    }
    return ucs32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_ucs4_to_lower(
    mut source: xdg_unichar_t,
) -> xdg_unichar_t {
    if source & 0xff as xdg_unichar_t == source {
        return ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int =
                        source as ::core::ffi::c_uchar as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res =
                        safe_c2rust_tolower(source as ::core::ffi::c_uchar as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(source as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) as xdg_unichar_t;
    }
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_utf8_validate(
    mut source: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_get_base_name(
    mut file_name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut base_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if file_name.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    base_name = strrchr(file_name, '/' as i32);
    if base_name.is_null() {
        return file_name;
    } else {
        return base_name.offset(1 as ::core::ffi::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_convert_to_ucs4(
    mut source: *const ::core::ffi::c_char,
    mut len: *mut ::core::ffi::c_int,
) -> *mut xdg_unichar_t {
    let mut out: *mut xdg_unichar_t = ::core::ptr::null_mut::<xdg_unichar_t>();
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    out = malloc(
        (::core::mem::size_of::<xdg_unichar_t>() as size_t)
            .wrapping_mul(strlen(source).wrapping_add(1 as size_t)),
    ) as *mut xdg_unichar_t;
    p = source;
    i = 0 as ::core::ffi::c_int;
    while *p != 0 {
        let fresh7 = i;
        i = i + 1;
        *out.offset(fresh7 as isize) = safe_c2rust___gio_xdg_utf8_to_ucs4(p);
        p = p.offset(
            *safe_c2rust___gio_xdg_utf8_skip.offset(*(p as *mut ::core::ffi::c_uchar) as isize)
                as ::core::ffi::c_int as isize,
        ) as *mut ::core::ffi::c_char;
    }
    *out.offset(i as isize) = 0 as xdg_unichar_t;
    *len = i;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_reverse_ucs4(
    mut source: *mut xdg_unichar_t,
    mut len: ::core::ffi::c_int,
) {
    let mut c: xdg_unichar_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < len - i - 1 as ::core::ffi::c_int {
        c = *source.offset(i as isize);
        *source.offset(i as isize) = *source.offset((len - i - 1 as ::core::ffi::c_int) as isize);
        *source.offset((len - i - 1 as ::core::ffi::c_int) as isize) = c;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__xdg_binary_or_text_fallback(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *const ::core::ffi::c_char {
    let mut chardata: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: size_t = 0;
    chardata = data as *mut ::core::ffi::c_uchar;
    i = 0 as size_t;
    while i < 128 as size_t && i < len {
        if (*chardata.offset(i as isize) as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            && *chardata.offset(i as isize) as ::core::ffi::c_int != 9 as ::core::ffi::c_int
            && *chardata.offset(i as isize) as ::core::ffi::c_int != 10 as ::core::ffi::c_int
            && *chardata.offset(i as isize) as ::core::ffi::c_int != 13 as ::core::ffi::c_int
        {
            return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    return &raw const safe_c2rust__gio_xdg_type_textplain as *const ::core::ffi::c_char;
}
