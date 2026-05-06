extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
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
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRand {
    pub mt: [guint32; 624],
    pub mti: guint,
}
pub type GRand = _GRand;
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
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub const _IONBF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
static mut safe_c2rust_g__global_random_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
pub const N: ::core::ffi::c_int = 624 as ::core::ffi::c_int;
pub const M: ::core::ffi::c_int = 397 as ::core::ffi::c_int;
pub const MATRIX_A: ::core::ffi::c_uint = 0x9908b0df as ::core::ffi::c_uint;
pub const UPPER_MASK: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const LOWER_MASK: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
pub const TEMPERING_MASK_B: ::core::ffi::c_uint = 0x9d2c5680 as ::core::ffi::c_uint;
pub const TEMPERING_MASK_C: ::core::ffi::c_uint = 0xefc60000 as ::core::ffi::c_uint;
unsafe extern "C" fn safe_c2rust_get_random_version() -> guint {
    static mut safe_c2rust_initialized: gsize = FALSE as gsize;
    static mut safe_c2rust_random_version: guint = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut version_string: *const gchar =
            g_getenv(b"G_RANDOM_VERSION\0" as *const u8 as *const gchar);
        if version_string.is_null()
            || *version_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
            || strcmp(
                version_string as *const ::core::ffi::c_char,
                b"2.2\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_random_version = 22 as guint;
        } else if strcmp(
            version_string as *const ::core::ffi::c_char,
            b"2.0\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_random_version = 20 as guint;
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unknown G_RANDOM_VERSION \"%s\". Using version 2.2.\0" as *const u8
                    as *const gchar,
                version_string,
            );
            safe_c2rust_random_version = 22 as guint;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
    return safe_c2rust_random_version;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_new_with_seed(mut seed: guint32) -> *mut GRand {
    let mut rand: *mut GRand = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRand>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRand;
    safe_c2rust_g_rand_set_seed(rand, seed);
    return rand;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_new_with_seed_array(
    mut seed: *const guint32,
    mut seed_length: guint,
) -> *mut GRand {
    let mut rand: *mut GRand = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRand>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRand;
    safe_c2rust_g_rand_set_seed_array(rand, seed, seed_length);
    return rand;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_new() -> *mut GRand {
    let mut seed: [guint32; 4] = [0; 4];
    static mut safe_c2rust_dev_urandom_exists: gboolean = TRUE;
    if safe_c2rust_dev_urandom_exists != 0 {
        let mut dev_urandom: *mut FILE = ::core::ptr::null_mut::<FILE>();
        loop {
            dev_urandom = fopen(
                b"/dev/urandom\0" as *const u8 as *const ::core::ffi::c_char,
                b"rbe\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if !(({
                let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
                if dev_urandom.is_null() && *__errno_location() == 4 as ::core::ffi::c_int {
                    _g_boolean_var_8 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_8 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_8
            }) as ::core::ffi::c_long
                != 0)
            {
                break;
            }
        }
        if !dev_urandom.is_null() {
            let mut r: ::core::ffi::c_int = 0;
            setvbuf(
                dev_urandom,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                _IONBF,
                0 as size_t,
            );
            loop {
                *__errno_location() = 0 as ::core::ffi::c_int;
                r = fread(
                    &raw mut seed as *mut guint32 as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<[guint32; 4]>() as size_t,
                    1 as size_t,
                    dev_urandom,
                ) as ::core::ffi::c_int;
                if !(({
                    let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
                    if *__errno_location() == 4 as ::core::ffi::c_int {
                        _g_boolean_var_9 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_9 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_9
                }) as ::core::ffi::c_long
                    != 0)
                {
                    break;
                }
            }
            if r != 1 as ::core::ffi::c_int {
                safe_c2rust_dev_urandom_exists = FALSE as gboolean;
            }
            fclose(dev_urandom);
        } else {
            safe_c2rust_dev_urandom_exists = FALSE as gboolean;
        }
    }
    if safe_c2rust_dev_urandom_exists == 0 {
        let mut now_us: gint64 = g_get_real_time();
        seed[0 as ::core::ffi::c_int as usize] = (now_us / G_USEC_PER_SEC as gint64) as guint32;
        seed[1 as ::core::ffi::c_int as usize] = (now_us % G_USEC_PER_SEC as gint64) as guint32;
        seed[2 as ::core::ffi::c_int as usize] = getpid() as guint32;
        seed[3 as ::core::ffi::c_int as usize] = getppid() as guint32;
    }
    return safe_c2rust_g_rand_new_with_seed_array(&raw mut seed as *mut guint32, 4 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_free(mut rand: *mut GRand) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free(rand as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_copy(mut rand: *mut GRand) -> *mut GRand {
    let mut new_rand: *mut GRand = ::core::ptr::null_mut::<GRand>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GRand>();
    }
    new_rand = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRand>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GRand;
    memcpy(
        new_rand as *mut ::core::ffi::c_void,
        rand as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GRand>() as size_t,
    );
    return new_rand;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_set_seed(mut rand: *mut GRand, mut seed: guint32) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    match safe_c2rust_get_random_version() {
        20 => {
            if seed == 0 as guint32 {
                seed = 0x6b842128 as ::core::ffi::c_int as guint32;
            }
            (*rand).mt[0 as ::core::ffi::c_int as usize] = seed;
            (*rand).mti = 1 as guint;
            while (*rand).mti < N as guint {
                (*rand).mt[(*rand).mti as usize] = (69069 as ::core::ffi::c_int as guint32)
                    .wrapping_mul((*rand).mt[(*rand).mti.wrapping_sub(1 as guint) as usize]);
                (*rand).mti = (*rand).mti.wrapping_add(1);
            }
        }
        22 => {
            (*rand).mt[0 as ::core::ffi::c_int as usize] = seed;
            (*rand).mti = 1 as guint;
            while (*rand).mti < N as guint {
                (*rand).mt[(*rand).mti as usize] = (1812433253 as ::core::ffi::c_ulong)
                    .wrapping_mul(
                        ((*rand).mt[(*rand).mti.wrapping_sub(1 as guint) as usize]
                            ^ (*rand).mt[(*rand).mti.wrapping_sub(1 as guint) as usize]
                                >> 30 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulong,
                    )
                    .wrapping_add((*rand).mti as ::core::ffi::c_ulong)
                    as guint32;
                (*rand).mti = (*rand).mti.wrapping_add(1);
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/grand.c\0" as *const u8 as *const ::core::ffi::c_char,
                319 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_set_seed_array(
    mut rand: *mut GRand,
    mut seed: *const guint32,
    mut seed_length: guint,
) {
    let mut i: guint = 0;
    let mut j: guint = 0;
    let mut k: guint = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if seed_length >= 1 as guint {
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
            b"seed_length >= 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_rand_set_seed(rand, 19650218 as ::core::ffi::c_ulong as guint32);
    i = 1 as guint;
    j = 0 as guint;
    k = if N as guint > seed_length {
        N as guint
    } else {
        seed_length
    };
    while k != 0 {
        (*rand).mt[i as usize] = ((*rand).mt[i as usize] as ::core::ffi::c_ulong
            ^ (((*rand).mt[i.wrapping_sub(1 as guint) as usize]
                ^ (*rand).mt[i.wrapping_sub(1 as guint) as usize] >> 30 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong)
                .wrapping_mul(1664525 as ::core::ffi::c_ulong))
        .wrapping_add(*seed.offset(j as isize) as ::core::ffi::c_ulong)
        .wrapping_add(j as ::core::ffi::c_ulong) as guint32;
        (*rand).mt[i as usize] = ((*rand).mt[i as usize] as ::core::ffi::c_ulong
            & 0xffffffff as ::core::ffi::c_ulong) as guint32;
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
        if i >= N as guint {
            (*rand).mt[0 as ::core::ffi::c_int as usize] =
                (*rand).mt[(N - 1 as ::core::ffi::c_int) as usize];
            i = 1 as guint;
        }
        if j >= seed_length {
            j = 0 as guint;
        }
        k = k.wrapping_sub(1);
    }
    k = (N - 1 as ::core::ffi::c_int) as guint;
    while k != 0 {
        (*rand).mt[i as usize] = ((*rand).mt[i as usize] as ::core::ffi::c_ulong
            ^ (((*rand).mt[i.wrapping_sub(1 as guint) as usize]
                ^ (*rand).mt[i.wrapping_sub(1 as guint) as usize] >> 30 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong)
                .wrapping_mul(1566083941 as ::core::ffi::c_ulong))
        .wrapping_sub(i as ::core::ffi::c_ulong) as guint32;
        (*rand).mt[i as usize] = ((*rand).mt[i as usize] as ::core::ffi::c_ulong
            & 0xffffffff as ::core::ffi::c_ulong) as guint32;
        i = i.wrapping_add(1);
        if i >= N as guint {
            (*rand).mt[0 as ::core::ffi::c_int as usize] =
                (*rand).mt[(N - 1 as ::core::ffi::c_int) as usize];
            i = 1 as guint;
        }
        k = k.wrapping_sub(1);
    }
    (*rand).mt[0 as ::core::ffi::c_int as usize] = 0x80000000 as ::core::ffi::c_ulong as guint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_int(mut rand: *mut GRand) -> guint32 {
    let mut y: guint32 = 0;
    static mut safe_c2rust_mag01: [guint32; 2] = [0 as ::core::ffi::c_int as guint32, MATRIX_A];
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if (*rand).mti >= N as guint {
        let mut kk: ::core::ffi::c_int = 0;
        kk = 0 as ::core::ffi::c_int;
        while kk < N - M {
            y = (*rand).mt[kk as usize] & UPPER_MASK
                | (*rand).mt[(kk + 1 as ::core::ffi::c_int) as usize] & LOWER_MASK as guint32;
            (*rand).mt[kk as usize] = (*rand).mt[(kk + M) as usize]
                ^ y >> 1 as ::core::ffi::c_int
                ^ safe_c2rust_mag01[(y & 0x1 as guint32) as usize];
            kk += 1;
        }
        while kk < N - 1 as ::core::ffi::c_int {
            y = (*rand).mt[kk as usize] & UPPER_MASK
                | (*rand).mt[(kk + 1 as ::core::ffi::c_int) as usize] & LOWER_MASK as guint32;
            (*rand).mt[kk as usize] = (*rand).mt[(kk + (M - N)) as usize]
                ^ y >> 1 as ::core::ffi::c_int
                ^ safe_c2rust_mag01[(y & 0x1 as guint32) as usize];
            kk += 1;
        }
        y = (*rand).mt[(N - 1 as ::core::ffi::c_int) as usize] & UPPER_MASK
            | (*rand).mt[0 as ::core::ffi::c_int as usize] & LOWER_MASK as guint32;
        (*rand).mt[(N - 1 as ::core::ffi::c_int) as usize] = (*rand).mt
            [(M - 1 as ::core::ffi::c_int) as usize]
            ^ y >> 1 as ::core::ffi::c_int
            ^ safe_c2rust_mag01[(y & 0x1 as guint32) as usize];
        (*rand).mti = 0 as guint;
    }
    let fresh0 = (*rand).mti;
    (*rand).mti = (*rand).mti.wrapping_add(1);
    y = (*rand).mt[fresh0 as usize];
    y ^= y >> 11 as ::core::ffi::c_int;
    y ^= (y as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int & TEMPERING_MASK_B;
    y ^= (y as ::core::ffi::c_uint) << 15 as ::core::ffi::c_int & TEMPERING_MASK_C;
    y ^= y >> 18 as ::core::ffi::c_int;
    return y;
}
pub const G_RAND_DOUBLE_TRANSFORM: ::core::ffi::c_double = 2.3283064365386962890625e-10f64;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_int_range(
    mut rand: *mut GRand,
    mut begin: gint32,
    mut end: gint32,
) -> gint32 {
    let mut dist: guint32 = (end - begin) as guint32;
    let mut random: guint32 = 0 as guint32;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !rand.is_null() {
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
            b"rand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return begin;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if end > begin {
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
            b"end > begin\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return begin;
    }
    match safe_c2rust_get_random_version() {
        20 => {
            if dist as ::core::ffi::c_long <= 0x10000 as ::core::ffi::c_long {
                let mut double_rand: gdouble = safe_c2rust_g_rand_int(rand) as gdouble
                    * (G_RAND_DOUBLE_TRANSFORM + G_RAND_DOUBLE_TRANSFORM * G_RAND_DOUBLE_TRANSFORM);
                random = (double_rand * dist as gdouble) as gint32 as guint32;
            } else {
                random = safe_c2rust_g_rand_double_range(
                    rand,
                    0 as ::core::ffi::c_int as gdouble,
                    dist as gdouble,
                ) as gint32 as guint32;
            }
        }
        22 => {
            if dist == 0 as guint32 {
                random = 0 as guint32;
            } else {
                let mut maxvalue: guint32 = 0;
                if dist <= 0x80000000 as ::core::ffi::c_uint {
                    let mut leftover: guint32 = (0x80000000 as guint32)
                        .wrapping_rem(dist)
                        .wrapping_mul(2 as guint32);
                    if leftover >= dist {
                        leftover = leftover.wrapping_sub(dist);
                    }
                    maxvalue = (0xffffffff as guint32).wrapping_sub(leftover);
                } else {
                    maxvalue = dist.wrapping_sub(1 as guint32);
                }
                loop {
                    random = safe_c2rust_g_rand_int(rand);
                    if !(random > maxvalue) {
                        break;
                    }
                }
                random = random.wrapping_rem(dist);
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/grand.c\0" as *const u8 as *const ::core::ffi::c_char,
                517 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return (begin as guint32).wrapping_add(random) as gint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_double(mut rand: *mut GRand) -> gdouble {
    let mut retval: gdouble = safe_c2rust_g_rand_int(rand) as gdouble * G_RAND_DOUBLE_TRANSFORM;
    retval = ((retval as ::core::ffi::c_double
        + safe_c2rust_g_rand_int(rand) as ::core::ffi::c_double)
        * G_RAND_DOUBLE_TRANSFORM) as gdouble;
    if retval >= 1.0f64 {
        return safe_c2rust_g_rand_double(rand);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rand_double_range(
    mut rand: *mut GRand,
    mut begin: gdouble,
    mut end: gdouble,
) -> gdouble {
    let mut r: gdouble = 0.;
    r = safe_c2rust_g_rand_double(rand);
    return r * end - (r - 1 as ::core::ffi::c_int as gdouble) * begin;
}
unsafe extern "C" fn safe_c2rust_get_global_random() -> *mut GRand {
    static mut safe_c2rust_global_random: *mut GRand = ::core::ptr::null::<GRand>() as *mut GRand;
    if safe_c2rust_global_random.is_null() {
        safe_c2rust_global_random = safe_c2rust_g_rand_new();
    }
    return safe_c2rust_global_random;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_random_int() -> guint32 {
    let mut result: guint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__global_random_lock);
    result = safe_c2rust_g_rand_int(safe_c2rust_get_global_random());
    g_mutex_unlock(&raw mut safe_c2rust_g__global_random_lock);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_random_int_range(
    mut begin: gint32,
    mut end: gint32,
) -> gint32 {
    let mut result: gint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__global_random_lock);
    result = safe_c2rust_g_rand_int_range(safe_c2rust_get_global_random(), begin, end);
    g_mutex_unlock(&raw mut safe_c2rust_g__global_random_lock);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_random_double() -> gdouble {
    let mut result: ::core::ffi::c_double = 0.;
    g_mutex_lock(&raw mut safe_c2rust_g__global_random_lock);
    result = safe_c2rust_g_rand_double(safe_c2rust_get_global_random()) as ::core::ffi::c_double;
    g_mutex_unlock(&raw mut safe_c2rust_g__global_random_lock);
    return result as gdouble;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_random_double_range(
    mut begin: gdouble,
    mut end: gdouble,
) -> gdouble {
    let mut result: ::core::ffi::c_double = 0.;
    g_mutex_lock(&raw mut safe_c2rust_g__global_random_lock);
    result = safe_c2rust_g_rand_double_range(safe_c2rust_get_global_random(), begin, end)
        as ::core::ffi::c_double;
    g_mutex_unlock(&raw mut safe_c2rust_g__global_random_lock);
    return result as gdouble;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_random_set_seed(mut seed: guint32) {
    g_mutex_lock(&raw mut safe_c2rust_g__global_random_lock);
    safe_c2rust_g_rand_set_seed(safe_c2rust_get_global_random(), seed);
    g_mutex_unlock(&raw mut safe_c2rust_g__global_random_lock);
}
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_rand_set_seed\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
