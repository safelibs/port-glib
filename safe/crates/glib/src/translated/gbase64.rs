extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
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
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
static mut safe_c2rust_base64_alphabet: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_encode_step(
    mut in_0: *const guchar,
    mut len: gsize,
    mut break_lines: gboolean,
    mut out: *mut gchar,
    mut state: *mut gint,
    mut save: *mut gint,
) -> gsize {
    let mut outptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut inptr: *const guchar = ::core::ptr::null::<guchar>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !in_0.is_null() || len == 0 as gsize {
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
            b"in != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !out.is_null() {
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
            b"out != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !state.is_null() {
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
            b"state != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !save.is_null() {
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
            b"save != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if len == 0 as gsize {
        return 0 as gsize;
    }
    inptr = in_0;
    outptr = out as *mut ::core::ffi::c_char;
    if len.wrapping_add(
        *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize) as gsize,
    ) > 2 as gsize
    {
        let mut current_block_49: u64;
        let mut inend: *const guchar = in_0
            .offset(len as isize)
            .offset(-(2 as ::core::ffi::c_int as isize));
        let mut c1: ::core::ffi::c_int = 0;
        let mut c2: ::core::ffi::c_int = 0;
        let mut c3: ::core::ffi::c_int = 0;
        let mut already: ::core::ffi::c_int = 0;
        already = *state as ::core::ffi::c_int;
        match *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
        {
            1 => {
                c1 = *(save as *mut ::core::ffi::c_uchar).offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
                current_block_49 = 17118455301284297392;
            }
            2 => {
                c1 = *(save as *mut ::core::ffi::c_uchar).offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
                c2 = *(save as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
                current_block_49 = 1650114636519394204;
            }
            _ => {
                current_block_49 = 18377268871191777778;
            }
        }
        loop {
            match current_block_49 {
                17118455301284297392 => {
                    let fresh1 = inptr;
                    inptr = inptr.offset(1);
                    c2 = *fresh1 as ::core::ffi::c_int;
                    current_block_49 = 1650114636519394204;
                }
                18377268871191777778 => {
                    if !(inptr < inend) {
                        break;
                    }
                    let fresh0 = inptr;
                    inptr = inptr.offset(1);
                    c1 = *fresh0 as ::core::ffi::c_int;
                    current_block_49 = 17118455301284297392;
                }
                _ => {
                    let fresh2 = inptr;
                    inptr = inptr.offset(1);
                    c3 = *fresh2 as ::core::ffi::c_int;
                    let fresh3 = outptr;
                    outptr = outptr.offset(1);
                    *fresh3 = safe_c2rust_base64_alphabet[(c1 >> 2 as ::core::ffi::c_int) as usize];
                    let fresh4 = outptr;
                    outptr = outptr.offset(1);
                    *fresh4 = safe_c2rust_base64_alphabet[(c2 >> 4 as ::core::ffi::c_int
                        | (c1 & 0x3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as usize];
                    let fresh5 = outptr;
                    outptr = outptr.offset(1);
                    *fresh5 = safe_c2rust_base64_alphabet[((c2 & 0xf as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                        | c3 >> 6 as ::core::ffi::c_int)
                        as usize];
                    let fresh6 = outptr;
                    outptr = outptr.offset(1);
                    *fresh6 =
                        safe_c2rust_base64_alphabet[(c3 & 0x3f as ::core::ffi::c_int) as usize];
                    if break_lines != 0 && {
                        already += 1;
                        already >= 19 as ::core::ffi::c_int
                    } {
                        let fresh7 = outptr;
                        outptr = outptr.offset(1);
                        *fresh7 = '\n' as i32 as ::core::ffi::c_char;
                        already = 0 as ::core::ffi::c_int;
                    }
                    current_block_49 = 18377268871191777778;
                }
            }
        }
        *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize) =
            0 as ::core::ffi::c_char;
        len = (2 as ::core::ffi::c_long - inptr.offset_from(inend) as ::core::ffi::c_long) as gsize;
        *state = already as gint;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if len == 0 as gsize || len == 1 as gsize || len == 2 as gsize {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbase64.c\0" as *const u8 as *const ::core::ffi::c_char,
            141 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len == 0 || len == 1 || len == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut saveout: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    saveout = ((save as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char)
        .offset(
            *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int as isize,
        );
    let mut current_block_60: u64;
    match len {
        2 => {
            let fresh8 = inptr;
            inptr = inptr.offset(1);
            let fresh9 = saveout;
            saveout = saveout.offset(1);
            *fresh9 = *fresh8 as ::core::ffi::c_char;
            current_block_60 = 8085337086024616189;
        }
        1 => {
            current_block_60 = 8085337086024616189;
        }
        _ => {
            current_block_60 = 7018308795614528254;
        }
    }
    match current_block_60 {
        8085337086024616189 => {
            let fresh10 = inptr;
            inptr = inptr.offset(1);
            let fresh11 = saveout;
            saveout = saveout.offset(1);
            *fresh11 = *fresh10 as ::core::ffi::c_char;
        }
        _ => {}
    }
    let ref mut fresh12 =
        *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize);
    *fresh12 = (*fresh12 as gsize).wrapping_add(len) as ::core::ffi::c_char as ::core::ffi::c_char;
    return outptr.offset_from(out) as ::core::ffi::c_long as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_encode_close(
    mut break_lines: gboolean,
    mut out: *mut gchar,
    mut state: *mut gint,
    mut save: *mut gint,
) -> gsize {
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut outptr: *mut ::core::ffi::c_char = out as *mut ::core::ffi::c_char;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !out.is_null() {
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
            b"out != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !state.is_null() {
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
            b"state != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !save.is_null() {
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
            b"save != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    c1 = *(save as *mut ::core::ffi::c_uchar).offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int;
    c2 = *(save as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int;
    let mut current_block_34: u64;
    match *(save as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
    {
        2 => {
            *outptr.offset(2 as ::core::ffi::c_int as isize) = safe_c2rust_base64_alphabet
                [((c2 & 0xf as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as usize];
            if ({
                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                if *outptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
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
                    b"../original/glib/gbase64.c\0" as *const u8 as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"outptr [2] != 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            current_block_34 = 1016252506495295328;
        }
        1 => {
            *outptr.offset(2 as ::core::ffi::c_int as isize) = '=' as i32 as ::core::ffi::c_char;
            c2 = 0 as ::core::ffi::c_int;
            current_block_34 = 1016252506495295328;
        }
        _ => {
            current_block_34 = 5689316957504528238;
        }
    }
    match current_block_34 {
        1016252506495295328 => {
            *outptr.offset(0 as ::core::ffi::c_int as isize) =
                safe_c2rust_base64_alphabet[(c1 >> 2 as ::core::ffi::c_int) as usize];
            *outptr.offset(1 as ::core::ffi::c_int as isize) = safe_c2rust_base64_alphabet[(c2
                >> 4 as ::core::ffi::c_int
                | (c1 & 0x3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                as usize];
            *outptr.offset(3 as ::core::ffi::c_int as isize) = '=' as i32 as ::core::ffi::c_char;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
        }
        _ => {}
    }
    if break_lines != 0 {
        let fresh13 = outptr;
        outptr = outptr.offset(1);
        *fresh13 = '\n' as i32 as ::core::ffi::c_char;
    }
    *save = 0 as ::core::ffi::c_int as gint;
    *state = 0 as ::core::ffi::c_int as gint;
    return outptr.offset_from(out) as ::core::ffi::c_long as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_encode(
    mut data: *const guchar,
    mut len: gsize,
) -> *mut gchar {
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut state: gint = 0 as gint;
    let mut save: gint = 0 as gint;
    let mut outlen: gsize = 0;
    let mut allocsize: gsize = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !data.is_null() || len == 0 as gsize {
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
            b"data != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if len
            < (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong)
                .wrapping_sub(1 as ::core::ffi::c_ulong)
                .wrapping_div(4 as ::core::ffi::c_ulong)
                .wrapping_sub(1 as ::core::ffi::c_ulong)
                .wrapping_mul(3 as ::core::ffi::c_ulong)
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
            b"len < ((G_MAXSIZE - 1) / 4 - 1) * 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    allocsize = len
        .wrapping_div(3 as gsize)
        .wrapping_add(1 as gsize)
        .wrapping_mul(4 as gsize)
        .wrapping_add(1 as gsize);
    out = g_malloc(allocsize) as *mut gchar;
    outlen = safe_c2rust_g_base64_encode_step(data, len, FALSE, out, &raw mut state, &raw mut save);
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if outlen <= allocsize {
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
            b"../original/glib/gbase64.c\0" as *const u8 as *const ::core::ffi::c_char,
            258 as ::core::ffi::c_int,
            G_STRFUNC,
            b"outlen <= allocsize\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    outlen = outlen.wrapping_add(safe_c2rust_g_base64_encode_close(
        FALSE,
        out.offset(outlen as isize),
        &raw mut state,
        &raw mut save,
    ));
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if outlen <= allocsize {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbase64.c\0" as *const u8 as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int,
            G_STRFUNC,
            b"outlen <= allocsize\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *out.offset(outlen as isize) = '\0' as i32 as gchar;
    return out;
}
static mut safe_c2rust_mime_base64_rank: [::core::ffi::c_uchar; 256] = [
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_decode_step(
    mut in_0: *const gchar,
    mut len: gsize,
    mut out: *mut guchar,
    mut state: *mut gint,
    mut save: *mut guint,
) -> gsize {
    let mut inptr: *const guchar = ::core::ptr::null::<guchar>();
    let mut outptr: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut inend: *const guchar = ::core::ptr::null::<guchar>();
    let mut c: guchar = 0;
    let mut rank: guchar = 0;
    let mut last: [guchar; 2] = [0; 2];
    let mut v: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !in_0.is_null() || len == 0 as gsize {
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
            b"in != NULL || len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !out.is_null() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"out != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !state.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"state != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !save.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"save != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if len == 0 as gsize {
        return 0 as gsize;
    }
    inend = (in_0 as *const guchar).offset(len as isize);
    outptr = out;
    v = *save as ::core::ffi::c_uint;
    i = *state as ::core::ffi::c_int;
    last[1 as ::core::ffi::c_int as usize] = 0 as guchar;
    last[0 as ::core::ffi::c_int as usize] = last[1 as ::core::ffi::c_int as usize];
    if i < 0 as ::core::ffi::c_int {
        i = -i;
        last[0 as ::core::ffi::c_int as usize] = '=' as i32 as guchar;
    }
    inptr = in_0 as *const guchar;
    while inptr < inend {
        let fresh14 = inptr;
        inptr = inptr.offset(1);
        c = *fresh14;
        rank = safe_c2rust_mime_base64_rank[c as usize] as guchar;
        if rank as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
            last[1 as ::core::ffi::c_int as usize] = last[0 as ::core::ffi::c_int as usize];
            last[0 as ::core::ffi::c_int as usize] = c;
            v = v << 6 as ::core::ffi::c_int | rank as ::core::ffi::c_uint;
            i += 1;
            if i == 4 as ::core::ffi::c_int {
                let fresh15 = outptr;
                outptr = outptr.offset(1);
                *fresh15 = (v >> 16 as ::core::ffi::c_int) as guchar;
                if last[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '=' as i32 {
                    let fresh16 = outptr;
                    outptr = outptr.offset(1);
                    *fresh16 = (v >> 8 as ::core::ffi::c_int) as guchar;
                }
                if last[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '=' as i32 {
                    let fresh17 = outptr;
                    outptr = outptr.offset(1);
                    *fresh17 = v as guchar;
                }
                i = 0 as ::core::ffi::c_int;
            }
        }
    }
    *save = v as guint;
    *state = (if last[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '=' as i32 {
        -i
    } else {
        i
    }) as gint;
    return outptr.offset_from(out) as ::core::ffi::c_long as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_decode(
    mut text: *const gchar,
    mut out_len: *mut gsize,
) -> *mut guchar {
    let mut ret: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut input_length: gsize = 0;
    let mut state: gint = 0 as gint;
    let mut save: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !text.is_null() {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !out_len.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"out_len != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    input_length = strlen(text as *const ::core::ffi::c_char) as gsize;
    ret = g_malloc0(
        input_length
            .wrapping_div(4 as gsize)
            .wrapping_mul(3 as gsize)
            .wrapping_add(1 as gsize),
    ) as *mut guchar;
    *out_len =
        safe_c2rust_g_base64_decode_step(text, input_length, ret, &raw mut state, &raw mut save);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_base64_decode_inplace(
    mut text: *mut gchar,
    mut out_len: *mut gsize,
) -> *mut guchar {
    let mut input_length: gint = 0;
    let mut state: gint = 0 as gint;
    let mut save: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !text.is_null() {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !out_len.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"out_len != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    input_length = strlen(text) as gint;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if input_length > 1 as ::core::ffi::c_int {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"input_length > 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    *out_len = safe_c2rust_g_base64_decode_step(
        text,
        input_length as gsize,
        text as *mut guchar,
        &raw mut state,
        &raw mut save,
    );
    return text as *mut guchar;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_base64_encode_step\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
