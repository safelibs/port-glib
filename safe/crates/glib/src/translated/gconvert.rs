extern "C" {
    pub type _GIConv;
    fn iconv_close(__cd: iconv_t) -> ::core::ffi::c_int;
    fn iconv_open(
        __tocode: *const ::core::ffi::c_char,
        __fromcode: *const ::core::ffi::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut ::core::ffi::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut ::core::ffi::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_get_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_make_valid(str: *const gchar, len: gssize) -> *mut gchar;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_tolower(c: gchar) -> gchar;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
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
    fn _g_charset_get_aliases(
        canonical_name: *const ::core::ffi::c_char,
    ) -> *mut *const ::core::ffi::c_char;
    fn _g_get_time_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn _g_get_ctype_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn g_private_set_alloc0(key: *mut GPrivate, size: gsize) -> gpointer;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type iconv_t = *mut ::core::ffi::c_void;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_CONVERT_ERROR_EMBEDDED_NUL: C2RustUnnamed = 7;
pub const G_CONVERT_ERROR_NO_MEMORY: C2RustUnnamed = 6;
pub const G_CONVERT_ERROR_NOT_ABSOLUTE_PATH: C2RustUnnamed = 5;
pub const G_CONVERT_ERROR_BAD_URI: C2RustUnnamed = 4;
pub const G_CONVERT_ERROR_PARTIAL_INPUT: C2RustUnnamed = 3;
pub const G_CONVERT_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: C2RustUnnamed = 1;
pub const G_CONVERT_ERROR_NO_CONVERSION: C2RustUnnamed = 0;
pub type GIConv = *mut _GIConv;
pub type gunichar = guint32;
pub type ConvertCheckFlags = ::core::ffi::c_uint;
pub const CONVERT_CHECK_NO_NULS_IN_OUTPUT: ConvertCheckFlags = 2;
pub const CONVERT_CHECK_NO_NULS_IN_INPUT: ConvertCheckFlags = 1;
pub type GFilenameCharsetCache = _GFilenameCharsetCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilenameCharsetCache {
    pub is_utf8: gboolean,
    pub charset: *mut gchar,
    pub filename_charsets: *mut *mut gchar,
}
pub type GPrivate = _GPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub const G_ASCII_ALPHA: C2RustUnnamed_0 = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed_0 = 1;
pub type UnsafeCharacterSet = ::core::ffi::c_uint;
pub const UNSAFE_SLASHES: UnsafeCharacterSet = 32;
pub const UNSAFE_HOST: UnsafeCharacterSet = 16;
pub const UNSAFE_PATH: UnsafeCharacterSet = 8;
pub const UNSAFE_ALLOW_PLUS: UnsafeCharacterSet = 2;
pub const UNSAFE_ALL: UnsafeCharacterSet = 1;
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub const G_ASCII_SPACE: C2RustUnnamed_0 = 256;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed_0 = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed_0 = 512;
pub const G_ASCII_PUNCT: C2RustUnnamed_0 = 128;
pub const G_ASCII_PRINT: C2RustUnnamed_0 = 64;
pub const G_ASCII_LOWER: C2RustUnnamed_0 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_0 = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed_0 = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed_0 = 4;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const EILSEQ: ::core::ffi::c_int = 84;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const E2BIG: ::core::ffi::c_int = 7;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const NUL_TERMINATOR_LENGTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_convert_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g_convert_error\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
unsafe extern "C" fn safe_c2rust_try_conversion(
    mut to_codeset: *const ::core::ffi::c_char,
    mut from_codeset: *const ::core::ffi::c_char,
    mut cd: *mut iconv_t,
) -> gboolean {
    *cd = iconv_open(to_codeset, from_codeset);
    if *cd == -(1 as ::core::ffi::c_int) as iconv_t && *__errno_location() == EINVAL {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_try_to_aliases(
    mut to_aliases: *mut *const ::core::ffi::c_char,
    mut from_codeset: *const ::core::ffi::c_char,
    mut cd: *mut iconv_t,
) -> gboolean {
    if !to_aliases.is_null() {
        let mut p: *mut *const ::core::ffi::c_char = to_aliases;
        while !(*p).is_null() {
            if safe_c2rust_try_conversion(*p, from_codeset, cd) != 0 {
                return TRUE;
            }
            p = p.offset(1);
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_iconv_open(
    mut to_codeset: *const gchar,
    mut from_codeset: *const gchar,
) -> GIConv {
    let mut current_block: u64;
    let mut cd: iconv_t = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if safe_c2rust_try_conversion(
        to_codeset as *const ::core::ffi::c_char,
        from_codeset as *const ::core::ffi::c_char,
        &raw mut cd,
    ) == 0
    {
        let mut to_aliases: *mut *const ::core::ffi::c_char =
            _g_charset_get_aliases(to_codeset as *const ::core::ffi::c_char);
        let mut from_aliases: *mut *const ::core::ffi::c_char =
            _g_charset_get_aliases(from_codeset as *const ::core::ffi::c_char);
        if !from_aliases.is_null() {
            let mut p: *mut *const ::core::ffi::c_char = from_aliases;
            loop {
                if (*p).is_null() {
                    current_block = 7815301370352969686;
                    break;
                }
                if safe_c2rust_try_conversion(
                    to_codeset as *const ::core::ffi::c_char,
                    *p,
                    &raw mut cd,
                ) != 0
                {
                    current_block = 4559881223852172572;
                    break;
                }
                if safe_c2rust_try_to_aliases(to_aliases, *p, &raw mut cd) != 0 {
                    current_block = 4559881223852172572;
                    break;
                }
                p = p.offset(1);
            }
        } else {
            current_block = 7815301370352969686;
        }
        match current_block {
            4559881223852172572 => {}
            _ => {
                safe_c2rust_try_to_aliases(
                    to_aliases,
                    from_codeset as *const ::core::ffi::c_char,
                    &raw mut cd,
                ) != 0;
            }
        }
    }
    return if cd == -(1 as ::core::ffi::c_int) as iconv_t {
        -(1 as ::core::ffi::c_int) as GIConv
    } else {
        cd as GIConv
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_iconv(
    mut converter: GIConv,
    mut inbuf: *mut *mut gchar,
    mut inbytes_left: *mut gsize,
    mut outbuf: *mut *mut gchar,
    mut outbytes_left: *mut gsize,
) -> gsize {
    let mut cd: iconv_t = converter as iconv_t;
    return iconv(
        cd,
        inbuf as *mut *mut ::core::ffi::c_char,
        inbytes_left as *mut size_t,
        outbuf as *mut *mut ::core::ffi::c_char,
        outbytes_left as *mut size_t,
    ) as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_iconv_close(mut converter: GIConv) -> gint {
    let mut cd: iconv_t = converter as iconv_t;
    return iconv_close(cd) as gint;
}
unsafe extern "C" fn safe_c2rust_open_converter(
    mut to_codeset: *const gchar,
    mut from_codeset: *const gchar,
    mut error: *mut *mut GError,
) -> GIConv {
    let mut cd: GIConv = ::core::ptr::null_mut::<_GIConv>();
    cd = safe_c2rust_g_iconv_open(to_codeset, from_codeset);
    if cd == -(1 as ::core::ffi::c_int) as GIConv {
        if !error.is_null() {
            if *__errno_location() == EINVAL {
                g_set_error(
                    error,
                    safe_c2rust_g_convert_error_quark(),
                    G_CONVERT_ERROR_NO_CONVERSION as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Conversion from character set \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D is not supported\0"
                            as *const u8 as *const gchar,
                    ),
                    from_codeset,
                    to_codeset,
                );
            } else {
                g_set_error(
                    error,
                    safe_c2rust_g_convert_error_quark(),
                    G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Could not open converter from \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    from_codeset,
                    to_codeset,
                );
            }
        }
    }
    return cd;
}
unsafe extern "C" fn safe_c2rust_close_converter(mut cd: GIConv) -> ::core::ffi::c_int {
    if cd == -(1 as ::core::ffi::c_int) as GIConv {
        return 0 as ::core::ffi::c_int;
    }
    return safe_c2rust_g_iconv_close(cd) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_convert_with_iconv(
    mut str: *const gchar,
    mut len: gssize,
    mut converter: GIConv,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut outp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut inbytes_remaining: gsize = 0;
    let mut outbytes_remaining: gsize = 0;
    let mut err: gsize = 0;
    let mut outbuf_size: gsize = 0;
    let mut have_error: gboolean = FALSE;
    let mut done: gboolean = FALSE;
    let mut reset: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if converter != -(1 as ::core::ffi::c_int) as GIConv {
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
            b"converter != (GIConv) -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        len = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    p = str;
    inbytes_remaining = len as gsize;
    outbuf_size = (len + NUL_TERMINATOR_LENGTH as gssize) as gsize;
    outbytes_remaining = outbuf_size.wrapping_sub(NUL_TERMINATOR_LENGTH as gsize);
    dest = g_malloc(outbuf_size) as *mut gchar;
    outp = dest;
    while done == 0 && have_error == 0 {
        if reset != 0 {
            err = safe_c2rust_g_iconv(
                converter,
                ::core::ptr::null_mut::<*mut gchar>(),
                &raw mut inbytes_remaining,
                &raw mut outp,
                &raw mut outbytes_remaining,
            );
        } else {
            err = safe_c2rust_g_iconv(
                converter,
                &raw mut p as *mut *mut gchar,
                &raw mut inbytes_remaining,
                &raw mut outp,
                &raw mut outbytes_remaining,
            );
        }
        if err == -(1 as ::core::ffi::c_int) as gsize {
            match *__errno_location() {
                EINVAL => {
                    done = TRUE as gboolean;
                }
                E2BIG => {
                    let mut used: gsize = outp.offset_from(dest) as ::core::ffi::c_long as gsize;
                    outbuf_size = outbuf_size.wrapping_mul(2 as gsize);
                    dest = g_realloc(dest as gpointer, outbuf_size) as *mut gchar;
                    outp = dest.offset(used as isize);
                    outbytes_remaining = outbuf_size
                        .wrapping_sub(used)
                        .wrapping_sub(NUL_TERMINATOR_LENGTH as gsize);
                }
                EILSEQ => {
                    g_set_error_literal(
                        error,
                        safe_c2rust_g_convert_error_quark(),
                        G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Invalid byte sequence in conversion input\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    have_error = TRUE as gboolean;
                }
                _ => {
                    let mut errsv: ::core::ffi::c_int = *__errno_location();
                    g_set_error(
                        error,
                        safe_c2rust_g_convert_error_quark(),
                        G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(b"Error during conversion: %s\0" as *const u8 as *const gchar),
                        g_strerror(errsv as gint),
                    );
                    have_error = TRUE as gboolean;
                }
            }
        } else if err > 0 as gsize {
            g_set_error_literal(
                error,
                safe_c2rust_g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unrepresentable character in conversion input\0" as *const u8 as *const gchar,
                ),
            );
            have_error = TRUE as gboolean;
        } else if reset == 0 {
            reset = TRUE as gboolean;
            inbytes_remaining = 0 as gsize;
        } else {
            done = TRUE as gboolean;
        }
    }
    memset(
        outp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        NUL_TERMINATOR_LENGTH as size_t,
    );
    if !bytes_read.is_null() {
        *bytes_read = p.offset_from(str) as ::core::ffi::c_long as gsize;
    } else if p.offset_from(str) as ::core::ffi::c_long != len {
        if have_error == 0 {
            g_set_error_literal(
                error,
                safe_c2rust_g_convert_error_quark(),
                G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Partial character sequence at end of input\0" as *const u8 as *const gchar,
                ),
            );
            have_error = TRUE as gboolean;
        }
    }
    if !bytes_written.is_null() {
        *bytes_written = outp.offset_from(dest) as ::core::ffi::c_long as gsize;
    }
    if have_error != 0 {
        g_free(dest as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    } else {
        return dest;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_convert(
    mut str: *const gchar,
    mut len: gssize,
    mut to_codeset: *const gchar,
    mut from_codeset: *const gchar,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut res: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cd: GIConv = ::core::ptr::null_mut::<_GIConv>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !to_codeset.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"to_codeset != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !from_codeset.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"from_codeset != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    cd = safe_c2rust_open_converter(to_codeset, from_codeset, error);
    if cd == -(1 as ::core::ffi::c_int) as GIConv {
        if !bytes_read.is_null() {
            *bytes_read = 0 as gsize;
        }
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    res = safe_c2rust_g_convert_with_iconv(str, len, cd, bytes_read, bytes_written, error);
    safe_c2rust_close_converter(cd);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_convert_with_fallback(
    mut str: *const gchar,
    mut len: gssize,
    mut to_codeset: *const gchar,
    mut from_codeset: *const gchar,
    mut fallback: *const gchar,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut utf8: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut outp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut insert_str: *const gchar = ::core::ptr::null::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut inbytes_remaining: gsize = 0;
    let mut save_p: *const gchar = ::core::ptr::null::<gchar>();
    let mut save_inbytes: gsize = 0 as gsize;
    let mut outbytes_remaining: gsize = 0;
    let mut err: gsize = 0;
    let mut cd: GIConv = ::core::ptr::null_mut::<_GIConv>();
    let mut outbuf_size: gsize = 0;
    let mut have_error: gboolean = FALSE;
    let mut done: gboolean = FALSE;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !to_codeset.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"to_codeset != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !from_codeset.is_null() {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"from_codeset != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        len = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    dest = safe_c2rust_g_convert(
        str,
        len,
        to_codeset,
        from_codeset,
        bytes_read,
        bytes_written,
        &raw mut local_error,
    );
    if local_error.is_null() {
        return dest;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if dest.is_null() {
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
            b"../original/glib/gconvert.c\0" as *const u8 as *const ::core::ffi::c_char,
            595 as ::core::ffi::c_int,
            G_STRFUNC,
            b"dest == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_error_matches(
        local_error,
        safe_c2rust_g_convert_error_quark(),
        G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
    ) == 0
    {
        g_propagate_error(error, local_error);
        return ::core::ptr::null_mut::<gchar>();
    } else {
        g_error_free(local_error);
    }
    local_error = ::core::ptr::null_mut::<GError>();
    cd = safe_c2rust_open_converter(to_codeset, b"UTF-8\0" as *const u8 as *const gchar, error);
    if cd == -(1 as ::core::ffi::c_int) as GIConv {
        if !bytes_read.is_null() {
            *bytes_read = 0 as gsize;
        }
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    utf8 = safe_c2rust_g_convert(
        str,
        len,
        b"UTF-8\0" as *const u8 as *const gchar,
        from_codeset,
        bytes_read,
        &raw mut inbytes_remaining,
        error,
    );
    if utf8.is_null() {
        safe_c2rust_close_converter(cd);
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    p = utf8;
    outbuf_size = (len + NUL_TERMINATOR_LENGTH as gssize) as gsize;
    outbytes_remaining = outbuf_size.wrapping_sub(NUL_TERMINATOR_LENGTH as gsize);
    dest = g_malloc(outbuf_size) as *mut gchar;
    outp = dest;
    while done == 0 && have_error == 0 {
        let mut inbytes_tmp: gsize = inbytes_remaining;
        err = safe_c2rust_g_iconv(
            cd,
            &raw mut p as *mut *mut gchar,
            &raw mut inbytes_tmp,
            &raw mut outp,
            &raw mut outbytes_remaining,
        );
        inbytes_remaining = inbytes_tmp;
        if err == -(1 as ::core::ffi::c_int) as gsize {
            let mut current_block_78: u64;
            match *__errno_location() {
                EINVAL => {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gconvert.c\0" as *const u8 as *const ::core::ffi::c_char,
                        657 as ::core::ffi::c_int,
                        G_STRFUNC,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                E2BIG => {
                    let mut used: gsize = outp.offset_from(dest) as ::core::ffi::c_long as gsize;
                    outbuf_size = outbuf_size.wrapping_mul(2 as gsize);
                    dest = g_realloc(dest as gpointer, outbuf_size) as *mut gchar;
                    outp = dest.offset(used as isize);
                    outbytes_remaining = outbuf_size
                        .wrapping_sub(used)
                        .wrapping_sub(NUL_TERMINATOR_LENGTH as gsize);
                    current_block_78 = 12961834331865314435;
                }
                EILSEQ => {
                    if !save_p.is_null() {
                        g_set_error(
                            error,
                            safe_c2rust_g_convert_error_quark(),
                            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int
                                as gint,
                            glib_gettext(
                                b"Cannot convert fallback \xE2\x80\x9C%s\xE2\x80\x9D to codeset \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                    as *const u8 as *const gchar,
                            ),
                            insert_str,
                            to_codeset,
                        );
                        have_error = TRUE as gboolean;
                        current_block_78 = 12961834331865314435;
                    } else if !p.is_null() {
                        if fallback.is_null() {
                            let mut ch: gunichar = g_utf8_get_char(p);
                            insert_str = g_strdup_printf(
                                if ch < 0x10000 as ::core::ffi::c_int as gunichar {
                                    b"\\u%04x\0" as *const u8 as *const gchar
                                } else {
                                    b"\\U%08x\0" as *const u8 as *const gchar
                                },
                                ch,
                            );
                        } else {
                            insert_str = fallback;
                        }
                        save_p = p.offset(
                            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                                as ::core::ffi::c_int as isize,
                        ) as *mut ::core::ffi::c_char;
                        save_inbytes = inbytes_remaining
                            .wrapping_sub(save_p.offset_from(p) as ::core::ffi::c_long as gsize);
                        p = insert_str;
                        inbytes_remaining = strlen(p as *const ::core::ffi::c_char) as gsize;
                        current_block_78 = 12961834331865314435;
                    } else {
                        current_block_78 = 3599108737183479273;
                    }
                }
                _ => {
                    current_block_78 = 3599108737183479273;
                }
            }
            match current_block_78 {
                3599108737183479273 => {
                    let mut errsv: ::core::ffi::c_int = *__errno_location();
                    g_set_error(
                        error,
                        safe_c2rust_g_convert_error_quark(),
                        G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(b"Error during conversion: %s\0" as *const u8 as *const gchar),
                        g_strerror(errsv as gint),
                    );
                    have_error = TRUE as gboolean;
                }
                _ => {}
            }
        } else if !save_p.is_null() {
            if fallback.is_null() {
                g_free(insert_str as *mut gchar as gpointer);
            }
            p = save_p;
            inbytes_remaining = save_inbytes;
            save_p = ::core::ptr::null::<gchar>();
        } else if !p.is_null() {
            p = ::core::ptr::null::<gchar>();
            inbytes_remaining = 0 as gsize;
        } else {
            done = TRUE as gboolean;
        }
    }
    memset(
        outp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        NUL_TERMINATOR_LENGTH as size_t,
    );
    safe_c2rust_close_converter(cd);
    if !bytes_written.is_null() {
        *bytes_written = outp.offset_from(dest) as ::core::ffi::c_long as gsize;
    }
    g_free(utf8 as gpointer);
    if have_error != 0 {
        if !save_p.is_null() && fallback.is_null() {
            g_free(insert_str as *mut gchar as gpointer);
        }
        g_free(dest as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    } else {
        return dest;
    };
}
unsafe extern "C" fn safe_c2rust_strdup_len(
    mut string: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut real_len: gsize = 0;
    let mut end_valid: *const gchar = ::core::ptr::null::<gchar>();
    if g_utf8_validate(string, len, &raw mut end_valid) == 0 {
        if !bytes_read.is_null() {
            *bytes_read = end_valid.offset_from(string) as ::core::ffi::c_long as gsize;
        }
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        g_set_error_literal(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid byte sequence in conversion input\0" as *const u8 as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    real_len = end_valid.offset_from(string) as ::core::ffi::c_long as gsize;
    if !bytes_read.is_null() {
        *bytes_read = real_len;
    }
    if !bytes_written.is_null() {
        *bytes_written = real_len;
    }
    return g_strndup(string, real_len);
}
unsafe extern "C" fn safe_c2rust_convert_checked(
    mut string: *const gchar,
    mut len: gssize,
    mut to_codeset: *const gchar,
    mut from_codeset: *const gchar,
    mut flags: ConvertCheckFlags,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut outbytes: gsize = 0;
    if flags as ::core::ffi::c_uint
        & CONVERT_CHECK_NO_NULS_IN_INPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && len > 0 as gssize
    {
        let mut early_nul: *const gchar = memchr(
            string as *const ::core::ffi::c_void,
            '\0' as i32,
            len as size_t,
        ) as *const gchar;
        if !early_nul.is_null() {
            if !bytes_read.is_null() {
                *bytes_read = early_nul.offset_from(string) as ::core::ffi::c_long as gsize;
            }
            if !bytes_written.is_null() {
                *bytes_written = 0 as gsize;
            }
            g_set_error_literal(
                error,
                safe_c2rust_g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Embedded NUL byte in conversion input\0" as *const u8 as *const gchar,
                ),
            );
            return ::core::ptr::null_mut::<gchar>();
        }
    }
    out = safe_c2rust_g_convert(
        string,
        len,
        to_codeset,
        from_codeset,
        bytes_read,
        &raw mut outbytes,
        error,
    );
    if out.is_null() {
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    if flags as ::core::ffi::c_uint
        & CONVERT_CHECK_NO_NULS_IN_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !memchr(
            out as *const ::core::ffi::c_void,
            '\0' as i32,
            outbytes as size_t,
        )
        .is_null()
    {
        g_free(out as gpointer);
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        g_set_error_literal(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_EMBEDDED_NUL as ::core::ffi::c_int as gint,
            glib_gettext(b"Embedded NUL byte in conversion output\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if !bytes_written.is_null() {
        *bytes_written = outbytes;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_locale_to_utf8(
    mut opsysstring: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if g_get_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(opsysstring, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            opsysstring,
            len,
            b"UTF-8\0" as *const u8 as *const gchar,
            charset as *const gchar,
            CONVERT_CHECK_NO_NULS_IN_OUTPUT,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_time_locale_to_utf8(
    mut opsysstring: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if _g_get_time_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(opsysstring, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            opsysstring,
            len,
            b"UTF-8\0" as *const u8 as *const gchar,
            charset as *const gchar,
            CONVERT_CHECK_NO_NULS_IN_OUTPUT,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_ctype_locale_to_utf8(
    mut opsysstring: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if _g_get_ctype_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(opsysstring, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            opsysstring,
            len,
            b"UTF-8\0" as *const u8 as *const gchar,
            charset as *const gchar,
            CONVERT_CHECK_NO_NULS_IN_OUTPUT,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_locale_from_utf8(
    mut utf8string: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    if g_get_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(utf8string, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            utf8string,
            len,
            charset,
            b"UTF-8\0" as *const u8 as *const gchar,
            CONVERT_CHECK_NO_NULS_IN_INPUT,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
unsafe extern "C" fn safe_c2rust_filename_charset_cache_free(mut data: gpointer) {
    let mut cache: *mut GFilenameCharsetCache = data as *mut GFilenameCharsetCache;
    g_free((*cache).charset as gpointer);
    g_strfreev((*cache).filename_charsets);
    g_free(cache as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_filename_charsets(
    mut filename_charsets: *mut *mut *const gchar,
) -> gboolean {
    static mut safe_c2rust_cache_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL,
            notify: Some(
                safe_c2rust_filename_charset_cache_free as unsafe extern "C" fn(gpointer) -> (),
            ),
            future: [NULL, NULL],
        }
    };
    let mut cache: *mut GFilenameCharsetCache =
        g_private_get(&raw mut safe_c2rust_cache_private) as *mut GFilenameCharsetCache;
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    if cache.is_null() {
        cache = g_private_set_alloc0(
            &raw mut safe_c2rust_cache_private,
            ::core::mem::size_of::<GFilenameCharsetCache>() as gsize,
        ) as *mut GFilenameCharsetCache;
    }
    g_get_charset(&raw mut charset);
    if !(!(*cache).charset.is_null()
        && strcmp((*cache).charset, charset as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int)
    {
        let mut new_charset: *const gchar = ::core::ptr::null::<gchar>();
        let mut p: *const gchar = ::core::ptr::null::<gchar>();
        let mut i: gint = 0;
        g_free((*cache).charset as gpointer);
        g_strfreev((*cache).filename_charsets);
        (*cache).charset =
            safe_c2rust_g_strdup_inline(charset as *const ::core::ffi::c_char) as *mut gchar;
        p = g_getenv(b"G_FILENAME_ENCODING\0" as *const u8 as *const gchar);
        if !p.is_null()
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
            (*cache).filename_charsets =
                g_strsplit(p, b",\0" as *const u8 as *const gchar, 0 as gint);
            (*cache).is_utf8 = (strcmp(
                *(*cache)
                    .filename_charsets
                    .offset(0 as ::core::ffi::c_int as isize),
                b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as gboolean;
            i = 0 as ::core::ffi::c_int as gint;
            while !(*(*cache).filename_charsets.offset(i as isize)).is_null() {
                if strcmp(
                    b"@locale\0" as *const u8 as *const ::core::ffi::c_char,
                    *(*cache).filename_charsets.offset(i as isize),
                ) == 0 as ::core::ffi::c_int
                {
                    g_get_charset(&raw mut new_charset);
                    g_free(*(*cache).filename_charsets.offset(i as isize) as gpointer);
                    let ref mut fresh0 = *(*cache).filename_charsets.offset(i as isize);
                    *fresh0 = safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char)
                        as *mut gchar;
                }
                i += 1;
            }
        } else if !g_getenv(b"G_BROKEN_FILENAMES\0" as *const u8 as *const gchar).is_null() {
            (*cache).filename_charsets = ({
                let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut *mut gchar;
            (*cache).is_utf8 = g_get_charset(&raw mut new_charset);
            let ref mut fresh1 = *(*cache)
                .filename_charsets
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh1 = safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char)
                as *mut gchar;
        } else {
            (*cache).filename_charsets = ({
                let mut __n: gsize = 3 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut *mut gchar;
            (*cache).is_utf8 = TRUE as gboolean;
            let ref mut fresh2 = *(*cache)
                .filename_charsets
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh2 =
                safe_c2rust_g_strdup_inline(b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char)
                    as *mut gchar;
            if g_get_charset(&raw mut new_charset) == 0 {
                let ref mut fresh3 = *(*cache)
                    .filename_charsets
                    .offset(1 as ::core::ffi::c_int as isize);
                *fresh3 = safe_c2rust_g_strdup_inline(new_charset as *const ::core::ffi::c_char)
                    as *mut gchar;
            }
        }
    }
    if !filename_charsets.is_null() {
        *filename_charsets = (*cache).filename_charsets as *mut *const gchar;
    }
    return (*cache).is_utf8;
}
unsafe extern "C" fn safe_c2rust_get_filename_charset(
    mut filename_charset: *mut *const gchar,
) -> gboolean {
    let mut charsets: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut is_utf8: gboolean = 0;
    is_utf8 = safe_c2rust_g_get_filename_charsets(&raw mut charsets);
    if !filename_charset.is_null() {
        *filename_charset = *charsets.offset(0 as ::core::ffi::c_int as isize);
    }
    return is_utf8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_to_utf8(
    mut opsysstring: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !opsysstring.is_null() {
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
            b"opsysstring != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_get_filename_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(opsysstring, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            opsysstring,
            len,
            b"UTF-8\0" as *const u8 as *const gchar,
            charset,
            (CONVERT_CHECK_NO_NULS_IN_INPUT as ::core::ffi::c_int
                | CONVERT_CHECK_NO_NULS_IN_OUTPUT as ::core::ffi::c_int)
                as ConvertCheckFlags,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_from_utf8(
    mut utf8string: *const gchar,
    mut len: gssize,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    if safe_c2rust_get_filename_charset(&raw mut charset) != 0 {
        return safe_c2rust_strdup_len(utf8string, len, bytes_read, bytes_written, error);
    } else {
        return safe_c2rust_convert_checked(
            utf8string,
            len,
            charset,
            b"UTF-8\0" as *const u8 as *const gchar,
            (CONVERT_CHECK_NO_NULS_IN_INPUT as ::core::ffi::c_int
                | CONVERT_CHECK_NO_NULS_IN_OUTPUT as ::core::ffi::c_int)
                as ConvertCheckFlags,
            bytes_read,
            bytes_written,
            error,
        );
    };
}
unsafe extern "C" fn safe_c2rust_has_case_prefix(
    mut haystack: *const gchar,
    mut needle: *const gchar,
) -> gboolean {
    let mut h: *const gchar = ::core::ptr::null::<gchar>();
    let mut n: *const gchar = ::core::ptr::null::<gchar>();
    h = haystack;
    n = needle;
    while *n as ::core::ffi::c_int != 0
        && *h as ::core::ffi::c_int != 0
        && g_ascii_tolower(*n) as ::core::ffi::c_int == g_ascii_tolower(*h) as ::core::ffi::c_int
    {
        n = n.offset(1);
        h = h.offset(1);
    }
    return (*n as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
}
static mut safe_c2rust_acceptable: [guchar; 96] = [
    0 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x28 as ::core::ffi::c_int as guchar,
    0 as ::core::ffi::c_int as guchar,
    0x2c as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x2a as ::core::ffi::c_int as guchar,
    0x28 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x1c as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x38 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x2c as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x38 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
    0x3f as ::core::ffi::c_int as guchar,
    0x20 as ::core::ffi::c_int as guchar,
];
static mut safe_c2rust_hex: [gchar; 17] =
    unsafe { ::core::mem::transmute::<[u8; 17], [gchar; 17]>(*b"0123456789ABCDEF\0") };
unsafe extern "C" fn safe_c2rust_g_escape_uri_string(
    mut string: *const gchar,
    mut mask: UnsafeCharacterSet,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut c: ::core::ffi::c_int = 0;
    let mut unacceptable: size_t = 0;
    let mut use_mask: UnsafeCharacterSet = 0 as UnsafeCharacterSet;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if mask as ::core::ffi::c_uint == UNSAFE_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
            || mask as ::core::ffi::c_uint
                == UNSAFE_ALLOW_PLUS as ::core::ffi::c_int as ::core::ffi::c_uint
            || mask as ::core::ffi::c_uint
                == UNSAFE_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            || mask as ::core::ffi::c_uint
                == UNSAFE_HOST as ::core::ffi::c_int as ::core::ffi::c_uint
            || mask as ::core::ffi::c_uint
                == UNSAFE_SLASHES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"mask == UNSAFE_ALL || mask == UNSAFE_ALLOW_PLUS || mask == UNSAFE_PATH || mask == UNSAFE_HOST || mask == UNSAFE_SLASHES\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    unacceptable = 0 as size_t;
    use_mask = mask;
    p = string;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        c = *p as guchar as ::core::ffi::c_int;
        if !(c >= 32 as ::core::ffi::c_int
            && c < 128 as ::core::ffi::c_int
            && safe_c2rust_acceptable[(c - 32 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_uint
                & use_mask as ::core::ffi::c_uint
                != 0)
        {
            unacceptable = unacceptable.wrapping_add(1);
        }
        p = p.offset(1);
    }
    if unacceptable
        >= (G_MAXSIZE as size_t)
            .wrapping_sub(p.offset_from(string) as ::core::ffi::c_long as size_t)
            .wrapping_div(2 as size_t)
    {
        g_set_error_literal(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_BAD_URI as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid hostname\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    result = g_malloc(
        (p.offset_from(string) as ::core::ffi::c_long as gsize)
            .wrapping_add((unacceptable as gsize).wrapping_mul(2 as gsize))
            .wrapping_add(1 as gsize),
    ) as *mut gchar;
    use_mask = mask;
    q = result;
    p = string;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        c = *p as guchar as ::core::ffi::c_int;
        if !(c >= 32 as ::core::ffi::c_int
            && c < 128 as ::core::ffi::c_int
            && safe_c2rust_acceptable[(c - 32 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_uint
                & use_mask as ::core::ffi::c_uint
                != 0)
        {
            let fresh5 = q;
            q = q.offset(1);
            *fresh5 = '%' as i32 as gchar;
            let fresh6 = q;
            q = q.offset(1);
            *fresh6 = safe_c2rust_hex[(c >> 4 as ::core::ffi::c_int) as usize];
            let fresh7 = q;
            q = q.offset(1);
            *fresh7 = safe_c2rust_hex[(c & 15 as ::core::ffi::c_int) as usize];
        } else {
            let fresh8 = q;
            q = q.offset(1);
            *fresh8 = *p;
        }
        p = p.offset(1);
    }
    *q = '\0' as i32 as gchar;
    return result;
}
unsafe extern "C" fn safe_c2rust_g_escape_file_uri(
    mut hostname: *const gchar,
    mut pathname: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut escaped_hostname: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut escaped_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !hostname.is_null() && *hostname as ::core::ffi::c_int != '\0' as i32 {
        escaped_hostname = safe_c2rust_g_escape_uri_string(hostname, UNSAFE_HOST, error)
            as *mut ::core::ffi::c_char;
        if escaped_hostname.is_null() {
            current_block = 3788575856143293949;
        } else {
            current_block = 17778012151635330486;
        }
    } else {
        current_block = 17778012151635330486;
    }
    match current_block {
        17778012151635330486 => {
            escaped_path = safe_c2rust_g_escape_uri_string(pathname, UNSAFE_PATH, error)
                as *mut ::core::ffi::c_char;
            if !escaped_path.is_null() {
                res = g_strconcat(
                    b"file://\0" as *const u8 as *const gchar,
                    if !escaped_hostname.is_null() {
                        escaped_hostname as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if *escaped_path as ::core::ffi::c_int != '/' as i32 {
                        b"/\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    escaped_path,
                    NULL,
                ) as *mut ::core::ffi::c_char;
            }
        }
        _ => {}
    }
    g_free(escaped_hostname as gpointer);
    g_free(escaped_path as gpointer);
    return res as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_unescape_character(
    mut scanner: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut first_digit: ::core::ffi::c_int = 0;
    let mut second_digit: ::core::ffi::c_int = 0;
    first_digit = g_ascii_xdigit_value(*scanner.offset(0 as ::core::ffi::c_int as isize) as gchar)
        as ::core::ffi::c_int;
    if first_digit < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    second_digit = g_ascii_xdigit_value(*scanner.offset(1 as ::core::ffi::c_int as isize) as gchar)
        as ::core::ffi::c_int;
    if second_digit < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return first_digit << 4 as ::core::ffi::c_int | second_digit;
}
unsafe extern "C" fn safe_c2rust_g_unescape_uri_string(
    mut escaped: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut illegal_escaped_characters: *const ::core::ffi::c_char,
    mut ascii_must_not_be_escaped: gboolean,
) -> *mut gchar {
    let mut in_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut in_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut c: ::core::ffi::c_int = 0;
    if escaped.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as ::core::ffi::c_int {
        len = strlen(escaped) as ::core::ffi::c_int;
    }
    result = g_malloc((len + 1 as ::core::ffi::c_int) as gsize) as *mut gchar;
    out = result;
    in_0 = escaped as *const gchar;
    in_end = escaped.offset(len as isize) as *const gchar;
    while in_0 < in_end {
        c = *in_0 as ::core::ffi::c_int;
        if c == '%' as i32 {
            if in_0.offset(3 as ::core::ffi::c_int as isize) > in_end {
                break;
            }
            c = safe_c2rust_unescape_character(in_0.offset(1 as ::core::ffi::c_int as isize));
            if c <= 0 as ::core::ffi::c_int {
                break;
            }
            if ascii_must_not_be_escaped != 0 && c <= 0x7f as ::core::ffi::c_int {
                break;
            }
            if !strchr(illegal_escaped_characters, c).is_null() {
                break;
            }
            in_0 = in_0.offset(2 as ::core::ffi::c_int as isize);
        }
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 = c as gchar;
        in_0 = in_0.offset(1);
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if out.offset_from(result) as ::core::ffi::c_long <= len as ::core::ffi::c_long {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gconvert.c\0" as *const u8 as *const ::core::ffi::c_char,
            1519 as ::core::ffi::c_int,
            G_STRFUNC,
            b"out - result <= len\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *out = '\0' as i32 as gchar;
    if in_0 != in_end {
        g_free(result as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_is_asciialphanum(mut c: gunichar) -> gboolean {
    return (c <= 0x7f as gunichar
        && *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_is_asciialpha(mut c: gunichar) -> gboolean {
    return (c <= 0x7f as gunichar
        && *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALPHA as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_hostname_validate(
    mut hostname: *const ::core::ffi::c_char,
) -> gboolean {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: gunichar = 0;
    let mut first_char: gunichar = 0;
    let mut last_char: gunichar = 0;
    p = hostname;
    if *p as ::core::ffi::c_int == '\0' as i32 {
        return TRUE;
    }
    loop {
        c = g_utf8_get_char(p as *const gchar);
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
        if safe_c2rust_is_asciialphanum(c) == 0 {
            return FALSE;
        }
        first_char = c;
        loop {
            last_char = c;
            c = g_utf8_get_char(p as *const gchar);
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            if !(safe_c2rust_is_asciialphanum(c) != 0 || c == '-' as i32 as gunichar) {
                break;
            }
        }
        if last_char == '-' as i32 as gunichar {
            return FALSE;
        }
        if c == '\0' as i32 as gunichar
            || c == '.' as i32 as gunichar && *p as ::core::ffi::c_int == '\0' as i32
        {
            return safe_c2rust_is_asciialpha(first_char);
        }
        if !(c == '.' as i32 as gunichar) {
            break;
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_from_uri(
    mut uri: *const gchar,
    mut hostname: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut past_scheme: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut host_part: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut unescaped_hostname: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut past_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut temp_uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut offs: ::core::ffi::c_int = 0;
    if !hostname.is_null() {
        *hostname = ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_has_case_prefix(uri, b"file:/\0" as *const u8 as *const gchar) == 0 {
        g_set_error(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_BAD_URI as ::core::ffi::c_int as gint,
            glib_gettext(
                b"The URI \xE2\x80\x9C%s\xE2\x80\x9D is not an absolute URI using the \xE2\x80\x9Cfile\xE2\x80\x9D scheme\0"
                    as *const u8 as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    temp_uri = safe_c2rust_g_strdup_inline(uri as *const ::core::ffi::c_char);
    past_scheme =
        temp_uri.offset(strlen(b"file:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    past_path = strchr(past_scheme, '?' as i32);
    if !past_path.is_null() {
        *past_path = '\0' as i32 as ::core::ffi::c_char;
    }
    past_path = strchr(past_scheme, '#' as i32);
    if !past_path.is_null() {
        *past_path = '\0' as i32 as ::core::ffi::c_char;
    }
    if safe_c2rust_has_case_prefix(
        past_scheme as *const gchar,
        b"///\0" as *const u8 as *const gchar,
    ) != 0
    {
        past_scheme = past_scheme.offset(2 as ::core::ffi::c_int as isize);
    } else if safe_c2rust_has_case_prefix(
        past_scheme as *const gchar,
        b"//\0" as *const u8 as *const gchar,
    ) != 0
    {
        past_scheme = past_scheme.offset(2 as ::core::ffi::c_int as isize);
        host_part = past_scheme;
        past_scheme = strchr(past_scheme, '/' as i32);
        if past_scheme.is_null() {
            g_free(temp_uri as gpointer);
            g_set_error(
                error,
                safe_c2rust_g_convert_error_quark(),
                G_CONVERT_ERROR_BAD_URI as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The URI \xE2\x80\x9C%s\xE2\x80\x9D is invalid\0" as *const u8 as *const gchar,
                ),
                uri,
            );
            return ::core::ptr::null_mut::<gchar>();
        }
        unescaped_hostname = safe_c2rust_g_unescape_uri_string(
            host_part,
            past_scheme.offset_from(host_part) as ::core::ffi::c_long as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            TRUE,
        ) as *mut ::core::ffi::c_char;
        if unescaped_hostname.is_null() || safe_c2rust_hostname_validate(unescaped_hostname) == 0 {
            g_free(unescaped_hostname as gpointer);
            g_free(temp_uri as gpointer);
            g_set_error(
                error,
                safe_c2rust_g_convert_error_quark(),
                G_CONVERT_ERROR_BAD_URI as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The hostname of the URI \xE2\x80\x9C%s\xE2\x80\x9D is invalid\0" as *const u8
                        as *const gchar,
                ),
                uri,
            );
            return ::core::ptr::null_mut::<gchar>();
        }
        if !hostname.is_null() {
            *hostname = unescaped_hostname as *mut gchar;
        } else {
            g_free(unescaped_hostname as gpointer);
        }
    }
    filename = safe_c2rust_g_unescape_uri_string(
        past_scheme,
        -(1 as ::core::ffi::c_int),
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        FALSE,
    ) as *mut ::core::ffi::c_char;
    if filename.is_null() {
        g_free(temp_uri as gpointer);
        g_set_error(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_BAD_URI as ::core::ffi::c_int as gint,
            glib_gettext(
                b"The URI \xE2\x80\x9C%s\xE2\x80\x9D contains invalidly escaped characters\0"
                    as *const u8 as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    offs = 0 as ::core::ffi::c_int;
    result = safe_c2rust_g_strdup_inline(filename.offset(offs as isize));
    g_free(filename as gpointer);
    g_free(temp_uri as gpointer);
    return result as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_to_uri(
    mut filename: *const gchar,
    mut hostname: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut escaped_uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !filename.is_null() {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if g_path_is_absolute(filename) == 0 {
        g_set_error(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_NOT_ABSOLUTE_PATH as ::core::ffi::c_int as gint,
            glib_gettext(
                b"The pathname \xE2\x80\x9C%s\xE2\x80\x9D is not an absolute path\0" as *const u8
                    as *const gchar,
            ),
            filename,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if !hostname.is_null()
        && !(g_utf8_validate(
            hostname,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
            && safe_c2rust_hostname_validate(hostname as *const ::core::ffi::c_char) != 0)
    {
        g_set_error_literal(
            error,
            safe_c2rust_g_convert_error_quark(),
            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid hostname\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    escaped_uri =
        safe_c2rust_g_escape_file_uri(hostname, filename, error) as *mut ::core::ffi::c_char;
    return escaped_uri as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_list_extract_uris(
    mut uri_list: *const gchar,
) -> *mut *mut gchar {
    let mut uris: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut q: *const gchar = ::core::ptr::null::<gchar>();
    uris = g_ptr_array_new();
    p = uri_list;
    while !p.is_null() {
        if *p as ::core::ffi::c_int != '#' as i32 {
            while *safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            q = p;
            while *q as ::core::ffi::c_int != 0
                && *q as ::core::ffi::c_int != '\n' as i32
                && *q as ::core::ffi::c_int != '\r' as i32
            {
                q = q.offset(1);
            }
            if q > p {
                q = q.offset(-1);
                while q > p
                    && *safe_c2rust_g_ascii_table.offset(*q as guchar as isize)
                        as ::core::ffi::c_int
                        & G_ASCII_SPACE as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    q = q.offset(-1);
                }
                if q > p {
                    g_ptr_array_add(
                        uris,
                        g_strndup(
                            p,
                            (q.offset_from(p) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                as gsize,
                        ) as gpointer,
                    );
                }
            }
        }
        p = strchr(p as *const ::core::ffi::c_char, '\n' as i32);
        if !p.is_null() {
            p = p.offset(1);
        }
    }
    g_ptr_array_add(uris, NULL);
    return g_ptr_array_free(uris, FALSE) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_display_basename(
    mut filename: *const gchar,
) -> *mut gchar {
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut display_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !filename.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    basename = g_path_get_basename(filename) as *mut ::core::ffi::c_char;
    display_name = safe_c2rust_g_filename_display_name(basename) as *mut ::core::ffi::c_char;
    g_free(basename as gpointer);
    return display_name as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_display_name(
    mut filename: *const gchar,
) -> *mut gchar {
    let mut i: gint = 0;
    let mut charsets: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut display_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut is_utf8: gboolean = 0;
    is_utf8 = safe_c2rust_g_get_filename_charsets(&raw mut charsets);
    if is_utf8 != 0 {
        if g_utf8_validate(
            filename,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
            display_name =
                safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
        }
    }
    if display_name.is_null() {
        i = (if is_utf8 != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as gint;
        while !(*charsets.offset(i as isize)).is_null() {
            display_name = safe_c2rust_g_convert(
                filename,
                -(1 as ::core::ffi::c_int) as gssize,
                b"UTF-8\0" as *const u8 as *const gchar,
                *charsets.offset(i as isize),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if !display_name.is_null() {
                break;
            }
            i += 1;
        }
    }
    if display_name.is_null() {
        display_name = g_utf8_make_valid(filename, -(1 as ::core::ffi::c_int) as gssize);
    }
    return display_name;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_convert_with_iconv\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
