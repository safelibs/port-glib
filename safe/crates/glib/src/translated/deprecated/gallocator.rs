extern "C" {
    pub type _GAllocator;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_alloc0(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
}
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GAllocator = _GAllocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemChunk {
    pub alloc_size: guint,
}
pub type GMemChunk = _GMemChunk;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_new(
    mut name: *const gchar,
    mut atom_size: gint,
    mut area_size: gsize,
    mut type_0: gint,
) -> *mut GMemChunk {
    let mut mem_chunk: *mut GMemChunk = ::core::ptr::null_mut::<GMemChunk>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if atom_size > 0 as ::core::ffi::c_int {
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
            b"atom_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMemChunk>();
    }
    mem_chunk = g_slice_alloc(::core::mem::size_of::<GMemChunk>() as gsize) as *mut GMemChunk;
    (*mem_chunk).alloc_size = atom_size as guint;
    return mem_chunk;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_destroy(mut mem_chunk: *mut GMemChunk) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !mem_chunk.is_null() {
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
            b"mem_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_slice_free1(
        ::core::mem::size_of::<GMemChunk>() as gsize,
        mem_chunk as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_alloc(mut mem_chunk: *mut GMemChunk) -> gpointer {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !mem_chunk.is_null() {
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
            b"mem_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_slice_alloc((*mem_chunk).alloc_size as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_alloc0(mut mem_chunk: *mut GMemChunk) -> gpointer {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !mem_chunk.is_null() {
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
            b"mem_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_slice_alloc0((*mem_chunk).alloc_size as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_free(
    mut mem_chunk: *mut GMemChunk,
    mut mem: gpointer,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !mem_chunk.is_null() {
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
            b"mem_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_slice_free1((*mem_chunk).alloc_size as gsize, mem);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_allocator_new(
    mut name: *const gchar,
    mut n_preallocs: guint,
) -> *mut GAllocator {
    return 1 as ::core::ffi::c_int as *mut ::core::ffi::c_void as *mut GAllocator;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_allocator_free(mut allocator: *mut GAllocator) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_clean(mut mem_chunk: *mut GMemChunk) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_reset(mut mem_chunk: *mut GMemChunk) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_print(mut mem_chunk: *mut GMemChunk) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mem_chunk_info() {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_blow_chunks() {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_push_allocator(mut allocator: *mut GAllocator) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_pop_allocator() {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_push_allocator(mut allocator: *mut GAllocator) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_pop_allocator() {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_push_allocator(mut allocator: *mut GAllocator) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_pop_allocator() {}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_mem_chunk_new\0" as *const u8 as *const ::core::ffi::c_char;
