use ::libc;
extern "C" {
    pub type _GHashTable;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_pointer_bit_lock_and_get(address: gpointer, lock_bit: guint, out_ptr: *mut guintptr);
    fn g_pointer_bit_unlock(address: *mut ::core::ffi::c_void, lock_bit: gint);
    fn g_pointer_bit_unlock_and_set(
        address: *mut ::core::ffi::c_void,
        lock_bit: guint,
        ptr: gpointer,
        preserve_mask: guintptr,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GData {
    pub len: guint32,
    pub alloc: guint32,
    pub data: [GDataElt; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDataElt {
    pub key: GQuark,
    pub data: gpointer,
    pub destroy: GDestroyNotify,
}
pub type GData = _GData;
pub type GDataForeachFunc = Option<unsafe extern "C" fn(GQuark, gpointer, gpointer) -> ()>;
pub type GDuplicateFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> gpointer>;
pub type GDataset = _GDataset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataset {
    pub location: gconstpointer,
    pub datalist: *mut GData,
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GHashTable = _GHashTable;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
pub const G_DATALIST_FLAGS_MASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_nearest_pow(mut num: gsize) -> gsize {
    let mut n: gsize = num.wrapping_sub(1 as gsize);
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if num > 0 as gsize
            && num
                <= (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
        {
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
            b"../original/glib/gutilsprivate.h\0" as *const u8 as *const ::core::ffi::c_char,
            44 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num > 0 && num <= G_MAXSIZE / 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as gsize);
}
pub const G_DATALIST_FLAGS_MASK_INTERNAL: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
static mut safe_c2rust_g__g_dataset_global_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_g_dataset_location_ht: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_g_dataset_cached: *mut GDataset =
    ::core::ptr::null::<GDataset>() as *mut GDataset;
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_datalist_lock_and_get(
    mut datalist: *mut *mut GData,
) -> *mut GData {
    let mut ptr: guintptr = 0;
    g_pointer_bit_lock_and_get(
        datalist as *mut *mut ::core::ffi::c_void as gpointer,
        2 as guint,
        &raw mut ptr,
    );
    return (ptr & !(G_DATALIST_FLAGS_MASK_INTERNAL as guintptr)) as gpointer as *mut GData;
}
unsafe extern "C" fn safe_c2rust_g_datalist_unlock(mut datalist: *mut *mut GData) {
    g_pointer_bit_unlock(
        datalist as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
        2 as gint,
    );
}
unsafe extern "C" fn safe_c2rust_g_datalist_unlock_and_set(
    mut datalist: *mut *mut GData,
    mut ptr: gpointer,
) {
    g_pointer_bit_unlock_and_set(
        datalist as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
        2 as guint,
        ptr,
        0x7 as guintptr,
    );
}
unsafe extern "C" fn safe_c2rust_datalist_append(
    mut data: *mut *mut GData,
    mut key_id: GQuark,
    mut new_data: gpointer,
    mut destroy_func: GDestroyNotify,
) -> gboolean {
    let mut reallocated: gboolean = 0;
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    d = *data;
    if d.is_null() {
        d =
            g_malloc((8 as ::core::ffi::c_ulong as glong as gsize).wrapping_add(
                (2 as gsize).wrapping_mul(::core::mem::size_of::<GDataElt>() as gsize),
            )) as *mut GData;
        (*d).len = 0 as guint32;
        (*d).alloc = 2 as ::core::ffi::c_uint as guint32;
        *data = d;
        reallocated = TRUE as gboolean;
    } else if (*d).len == (*d).alloc {
        (*d).alloc =
            ((*d).alloc as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint) as guint32;
        d = g_realloc(
            d as gpointer,
            (8 as ::core::ffi::c_ulong as glong as gsize).wrapping_add(
                ((*d).alloc as gsize).wrapping_mul(::core::mem::size_of::<GDataElt>() as gsize),
            ),
        ) as *mut GData;
        *data = d;
        reallocated = TRUE as gboolean;
    } else {
        reallocated = FALSE as gboolean;
    }
    *(&raw mut (*d).data as *mut GDataElt).offset((*d).len as isize) = GDataElt {
        key: key_id,
        data: new_data,
        destroy: destroy_func,
    };
    (*d).len = (*d).len.wrapping_add(1);
    return reallocated;
}
unsafe extern "C" fn safe_c2rust_datalist_remove(mut data: *mut GData, mut idx: guint32) {
    (*data).len = (*data).len.wrapping_sub(1);
    if idx != (*data).len {
        *(&raw mut (*data).data as *mut GDataElt).offset(idx as isize) =
            *(&raw mut (*data).data as *mut GDataElt).offset((*data).len as isize);
    }
}
unsafe extern "C" fn safe_c2rust_datalist_shrink(
    mut data: *mut *mut GData,
    mut d_to_free: *mut *mut GData,
) -> gboolean {
    let mut alloc_by_4: guint32 = 0;
    let mut v: guint32 = 0;
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    d = *data;
    alloc_by_4 =
        ((*d).alloc as ::core::ffi::c_uint).wrapping_div(4 as ::core::ffi::c_uint) as guint32;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*d).len > alloc_by_4 {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        return FALSE;
    }
    if (*d).len == 0 as guint32 {
        *d_to_free = d;
        *data = ::core::ptr::null_mut::<GData>();
        return TRUE;
    }
    v = (*d).len;
    if v != alloc_by_4 {
        v = safe_c2rust_g_nearest_pow(v as gsize) as guint32;
    }
    v = (v as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint) as guint32 as guint32;
    (*d).alloc = v;
    d = g_realloc(
        d as gpointer,
        (8 as ::core::ffi::c_ulong as glong as gsize)
            .wrapping_add((v as gsize).wrapping_mul(::core::mem::size_of::<GDataElt>() as gsize)),
    ) as *mut GData;
    *d_to_free = ::core::ptr::null_mut::<GData>();
    *data = d;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_datalist_find(
    mut data: *mut GData,
    mut key_id: GQuark,
    mut out_idx: *mut guint32,
) -> *mut GDataElt {
    let mut i: guint32 = 0;
    if !data.is_null() {
        i = 0 as guint32;
        while i < (*data).len {
            let mut data_elt: *mut GDataElt =
                (&raw mut (*data).data as *mut GDataElt).offset(i as isize) as *mut GDataElt;
            if (*data_elt).key == key_id {
                if !out_idx.is_null() {
                    *out_idx = i;
                }
                return data_elt;
            }
            i = i.wrapping_add(1);
        }
    }
    if !out_idx.is_null() {
        *out_idx = G_MAXUINT32;
    }
    return ::core::ptr::null_mut::<GDataElt>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_clear(mut datalist: *mut *mut GData) {
    let mut data: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = safe_c2rust_g_datalist_lock_and_get(datalist);
    if data.is_null() {
        safe_c2rust_g_datalist_unlock(datalist);
        return;
    }
    safe_c2rust_g_datalist_unlock_and_set(datalist, NULL);
    i = 0 as guint;
    while i < (*data).len {
        if !(*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
            .data
            .is_null()
            && (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
                .destroy
                .is_some()
        {
            (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
                .destroy
                .expect("non-null function pointer")(
                (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize)).data,
            );
        }
        i = i.wrapping_add(1);
    }
    g_free(data as gpointer);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_dataset_lookup(
    mut dataset_location: gconstpointer,
) -> *mut GDataset {
    let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
    if !safe_c2rust_g_dataset_cached.is_null()
        && (*safe_c2rust_g_dataset_cached).location == dataset_location
    {
        return safe_c2rust_g_dataset_cached;
    }
    dataset =
        g_hash_table_lookup(safe_c2rust_g_dataset_location_ht, dataset_location) as *mut GDataset;
    if !dataset.is_null() {
        safe_c2rust_g_dataset_cached = dataset;
    }
    return dataset;
}
unsafe extern "C" fn safe_c2rust_g_dataset_destroy_internal(mut dataset: *mut GDataset) {
    let mut dataset_location: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    dataset_location = (*dataset).location;
    while !dataset.is_null() {
        let mut data: *mut GData = ::core::ptr::null_mut::<GData>();
        let mut i: guint = 0;
        data = (({
            let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
            let mut gapg_temp_atomic: *mut *mut GData = &raw mut (*dataset).datalist;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as guintptr
            & !(G_DATALIST_FLAGS_MASK_INTERNAL as guintptr)) as gpointer
            as *mut GData;
        if data.is_null() {
            if dataset == safe_c2rust_g_dataset_cached {
                safe_c2rust_g_dataset_cached = ::core::ptr::null_mut::<GDataset>();
            }
            g_hash_table_remove(safe_c2rust_g_dataset_location_ht, dataset_location);
            g_slice_free1(
                ::core::mem::size_of::<GDataset>() as gsize,
                dataset as gpointer,
            );
            break;
        } else {
            let mut _oldv: gpointer = ({
                let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
                let mut gapg_temp_atomic: *mut *mut GData = &raw mut (*dataset).datalist;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) as gpointer;
            let mut _newv: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            loop {
                _newv = (_oldv as guintptr & G_DATALIST_FLAGS_MASK_INTERNAL as guintptr
                    | ::core::ptr::null_mut::<::core::ffi::c_void>() as guintptr)
                    as gpointer;
                if !(({
                    if 0 as ::core::ffi::c_int != 0 {
                        *(&raw mut (*dataset).datalist as *mut *mut ::core::ffi::c_void);
                    } else {
                    };
                    if 0 as ::core::ffi::c_int != 0 {
                        _oldv;
                    } else {
                    };
                    _oldv = _oldv;
                    let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                        &raw mut (*dataset).datalist as *mut *mut ::core::ffi::c_void,
                        *&raw mut _oldv,
                        _newv,
                    );
                    *&raw mut _oldv = fresh0.0;
                    if fresh0.1 as ::core::ffi::c_int != 0 {
                        TRUE
                    } else {
                        FALSE
                    }
                }) == 0)
                {
                    break;
                }
            }
            g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
            i = 0 as guint;
            while i < (*data).len {
                if !(*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
                    .data
                    .is_null()
                    && (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
                        .destroy
                        .is_some()
                {
                    (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize))
                        .destroy
                        .expect("non-null function pointer")(
                        (*(&raw mut (*data).data as *mut GDataElt).offset(i as isize)).data,
                    );
                }
                i = i.wrapping_add(1);
            }
            g_free(data as gpointer);
            g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
            dataset = safe_c2rust_g_dataset_lookup(dataset_location);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dataset_destroy(mut dataset_location: gconstpointer) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !dataset_location.is_null() {
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
            b"dataset_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    if !safe_c2rust_g_dataset_location_ht.is_null() {
        let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
        dataset = safe_c2rust_g_dataset_lookup(dataset_location);
        if !dataset.is_null() {
            safe_c2rust_g_dataset_destroy_internal(dataset);
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_data_set_internal(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
    mut new_data: gpointer,
    mut new_destroy_func: GDestroyNotify,
    mut dataset: *mut GDataset,
) -> gpointer {
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut new_d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut old: GDataElt = GDataElt {
        key: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        destroy: None,
    };
    let mut data: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut idx: guint32 = 0;
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    data = safe_c2rust_datalist_find(d, key_id, &raw mut idx);
    if new_data.is_null() {
        if !data.is_null() {
            let mut d_to_free: *mut GData = ::core::ptr::null_mut::<GData>();
            old = *data;
            safe_c2rust_datalist_remove(d, idx);
            if safe_c2rust_datalist_shrink(&raw mut d, &raw mut d_to_free) != 0 {
                safe_c2rust_g_datalist_unlock_and_set(datalist, d as gpointer);
                if !dataset.is_null() && d.is_null() {
                    safe_c2rust_g_dataset_destroy_internal(dataset);
                }
                if !d_to_free.is_null() {
                    g_free(d_to_free as gpointer);
                }
            } else {
                safe_c2rust_g_datalist_unlock(datalist);
            }
            if old.destroy.is_some() && new_destroy_func.is_none() {
                if !dataset.is_null() {
                    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
                }
                old.destroy.expect("non-null function pointer")(old.data);
                if !dataset.is_null() {
                    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
                }
                old.data = NULL as gpointer;
            }
            return old.data;
        }
    } else {
        if !data.is_null() {
            if (*data).destroy.is_none() {
                (*data).data = new_data;
                (*data).destroy = new_destroy_func;
                safe_c2rust_g_datalist_unlock(datalist);
            } else {
                old = *data;
                (*data).data = new_data;
                (*data).destroy = new_destroy_func;
                safe_c2rust_g_datalist_unlock(datalist);
                if !dataset.is_null() {
                    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
                }
                old.destroy.expect("non-null function pointer")(old.data);
                if !dataset.is_null() {
                    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
                }
            }
            return NULL;
        }
        if safe_c2rust_datalist_append(&raw mut d, key_id, new_data, new_destroy_func) != 0 {
            new_d = d;
        }
    }
    if !new_d.is_null() {
        safe_c2rust_g_datalist_unlock_and_set(datalist, new_d as gpointer);
    } else {
        safe_c2rust_g_datalist_unlock(datalist);
    }
    return NULL;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_data_remove_internal(
    mut datalist: *mut *mut GData,
    mut keys: *mut GQuark,
    mut n_keys: gsize,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut old: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut old_to_free: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut d_to_free: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut found_keys: gsize = 0;
    let mut i_keys: gsize = 0;
    let mut i_data: guint32 = 0;
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    if d.is_null() {
        safe_c2rust_g_datalist_unlock(datalist);
        return;
    }
    if n_keys as usize <= (400 as usize).wrapping_div(::core::mem::size_of::<GDataElt>() as usize) {
        old = (if (::core::mem::size_of::<GDataElt>() as usize).wrapping_mul(n_keys as usize)
            == 0 as usize
        {
            NULL
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (::core::mem::size_of::<GDataElt>() as usize).wrapping_mul(n_keys as usize)
                    as usize,
            ));
            memset(
                alloca_allocations.last_mut().unwrap().as_mut_ptr().cast(),
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<GDataElt>() as size_t).wrapping_mul(n_keys as size_t),
            )
        }) as *mut GDataElt;
    } else {
        old_to_free = ({
            let mut __n: gsize = n_keys;
            let mut __s: gsize = ::core::mem::size_of::<GDataElt>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut GDataElt;
        old = old_to_free;
    }
    i_data = 0 as guint32;
    found_keys = 0 as gsize;
    while i_data < (*d).len && found_keys < n_keys {
        let mut data: *mut GDataElt =
            (&raw mut (*d).data as *mut GDataElt).offset(i_data as isize) as *mut GDataElt;
        let mut remove: gboolean = FALSE;
        i_keys = 0 as gsize;
        while i_keys < n_keys {
            if (*data).key == *keys.offset(i_keys as isize) {
                *old.offset(i_keys as isize) = *data;
                found_keys = found_keys.wrapping_add(1);
                remove = TRUE as gboolean;
                break;
            } else {
                i_keys = i_keys.wrapping_add(1);
            }
        }
        if remove == 0 {
            i_data = i_data.wrapping_add(1);
        } else {
            safe_c2rust_datalist_remove(d, i_data);
        }
    }
    if found_keys > 0 as gsize && safe_c2rust_datalist_shrink(&raw mut d, &raw mut d_to_free) != 0 {
        safe_c2rust_g_datalist_unlock_and_set(datalist, d as gpointer);
        if !d_to_free.is_null() {
            g_free(d_to_free as gpointer);
        }
    } else {
        safe_c2rust_g_datalist_unlock(datalist);
    }
    if found_keys > 0 as gsize {
        i_keys = 0 as gsize;
        while i_keys < n_keys {
            if (*old.offset(i_keys as isize)).destroy.is_some() {
                (*old.offset(i_keys as isize))
                    .destroy
                    .expect("non-null function pointer")(
                    (*old.offset(i_keys as isize)).data
                );
            }
            i_keys = i_keys.wrapping_add(1);
        }
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !old_to_free.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        g_free(old_to_free as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dataset_id_set_data_full(
    mut dataset_location: gconstpointer,
    mut key_id: GQuark,
    mut data: gpointer,
    mut destroy_func: GDestroyNotify,
) {
    let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !dataset_location.is_null() {
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
            b"dataset_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if data.is_null() {
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if destroy_func.is_none() {
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
                b"destroy_func == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if key_id == 0 {
        if !data.is_null() {
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if key_id > 0 as GQuark {
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
                    b"key_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
        } else {
            return;
        }
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    if safe_c2rust_g_dataset_location_ht.is_null() {
        safe_c2rust_g_data_initialize();
    }
    dataset = safe_c2rust_g_dataset_lookup(dataset_location);
    if dataset.is_null() {
        dataset = g_slice_alloc(::core::mem::size_of::<GDataset>() as gsize) as *mut GDataset;
        (*dataset).location = dataset_location;
        safe_c2rust_g_datalist_init(&raw mut (*dataset).datalist);
        g_hash_table_insert(
            safe_c2rust_g_dataset_location_ht,
            (*dataset).location as gpointer,
            dataset as gpointer,
        );
    }
    safe_c2rust_g_data_set_internal(
        &raw mut (*dataset).datalist,
        key_id,
        data,
        destroy_func,
        dataset,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_set_data_full(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
    mut data: gpointer,
    mut destroy_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if data.is_null() {
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if destroy_func.is_none() {
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
                b"destroy_func == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if key_id == 0 {
        if !data.is_null() {
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if key_id > 0 as GQuark {
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
                    b"key_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
        } else {
            return;
        }
    }
    safe_c2rust_g_data_set_internal(
        datalist,
        key_id,
        data,
        destroy_func,
        ::core::ptr::null_mut::<GDataset>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_remove_multiple(
    mut datalist: *mut *mut GData,
    mut keys: *mut GQuark,
    mut n_keys: gsize,
) {
    safe_c2rust_g_data_remove_internal(datalist, keys, n_keys);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dataset_id_remove_no_notify(
    mut dataset_location: gconstpointer,
    mut key_id: GQuark,
) -> gpointer {
    let mut ret_data: gpointer = NULL;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !dataset_location.is_null() {
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
            b"dataset_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    if key_id != 0 && !safe_c2rust_g_dataset_location_ht.is_null() {
        let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
        dataset = safe_c2rust_g_dataset_lookup(dataset_location);
        if !dataset.is_null() {
            ret_data = safe_c2rust_g_data_set_internal(
                &raw mut (*dataset).datalist,
                key_id,
                NULL,
                ::core::mem::transmute::<::libc::intptr_t, GDestroyNotify>(
                    42 as ::core::ffi::c_int as ::libc::intptr_t,
                ),
                dataset,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    return ret_data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_remove_no_notify(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
) -> gpointer {
    let mut ret_data: gpointer = NULL;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if key_id != 0 {
        ret_data = safe_c2rust_g_data_set_internal(
            datalist,
            key_id,
            NULL,
            ::core::mem::transmute::<::libc::intptr_t, GDestroyNotify>(
                42 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
            ::core::ptr::null_mut::<GDataset>(),
        );
    }
    return ret_data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_update_atomic(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
    mut callback: GDataListUpdateAtomicFunc,
    mut user_data: gpointer,
) -> gpointer {
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut data: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut new_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut result: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut new_destroy: GDestroyNotify = None;
    let mut idx: guint32 = 0;
    let mut to_unlock: gboolean = TRUE;
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    data = safe_c2rust_datalist_find(d, key_id, &raw mut idx);
    if !data.is_null() {
        new_data = (*data).data;
        new_destroy = (*data).destroy;
    } else {
        new_data = NULL as gpointer;
        new_destroy = None;
    }
    result = callback.expect("non-null function pointer")(
        key_id,
        &raw mut new_data,
        &raw mut new_destroy,
        user_data,
    );
    if !data.is_null() && new_data.is_null() {
        let mut d_to_free: *mut GData = ::core::ptr::null_mut::<GData>();
        safe_c2rust_datalist_remove(d, idx);
        if safe_c2rust_datalist_shrink(&raw mut d, &raw mut d_to_free) != 0 {
            safe_c2rust_g_datalist_unlock_and_set(datalist, d as gpointer);
            if !d_to_free.is_null() {
                g_free(d_to_free as gpointer);
            }
            to_unlock = FALSE as gboolean;
        }
    } else if !data.is_null() {
        (*data).data = new_data;
        (*data).destroy = new_destroy;
    } else if !(data.is_null() && new_data.is_null()) {
        if safe_c2rust_datalist_append(&raw mut d, key_id, new_data, new_destroy) != 0 {
            safe_c2rust_g_datalist_unlock_and_set(datalist, d as gpointer);
            to_unlock = FALSE as gboolean;
        }
    }
    if to_unlock != 0 {
        safe_c2rust_g_datalist_unlock(datalist);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dataset_id_get_data(
    mut dataset_location: gconstpointer,
    mut key_id: GQuark,
) -> gpointer {
    let mut retval: gpointer = NULL;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !dataset_location.is_null() {
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
            b"dataset_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    if key_id != 0 && !safe_c2rust_g_dataset_location_ht.is_null() {
        let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
        dataset = safe_c2rust_g_dataset_lookup(dataset_location);
        if !dataset.is_null() {
            retval = safe_c2rust_g_datalist_id_get_data(&raw mut (*dataset).datalist, key_id);
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_get_data(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
) -> gpointer {
    return safe_c2rust_g_datalist_id_dup_data(datalist, key_id, None, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_dup_data(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
    mut dup_func: GDuplicateFunc,
    mut user_data: gpointer,
) -> gpointer {
    let mut val: gpointer = NULL;
    let mut retval: gpointer = NULL;
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut data: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    data = safe_c2rust_datalist_find(d, key_id, ::core::ptr::null_mut::<guint32>());
    if !data.is_null() {
        val = (*data).data;
    }
    if dup_func.is_some() {
        retval = dup_func.expect("non-null function pointer")(val, user_data);
    } else {
        retval = val;
    }
    safe_c2rust_g_datalist_unlock(datalist);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_id_replace_data(
    mut datalist: *mut *mut GData,
    mut key_id: GQuark,
    mut oldval: gpointer,
    mut newval: gpointer,
    mut destroy: GDestroyNotify,
    mut old_destroy: *mut GDestroyNotify,
) -> gboolean {
    let mut val: gpointer = NULL;
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut data: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut d_to_free: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut set_d: gboolean = FALSE;
    let mut idx: guint32 = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if key_id != 0 as GQuark {
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
            b"key_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !old_destroy.is_null() {
        *old_destroy = None;
    }
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    data = safe_c2rust_datalist_find(d, key_id, &raw mut idx);
    if !data.is_null() {
        val = (*data).data;
        if val == oldval {
            if !old_destroy.is_null() {
                *old_destroy = (*data).destroy;
            }
            if !newval.is_null() {
                (*data).data = newval;
                (*data).destroy = destroy;
            } else {
                safe_c2rust_datalist_remove(d, idx);
                if safe_c2rust_datalist_shrink(&raw mut d, &raw mut d_to_free) != 0 {
                    set_d = TRUE as gboolean;
                }
            }
        }
    }
    if val.is_null() && oldval.is_null() && !newval.is_null() {
        if safe_c2rust_datalist_append(&raw mut d, key_id, newval, destroy) != 0 {
            set_d = TRUE as gboolean;
        }
    }
    if set_d != 0 {
        safe_c2rust_g_datalist_unlock_and_set(datalist, d as gpointer);
    } else {
        safe_c2rust_g_datalist_unlock(datalist);
    }
    if !d_to_free.is_null() {
        g_free(d_to_free as gpointer);
    }
    return (val == oldval) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_get_data(
    mut datalist: *mut *mut GData,
    mut key: *const gchar,
) -> gpointer {
    let mut res: gpointer = NULL;
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut data: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    let mut data_end: *mut GDataElt = ::core::ptr::null_mut::<GDataElt>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    d = safe_c2rust_g_datalist_lock_and_get(datalist);
    if !d.is_null() {
        data = &raw mut (*d).data as *mut GDataElt;
        data_end = data.offset((*d).len as isize);
        while data < data_end {
            if g_strcmp0(
                g_quark_to_string((*data).key) as *const ::core::ffi::c_char,
                key as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                res = (*data).data;
                break;
            } else {
                data = data.offset(1);
            }
        }
    }
    safe_c2rust_g_datalist_unlock(datalist);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dataset_foreach(
    mut dataset_location: gconstpointer,
    mut func: GDataForeachFunc,
    mut user_data: gpointer,
) {
    let mut dataset: *mut GDataset = ::core::ptr::null_mut::<GDataset>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !dataset_location.is_null() {
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
            b"dataset_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    if !safe_c2rust_g_dataset_location_ht.is_null() {
        dataset = safe_c2rust_g_dataset_lookup(dataset_location);
        g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
        if !dataset.is_null() {
            safe_c2rust_g_datalist_foreach(&raw mut (*dataset).datalist, func, user_data);
        }
    } else {
        g_mutex_unlock(&raw mut safe_c2rust_g__g_dataset_global_lock);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_foreach(
    mut datalist: *mut *mut GData,
    mut func: GDataForeachFunc,
    mut user_data: gpointer,
) {
    let mut d: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    let mut len: guint = 0;
    let mut keys: *mut GQuark = ::core::ptr::null_mut::<GQuark>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    d = (({
        let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
        let mut gapg_temp_atomic: *mut *mut GData = datalist as *mut *mut GData;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as guintptr
        & !(G_DATALIST_FLAGS_MASK_INTERNAL as guintptr)) as gpointer as *mut GData;
    if d.is_null() {
        return;
    }
    len = (*d).len as guint;
    keys = ({
        let mut __n: gsize = len as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GQuark>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GQuark;
    i = 0 as guint;
    while i < len {
        *keys.offset(i as isize) = (*(&raw mut (*d).data as *mut GDataElt).offset(i as isize)).key;
        i = i.wrapping_add(1);
    }
    i = 0 as guint;
    while i < len {
        d = (({
            let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
            let mut gapg_temp_atomic: *mut *mut GData = datalist as *mut *mut GData;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as guintptr
            & !(G_DATALIST_FLAGS_MASK_INTERNAL as guintptr)) as gpointer as *mut GData;
        if d.is_null() {
            break;
        }
        j = 0 as guint;
        while j < (*d).len {
            if (*(&raw mut (*d).data as *mut GDataElt).offset(j as isize)).key
                == *keys.offset(i as isize)
            {
                func.expect("non-null function pointer")(
                    (*(&raw mut (*d).data as *mut GDataElt).offset(i as isize)).key,
                    (*(&raw mut (*d).data as *mut GDataElt).offset(i as isize)).data,
                    user_data,
                );
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    g_free(keys as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_init(mut datalist: *mut *mut GData) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut gaps_temp_atomic: *mut *mut GData = datalist as *mut *mut GData;
    let mut gaps_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
    if 0 as ::core::ffi::c_int != 0 {
        *datalist;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_set_flags(
    mut datalist: *mut *mut GData,
    mut flags: guint,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
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
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if flags & !(0x3 as ::core::ffi::c_int) as guint == 0 as guint {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DATALIST_FLAGS_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut gapo_atomic: *mut guintptr = datalist as *mut guintptr;
    if 0 as ::core::ffi::c_int != 0 {
        *datalist;
    } else {
    };
    if 0 as ::core::ffi::c_int != 0 {
    } else {
    };
    crate::translated::compat::atomic_or_seqcst(gapo_atomic, flags as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_unset_flags(
    mut datalist: *mut *mut GData,
    mut flags: guint,
) {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if flags & !(0x3 as ::core::ffi::c_int) as guint == 0 as guint {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DATALIST_FLAGS_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut gapa_atomic: *mut guintptr = datalist as *mut guintptr;
    if 0 as ::core::ffi::c_int != 0 {
        *datalist;
    } else {
    };
    if 0 as ::core::ffi::c_int != 0 {
        !(flags as gsize);
        !(flags as gsize);
    } else {
    };
    crate::translated::compat::atomic_and_seqcst(gapa_atomic, !(flags as gsize));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datalist_get_flags(mut datalist: *mut *mut GData) -> guint {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !datalist.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"datalist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (({
        let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
        let mut gapg_temp_atomic: *mut *mut GData = datalist as *mut *mut GData;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as gsize
        & G_DATALIST_FLAGS_MASK as gsize) as guint;
}
unsafe extern "C" fn safe_c2rust_g_data_initialize() {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if safe_c2rust_g_dataset_location_ht.is_null() {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_dataset_location_ht == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_dataset_location_ht = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        None,
    );
    safe_c2rust_g_dataset_cached = ::core::ptr::null_mut::<GDataset>();
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_datalist_init\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
