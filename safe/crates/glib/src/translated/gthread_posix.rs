extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn g_thread_error_quark() -> GQuark;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
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
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn sched_yield() -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_exit(__retval: *mut ::core::ffi::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
    fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> ::core::ffi::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut ::core::ffi::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    static mut safe_c2rust_stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __rwelision: ::core::ffi::c_schar,
    pub __pad1: [::core::ffi::c_uchar; 7],
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
pub type pthread_key_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [::core::ffi::c_char; 8],
    pub __align: ::core::ffi::c_long,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_THREAD_ERROR_AGAIN: C2RustUnnamed = 0;
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
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRecMutex = _GRecMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRWLock = _GRWLock;
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
pub const G_MUTEX_STATE_EMPTY: GMutexState = 0;
pub type FILE = _IO_FILE;
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
pub const G_MUTEX_STATE_CONTENDED: GMutexState = 2;
pub type atomic_uint = ::core::ffi::c_uint;
pub const G_MUTEX_STATE_OWNED: GMutexState = 1;
pub type GMutexState = ::core::ffi::c_uint;
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
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tv_sec: __kernel_time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PTHREAD_EXPLICIT_SCHED: C2RustUnnamed_2 = 1;
pub const PTHREAD_INHERIT_SCHED: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealThread {
    pub thread: GThread,
    pub ref_count: gint,
    pub ours: gboolean,
    pub name: *mut gchar,
    pub retval: gpointer,
}
pub type GRealThread = _GRealThread;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_3 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_3 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_3 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_3 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_3 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_3 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_3 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_3 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_3 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_3 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_3 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_3 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_3 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_3 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_3 = 236;
pub const _SC_IPV6: C2RustUnnamed_3 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_3 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_3 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_3 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_3 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_3 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_3 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_3 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_3 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_3 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_3 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_3 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_3 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_3 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_3 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_3 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_3 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_3 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_3 = 182;
pub const _SC_TRACE: C2RustUnnamed_3 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_3 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_3 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_3 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_3 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_3 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_3 = 175;
pub const _SC_STREAMS: C2RustUnnamed_3 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_3 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_3 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_3 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_3 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_3 = 169;
pub const _SC_2_PBS: C2RustUnnamed_3 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_3 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_3 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_3 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_3 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_3 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_3 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_3 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_3 = 160;
pub const _SC_SPAWN: C2RustUnnamed_3 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_3 = 158;
pub const _SC_SHELL: C2RustUnnamed_3 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_3 = 156;
pub const _SC_REGEXP: C2RustUnnamed_3 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_3 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_3 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_3 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_3 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_3 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_3 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_3 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_3 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_3 = 146;
pub const _SC_PIPE: C2RustUnnamed_3 = 145;
pub const _SC_FIFO: C2RustUnnamed_3 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_3 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_3 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_3 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_3 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_3 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_3 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_3 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_3 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_3 = 135;
pub const _SC_BASE: C2RustUnnamed_3 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_3 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_3 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_3 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_3 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_3 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_3 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_3 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_3 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_3 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_3 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_3 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_3 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_3 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_3 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_3 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_3 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_3 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_3 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_3 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_3 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_3 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_3 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_3 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_3 = 110;
pub const _SC_NZERO: C2RustUnnamed_3 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_3 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_3 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_3 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_3 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_3 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_3 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_3 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_3 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_3 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_3 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_3 = 98;
pub const _SC_2_UPE: C2RustUnnamed_3 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_3 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_3 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_3 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_3 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_3 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_3 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_3 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_3 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_3 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_3 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_3 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_3 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_3 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_3 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_3 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_3 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_3 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_3 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_3 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_3 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_3 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_3 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_3 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_3 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_3 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_3 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_3 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_3 = 68;
pub const _SC_THREADS: C2RustUnnamed_3 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_3 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_3 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_3 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_3 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_3 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_3 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_3 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_3 = 60;
pub const _SC_SELECT: C2RustUnnamed_3 = 59;
pub const _SC_POLL: C2RustUnnamed_3 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_3 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_3 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_3 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_3 = 54;
pub const _SC_PII: C2RustUnnamed_3 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_3 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_3 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_3 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_3 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_3 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_3 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_3 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_3 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_3 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_3 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_3 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_3 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_3 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_3 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_3 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_3 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_3 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_3 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_3 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_3 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_3 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_3 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_3 = 30;
pub const _SC_VERSION: C2RustUnnamed_3 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_3 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_3 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_3 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_3 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_3 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_3 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_3 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_3 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_3 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_3 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_3 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_3 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_3 = 16;
pub const _SC_FSYNC: C2RustUnnamed_3 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_3 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_3 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_3 = 12;
pub const _SC_TIMERS: C2RustUnnamed_3 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_3 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_3 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_3 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_3 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_3 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_3 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_3 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_3 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_3 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GThreadPosix {
    pub thread: GRealThread,
    pub system_thread: pthread_t,
    pub joined: gboolean,
    pub lock: GMutex,
    pub proxy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
}
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
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
pub const __NR_futex: ::core::ffi::c_int = 202 as ::core::ffi::c_int;
pub const FUTEX_WAIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FUTEX_PRIVATE_FLAG: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const FUTEX_WAIT_PRIVATE: ::core::ffi::c_int = FUTEX_WAIT | FUTEX_PRIVATE_FLAG;
unsafe extern "C" fn safe_c2rust_g_thread_abort(mut status: gint, mut function: *const gchar) {
    fprintf(
        safe_c2rust_stderr,
        b"GLib (gthread-posix.c): Unexpected error from C library during '%s': %s.  Aborting.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        function,
        strerror(status as ::core::ffi::c_int),
    );
    abort();
}
unsafe extern "C" fn safe_c2rust_g_rec_mutex_impl_new() -> *mut pthread_mutex_t {
    let mut attr: pthread_mutexattr_t = pthread_mutexattr_t { __size: [0; 4] };
    let mut mutex: *mut pthread_mutex_t = ::core::ptr::null_mut::<pthread_mutex_t>();
    mutex = malloc(::core::mem::size_of::<pthread_mutex_t>() as size_t) as *mut pthread_mutex_t;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if mutex.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            *__errno_location(),
            b"malloc\0" as *const u8 as *const gchar,
        );
    }
    pthread_mutexattr_init(&raw mut attr);
    pthread_mutexattr_settype(&raw mut attr, PTHREAD_MUTEX_RECURSIVE as ::core::ffi::c_int);
    pthread_mutex_init(mutex, &raw mut attr);
    pthread_mutexattr_destroy(&raw mut attr);
    return mutex;
}
unsafe extern "C" fn safe_c2rust_g_rec_mutex_impl_free(mut mutex: *mut pthread_mutex_t) {
    pthread_mutex_destroy(mutex);
    free(mutex as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_rec_mutex_get_impl(
    mut rec_mutex: *mut GRecMutex,
) -> *mut pthread_mutex_t {
    let mut impl_0: *mut pthread_mutex_t = ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = &raw mut (*rec_mutex).p;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut pthread_mutex_t;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if impl_0.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        impl_0 = safe_c2rust_g_rec_mutex_impl_new();
        if ({
            let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if 0 as ::core::ffi::c_int != 0 {
                (*rec_mutex).p;
            } else {
            };
            let fresh3 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*rec_mutex).p,
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
                impl_0 as gpointer,
            );
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh3.0;
            if fresh3.1 as ::core::ffi::c_int != 0 {
                TRUE
            } else {
                FALSE
            }
        }) == 0
        {
            safe_c2rust_g_rec_mutex_impl_free(impl_0);
        }
        impl_0 = (*rec_mutex).p as *mut pthread_mutex_t;
    }
    return impl_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rec_mutex_init(mut rec_mutex: *mut GRecMutex) {
    (*rec_mutex).p = safe_c2rust_g_rec_mutex_impl_new() as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rec_mutex_clear(mut rec_mutex: *mut GRecMutex) {
    safe_c2rust_g_rec_mutex_impl_free((*rec_mutex).p as *mut pthread_mutex_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rec_mutex_lock(mut mutex: *mut GRecMutex) {
    pthread_mutex_lock(safe_c2rust_g_rec_mutex_get_impl(mutex));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rec_mutex_unlock(mut rec_mutex: *mut GRecMutex) {
    pthread_mutex_unlock((*rec_mutex).p as *mut pthread_mutex_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rec_mutex_trylock(
    mut rec_mutex: *mut GRecMutex,
) -> gboolean {
    if pthread_mutex_trylock(safe_c2rust_g_rec_mutex_get_impl(rec_mutex)) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_rw_lock_impl_new() -> *mut pthread_rwlock_t {
    let mut rwlock: *mut pthread_rwlock_t = ::core::ptr::null_mut::<pthread_rwlock_t>();
    let mut status: gint = 0;
    rwlock = malloc(::core::mem::size_of::<pthread_rwlock_t>() as size_t) as *mut pthread_rwlock_t;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if rwlock.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            *__errno_location(),
            b"malloc\0" as *const u8 as *const gchar,
        );
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        status = pthread_rwlock_init(rwlock, ::core::ptr::null::<pthread_rwlockattr_t>()) as gint;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            status,
            b"pthread_rwlock_init\0" as *const u8 as *const gchar,
        );
    }
    return rwlock;
}
unsafe extern "C" fn safe_c2rust_g_rw_lock_impl_free(mut rwlock: *mut pthread_rwlock_t) {
    pthread_rwlock_destroy(rwlock);
    free(rwlock as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_rw_lock_get_impl(
    mut lock: *mut GRWLock,
) -> *mut pthread_rwlock_t {
    let mut impl_0: *mut pthread_rwlock_t = ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = &raw mut (*lock).p;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut pthread_rwlock_t;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if impl_0.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        impl_0 = safe_c2rust_g_rw_lock_impl_new();
        if ({
            let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if 0 as ::core::ffi::c_int != 0 {
                (*lock).p;
            } else {
            };
            let fresh2 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*lock).p,
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
                impl_0 as gpointer,
            );
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh2.0;
            if fresh2.1 as ::core::ffi::c_int != 0 {
                TRUE
            } else {
                FALSE
            }
        }) == 0
        {
            safe_c2rust_g_rw_lock_impl_free(impl_0);
        }
        impl_0 = (*lock).p as *mut pthread_rwlock_t;
    }
    return impl_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_init(mut rw_lock: *mut GRWLock) {
    (*rw_lock).p = safe_c2rust_g_rw_lock_impl_new() as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_clear(mut rw_lock: *mut GRWLock) {
    safe_c2rust_g_rw_lock_impl_free((*rw_lock).p as *mut pthread_rwlock_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_writer_lock(mut rw_lock: *mut GRWLock) {
    let mut retval: ::core::ffi::c_int =
        pthread_rwlock_wrlock(safe_c2rust_g_rw_lock_get_impl(rw_lock));
    if retval != 0 as ::core::ffi::c_int {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Failed to get RW lock %p: %s\0" as *const u8 as *const gchar,
            rw_lock,
            g_strerror(retval as gint),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_writer_trylock(
    mut rw_lock: *mut GRWLock,
) -> gboolean {
    if pthread_rwlock_trywrlock(safe_c2rust_g_rw_lock_get_impl(rw_lock)) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_writer_unlock(mut rw_lock: *mut GRWLock) {
    pthread_rwlock_unlock(safe_c2rust_g_rw_lock_get_impl(rw_lock));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_reader_lock(mut rw_lock: *mut GRWLock) {
    let mut retval: ::core::ffi::c_int =
        pthread_rwlock_rdlock(safe_c2rust_g_rw_lock_get_impl(rw_lock));
    if retval != 0 as ::core::ffi::c_int {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Failed to get RW lock %p: %s\0" as *const u8 as *const gchar,
            rw_lock,
            g_strerror(retval as gint),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_reader_trylock(
    mut rw_lock: *mut GRWLock,
) -> gboolean {
    if pthread_rwlock_tryrdlock(safe_c2rust_g_rw_lock_get_impl(rw_lock)) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rw_lock_reader_unlock(mut rw_lock: *mut GRWLock) {
    pthread_rwlock_unlock(safe_c2rust_g_rw_lock_get_impl(rw_lock));
}
unsafe extern "C" fn safe_c2rust_g_private_impl_new(
    mut notify: GDestroyNotify,
) -> *mut pthread_key_t {
    let mut key: *mut pthread_key_t = ::core::ptr::null_mut::<pthread_key_t>();
    let mut status: gint = 0;
    key = malloc(::core::mem::size_of::<pthread_key_t>() as size_t) as *mut pthread_key_t;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if key.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            *__errno_location(),
            b"malloc\0" as *const u8 as *const gchar,
        );
    }
    status = pthread_key_create(
        key,
        notify as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) as gint;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(status, b"pthread_key_create\0" as *const u8 as *const gchar);
    }
    return key;
}
unsafe extern "C" fn safe_c2rust_g_private_impl_free(mut key: *mut pthread_key_t) {
    let mut status: gint = 0;
    status = pthread_key_delete(*key) as gint;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(status, b"pthread_key_delete\0" as *const u8 as *const gchar);
    }
    free(key as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn safe_c2rust_g_private_impl_new_direct(mut notify: GDestroyNotify) -> gpointer {
    let mut impl_0: gpointer = -(1 as ::core::ffi::c_int) as gssize as gpointer;
    let mut key: pthread_key_t = 0;
    let mut status: gint = 0;
    status = pthread_key_create(
        &raw mut key,
        notify as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) as gint;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(status, b"pthread_key_create\0" as *const u8 as *const gchar);
    }
    memcpy(
        &raw mut impl_0 as *mut ::core::ffi::c_void,
        &raw mut key as *const ::core::ffi::c_void,
        ::core::mem::size_of::<pthread_key_t>() as size_t,
    );
    if ::core::mem::size_of::<pthread_key_t>() as usize
        == ::core::mem::size_of::<gpointer>() as usize
    {
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if impl_0.is_null() {
                _g_boolean_var_17 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_17 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_17
        }) as ::core::ffi::c_long
            != 0
        {
            status = pthread_key_create(
                &raw mut key,
                notify as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) as gint;
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if status != 0 as ::core::ffi::c_int {
                    _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_18
            }) as ::core::ffi::c_long
                != 0
            {
                safe_c2rust_g_thread_abort(
                    status,
                    b"pthread_key_create\0" as *const u8 as *const gchar,
                );
            }
            memcpy(
                &raw mut impl_0 as *mut ::core::ffi::c_void,
                &raw mut key as *const ::core::ffi::c_void,
                ::core::mem::size_of::<pthread_key_t>() as size_t,
            );
            if ({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if impl_0.is_null() {
                    _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_19
            }) as ::core::ffi::c_long
                != 0
            {
                safe_c2rust_g_thread_abort(
                    status,
                    b"pthread_key_create (gave NULL result twice)\0" as *const u8 as *const gchar,
                );
            }
        }
    }
    return impl_0;
}
unsafe extern "C" fn safe_c2rust_g_private_impl_free_direct(mut impl_0: gpointer) {
    let mut tmp: pthread_key_t = 0;
    let mut status: gint = 0;
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_void,
        &raw mut impl_0 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<pthread_key_t>() as size_t,
    );
    status = pthread_key_delete(tmp) as gint;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(status, b"pthread_key_delete\0" as *const u8 as *const gchar);
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_private_get_impl(mut key: *mut GPrivate) -> pthread_key_t {
    if ::core::mem::size_of::<pthread_key_t>() as usize
        > ::core::mem::size_of::<gpointer>() as usize
    {
        let mut impl_0: *mut pthread_key_t = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = &raw mut (*key).p;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as *mut pthread_key_t;
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if impl_0.is_null() {
                _g_boolean_var_21 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_21 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_21
        }) as ::core::ffi::c_long
            != 0
        {
            impl_0 = safe_c2rust_g_private_impl_new((*key).notify);
            if ({
                let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if 0 as ::core::ffi::c_int != 0 {
                    (*key).p;
                } else {
                };
                let fresh4 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*key).p,
                    *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
                    impl_0 as gpointer,
                );
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh4.0;
                if fresh4.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) == 0
            {
                safe_c2rust_g_private_impl_free(impl_0);
                impl_0 = (*key).p as *mut pthread_key_t;
            }
        }
        return *impl_0;
    } else {
        let mut impl_1: gpointer = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = &raw mut (*key).p;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        });
        let mut tmp: pthread_key_t = 0;
        if ({
            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
            if impl_1.is_null() {
                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_22
        }) as ::core::ffi::c_long
            != 0
        {
            impl_1 = safe_c2rust_g_private_impl_new_direct((*key).notify);
            if ({
                let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if 0 as ::core::ffi::c_int != 0 {
                    (*key).p;
                } else {
                };
                let fresh5 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*key).p,
                    *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
                    impl_1,
                );
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh5.0;
                if fresh5.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) == 0
            {
                safe_c2rust_g_private_impl_free_direct(impl_1);
                impl_1 = (*key).p;
            }
        }
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_void,
            &raw mut impl_1 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<pthread_key_t>() as size_t,
        );
        return tmp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_private_get(mut key: *mut GPrivate) -> gpointer {
    return pthread_getspecific(safe_c2rust_g_private_get_impl(key)) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_private_set(mut key: *mut GPrivate, mut value: gpointer) {
    let mut status: gint = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        status = pthread_setspecific(
            safe_c2rust_g_private_get_impl(key),
            value as *const ::core::ffi::c_void,
        ) as gint;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            status,
            b"pthread_setspecific\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_private_replace(
    mut key: *mut GPrivate,
    mut value: gpointer,
) {
    let mut impl_0: pthread_key_t = safe_c2rust_g_private_get_impl(key);
    let mut old: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut status: gint = 0;
    old = pthread_getspecific(impl_0) as gpointer;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        status = pthread_setspecific(impl_0, value as *const ::core::ffi::c_void) as gint;
        if status != 0 as ::core::ffi::c_int {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_thread_abort(
            status,
            b"pthread_setspecific\0" as *const u8 as *const gchar,
        );
    }
    if !old.is_null() && (*key).notify.is_some() {
        (*key).notify.expect("non-null function pointer")(old);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_system_thread_free(mut thread: *mut GRealThread) {
    let mut pt: *mut GThreadPosix = thread as *mut GThreadPosix;
    if (*pt).joined == 0 {
        pthread_detach((*pt).system_thread);
    }
    safe_c2rust_g_mutex_clear(&raw mut (*pt).lock);
    g_slice_free1(
        ::core::mem::size_of::<GThreadPosix>() as gsize,
        pt as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_system_thread_new(
    mut proxy: GThreadFunc,
    mut stack_size: gulong,
    mut name: *const ::core::ffi::c_char,
    mut func: GThreadFunc,
    mut data: gpointer,
    mut error: *mut *mut GError,
) -> *mut GRealThread {
    let mut thread: *mut GThreadPosix = ::core::ptr::null_mut::<GThreadPosix>();
    let mut base_thread: *mut GRealThread = ::core::ptr::null_mut::<GRealThread>();
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut ret: gint = 0;
    thread = ({
        let mut __s: gsize = ::core::mem::size_of::<GThreadPosix>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GThreadPosix;
    base_thread = thread as *mut GRealThread;
    (*base_thread).ref_count = 2 as ::core::ffi::c_int as gint;
    (*base_thread).ours = TRUE as gboolean;
    (*base_thread).thread.joinable = TRUE as gboolean;
    (*base_thread).thread.func = func;
    (*base_thread).thread.data = data;
    (*base_thread).name = safe_c2rust_g_strdup_inline(name) as *mut gchar;
    (*thread).proxy =
        proxy as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
    let mut error_0: ::core::ffi::c_int = pthread_attr_init(&raw mut attr);
    if error_0 != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"file %s: line %d (%s): error '%s' during '%s'\0" as *const u8 as *const gchar,
            b"../original/glib/gthread-posix.c\0" as *const u8 as *const ::core::ffi::c_char,
            1275 as ::core::ffi::c_int,
            b"g_system_thread_new\0" as *const u8 as *const ::core::ffi::c_char,
            g_strerror(error_0 as gint),
            b"pthread_attr_init (&attr)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        loop {}
    }
    if stack_size != 0 {
        let mut min_stack_size: ::core::ffi::c_long =
            sysconf(_SC_THREAD_STACK_MIN as ::core::ffi::c_int);
        if min_stack_size >= 0 as ::core::ffi::c_long {
            stack_size = if min_stack_size as gulong > stack_size {
                min_stack_size as gulong
            } else {
                stack_size
            };
        }
        pthread_attr_setstacksize(&raw mut attr, stack_size as size_t);
    }
    pthread_attr_setinheritsched(&raw mut attr, PTHREAD_INHERIT_SCHED as ::core::ffi::c_int);
    ret = pthread_create(
        &raw mut (*thread).system_thread,
        &raw mut attr,
        ::core::mem::transmute::<
            GThreadFunc,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        >(proxy),
        thread as *mut ::core::ffi::c_void,
    ) as gint;
    let mut error_1: ::core::ffi::c_int = pthread_attr_destroy(&raw mut attr);
    if error_1 != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"file %s: line %d (%s): error '%s' during '%s'\0" as *const u8 as *const gchar,
            b"../original/glib/gthread-posix.c\0" as *const u8 as *const ::core::ffi::c_char,
            1300 as ::core::ffi::c_int,
            b"g_system_thread_new\0" as *const u8 as *const ::core::ffi::c_char,
            g_strerror(error_1 as gint),
            b"pthread_attr_destroy (&attr)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        loop {}
    }
    if ret == EAGAIN {
        g_set_error(
            error,
            g_thread_error_quark(),
            G_THREAD_ERROR_AGAIN as ::core::ffi::c_int as gint,
            b"Error creating thread: %s\0" as *const u8 as *const gchar,
            g_strerror(ret),
        );
        g_free((*thread).thread.name as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<GThreadPosix>() as gsize,
            thread as gpointer,
        );
        return ::core::ptr::null_mut::<GRealThread>();
    }
    let mut error_2: ::core::ffi::c_int = ret as ::core::ffi::c_int;
    if error_2 != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"file %s: line %d (%s): error '%s' during '%s'\0" as *const u8 as *const gchar,
            b"../original/glib/gthread-posix.c\0" as *const u8 as *const ::core::ffi::c_char,
            1311 as ::core::ffi::c_int,
            b"g_system_thread_new\0" as *const u8 as *const ::core::ffi::c_char,
            g_strerror(error_2 as gint),
            b"pthread_create\0" as *const u8 as *const ::core::ffi::c_char,
        );
        loop {}
    }
    safe_c2rust_g_mutex_init(&raw mut (*thread).lock);
    return thread as *mut GRealThread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_yield() {
    sched_yield();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_system_thread_wait(mut thread: *mut GRealThread) {
    let mut pt: *mut GThreadPosix = thread as *mut GThreadPosix;
    safe_c2rust_g_mutex_lock(&raw mut (*pt).lock);
    if (*pt).joined == 0 {
        let mut error: ::core::ffi::c_int = pthread_join(
            (*pt).system_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        if error != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"file %s: line %d (%s): error '%s' during '%s'\0" as *const u8 as *const gchar,
                b"../original/glib/gthread-posix.c\0" as *const u8 as *const ::core::ffi::c_char,
                1341 as ::core::ffi::c_int,
                b"g_system_thread_wait\0" as *const u8 as *const ::core::ffi::c_char,
                g_strerror(error as gint),
                b"pthread_join (pt->system_thread, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            loop {}
        }
        (*pt).joined = TRUE as gboolean;
    }
    safe_c2rust_g_mutex_unlock(&raw mut (*pt).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_system_thread_exit() -> ! {
    pthread_exit(NULL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_system_thread_set_name(mut name: *const gchar) {
    pthread_setname_np(pthread_self(), name as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_init(mut mutex: *mut GMutex) {
    (*mutex).i[0 as ::core::ffi::c_int as usize] =
        G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_clear(mut mutex: *mut GMutex) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (*mutex).i[0 as ::core::ffi::c_int as usize]
            != G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as guint
        {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
        fprintf(
            safe_c2rust_stderr,
            b"g_mutex_clear() called on uninitialised or locked mutex\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        abort();
    }
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_mutex_lock_slowpath(mut mutex: *mut GMutex) {
    while crate::translated::compat::atomic_xchg_acquire(
        (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint
            as *mut atomic_uint,
        G_MUTEX_STATE_CONTENDED as ::core::ffi::c_int as ::core::ffi::c_uint,
    ) != G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        let mut res: ::core::ffi::c_int = syscall(
            __NR_futex as ::core::ffi::c_long,
            (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize)
                as *mut guint,
            (0 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
            G_MUTEX_STATE_CONTENDED as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as ::core::ffi::c_int;
        if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
            *__errno_location() = saved_errno;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_mutex_unlock_slowpath(mut mutex: *mut GMutex, mut prev: guint) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if prev == G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as guint {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
        fprintf(
            safe_c2rust_stderr,
            b"Attempt to unlock mutex that was not locked\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        abort();
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        (1 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        1 as ::core::ffi::c_int as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_lock(mut mutex: *mut GMutex) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut gaicae_oldval: gint = G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as gint;
            if 0 as ::core::ffi::c_int != 0 {
                (*mutex).i[0 as ::core::ffi::c_int as usize];
            } else {
            };
            let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize)
                    as *mut guint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut guint),
                G_MUTEX_STATE_OWNED as ::core::ffi::c_int as guint,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut guint) = fresh0.0;
            if fresh0.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) == 0
        {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_mutex_lock_slowpath(mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_unlock(mut mutex: *mut GMutex) {
    let mut prev: guint = 0;
    prev = crate::translated::compat::atomic_xchg_release(
        (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint
            as *mut atomic_uint,
        G_MUTEX_STATE_EMPTY as ::core::ffi::c_int as ::core::ffi::c_uint,
    ) as guint;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if prev != G_MUTEX_STATE_OWNED as ::core::ffi::c_int as guint {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_mutex_unlock_slowpath(mutex, prev);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_trylock(mut mutex: *mut GMutex) -> gboolean {
    let mut empty: GMutexState = G_MUTEX_STATE_EMPTY;
    let fresh1 = crate::translated::compat::atomic_cxchg_acquire_relaxed(
        (&raw mut (*mutex).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint
            as *mut atomic_uint,
        *(&raw mut empty as *mut ::core::ffi::c_uint),
        G_MUTEX_STATE_OWNED as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    *(&raw mut empty as *mut ::core::ffi::c_uint) = fresh1.0;
    return fresh1.1 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_init(mut cond: *mut GCond) {
    (*cond).i[0 as ::core::ffi::c_int as usize] = 0 as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_clear(mut cond: *mut GCond) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_wait(mut cond: *mut GCond, mut mutex: *mut GMutex) {
    let mut sampled: guint = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*cond).i[0 as ::core::ffi::c_int as usize];
            (*cond).i[0 as ::core::ffi::c_int as usize];
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize)
                as *mut guint as *mut gint,
        );
        gaig_temp
    }) as guint;
    safe_c2rust_g_mutex_unlock(mutex);
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        (0 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        sampled as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
    safe_c2rust_g_mutex_lock(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_signal(mut cond: *mut GCond) {
    if 0 as ::core::ffi::c_int != 0 {
        (*cond).i[0 as ::core::ffi::c_int as usize];
        (*cond).i[0 as ::core::ffi::c_int as usize];
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        1 as ::core::ffi::c_int as guint,
    );
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        (1 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        1 as ::core::ffi::c_int as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_broadcast(mut cond: *mut GCond) {
    if 0 as ::core::ffi::c_int != 0 {
        (*cond).i[0 as ::core::ffi::c_int as usize];
        (*cond).i[0 as ::core::ffi::c_int as usize];
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        1 as ::core::ffi::c_int as guint,
    );
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        (1 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        2147483647 as ::core::ffi::c_int as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_wait_until(
    mut cond: *mut GCond,
    mut mutex: *mut GMutex,
    mut end_time: gint64,
) -> gboolean {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut span: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut sampled: guint = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut success: gboolean = 0;
    if end_time < 0 as gint64 {
        return FALSE;
    }
    clock_gettime(CLOCK_MONOTONIC, &raw mut now);
    span.tv_sec = end_time as __time_t / 1000000 as __time_t - now.tv_sec;
    span.tv_nsec = end_time as __syscall_slong_t % 1000000 as __syscall_slong_t
        * 1000 as __syscall_slong_t
        - now.tv_nsec;
    if span.tv_nsec < 0 as __syscall_slong_t {
        span.tv_nsec += 1000000000 as __syscall_slong_t;
        span.tv_sec -= 1;
    }
    if span.tv_sec < 0 as __time_t {
        return FALSE;
    }
    sampled = (*cond).i[0 as ::core::ffi::c_int as usize];
    safe_c2rust_g_mutex_unlock(mutex);
    let mut span_arg: C2RustUnnamed_0 = C2RustUnnamed_0 {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (::core::mem::size_of::<__kernel_time_t>() as usize) < 8 as usize
            && span.tv_sec > 0x7fffffff as ::core::ffi::c_int as __time_t
        {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"%s: Can\xE2\x80\x99t wait for more than %us\0" as *const u8 as *const gchar,
            b"g_cond_wait_until\0" as *const u8 as *const ::core::ffi::c_char,
            0x7fffffff as ::core::ffi::c_int,
        );
        loop {}
    }
    span_arg.tv_sec = span.tv_sec as __kernel_time_t;
    span_arg.tv_nsec = span.tv_nsec as ::core::ffi::c_long;
    res = syscall(
        __NR_futex as ::core::ffi::c_long,
        (&raw mut (*cond).i as *mut guint).offset(0 as ::core::ffi::c_int as isize) as *mut guint,
        FUTEX_WAIT_PRIVATE as gsize,
        sampled as gsize,
        &raw mut span_arg,
    ) as ::core::ffi::c_int;
    success = (if res < 0 as ::core::ffi::c_int && *__errno_location() == ETIMEDOUT {
        FALSE
    } else {
        TRUE
    }) as gboolean;
    safe_c2rust_g_mutex_lock(mutex);
    return success;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_cond_wait_until\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
