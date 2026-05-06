use ::libc;
extern "C" {
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn pthread_self() -> pthread_t;
    fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn g_system_thread_wait(thread: *mut GRealThread);
    fn g_system_thread_new(
        proxy: GThreadFunc,
        stack_size: gulong,
        name: *const ::core::ffi::c_char,
        func: GThreadFunc,
        data: gpointer,
        error: *mut *mut GError,
    ) -> *mut GRealThread;
    fn g_system_thread_free(thread: *mut GRealThread);
    fn g_system_thread_exit() -> !;
    fn g_system_thread_set_name(name: *const gchar);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
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
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type pthread_t = ::core::ffi::c_ulong;
pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThread {
    pub func: GThreadFunc,
    pub data: gpointer,
    pub joinable: gboolean,
    pub priority: GThreadPriority,
}
pub type GThreadPriority = ::core::ffi::c_uint;
pub const G_THREAD_PRIORITY_URGENT: GThreadPriority = 3;
pub const G_THREAD_PRIORITY_HIGH: GThreadPriority = 2;
pub const G_THREAD_PRIORITY_NORMAL: GThreadPriority = 1;
pub const G_THREAD_PRIORITY_LOW: GThreadPriority = 0;
pub type GThread = _GThread;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub type GPrivate = _GPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOnce {
    pub status: GOnceStatus,
    pub retval: gpointer,
}
pub type GOnceStatus = ::core::ffi::c_uint;
pub const G_ONCE_STATUS_READY: GOnceStatus = 2;
pub const G_ONCE_STATUS_PROGRESS: GOnceStatus = 1;
pub const G_ONCE_STATUS_NOTCALLED: GOnceStatus = 0;
pub type GOnce = _GOnce;
pub type GRealThread = _GRealThread;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealThread {
    pub thread: GThread,
    pub ref_count: gint,
    pub ours: gboolean,
    pub name: *mut gchar,
    pub retval: gpointer,
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
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub type __cpu_mask = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __CPU_SETSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __NCPUBITS: usize =
    (8 as usize).wrapping_mul(::core::mem::size_of::<__cpu_mask>() as usize);
pub const CPU_SETSIZE: ::core::ffi::c_int = __CPU_SETSIZE;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g_thread_error\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
static mut safe_c2rust_g_once_mutex: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_g_once_cond: GCond = _GCond {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_g_once_init_list: *mut GSList = ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_g_thread_n_created_counter: guint = 0 as guint;
static mut safe_c2rust_g_thread_specific_private: GPrivate = unsafe {
    _GPrivate {
        p: NULL,
        notify: Some(safe_c2rust_g_thread_cleanup as unsafe extern "C" fn(gpointer) -> ()),
        future: [NULL, NULL],
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_private_set_alloc0(
    mut key: *mut GPrivate,
    mut size: gsize,
) -> gpointer {
    let mut allocated: gpointer = g_malloc0(size);
    g_private_set(key, allocated);
    return safe_c2rust_g_steal_pointer(&raw mut allocated as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_impl(
    mut once: *mut GOnce,
    mut func: GThreadFunc,
    mut arg: gpointer,
) -> gpointer {
    g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
    while (*once).status as ::core::ffi::c_uint
        == G_ONCE_STATUS_PROGRESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_cond_wait(
            &raw mut safe_c2rust_g_once_cond,
            &raw mut safe_c2rust_g_once_mutex,
        );
    }
    if (*once).status as ::core::ffi::c_uint
        != G_ONCE_STATUS_READY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ::core::ptr::write_volatile(
            &mut (*once).status as *mut GOnceStatus,
            G_ONCE_STATUS_PROGRESS,
        );
        g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
        retval = func.expect("non-null function pointer")(arg);
        g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
        ::core::ptr::write_volatile(&mut (*once).retval as *mut gpointer, retval);
        crate::translated::compat::atomic_store_release(&raw mut (*once).status, G_ONCE_STATUS_READY);
        g_cond_broadcast(&raw mut safe_c2rust_g_once_cond);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
    return (*once).retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_init_enter(
    mut location: *mut ::core::ffi::c_void,
) -> gboolean {
    let mut value_location: *mut gsize = location as *mut gsize;
    let mut need_init: gboolean = FALSE;
    g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
    if ({
        let mut gapg_temp_newval: gsize = 0;
        let mut gapg_temp_atomic: *mut gsize = value_location as *mut gsize;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) == 0 as gsize
    {
        if g_slist_find(
            safe_c2rust_g_once_init_list,
            value_location as *mut ::core::ffi::c_void as gconstpointer,
        )
        .is_null()
        {
            need_init = TRUE as gboolean;
            safe_c2rust_g_once_init_list =
                g_slist_prepend(safe_c2rust_g_once_init_list, value_location as gpointer);
        } else {
            loop {
                g_cond_wait(
                    &raw mut safe_c2rust_g_once_cond,
                    &raw mut safe_c2rust_g_once_mutex,
                );
                if g_slist_find(
                    safe_c2rust_g_once_init_list,
                    value_location as *mut ::core::ffi::c_void as gconstpointer,
                )
                .is_null()
                {
                    break;
                }
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
    return need_init;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_init_enter_pointer(mut location: gpointer) -> gboolean {
    let mut value_location: *mut gpointer = location as *mut gpointer;
    let mut need_init: gboolean = FALSE;
    g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
    if ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = value_location as *mut gpointer;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    })
    .is_null()
    {
        if g_slist_find(
            safe_c2rust_g_once_init_list,
            value_location as *mut ::core::ffi::c_void as gconstpointer,
        )
        .is_null()
        {
            need_init = TRUE as gboolean;
            safe_c2rust_g_once_init_list =
                g_slist_prepend(safe_c2rust_g_once_init_list, value_location as gpointer);
        } else {
            loop {
                g_cond_wait(
                    &raw mut safe_c2rust_g_once_cond,
                    &raw mut safe_c2rust_g_once_mutex,
                );
                if g_slist_find(
                    safe_c2rust_g_once_init_list,
                    value_location as *mut ::core::ffi::c_void as gconstpointer,
                )
                .is_null()
                {
                    break;
                }
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
    return need_init;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_init_leave(
    mut location: *mut ::core::ffi::c_void,
    mut result: gsize,
) {
    let mut value_location: *mut gsize = location as *mut gsize;
    let mut old_value: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if result != 0 as gsize {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"result != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    old_value = ({
        if 0 as ::core::ffi::c_int != 0 {
            *value_location;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(value_location, result) as gpointer
    }) as gsize;
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if old_value == 0 as gsize {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"old_value == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if !safe_c2rust_g_once_init_list.is_null() {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_once_init_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_once_init_list = g_slist_remove(
        safe_c2rust_g_once_init_list,
        value_location as *mut ::core::ffi::c_void as gconstpointer,
    );
    g_cond_broadcast(&raw mut safe_c2rust_g_once_cond);
    g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_init_leave_pointer(
    mut location: gpointer,
    mut result: gpointer,
) {
    let mut value_location: *mut gpointer = location as *mut gpointer;
    let mut old_value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
        if !result.is_null() {
            _g_boolean_var_7 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_7 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_7
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"result != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    old_value = ({
        if 0 as ::core::ffi::c_int != 0 {
            *value_location;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(value_location, result)
    });
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if old_value.is_null() {
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
            b"old_value == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g_once_mutex);
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !safe_c2rust_g_once_init_list.is_null() {
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
            b"g_once_init_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_once_init_list = g_slist_remove(
        safe_c2rust_g_once_init_list,
        value_location as *mut ::core::ffi::c_void as gconstpointer,
    );
    g_cond_broadcast(&raw mut safe_c2rust_g_once_cond);
    g_mutex_unlock(&raw mut safe_c2rust_g_once_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_ref(mut thread: *mut GThread) -> *mut GThread {
    let mut real: *mut GRealThread = thread as *mut GRealThread;
    if 0 as ::core::ffi::c_int != 0 {
        (*real).ref_count;
        (*real).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*real).ref_count, 1 as ::core::ffi::c_int);
    return thread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_unref(mut thread: *mut GThread) {
    let mut real: *mut GRealThread = thread as *mut GRealThread;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*real).ref_count;
            (*real).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*real).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if (*real).ours != 0 {
            g_system_thread_free(real);
        } else {
            g_slice_free1(
                ::core::mem::size_of::<GRealThread>() as gsize,
                real as gpointer,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_thread_cleanup(mut data: gpointer) {
    safe_c2rust_g_thread_unref(data as *mut GThread);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_proxy(mut data: gpointer) -> gpointer {
    let mut thread: *mut GRealThread = data as *mut GRealThread;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !data.is_null() {
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
            b"../original/glib/gthread.c\0" as *const u8 as *const ::core::ffi::c_char,
            822 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_private_set(&raw mut safe_c2rust_g_thread_specific_private, data);
    if !(*thread).name.is_null() {
        g_system_thread_set_name((*thread).name);
        g_free((*thread).name as gpointer);
        (*thread).name = ::core::ptr::null_mut::<gchar>();
    }
    (*thread).retval =
        (*thread).thread.func.expect("non-null function pointer")((*thread).thread.data);
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_n_created() -> guint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_thread_n_created_counter;
            safe_c2rust_g_thread_n_created_counter;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut safe_c2rust_g_thread_n_created_counter as *mut gint,
        );
        gaig_temp
    }) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_new(
    mut name: *const gchar,
    mut func: GThreadFunc,
    mut data: gpointer,
) -> *mut GThread {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut thread: *mut GThread = ::core::ptr::null_mut::<GThread>();
    thread = safe_c2rust_g_thread_new_internal(
        name,
        Some(safe_c2rust_g_thread_proxy as unsafe extern "C" fn(gpointer) -> gpointer),
        func,
        data,
        0 as gsize,
        &raw mut error,
    );
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if thread.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"creating thread '%s': %s\0" as *const u8 as *const gchar,
            if !name.is_null() {
                name as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            (*error).message,
        );
        loop {}
    }
    return thread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_try_new(
    mut name: *const gchar,
    mut func: GThreadFunc,
    mut data: gpointer,
    mut error: *mut *mut GError,
) -> *mut GThread {
    return safe_c2rust_g_thread_new_internal(
        name,
        Some(safe_c2rust_g_thread_proxy as unsafe extern "C" fn(gpointer) -> gpointer),
        func,
        data,
        0 as gsize,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_new_internal(
    mut name: *const gchar,
    mut proxy: GThreadFunc,
    mut func: GThreadFunc,
    mut data: gpointer,
    mut stack_size: gsize,
    mut error: *mut *mut GError,
) -> *mut GThread {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GThread>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_g_thread_n_created_counter;
        safe_c2rust_g_thread_n_created_counter;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut safe_c2rust_g_thread_n_created_counter,
        1 as ::core::ffi::c_int as guint,
    );
    return g_system_thread_new(
        proxy,
        stack_size as gulong,
        name as *const ::core::ffi::c_char,
        func,
        data,
        error,
    ) as *mut GThread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_exit(mut retval: gpointer) -> ! {
    let mut real: *mut GRealThread = safe_c2rust_g_thread_self() as *mut GRealThread;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*real).ours == 0 {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"attempt to g_thread_exit() a thread not created by GLib\0" as *const u8
                as *const gchar,
        );
        loop {}
    }
    (*real).retval = retval;
    g_system_thread_exit();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_join(mut thread: *mut GThread) -> gpointer {
    let mut real: *mut GRealThread = thread as *mut GRealThread;
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !thread.is_null() {
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
            b"thread\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*real).ours != 0 {
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
            b"real->ours\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_system_thread_wait(real);
    retval = (*real).retval;
    (*thread).joinable = 0 as ::core::ffi::c_int as gboolean;
    safe_c2rust_g_thread_unref(thread);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_self() -> *mut GThread {
    let mut thread: *mut GRealThread =
        g_private_get(&raw mut safe_c2rust_g_thread_specific_private) as *mut GRealThread;
    if thread.is_null() {
        thread = ({
            let mut __s: gsize = ::core::mem::size_of::<GRealThread>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut GRealThread;
        (*thread).ref_count = 1 as ::core::ffi::c_int as gint;
        g_private_set(
            &raw mut safe_c2rust_g_thread_specific_private,
            thread as gpointer,
        );
    }
    return thread as *mut GThread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_num_processors() -> guint {
    let mut idx: ::core::ffi::c_int = 0;
    let mut ncores: ::core::ffi::c_int =
        (if sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int) < 1024 as ::core::ffi::c_long {
            sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int)
        } else {
            1024 as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
    let mut cpu_mask: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    ::libc::memset(
        &raw mut cpu_mask as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<cpu_set_t>() as ::libc::size_t,
    );
    let mut af_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int = pthread_getaffinity_np(
        pthread_self(),
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &raw mut cpu_mask,
    );
    if err == 0 {
        idx = 0 as ::core::ffi::c_int;
        while idx < ncores && idx < CPU_SETSIZE {
            af_count += ({
                let mut __cpu: size_t = idx as size_t;
                if __cpu.wrapping_div(8 as size_t) < ::core::mem::size_of::<cpu_set_t>() as usize {
                    (*(&raw mut cpu_mask.__bits as *mut __cpu_mask as *const __cpu_mask)
                        .offset(__cpu.wrapping_div(__NCPUBITS) as isize)
                        & (1 as ::core::ffi::c_int as __cpu_mask) << __cpu.wrapping_rem(__NCPUBITS)
                        != 0 as __cpu_mask) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            });
            idx += 1;
        }
    }
    let mut count: ::core::ffi::c_int = if af_count > 0 as ::core::ffi::c_int {
        af_count
    } else {
        ncores
    };
    return count as guint;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_thread_proxy\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
