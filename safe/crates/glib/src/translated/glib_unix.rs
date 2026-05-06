extern "C" {
    pub type _GMainContext;
    pub type _GSourcePrivate;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe2(__pipedes: *mut ::core::ffi::c_int, __flags: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn close_range(
        __fd: ::core::ffi::c_uint,
        __max_fd: ::core::ffi::c_uint,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_add_unix_fd(source: *mut GSource, fd: gint, events: GIOCondition) -> gpointer;
    fn g_source_query_unix_fd(source: *mut GSource, tag: gpointer) -> GIOCondition;
    fn g_strerror(errnum: gint) -> *const gchar;
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
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn _g_main_create_unix_signal_watch(signum: ::core::ffi::c_int) -> *mut GSource;
    fn getpwnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
}
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type size_t = usize;
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
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
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
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUnixFDSource {
    pub source: GSource,
    pub fd: gint,
    pub tag: gpointer,
}
pub type GUnixFDSourceFunc = Option<unsafe extern "C" fn(gint, GIOCondition, gpointer) -> gboolean>;
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
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pwd: passwd,
    pub string_buffer: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_dirent64 {
    pub d_ino: guint64,
    pub d_off: guint64,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub buf: [::core::ffi::c_char; 4096],
    pub alignment: linux_dirent64,
}
pub const CLOSE_RANGE_CLOEXEC: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_unix_open_pipe\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const __NR_getdents64: ::core::ffi::c_int = 217 as ::core::ffi::c_int;
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const SYS_getdents64: ::core::ffi::c_int = __NR_getdents64;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_open_pipe_internal(
    mut fds: *mut ::core::ffi::c_int,
    mut close_on_exec: gboolean,
    mut nonblock: gboolean,
) -> gboolean {
    let mut ecode: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if close_on_exec != 0 {
        flags |= O_CLOEXEC;
    }
    if nonblock != 0 {
        flags |= O_NONBLOCK;
    }
    ecode = pipe2(fds as *mut ::core::ffi::c_int, flags);
    if ecode == -(1 as ::core::ffi::c_int) && *__errno_location() != ENOSYS {
        return FALSE;
    } else if ecode == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    if pipe(fds as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
        return FALSE;
    }
    if close_on_exec != 0 {
        if fcntl(
            *fds.offset(0 as ::core::ffi::c_int as isize),
            F_SETFD,
            FD_CLOEXEC,
        ) == -(1 as ::core::ffi::c_int)
            || fcntl(
                *fds.offset(1 as ::core::ffi::c_int as isize),
                F_SETFD,
                FD_CLOEXEC,
            ) == -(1 as ::core::ffi::c_int)
        {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            close(*fds.offset(0 as ::core::ffi::c_int as isize));
            close(*fds.offset(1 as ::core::ffi::c_int as isize));
            *fds.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *fds.offset(1 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *__errno_location() = saved_errno;
            return FALSE;
        }
    }
    if nonblock != 0 {
        let mut flags_0: ::core::ffi::c_int = O_NONBLOCK;
        if fcntl(
            *fds.offset(0 as ::core::ffi::c_int as isize),
            F_SETFL,
            flags_0,
        ) == -(1 as ::core::ffi::c_int)
            || fcntl(
                *fds.offset(1 as ::core::ffi::c_int as isize),
                F_SETFL,
                flags_0,
            ) == -(1 as ::core::ffi::c_int)
        {
            let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
            close(*fds.offset(0 as ::core::ffi::c_int as isize));
            close(*fds.offset(1 as ::core::ffi::c_int as isize));
            *fds.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *fds.offset(1 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *__errno_location() = saved_errno_0;
            return FALSE;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-unix-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
unsafe extern "C" fn safe_c2rust_g_unix_set_error_from_errno(
    mut error: *mut *mut GError,
    mut saved_errno: gint,
) -> gboolean {
    g_set_error_literal(
        error,
        safe_c2rust_g_unix_error_quark(),
        0 as gint,
        g_strerror(saved_errno),
    );
    *__errno_location() = saved_errno as ::core::ffi::c_int;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_open_pipe(
    mut fds: *mut ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if flags
            & (0o2000000 as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int
                | 0o4000 as ::core::ffi::c_int)
            == flags
        {
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
            b"(flags & (O_CLOEXEC | FD_CLOEXEC | O_NONBLOCK)) == flags\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if flags & FD_CLOEXEC != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"g_unix_open_pipe() called with FD_CLOEXEC; please migrate to using O_CLOEXEC instead\0"
                as *const u8 as *const gchar,
        );
    }
    if safe_c2rust_g_unix_open_pipe_internal(
        fds,
        (flags & (O_CLOEXEC | FD_CLOEXEC) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (flags & O_NONBLOCK != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    ) == 0
    {
        return safe_c2rust_g_unix_set_error_from_errno(error, *__errno_location());
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_set_fd_nonblocking(
    mut fd: gint,
    mut nonblock: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut fcntl_flags: glong = 0;
    fcntl_flags = fcntl(fd as ::core::ffi::c_int, F_GETFL) as glong;
    if fcntl_flags == -(1 as ::core::ffi::c_int) as glong {
        return safe_c2rust_g_unix_set_error_from_errno(error, *__errno_location());
    }
    if nonblock != 0 {
        fcntl_flags |= O_NONBLOCK as glong;
    } else {
        fcntl_flags &= !O_NONBLOCK as glong;
    }
    if fcntl(fd as ::core::ffi::c_int, F_SETFL, fcntl_flags) == -(1 as ::core::ffi::c_int) {
        return safe_c2rust_g_unix_set_error_from_errno(error, *__errno_location());
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_signal_source_new(
    mut signum: ::core::ffi::c_int,
) -> *mut GSource {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if signum == 1 as ::core::ffi::c_int
            || signum == 2 as ::core::ffi::c_int
            || signum == 15 as ::core::ffi::c_int
            || signum == 10 as ::core::ffi::c_int
            || signum == 12 as ::core::ffi::c_int
            || signum == 28 as ::core::ffi::c_int
        {
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
            b"signum == SIGHUP || signum == SIGINT || signum == SIGTERM || signum == SIGUSR1 || signum == SIGUSR2 || signum == SIGWINCH\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    return _g_main_create_unix_signal_watch(signum);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_signal_add_full(
    mut priority: ::core::ffi::c_int,
    mut signum: ::core::ffi::c_int,
    mut handler: GSourceFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut id: guint = 0;
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source = safe_c2rust_g_unix_signal_source_new(signum);
    if priority != G_PRIORITY_DEFAULT {
        g_source_set_priority(source, priority as gint);
    }
    g_source_set_callback(source, handler, user_data, notify);
    id = g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_signal_add(
    mut signum: ::core::ffi::c_int,
    mut handler: GSourceFunc,
    mut user_data: gpointer,
) -> guint {
    return safe_c2rust_g_unix_signal_add_full(
        G_PRIORITY_DEFAULT,
        signum,
        handler,
        user_data,
        None,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut fd_source: *mut GUnixFDSource = source as *mut GUnixFDSource;
    let mut func: GUnixFDSourceFunc =
        ::core::mem::transmute::<GSourceFunc, GUnixFDSourceFunc>(callback);
    if callback.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"GUnixFDSource dispatched without callback. You must call g_source_set_callback().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*fd_source).fd,
        g_source_query_unix_fd(source, (*fd_source).tag),
        user_data,
    );
}
#[no_mangle]
pub static mut safe_c2rust_g_unix_fd_source_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_g_unix_fd_source_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: None,
        closure_marshal: None,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_source_new(
    mut fd: gint,
    mut condition: GIOCondition,
) -> *mut GSource {
    let mut fd_source: *mut GUnixFDSource = ::core::ptr::null_mut::<GUnixFDSource>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source = g_source_new(
        &raw mut safe_c2rust_g_unix_fd_source_funcs,
        ::core::mem::size_of::<GUnixFDSource>() as guint,
    );
    fd_source = source as *mut GUnixFDSource;
    (*fd_source).fd = fd;
    (*fd_source).tag = g_source_add_unix_fd(source, fd, condition);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_add_full(
    mut priority: gint,
    mut fd: gint,
    mut condition: GIOCondition,
    mut function: GUnixFDSourceFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if function.is_some() {
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
            b"function != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    source = safe_c2rust_g_unix_fd_source_new(fd, condition);
    if priority != G_PRIORITY_DEFAULT {
        g_source_set_priority(source, priority);
    }
    g_source_set_callback(
        source,
        ::core::mem::transmute::<GUnixFDSourceFunc, GSourceFunc>(function),
        user_data,
        notify,
    );
    id = g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_add(
    mut fd: gint,
    mut condition: GIOCondition,
    mut function: GUnixFDSourceFunc,
    mut user_data: gpointer,
) -> guint {
    return safe_c2rust_g_unix_fd_add_full(
        G_PRIORITY_DEFAULT,
        fd,
        condition,
        function,
        user_data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_get_passwd_entry(
    mut user_name: *const gchar,
    mut error: *mut *mut GError,
) -> *mut passwd {
    let mut passwd_file_entry: *mut passwd = ::core::ptr::null_mut::<passwd>();
    let mut buffer: *mut C2RustUnnamed_0 = ::core::ptr::null_mut::<C2RustUnnamed_0>();
    let mut string_buffer_size: gsize = 0 as gsize;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !user_name.is_null() {
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
            b"user_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<passwd>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<passwd>();
    }
    let mut string_buffer_size_long: glong =
        sysconf(_SC_GETPW_R_SIZE_MAX as ::core::ffi::c_int) as glong;
    if string_buffer_size_long > 0 as glong {
        string_buffer_size = string_buffer_size_long as gsize;
    }
    if string_buffer_size == 0 as gsize {
        string_buffer_size = 64 as gsize;
    }
    loop {
        let mut retval: ::core::ffi::c_int = 0;
        g_free(buffer as gpointer);
        buffer = g_malloc0(
            (::core::mem::size_of::<C2RustUnnamed_0>() as gsize)
                .wrapping_add(string_buffer_size)
                .wrapping_add(6 as gsize),
        ) as *mut C2RustUnnamed_0;
        retval = getpwnam_r(
            user_name as *const ::core::ffi::c_char,
            &raw mut (*buffer).pwd,
            &raw mut (*buffer).string_buffer as *mut ::core::ffi::c_char,
            string_buffer_size as size_t,
            &raw mut passwd_file_entry,
        );
        if !passwd_file_entry.is_null() {
            break;
        }
        if retval == 0 as ::core::ffi::c_int
            || retval == ENOENT
            || retval == ESRCH
            || retval == EBADF
            || retval == EPERM
        {
            safe_c2rust_g_unix_set_error_from_errno(&raw mut local_error, retval as gint);
            break;
        } else if retval == ERANGE {
            if string_buffer_size > (32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as gsize
            {
                safe_c2rust_g_unix_set_error_from_errno(&raw mut local_error, retval as gint);
                break;
            } else {
                string_buffer_size = string_buffer_size.wrapping_mul(2 as gsize);
                if !passwd_file_entry.is_null() {
                    break;
                }
            }
        } else {
            safe_c2rust_g_unix_set_error_from_errno(&raw mut local_error, retval as gint);
            break;
        }
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if passwd_file_entry.is_null() || passwd_file_entry as gpointer == buffer as gpointer {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/glib-unix.c\0" as *const u8 as *const ::core::ffi::c_char,
            531 as ::core::ffi::c_int,
            G_STRFUNC,
            b"passwd_file_entry == NULL || (gpointer) passwd_file_entry == (gpointer) buffer\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !local_error.is_null() {
        let mut _pp: *mut *mut C2RustUnnamed_0 = &raw mut buffer;
        let mut _ptr: *mut C2RustUnnamed_0 = *_pp;
        *_pp = ::core::ptr::null_mut::<C2RustUnnamed_0>();
        if !_ptr.is_null() {
            g_free(_ptr as gpointer);
        }
        g_propagate_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
    }
    return safe_c2rust_g_steal_pointer(&raw mut buffer as gpointer) as *mut C2RustUnnamed_0
        as *mut passwd;
}
unsafe extern "C" fn safe_c2rust_set_cloexec(
    mut data: *mut ::core::ffi::c_void,
    mut fd: gint,
) -> ::core::ffi::c_int {
    if fd >= data as glong as gint {
        fcntl(fd as ::core::ffi::c_int, F_SETFD, FD_CLOEXEC);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_close_func_with_invalid_fds(
    mut data: *mut ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if fd >= data as glong as gint {
        close(fd);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_filename_to_fd(mut p: *const ::core::ffi::c_char) -> gint {
    let mut c: ::core::ffi::c_char = 0;
    let mut fd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let cutoff: ::core::ffi::c_int = G_MAXINT / 10 as ::core::ffi::c_int;
    let cutlim: ::core::ffi::c_int = G_MAXINT % 10 as ::core::ffi::c_int;
    if *p as ::core::ffi::c_int == '\0' as i32 {
        return -(1 as gint);
    }
    loop {
        let fresh0 = p;
        p = p.offset(1);
        c = *fresh0;
        if !(c as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
        if (c as ::core::ffi::c_int) < '0' as i32 || c as ::core::ffi::c_int > '9' as i32 {
            return -(1 as gint);
        }
        c = (c as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_char;
        if fd > cutoff || fd == cutoff && c as ::core::ffi::c_int > cutlim {
            return -(1 as gint);
        }
        fd = fd * 10 as ::core::ffi::c_int + c as ::core::ffi::c_int;
    }
    return fd as gint;
}
unsafe extern "C" fn safe_c2rust_safe_fdwalk(
    mut cb: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut fd: gint = 0;
    let mut res: gint = 0 as gint;
    let mut dir_fd: ::core::ffi::c_int = open(
        b"/proc/self/fd\0" as *const u8 as *const ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY,
    );
    if dir_fd >= 0 as ::core::ffi::c_int {
        let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { buf: [0; 4096] };
        let mut pos: ::core::ffi::c_int = 0;
        let mut nread: ::core::ffi::c_int = 0;
        let mut de: *mut linux_dirent64 = ::core::ptr::null_mut::<linux_dirent64>();
        loop {
            nread = syscall(
                SYS_getdents64 as ::core::ffi::c_long,
                dir_fd,
                &raw mut u.buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            ) as ::core::ffi::c_int;
            if !(nread > 0 as ::core::ffi::c_int) {
                break;
            }
            pos = 0 as ::core::ffi::c_int;
            while pos < nread {
                de = (&raw mut u.buf as *mut ::core::ffi::c_char).offset(pos as isize)
                    as *mut linux_dirent64;
                fd = safe_c2rust_filename_to_fd(&raw mut (*de).d_name as *mut ::core::ffi::c_char);
                if !(fd < 0 as ::core::ffi::c_int || fd == dir_fd) {
                    res = cb.expect("non-null function pointer")(data, fd as ::core::ffi::c_int)
                        as gint;
                    if res != 0 as ::core::ffi::c_int {
                        break;
                    }
                }
                pos += (*de).d_reclen as ::core::ffi::c_int;
            }
        }
        g_close(dir_fd as gint, ::core::ptr::null_mut::<*mut GError>());
        return res as ::core::ffi::c_int;
    }
    return safe_c2rust_safe_fdwalk_with_invalid_fds(cb, data);
}
unsafe extern "C" fn safe_c2rust_safe_fdwalk_with_invalid_fds(
    mut cb: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut open_max: gint = -(1 as gint);
    let mut fd: gint = 0;
    let mut res: gint = 0 as gint;
    if open_max < 0 as ::core::ffi::c_int {
        open_max = 4096 as ::core::ffi::c_int as gint;
    }
    fd = 0 as ::core::ffi::c_int as gint;
    while fd < open_max {
        res = cb.expect("non-null function pointer")(data, fd as ::core::ffi::c_int) as gint;
        if res != 0 as ::core::ffi::c_int {
            break;
        }
        fd += 1;
    }
    return res as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_fdwalk_set_cloexec(
    mut lowfd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if lowfd >= 0 as ::core::ffi::c_int {
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
            b"lowfd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *__errno_location() = 22 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    ret = close_range(
        lowfd as ::core::ffi::c_uint,
        G_MAXUINT,
        CLOSE_RANGE_CLOEXEC as ::core::ffi::c_int,
    );
    if ret == 0 as ::core::ffi::c_int
        || !(*__errno_location() == ENOSYS || *__errno_location() == EINVAL)
    {
        return ret;
    }
    ret = safe_c2rust_safe_fdwalk(
        Some(
            safe_c2rust_set_cloexec
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, gint) -> ::core::ffi::c_int,
        ),
        lowfd as glong as *mut ::core::ffi::c_void,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_closefrom(
    mut lowfd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if lowfd >= 0 as ::core::ffi::c_int {
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
            b"lowfd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *__errno_location() = 22 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    ret = close_range(
        lowfd as ::core::ffi::c_uint,
        G_MAXUINT,
        0 as ::core::ffi::c_int,
    );
    if ret == 0 as ::core::ffi::c_int || *__errno_location() != ENOSYS {
        return ret;
    }
    ret = safe_c2rust_safe_fdwalk(
        Some(
            safe_c2rust_close_func_with_invalid_fds
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        lowfd as glong as *mut ::core::ffi::c_void,
    );
    return ret;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
