extern "C" {
    fn posix_memalign(
        __memptr: *mut *mut ::core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemVTable {
    pub malloc: Option<unsafe extern "C" fn(gsize) -> gpointer>,
    pub realloc: Option<unsafe extern "C" fn(gpointer, gsize) -> gpointer>,
    pub free: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub calloc: Option<unsafe extern "C" fn(gsize, gsize) -> gpointer>,
    pub try_malloc: Option<unsafe extern "C" fn(gsize) -> gpointer>,
    pub try_realloc: Option<unsafe extern "C" fn(gpointer, gsize) -> gpointer>,
}
pub type GMemVTable = _GMemVTable;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

unsafe extern "C" fn safe_c2rust_libc_malloc_bridge(n: gsize) -> gpointer { malloc(n as size_t) }
unsafe extern "C" fn safe_c2rust_libc_realloc_bridge(ptr: gpointer, n: gsize) -> gpointer { realloc(ptr, n as size_t) }
unsafe extern "C" fn safe_c2rust_libc_calloc_bridge(n: gsize, size: gsize) -> gpointer { calloc(n as size_t, size as size_t) }
static mut safe_c2rust_glib_mem_vtable: GMemVTable = unsafe {
    _GMemVTable {
        malloc: Some(safe_c2rust_libc_malloc_bridge),
        realloc: Some(safe_c2rust_libc_realloc_bridge),
        free: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        calloc: Some(safe_c2rust_libc_calloc_bridge),
        try_malloc: Some(safe_c2rust_libc_malloc_bridge),
        try_realloc: Some(safe_c2rust_libc_realloc_bridge),
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_malloc(mut n_bytes: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        mem = malloc(n_bytes as size_t) as gpointer;
        if !mem.is_null() {
            return mem;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: failed to allocate %lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:106\0" as *const u8 as *const ::core::ffi::c_char,
            n_bytes,
        );
        loop {}
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_malloc0(mut n_bytes: gsize) -> gpointer {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        mem = calloc(1 as size_t, n_bytes as size_t) as gpointer;
        if !mem.is_null() {
            return mem;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: failed to allocate %lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:139\0" as *const u8 as *const ::core::ffi::c_char,
            n_bytes,
        );
        loop {}
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_realloc(mut mem: gpointer, mut n_bytes: gsize) -> gpointer {
    let mut newmem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        newmem = realloc(mem as *mut ::core::ffi::c_void, n_bytes as size_t) as gpointer;
        if !newmem.is_null() {
            return newmem;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: failed to allocate %lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:177\0" as *const u8 as *const ::core::ffi::c_char,
            n_bytes,
        );
        loop {}
    }
    free(mem as *mut ::core::ffi::c_void);
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_free(mut mem: gpointer) {
    free(mem as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_free_sized(
    mut mem: *mut ::core::ffi::c_void,
    mut size: size_t,
) {
    free(mem);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_clear_pointer(
    mut pp: *mut gpointer,
    mut destroy: GDestroyNotify,
) {
    let mut _p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    _p = *pp;
    if !_p.is_null() {
        *pp = NULL as gpointer;
        destroy.expect("non-null function pointer")(_p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_malloc(mut n_bytes: gsize) -> gpointer {
    let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        mem = malloc(n_bytes as size_t) as gpointer;
    } else {
        mem = NULL as gpointer;
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_malloc0(mut n_bytes: gsize) -> gpointer {
    let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        mem = calloc(1 as size_t, n_bytes as size_t) as gpointer;
    } else {
        mem = NULL as gpointer;
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_realloc(
    mut mem: gpointer,
    mut n_bytes: gsize,
) -> gpointer {
    let mut newmem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if n_bytes != 0 {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        newmem = realloc(mem as *mut ::core::ffi::c_void, n_bytes as size_t) as gpointer;
    } else {
        newmem = NULL as gpointer;
        free(mem as *mut ::core::ffi::c_void);
    }
    return newmem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_malloc_n(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: overflow allocating %lu*%lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:384\0" as *const u8 as *const ::core::ffi::c_char,
            n_blocks,
            n_block_bytes,
        );
        loop {}
    }
    return safe_c2rust_g_malloc(n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_malloc0_n(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: overflow allocating %lu*%lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:411\0" as *const u8 as *const ::core::ffi::c_char,
            n_blocks,
            n_block_bytes,
        );
        loop {}
    }
    return safe_c2rust_g_malloc0(n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_realloc_n(
    mut mem: gpointer,
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: overflow allocating %lu*%lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:440\0" as *const u8 as *const ::core::ffi::c_char,
            n_blocks,
            n_block_bytes,
        );
        loop {}
    }
    return safe_c2rust_g_realloc(mem, n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_malloc_n(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
        return NULL;
    }
    return safe_c2rust_g_try_malloc(n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_malloc0_n(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
        return NULL;
    }
    return safe_c2rust_g_try_malloc0(n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_try_realloc_n(
    mut mem: gpointer,
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
) -> gpointer {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
        return NULL;
    }
    return safe_c2rust_g_try_realloc(mem, n_blocks.wrapping_mul(n_block_bytes));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_is_system_malloc() -> gboolean {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_set_vtable(mut vtable: *mut GMemVTable) {
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"../original/glib/gmem.c:546: custom memory allocation vtable not supported\0" as *const u8
            as *const gchar,
    );
}
#[no_mangle]
pub static mut safe_c2rust_glib_mem_profiler_table: *mut GMemVTable =
    unsafe { &raw const safe_c2rust_glib_mem_vtable as *mut GMemVTable };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_profile() {
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"../original/glib/gmem.c:572: memory profiling not supported\0" as *const u8
            as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_aligned_alloc(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
    mut alignment: gsize,
) -> gpointer {
    let mut res: gpointer = NULL;
    let mut real_size: gsize = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if alignment == 0 as gsize || alignment & alignment.wrapping_sub(1 as gsize) != 0 as gsize {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: alignment %lu must be a positive power of two\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:608\0" as *const u8 as *const ::core::ffi::c_char,
            alignment,
        );
        loop {}
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (alignment as usize)
            .wrapping_rem(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            != 0 as usize
        {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: alignment %lu must be a multiple of %lu\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:614\0" as *const u8 as *const ::core::ffi::c_char,
            alignment,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
        );
        loop {}
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if n_block_bytes > 0 as gsize
            && n_blocks
                > (9223372036854775807 as ::core::ffi::c_long as gsize)
                    .wrapping_mul(2 as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_div(n_block_bytes)
        {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: overflow allocating %lu*%lu bytes\0" as *const u8 as *const gchar,
            b"../original/glib/gmem.c:620\0" as *const u8 as *const ::core::ffi::c_char,
            n_blocks,
            n_block_bytes,
        );
        loop {}
    }
    real_size = n_blocks.wrapping_mul(n_block_bytes);
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if real_size == 0 as gsize {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
        return NULL;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    *__errno_location() = posix_memalign(&raw mut res, alignment as size_t, real_size as size_t);
    if !res.is_null() {
        return res;
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_ERROR,
        b"%s: failed to allocate %lu bytes\0" as *const u8 as *const gchar,
        b"../original/glib/gmem.c:670\0" as *const u8 as *const ::core::ffi::c_char,
        real_size,
    );
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_aligned_alloc0(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
    mut alignment: gsize,
) -> gpointer {
    let mut res: gpointer = safe_c2rust_g_aligned_alloc(n_blocks, n_block_bytes, alignment);
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !res.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
        memset(
            res as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (n_blocks as size_t).wrapping_mul(n_block_bytes as size_t),
        );
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_aligned_free(mut mem: gpointer) {
    free(mem as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_aligned_free_sized(
    mut mem: *mut ::core::ffi::c_void,
    mut alignment: size_t,
    mut size: size_t,
) {
    free(mem);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
