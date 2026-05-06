extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_io_module_extract_name(
    mut filename: *const ::core::ffi::c_char,
) -> *mut gchar {
    let mut bname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut dot: *const gchar = ::core::ptr::null::<gchar>();
    let mut prefix_len: gsize = 0;
    let mut len: gsize = 0;
    let mut i: gsize = 0;
    bname = g_path_get_basename(filename as *const gchar);
    i = 0 as gsize;
    while *bname.offset(i as isize) != 0 {
        if *bname.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
            *bname.offset(i as isize) = '_' as i32 as gchar;
        }
        i = i.wrapping_add(1);
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = bname;
            let __prefix: *const ::core::ffi::c_char =
                b"libgio\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_10
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(bname, b"libgio\0" as *const u8 as *const gchar)
    } != 0
    {
        prefix_len = 6 as gsize;
    } else if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = bname;
            let __prefix: *const ::core::ffi::c_char =
                b"lib\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(bname, b"lib\0" as *const u8 as *const gchar)
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = bname;
                let __prefix: *const ::core::ffi::c_char =
                    b"gio\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(bname, b"gio\0" as *const u8 as *const gchar)
        }) != 0
    {
        prefix_len = 3 as gsize;
    } else {
        prefix_len = 0 as gsize;
    }
    dot = strchr(bname, '.' as i32);
    if !dot.is_null() {
        len = (dot.offset_from(bname) as ::core::ffi::c_long as gsize).wrapping_sub(prefix_len);
    } else {
        len = strlen(bname.offset(prefix_len as isize)) as gsize;
    }
    name = g_strndup(bname.offset(prefix_len as isize), len);
    g_free(bname as gpointer);
    return name;
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
