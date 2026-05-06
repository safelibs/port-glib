use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;
    fn g_ptr_array_set_free_func(array: *mut GPtrArray, element_free_func: GDestroyNotify);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gintptr = ::core::ffi::c_long;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GHashTable {
    pub size: gsize,
    pub mod_0: gint,
    pub mask: guint,
    pub nnodes: guint,
    pub noccupied: guint,
    #[bitfield(name = "have_big_keys", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "have_big_values", ty = "guint", bits = "1..=1")]
    pub have_big_keys_have_big_values: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub keys: gpointer,
    pub hashes: *mut guint,
    pub values: gpointer,
    pub hash_func: GHashFunc,
    pub key_equal_func: GEqualFunc,
    pub ref_count: gatomicrefcount,
    pub version: ::core::ffi::c_int,
    pub key_destroy_func: GDestroyNotify,
    pub value_destroy_func: GDestroyNotify,
}
pub type GHashTable = _GHashTable;
pub type GHRFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
pub type uintptr_t = usize;
pub const VG_USERREQ__RUNNING_ON_VALGRIND: C2RustUnnamed = 4097;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RealIter {
    pub hash_table: *mut GHashTable,
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub position: gint,
    pub dummy3: gboolean,
    pub version: gintptr,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const VG_USERREQ__INNER_THREADS: C2RustUnnamed = 6402;
pub const VG_USERREQ__VEX_INIT_FOR_IRI: C2RustUnnamed = 6401;
pub const VG_USERREQ__CHANGE_ERR_DISABLEMENT: C2RustUnnamed = 6145;
pub const VG_USERREQ__MAP_IP_TO_SRCLOC: C2RustUnnamed = 5889;
pub const VG_USERREQ__LOAD_PDB_DEBUGINFO: C2RustUnnamed = 5633;
pub const VG_USERREQ__STACK_CHANGE: C2RustUnnamed = 5379;
pub const VG_USERREQ__STACK_DEREGISTER: C2RustUnnamed = 5378;
pub const VG_USERREQ__STACK_REGISTER: C2RustUnnamed = 5377;
pub const VG_USERREQ__PRINTF_BACKTRACE_VALIST_BY_REF: C2RustUnnamed = 5124;
pub const VG_USERREQ__PRINTF_VALIST_BY_REF: C2RustUnnamed = 5123;
pub const VG_USERREQ__PRINTF_BACKTRACE: C2RustUnnamed = 5122;
pub const VG_USERREQ__PRINTF: C2RustUnnamed = 5121;
pub const VG_USERREQ__MEMPOOL_EXISTS: C2RustUnnamed = 4874;
pub const VG_USERREQ__MEMPOOL_CHANGE: C2RustUnnamed = 4873;
pub const VG_USERREQ__MOVE_MEMPOOL: C2RustUnnamed = 4872;
pub const VG_USERREQ__MEMPOOL_TRIM: C2RustUnnamed = 4871;
pub const VG_USERREQ__MEMPOOL_FREE: C2RustUnnamed = 4870;
pub const VG_USERREQ__MEMPOOL_ALLOC: C2RustUnnamed = 4869;
pub const VG_USERREQ__DESTROY_MEMPOOL: C2RustUnnamed = 4868;
pub const VG_USERREQ__CREATE_MEMPOOL: C2RustUnnamed = 4867;
pub const VG_USERREQ__FREELIKE_BLOCK: C2RustUnnamed = 4866;
pub const VG_USERREQ__RESIZEINPLACE_BLOCK: C2RustUnnamed = 4875;
pub const VG_USERREQ__MALLOCLIKE_BLOCK: C2RustUnnamed = 4865;
pub const VG_USERREQ__GDB_MONITOR_COMMAND: C2RustUnnamed = 4610;
pub const VG_USERREQ__COUNT_ERRORS: C2RustUnnamed = 4609;
pub const VG_USERREQ__CLIENT_CALL3: C2RustUnnamed = 4356;
pub const VG_USERREQ__CLIENT_CALL2: C2RustUnnamed = 4355;
pub const VG_USERREQ__CLIENT_CALL1: C2RustUnnamed = 4354;
pub const VG_USERREQ__CLIENT_CALL0: C2RustUnnamed = 4353;
pub const VG_USERREQ__DISCARD_TRANSLATIONS: C2RustUnnamed = 4098;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_hash_table_set_shift\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const HASH_TABLE_MIN_SHIFT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNUSED_HASH_VALUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOMBSTONE_HASH_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BIG_ENTRY_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SMALL_ENTRY_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut safe_c2rust_prime_mod: [gint; 32] = [
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    31 as ::core::ffi::c_int,
    61 as ::core::ffi::c_int,
    127 as ::core::ffi::c_int,
    251 as ::core::ffi::c_int,
    509 as ::core::ffi::c_int,
    1021 as ::core::ffi::c_int,
    2039 as ::core::ffi::c_int,
    4093 as ::core::ffi::c_int,
    8191 as ::core::ffi::c_int,
    16381 as ::core::ffi::c_int,
    32749 as ::core::ffi::c_int,
    65521 as ::core::ffi::c_int,
    131071 as ::core::ffi::c_int,
    262139 as ::core::ffi::c_int,
    524287 as ::core::ffi::c_int,
    1048573 as ::core::ffi::c_int,
    2097143 as ::core::ffi::c_int,
    4194301 as ::core::ffi::c_int,
    8388593 as ::core::ffi::c_int,
    16777213 as ::core::ffi::c_int,
    33554393 as ::core::ffi::c_int,
    67108859 as ::core::ffi::c_int,
    134217689 as ::core::ffi::c_int,
    268435399 as ::core::ffi::c_int,
    536870909 as ::core::ffi::c_int,
    1073741789 as ::core::ffi::c_int,
    2147483647 as ::core::ffi::c_int,
];
unsafe extern "C" fn safe_c2rust_g_hash_table_set_shift(
    mut hash_table: *mut GHashTable,
    mut shift: gint,
) {
    (*hash_table).size = ((1 as ::core::ffi::c_int) << shift) as gsize;
    (*hash_table).mod_0 = safe_c2rust_prime_mod[shift as usize];
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if (*hash_table).size & (*hash_table).size.wrapping_sub(1 as gsize) == 0 as gsize {
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
            b"../original/glib/ghash.c\0" as *const u8 as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(hash_table->size & (hash_table->size - 1)) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*hash_table).mask = (*hash_table).size.wrapping_sub(1 as gsize) as guint;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_find_closest_shift(mut n: gint) -> gint {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while n != 0 {
        n >>= 1 as ::core::ffi::c_int;
        i += 1;
    }
    return i;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_set_shift_from_size(
    mut hash_table: *mut GHashTable,
    mut size: gint,
) {
    let mut shift: gint = 0;
    shift = safe_c2rust_g_hash_table_find_closest_shift(size);
    shift = (if shift > 3 as ::core::ffi::c_int {
        shift as ::core::ffi::c_int
    } else {
        3 as ::core::ffi::c_int
    }) as gint;
    safe_c2rust_g_hash_table_set_shift(hash_table, shift);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_realloc_key_or_value_array(
    mut a: gpointer,
    mut size: guint,
    mut is_big: gboolean,
) -> gpointer {
    return g_realloc(
        a,
        size.wrapping_mul(
            (if is_big != 0 {
                BIG_ENTRY_SIZE
            } else {
                SMALL_ENTRY_SIZE
            }) as guint,
        ) as gsize,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_fetch_key_or_value(
    mut a: gpointer,
    mut index: guint,
    mut is_big: gboolean,
) -> gpointer {
    return if is_big != 0 {
        *(a as *mut gpointer).offset(index as isize)
    } else {
        *(a as *mut guint).offset(index as isize) as gulong as gpointer
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_assign_key_or_value(
    mut a: gpointer,
    mut index: guint,
    mut is_big: gboolean,
    mut v: gpointer,
) {
    if is_big != 0 {
        let ref mut fresh3 = *(a as *mut gpointer).offset(index as isize);
        *fresh3 = v;
    } else {
        *(a as *mut guint).offset(index as isize) = v as gulong as guint;
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_evict_key_or_value(
    mut a: gpointer,
    mut index: guint,
    mut is_big: gboolean,
    mut v: gpointer,
) -> gpointer {
    if is_big != 0 {
        let mut r: gpointer = *(a as *mut gpointer).offset(index as isize);
        let ref mut fresh4 = *(a as *mut gpointer).offset(index as isize);
        *fresh4 = v;
        return r;
    } else {
        let mut r_0: gpointer = *(a as *mut guint).offset(index as isize) as gulong as gpointer;
        *(a as *mut guint).offset(index as isize) = v as gulong as guint;
        return r_0;
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_hash_to_index(
    mut hash_table: *mut GHashTable,
    mut hash: guint,
) -> guint {
    return hash
        .wrapping_mul(11 as guint)
        .wrapping_rem((*hash_table).mod_0 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_lookup_node(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
    mut hash_return: *mut guint,
) -> guint {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    let mut hash_value: guint = 0;
    let mut first_tombstone: guint = 0 as guint;
    let mut have_tombstone: gboolean = FALSE;
    let mut step: guint = 0 as guint;
    hash_value = (*hash_table).hash_func.expect("non-null function pointer")(key);
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !(hash_value >= 2 as guint) {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        hash_value = 2 as guint;
    }
    *hash_return = hash_value;
    node_index = safe_c2rust_g_hash_table_hash_to_index(hash_table, hash_value);
    node_hash = *(*hash_table).hashes.offset(node_index as isize);
    while !(node_hash == UNUSED_HASH_VALUE as guint) {
        if node_hash == hash_value {
            let mut node_key: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
                (*hash_table).keys,
                node_index,
                (*hash_table).have_big_keys() as gboolean,
            );
            if (*hash_table).key_equal_func.is_some() {
                if (*hash_table)
                    .key_equal_func
                    .expect("non-null function pointer")(
                    node_key as gconstpointer, key
                ) != 0
                {
                    return node_index;
                }
            } else if node_key == key as gpointer {
                return node_index;
            }
        } else if node_hash == TOMBSTONE_HASH_VALUE as guint && have_tombstone == 0 {
            first_tombstone = node_index;
            have_tombstone = TRUE as gboolean;
        }
        step = step.wrapping_add(1);
        node_index = node_index.wrapping_add(step);
        node_index &= (*hash_table).mask;
        node_hash = *(*hash_table).hashes.offset(node_index as isize);
    }
    if have_tombstone != 0 {
        return first_tombstone;
    }
    return node_index;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_remove_node(
    mut hash_table: *mut GHashTable,
    mut i: gint,
    mut notify: gboolean,
) {
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    key = safe_c2rust_g_hash_table_fetch_key_or_value(
        (*hash_table).keys,
        i as guint,
        (*hash_table).have_big_keys() as gboolean,
    );
    value = safe_c2rust_g_hash_table_fetch_key_or_value(
        (*hash_table).values,
        i as guint,
        (*hash_table).have_big_values() as gboolean,
    );
    *(*hash_table).hashes.offset(i as isize) = TOMBSTONE_HASH_VALUE as guint;
    safe_c2rust_g_hash_table_assign_key_or_value(
        (*hash_table).keys,
        i as guint,
        (*hash_table).have_big_keys() as gboolean,
        NULL_0,
    );
    safe_c2rust_g_hash_table_assign_key_or_value(
        (*hash_table).values,
        i as guint,
        (*hash_table).have_big_values() as gboolean,
        NULL_0,
    );
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*hash_table).nnodes > 0 as guint {
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
            b"../original/glib/ghash.c\0" as *const u8 as *const ::core::ffi::c_char,
            493 as ::core::ffi::c_int,
            G_STRFUNC,
            b"hash_table->nnodes > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*hash_table).nnodes = (*hash_table).nnodes.wrapping_sub(1);
    if notify != 0 && (*hash_table).key_destroy_func.is_some() {
        (*hash_table)
            .key_destroy_func
            .expect("non-null function pointer")(key);
    }
    if notify != 0 && (*hash_table).value_destroy_func.is_some() {
        (*hash_table)
            .value_destroy_func
            .expect("non-null function pointer")(value);
    }
}
unsafe extern "C" fn safe_c2rust_g_hash_table_setup_storage(mut hash_table: *mut GHashTable) {
    let mut small: gboolean = FALSE;
    small = TRUE as gboolean;
    if ({
        let mut _zzq_args: [uintptr_t; 6] = [0; 6];
        let mut _zzq_result: ::core::ffi::c_ulong = 0;
        ::core::ptr::write_volatile(
            &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            VG_USERREQ__RUNNING_ON_VALGRIND as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        let fresh0 = &mut _zzq_result;
        let fresh1;
        let fresh2 = 0 as ::core::ffi::c_int;
        asm!(
            "rolq $3,  %rdi ; rolq $13, %rdi\n", "\trolq $61, %rdi ; rolq $51, %rdi\n",
            "\txchgq %rbx,%rbx\n", inlateout("dx")
            c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1, inlateout("ax")
            (& raw mut _zzq_args as * mut uintptr_t).offset(0 as ::core::ffi::c_int as
            isize) as * mut uintptr_t => _, options(att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        _zzq_result
    }) as ::core::ffi::c_uint
        != 0
    {
        small = FALSE as gboolean;
    }
    safe_c2rust_g_hash_table_set_shift(hash_table, HASH_TABLE_MIN_SHIFT);
    (*hash_table).set_have_big_keys((small == 0) as ::core::ffi::c_int as guint as guint);
    (*hash_table).set_have_big_values((small == 0) as ::core::ffi::c_int as guint as guint);
    (*hash_table).keys = safe_c2rust_g_hash_table_realloc_key_or_value_array(
        NULL_0,
        (*hash_table).size as guint,
        (*hash_table).have_big_keys() as gboolean,
    );
    (*hash_table).values = (*hash_table).keys;
    (*hash_table).hashes = ({
        let mut __n: gsize = (*hash_table).size;
        let mut __s: gsize = ::core::mem::size_of::<guint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut guint;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_remove_all_nodes(
    mut hash_table: *mut GHashTable,
    mut notify: gboolean,
    mut destruction: gboolean,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut old_size: gint = 0;
    let mut old_keys: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    let mut old_values: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    let mut old_hashes: *mut guint = ::core::ptr::null_mut::<guint>();
    let mut old_have_big_keys: gboolean = 0;
    let mut old_have_big_values: gboolean = 0;
    if (*hash_table).nnodes == 0 as guint {
        return;
    }
    (*hash_table).nnodes = 0 as guint;
    (*hash_table).noccupied = 0 as guint;
    if notify == 0
        || (*hash_table).key_destroy_func.is_none() && (*hash_table).value_destroy_func.is_none()
    {
        if destruction == 0 {
            memset(
                (*hash_table).hashes as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*hash_table).size as size_t)
                    .wrapping_mul(::core::mem::size_of::<guint>() as size_t),
            );
            memset(
                (*hash_table).keys as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*hash_table).size as size_t).wrapping_mul(
                    (if (*hash_table).have_big_keys() as ::core::ffi::c_int != 0 {
                        BIG_ENTRY_SIZE
                    } else {
                        SMALL_ENTRY_SIZE
                    }) as size_t,
                ),
            );
            memset(
                (*hash_table).values as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*hash_table).size as size_t).wrapping_mul(
                    (if (*hash_table).have_big_values() as ::core::ffi::c_int != 0 {
                        BIG_ENTRY_SIZE
                    } else {
                        SMALL_ENTRY_SIZE
                    }) as size_t,
                ),
            );
        }
        return;
    }
    old_size = (*hash_table).size as gint;
    old_have_big_keys = (*hash_table).have_big_keys() as gboolean;
    old_have_big_values = (*hash_table).have_big_values() as gboolean;
    old_keys =
        safe_c2rust_g_steal_pointer(&raw mut (*hash_table).keys as gpointer) as *mut gpointer;
    old_values =
        safe_c2rust_g_steal_pointer(&raw mut (*hash_table).values as gpointer) as *mut gpointer;
    old_hashes = safe_c2rust_g_steal_pointer(&raw mut (*hash_table).hashes as gpointer)
        as *mut guint as *mut guint;
    if destruction == 0 {
        safe_c2rust_g_hash_table_setup_storage(hash_table);
    } else {
        (*hash_table).mask = 0 as guint;
        (*hash_table).mod_0 = (*hash_table).mask as gint;
        (*hash_table).size = (*hash_table).mod_0 as gsize;
    }
    i = 0 as ::core::ffi::c_int;
    while i < old_size {
        if *old_hashes.offset(i as isize) >= 2 as guint {
            key = safe_c2rust_g_hash_table_fetch_key_or_value(
                old_keys as gpointer,
                i as guint,
                old_have_big_keys,
            );
            value = safe_c2rust_g_hash_table_fetch_key_or_value(
                old_values as gpointer,
                i as guint,
                old_have_big_values,
            );
            *old_hashes.offset(i as isize) = UNUSED_HASH_VALUE as guint;
            safe_c2rust_g_hash_table_assign_key_or_value(
                old_keys as gpointer,
                i as guint,
                old_have_big_keys,
                NULL_0,
            );
            safe_c2rust_g_hash_table_assign_key_or_value(
                old_values as gpointer,
                i as guint,
                old_have_big_values,
                NULL_0,
            );
            if (*hash_table).key_destroy_func.is_some() {
                (*hash_table)
                    .key_destroy_func
                    .expect("non-null function pointer")(key);
            }
            if (*hash_table).value_destroy_func.is_some() {
                (*hash_table)
                    .value_destroy_func
                    .expect("non-null function pointer")(value);
            }
        }
        i += 1;
    }
    if old_keys != old_values {
        g_free(old_values as gpointer);
    }
    g_free(old_keys as gpointer);
    g_free(old_hashes as gpointer);
}
unsafe extern "C" fn safe_c2rust_realloc_arrays(
    mut hash_table: *mut GHashTable,
    mut is_a_set: gboolean,
) {
    (*hash_table).hashes = ({
        let mut __n: gsize = (*hash_table).size;
        let mut __s: gsize = ::core::mem::size_of::<guint>() as gsize;
        let mut __p: gpointer = (*hash_table).hashes as gpointer;
        if __s == 1 as gsize {
            __p = g_realloc(__p, __n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_realloc(__p, __n.wrapping_mul(__s));
        } else {
            __p = g_realloc_n(__p, __n, __s);
        }
        __p
    }) as *mut guint;
    (*hash_table).keys = safe_c2rust_g_hash_table_realloc_key_or_value_array(
        (*hash_table).keys,
        (*hash_table).size as guint,
        (*hash_table).have_big_keys() as gboolean,
    );
    if is_a_set != 0 {
        (*hash_table).values = (*hash_table).keys;
    } else {
        (*hash_table).values = safe_c2rust_g_hash_table_realloc_key_or_value_array(
            (*hash_table).values,
            (*hash_table).size as guint,
            (*hash_table).have_big_values() as gboolean,
        );
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_get_status_bit(
    mut bitmap: *const guint32,
    mut index: guint,
) -> gboolean {
    return (*bitmap.offset(index.wrapping_div(32 as guint) as isize)
        >> index.wrapping_rem(32 as guint)
        & 1 as guint32) as gboolean;
}
#[inline]
unsafe extern "C" fn safe_c2rust_set_status_bit(mut bitmap: *mut guint32, mut index: guint) {
    let ref mut fresh5 = *bitmap.offset(index.wrapping_div(32 as guint) as isize);
    *fresh5 |= (1 as ::core::ffi::c_uint) << index.wrapping_rem(32 as guint);
}
unsafe extern "C" fn safe_c2rust_resize_map(
    mut hash_table: *mut GHashTable,
    mut old_size: guint,
    mut reallocated_buckets_bitmap: *mut guint32,
) {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < old_size {
        let mut node_hash: guint = *(*hash_table).hashes.offset(i as isize);
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if !(node_hash >= 2 as guint) {
            *(*hash_table).hashes.offset(i as isize) = UNUSED_HASH_VALUE as guint;
        } else if !(safe_c2rust_get_status_bit(reallocated_buckets_bitmap, i) != 0) {
            *(*hash_table).hashes.offset(i as isize) = UNUSED_HASH_VALUE as guint;
            key = safe_c2rust_g_hash_table_evict_key_or_value(
                (*hash_table).keys,
                i,
                (*hash_table).have_big_keys() as gboolean,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            value = safe_c2rust_g_hash_table_evict_key_or_value(
                (*hash_table).values,
                i,
                (*hash_table).have_big_values() as gboolean,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            loop {
                let mut hash_val: guint = 0;
                let mut replaced_hash: guint = 0;
                let mut step: guint = 0 as guint;
                hash_val = safe_c2rust_g_hash_table_hash_to_index(hash_table, node_hash);
                while safe_c2rust_get_status_bit(reallocated_buckets_bitmap, hash_val) != 0 {
                    step = step.wrapping_add(1);
                    hash_val = hash_val.wrapping_add(step);
                    hash_val &= (*hash_table).mask;
                }
                safe_c2rust_set_status_bit(reallocated_buckets_bitmap, hash_val);
                replaced_hash = *(*hash_table).hashes.offset(hash_val as isize);
                *(*hash_table).hashes.offset(hash_val as isize) = node_hash;
                if !(replaced_hash >= 2 as guint) {
                    safe_c2rust_g_hash_table_assign_key_or_value(
                        (*hash_table).keys,
                        hash_val,
                        (*hash_table).have_big_keys() as gboolean,
                        key,
                    );
                    safe_c2rust_g_hash_table_assign_key_or_value(
                        (*hash_table).values,
                        hash_val,
                        (*hash_table).have_big_values() as gboolean,
                        value,
                    );
                    break;
                } else {
                    node_hash = replaced_hash;
                    key = safe_c2rust_g_hash_table_evict_key_or_value(
                        (*hash_table).keys,
                        hash_val,
                        (*hash_table).have_big_keys() as gboolean,
                        key,
                    );
                    value = safe_c2rust_g_hash_table_evict_key_or_value(
                        (*hash_table).values,
                        hash_val,
                        (*hash_table).have_big_values() as gboolean,
                        value,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_resize_set(
    mut hash_table: *mut GHashTable,
    mut old_size: guint,
    mut reallocated_buckets_bitmap: *mut guint32,
) {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < old_size {
        let mut node_hash: guint = *(*hash_table).hashes.offset(i as isize);
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if !(node_hash >= 2 as guint) {
            *(*hash_table).hashes.offset(i as isize) = UNUSED_HASH_VALUE as guint;
        } else if !(safe_c2rust_get_status_bit(reallocated_buckets_bitmap, i) != 0) {
            *(*hash_table).hashes.offset(i as isize) = UNUSED_HASH_VALUE as guint;
            key = safe_c2rust_g_hash_table_evict_key_or_value(
                (*hash_table).keys,
                i,
                (*hash_table).have_big_keys() as gboolean,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            loop {
                let mut hash_val: guint = 0;
                let mut replaced_hash: guint = 0;
                let mut step: guint = 0 as guint;
                hash_val = safe_c2rust_g_hash_table_hash_to_index(hash_table, node_hash);
                while safe_c2rust_get_status_bit(reallocated_buckets_bitmap, hash_val) != 0 {
                    step = step.wrapping_add(1);
                    hash_val = hash_val.wrapping_add(step);
                    hash_val &= (*hash_table).mask;
                }
                safe_c2rust_set_status_bit(reallocated_buckets_bitmap, hash_val);
                replaced_hash = *(*hash_table).hashes.offset(hash_val as isize);
                *(*hash_table).hashes.offset(hash_val as isize) = node_hash;
                if !(replaced_hash >= 2 as guint) {
                    safe_c2rust_g_hash_table_assign_key_or_value(
                        (*hash_table).keys,
                        hash_val,
                        (*hash_table).have_big_keys() as gboolean,
                        key,
                    );
                    break;
                } else {
                    node_hash = replaced_hash;
                    key = safe_c2rust_g_hash_table_evict_key_or_value(
                        (*hash_table).keys,
                        hash_val,
                        (*hash_table).have_big_keys() as gboolean,
                        key,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_g_hash_table_resize(mut hash_table: *mut GHashTable) {
    let mut reallocated_buckets_bitmap: *mut guint32 = ::core::ptr::null_mut::<guint32>();
    let mut old_size: gsize = 0;
    let mut is_a_set: gboolean = 0;
    old_size = (*hash_table).size;
    is_a_set = ((*hash_table).keys == (*hash_table).values) as ::core::ffi::c_int as gboolean;
    safe_c2rust_g_hash_table_set_shift_from_size(
        hash_table,
        ((*hash_table).nnodes as ::core::ffi::c_double * 1.333f64) as gint,
    );
    if (*hash_table).size > old_size {
        safe_c2rust_realloc_arrays(hash_table, is_a_set);
        memset(
            (*hash_table).hashes.offset(old_size as isize) as *mut guint
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*hash_table).size as size_t)
                .wrapping_sub(old_size as size_t)
                .wrapping_mul(::core::mem::size_of::<guint>() as size_t),
        );
        reallocated_buckets_bitmap = ({
            let mut __n: gsize = (*hash_table)
                .size
                .wrapping_add(31 as gsize)
                .wrapping_div(32 as gsize);
            let mut __s: gsize = ::core::mem::size_of::<guint32>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut guint32;
    } else {
        reallocated_buckets_bitmap = ({
            let mut __n: gsize = old_size.wrapping_add(31 as gsize).wrapping_div(32 as gsize);
            let mut __s: gsize = ::core::mem::size_of::<guint32>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut guint32;
    }
    if is_a_set != 0 {
        safe_c2rust_resize_set(hash_table, old_size as guint, reallocated_buckets_bitmap);
    } else {
        safe_c2rust_resize_map(hash_table, old_size as guint, reallocated_buckets_bitmap);
    }
    g_free(reallocated_buckets_bitmap as gpointer);
    if (*hash_table).size < old_size {
        safe_c2rust_realloc_arrays(hash_table, is_a_set);
    }
    (*hash_table).noccupied = (*hash_table).nnodes;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_maybe_resize(mut hash_table: *mut GHashTable) {
    let mut noccupied: gsize = (*hash_table).noccupied as gsize;
    let mut size: gsize = (*hash_table).size;
    if size > (*hash_table).nnodes.wrapping_mul(4 as guint) as gsize
        && size > ((1 as ::core::ffi::c_int) << HASH_TABLE_MIN_SHIFT) as gsize
        || size <= noccupied.wrapping_add(noccupied.wrapping_div(16 as gsize))
    {
        safe_c2rust_g_hash_table_resize(hash_table);
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_entry_is_big(mut v: gpointer) -> gboolean {
    return (v as guintptr >> (BIG_ENTRY_SIZE - SMALL_ENTRY_SIZE) * 8 as ::core::ffi::c_int
        != 0 as guintptr) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_maybe_make_big_keys_or_values(
    mut a_p: *mut gpointer,
    mut v: gpointer,
    mut ht_size: gint,
) -> gboolean {
    if safe_c2rust_entry_is_big(v) != 0 {
        let mut a: *mut guint = *a_p as *mut guint;
        let mut a_new: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
        let mut i: gint = 0;
        a_new = ({
            let mut __n: gsize = ht_size as gsize;
            let mut __s: gsize = ::core::mem::size_of::<gpointer>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut gpointer;
        i = 0 as ::core::ffi::c_int as gint;
        while i < ht_size {
            let ref mut fresh6 = *a_new.offset(i as isize);
            *fresh6 = *a.offset(i as isize) as gulong as gpointer;
            i += 1;
        }
        g_free(a as gpointer);
        *a_p = a_new as gpointer;
        return TRUE;
    }
    return FALSE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_hash_table_ensure_keyval_fits(
    mut hash_table: *mut GHashTable,
    mut key: gpointer,
    mut value: gpointer,
) {
    let mut is_a_set: gboolean = ((*hash_table).keys == (*hash_table).values) as ::core::ffi::c_int;
    if is_a_set != 0 {
        if (*hash_table).have_big_keys() != 0 {
            if key != value {
                (*hash_table).values = g_memdup2(
                    (*hash_table).keys as gconstpointer,
                    (::core::mem::size_of::<gpointer>() as gsize).wrapping_mul((*hash_table).size),
                );
            }
            return;
        } else if key != value {
            (*hash_table).values = g_memdup2(
                (*hash_table).keys as gconstpointer,
                (::core::mem::size_of::<guint>() as gsize).wrapping_mul((*hash_table).size),
            );
            is_a_set = FALSE as gboolean;
        }
    }
    if (*hash_table).have_big_keys() == 0 {
        (*hash_table).set_have_big_keys(safe_c2rust_g_hash_table_maybe_make_big_keys_or_values(
            &raw mut (*hash_table).keys,
            key,
            (*hash_table).size as gint,
        ) as guint as guint);
        if is_a_set != 0 {
            (*hash_table).values = (*hash_table).keys;
            (*hash_table).set_have_big_values((*hash_table).have_big_keys() as guint);
        }
    }
    if is_a_set == 0 && (*hash_table).have_big_values() == 0 {
        (*hash_table).set_have_big_values(safe_c2rust_g_hash_table_maybe_make_big_keys_or_values(
            &raw mut (*hash_table).values,
            value,
            (*hash_table).size as gint,
        ) as guint as guint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_new(
    mut hash_func: GHashFunc,
    mut key_equal_func: GEqualFunc,
) -> *mut GHashTable {
    return safe_c2rust_g_hash_table_new_full(hash_func, key_equal_func, None, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_new_full(
    mut hash_func: GHashFunc,
    mut key_equal_func: GEqualFunc,
    mut key_destroy_func: GDestroyNotify,
    mut value_destroy_func: GDestroyNotify,
) -> *mut GHashTable {
    let mut hash_table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    hash_table = g_slice_alloc(::core::mem::size_of::<GHashTable>() as gsize) as *mut GHashTable;
    g_atomic_ref_count_init(&raw mut (*hash_table).ref_count);
    (*hash_table).nnodes = 0 as guint;
    (*hash_table).noccupied = 0 as guint;
    (*hash_table).hash_func = (if hash_func.is_some() {
        hash_func as Option<unsafe extern "C" fn(gconstpointer) -> guint>
    } else {
        Some(safe_c2rust_g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint)
    }) as GHashFunc;
    (*hash_table).key_equal_func = key_equal_func;
    (*hash_table).version = 0 as ::core::ffi::c_int;
    (*hash_table).key_destroy_func = key_destroy_func;
    (*hash_table).value_destroy_func = value_destroy_func;
    safe_c2rust_g_hash_table_setup_storage(hash_table);
    return hash_table;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_new_similar(
    mut other_hash_table: *mut GHashTable,
) -> *mut GHashTable {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !other_hash_table.is_null() {
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
            b"other_hash_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    return safe_c2rust_g_hash_table_new_full(
        (*other_hash_table).hash_func,
        (*other_hash_table).key_equal_func,
        (*other_hash_table).key_destroy_func,
        (*other_hash_table).value_destroy_func,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_init(
    mut iter: *mut GHashTableIter,
    mut hash_table: *mut GHashTable,
) {
    let mut ri: *mut RealIter = iter as *mut RealIter;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*ri).hash_table = hash_table;
    (*ri).position = -(1 as ::core::ffi::c_int) as gint;
    (*ri).version = (*hash_table).version as gintptr;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_next(
    mut iter: *mut GHashTableIter,
    mut key: *mut gpointer,
    mut value: *mut gpointer,
) -> gboolean {
    let mut ri: *mut RealIter = iter as *mut RealIter;
    let mut position: gint = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*ri).version == (*(*ri).hash_table).version as gintptr {
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
            b"ri->version == ri->hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ((*ri).position as gssize) < (*(*ri).hash_table).size as gssize {
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
            b"ri->position < (gssize) ri->hash_table->size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    position = (*ri).position;
    loop {
        position += 1;
        if position as gssize >= (*(*ri).hash_table).size as gssize {
            (*ri).position = position;
            return FALSE;
        }
        if *(*(*ri).hash_table).hashes.offset(position as isize) >= 2 as guint {
            break;
        }
    }
    if !key.is_null() {
        *key = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*(*ri).hash_table).keys,
            position as guint,
            (*(*ri).hash_table).have_big_keys() as gboolean,
        );
    }
    if !value.is_null() {
        *value = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*(*ri).hash_table).values,
            position as guint,
            (*(*ri).hash_table).have_big_values() as gboolean,
        );
    }
    (*ri).position = position;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_get_hash_table(
    mut iter: *mut GHashTableIter,
) -> *mut GHashTable {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    return (*(iter as *mut RealIter)).hash_table;
}
unsafe extern "C" fn safe_c2rust_iter_remove_or_steal(mut ri: *mut RealIter, mut notify: gboolean) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !ri.is_null() {
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
            b"ri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*ri).version == (*(*ri).hash_table).version as gintptr {
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
            b"ri->version == ri->hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*ri).position >= 0 as ::core::ffi::c_int {
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
            b"ri->position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ((*ri).position as gsize) < (*(*ri).hash_table).size {
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
            b"(gsize) ri->position < ri->hash_table->size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_hash_table_remove_node((*ri).hash_table, (*ri).position, notify);
    (*ri).version += 1;
    (*(*ri).hash_table).version += 1;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_remove(mut iter: *mut GHashTableIter) {
    safe_c2rust_iter_remove_or_steal(iter as *mut RealIter, TRUE);
}
unsafe extern "C" fn safe_c2rust_g_hash_table_insert_node(
    mut hash_table: *mut GHashTable,
    mut node_index: guint,
    mut key_hash: guint,
    mut new_key: gpointer,
    mut new_value: gpointer,
    mut keep_new_key: gboolean,
    mut reusing_key: gboolean,
) -> gboolean {
    let mut already_exists: gboolean = 0;
    let mut old_hash: guint = 0;
    let mut key_to_free: gpointer = NULL_0;
    let mut key_to_keep: gpointer = NULL_0;
    let mut value_to_free: gpointer = NULL_0;
    old_hash = *(*hash_table).hashes.offset(node_index as isize);
    already_exists = (old_hash >= 2 as guint) as ::core::ffi::c_int as gboolean;
    if already_exists != 0 {
        value_to_free = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            node_index,
            (*hash_table).have_big_values() as gboolean,
        );
        if keep_new_key != 0 {
            key_to_free = safe_c2rust_g_hash_table_fetch_key_or_value(
                (*hash_table).keys,
                node_index,
                (*hash_table).have_big_keys() as gboolean,
            );
            key_to_keep = new_key;
        } else {
            key_to_free = new_key;
            key_to_keep = safe_c2rust_g_hash_table_fetch_key_or_value(
                (*hash_table).keys,
                node_index,
                (*hash_table).have_big_keys() as gboolean,
            );
        }
    } else {
        *(*hash_table).hashes.offset(node_index as isize) = key_hash;
        key_to_keep = new_key;
    }
    safe_c2rust_g_hash_table_ensure_keyval_fits(hash_table, key_to_keep, new_value);
    safe_c2rust_g_hash_table_assign_key_or_value(
        (*hash_table).keys,
        node_index,
        (*hash_table).have_big_keys() as gboolean,
        key_to_keep,
    );
    safe_c2rust_g_hash_table_assign_key_or_value(
        (*hash_table).values,
        node_index,
        (*hash_table).have_big_values() as gboolean,
        new_value,
    );
    if already_exists == 0 {
        (*hash_table).nnodes = (*hash_table).nnodes.wrapping_add(1);
        if old_hash == UNUSED_HASH_VALUE as guint {
            (*hash_table).noccupied = (*hash_table).noccupied.wrapping_add(1);
            safe_c2rust_g_hash_table_maybe_resize(hash_table);
        }
        (*hash_table).version += 1;
    }
    if already_exists != 0 {
        if (*hash_table).key_destroy_func.is_some() && reusing_key == 0 {
            Some(
                (*hash_table)
                    .key_destroy_func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(key_to_free);
        }
        if (*hash_table).value_destroy_func.is_some() {
            Some(
                (*hash_table)
                    .value_destroy_func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(value_to_free);
        }
    }
    return (already_exists == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_replace(
    mut iter: *mut GHashTableIter,
    mut value: gpointer,
) {
    let mut ri: *mut RealIter = ::core::ptr::null_mut::<RealIter>();
    let mut node_hash: guint = 0;
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ri = iter as *mut RealIter;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !ri.is_null() {
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
            b"ri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*ri).version == (*(*ri).hash_table).version as gintptr {
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
            b"ri->version == ri->hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*ri).position >= 0 as ::core::ffi::c_int {
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
            b"ri->position >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ((*ri).position as gsize) < (*(*ri).hash_table).size {
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
            b"(gsize) ri->position < ri->hash_table->size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    node_hash = *(*(*ri).hash_table).hashes.offset((*ri).position as isize);
    key = safe_c2rust_g_hash_table_fetch_key_or_value(
        (*(*ri).hash_table).keys,
        (*ri).position as guint,
        (*(*ri).hash_table).have_big_keys() as gboolean,
    );
    safe_c2rust_g_hash_table_insert_node(
        (*ri).hash_table,
        (*ri).position as guint,
        node_hash,
        key,
        value,
        TRUE,
        TRUE,
    );
    (*ri).version += 1;
    (*(*ri).hash_table).version += 1;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_iter_steal(mut iter: *mut GHashTableIter) {
    safe_c2rust_iter_remove_or_steal(iter as *mut RealIter, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_ref(
    mut hash_table: *mut GHashTable,
) -> *mut GHashTable {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    g_atomic_ref_count_inc(&raw mut (*hash_table).ref_count);
    return hash_table;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_unref(mut hash_table: *mut GHashTable) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*hash_table).ref_count) != 0 {
        safe_c2rust_g_hash_table_remove_all_nodes(hash_table, TRUE, TRUE);
        if (*hash_table).keys != (*hash_table).values {
            g_free((*hash_table).values);
        }
        g_free((*hash_table).keys);
        g_free((*hash_table).hashes as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<GHashTable>() as gsize,
            hash_table as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_destroy(mut hash_table: *mut GHashTable) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_hash_table_remove_all(hash_table);
    safe_c2rust_g_hash_table_unref(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_lookup(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
) -> gpointer {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    node_index = safe_c2rust_g_hash_table_lookup_node(hash_table, key, &raw mut node_hash);
    return if *(*hash_table).hashes.offset(node_index as isize) >= 2 as guint {
        safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            node_index,
            (*hash_table).have_big_values() as gboolean,
        )
    } else {
        NULL_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_lookup_extended(
    mut hash_table: *mut GHashTable,
    mut lookup_key: gconstpointer,
    mut orig_key: *mut gpointer,
    mut value: *mut gpointer,
) -> gboolean {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node_index = safe_c2rust_g_hash_table_lookup_node(hash_table, lookup_key, &raw mut node_hash);
    if !(*(*hash_table).hashes.offset(node_index as isize) >= 2 as guint) {
        if !orig_key.is_null() {
            *orig_key = NULL_0 as gpointer;
        }
        if !value.is_null() {
            *value = NULL_0 as gpointer;
        }
        return FALSE;
    }
    if !orig_key.is_null() {
        *orig_key = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).keys,
            node_index,
            (*hash_table).have_big_keys() as gboolean,
        );
    }
    if !value.is_null() {
        *value = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            node_index,
            (*hash_table).have_big_values() as gboolean,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_insert_internal(
    mut hash_table: *mut GHashTable,
    mut key: gpointer,
    mut value: gpointer,
    mut keep_new_key: gboolean,
) -> gboolean {
    let mut key_hash: guint = 0;
    let mut node_index: guint = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node_index =
        safe_c2rust_g_hash_table_lookup_node(hash_table, key as gconstpointer, &raw mut key_hash);
    return safe_c2rust_g_hash_table_insert_node(
        hash_table,
        node_index,
        key_hash,
        key,
        value,
        keep_new_key,
        FALSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_insert(
    mut hash_table: *mut GHashTable,
    mut key: gpointer,
    mut value: gpointer,
) -> gboolean {
    return safe_c2rust_g_hash_table_insert_internal(hash_table, key, value, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_replace(
    mut hash_table: *mut GHashTable,
    mut key: gpointer,
    mut value: gpointer,
) -> gboolean {
    return safe_c2rust_g_hash_table_insert_internal(hash_table, key, value, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_add(
    mut hash_table: *mut GHashTable,
    mut key: gpointer,
) -> gboolean {
    return safe_c2rust_g_hash_table_insert_internal(hash_table, key, key, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_contains(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
) -> gboolean {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node_index = safe_c2rust_g_hash_table_lookup_node(hash_table, key, &raw mut node_hash);
    return (*(*hash_table).hashes.offset(node_index as isize) >= 2 as guint) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_remove_internal(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
    mut notify: gboolean,
) -> gboolean {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node_index = safe_c2rust_g_hash_table_lookup_node(hash_table, key, &raw mut node_hash);
    if !(*(*hash_table).hashes.offset(node_index as isize) >= 2 as guint) {
        return FALSE;
    }
    safe_c2rust_g_hash_table_remove_node(hash_table, node_index as gint, notify);
    safe_c2rust_g_hash_table_maybe_resize(hash_table);
    (*hash_table).version += 1;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_remove(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
) -> gboolean {
    return safe_c2rust_g_hash_table_remove_internal(hash_table, key, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_steal(
    mut hash_table: *mut GHashTable,
    mut key: gconstpointer,
) -> gboolean {
    return safe_c2rust_g_hash_table_remove_internal(hash_table, key, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_steal_extended(
    mut hash_table: *mut GHashTable,
    mut lookup_key: gconstpointer,
    mut stolen_key: *mut gpointer,
    mut stolen_value: *mut gpointer,
) -> gboolean {
    let mut node_index: guint = 0;
    let mut node_hash: guint = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node_index = safe_c2rust_g_hash_table_lookup_node(hash_table, lookup_key, &raw mut node_hash);
    if !(*(*hash_table).hashes.offset(node_index as isize) >= 2 as guint) {
        if !stolen_key.is_null() {
            *stolen_key = NULL_0 as gpointer;
        }
        if !stolen_value.is_null() {
            *stolen_value = NULL_0 as gpointer;
        }
        return FALSE;
    }
    if !stolen_key.is_null() {
        *stolen_key = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).keys,
            node_index,
            (*hash_table).have_big_keys() as gboolean,
        );
        safe_c2rust_g_hash_table_assign_key_or_value(
            (*hash_table).keys,
            node_index,
            (*hash_table).have_big_keys() as gboolean,
            NULL_0,
        );
    }
    if !stolen_value.is_null() {
        *stolen_value = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            node_index,
            (*hash_table).have_big_values() as gboolean,
        );
        safe_c2rust_g_hash_table_assign_key_or_value(
            (*hash_table).values,
            node_index,
            (*hash_table).have_big_values() as gboolean,
            NULL_0,
        );
    }
    safe_c2rust_g_hash_table_remove_node(hash_table, node_index as gint, FALSE);
    safe_c2rust_g_hash_table_maybe_resize(hash_table);
    (*hash_table).version += 1;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_remove_all(mut hash_table: *mut GHashTable) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
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
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*hash_table).nnodes != 0 as guint {
        (*hash_table).version += 1;
    }
    safe_c2rust_g_hash_table_remove_all_nodes(hash_table, TRUE, FALSE);
    safe_c2rust_g_hash_table_maybe_resize(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_steal_all(mut hash_table: *mut GHashTable) {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*hash_table).nnodes != 0 as guint {
        (*hash_table).version += 1;
    }
    safe_c2rust_g_hash_table_remove_all_nodes(hash_table, FALSE, FALSE);
    safe_c2rust_g_hash_table_maybe_resize(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_steal_all_keys(
    mut hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut key_destroy_func: GDestroyNotify = None;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = safe_c2rust_g_hash_table_get_keys_as_ptr_array(hash_table);
    key_destroy_func = ::core::mem::transmute::<gpointer, GDestroyNotify>(
        safe_c2rust_g_steal_pointer(&raw mut (*hash_table).key_destroy_func as gpointer),
    ) as GDestroyNotify;
    g_ptr_array_set_free_func(array, key_destroy_func);
    safe_c2rust_g_hash_table_remove_all(hash_table);
    (*hash_table).key_destroy_func = ::core::mem::transmute::<gpointer, GDestroyNotify>(
        safe_c2rust_g_steal_pointer(&raw mut key_destroy_func as gpointer),
    ) as GDestroyNotify;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_steal_all_values(
    mut hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut value_destroy_func: GDestroyNotify = None;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = safe_c2rust_g_hash_table_get_values_as_ptr_array(hash_table);
    value_destroy_func = ::core::mem::transmute::<gpointer, GDestroyNotify>(
        safe_c2rust_g_steal_pointer(&raw mut (*hash_table).value_destroy_func as gpointer),
    ) as GDestroyNotify;
    g_ptr_array_set_free_func(array, value_destroy_func);
    safe_c2rust_g_hash_table_remove_all(hash_table);
    (*hash_table).value_destroy_func = ::core::mem::transmute::<gpointer, GDestroyNotify>(
        safe_c2rust_g_steal_pointer(&raw mut value_destroy_func as gpointer),
    ) as GDestroyNotify;
    return array;
}
unsafe extern "C" fn safe_c2rust_g_hash_table_foreach_remove_or_steal(
    mut hash_table: *mut GHashTable,
    mut func: GHRFunc,
    mut user_data: gpointer,
    mut notify: gboolean,
) -> guint {
    let mut deleted: guint = 0 as guint;
    let mut i: gsize = 0;
    let mut version: gint = (*hash_table).version as gint;
    i = 0 as gsize;
    while i < (*hash_table).size {
        let mut node_hash: guint = *(*hash_table).hashes.offset(i as isize);
        let mut node_key: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).keys,
            i as guint,
            (*hash_table).have_big_keys() as gboolean,
        );
        let mut node_value: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            i as guint,
            (*hash_table).have_big_values() as gboolean,
        );
        if node_hash >= 2 as guint
            && Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
                node_key, node_value, user_data,
            ) != 0
        {
            safe_c2rust_g_hash_table_remove_node(hash_table, i as gint, notify);
            deleted = deleted.wrapping_add(1);
        }
        if ({
            let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
            if version == (*hash_table).version {
                _g_boolean_var_39 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_39 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_39
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"version == hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as guint;
        }
        i = i.wrapping_add(1);
    }
    safe_c2rust_g_hash_table_maybe_resize(hash_table);
    if deleted > 0 as guint {
        (*hash_table).version += 1;
    }
    return deleted;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_foreach_remove(
    mut hash_table: *mut GHashTable,
    mut func: GHRFunc,
    mut user_data: gpointer,
) -> guint {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if func.is_some() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return safe_c2rust_g_hash_table_foreach_remove_or_steal(hash_table, func, user_data, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_foreach_steal(
    mut hash_table: *mut GHashTable,
    mut func: GHRFunc,
    mut user_data: gpointer,
) -> guint {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if func.is_some() {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return safe_c2rust_g_hash_table_foreach_remove_or_steal(hash_table, func, user_data, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_foreach(
    mut hash_table: *mut GHashTable,
    mut func: GHFunc,
    mut user_data: gpointer,
) {
    let mut i: gsize = 0;
    let mut version: gint = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if func.is_some() {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
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
    version = (*hash_table).version as gint;
    i = 0 as gsize;
    while i < (*hash_table).size {
        let mut node_hash: guint = *(*hash_table).hashes.offset(i as isize);
        let mut node_key: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).keys,
            i as guint,
            (*hash_table).have_big_keys() as gboolean,
        );
        let mut node_value: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            i as guint,
            (*hash_table).have_big_values() as gboolean,
        );
        if node_hash >= 2 as guint {
            Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
                node_key, node_value, user_data,
            );
        }
        if ({
            let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
            if version == (*hash_table).version {
                _g_boolean_var_46 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_46 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_46
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"version == hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_find(
    mut hash_table: *mut GHashTable,
    mut predicate: GHRFunc,
    mut user_data: gpointer,
) -> gpointer {
    let mut i: gsize = 0;
    let mut version: gint = 0;
    let mut match_0: gboolean = 0;
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if predicate.is_some() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"predicate != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    version = (*hash_table).version as gint;
    match_0 = FALSE as gboolean;
    i = 0 as gsize;
    while i < (*hash_table).size {
        let mut node_hash: guint = *(*hash_table).hashes.offset(i as isize);
        let mut node_key: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).keys,
            i as guint,
            (*hash_table).have_big_keys() as gboolean,
        );
        let mut node_value: gpointer = safe_c2rust_g_hash_table_fetch_key_or_value(
            (*hash_table).values,
            i as guint,
            (*hash_table).have_big_values() as gboolean,
        );
        if node_hash >= 2 as guint {
            match_0 =
                predicate.expect("non-null function pointer")(node_key, node_value, user_data);
        }
        if ({
            let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
            if version == (*hash_table).version {
                _g_boolean_var_49 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_49 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_49
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"version == hash_table->version\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if match_0 != 0 {
            return node_value;
        }
        i = i.wrapping_add(1);
    }
    return NULL_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_size(mut hash_table: *mut GHashTable) -> guint {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*hash_table).nnodes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_get_keys(
    mut hash_table: *mut GHashTable,
) -> *mut GList {
    let mut i: gsize = 0;
    let mut retval: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    retval = ::core::ptr::null_mut::<GList>();
    i = 0 as gsize;
    while i < (*hash_table).size {
        if *(*hash_table).hashes.offset(i as isize) >= 2 as guint {
            retval = g_list_prepend(
                retval,
                safe_c2rust_g_hash_table_fetch_key_or_value(
                    (*hash_table).keys,
                    i as guint,
                    (*hash_table).have_big_keys() as gboolean,
                ),
            );
        }
        i = i.wrapping_add(1);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_get_keys_as_array(
    mut hash_table: *mut GHashTable,
    mut length: *mut guint,
) -> *mut gpointer {
    let mut result: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    let mut i: gsize = 0;
    let mut j: gsize = 0 as gsize;
    result = ({
        let mut __n: gsize = (*hash_table).nnodes.wrapping_add(1 as guint) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gpointer>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gpointer;
    i = 0 as gsize;
    while i < (*hash_table).size {
        if *(*hash_table).hashes.offset(i as isize) >= 2 as guint {
            let fresh7 = j;
            j = j.wrapping_add(1);
            let ref mut fresh8 = *result.offset(fresh7 as isize);
            *fresh8 = safe_c2rust_g_hash_table_fetch_key_or_value(
                (*hash_table).keys,
                i as guint,
                (*hash_table).have_big_keys() as gboolean,
            );
        }
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if j == (*hash_table).nnodes as gsize {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghash.c\0" as *const u8 as *const ::core::ffi::c_char,
            2284 as ::core::ffi::c_int,
            G_STRFUNC,
            b"j == hash_table->nnodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let ref mut fresh9 = *result.offset(j as isize);
    *fresh9 = NULL_0 as gpointer;
    if !length.is_null() {
        *length = j as guint;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_get_keys_as_ptr_array(
    mut hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = g_ptr_array_sized_new((*hash_table).size as guint);
    let mut i: gsize = 0 as gsize;
    while i < (*hash_table).size {
        if *(*hash_table).hashes.offset(i as isize) >= 2 as guint {
            g_ptr_array_add(
                array,
                safe_c2rust_g_hash_table_fetch_key_or_value(
                    (*hash_table).keys,
                    i as guint,
                    (*hash_table).have_big_keys() as gboolean,
                ),
            );
        }
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if (*array).len == (*hash_table).nnodes {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghash.c\0" as *const u8 as *const ::core::ffi::c_char,
            2327 as ::core::ffi::c_int,
            G_STRFUNC,
            b"array->len == hash_table->nnodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_get_values(
    mut hash_table: *mut GHashTable,
) -> *mut GList {
    let mut i: gsize = 0;
    let mut retval: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    retval = ::core::ptr::null_mut::<GList>();
    i = 0 as gsize;
    while i < (*hash_table).size {
        if *(*hash_table).hashes.offset(i as isize) >= 2 as guint {
            retval = g_list_prepend(
                retval,
                safe_c2rust_g_hash_table_fetch_key_or_value(
                    (*hash_table).values,
                    i as guint,
                    (*hash_table).have_big_values() as gboolean,
                ),
            );
        }
        i = i.wrapping_add(1);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hash_table_get_values_as_ptr_array(
    mut hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !hash_table.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"hash_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    array = g_ptr_array_sized_new((*hash_table).size as guint);
    let mut i: gsize = 0 as gsize;
    while i < (*hash_table).size {
        if *(*hash_table).hashes.offset(i as isize) >= 2 as guint {
            g_ptr_array_add(
                array,
                safe_c2rust_g_hash_table_fetch_key_or_value(
                    (*hash_table).values,
                    i as guint,
                    (*hash_table).have_big_values() as gboolean,
                ),
            );
        }
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if (*array).len == (*hash_table).nnodes {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/ghash.c\0" as *const u8 as *const ::core::ffi::c_char,
            2402 as ::core::ffi::c_int,
            G_STRFUNC,
            b"array->len == hash_table->nnodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    let mut string1: *const gchar = v1 as *const gchar;
    let mut string2: *const gchar = v2 as *const gchar;
    return (strcmp(
        string1 as *const ::core::ffi::c_char,
        string2 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_hash(mut v: gconstpointer) -> guint {
    let mut p: *const ::core::ffi::c_schar = ::core::ptr::null::<::core::ffi::c_schar>();
    let mut h: guint32 = 5381 as guint32;
    p = v as *const ::core::ffi::c_schar;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_add(h)
            .wrapping_add(*p as guint32);
        p = p.offset(1);
    }
    return h as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_direct_hash(mut v: gconstpointer) -> guint {
    return v as gulong as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_direct_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (v1 == v2) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_int_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (*(v1 as *const gint) == *(v2 as *const gint)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_int_hash(mut v: gconstpointer) -> guint {
    return *(v as *const gint) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uint_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (*(v1 as *const guint) == *(v2 as *const guint)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uint_hash(mut v: gconstpointer) -> guint {
    return *(v as *const guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_int64_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (*(v1 as *const gint64) == *(v2 as *const gint64)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_int64_hash(mut v: gconstpointer) -> guint {
    let mut bits: *const guint64 = v as *const guint64;
    return (*bits >> 32 as ::core::ffi::c_int ^ *bits & 0xffffffff as guint64) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_double_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (*(v1 as *const gdouble) == *(v2 as *const gdouble)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_double_hash(mut v: gconstpointer) -> guint {
    let mut bits: *const guint64 = v as *const guint64;
    return (*bits >> 32 as ::core::ffi::c_int ^ *bits & 0xffffffff as guint64) as guint;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const SIZEOF_INT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIZEOF_VOID_P: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
