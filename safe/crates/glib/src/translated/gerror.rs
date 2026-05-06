use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GHashTable;
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
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc0(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_rw_lock_writer_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_writer_unlock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_unlock(rw_lock: *mut GRWLock);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type uintptr_t = usize;
pub type va_list = __builtin_va_list;
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
pub const VG_USERREQ__RUNNING_ON_VALGRIND: C2RustUnnamed = 4097;
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GErrorInitFunc = Option<unsafe extern "C" fn(*mut GError) -> ()>;
pub type GErrorCopyFunc = Option<unsafe extern "C" fn(*const GError, *mut GError) -> ()>;
pub type GErrorClearFunc = Option<unsafe extern "C" fn(*mut GError) -> ()>;
pub type GRWLock = _GRWLock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
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
pub struct ErrorDomainInfo {
    pub private_size: gsize,
    pub init: GErrorInitFunc,
    pub copy: GErrorCopyFunc,
    pub clear: GErrorClearFunc,
}
pub type GHashTable = _GHashTable;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_error_domain_register_static\0" as *const u8 as *const ::core::ffi::c_char;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
static mut safe_c2rust_error_domain_global: GRWLock = _GRWLock {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_error_domain_ht: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_init() {
    safe_c2rust_error_domain_ht = g_hash_table_new(None, None);
}
#[inline]
unsafe extern "C" fn safe_c2rust_error_domain_lookup(mut domain: GQuark) -> *mut ErrorDomainInfo {
    return g_hash_table_lookup(
        safe_c2rust_error_domain_ht,
        domain as gulong as gpointer as gconstpointer,
    ) as *mut ErrorDomainInfo;
}
pub const STRUCT_ALIGNMENT: usize =
    (2 as usize).wrapping_mul(::core::mem::size_of::<gsize>() as usize);
unsafe extern "C" fn safe_c2rust_error_domain_register(
    mut error_quark: GQuark,
    mut error_type_private_size: gsize,
    mut error_type_init: GErrorInitFunc,
    mut error_type_copy: GErrorCopyFunc,
    mut error_type_clear: GErrorClearFunc,
) {
    g_rw_lock_writer_lock(&raw mut safe_c2rust_error_domain_global);
    if safe_c2rust_error_domain_lookup(error_quark).is_null() {
        let mut info: *mut ErrorDomainInfo = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ErrorDomainInfo>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut ErrorDomainInfo;
        (*info).private_size = ((error_type_private_size as usize)
            .wrapping_add(STRUCT_ALIGNMENT.wrapping_sub(1 as usize))
            & STRUCT_ALIGNMENT.wrapping_neg()) as gsize;
        (*info).init = error_type_init;
        (*info).copy = error_type_copy;
        (*info).clear = error_type_clear;
        g_hash_table_insert(
            safe_c2rust_error_domain_ht,
            error_quark as gulong as gpointer,
            info as gpointer,
        );
    } else {
        let mut name: *const ::core::ffi::c_char =
            g_quark_to_string(error_quark) as *const ::core::ffi::c_char;
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Attempted to register an extended error domain for %s more than once\0" as *const u8
                as *const gchar,
            name,
        );
    }
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_error_domain_global);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_domain_register_static(
    mut error_type_name: *const ::core::ffi::c_char,
    mut error_type_private_size: gsize,
    mut error_type_init: GErrorInitFunc,
    mut error_type_copy: GErrorCopyFunc,
    mut error_type_clear: GErrorClearFunc,
) -> GQuark {
    let mut error_quark: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !error_type_name.is_null() {
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
            b"error_type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if error_type_private_size > 0 as gsize {
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
            b"error_type_private_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if error_type_init.is_some() {
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
            b"error_type_init != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if error_type_copy.is_some() {
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
            b"error_type_copy != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if error_type_clear.is_some() {
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
            b"error_type_clear != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    error_quark = g_quark_from_static_string(error_type_name as *const gchar);
    safe_c2rust_error_domain_register(
        error_quark,
        error_type_private_size,
        error_type_init,
        error_type_copy,
        error_type_clear,
    );
    return error_quark;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_domain_register(
    mut error_type_name: *const ::core::ffi::c_char,
    mut error_type_private_size: gsize,
    mut error_type_init: GErrorInitFunc,
    mut error_type_copy: GErrorCopyFunc,
    mut error_type_clear: GErrorClearFunc,
) -> GQuark {
    let mut error_quark: GQuark = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !error_type_name.is_null() {
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
            b"error_type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error_type_private_size > 0 as gsize {
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
            b"error_type_private_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if error_type_init.is_some() {
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
            b"error_type_init != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if error_type_copy.is_some() {
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
            b"error_type_copy != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error_type_clear.is_some() {
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
            b"error_type_clear != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GQuark;
    }
    error_quark = g_quark_from_string(error_type_name as *const gchar);
    safe_c2rust_error_domain_register(
        error_quark,
        error_type_private_size,
        error_type_init,
        error_type_copy,
        error_type_clear,
    );
    return error_quark;
}
unsafe extern "C" fn safe_c2rust_g_error_allocate(
    mut domain: GQuark,
    mut out_info: *mut ErrorDomainInfo,
) -> *mut GError {
    let mut allocated: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut info: *mut ErrorDomainInfo = ::core::ptr::null_mut::<ErrorDomainInfo>();
    let mut private_size: gsize = 0;
    g_rw_lock_reader_lock(&raw mut safe_c2rust_error_domain_global);
    info = safe_c2rust_error_domain_lookup(domain);
    if !info.is_null() {
        if !out_info.is_null() {
            *out_info = *info;
        }
        private_size = (*info).private_size;
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_error_domain_global);
    } else {
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_error_domain_global);
        if !out_info.is_null() {
            memset(
                out_info as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ErrorDomainInfo>() as size_t,
            );
        }
        private_size = 0 as gsize;
    }
    if private_size > 0 as gsize
        && ({
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
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) =>
                fresh1, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t).offset(0
                as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            _zzq_result
        }) as ::core::ffi::c_uint
            != 0
    {
        private_size = (private_size as ::core::ffi::c_ulong).wrapping_add(
            ((1 as usize).wrapping_add(STRUCT_ALIGNMENT.wrapping_sub(1 as usize))
                & STRUCT_ALIGNMENT.wrapping_neg()) as ::core::ffi::c_ulong,
        ) as gsize as gsize;
        allocated = g_slice_alloc0(
            private_size
                .wrapping_add(::core::mem::size_of::<GError>() as gsize)
                .wrapping_add(::core::mem::size_of::<gpointer>() as gsize),
        ) as *mut guint8;
        let ref mut fresh3 = *(allocated
            .offset(private_size as isize)
            .offset(::core::mem::size_of::<GError>() as usize as isize)
            as *mut gpointer);
        *fresh3 = allocated.offset(
            ((1 as usize).wrapping_add(STRUCT_ALIGNMENT.wrapping_sub(1 as usize))
                & STRUCT_ALIGNMENT.wrapping_neg()) as isize,
        ) as gpointer;
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(private_size as isize) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (::core::mem::size_of::<GError>() as usize)
                    .wrapping_add(::core::mem::size_of::<gpointer>() as usize)
                    as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh4 = &mut _zzq_result;
            let fresh5;
            let fresh6 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6) =>
                fresh5, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t).offset(0
                as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        });
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(
                    ((1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg()) as isize,
                ) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (private_size as usize).wrapping_sub(
                    (1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg(),
                ) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh7 = &mut _zzq_result;
            let fresh8;
            let fresh9 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9) =>
                fresh8, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t).offset(0
                as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
        });
    } else {
        allocated =
            g_slice_alloc0(private_size.wrapping_add(::core::mem::size_of::<GError>() as gsize))
                as *mut guint8;
    }
    error = allocated.offset(private_size as isize) as *mut GError;
    return error;
}
unsafe extern "C" fn safe_c2rust_g_error_new_steal(
    mut domain: GQuark,
    mut code: gint,
    mut message: *mut gchar,
    mut out_info: *mut ErrorDomainInfo,
) -> *mut GError {
    let mut info: ErrorDomainInfo = ErrorDomainInfo {
        private_size: 0,
        init: None,
        copy: None,
        clear: None,
    };
    let mut error: *mut GError = safe_c2rust_g_error_allocate(domain, &raw mut info);
    (*error).domain = domain;
    (*error).code = code;
    (*error).message = message;
    if info.init.is_some() {
        info.init.expect("non-null function pointer")(error);
    }
    if !out_info.is_null() {
        *out_info = info;
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_new_valist(
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> *mut GError {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if !(({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gerror.c\0" as *const u8 as *const ::core::ffi::c_char,
            290 as ::core::ffi::c_int,
            G_STRFUNC,
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_g_error_new_steal(
        domain,
        code,
        g_strdup_vprintf(format, args.clone()),
        ::core::ptr::null_mut::<ErrorDomainInfo>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_new(
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) -> *mut GError {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    args_0 = args.clone();
    error = safe_c2rust_g_error_new_valist(domain, code, format, args_0.clone());
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_new_literal(
    mut domain: GQuark,
    mut code: gint,
    mut message: *const gchar,
) -> *mut GError {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !message.is_null() {
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
            b"message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    return safe_c2rust_g_error_new_steal(
        domain,
        code,
        safe_c2rust_g_strdup_inline(message as *const ::core::ffi::c_char) as *mut gchar,
        ::core::ptr::null_mut::<ErrorDomainInfo>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_free(mut error: *mut GError) {
    let mut private_size: gsize = 0;
    let mut info: *mut ErrorDomainInfo = ::core::ptr::null_mut::<ErrorDomainInfo>();
    let mut allocated: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rw_lock_reader_lock(&raw mut safe_c2rust_error_domain_global);
    info = safe_c2rust_error_domain_lookup((*error).domain);
    if !info.is_null() {
        let mut clear: GErrorClearFunc = (*info).clear;
        private_size = (*info).private_size;
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_error_domain_global);
        clear.expect("non-null function pointer")(error);
    } else {
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_error_domain_global);
        private_size = 0 as gsize;
    }
    g_free((*error).message as gpointer);
    allocated = (error as *mut guint8).offset(-(private_size as isize));
    if private_size > 0 as gsize
        && ({
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
            let fresh10 = &mut _zzq_result;
            let fresh11;
            let fresh12 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh12) =>
                fresh11, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh12, fresh11);
            _zzq_result
        }) as ::core::ffi::c_uint
            != 0
    {
        private_size = (private_size as ::core::ffi::c_ulong).wrapping_add(
            ((1 as usize).wrapping_add(STRUCT_ALIGNMENT.wrapping_sub(1 as usize))
                & STRUCT_ALIGNMENT.wrapping_neg()) as ::core::ffi::c_ulong,
        ) as gsize as gsize;
        allocated = allocated.offset(
            -(((1 as usize).wrapping_add(STRUCT_ALIGNMENT.wrapping_sub(1 as usize))
                & STRUCT_ALIGNMENT.wrapping_neg()) as isize),
        );
        let ref mut fresh13 = *(allocated
            .offset(private_size as isize)
            .offset(::core::mem::size_of::<GError>() as usize as isize)
            as *mut gpointer);
        *fresh13 = NULL as gpointer;
        g_slice_free1(
            private_size
                .wrapping_add(::core::mem::size_of::<GError>() as gsize)
                .wrapping_add(::core::mem::size_of::<gpointer>() as gsize),
            allocated as gpointer,
        );
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(
                    ((1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg()) as isize,
                ) as uintptr_t,
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
            let fresh14 = &mut _zzq_result;
            let fresh15;
            let fresh16 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) =>
                fresh15, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
        });
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                error as uintptr_t,
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
            let fresh17 = &mut _zzq_result;
            let fresh18;
            let fresh19 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19) =>
                fresh18, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
        });
    } else {
        g_slice_free1(
            private_size.wrapping_add(::core::mem::size_of::<GError>() as gsize),
            allocated as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_copy(mut error: *const GError) -> *mut GError {
    let mut copy: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut info: ErrorDomainInfo = ErrorDomainInfo {
        private_size: 0,
        init: None,
        copy: None,
        clear: None,
    };
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !(*error).message.is_null() {
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
            b"error->message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if !(({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*error).domain != 0 as GQuark {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gerror.c\0" as *const u8 as *const ::core::ffi::c_char,
            419 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error->domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    copy = safe_c2rust_g_error_new_steal(
        (*error).domain,
        (*error).code,
        safe_c2rust_g_strdup_inline((*error).message) as *mut gchar,
        &raw mut info,
    );
    if info.copy.is_some() {
        info.copy.expect("non-null function pointer")(error, copy);
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_error_matches(
    mut error: *const GError,
    mut domain: GQuark,
    mut code: gint,
) -> gboolean {
    return (!error.is_null() && (*error).domain == domain && (*error).code == code)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_error(
    mut err: *mut *mut GError,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut new: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut args_0: ::core::ffi::VaList;
    if err.is_null() {
        return;
    }
    args_0 = args.clone();
    new = safe_c2rust_g_error_new_valist(domain, code, format, args_0.clone());
    if (*err).is_null() {
        *err = new;
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"GError set over the top of a previous GError or uninitialized memory.\nThis indicates a bug in someone's code. You must ensure an error is NULL before it's set.\nThe overwriting error message was: %s\0"
                as *const u8 as *const gchar,
            (*new).message,
        );
        safe_c2rust_g_error_free(new);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_error_literal(
    mut err: *mut *mut GError,
    mut domain: GQuark,
    mut code: gint,
    mut message: *const gchar,
) {
    if err.is_null() {
        return;
    }
    if (*err).is_null() {
        *err = safe_c2rust_g_error_new_literal(domain, code, message);
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"GError set over the top of a previous GError or uninitialized memory.\nThis indicates a bug in someone's code. You must ensure an error is NULL before it's set.\nThe overwriting error message was: %s\0"
                as *const u8 as *const gchar,
            message,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_propagate_error(
    mut dest: *mut *mut GError,
    mut src: *mut GError,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !src.is_null() {
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
            b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if dest.is_null() {
        safe_c2rust_g_error_free(src);
        return;
    } else if !(*dest).is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"GError set over the top of a previous GError or uninitialized memory.\nThis indicates a bug in someone's code. You must ensure an error is NULL before it's set.\nThe overwriting error message was: %s\0"
                as *const u8 as *const gchar,
            (*src).message,
        );
        safe_c2rust_g_error_free(src);
    } else {
        *dest = src;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_clear_error(mut err: *mut *mut GError) {
    if !err.is_null() && !(*err).is_null() {
        safe_c2rust_g_error_free(*err);
        *err = ::core::ptr::null_mut::<GError>();
    }
}
unsafe extern "C" fn safe_c2rust_g_error_add_prefix(
    mut string: *mut *mut gchar,
    mut format: *const gchar,
    mut ap: ::core::ffi::VaList,
) {
    let mut oldstring: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut prefix: *mut gchar = ::core::ptr::null_mut::<gchar>();
    prefix = g_strdup_vprintf(format, ap.clone());
    oldstring = *string;
    *string = g_strconcat(prefix, oldstring, NULL);
    g_free(oldstring as gpointer);
    g_free(prefix as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_prefix_error(
    mut err: *mut *mut GError,
    mut format: *const gchar,
    mut args: ...
) {
    if !err.is_null() && !(*err).is_null() {
        let mut ap: ::core::ffi::VaList;
        ap = args.clone();
        safe_c2rust_g_error_add_prefix(&raw mut (**err).message, format, ap.clone());
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_prefix_error_literal(
    mut err: *mut *mut GError,
    mut prefix: *const gchar,
) {
    if !err.is_null() && !(*err).is_null() {
        let mut oldstring: *mut gchar = ::core::ptr::null_mut::<gchar>();
        oldstring = (**err).message;
        (**err).message = g_strconcat(prefix, oldstring, NULL);
        g_free(oldstring as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_propagate_prefixed_error(
    mut dest: *mut *mut GError,
    mut src: *mut GError,
    mut format: *const gchar,
    mut args: ...
) {
    safe_c2rust_g_propagate_error(dest, src);
    if !dest.is_null() {
        let mut ap: ::core::ffi::VaList;
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if !(*dest).is_null() {
                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_29
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gerror.c\0" as *const u8 as *const ::core::ffi::c_char,
                681 as ::core::ffi::c_int,
                G_STRFUNC,
                b"*dest != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ap = args.clone();
        safe_c2rust_g_error_add_prefix(&raw mut (**dest).message, format, ap.clone());
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
