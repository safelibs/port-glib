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
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn __lsan_ignore_object(p: *const ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type GHashTable = _GHashTable;
pub type GQuark = guint32;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_ignore_leak(mut p: gconstpointer) {
    if !p.is_null()
        && Some(__lsan_ignore_object as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ())
            .is_some()
    {
        __lsan_ignore_object(p as *const ::core::ffi::c_void);
    }
}
pub const QUARK_BLOCK_SIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const QUARK_STRING_BLOCK_SIZE: usize =
    (4096 as usize).wrapping_sub(::core::mem::size_of::<gsize>() as usize);
static mut safe_c2rust_g__quark_global_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_quark_ht: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_quarks: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_quark_seq_id: gint = 0 as gint;
static mut safe_c2rust_quark_block: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_quark_block_offset: gint = 0 as gint;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_quark_init() {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_quark_seq_id == 0 as ::core::ffi::c_int {
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
            b"../original/glib/gquark.c\0" as *const u8 as *const ::core::ffi::c_char,
            63 as ::core::ffi::c_int,
            G_STRFUNC,
            b"quark_seq_id == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_quark_ht = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_quarks = ({
        let mut __n: gsize = 2048 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut gchar;
    let ref mut fresh0 = *safe_c2rust_quarks.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = ::core::ptr::null_mut::<gchar>();
    safe_c2rust_quark_seq_id = 1 as ::core::ffi::c_int as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_quark_try_string(mut string: *const gchar) -> GQuark {
    let mut quark: GQuark = 0 as GQuark;
    if string.is_null() {
        return 0 as GQuark;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__quark_global_lock);
    quark = g_hash_table_lookup(safe_c2rust_quark_ht, string as gconstpointer) as gulong as guint
        as GQuark;
    g_mutex_unlock(&raw mut safe_c2rust_g__quark_global_lock);
    return quark;
}
unsafe extern "C" fn safe_c2rust_quark_strdup(
    mut string: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    len = strlen(string as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
    if len as usize > QUARK_STRING_BLOCK_SIZE.wrapping_div(2 as usize) {
        return safe_c2rust_g_strdup_inline(string as *const ::core::ffi::c_char);
    }
    if safe_c2rust_quark_block.is_null()
        || QUARK_STRING_BLOCK_SIZE.wrapping_sub(safe_c2rust_quark_block_offset as usize)
            < len as usize
    {
        safe_c2rust_quark_block = g_malloc(QUARK_STRING_BLOCK_SIZE as gsize) as *mut gchar;
        safe_c2rust_quark_block_offset = 0 as ::core::ffi::c_int as gint;
    }
    copy = safe_c2rust_quark_block.offset(safe_c2rust_quark_block_offset as isize);
    memcpy(
        copy as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        len as size_t,
    );
    safe_c2rust_quark_block_offset =
        (safe_c2rust_quark_block_offset as gsize).wrapping_add(len) as gint as gint;
    return copy as *mut ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn safe_c2rust_quark_from_string(
    mut string: *const gchar,
    mut duplicate: gboolean,
) -> GQuark {
    let mut quark: GQuark = 0 as GQuark;
    quark = g_hash_table_lookup(safe_c2rust_quark_ht, string as gconstpointer) as gulong as guint
        as GQuark;
    if quark == 0 {
        quark = safe_c2rust_quark_new(if duplicate != 0 {
            safe_c2rust_quark_strdup(string) as *mut gchar
        } else {
            string as *mut gchar
        });
    }
    return quark;
}
#[inline]
unsafe extern "C" fn safe_c2rust_quark_from_string_locked(
    mut string: *const gchar,
    mut duplicate: gboolean,
) -> GQuark {
    let mut quark: GQuark = 0 as GQuark;
    if string.is_null() {
        return 0 as GQuark;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__quark_global_lock);
    quark = safe_c2rust_quark_from_string(string, duplicate);
    g_mutex_unlock(&raw mut safe_c2rust_g__quark_global_lock);
    return quark;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_quark_from_string(mut string: *const gchar) -> GQuark {
    return safe_c2rust_quark_from_string_locked(string, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_quark_from_static_string(
    mut string: *const gchar,
) -> GQuark {
    return safe_c2rust_quark_from_string_locked(string, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_quark_to_string(mut quark: GQuark) -> *const gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut strings: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut seq_id: guint = 0;
    seq_id = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_quark_seq_id;
            safe_c2rust_quark_seq_id;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_quark_seq_id);
        gaig_temp
    }) as guint;
    strings = ({
        let mut gapg_temp_newval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut gapg_temp_atomic: *mut *mut *mut gchar = &raw mut safe_c2rust_quarks;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut *mut gchar;
    if quark < seq_id {
        result = *strings.offset(quark as isize);
    }
    return result;
}
#[inline]
unsafe extern "C" fn safe_c2rust_quark_new(mut string: *mut gchar) -> GQuark {
    let mut quark: GQuark = 0;
    let mut quarks_new: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if safe_c2rust_quark_seq_id as ::core::ffi::c_int % QUARK_BLOCK_SIZE == 0 as ::core::ffi::c_int
    {
        quarks_new = ({
            let mut __n: gsize = (safe_c2rust_quark_seq_id as ::core::ffi::c_int
                + 2048 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        if safe_c2rust_quark_seq_id != 0 as ::core::ffi::c_int {
            memcpy(
                quarks_new as *mut ::core::ffi::c_void,
                safe_c2rust_quarks as *const ::core::ffi::c_void,
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                    .wrapping_mul(safe_c2rust_quark_seq_id as size_t),
            );
        }
        memset(
            quarks_new.offset(safe_c2rust_quark_seq_id as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(QUARK_BLOCK_SIZE as size_t),
        );
        safe_c2rust_g_ignore_leak(
            ({
                let mut gapg_temp_newval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
                let mut gapg_temp_atomic: *mut *mut *mut gchar = &raw mut safe_c2rust_quarks;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) as gconstpointer,
        );
        let mut gaps_temp_atomic: *mut *mut *mut gchar = &raw mut safe_c2rust_quarks;
        let mut gaps_temp_newval: *mut *mut gchar = quarks_new as *mut *mut gchar;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_quarks;
        } else {
        };
        crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
    }
    quark = safe_c2rust_quark_seq_id as GQuark;
    let mut gaps_temp_atomic_0: *mut *mut gchar =
        safe_c2rust_quarks.offset(quark as isize) as *mut *mut gchar;
    let mut gaps_temp_newval_0: *mut gchar = string as *mut gchar;
    if 0 as ::core::ffi::c_int != 0 {
        *safe_c2rust_quarks.offset(quark as isize);
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic_0, *&raw mut gaps_temp_newval_0);
    g_hash_table_insert(
        safe_c2rust_quark_ht,
        string as gpointer,
        quark as gulong as gpointer,
    );
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_quark_seq_id;
        safe_c2rust_quark_seq_id;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut safe_c2rust_quark_seq_id,
        1 as ::core::ffi::c_int,
    );
    return quark;
}
#[inline]
unsafe extern "C" fn safe_c2rust_quark_intern_string_locked(
    mut string: *const gchar,
    mut duplicate: gboolean,
) -> *const gchar {
    let mut result: *const gchar = ::core::ptr::null::<gchar>();
    let mut quark: GQuark = 0;
    if string.is_null() {
        return ::core::ptr::null::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__quark_global_lock);
    quark = safe_c2rust_quark_from_string(string, duplicate);
    result = *safe_c2rust_quarks.offset(quark as isize);
    g_mutex_unlock(&raw mut safe_c2rust_g__quark_global_lock);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_intern_string(mut string: *const gchar) -> *const gchar {
    return safe_c2rust_quark_intern_string_locked(string, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_intern_static_string(
    mut string: *const gchar,
) -> *const gchar {
    return safe_c2rust_quark_intern_string_locked(string, FALSE);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_quark_init\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
