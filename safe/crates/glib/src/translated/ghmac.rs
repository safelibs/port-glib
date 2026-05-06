extern "C" {
    pub type _GBytes;
    pub type _GChecksum;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_checksum_type_get_length(checksum_type: GChecksumType) -> gssize;
    fn g_checksum_new(checksum_type: GChecksumType) -> *mut GChecksum;
    fn g_checksum_reset(checksum: *mut GChecksum);
    fn g_checksum_copy(checksum: *const GChecksum) -> *mut GChecksum;
    fn g_checksum_free(checksum: *mut GChecksum);
    fn g_checksum_update(checksum: *mut GChecksum, data: *const guchar, length: gssize);
    fn g_checksum_get_string(checksum: *mut GChecksum) -> *const gchar;
    fn g_checksum_get_digest(checksum: *mut GChecksum, buffer: *mut guint8, digest_len: *mut gsize);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type guchar = ::core::ffi::c_uchar;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GBytes = _GBytes;
pub type GChecksumType = ::core::ffi::c_uint;
pub const G_CHECKSUM_SHA384: GChecksumType = 4;
pub const G_CHECKSUM_SHA512: GChecksumType = 3;
pub const G_CHECKSUM_SHA256: GChecksumType = 2;
pub const G_CHECKSUM_SHA1: GChecksumType = 1;
pub const G_CHECKSUM_MD5: GChecksumType = 0;
pub type GChecksum = _GChecksum;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHmac {
    pub ref_count: ::core::ffi::c_int,
    pub digest_type: GChecksumType,
    pub digesti: *mut GChecksum,
    pub digesto: *mut GChecksum,
}
pub type GHmac = _GHmac;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_new(
    mut digest_type: GChecksumType,
    mut key: *const guchar,
    mut key_len: gsize,
) -> *mut GHmac {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut checksum: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    let mut hmac: *mut GHmac = ::core::ptr::null_mut::<GHmac>();
    let mut buffer: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut pad: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut i: gsize = 0;
    let mut len: gsize = 0;
    let mut block_size: gsize = 0;
    let mut block_size_signed: gssize = 0;
    let mut key_len_signed: gssize = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if key_len <= 9223372036854775807 as ::core::ffi::c_long as gsize {
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
            b"key_len <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHmac>();
    }
    checksum = g_checksum_new(digest_type);
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
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
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHmac>();
    }
    match digest_type as ::core::ffi::c_uint {
        0 | 1 => {
            block_size = 64 as gsize;
        }
        2 => {
            block_size = 64 as gsize;
        }
        4 | 3 => {
            block_size = 128 as gsize;
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
                b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int,
                G_STRFUNC,
            );
            return ::core::ptr::null_mut::<GHmac>();
        }
    }
    hmac = ({
        let mut __s: gsize = ::core::mem::size_of::<GHmac>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GHmac;
    (*hmac).ref_count = 1 as ::core::ffi::c_int;
    (*hmac).digest_type = digest_type;
    (*hmac).digesti = checksum;
    (*hmac).digesto = g_checksum_new(digest_type);
    buffer = (if block_size == 0 as gsize {
        NULL
    } else {
        alloca_allocations.push(::std::vec::from_elem(0, block_size as usize));
        memset(
            alloca_allocations.last_mut().unwrap().as_mut_ptr().cast(),
            0 as ::core::ffi::c_int,
            block_size as size_t,
        )
    }) as *mut guchar;
    alloca_allocations.push(::std::vec::from_elem(0, block_size as usize));
    pad = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut guchar;
    if key_len > block_size {
        len = block_size;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if key_len <= 9223372036854775807 as ::core::ffi::c_long as gsize {
                _g_boolean_var_10 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_10 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_10
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_int,
                G_STRFUNC,
                b"key_len <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        key_len_signed = key_len as gssize;
        g_checksum_update((*hmac).digesti, key, key_len_signed);
        g_checksum_get_digest((*hmac).digesti, buffer as *mut guint8, &raw mut len);
        g_checksum_reset((*hmac).digesti);
    } else {
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            key as *const ::core::ffi::c_void,
            key_len as size_t,
        );
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if block_size <= 9223372036854775807 as ::core::ffi::c_long as gsize {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
            159 as ::core::ffi::c_int,
            G_STRFUNC,
            b"block_size <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    block_size_signed = block_size as gssize;
    i = 0 as gsize;
    while i < block_size {
        *pad.offset(i as isize) = (0x36 as ::core::ffi::c_int
            ^ *buffer.offset(i as isize) as ::core::ffi::c_int)
            as guchar;
        i = i.wrapping_add(1);
    }
    g_checksum_update((*hmac).digesti, pad, block_size_signed);
    i = 0 as gsize;
    while i < block_size {
        *pad.offset(i as isize) = (0x5c as ::core::ffi::c_int
            ^ *buffer.offset(i as isize) as ::core::ffi::c_int)
            as guchar;
        i = i.wrapping_add(1);
    }
    g_checksum_update((*hmac).digesto, pad, block_size_signed);
    return hmac;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_copy(mut hmac: *const GHmac) -> *mut GHmac {
    let mut copy: *mut GHmac = ::core::ptr::null_mut::<GHmac>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
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
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHmac>();
    }
    copy = g_slice_alloc(::core::mem::size_of::<GHmac>() as gsize) as *mut GHmac;
    (*copy).ref_count = 1 as ::core::ffi::c_int;
    (*copy).digest_type = (*hmac).digest_type;
    (*copy).digesti = g_checksum_copy((*hmac).digesti);
    (*copy).digesto = g_checksum_copy((*hmac).digesto);
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_ref(mut hmac: *mut GHmac) -> *mut GHmac {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
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
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHmac>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*hmac).ref_count;
        (*hmac).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*hmac).ref_count, 1 as ::core::ffi::c_int);
    return hmac;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_unref(mut hmac: *mut GHmac) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
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
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*hmac).ref_count;
            (*hmac).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*hmac).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_checksum_free((*hmac).digesti);
        g_checksum_free((*hmac).digesto);
        g_slice_free1(::core::mem::size_of::<GHmac>() as gsize, hmac as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_update(
    mut hmac: *mut GHmac,
    mut data: *const guchar,
    mut length: gssize,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
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
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !data.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_checksum_update((*hmac).digesti, data, length);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_get_string(mut hmac: *mut GHmac) -> *const gchar {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut buffer: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut digest_len_signed: gssize = 0;
    let mut digest_len: gsize = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
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
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    digest_len_signed = g_checksum_type_get_length((*hmac).digest_type);
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if digest_len_signed >= 0 as gssize {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
            305 as ::core::ffi::c_int,
            G_STRFUNC,
            b"digest_len_signed >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    digest_len = digest_len_signed as gsize;
    alloca_allocations.push(::std::vec::from_elem(0, digest_len as usize));
    buffer = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut guint8;
    safe_c2rust_g_hmac_get_digest(hmac, buffer, &raw mut digest_len);
    return g_checksum_get_string((*hmac).digesto);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hmac_get_digest(
    mut hmac: *mut GHmac,
    mut buffer: *mut guint8,
    mut digest_len: *mut gsize,
) {
    let mut len: gsize = 0;
    let mut len_signed: gssize = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !hmac.is_null() {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hmac != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    len_signed = g_checksum_type_get_length((*hmac).digest_type);
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if len_signed >= 0 as gssize {
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
            b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
            346 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len_signed >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    len = len_signed as gsize;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if *digest_len >= len {
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
            b"*digest_len >= len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_checksum_get_digest((*hmac).digesti, buffer, &raw mut len);
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if len <= 9223372036854775807 as ::core::ffi::c_long as gsize {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghmac.c\0" as *const u8 as *const ::core::ffi::c_char,
            354 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    len_signed = len as gssize;
    g_checksum_update((*hmac).digesto, buffer, len_signed);
    g_checksum_get_digest((*hmac).digesto, buffer, digest_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_hmac_for_data(
    mut digest_type: GChecksumType,
    mut key: *const guchar,
    mut key_len: gsize,
    mut data: *const guchar,
    mut length: gsize,
) -> *mut gchar {
    let mut hmac: *mut GHmac = ::core::ptr::null_mut::<GHmac>();
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if length == 0 as gsize || !data.is_null() {
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
            b"length == 0 || data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    hmac = safe_c2rust_g_hmac_new(digest_type, key, key_len);
    if hmac.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    safe_c2rust_g_hmac_update(hmac, data, length as gssize);
    retval = safe_c2rust_g_strdup_inline(
        safe_c2rust_g_hmac_get_string(hmac) as *const ::core::ffi::c_char
    ) as *mut gchar;
    safe_c2rust_g_hmac_unref(hmac);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_hmac_for_bytes(
    mut digest_type: GChecksumType,
    mut key: *mut GBytes,
    mut data: *mut GBytes,
) -> *mut gchar {
    let mut byte_data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    let mut length: gsize = 0;
    let mut key_data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    let mut key_len: gsize = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !data.is_null() {
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
            b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    byte_data = g_bytes_get_data(data, &raw mut length);
    key_data = g_bytes_get_data(key, &raw mut key_len);
    return safe_c2rust_g_compute_hmac_for_data(
        digest_type,
        key_data as *const guchar,
        key_len,
        byte_data as *const guchar,
        length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_hmac_for_string(
    mut digest_type: GChecksumType,
    mut key: *const guchar,
    mut key_len: gsize,
    mut str: *const gchar,
    mut length: gssize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !str.is_null() {
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
            b"length == 0 || str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if length < 0 as gssize {
        length = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    return safe_c2rust_g_compute_hmac_for_data(
        digest_type,
        key,
        key_len,
        str as *const guchar,
        length as gsize,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_hmac_new\0" as *const u8 as *const ::core::ffi::c_char;
