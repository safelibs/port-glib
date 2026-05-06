extern "C" {
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_ref_count_init(rc: *mut grefcount);
    fn g_ref_count_inc(rc: *mut grefcount);
    fn g_ref_count_dec(rc: *mut grefcount) -> gboolean;
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type grefcount = gint;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GArcBox {
    pub ref_count: gatomicrefcount,
    pub mem_size: gsize,
    pub private_offset: gsize,
    pub magic: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GRcBox {
    pub ref_count: grefcount,
    pub mem_size: gsize,
    pub private_offset: gsize,
    pub magic: guint32,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_BOX_MAGIC: ::core::ffi::c_int = 0x44ae2bf0 as ::core::ffi::c_int;
pub const STRUCT_ALIGNMENT: usize =
    (2 as usize).wrapping_mul(::core::mem::size_of::<gsize>() as usize);
pub const G_RC_BOX_SIZE: usize = ::core::mem::size_of::<GRcBox>();
pub const G_ARC_BOX_SIZE: usize = ::core::mem::size_of::<GArcBox>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_alloc_full(
    mut block_size: gsize,
    mut alignment: gsize,
    mut atomic: gboolean,
    mut clear: gboolean,
) -> gpointer {
    let mut private_size: gsize = G_ARC_BOX_SIZE as gsize;
    let mut private_offset: gsize = 0 as gsize;
    let mut real_size: gsize = 0;
    let mut allocated: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if alignment != 0 as gsize {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/grcbox.c\0" as *const u8 as *const ::core::ffi::c_char,
            58 as ::core::ffi::c_int,
            G_STRFUNC,
            b"alignment != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if private_size.wrapping_rem(alignment) != 0 as gsize {
        private_offset = private_size.wrapping_rem(alignment);
        private_size = private_size.wrapping_add(alignment.wrapping_sub(private_offset));
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if block_size
            < (9223372036854775807 as ::core::ffi::c_long as gsize)
                .wrapping_mul(2 as gsize)
                .wrapping_add(1 as gsize)
                .wrapping_sub(private_size)
        {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/grcbox.c\0" as *const u8 as *const ::core::ffi::c_char,
            67 as ::core::ffi::c_int,
            G_STRFUNC,
            b"block_size < (G_MAXSIZE - private_size)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    real_size = private_size.wrapping_add(block_size);
    if real_size.wrapping_rem(alignment) != 0 as gsize {
        let mut offset: gsize = real_size.wrapping_rem(alignment);
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if real_size
                < (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_sub(alignment.wrapping_sub(offset))
            {
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
                b"../original/glib/grcbox.c\0" as *const u8 as *const ::core::ffi::c_char,
                76 as ::core::ffi::c_int,
                G_STRFUNC,
                b"real_size < (G_MAXSIZE - (alignment - offset))\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        real_size = real_size.wrapping_add(alignment.wrapping_sub(offset));
    }
    if clear != 0 {
        allocated = g_malloc0(real_size) as *mut ::core::ffi::c_char;
    } else {
        allocated = g_malloc(real_size) as *mut ::core::ffi::c_char;
    }
    if atomic != 0 {
        let mut real_box: *mut GArcBox = allocated.offset(private_offset as isize) as *mut GArcBox;
        (*real_box).mem_size = block_size;
        (*real_box).private_offset = private_offset;
        (*real_box).magic = G_BOX_MAGIC as guint32;
        g_atomic_ref_count_init(&raw mut (*real_box).ref_count);
    } else {
        let mut real_box_0: *mut GRcBox = allocated.offset(private_offset as isize) as *mut GRcBox;
        (*real_box_0).mem_size = block_size;
        (*real_box_0).private_offset = private_offset;
        (*real_box_0).magic = G_BOX_MAGIC as guint32;
        g_ref_count_init(&raw mut (*real_box_0).ref_count);
    }
    return allocated.offset(private_size as isize) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_alloc(mut block_size: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return safe_c2rust_g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, FALSE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_alloc0(mut block_size: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return safe_c2rust_g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, FALSE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_dup(
    mut block_size: gsize,
    mut mem_block: gconstpointer,
) -> gpointer {
    let mut res: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    res = safe_c2rust_g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, FALSE, FALSE);
    memcpy(
        res as *mut ::core::ffi::c_void,
        mem_block as *const ::core::ffi::c_void,
        block_size as size_t,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_acquire(mut mem_block: gpointer) -> gpointer {
    let mut real_box: *mut GRcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_RC_BOX_SIZE as isize)) as *mut GRcBox;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_ref_count_inc(&raw mut (*real_box).ref_count);
    return mem_block;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_release(mut mem_block: gpointer) {
    safe_c2rust_g_rc_box_release_full(mem_block, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_release_full(
    mut mem_block: gpointer,
    mut clear_func: GDestroyNotify,
) {
    let mut real_box: *mut GRcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_RC_BOX_SIZE as isize)) as *mut GRcBox;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_ref_count_dec(&raw mut (*real_box).ref_count) != 0 {
        let mut real_mem: *mut ::core::ffi::c_char =
            (real_box as *mut ::core::ffi::c_char).offset(-((*real_box).private_offset as isize));
        if clear_func.is_some() {
            clear_func.expect("non-null function pointer")(mem_block);
        }
        g_free(real_mem as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rc_box_get_size(mut mem_block: gpointer) -> gsize {
    let mut real_box: *mut GRcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_RC_BOX_SIZE as isize)) as *mut GRcBox;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*real_box).mem_size;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_rc_box_alloc_full\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
