extern "C" {
    pub type _GVariantType;
    pub type _GVariantTypeInfo;
    pub type _GBytes;
    fn g_variant_type_info_query(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_query_depth(typeinfo: *mut GVariantTypeInfo) -> gsize;
    fn g_variant_type_info_get(type_0: *const GVariantType) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_unref(typeinfo: *mut GVariantTypeInfo);
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_bytes_new_from_bytes(bytes: *mut GBytes, offset: gsize, length: gsize) -> *mut GBytes;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_get_size(bytes: *mut GBytes) -> gsize;
    fn g_bytes_ref(bytes: *mut GBytes) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut ::core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::core::ffi::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_tuple(children: *const *mut GVariant, n_children: gsize) -> *mut GVariant;
    fn g_variant_serialised_n_children(container: GVariantSerialised) -> gsize;
    fn g_variant_serialised_get_child(
        container: GVariantSerialised,
        index: gsize,
    ) -> GVariantSerialised;
    fn g_variant_serialiser_needed_size(
        info: *mut GVariantTypeInfo,
        gsv_filler: GVariantSerialisedFiller,
        children: *const gpointer,
        n_children: gsize,
    ) -> gsize;
    fn g_variant_serialiser_serialise(
        container: GVariantSerialised,
        gsv_filler: GVariantSerialisedFiller,
        children: *const gpointer,
        n_children: gsize,
    );
    fn g_variant_serialised_check(serialised: GVariantSerialised) -> gboolean;
    fn g_variant_serialised_is_normal(value: GVariantSerialised) -> gboolean;
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
    fn g_bit_lock(address: *mut gint, lock_bit: gint);
    fn g_bit_unlock(address: *mut gint, lock_bit: gint);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_atomic_ref_count_compare(arc: *mut gatomicrefcount, val: gint) -> gboolean;
}
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type gatomicrefcount = gint;
pub type GVariantType = _GVariantType;
pub type GVariantTypeInfo = _GVariantTypeInfo;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariant {
    pub type_info: *mut GVariantTypeInfo,
    pub size: gsize,
    pub contents: C2RustUnnamed,
    pub state: gint,
    pub ref_count: gatomicrefcount,
    pub depth: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub serialised: C2RustUnnamed_1,
    pub tree: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub children: *mut *mut GVariant,
    pub n_children: gsize,
}
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub bytes: *mut GBytes,
    pub data: gconstpointer,
    pub ordered_offsets_up_to: gsize,
    pub checked_offsets_up_to: gsize,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVariantSerialised {
    pub type_info: *mut GVariantTypeInfo,
    pub data: *mut guchar,
    pub size: gsize,
    pub depth: gsize,
    pub ordered_offsets_up_to: gsize,
    pub checked_offsets_up_to: gsize,
}
pub type GVariantSerialisedFiller =
    Option<unsafe extern "C" fn(*mut GVariantSerialised, gpointer) -> ()>;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_MAX_RECURSION_DEPTH: gsize = 128 as ::core::ffi::c_int as gsize;
pub const STATE_LOCKED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STATE_SERIALISED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STATE_TRUSTED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STATE_FLOATING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_variant_lock(mut value: *mut GVariant) {
    g_bit_lock(&raw mut (*value).state as *mut gint, 0 as gint);
}
unsafe extern "C" fn safe_c2rust_g_variant_unlock(mut value: *mut GVariant) {
    g_bit_unlock(&raw mut (*value).state as *mut gint, 0 as gint);
}
unsafe extern "C" fn safe_c2rust_g_variant_release_children(mut value: *mut GVariant) {
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if (*value).state as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value->state & STATE_LOCKED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !((*value).state as ::core::ffi::c_int) & 2 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            296 as ::core::ffi::c_int,
            G_STRFUNC,
            b"~value->state & STATE_SERIALISED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as gsize;
    while i < (*value).contents.tree.n_children {
        safe_c2rust_g_variant_unref(*(*value).contents.tree.children.offset(i as isize));
        i = i.wrapping_add(1);
    }
    g_free((*value).contents.tree.children as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_variant_ensure_size(mut value: *mut GVariant) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*value).state as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            371 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value->state & STATE_LOCKED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*value).size == -(1 as ::core::ffi::c_int) as gsize {
        let mut children: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
        let mut n_children: gsize = 0;
        children = (*value).contents.tree.children as *mut gpointer;
        n_children = (*value).contents.tree.n_children;
        (*value).size = g_variant_serialiser_needed_size(
            (*value).type_info,
            Some(
                safe_c2rust_g_variant_fill_gvs
                    as unsafe extern "C" fn(*mut GVariantSerialised, gpointer) -> (),
            ),
            children,
            n_children,
        );
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_variant_to_serialised(
    mut value: *mut GVariant,
) -> GVariantSerialised {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*value).state as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            395 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value->state & STATE_SERIALISED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut serialised: GVariantSerialised = GVariantSerialised {
        type_info: (*value).type_info,
        data: (*value).contents.serialised.data as gpointer as *mut guchar,
        size: (*value).size,
        depth: (*value).depth,
        ordered_offsets_up_to: (*value).contents.serialised.ordered_offsets_up_to,
        checked_offsets_up_to: (*value).contents.serialised.checked_offsets_up_to,
    };
    return serialised;
}
unsafe extern "C" fn safe_c2rust_g_variant_serialise(mut value: *mut GVariant, mut data: gpointer) {
    let mut serialised: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut children: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    let mut n_children: gsize = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !((*value).state as ::core::ffi::c_int) & 2 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            428 as ::core::ffi::c_int,
            G_STRFUNC,
            b"~value->state & STATE_SERIALISED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*value).state as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            429 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value->state & STATE_LOCKED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    serialised.type_info = (*value).type_info;
    serialised.size = (*value).size;
    serialised.data = data as *mut guchar;
    serialised.depth = (*value).depth;
    serialised.ordered_offsets_up_to = 0 as gsize;
    serialised.checked_offsets_up_to = 0 as gsize;
    children = (*value).contents.tree.children as *mut gpointer;
    n_children = (*value).contents.tree.n_children;
    g_variant_serialiser_serialise(
        serialised,
        Some(
            safe_c2rust_g_variant_fill_gvs
                as unsafe extern "C" fn(*mut GVariantSerialised, gpointer) -> (),
        ),
        children,
        n_children,
    );
}
unsafe extern "C" fn safe_c2rust_g_variant_fill_gvs(
    mut serialised: *mut GVariantSerialised,
    mut data: gpointer,
) {
    let mut value: *mut GVariant = data as *mut GVariant;
    safe_c2rust_g_variant_lock(value);
    safe_c2rust_g_variant_ensure_size(value);
    safe_c2rust_g_variant_unlock(value);
    if (*serialised).type_info.is_null() {
        (*serialised).type_info = (*value).type_info;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*serialised).type_info == (*value).type_info {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            473 as ::core::ffi::c_int,
            G_STRFUNC,
            b"serialised->type_info == value->type_info\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*serialised).size == 0 as gsize {
        (*serialised).size = (*value).size;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*serialised).size == (*value).size {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            477 as ::core::ffi::c_int,
            G_STRFUNC,
            b"serialised->size == value->size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*serialised).depth = (*value).depth;
    if (*value).state as ::core::ffi::c_int & STATE_SERIALISED != 0 {
        (*serialised).ordered_offsets_up_to = (*value).contents.serialised.ordered_offsets_up_to;
        (*serialised).checked_offsets_up_to = (*value).contents.serialised.checked_offsets_up_to;
    } else {
        (*serialised).ordered_offsets_up_to = 0 as gsize;
        (*serialised).checked_offsets_up_to = 0 as gsize;
    }
    if !(*serialised).data.is_null() {
        safe_c2rust_g_variant_store(value, (*serialised).data as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_variant_ensure_serialised(mut value: *mut GVariant) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*value).state as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
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
            b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value->state & STATE_LOCKED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !((*value).state as ::core::ffi::c_int) & STATE_SERIALISED != 0 {
        let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        safe_c2rust_g_variant_ensure_size(value);
        data = g_malloc((*value).size);
        safe_c2rust_g_variant_serialise(value, data);
        safe_c2rust_g_variant_release_children(value);
        bytes = g_bytes_new_take(data, (*value).size);
        (*value).contents.serialised.data =
            g_bytes_get_data(bytes, ::core::ptr::null_mut::<gsize>());
        (*value).contents.serialised.bytes = bytes;
        (*value).contents.serialised.ordered_offsets_up_to = G_MAXSIZE as gsize;
        (*value).contents.serialised.checked_offsets_up_to = G_MAXSIZE as gsize;
        (*value).state |= STATE_SERIALISED;
    }
}
unsafe extern "C" fn safe_c2rust_g_variant_alloc(
    mut type_0: *const GVariantType,
    mut serialised: gboolean,
    mut trusted: gboolean,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = g_slice_alloc(::core::mem::size_of::<GVariant>() as gsize) as *mut GVariant;
    (*value).type_info = g_variant_type_info_get(type_0);
    (*value).state = ((if serialised != 0 {
        STATE_SERIALISED
    } else {
        0 as ::core::ffi::c_int
    }) | (if trusted != 0 {
        STATE_TRUSTED
    } else {
        0 as ::core::ffi::c_int
    }) | STATE_FLOATING) as gint;
    (*value).size = -(1 as ::core::ffi::c_int) as gssize as gsize;
    g_atomic_ref_count_init(&raw mut (*value).ref_count);
    (*value).depth = 0 as gsize;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_from_bytes(
    mut type_0: *const GVariantType,
    mut bytes: *mut GBytes,
    mut trusted: gboolean,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut alignment: guint = 0;
    let mut size: gsize = 0;
    let mut owned_bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut serialised: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    value = safe_c2rust_g_variant_alloc(type_0, TRUE, trusted);
    g_variant_type_info_query((*value).type_info, &raw mut alignment, &raw mut size);
    serialised.type_info = (*value).type_info;
    serialised.data = g_bytes_get_data(bytes, &raw mut serialised.size) as *mut guchar;
    serialised.depth = 0 as gsize;
    serialised.ordered_offsets_up_to = (if trusted != 0 {
        G_MAXSIZE
    } else {
        0 as ::core::ffi::c_ulong
    }) as gsize;
    serialised.checked_offsets_up_to = (if trusted != 0 {
        G_MAXSIZE
    } else {
        0 as ::core::ffi::c_ulong
    }) as gsize;
    if g_variant_serialised_check(serialised) == 0 {
        let mut aligned_data: gpointer = NULL;
        let mut aligned_size: gsize = g_bytes_get_size(bytes);
        if aligned_size != 0 as gsize
            && posix_memalign(
                &raw mut aligned_data,
                (if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                    > alignment.wrapping_add(1 as guint) as usize
                {
                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t
                } else {
                    alignment.wrapping_add(1 as guint) as size_t
                }),
                aligned_size as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"posix_memalign failed\0" as *const u8 as *const gchar,
            );
            loop {}
        }
        if aligned_size != 0 as gsize {
            memcpy(
                aligned_data as *mut ::core::ffi::c_void,
                g_bytes_get_data(bytes, ::core::ptr::null_mut::<gsize>())
                    as *const ::core::ffi::c_void,
                aligned_size as size_t,
            );
        }
        owned_bytes = g_bytes_new_with_free_func(
            aligned_data as gconstpointer,
            aligned_size,
            Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            aligned_data,
        );
        bytes = owned_bytes;
        aligned_data = NULL as gpointer;
    }
    (*value).contents.serialised.bytes = g_bytes_ref(bytes);
    if size != 0 && g_bytes_get_size(bytes) != size {
        (*value).contents.serialised.data = ::core::ptr::null::<::core::ffi::c_void>();
        (*value).size = size;
    } else {
        (*value).contents.serialised.data = g_bytes_get_data(bytes, &raw mut (*value).size);
    }
    (*value).contents.serialised.ordered_offsets_up_to = (if trusted != 0 {
        G_MAXSIZE
    } else {
        0 as ::core::ffi::c_ulong
    }) as gsize;
    (*value).contents.serialised.checked_offsets_up_to = (if trusted != 0 {
        G_MAXSIZE
    } else {
        0 as ::core::ffi::c_ulong
    }) as gsize;
    let mut _pp: *mut *mut GBytes = &raw mut owned_bytes;
    let mut _ptr: *mut GBytes = *_pp;
    *_pp = ::core::ptr::null_mut::<GBytes>();
    if !_ptr.is_null() {
        g_bytes_unref(_ptr as *mut GBytes);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_from_children(
    mut type_0: *const GVariantType,
    mut children: *mut *mut GVariant,
    mut n_children: gsize,
    mut trusted: gboolean,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = safe_c2rust_g_variant_alloc(type_0, FALSE, trusted);
    (*value).contents.tree.children = children;
    (*value).contents.tree.n_children = n_children;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_type_info(
    mut value: *mut GVariant,
) -> *mut GVariantTypeInfo {
    return (*value).type_info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_trusted(mut value: *mut GVariant) -> gboolean {
    return ((*value).state as ::core::ffi::c_int & STATE_TRUSTED != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_depth(mut value: *mut GVariant) -> gsize {
    return (*value).depth;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_unref(mut value: *mut GVariant) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*value).ref_count) != 0 {
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if (*value).state as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"attempting to free a locked GVariant instance.  This should never happen.\0"
                    as *const u8 as *const gchar,
            );
        }
        (*value).state |= STATE_LOCKED;
        g_variant_type_info_unref((*value).type_info);
        if (*value).state as ::core::ffi::c_int & STATE_SERIALISED != 0 {
            g_bytes_unref((*value).contents.serialised.bytes);
        } else {
            safe_c2rust_g_variant_release_children(value);
        }
        memset(
            value as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GVariant>() as size_t,
        );
        g_slice_free1(
            ::core::mem::size_of::<GVariant>() as gsize,
            value as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_ref(mut value: *mut GVariant) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_atomic_ref_count_inc(&raw mut (*value).ref_count);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_ref_sink(mut value: *mut GVariant) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if g_atomic_ref_count_compare(&raw mut (*value).ref_count, 0 as gint) == 0 {
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
            b"!g_atomic_ref_count_compare (&value->ref_count, 0)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_variant_lock(value);
    if !((*value).state as ::core::ffi::c_int) & STATE_FLOATING != 0 {
        safe_c2rust_g_variant_ref(value);
    } else {
        (*value).state &= !STATE_FLOATING;
    }
    safe_c2rust_g_variant_unlock(value);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_take_ref(mut value: *mut GVariant) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_atomic_ref_count_compare(&raw mut (*value).ref_count, 0 as gint) == 0 {
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
            b"!g_atomic_ref_count_compare (&value->ref_count, 0)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*value).state;
        !(8 as ::core::ffi::c_int);
    } else {
    };
    crate::translated::compat::atomic_and_seqcst(&raw mut (*value).state, !(8 as ::core::ffi::c_int));
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_floating(mut value: *mut GVariant) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*value).state as ::core::ffi::c_int & STATE_FLOATING != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_size(mut value: *mut GVariant) -> gsize {
    safe_c2rust_g_variant_lock(value);
    safe_c2rust_g_variant_ensure_size(value);
    safe_c2rust_g_variant_unlock(value);
    return (*value).size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_data(mut value: *mut GVariant) -> gconstpointer {
    safe_c2rust_g_variant_lock(value);
    safe_c2rust_g_variant_ensure_serialised(value);
    safe_c2rust_g_variant_unlock(value);
    return (*value).contents.serialised.data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_data_as_bytes(
    mut value: *mut GVariant,
) -> *mut GBytes {
    let mut bytes_data: *const gchar = ::core::ptr::null::<gchar>();
    let mut data: *const gchar = ::core::ptr::null::<gchar>();
    let mut bytes_size: gsize = 0;
    let mut size: gsize = 0;
    safe_c2rust_g_variant_lock(value);
    safe_c2rust_g_variant_ensure_serialised(value);
    safe_c2rust_g_variant_unlock(value);
    bytes_data =
        g_bytes_get_data((*value).contents.serialised.bytes, &raw mut bytes_size) as *const gchar;
    data = (*value).contents.serialised.data as *const gchar;
    size = (*value).size;
    if data.is_null() {
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if size == 0 as gsize {
                _g_boolean_var_25 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_25 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_25
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
                1065 as ::core::ffi::c_int,
                G_STRFUNC,
                b"size == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        data = bytes_data;
    }
    if data == bytes_data && size == bytes_size {
        return g_bytes_ref((*value).contents.serialised.bytes);
    } else {
        return g_bytes_new_from_bytes(
            (*value).contents.serialised.bytes,
            data.offset_from(bytes_data) as ::core::ffi::c_long as gsize,
            size,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_n_children(mut value: *mut GVariant) -> gsize {
    let mut n_children: gsize = 0;
    safe_c2rust_g_variant_lock(value);
    if (*value).state as ::core::ffi::c_int & STATE_SERIALISED != 0 {
        n_children = g_variant_serialised_n_children(safe_c2rust_g_variant_to_serialised(value));
    } else {
        n_children = (*value).contents.tree.n_children;
    }
    safe_c2rust_g_variant_unlock(value);
    return n_children;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_child_value(
    mut value: *mut GVariant,
    mut index_: gsize,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if (*value).depth
            < (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong)
        {
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
            b"value->depth < G_MAXSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if !({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*value).state;
            (*value).state;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*value).state);
        gaig_temp
    }) & STATE_SERIALISED
        != 0
    {
        if ({
            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
            if index_ < safe_c2rust_g_variant_n_children(value) {
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
                b"index_ < g_variant_n_children (value)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        safe_c2rust_g_variant_lock(value);
        if !((*value).state as ::core::ffi::c_int) & STATE_SERIALISED != 0 {
            let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            child =
                safe_c2rust_g_variant_ref(*(*value).contents.tree.children.offset(index_ as isize));
            safe_c2rust_g_variant_unlock(value);
            return child;
        }
        safe_c2rust_g_variant_unlock(value);
    }
    let mut serialised: GVariantSerialised = safe_c2rust_g_variant_to_serialised(value);
    let mut s_child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut child_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    s_child = g_variant_serialised_get_child(serialised, index_);
    (*value).contents.serialised.ordered_offsets_up_to =
        if (*value).contents.serialised.ordered_offsets_up_to > serialised.ordered_offsets_up_to {
            (*value).contents.serialised.ordered_offsets_up_to
        } else {
            serialised.ordered_offsets_up_to
        };
    (*value).contents.serialised.checked_offsets_up_to =
        if (*value).contents.serialised.checked_offsets_up_to > serialised.checked_offsets_up_to {
            (*value).contents.serialised.checked_offsets_up_to
        } else {
            serialised.checked_offsets_up_to
        };
    if (*value).state as ::core::ffi::c_int & STATE_TRUSTED == 0
        && g_variant_type_info_query_depth(s_child.type_info)
            >= G_VARIANT_MAX_RECURSION_DEPTH.wrapping_sub((*value).depth)
    {
        if ({
            let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
            if g_variant_is_of_type(
                value,
                b"v\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
            {
                _g_boolean_var_28 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_28 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_28
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant-core.c\0" as *const u8 as *const ::core::ffi::c_char,
                1198 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_variant_is_of_type (value, G_VARIANT_TYPE_VARIANT)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_variant_type_info_unref(s_child.type_info);
        return g_variant_new_tuple(::core::ptr::null::<*mut GVariant>(), 0 as gsize);
    }
    child_0 = g_slice_alloc(::core::mem::size_of::<GVariant>() as gsize) as *mut GVariant;
    (*child_0).type_info = s_child.type_info;
    (*child_0).state =
        ((*value).state as ::core::ffi::c_int & STATE_TRUSTED | STATE_SERIALISED) as gint;
    (*child_0).size = s_child.size;
    g_atomic_ref_count_init(&raw mut (*child_0).ref_count);
    (*child_0).depth = (*value).depth.wrapping_add(1 as gsize);
    (*child_0).contents.serialised.bytes = g_bytes_ref((*value).contents.serialised.bytes);
    (*child_0).contents.serialised.data = s_child.data as gconstpointer;
    (*child_0).contents.serialised.ordered_offsets_up_to =
        (if (*value).state as ::core::ffi::c_int & STATE_TRUSTED != 0 {
            G_MAXSIZE
        } else {
            s_child.ordered_offsets_up_to as ::core::ffi::c_ulong
        }) as gsize;
    (*child_0).contents.serialised.checked_offsets_up_to =
        (if (*value).state as ::core::ffi::c_int & STATE_TRUSTED != 0 {
            G_MAXSIZE
        } else {
            s_child.checked_offsets_up_to as ::core::ffi::c_ulong
        }) as gsize;
    return child_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_maybe_get_child_value(
    mut value: *mut GVariant,
    mut index_: gsize,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*value).depth
            < (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong)
        {
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
            b"value->depth < G_MAXSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if !({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*value).state;
            (*value).state;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*value).state);
        gaig_temp
    }) & STATE_SERIALISED
        != 0
    {
        if ({
            let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
            if index_ < safe_c2rust_g_variant_n_children(value) {
                _g_boolean_var_30 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_30 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_30
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"index_ < g_variant_n_children (value)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
        safe_c2rust_g_variant_lock(value);
        if !((*value).state as ::core::ffi::c_int) & STATE_SERIALISED != 0 {
            let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            child =
                safe_c2rust_g_variant_ref(*(*value).contents.tree.children.offset(index_ as isize));
            safe_c2rust_g_variant_unlock(value);
            return child;
        }
        safe_c2rust_g_variant_unlock(value);
    }
    let mut serialised: GVariantSerialised = safe_c2rust_g_variant_to_serialised(value);
    let mut s_child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    s_child = g_variant_serialised_get_child(serialised, index_);
    if (*value).state as ::core::ffi::c_int & STATE_TRUSTED == 0 && s_child.data.is_null() {
        g_variant_type_info_unref(s_child.type_info);
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_variant_type_info_unref(s_child.type_info);
    return safe_c2rust_g_variant_get_child_value(value, index_);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_store(mut value: *mut GVariant, mut data: gpointer) {
    safe_c2rust_g_variant_lock(value);
    if (*value).state as ::core::ffi::c_int & STATE_SERIALISED != 0 {
        if !(*value).contents.serialised.data.is_null() {
            memcpy(
                data as *mut ::core::ffi::c_void,
                (*value).contents.serialised.data as *const ::core::ffi::c_void,
                (*value).size as size_t,
            );
        } else {
            memset(
                data as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*value).size as size_t,
            );
        }
    } else {
        safe_c2rust_g_variant_serialise(value, data);
    }
    safe_c2rust_g_variant_unlock(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_normal_form(
    mut value: *mut GVariant,
) -> gboolean {
    if (*value).state as ::core::ffi::c_int & STATE_TRUSTED != 0 {
        return TRUE;
    }
    safe_c2rust_g_variant_lock(value);
    if (*value).depth >= G_VARIANT_MAX_RECURSION_DEPTH {
        return FALSE;
    }
    if (*value).state as ::core::ffi::c_int & STATE_SERIALISED != 0 {
        if g_variant_serialised_is_normal(safe_c2rust_g_variant_to_serialised(value)) != 0 {
            (*value).state |= STATE_TRUSTED;
        }
    } else {
        let mut normal: gboolean = TRUE;
        let mut i: gsize = 0;
        i = 0 as gsize;
        while i < (*value).contents.tree.n_children {
            normal &= safe_c2rust_g_variant_is_normal_form(
                *(*value).contents.tree.children.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        if normal != 0 {
            (*value).state |= STATE_TRUSTED;
        }
    }
    safe_c2rust_g_variant_unlock(value);
    return ((*value).state as ::core::ffi::c_int & STATE_TRUSTED != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_variant_release_children\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
