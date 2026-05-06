extern "C" {
    fn g_free(mem: gpointer);
    fn g_rc_box_alloc_full(
        block_size: gsize,
        alignment: gsize,
        atomic: gboolean,
        clear: gboolean,
    ) -> gpointer;
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
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GArcBox {
    pub ref_count: gatomicrefcount,
    pub mem_size: gsize,
    pub private_offset: gsize,
    pub magic: guint32,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STRUCT_ALIGNMENT: usize =
    (2 as usize).wrapping_mul(::core::mem::size_of::<gsize>() as usize);
pub const G_ARC_BOX_SIZE: usize = ::core::mem::size_of::<GArcBox>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_alloc(mut block_size: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, TRUE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_alloc0(mut block_size: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, TRUE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_dup(
    mut block_size: gsize,
    mut mem_block: gconstpointer,
) -> gpointer {
    let mut res: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if block_size > 0 as gsize {
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
            b"block_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    res = g_rc_box_alloc_full(block_size, STRUCT_ALIGNMENT as gsize, TRUE, FALSE);
    memcpy(
        res as *mut ::core::ffi::c_void,
        mem_block as *const ::core::ffi::c_void,
        block_size as size_t,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_acquire(mut mem_block: gpointer) -> gpointer {
    let mut real_box: *mut GArcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_ARC_BOX_SIZE as isize)) as *mut GArcBox;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_atomic_ref_count_inc(&raw mut (*real_box).ref_count);
    return mem_block;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_release(mut mem_block: gpointer) {
    safe_c2rust_g_atomic_rc_box_release_full(mem_block, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_release_full(
    mut mem_block: gpointer,
    mut clear_func: GDestroyNotify,
) {
    let mut real_box: *mut GArcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_ARC_BOX_SIZE as isize)) as *mut GArcBox;
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
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*real_box).ref_count) != 0 {
        let mut real_mem: *mut ::core::ffi::c_char =
            (real_box as *mut ::core::ffi::c_char).offset(-((*real_box).private_offset as isize));
        if clear_func.is_some() {
            clear_func.expect("non-null function pointer")(mem_block);
        }
        g_free(real_mem as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_rc_box_get_size(mut mem_block: gpointer) -> gsize {
    let mut real_box: *mut GArcBox =
        (mem_block as *mut ::core::ffi::c_char).offset(-(G_ARC_BOX_SIZE as isize)) as *mut GArcBox;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !mem_block.is_null() {
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
            b"mem_block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*real_box).magic == 0x44ae2bf0 as ::core::ffi::c_int as guint32 {
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
            b"real_box->magic == G_BOX_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*real_box).mem_size;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_atomic_rc_box_alloc\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
