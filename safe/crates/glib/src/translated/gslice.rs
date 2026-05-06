extern "C" {
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
    fn g_free_sized(mem: gpointer, size: size_t);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    static mut safe_c2rust_g_mem_gc_friendly: gboolean;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint64 = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GSliceConfig = ::core::ffi::c_uint;
pub const G_SLICE_CONFIG_CONTENTION_COUNTER: GSliceConfig = 6;
pub const G_SLICE_CONFIG_CHUNK_SIZES: GSliceConfig = 5;
pub const G_SLICE_CONFIG_COLOR_INCREMENT: GSliceConfig = 4;
pub const G_SLICE_CONFIG_WORKING_SET_MSECS: GSliceConfig = 3;
pub const G_SLICE_CONFIG_BYPASS_MAGAZINES: GSliceConfig = 2;
pub const G_SLICE_CONFIG_ALWAYS_MALLOC: GSliceConfig = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_set_config(mut ckey: GSliceConfig, mut value: gint64) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_get_config(mut ckey: GSliceConfig) -> gint64 {
    return 0 as gint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_get_config_state(
    mut ckey: GSliceConfig,
    mut address: gint64,
    mut n_values: *mut guint,
) -> *mut gint64 {
    return ::core::ptr::null_mut::<gint64>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_alloc(mut mem_size: gsize) -> gpointer {
    let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    mem = g_malloc(mem_size);
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_alloc0(mut mem_size: gsize) -> gpointer {
    let mut mem: gpointer = safe_c2rust_g_slice_alloc(mem_size);
    if !mem.is_null() {
        memset(
            mem as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            mem_size as size_t,
        );
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_copy(
    mut mem_size: gsize,
    mut mem_block: gconstpointer,
) -> gpointer {
    let mut mem: gpointer = safe_c2rust_g_slice_alloc(mem_size);
    if !mem.is_null() {
        memcpy(
            mem as *mut ::core::ffi::c_void,
            mem_block as *const ::core::ffi::c_void,
            mem_size as size_t,
        );
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_free1(mut mem_size: gsize, mut mem_block: gpointer) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_g_mem_gc_friendly != 0 && !mem_block.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        memset(
            mem_block as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            mem_size as size_t,
        );
    }
    g_free_sized(mem_block, mem_size as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slice_free_chain_with_offset(
    mut mem_size: gsize,
    mut mem_chain: gpointer,
    mut next_offset: gsize,
) {
    let mut slice: gpointer = mem_chain;
    while !slice.is_null() {
        let mut current: *mut guint8 = slice as *mut guint8;
        slice = *(current.offset(next_offset as isize) as *mut gpointer);
        if ({
            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
            if safe_c2rust_g_mem_gc_friendly != 0 {
                _g_boolean_var_9 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_9 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_9
        }) as ::core::ffi::c_long
            != 0
        {
            memset(
                current as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                mem_size as size_t,
            );
        }
        g_free_sized(current as gpointer, mem_size as size_t);
    }
}
