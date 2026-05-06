extern "C" {
    pub type _GHashTable;
    pub type _GWakeup;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigfillset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn waitid(
        __idtype: idtype_t,
        __id: __id_t,
        __infop: *mut siginfo_t,
        __options: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_set_size(array: *mut GPtrArray, length: gint);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_thread_new(name: *const gchar, func: GThreadFunc, data: gpointer) -> *mut GThread;
    fn g_thread_self() -> *mut GThread;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_poll(fds: *mut GPollFD, nfds: guint, timeout: gint) -> gint;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn g_queue_new() -> *mut GQueue;
    fn g_queue_free_full(queue: *mut GQueue, free_func: GDestroyNotify);
    fn g_queue_push_head(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    fn g_queue_peek_head(queue: *mut GQueue) -> gpointer;
    fn g_queue_insert_before_link(queue: *mut GQueue, sibling: *mut GList, link_: *mut GList);
    fn g_queue_push_tail_link(queue: *mut GQueue, link_: *mut GList);
    fn g_queue_unlink(queue: *mut GQueue, link_: *mut GList);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_slice_free_chain_with_offset(block_size: gsize, mem_chain: gpointer, next_offset: gsize);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn g_private_set_alloc0(key: *mut GPrivate, size: gsize) -> gpointer;
    fn g_wakeup_new() -> *mut GWakeup;
    fn g_wakeup_free(wakeup: *mut GWakeup);
    fn g_wakeup_get_pollfd(wakeup: *mut GWakeup, poll_fd: *mut GPollFD);
    fn g_wakeup_signal(wakeup: *mut GWakeup);
    fn g_wakeup_acknowledge(wakeup: *mut GWakeup);
    fn g_uint_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_uint_hash(v: gconstpointer) -> guint;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
pub type __uint32_t = u32;
pub type __uid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const CLD_CONTINUED: C2RustUnnamed_9 = 6;
pub const CLD_STOPPED: C2RustUnnamed_9 = 5;
pub const CLD_TRAPPED: C2RustUnnamed_9 = 4;
pub const CLD_DUMPED: C2RustUnnamed_9 = 3;
pub const CLD_KILLED: C2RustUnnamed_9 = 2;
pub const CLD_EXITED: C2RustUnnamed_9 = 1;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
pub type idtype_t = ::core::ffi::c_uint;
pub const P_PIDFD: idtype_t = 3;
pub const P_PGID: idtype_t = 2;
pub const P_PID: idtype_t = 1;
pub const P_ALL: idtype_t = 0;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const G_HOOK_FLAG_MASK: C2RustUnnamed_11 = 15;
pub const G_HOOK_FLAG_IN_CALL: C2RustUnnamed_11 = 2;
pub const G_HOOK_FLAG_ACTIVE: C2RustUnnamed_11 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
pub type GPollFunc = Option<unsafe extern "C" fn(*mut GPollFD, guint, gint) -> gint>;
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
pub type GMainContextFlags = ::core::ffi::c_uint;
pub const G_MAIN_CONTEXT_FLAGS_OWNERLESS_POLLING: GMainContextFlags = 1;
pub const G_MAIN_CONTEXT_FLAGS_NONE: GMainContextFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMainContext {
    pub mutex: GMutex,
    pub cond: GCond,
    pub owner: *mut GThread,
    pub owner_count: guint,
    pub flags: GMainContextFlags,
    pub waiters: *mut GSList,
    pub ref_count: gint,
    pub sources: *mut GHashTable,
    pub pending_dispatches: *mut GPtrArray,
    pub timeout: gint,
    pub next_id: guint,
    pub source_lists: GQueue,
    pub in_check_or_prepare: gint,
    pub poll_records: *mut GPollRec,
    pub n_poll_records: guint,
    pub cached_poll_array: *mut GPollFD,
    pub cached_poll_array_size: guint,
    pub wakeup: *mut GWakeup,
    pub wake_up_rec: GPollFD,
    pub poll_changed: gboolean,
    pub poll_func: GPollFunc,
    pub time: gint64,
    pub time_is_fresh: gboolean,
}
pub type GWakeup = _GWakeup;
pub type GPollRec = _GPollRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollRec {
    pub fd: *mut GPollFD,
    pub prev: *mut GPollRec,
    pub next: *mut GPollRec,
    pub priority: gint,
}
pub type GQueue = _GQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMainLoop {
    pub context: *mut GMainContext,
    pub is_running: gboolean,
    pub ref_count: gint,
}
pub type GMainLoop = _GMainLoop;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourcePrivate {
    pub child_sources: *mut GSList,
    pub parent_source: *mut GSource,
    pub ready_time: gint64,
    pub fds: *mut GSList,
    pub dispose: GSourceDisposeFunc,
    pub static_name: gboolean,
}
pub type GSourceDisposeFunc = Option<unsafe extern "C" fn(*mut GSource) -> ()>;
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
pub type GSourceOnceFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GChildWatchFunc = Option<unsafe extern "C" fn(GPid, gint, gpointer) -> ()>;
pub type GSourceList = _GSourceList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceList {
    pub link: GList,
    pub head: *mut GSource,
    pub tail: *mut GSource,
    pub priority: gint,
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
pub const G_SOURCE_BLOCKED: C2RustUnnamed_12 = 64;
pub type GSourceIter = _GSourceIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceIter {
    pub context: *mut GMainContext,
    pub may_modify: gboolean,
    pub current_list: *mut GList,
    pub source: *mut GSource,
}
pub type GMainWaiter = _GMainWaiter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMainWaiter {
    pub cond: *mut GCond,
    pub mutex: *mut GMutex,
}
pub type GMainDispatch = _GMainDispatch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMainDispatch {
    pub depth: gint,
    pub source: *mut GSource,
}
pub const G_SOURCE_CAN_RECURSE: C2RustUnnamed_12 = 32;
pub const G_SOURCE_READY: C2RustUnnamed_12 = 16;
pub type GSourceCallback = _GSourceCallback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallback {
    pub ref_count: gint,
    pub func: GSourceFunc,
    pub data: gpointer,
    pub notify: GDestroyNotify,
}
pub type GIdleSource = _GIdleSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIdleSource {
    pub source: GSource,
    pub one_shot: gboolean,
}
pub type GChildWatchSource = _GChildWatchSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GChildWatchSource {
    pub source: GSource,
    pub pid: GPid,
    pub poll: GPollFD,
    pub child_maybe_exited: gboolean,
}
pub type GUnixSignalWatchSource = _GUnixSignalWatchSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixSignalWatchSource {
    pub source: GSource,
    pub signum: ::core::ffi::c_int,
    pub pending: gboolean,
}
pub type GTimeoutSource = _GTimeoutSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeoutSource {
    pub source: GSource,
    pub interval: guint,
    pub seconds: gboolean,
    pub one_shot: gboolean,
}
pub type GClearHandleFunc = Option<unsafe extern "C" fn(guint) -> ()>;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const _NSIG: ::core::ffi::c_int = __SIGRTMAX + 1 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const __SIGRTMAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const CLD_EXITED_0: ::core::ffi::c_int = 1;
pub const CLD_KILLED_0: ::core::ffi::c_int = 2;
pub const CLD_DUMPED_0: ::core::ffi::c_int = 3;
pub const CLD_TRAPPED_0: ::core::ffi::c_int = 4;
pub const CLD_STOPPED_0: ::core::ffi::c_int = 5;
pub const CLD_CONTINUED_0: ::core::ffi::c_int = 6;
pub const NSIG: ::core::ffi::c_int = _NSIG;
pub const SA_NOCLDSTOP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __W_CONTINUED: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __NR_pidfd_open: ::core::ffi::c_int = 434 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SYS_pidfd_open: ::core::ffi::c_int = __NR_pidfd_open;
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_PRIORITY_DEFAULT_IDLE: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
pub const WNOHANG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WEXITED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const P_PIDFD_0: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const G_TRACE_CURRENT_TIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut safe_c2rust_glib_worker_context: *mut GMainContext =
    ::core::ptr::null::<GMainContext>() as *mut GMainContext;
static mut safe_c2rust_unix_signal_pending: [::core::ffi::c_int; 65] = [0; 65];
static mut safe_c2rust_any_unix_signal_pending: ::core::ffi::c_int = 0;
static mut safe_c2rust_g__unix_signal_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_unix_signal_refcount: [guint; 65] = [0; 65];
static mut safe_c2rust_unix_signal_watches: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_unix_child_watches: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
#[no_mangle]
pub static mut safe_c2rust_g_unix_signal_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: Some(
            safe_c2rust_g_unix_signal_watch_prepare
                as unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
        ),
        check: Some(
            safe_c2rust_g_unix_signal_watch_check as unsafe extern "C" fn(*mut GSource) -> gboolean,
        ),
        dispatch: Some(
            safe_c2rust_g_unix_signal_watch_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: Some(
            safe_c2rust_g_unix_signal_watch_finalize as unsafe extern "C" fn(*mut GSource) -> (),
        ),
        closure_callback: None,
        closure_marshal: None,
    }
};
static mut safe_c2rust_g__main_context_list_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_main_context_list: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
#[no_mangle]
pub static mut safe_c2rust_g_timeout_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_g_timeout_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: None,
        closure_marshal: None,
    }
};
#[no_mangle]
pub static mut safe_c2rust_g_child_watch_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: Some(
            safe_c2rust_g_child_watch_prepare
                as unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
        ),
        check: Some(
            safe_c2rust_g_child_watch_check as unsafe extern "C" fn(*mut GSource) -> gboolean,
        ),
        dispatch: Some(
            safe_c2rust_g_child_watch_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: Some(
            safe_c2rust_g_child_watch_finalize as unsafe extern "C" fn(*mut GSource) -> (),
        ),
        closure_callback: None,
        closure_marshal: None,
    }
};
#[no_mangle]
pub static mut safe_c2rust_g_idle_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: Some(
            safe_c2rust_g_idle_prepare as unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
        ),
        check: Some(safe_c2rust_g_idle_check as unsafe extern "C" fn(*mut GSource) -> gboolean),
        dispatch: Some(
            safe_c2rust_g_idle_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: None,
        closure_marshal: None,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_ref(
    mut context: *mut GMainContext,
) -> *mut GMainContext {
    let mut old_ref_count: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    old_ref_count = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*context).ref_count;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut (*context).ref_count,
            1 as ::core::ffi::c_int,
        )
    }) as ::core::ffi::c_int;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if old_ref_count > 0 as ::core::ffi::c_int {
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
            b"old_ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    return context;
}
#[inline]
unsafe extern "C" fn safe_c2rust_poll_rec_list_free(
    mut context: *mut GMainContext,
    mut list: *mut GPollRec,
) {
    g_slice_free_chain_with_offset(
        ::core::mem::size_of::<GPollRec>() as gsize,
        list as gpointer,
        16 as ::core::ffi::c_ulong as glong as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_unref(mut context: *mut GMainContext) {
    let mut iter: GSourceIter = _GSourceIter {
        context: ::core::ptr::null_mut::<GMainContext>(),
        may_modify: 0,
        current_list: ::core::ptr::null_mut::<GList>(),
        source: ::core::ptr::null_mut::<GSource>(),
    };
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut sl_iter: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut s_iter: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut remaining_sources: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut list: *mut GSourceList = ::core::ptr::null_mut::<GSourceList>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*context).ref_count;
            (*context).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*context).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__main_context_list_lock);
    safe_c2rust_main_context_list =
        g_slist_remove(safe_c2rust_main_context_list, context as gconstpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__main_context_list_lock);
    i = 0 as guint;
    while i < (*(*context).pending_dispatches).len {
        safe_c2rust_g_source_unref_internal(
            *(*(*context).pending_dispatches).pdata.offset(i as isize) as *mut GSource,
            context,
            FALSE,
        );
        i = i.wrapping_add(1);
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_source_iter_init(&raw mut iter, context, FALSE);
    while safe_c2rust_g_source_iter_next(&raw mut iter, &raw mut source) != 0 {
        (*source).context = ::core::ptr::null_mut::<GMainContext>();
        remaining_sources = g_slist_prepend(
            remaining_sources,
            safe_c2rust_g_source_ref(source) as gpointer,
        );
    }
    safe_c2rust_g_source_iter_clear(&raw mut iter);
    s_iter = remaining_sources;
    while !s_iter.is_null() {
        source = (*s_iter).data as *mut GSource;
        safe_c2rust_g_source_destroy_internal(source, context, TRUE);
        s_iter = (*s_iter).next;
    }
    sl_iter = (*context).source_lists.head;
    while !sl_iter.is_null() {
        list = (*sl_iter).data as *mut GSourceList;
        sl_iter = (*sl_iter).next;
        g_slice_free1(
            ::core::mem::size_of::<GSourceList>() as gsize,
            list as gpointer,
        );
    }
    g_hash_table_destroy((*context).sources);
    g_mutex_unlock(&raw mut (*context).mutex);
    g_mutex_clear(&raw mut (*context).mutex);
    g_ptr_array_free((*context).pending_dispatches, TRUE);
    g_free((*context).cached_poll_array as gpointer);
    safe_c2rust_poll_rec_list_free(context, (*context).poll_records);
    g_wakeup_free((*context).wakeup);
    g_cond_clear(&raw mut (*context).cond);
    g_free(context as gpointer);
    s_iter = remaining_sources;
    while !s_iter.is_null() {
        source = (*s_iter).data as *mut GSource;
        safe_c2rust_g_source_unref_internal(source, ::core::ptr::null_mut::<GMainContext>(), FALSE);
        s_iter = (*s_iter).next;
    }
    g_slist_free(remaining_sources);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_new_with_next_id(
    mut next_id: guint,
) -> *mut GMainContext {
    let mut ret: *mut GMainContext = safe_c2rust_g_main_context_new();
    (*ret).next_id = next_id;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_new() -> *mut GMainContext {
    return safe_c2rust_g_main_context_new_with_flags(G_MAIN_CONTEXT_FLAGS_NONE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_new_with_flags(
    mut flags: GMainContextFlags,
) -> *mut GMainContext {
    static mut safe_c2rust_initialised: gsize = 0;
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialised;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
    context = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GMainContext>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GMainContext;
    g_mutex_init(&raw mut (*context).mutex);
    g_cond_init(&raw mut (*context).cond);
    (*context).sources = g_hash_table_new(
        Some(g_uint_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_uint_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*context).owner = ::core::ptr::null_mut::<GThread>();
    (*context).flags = flags;
    (*context).waiters = ::core::ptr::null_mut::<GSList>();
    (*context).ref_count = 1 as ::core::ffi::c_int as gint;
    (*context).next_id = 1 as guint;
    (*context).poll_func =
        Some(g_poll as unsafe extern "C" fn(*mut GPollFD, guint, gint) -> gint) as GPollFunc;
    (*context).cached_poll_array = ::core::ptr::null_mut::<GPollFD>();
    (*context).cached_poll_array_size = 0 as guint;
    (*context).pending_dispatches = g_ptr_array_new();
    (*context).time_is_fresh = FALSE as gboolean;
    (*context).wakeup = g_wakeup_new();
    g_wakeup_get_pollfd((*context).wakeup, &raw mut (*context).wake_up_rec);
    safe_c2rust_g_main_context_add_poll_unlocked(
        context,
        0 as gint,
        &raw mut (*context).wake_up_rec,
    );
    g_mutex_lock(&raw mut safe_c2rust_g__main_context_list_lock);
    safe_c2rust_main_context_list =
        g_slist_append(safe_c2rust_main_context_list, context as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__main_context_list_lock);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_default() -> *mut GMainContext {
    static mut safe_c2rust_default_main_context: *mut GMainContext =
        ::core::ptr::null::<GMainContext>() as *mut GMainContext;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_default_main_context;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
            let mut gapg_temp_atomic: *mut *mut GMainContext =
                &raw mut safe_c2rust_default_main_context;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_default_main_context as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
        context = safe_c2rust_g_main_context_new();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_default_main_context = context;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_default_main_context as *mut ::core::ffi::c_void,
            context as guintptr as gpointer,
        );
    }
    return safe_c2rust_default_main_context;
}
unsafe extern "C" fn safe_c2rust_free_context(mut data: gpointer) {
    let mut context: *mut GMainContext = data as *mut GMainContext;
    safe_c2rust_g_main_context_release(context);
    if !context.is_null() {
        safe_c2rust_g_main_context_unref(context);
    }
}
unsafe extern "C" fn safe_c2rust_free_context_stack(mut data: gpointer) {
    g_queue_free_full(
        data as *mut GQueue,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_free_context as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
}
static mut safe_c2rust_thread_context_stack: GPrivate = unsafe {
    _GPrivate {
        p: NULL_0,
        notify: Some(safe_c2rust_free_context_stack as unsafe extern "C" fn(gpointer) -> ()),
        future: [NULL_0, NULL_0],
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_push_thread_default(
    mut context: *mut GMainContext,
) {
    let mut stack: *mut GQueue = ::core::ptr::null_mut::<GQueue>();
    let mut acquired_context: gboolean = 0;
    acquired_context = safe_c2rust_g_main_context_acquire(context);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if acquired_context != 0 {
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
            b"acquired_context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if context == safe_c2rust_g_main_context_default() {
        context = ::core::ptr::null_mut::<GMainContext>();
    } else if !context.is_null() {
        safe_c2rust_g_main_context_ref(context);
    }
    stack = g_private_get(&raw mut safe_c2rust_thread_context_stack) as *mut GQueue;
    if stack.is_null() {
        stack = g_queue_new();
        g_private_set(&raw mut safe_c2rust_thread_context_stack, stack as gpointer);
    }
    g_queue_push_head(stack, context as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_pop_thread_default(
    mut context: *mut GMainContext,
) {
    let mut stack: *mut GQueue = ::core::ptr::null_mut::<GQueue>();
    if context == safe_c2rust_g_main_context_default() {
        context = ::core::ptr::null_mut::<GMainContext>();
    }
    stack = g_private_get(&raw mut safe_c2rust_thread_context_stack) as *mut GQueue;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !stack.is_null() {
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
            b"stack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_queue_peek_head(stack) == context as gpointer {
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
            b"g_queue_peek_head (stack) == context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_queue_pop_head(stack);
    safe_c2rust_g_main_context_release(context);
    if !context.is_null() {
        safe_c2rust_g_main_context_unref(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_get_thread_default() -> *mut GMainContext {
    let mut stack: *mut GQueue = ::core::ptr::null_mut::<GQueue>();
    stack = g_private_get(&raw mut safe_c2rust_thread_context_stack) as *mut GQueue;
    if !stack.is_null() {
        return g_queue_peek_head(stack) as *mut GMainContext;
    } else {
        return ::core::ptr::null_mut::<GMainContext>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_ref_thread_default() -> *mut GMainContext {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    context = safe_c2rust_g_main_context_get_thread_default();
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    return safe_c2rust_g_main_context_ref(context);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_new(
    mut source_funcs: *mut GSourceFuncs,
    mut struct_size: guint,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !source_funcs.is_null() {
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
            b"source_funcs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if struct_size as usize >= ::core::mem::size_of::<GSource>() as usize {
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
            b"struct_size >= sizeof (GSource)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    source = g_malloc0(struct_size as gsize) as *mut GSource;
    (*source).priv_0 = ({
        let mut __s: gsize = ::core::mem::size_of::<GSourcePrivate>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSourcePrivate;
    (*source).source_funcs = source_funcs;
    (*source).ref_count = 1 as guint;
    (*source).priority = G_PRIORITY_DEFAULT as gint;
    (*source).flags = G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint;
    (*(*source).priv_0).ready_time = -(1 as ::core::ffi::c_int) as gint64;
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_dispose_function(
    mut source: *mut GSource,
    mut dispose: GSourceDisposeFunc,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*(*source).priv_0).dispose.is_none() {
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
            b"source->priv->dispose == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*source).priv_0).dispose = dispose;
}
unsafe extern "C" fn safe_c2rust_g_source_iter_init(
    mut iter: *mut GSourceIter,
    mut context: *mut GMainContext,
    mut may_modify: gboolean,
) {
    (*iter).context = context;
    (*iter).current_list = ::core::ptr::null_mut::<GList>();
    (*iter).source = ::core::ptr::null_mut::<GSource>();
    (*iter).may_modify = may_modify;
}
unsafe extern "C" fn safe_c2rust_g_source_iter_next(
    mut iter: *mut GSourceIter,
    mut source: *mut *mut GSource,
) -> gboolean {
    let mut next_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if !(*iter).source.is_null() {
        next_source = (*(*iter).source).next;
    } else {
        next_source = ::core::ptr::null_mut::<GSource>();
    }
    if next_source.is_null() {
        if !(*iter).current_list.is_null() {
            (*iter).current_list = (*(*iter).current_list).next;
        } else {
            (*iter).current_list = (*(*iter).context).source_lists.head;
        }
        if !(*iter).current_list.is_null() {
            let mut source_list: *mut GSourceList =
                (*(*iter).current_list).data as *mut GSourceList;
            next_source = (*source_list).head;
        }
    }
    if !next_source.is_null() && (*iter).may_modify != 0 {
        safe_c2rust_g_source_ref(next_source);
    }
    if !(*iter).source.is_null() && (*iter).may_modify != 0 {
        safe_c2rust_g_source_unref_internal((*iter).source, (*iter).context, TRUE);
    }
    (*iter).source = next_source;
    *source = (*iter).source;
    return (*source != NULL_0 as *mut GSource) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_source_iter_clear(mut iter: *mut GSourceIter) {
    if !(*iter).source.is_null() && (*iter).may_modify != 0 {
        safe_c2rust_g_source_unref_internal((*iter).source, (*iter).context, TRUE);
        (*iter).source = ::core::ptr::null_mut::<GSource>();
    }
}
unsafe extern "C" fn safe_c2rust_find_source_list_for_priority(
    mut context: *mut GMainContext,
    mut priority: gint,
    mut create: gboolean,
) -> *mut GSourceList {
    let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut source_list: *mut GSourceList = ::core::ptr::null_mut::<GSourceList>();
    iter = (*context).source_lists.head;
    while !iter.is_null() {
        source_list = (*iter).data as *mut GSourceList;
        if (*source_list).priority == priority {
            return source_list;
        }
        if (*source_list).priority > priority {
            if create == 0 {
                return ::core::ptr::null_mut::<GSourceList>();
            }
            source_list = ({
                let mut __s: gsize = ::core::mem::size_of::<GSourceList>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                __p = g_slice_alloc(__s);
                memset(
                    __p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    __s as size_t,
                );
                __p
            }) as *mut GSourceList;
            (*source_list).link.data = source_list as gpointer;
            (*source_list).priority = priority;
            g_queue_insert_before_link(
                &raw mut (*context).source_lists,
                iter,
                &raw mut (*source_list).link,
            );
            return source_list;
        }
        iter = (*iter).next;
    }
    if create == 0 {
        return ::core::ptr::null_mut::<GSourceList>();
    }
    source_list = ({
        let mut __s: gsize = ::core::mem::size_of::<GSourceList>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSourceList;
    (*source_list).link.data = source_list as gpointer;
    (*source_list).priority = priority;
    g_queue_push_tail_link(
        &raw mut (*context).source_lists,
        &raw mut (*source_list).link,
    );
    return source_list;
}
unsafe extern "C" fn safe_c2rust_source_add_to_context(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
) {
    let mut source_list: *mut GSourceList = ::core::ptr::null_mut::<GSourceList>();
    let mut prev: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut next: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source_list = safe_c2rust_find_source_list_for_priority(context, (*source).priority, TRUE);
    if !(*(*source).priv_0).parent_source.is_null() {
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if !(*source_list).head.is_null() {
                _g_boolean_var_20 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_20 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_20
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                1100 as ::core::ffi::c_int,
                G_STRFUNC,
                b"source_list->head != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        prev = (*(*(*source).priv_0).parent_source).prev;
        next = (*(*source).priv_0).parent_source;
    } else {
        prev = (*source_list).tail;
        next = ::core::ptr::null_mut::<GSource>();
    }
    (*source).next = next;
    if !next.is_null() {
        (*next).prev = source;
    } else {
        (*source_list).tail = source;
    }
    (*source).prev = prev;
    if !prev.is_null() {
        (*prev).next = source;
    } else {
        (*source_list).head = source;
    };
}
unsafe extern "C" fn safe_c2rust_source_remove_from_context(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
) {
    let mut source_list: *mut GSourceList = ::core::ptr::null_mut::<GSourceList>();
    source_list = safe_c2rust_find_source_list_for_priority(context, (*source).priority, FALSE);
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !source_list.is_null() {
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
            b"source_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*source).prev.is_null() {
        (*(*source).prev).next = (*source).next;
    } else {
        (*source_list).head = (*source).next;
    }
    if !(*source).next.is_null() {
        (*(*source).next).prev = (*source).prev;
    } else {
        (*source_list).tail = (*source).prev;
    }
    (*source).prev = ::core::ptr::null_mut::<GSource>();
    (*source).next = ::core::ptr::null_mut::<GSource>();
    if (*source_list).head.is_null() {
        g_queue_unlink(
            &raw mut (*context).source_lists,
            &raw mut (*source_list).link,
        );
        g_slice_free1(
            ::core::mem::size_of::<GSourceList>() as gsize,
            source_list as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_source_attach_unlocked(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
    mut do_wakeup: gboolean,
) -> guint {
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut id: guint = 0;
    loop {
        let fresh2 = (*context).next_id;
        (*context).next_id = (*context).next_id.wrapping_add(1);
        id = fresh2;
        if !(id == 0 as guint
            || g_hash_table_contains((*context).sources, &raw mut id as gconstpointer) != 0)
        {
            break;
        }
    }
    (*source).context = context;
    (*source).source_id = id;
    safe_c2rust_g_source_ref(source);
    g_hash_table_add((*context).sources, &raw mut (*source).source_id as gpointer);
    safe_c2rust_source_add_to_context(source, context);
    if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
        tmp_list = (*source).poll_fds;
        while !tmp_list.is_null() {
            safe_c2rust_g_main_context_add_poll_unlocked(
                context,
                (*source).priority,
                (*tmp_list).data as *mut GPollFD,
            );
            tmp_list = (*tmp_list).next;
        }
        tmp_list = (*(*source).priv_0).fds;
        while !tmp_list.is_null() {
            safe_c2rust_g_main_context_add_poll_unlocked(
                context,
                (*source).priority,
                (*tmp_list).data as *mut GPollFD,
            );
            tmp_list = (*tmp_list).next;
        }
    }
    tmp_list = (*(*source).priv_0).child_sources;
    while !tmp_list.is_null() {
        safe_c2rust_g_source_attach_unlocked((*tmp_list).data as *mut GSource, context, FALSE);
        tmp_list = (*tmp_list).next;
    }
    if do_wakeup != 0
        && ((*context).flags as ::core::ffi::c_uint
            & G_MAIN_CONTEXT_FLAGS_OWNERLESS_POLLING as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || !(*context).owner.is_null() && (*context).owner != g_thread_self())
    {
        g_wakeup_signal((*context).wakeup);
    }
    return (*source).source_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_attach(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
) -> guint {
    let mut result: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*source).context.is_null() {
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
            b"source->context == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
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
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    result = safe_c2rust_g_source_attach_unlocked(source, context, TRUE);
    g_mutex_unlock(&raw mut (*context).mutex);
    return result;
}
unsafe extern "C" fn safe_c2rust_g_source_destroy_internal(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
    mut have_lock: gboolean,
) {
    if have_lock == 0 {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
        let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
        let mut old_cb_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut old_cb_funcs: *mut GSourceCallbackFuncs =
            ::core::ptr::null_mut::<GSourceCallbackFuncs>();
        (*source).flags &= !(G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int) as guint;
        old_cb_data = (*source).callback_data;
        old_cb_funcs = (*source).callback_funcs;
        (*source).callback_data = NULL_0 as gpointer;
        (*source).callback_funcs = ::core::ptr::null_mut::<GSourceCallbackFuncs>();
        if !old_cb_funcs.is_null() {
            g_mutex_unlock(&raw mut (*context).mutex);
            (*old_cb_funcs).unref.expect("non-null function pointer")(old_cb_data);
            g_mutex_lock(&raw mut (*context).mutex);
        }
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            tmp_list = (*source).poll_fds;
            while !tmp_list.is_null() {
                safe_c2rust_g_main_context_remove_poll_unlocked(
                    context,
                    (*tmp_list).data as *mut GPollFD,
                );
                tmp_list = (*tmp_list).next;
            }
            tmp_list = (*(*source).priv_0).fds;
            while !tmp_list.is_null() {
                safe_c2rust_g_main_context_remove_poll_unlocked(
                    context,
                    (*tmp_list).data as *mut GPollFD,
                );
                tmp_list = (*tmp_list).next;
            }
        }
        while !(*(*source).priv_0).child_sources.is_null() {
            safe_c2rust_g_child_source_remove_internal(
                (*(*(*source).priv_0).child_sources).data as *mut GSource,
                context,
            );
        }
        if !(*(*source).priv_0).parent_source.is_null() {
            safe_c2rust_g_child_source_remove_internal(source, context);
        }
        safe_c2rust_g_source_unref_internal(source, context, TRUE);
    }
    if have_lock == 0 {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_destroy(mut source: *mut GSource) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        safe_c2rust_g_source_destroy_internal(source, context, FALSE);
    } else {
        (*source).flags &= !(G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int) as guint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_id(mut source: *mut GSource) -> guint {
    let mut result: guint = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !(*source).context.is_null() {
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
            b"source->context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut (*(*source).context).mutex);
    result = (*source).source_id;
    g_mutex_unlock(&raw mut (*(*source).context).mutex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_context(
    mut source: *mut GSource,
) -> *mut GMainContext {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !(*source).context.is_null()
            || !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint)
        {
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
            b"source->context != NULL || !SOURCE_DESTROYED (source)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    return (*source).context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_add_poll(
    mut source: *mut GSource,
    mut fd: *mut GPollFD,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !fd.is_null() {
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
            b"fd != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
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
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    (*source).poll_fds = g_slist_prepend((*source).poll_fds, fd as gpointer);
    if !context.is_null() {
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            safe_c2rust_g_main_context_add_poll_unlocked(context, (*source).priority, fd);
        }
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove_poll(
    mut source: *mut GSource,
    mut fd: *mut GPollFD,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !fd.is_null() {
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
            b"fd != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
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
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    (*source).poll_fds = g_slist_remove((*source).poll_fds, fd as gconstpointer);
    if !context.is_null() {
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            safe_c2rust_g_main_context_remove_poll_unlocked(context, fd);
        }
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_add_child_source(
    mut source: *mut GSource,
    mut child_source: *mut GSource,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !child_source.is_null() {
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
            b"child_source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*child_source).ref_count;
                (*child_source).ref_count;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*child_source).ref_count as *mut gint,
            );
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&child_source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
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
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !((*child_source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint
            == 0 as guint)
        {
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
            b"!SOURCE_DESTROYED (child_source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if (*child_source).context.is_null() {
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
            b"child_source->context == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if (*(*child_source).priv_0).parent_source.is_null() {
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
            b"child_source->priv->parent_source == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    (*(*source).priv_0).child_sources = g_slist_prepend(
        (*(*source).priv_0).child_sources,
        safe_c2rust_g_source_ref(child_source) as gpointer,
    );
    (*(*child_source).priv_0).parent_source = source;
    safe_c2rust_g_source_set_priority_unlocked(
        child_source,
        ::core::ptr::null_mut::<GMainContext>(),
        (*source).priority,
    );
    if (*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint {
        safe_c2rust_block_source(child_source);
    }
    if !context.is_null() {
        safe_c2rust_g_source_attach_unlocked(child_source, context, TRUE);
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
unsafe extern "C" fn safe_c2rust_g_child_source_remove_internal(
    mut child_source: *mut GSource,
    mut context: *mut GMainContext,
) {
    let mut parent_source: *mut GSource = (*(*child_source).priv_0).parent_source;
    (*(*parent_source).priv_0).child_sources = g_slist_remove(
        (*(*parent_source).priv_0).child_sources,
        child_source as gconstpointer,
    );
    (*(*child_source).priv_0).parent_source = ::core::ptr::null_mut::<GSource>();
    safe_c2rust_g_source_destroy_internal(child_source, context, TRUE);
    safe_c2rust_g_source_unref_internal(child_source, context, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove_child_source(
    mut source: *mut GSource,
    mut child_source: *mut GSource,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !child_source.is_null() {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"child_source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*child_source).ref_count;
                (*child_source).ref_count;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*child_source).ref_count as *mut gint,
            );
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
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
            b"g_atomic_int_get (&child_source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if (*(*child_source).priv_0).parent_source == source {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"child_source->priv->parent_source == source\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
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
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !((*child_source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint
            == 0 as guint)
        {
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
            b"!SOURCE_DESTROYED (child_source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    safe_c2rust_g_child_source_remove_internal(child_source, context);
    if !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
unsafe extern "C" fn safe_c2rust_g_source_callback_ref(mut cb_data: gpointer) {
    let mut callback: *mut GSourceCallback = cb_data as *mut GSourceCallback;
    if 0 as ::core::ffi::c_int != 0 {
        (*callback).ref_count;
        (*callback).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*callback).ref_count, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn safe_c2rust_g_source_callback_unref(mut cb_data: gpointer) {
    let mut callback: *mut GSourceCallback = cb_data as *mut GSourceCallback;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*callback).ref_count;
            (*callback).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*callback).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if (*callback).notify.is_some() {
            (*callback).notify.expect("non-null function pointer")((*callback).data);
        }
        g_free(callback as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_source_callback_get(
    mut cb_data: gpointer,
    mut source: *mut GSource,
    mut func: *mut GSourceFunc,
    mut data: *mut gpointer,
) {
    let mut callback: *mut GSourceCallback = cb_data as *mut GSourceCallback;
    *func = (*callback).func;
    *data = (*callback).data;
}
static mut safe_c2rust_g_source_callback_funcs: GSourceCallbackFuncs = unsafe {
    _GSourceCallbackFuncs {
        ref_0: Some(safe_c2rust_g_source_callback_ref as unsafe extern "C" fn(gpointer) -> ()),
        unref: Some(safe_c2rust_g_source_callback_unref as unsafe extern "C" fn(gpointer) -> ()),
        get: Some(
            safe_c2rust_g_source_callback_get
                as unsafe extern "C" fn(
                    gpointer,
                    *mut GSource,
                    *mut GSourceFunc,
                    *mut gpointer,
                ) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_callback_indirect(
    mut source: *mut GSource,
    mut callback_data: gpointer,
    mut callback_funcs: *mut GSourceCallbackFuncs,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut old_cb_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut old_cb_funcs: *mut GSourceCallbackFuncs =
        ::core::ptr::null_mut::<GSourceCallbackFuncs>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !callback_funcs.is_null() || callback_data.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"callback_funcs != NULL || callback_data == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    callback_funcs != &raw mut safe_c2rust_g_source_callback_funcs;
    old_cb_data = (*source).callback_data;
    old_cb_funcs = (*source).callback_funcs;
    (*source).callback_data = callback_data;
    (*source).callback_funcs = callback_funcs;
    if !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
    if !old_cb_funcs.is_null() {
        (*old_cb_funcs).unref.expect("non-null function pointer")(old_cb_data);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_callback(
    mut source: *mut GSource,
    mut func: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) {
    let mut new_callback: *mut GSourceCallback = ::core::ptr::null_mut::<GSourceCallback>();
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    new_callback = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GSourceCallback>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GSourceCallback;
    (*new_callback).ref_count = 1 as ::core::ffi::c_int as gint;
    (*new_callback).func = func;
    (*new_callback).data = data;
    (*new_callback).notify = notify;
    safe_c2rust_g_source_set_callback_indirect(
        source,
        new_callback as gpointer,
        &raw mut safe_c2rust_g_source_callback_funcs,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_funcs(
    mut source: *mut GSource,
    mut funcs: *mut GSourceFuncs,
) {
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if (*source).context.is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source->context == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !funcs.is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"funcs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*source).source_funcs = funcs;
}
unsafe extern "C" fn safe_c2rust_g_source_set_priority_unlocked(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
    mut priority: gint,
) {
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if (*(*source).priv_0).parent_source.is_null()
            || (*(*(*source).priv_0).parent_source).priority == priority
        {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source->priv->parent_source == NULL || source->priv->parent_source->priority == priority\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !context.is_null() {
        safe_c2rust_source_remove_from_context(source, (*source).context);
    }
    (*source).priority = priority;
    if !context.is_null() {
        safe_c2rust_source_add_to_context(source, (*source).context);
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            tmp_list = (*source).poll_fds;
            while !tmp_list.is_null() {
                safe_c2rust_g_main_context_remove_poll_unlocked(
                    context,
                    (*tmp_list).data as *mut GPollFD,
                );
                safe_c2rust_g_main_context_add_poll_unlocked(
                    context,
                    priority,
                    (*tmp_list).data as *mut GPollFD,
                );
                tmp_list = (*tmp_list).next;
            }
            tmp_list = (*(*source).priv_0).fds;
            while !tmp_list.is_null() {
                safe_c2rust_g_main_context_remove_poll_unlocked(
                    context,
                    (*tmp_list).data as *mut GPollFD,
                );
                safe_c2rust_g_main_context_add_poll_unlocked(
                    context,
                    priority,
                    (*tmp_list).data as *mut GPollFD,
                );
                tmp_list = (*tmp_list).next;
            }
        }
    }
    if !(*(*source).priv_0).child_sources.is_null() {
        tmp_list = (*(*source).priv_0).child_sources;
        while !tmp_list.is_null() {
            safe_c2rust_g_source_set_priority_unlocked(
                (*tmp_list).data as *mut GSource,
                context,
                priority,
            );
            tmp_list = (*tmp_list).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_priority(
    mut source: *mut GSource,
    mut priority: gint,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if (*(*source).priv_0).parent_source.is_null() {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source->priv->parent_source == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    safe_c2rust_g_source_set_priority_unlocked(source, context, priority);
    if !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_priority(mut source: *mut GSource) -> gint {
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*source).priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_ready_time(
    mut source: *mut GSource,
    mut ready_time: gint64,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if (*(*source).priv_0).ready_time == ready_time {
        if !context.is_null() {
            g_mutex_unlock(&raw mut (*context).mutex);
        }
        return;
    }
    (*(*source).priv_0).ready_time = ready_time;
    if !context.is_null() {
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            g_wakeup_signal((*context).wakeup);
        }
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_ready_time(mut source: *mut GSource) -> gint64 {
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    return (*(*source).priv_0).ready_time;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_can_recurse(
    mut source: *mut GSource,
    mut can_recurse: gboolean,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if can_recurse != 0 {
        (*source).flags |= G_SOURCE_CAN_RECURSE as ::core::ffi::c_int as guint;
    } else {
        (*source).flags &= !(G_SOURCE_CAN_RECURSE as ::core::ffi::c_int) as guint;
    }
    if !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_can_recurse(
    mut source: *mut GSource,
) -> gboolean {
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*source).flags & G_SOURCE_CAN_RECURSE as ::core::ffi::c_int as guint != 0 as guint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_source_set_name_full(
    mut source: *mut GSource,
    mut name: *const ::core::ffi::c_char,
    mut is_static: gboolean,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if (*(*source).priv_0).static_name == 0 {
        g_free((*source).name as gpointer);
    }
    if is_static != 0 {
        (*source).name = name as *mut ::core::ffi::c_char;
    } else {
        (*source).name = safe_c2rust_g_strdup_inline(name);
    }
    (*(*source).priv_0).static_name = is_static;
    if !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_name(
    mut source: *mut GSource,
    mut name: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_source_set_name_full(source, name, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_static_name(
    mut source: *mut GSource,
    mut name: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_source_set_name_full(source, name, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_name(
    mut source: *mut GSource,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*source).name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_set_name_by_id(
    mut tag: guint,
    mut name: *const ::core::ffi::c_char,
) {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if tag > 0 as guint {
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"tag > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    source =
        safe_c2rust_g_main_context_find_source_by_id(::core::ptr::null_mut::<GMainContext>(), tag);
    if source.is_null() {
        return;
    }
    safe_c2rust_g_source_set_name(source, name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_ref(mut source: *mut GSource) -> *mut GSource {
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) >= 0 as ::core::ffi::c_int
        {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) >= 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*source).ref_count;
        (*source).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut (*source).ref_count,
        1 as ::core::ffi::c_int as guint,
    );
    return source;
}
unsafe extern "C" fn safe_c2rust_g_source_unref_internal(
    mut source: *mut GSource,
    mut context: *mut GMainContext,
    mut have_lock: gboolean,
) {
    let mut old_cb_data: gpointer = NULL_0;
    let mut old_cb_funcs: *mut GSourceCallbackFuncs =
        ::core::ptr::null_mut::<GSourceCallbackFuncs>();
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if have_lock == 0 && !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*source).ref_count;
            (*source).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*source).ref_count,
            1 as ::core::ffi::c_int as guint,
        ) == 1 as guint) as ::core::ffi::c_int
    }) != 0
    {
        if (*(*source).priv_0).dispose.is_some() {
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut (*source).ref_count,
                1 as ::core::ffi::c_int as guint,
            );
            if !context.is_null() {
                g_mutex_unlock(&raw mut (*context).mutex);
            }
            (*(*source).priv_0)
                .dispose
                .expect("non-null function pointer")(source);
            if !context.is_null() {
                g_mutex_lock(&raw mut (*context).mutex);
            }
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    (*source).ref_count;
                    (*source).ref_count;
                } else {
                };
                (crate::translated::compat::atomic_xsub_seqcst(
                    &raw mut (*source).ref_count,
                    1 as ::core::ffi::c_int as guint,
                ) == 1 as guint) as ::core::ffi::c_int
            }) == 0
            {
                if have_lock == 0 && !context.is_null() {
                    g_mutex_unlock(&raw mut (*context).mutex);
                }
                return;
            }
        }
        old_cb_data = (*source).callback_data;
        old_cb_funcs = (*source).callback_funcs;
        (*source).callback_data = NULL_0 as gpointer;
        (*source).callback_funcs = ::core::ptr::null_mut::<GSourceCallbackFuncs>();
        if !context.is_null() {
            if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint)
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"../original/glib/gmain.c:2239: ref_count == 0, but source was still attached to a context!\0"
                        as *const u8 as *const gchar,
                );
            }
            safe_c2rust_source_remove_from_context(source, context);
            g_hash_table_remove(
                (*context).sources,
                &raw mut (*source).source_id as gconstpointer,
            );
        }
        if (*(*source).source_funcs).finalize.is_some() {
            let mut old_ref_count: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut (*source).ref_count,
                1 as ::core::ffi::c_int as guint,
            );
            if !context.is_null() {
                g_mutex_unlock(&raw mut (*context).mutex);
            }
            (*(*source).source_funcs)
                .finalize
                .expect("non-null function pointer")(source);
            if !context.is_null() {
                g_mutex_lock(&raw mut (*context).mutex);
            }
            old_ref_count = ({
                if 0 as ::core::ffi::c_int != 0 {
                    (*source).ref_count;
                    -(1 as ::core::ffi::c_int);
                } else {
                };
                crate::translated::compat::atomic_xadd_seqcst(
                    &raw mut (*source).ref_count,
                    -(1 as ::core::ffi::c_int) as guint,
                ) as gint
            });
            if !(({
                let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
                if old_ref_count == 1 as ::core::ffi::c_int {
                    _g_boolean_var_88 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_88 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_88
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2258 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"old_ref_count == 1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        if !old_cb_funcs.is_null() {
            let mut old_ref_count_0: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut (*source).ref_count,
                1 as ::core::ffi::c_int as guint,
            );
            if !context.is_null() {
                g_mutex_unlock(&raw mut (*context).mutex);
            }
            (*old_cb_funcs).unref.expect("non-null function pointer")(old_cb_data);
            if !context.is_null() {
                g_mutex_lock(&raw mut (*context).mutex);
            }
            old_ref_count_0 = ({
                if 0 as ::core::ffi::c_int != 0 {
                    (*source).ref_count;
                    -(1 as ::core::ffi::c_int);
                } else {
                };
                crate::translated::compat::atomic_xadd_seqcst(
                    &raw mut (*source).ref_count,
                    -(1 as ::core::ffi::c_int) as guint,
                ) as gint
            });
            if !(({
                let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
                if old_ref_count_0 == 1 as ::core::ffi::c_int {
                    _g_boolean_var_89 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_89 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_89
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2276 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"old_ref_count == 1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        if (*(*source).priv_0).static_name == 0 {
            g_free((*source).name as gpointer);
        }
        (*source).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_slist_free((*source).poll_fds);
        (*source).poll_fds = ::core::ptr::null_mut::<GSList>();
        g_slist_free_full(
            (*(*source).priv_0).fds,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        while !(*(*source).priv_0).child_sources.is_null() {
            let mut child_source: *mut GSource =
                (*(*(*source).priv_0).child_sources).data as *mut GSource;
            (*(*source).priv_0).child_sources = g_slist_remove(
                (*(*source).priv_0).child_sources,
                child_source as gconstpointer,
            );
            (*(*child_source).priv_0).parent_source = ::core::ptr::null_mut::<GSource>();
            safe_c2rust_g_source_unref_internal(child_source, context, TRUE);
        }
        g_slice_free1(
            ::core::mem::size_of::<GSourcePrivate>() as gsize,
            (*source).priv_0 as gpointer,
        );
        (*source).priv_0 = ::core::ptr::null_mut::<GSourcePrivate>();
        g_free(source as gpointer);
    }
    if have_lock == 0 && !context.is_null() {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_unref(mut source: *mut GSource) {
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_source_unref_internal(source, (*source).context, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_find_source_by_id(
    mut context: *mut GMainContext,
    mut source_id: guint,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut ptr: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if source_id > 0 as guint {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    ptr = g_hash_table_lookup((*context).sources, &raw mut source_id as gconstpointer)
        as gconstpointer;
    if !ptr.is_null() {
        source = (ptr as *mut guint8).offset(-(48 as ::core::ffi::c_ulong as glong) as isize)
            as gpointer as *mut GSource;
        if (*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint {
            source = ::core::ptr::null_mut::<GSource>();
        }
    }
    g_mutex_unlock(&raw mut (*context).mutex);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_find_source_by_funcs_user_data(
    mut context: *mut GMainContext,
    mut funcs: *mut GSourceFuncs,
    mut user_data: gpointer,
) -> *mut GSource {
    let mut iter: GSourceIter = _GSourceIter {
        context: ::core::ptr::null_mut::<GMainContext>(),
        may_modify: 0,
        current_list: ::core::ptr::null_mut::<GList>(),
        source: ::core::ptr::null_mut::<GSource>(),
    };
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if !funcs.is_null() {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"funcs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_source_iter_init(&raw mut iter, context, FALSE);
    while safe_c2rust_g_source_iter_next(&raw mut iter, &raw mut source) != 0 {
        if !(!((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint)
            && (*source).source_funcs == funcs as *const GSourceFuncs
            && !(*source).callback_funcs.is_null())
        {
            continue;
        }
        let mut callback: GSourceFunc = None;
        let mut callback_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*(*source).callback_funcs)
            .get
            .expect("non-null function pointer")(
            (*source).callback_data,
            source,
            &raw mut callback,
            &raw mut callback_data,
        );
        if callback_data == user_data {
            break;
        }
    }
    safe_c2rust_g_source_iter_clear(&raw mut iter);
    g_mutex_unlock(&raw mut (*context).mutex);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_find_source_by_user_data(
    mut context: *mut GMainContext,
    mut user_data: gpointer,
) -> *mut GSource {
    let mut iter: GSourceIter = _GSourceIter {
        context: ::core::ptr::null_mut::<GMainContext>(),
        may_modify: 0,
        current_list: ::core::ptr::null_mut::<GList>(),
        source: ::core::ptr::null_mut::<GSource>(),
    };
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_source_iter_init(&raw mut iter, context, FALSE);
    while safe_c2rust_g_source_iter_next(&raw mut iter, &raw mut source) != 0 {
        if !(!((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint)
            && !(*source).callback_funcs.is_null())
        {
            continue;
        }
        let mut callback: GSourceFunc = None;
        let mut callback_data: gpointer = NULL_0;
        (*(*source).callback_funcs)
            .get
            .expect("non-null function pointer")(
            (*source).callback_data,
            source,
            &raw mut callback,
            &raw mut callback_data,
        );
        if callback_data == user_data {
            break;
        }
    }
    safe_c2rust_g_source_iter_clear(&raw mut iter);
    g_mutex_unlock(&raw mut (*context).mutex);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove(mut tag: guint) -> gboolean {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if tag > 0 as guint {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"tag > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    source =
        safe_c2rust_g_main_context_find_source_by_id(::core::ptr::null_mut::<GMainContext>(), tag);
    if !source.is_null() {
        safe_c2rust_g_source_destroy(source);
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Source ID %u was not found when attempting to remove it\0" as *const u8
                as *const gchar,
            tag,
        );
    }
    return (source != NULL_0 as *mut GSource) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove_by_user_data(
    mut user_data: gpointer,
) -> gboolean {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source = safe_c2rust_g_main_context_find_source_by_user_data(
        ::core::ptr::null_mut::<GMainContext>(),
        user_data,
    );
    if !source.is_null() {
        safe_c2rust_g_source_destroy(source);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove_by_funcs_user_data(
    mut funcs: *mut GSourceFuncs,
    mut user_data: gpointer,
) -> gboolean {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !funcs.is_null() {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"funcs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    source = safe_c2rust_g_main_context_find_source_by_funcs_user_data(
        ::core::ptr::null_mut::<GMainContext>(),
        funcs,
        user_data,
    );
    if !source.is_null() {
        safe_c2rust_g_source_destroy(source);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_clear_handle_id(
    mut tag_ptr: *mut guint,
    mut clear_func: GClearHandleFunc,
) {
    let mut _handle_id: guint = 0;
    _handle_id = *tag_ptr;
    if _handle_id > 0 as guint {
        *tag_ptr = 0 as guint;
        clear_func.expect("non-null function pointer")(_handle_id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_add_unix_fd(
    mut source: *mut GSource,
    mut fd: gint,
    mut events: GIOCondition,
) -> gpointer {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut poll_fd: *mut GPollFD = ::core::ptr::null_mut::<GPollFD>();
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    poll_fd = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GPollFD>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GPollFD;
    (*poll_fd).fd = fd;
    (*poll_fd).events = events as gushort;
    (*poll_fd).revents = 0 as gushort;
    context = (*source).context;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    (*(*source).priv_0).fds = g_slist_prepend((*(*source).priv_0).fds, poll_fd as gpointer);
    if !context.is_null() {
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            safe_c2rust_g_main_context_add_poll_unlocked(context, (*source).priority, poll_fd);
        }
        g_mutex_unlock(&raw mut (*context).mutex);
    }
    return poll_fd as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_modify_unix_fd(
    mut source: *mut GSource,
    mut tag: gpointer,
    mut new_events: GIOCondition,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut poll_fd: *mut GPollFD = ::core::ptr::null_mut::<GPollFD>();
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if !g_slist_find((*(*source).priv_0).fds, tag as gconstpointer).is_null() {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_slist_find (source->priv->fds, tag)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    poll_fd = tag as *mut GPollFD;
    (*poll_fd).events = new_events as gushort;
    if !context.is_null() {
        safe_c2rust_g_main_context_wakeup(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_remove_unix_fd(
    mut source: *mut GSource,
    mut tag: gpointer,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut poll_fd: *mut GPollFD = ::core::ptr::null_mut::<GPollFD>();
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if !g_slist_find((*(*source).priv_0).fds, tag as gconstpointer).is_null() {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_slist_find (source->priv->fds, tag)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    context = (*source).context;
    poll_fd = tag as *mut GPollFD;
    if !context.is_null() {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    (*(*source).priv_0).fds = g_slist_remove((*(*source).priv_0).fds, poll_fd as gconstpointer);
    if !context.is_null() {
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            safe_c2rust_g_main_context_remove_poll_unlocked(context, poll_fd);
        }
        g_mutex_unlock(&raw mut (*context).mutex);
    }
    g_free(poll_fd as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_query_unix_fd(
    mut source: *mut GSource,
    mut tag: gpointer,
) -> GIOCondition {
    let mut poll_fd: *mut GPollFD = ::core::ptr::null_mut::<GPollFD>();
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GIOCondition;
    }
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as GIOCondition;
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if !g_slist_find((*(*source).priv_0).fds, tag as gconstpointer).is_null() {
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_slist_find (source->priv->fds, tag)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GIOCondition;
    }
    poll_fd = tag as *mut GPollFD;
    return (*poll_fd).revents as GIOCondition;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_current_time(mut result: *mut GTimeVal) {
    let mut tv: gint64 = 0;
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if !result.is_null() {
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"result != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    tv = safe_c2rust_g_get_real_time();
    (*result).tv_sec = (tv / 1000000 as gint64) as glong;
    (*result).tv_usec = (tv % 1000000 as gint64) as glong;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_real_time() -> gint64 {
    let mut r: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut r, NULL_0);
    return r.tv_sec * 1000000 as gint64 + r.tv_usec as gint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_monotonic_time() -> gint64 {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut result: gint = 0;
    result = clock_gettime(CLOCK_MONOTONIC, &raw mut ts) as gint;
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if result != 0 as ::core::ffi::c_int {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"GLib requires working CLOCK_MONOTONIC\0" as *const u8 as *const gchar,
        );
        loop {}
    }
    return ts.tv_sec * 1000000 as gint64 + ts.tv_nsec as gint64 / 1000 as gint64;
}
unsafe extern "C" fn safe_c2rust_g_main_dispatch_free(mut dispatch: gpointer) {
    g_free(dispatch);
}
unsafe extern "C" fn safe_c2rust_get_dispatch() -> *mut GMainDispatch {
    static mut safe_c2rust_depth_private: GPrivate = unsafe {
        _GPrivate {
            p: NULL_0,
            notify: Some(safe_c2rust_g_main_dispatch_free as unsafe extern "C" fn(gpointer) -> ()),
            future: [NULL_0, NULL_0],
        }
    };
    let mut dispatch: *mut GMainDispatch = ::core::ptr::null_mut::<GMainDispatch>();
    dispatch = g_private_get(&raw mut safe_c2rust_depth_private) as *mut GMainDispatch;
    if dispatch.is_null() {
        dispatch = g_private_set_alloc0(
            &raw mut safe_c2rust_depth_private,
            ::core::mem::size_of::<GMainDispatch>() as gsize,
        ) as *mut GMainDispatch;
    }
    return dispatch;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_depth() -> gint {
    let mut dispatch: *mut GMainDispatch = safe_c2rust_get_dispatch();
    return (*dispatch).depth;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_current_source() -> *mut GSource {
    let mut dispatch: *mut GMainDispatch = safe_c2rust_get_dispatch();
    return (*dispatch).source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_is_destroyed(mut source: *mut GSource) -> gboolean {
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return ((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_block_source(mut source: *mut GSource) {
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if !((*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint) {
            _g_boolean_var_112 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_112 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_112
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!SOURCE_BLOCKED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*source).flags |= G_SOURCE_BLOCKED as ::core::ffi::c_int as guint;
    if !(*source).context.is_null() {
        tmp_list = (*source).poll_fds;
        while !tmp_list.is_null() {
            safe_c2rust_g_main_context_remove_poll_unlocked(
                (*source).context,
                (*tmp_list).data as *mut GPollFD,
            );
            tmp_list = (*tmp_list).next;
        }
        tmp_list = (*(*source).priv_0).fds;
        while !tmp_list.is_null() {
            safe_c2rust_g_main_context_remove_poll_unlocked(
                (*source).context,
                (*tmp_list).data as *mut GPollFD,
            );
            tmp_list = (*tmp_list).next;
        }
    }
    if !(*source).priv_0.is_null() && !(*(*source).priv_0).child_sources.is_null() {
        tmp_list = (*(*source).priv_0).child_sources;
        while !tmp_list.is_null() {
            safe_c2rust_block_source((*tmp_list).data as *mut GSource);
            tmp_list = (*tmp_list).next;
        }
    }
}
unsafe extern "C" fn safe_c2rust_unblock_source(mut source: *mut GSource) {
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if (*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint {
            _g_boolean_var_113 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_113 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_113
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"SOURCE_BLOCKED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
            _g_boolean_var_114 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_114 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_114
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!SOURCE_DESTROYED (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*source).flags &= !(G_SOURCE_BLOCKED as ::core::ffi::c_int) as guint;
    tmp_list = (*source).poll_fds;
    while !tmp_list.is_null() {
        safe_c2rust_g_main_context_add_poll_unlocked(
            (*source).context,
            (*source).priority,
            (*tmp_list).data as *mut GPollFD,
        );
        tmp_list = (*tmp_list).next;
    }
    tmp_list = (*(*source).priv_0).fds;
    while !tmp_list.is_null() {
        safe_c2rust_g_main_context_add_poll_unlocked(
            (*source).context,
            (*source).priority,
            (*tmp_list).data as *mut GPollFD,
        );
        tmp_list = (*tmp_list).next;
    }
    if !(*source).priv_0.is_null() && !(*(*source).priv_0).child_sources.is_null() {
        tmp_list = (*(*source).priv_0).child_sources;
        while !tmp_list.is_null() {
            safe_c2rust_unblock_source((*tmp_list).data as *mut GSource);
            tmp_list = (*tmp_list).next;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_main_dispatch(mut context: *mut GMainContext) {
    let mut current: *mut GMainDispatch = safe_c2rust_get_dispatch();
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*(*context).pending_dispatches).len {
        let mut source: *mut GSource =
            *(*(*context).pending_dispatches).pdata.offset(i as isize) as *mut GSource;
        let ref mut fresh0 = *(*(*context).pending_dispatches).pdata.offset(i as isize);
        *fresh0 = NULL_0 as gpointer;
        if ({
            let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
            if !source.is_null() {
                _g_boolean_var_115 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_115 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_115
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                3296 as ::core::ffi::c_int,
                G_STRFUNC,
                b"source\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*source).flags &= !(G_SOURCE_READY as ::core::ffi::c_int) as guint;
        if !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint) {
            let mut was_in_call: gboolean = 0;
            let mut user_data: gpointer = NULL_0;
            let mut callback: GSourceFunc = None;
            let mut cb_funcs: *mut GSourceCallbackFuncs =
                ::core::ptr::null_mut::<GSourceCallbackFuncs>();
            let mut cb_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut need_destroy: gboolean = 0;
            let mut dispatch: Option<
                unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
            > = None;
            let mut prev_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
            let mut begin_time_nsec: gint64 = 0;
            dispatch = (*(*source).source_funcs).dispatch;
            cb_funcs = (*source).callback_funcs;
            cb_data = (*source).callback_data;
            if !cb_funcs.is_null() {
                (*cb_funcs).ref_0.expect("non-null function pointer")(cb_data);
            }
            if (*source).flags & G_SOURCE_CAN_RECURSE as ::core::ffi::c_int as guint == 0 as guint {
                safe_c2rust_block_source(source);
            }
            was_in_call =
                ((*source).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint) as gboolean;
            (*source).flags |= G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint;
            if !cb_funcs.is_null() {
                (*cb_funcs).get.expect("non-null function pointer")(
                    cb_data,
                    source,
                    &raw mut callback,
                    &raw mut user_data,
                );
            }
            g_mutex_unlock(&raw mut (*context).mutex);
            prev_source = (*current).source;
            (*current).source = source;
            (*current).depth += 1;
            begin_time_nsec = G_TRACE_CURRENT_TIME as gint64;
            need_destroy = (Some(dispatch.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                source, callback, user_data
            ) == 0) as ::core::ffi::c_int as gboolean;
            (*current).source = prev_source;
            (*current).depth -= 1;
            if !cb_funcs.is_null() {
                (*cb_funcs).unref.expect("non-null function pointer")(cb_data);
            }
            g_mutex_lock(&raw mut (*context).mutex);
            if was_in_call == 0 {
                (*source).flags &= !(G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int) as guint;
            }
            if (*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint
                && !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint
                    == 0 as guint)
            {
                safe_c2rust_unblock_source(source);
            }
            if need_destroy != 0
                && !((*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint
                    == 0 as guint)
            {
                if ({
                    let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
                    if (*source).context == context {
                        _g_boolean_var_116 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_116 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_116
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                        3373 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"source->context == context\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_g_source_destroy_internal(source, context, TRUE);
            }
        }
        safe_c2rust_g_source_unref_internal(source, context, TRUE);
        i = i.wrapping_add(1);
    }
    g_ptr_array_set_size((*context).pending_dispatches, 0 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_acquire(
    mut context: *mut GMainContext,
) -> gboolean {
    let mut result: gboolean = FALSE;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    result = safe_c2rust_g_main_context_acquire_unlocked(context);
    g_mutex_unlock(&raw mut (*context).mutex);
    return result;
}
unsafe extern "C" fn safe_c2rust_g_main_context_acquire_unlocked(
    mut context: *mut GMainContext,
) -> gboolean {
    let mut self_0: *mut GThread = g_thread_self();
    if (*context).owner.is_null() {
        (*context).owner = self_0;
        if ({
            let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
            if (*context).owner_count == 0 as guint {
                _g_boolean_var_117 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_117 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_117
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                3431 as ::core::ffi::c_int,
                G_STRFUNC,
                b"context->owner_count == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if (*context).owner == self_0 {
        (*context).owner_count = (*context).owner_count.wrapping_add(1);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_release(mut context: *mut GMainContext) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_main_context_release_unlocked(context);
    g_mutex_unlock(&raw mut (*context).mutex);
}
unsafe extern "C" fn safe_c2rust_g_main_context_release_unlocked(mut context: *mut GMainContext) {
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if (*context).owner_count > 0 as guint {
            _g_boolean_var_118 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_118 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_118
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"context->owner_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*context).owner_count = (*context).owner_count.wrapping_sub(1);
    if (*context).owner_count == 0 as guint {
        (*context).owner = ::core::ptr::null_mut::<GThread>();
        if !(*context).waiters.is_null() {
            let mut waiter: *mut GMainWaiter = (*(*context).waiters).data as *mut GMainWaiter;
            let mut loop_internal_waiter: gboolean =
                ((*waiter).mutex == &raw mut (*context).mutex) as ::core::ffi::c_int;
            (*context).waiters = g_slist_delete_link((*context).waiters, (*context).waiters);
            if loop_internal_waiter == 0 {
                g_mutex_lock((*waiter).mutex);
            }
            g_cond_signal((*waiter).cond);
            if loop_internal_waiter == 0 {
                g_mutex_unlock((*waiter).mutex);
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_main_context_wait_internal(
    mut context: *mut GMainContext,
    mut cond: *mut GCond,
    mut mutex: *mut GMutex,
) -> gboolean {
    let mut result: gboolean = FALSE;
    let mut self_0: *mut GThread = g_thread_self();
    let mut loop_internal_waiter: gboolean = 0;
    loop_internal_waiter = (mutex == &raw mut (*context).mutex) as ::core::ffi::c_int as gboolean;
    if loop_internal_waiter == 0 {
        g_mutex_lock(&raw mut (*context).mutex);
    }
    if !(*context).owner.is_null() && (*context).owner != self_0 {
        let mut waiter: GMainWaiter = _GMainWaiter {
            cond: ::core::ptr::null_mut::<GCond>(),
            mutex: ::core::ptr::null_mut::<GMutex>(),
        };
        waiter.cond = cond;
        waiter.mutex = mutex;
        (*context).waiters = g_slist_append((*context).waiters, &raw mut waiter as gpointer);
        if loop_internal_waiter == 0 {
            g_mutex_unlock(&raw mut (*context).mutex);
        }
        g_cond_wait(cond, mutex);
        if loop_internal_waiter == 0 {
            g_mutex_lock(&raw mut (*context).mutex);
        }
        (*context).waiters = g_slist_remove((*context).waiters, &raw mut waiter as gconstpointer);
    }
    if (*context).owner.is_null() {
        (*context).owner = self_0;
        if ({
            let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
            if (*context).owner_count == 0 as guint {
                _g_boolean_var_119 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_119 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_119
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                3547 as ::core::ffi::c_int,
                G_STRFUNC,
                b"context->owner_count == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if (*context).owner == self_0 {
        (*context).owner_count = (*context).owner_count.wrapping_add(1);
        result = TRUE as gboolean;
    }
    if loop_internal_waiter == 0 {
        g_mutex_unlock(&raw mut (*context).mutex);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_wait(
    mut context: *mut GMainContext,
    mut cond: *mut GCond,
    mut mutex: *mut GMutex,
) -> gboolean {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if cond != &raw mut (*context).cond || mutex != &raw mut (*context).mutex {
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
    }) as ::core::ffi::c_long
        != 0
    {
        static mut safe_c2rust_warned: gboolean = 0;
        if safe_c2rust_warned == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"WARNING!! g_main_context_wait() will be removed in a future release.  If you see this message, please file a bug immediately.\0"
                    as *const u8 as *const gchar,
            );
            safe_c2rust_warned = TRUE as gboolean;
        }
    }
    return safe_c2rust_g_main_context_wait_internal(context, cond, mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_prepare(
    mut context: *mut GMainContext,
    mut priority: *mut gint,
) -> gboolean {
    let mut ready: gboolean = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    ready = safe_c2rust_g_main_context_prepare_unlocked(context, priority);
    g_mutex_unlock(&raw mut (*context).mutex);
    return ready;
}
unsafe extern "C" fn safe_c2rust_g_main_context_prepare_unlocked(
    mut context: *mut GMainContext,
    mut priority: *mut gint,
) -> gboolean {
    let mut i: guint = 0;
    let mut n_ready: gint = 0 as gint;
    let mut current_priority: gint = G_MAXINT;
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut iter: GSourceIter = _GSourceIter {
        context: ::core::ptr::null_mut::<GMainContext>(),
        may_modify: 0,
        current_list: ::core::ptr::null_mut::<GList>(),
        source: ::core::ptr::null_mut::<GSource>(),
    };
    (*context).time_is_fresh = FALSE as gboolean;
    if (*context).in_check_or_prepare != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_main_context_prepare() called recursively from within a source's check() or prepare() member.\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    i = 0 as guint;
    while i < (*(*context).pending_dispatches).len {
        if !(*(*(*context).pending_dispatches).pdata.offset(i as isize)).is_null() {
            safe_c2rust_g_source_unref_internal(
                *(*(*context).pending_dispatches).pdata.offset(i as isize) as *mut GSource,
                context,
                TRUE,
            );
        }
        i = i.wrapping_add(1);
    }
    g_ptr_array_set_size((*context).pending_dispatches, 0 as gint);
    (*context).timeout = -(1 as ::core::ffi::c_int) as gint;
    safe_c2rust_g_source_iter_init(&raw mut iter, context, TRUE);
    while safe_c2rust_g_source_iter_next(&raw mut iter, &raw mut source) != 0 {
        let mut source_timeout: gint = -(1 as gint);
        if (*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint
            || (*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint
        {
            continue;
        }
        if n_ready > 0 as ::core::ffi::c_int && (*source).priority > current_priority {
            break;
        }
        if (*source).flags & G_SOURCE_READY as ::core::ffi::c_int as guint == 0 {
            let mut result: gboolean = 0;
            let mut prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean> =
                None;
            prepare = (*(*source).source_funcs).prepare;
            if prepare.is_some() {
                let mut begin_time_nsec: gint64 = 0;
                (*context).in_check_or_prepare += 1;
                g_mutex_unlock(&raw mut (*context).mutex);
                begin_time_nsec = G_TRACE_CURRENT_TIME as gint64;
                result = Some(prepare.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    source, &raw mut source_timeout
                );
                g_mutex_lock(&raw mut (*context).mutex);
                (*context).in_check_or_prepare -= 1;
            } else {
                result = FALSE as gboolean;
            }
            if result == FALSE
                && (*(*source).priv_0).ready_time != -(1 as ::core::ffi::c_int) as gint64
            {
                if (*context).time_is_fresh == 0 {
                    (*context).time = safe_c2rust_g_get_monotonic_time();
                    (*context).time_is_fresh = TRUE as gboolean;
                }
                if (*(*source).priv_0).ready_time <= (*context).time {
                    source_timeout = 0 as ::core::ffi::c_int as gint;
                    result = TRUE as gboolean;
                } else {
                    let mut timeout: gint64 = 0;
                    timeout = ((*(*source).priv_0).ready_time - (*context).time + 999 as gint64)
                        / 1000 as gint64;
                    if source_timeout < 0 as ::core::ffi::c_int
                        || timeout < source_timeout as gint64
                    {
                        source_timeout = (if timeout < 2147483647 as gint64 {
                            timeout
                        } else {
                            2147483647 as gint64
                        }) as gint;
                    }
                }
            }
            if result != 0 {
                let mut ready_source: *mut GSource = source;
                while !ready_source.is_null() {
                    (*ready_source).flags |= G_SOURCE_READY as ::core::ffi::c_int as guint;
                    ready_source = (*(*ready_source).priv_0).parent_source;
                }
            }
        }
        if (*source).flags & G_SOURCE_READY as ::core::ffi::c_int as guint != 0 {
            n_ready += 1;
            current_priority = (*source).priority;
            (*context).timeout = 0 as ::core::ffi::c_int as gint;
        }
        if source_timeout >= 0 as ::core::ffi::c_int {
            if (*context).timeout < 0 as ::core::ffi::c_int {
                (*context).timeout = source_timeout;
            } else {
                (*context).timeout = if (*context).timeout < source_timeout {
                    (*context).timeout
                } else {
                    source_timeout
                };
            }
        }
    }
    safe_c2rust_g_source_iter_clear(&raw mut iter);
    if !priority.is_null() {
        *priority = current_priority;
    }
    return (n_ready > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_query(
    mut context: *mut GMainContext,
    mut max_priority: gint,
    mut timeout: *mut gint,
    mut fds: *mut GPollFD,
    mut n_fds: gint,
) -> gint {
    let mut n_poll: gint = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    n_poll = safe_c2rust_g_main_context_query_unlocked(context, max_priority, timeout, fds, n_fds);
    g_mutex_unlock(&raw mut (*context).mutex);
    return n_poll;
}
unsafe extern "C" fn safe_c2rust_g_main_context_query_unlocked(
    mut context: *mut GMainContext,
    mut max_priority: gint,
    mut timeout: *mut gint,
    mut fds: *mut GPollFD,
    mut n_fds: gint,
) -> gint {
    let mut n_poll: gint = 0;
    let mut pollrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut lastpollrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut events: gushort = 0;
    n_poll = 0 as ::core::ffi::c_int as gint;
    lastpollrec = ::core::ptr::null_mut::<GPollRec>();
    pollrec = (*context).poll_records;
    while !pollrec.is_null() {
        if !((*pollrec).priority > max_priority) {
            events = ((*(*pollrec).fd).events as ::core::ffi::c_int
                & !(G_IO_ERR as ::core::ffi::c_int
                    | G_IO_HUP as ::core::ffi::c_int
                    | G_IO_NVAL as ::core::ffi::c_int)) as gushort;
            if !lastpollrec.is_null() && (*(*pollrec).fd).fd == (*(*lastpollrec).fd).fd {
                if (n_poll as ::core::ffi::c_int - 1 as ::core::ffi::c_int) < n_fds {
                    let ref mut fresh1 = (*fds
                        .offset((n_poll as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                    .events;
                    *fresh1 =
                        (*fresh1 as ::core::ffi::c_int | events as ::core::ffi::c_int) as gushort;
                }
            } else {
                if n_poll < n_fds {
                    (*fds.offset(n_poll as isize)).fd = (*(*pollrec).fd).fd;
                    (*fds.offset(n_poll as isize)).events = events;
                    (*fds.offset(n_poll as isize)).revents = 0 as gushort;
                }
                n_poll += 1;
            }
            lastpollrec = pollrec;
        }
        pollrec = (*pollrec).next;
    }
    (*context).poll_changed = FALSE as gboolean;
    if !timeout.is_null() {
        *timeout = (*context).timeout;
        if *timeout != 0 as ::core::ffi::c_int {
            (*context).time_is_fresh = FALSE as gboolean;
        }
    }
    return n_poll;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_check(
    mut context: *mut GMainContext,
    mut max_priority: gint,
    mut fds: *mut GPollFD,
    mut n_fds: gint,
) -> gboolean {
    let mut ready: gboolean = 0;
    g_mutex_lock(&raw mut (*context).mutex);
    ready = safe_c2rust_g_main_context_check_unlocked(context, max_priority, fds, n_fds);
    g_mutex_unlock(&raw mut (*context).mutex);
    return ready;
}
unsafe extern "C" fn safe_c2rust_g_main_context_check_unlocked(
    mut context: *mut GMainContext,
    mut max_priority: gint,
    mut fds: *mut GPollFD,
    mut n_fds: gint,
) -> gboolean {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut iter: GSourceIter = _GSourceIter {
        context: ::core::ptr::null_mut::<GMainContext>(),
        may_modify: 0,
        current_list: ::core::ptr::null_mut::<GList>(),
        source: ::core::ptr::null_mut::<GSource>(),
    };
    let mut pollrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut n_ready: gint = 0 as gint;
    let mut i: gint = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if (*context).in_check_or_prepare != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_main_context_check() called recursively from within a source's check() or prepare() member.\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_fds {
        if (*fds.offset(i as isize)).fd == (*context).wake_up_rec.fd {
            if (*fds.offset(i as isize)).revents != 0 {
                g_wakeup_acknowledge((*context).wakeup);
            }
            break;
        } else {
            i += 1;
        }
    }
    if (*context).poll_changed != 0 {
        return FALSE;
    }
    pollrec = (*context).poll_records;
    i = 0 as ::core::ffi::c_int as gint;
    while !pollrec.is_null() && i < n_fds {
        if ({
            let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
            if i <= 0 as ::core::ffi::c_int
                || (*fds.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)).fd
                    < (*fds.offset(i as isize)).fd
            {
                _g_boolean_var_121 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_121 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_121
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                3993 as ::core::ffi::c_int,
                G_STRFUNC,
                b"i <= 0 || fds[i - 1].fd < fds[i].fd\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        while !pollrec.is_null() && (*(*pollrec).fd).fd != (*fds.offset(i as isize)).fd {
            pollrec = (*pollrec).next;
        }
        while !pollrec.is_null() && (*(*pollrec).fd).fd == (*fds.offset(i as isize)).fd {
            if (*pollrec).priority <= max_priority {
                (*(*pollrec).fd).revents = ((*fds.offset(i as isize)).revents as ::core::ffi::c_int
                    & ((*(*pollrec).fd).events as ::core::ffi::c_int
                        | G_IO_ERR as ::core::ffi::c_int
                        | G_IO_HUP as ::core::ffi::c_int
                        | G_IO_NVAL as ::core::ffi::c_int))
                    as gushort;
            }
            pollrec = (*pollrec).next;
        }
        i += 1;
    }
    safe_c2rust_g_source_iter_init(&raw mut iter, context, TRUE);
    while safe_c2rust_g_source_iter_next(&raw mut iter, &raw mut source) != 0 {
        if (*source).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint == 0 as guint
            || (*source).flags & G_SOURCE_BLOCKED as ::core::ffi::c_int as guint != 0 as guint
        {
            continue;
        }
        if n_ready > 0 as ::core::ffi::c_int && (*source).priority > max_priority {
            break;
        }
        if (*source).flags & G_SOURCE_READY as ::core::ffi::c_int as guint == 0 {
            let mut result: gboolean = 0;
            let mut check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean> = None;
            check = (*(*source).source_funcs).check;
            if check.is_some() {
                let mut begin_time_nsec: gint64 = 0;
                (*context).in_check_or_prepare += 1;
                g_mutex_unlock(&raw mut (*context).mutex);
                begin_time_nsec = G_TRACE_CURRENT_TIME as gint64;
                result = Some(check.expect("non-null function pointer"))
                    .expect("non-null function pointer")(source);
                g_mutex_lock(&raw mut (*context).mutex);
                (*context).in_check_or_prepare -= 1;
            } else {
                result = FALSE as gboolean;
            }
            if result == FALSE {
                let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
                tmp_list = (*(*source).priv_0).fds;
                while !tmp_list.is_null() {
                    let mut pollfd: *mut GPollFD = (*tmp_list).data as *mut GPollFD;
                    if (*pollfd).revents != 0 {
                        result = TRUE as gboolean;
                        break;
                    } else {
                        tmp_list = (*tmp_list).next;
                    }
                }
            }
            if result == FALSE
                && (*(*source).priv_0).ready_time != -(1 as ::core::ffi::c_int) as gint64
            {
                if (*context).time_is_fresh == 0 {
                    (*context).time = safe_c2rust_g_get_monotonic_time();
                    (*context).time_is_fresh = TRUE as gboolean;
                }
                if (*(*source).priv_0).ready_time <= (*context).time {
                    result = TRUE as gboolean;
                }
            }
            if result != 0 {
                let mut ready_source: *mut GSource = source;
                while !ready_source.is_null() {
                    (*ready_source).flags |= G_SOURCE_READY as ::core::ffi::c_int as guint;
                    ready_source = (*(*ready_source).priv_0).parent_source;
                }
            }
        }
        if (*source).flags & G_SOURCE_READY as ::core::ffi::c_int as guint != 0 {
            safe_c2rust_g_source_ref(source);
            g_ptr_array_add((*context).pending_dispatches, source as gpointer);
            n_ready += 1;
            max_priority = (*source).priority;
        }
    }
    safe_c2rust_g_source_iter_clear(&raw mut iter);
    return (n_ready > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_dispatch(mut context: *mut GMainContext) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_main_context_dispatch_unlocked(context);
    g_mutex_unlock(&raw mut (*context).mutex);
}
unsafe extern "C" fn safe_c2rust_g_main_context_dispatch_unlocked(mut context: *mut GMainContext) {
    if (*(*context).pending_dispatches).len > 0 as guint {
        safe_c2rust_g_main_dispatch(context);
    }
}
unsafe extern "C" fn safe_c2rust_g_main_context_iterate_unlocked(
    mut context: *mut GMainContext,
    mut block: gboolean,
    mut dispatch: gboolean,
    mut self_0: *mut GThread,
) -> gboolean {
    let mut max_priority: gint = 0 as gint;
    let mut timeout: gint = 0;
    let mut some_ready: gboolean = 0;
    let mut nfds: gint = 0;
    let mut allocated_nfds: gint = 0;
    let mut fds: *mut GPollFD = ::core::ptr::null_mut::<GPollFD>();
    let mut begin_time_nsec: gint64 = 0;
    begin_time_nsec = G_TRACE_CURRENT_TIME as gint64;
    if safe_c2rust_g_main_context_acquire_unlocked(context) == 0 {
        let mut got_ownership: gboolean = 0;
        if block == 0 {
            return FALSE;
        }
        got_ownership = safe_c2rust_g_main_context_wait_internal(
            context,
            &raw mut (*context).cond,
            &raw mut (*context).mutex,
        );
        if got_ownership == 0 {
            return FALSE;
        }
    }
    if (*context).cached_poll_array.is_null() {
        (*context).cached_poll_array_size = (*context).n_poll_records;
        (*context).cached_poll_array = ({
            let mut __n: gsize = (*context).n_poll_records as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GPollFD>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GPollFD;
    }
    allocated_nfds = (*context).cached_poll_array_size as gint;
    fds = (*context).cached_poll_array;
    safe_c2rust_g_main_context_prepare_unlocked(context, &raw mut max_priority);
    loop {
        nfds = safe_c2rust_g_main_context_query_unlocked(
            context,
            max_priority,
            &raw mut timeout,
            fds,
            allocated_nfds,
        );
        if !(nfds > allocated_nfds) {
            break;
        }
        g_free(fds as gpointer);
        allocated_nfds = nfds;
        (*context).cached_poll_array_size = allocated_nfds as guint;
        fds = ({
            let mut __n: gsize = nfds as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GPollFD>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GPollFD;
        (*context).cached_poll_array = fds;
    }
    if block == 0 {
        timeout = 0 as ::core::ffi::c_int as gint;
    }
    safe_c2rust_g_main_context_poll_unlocked(
        context,
        timeout as ::core::ffi::c_int,
        max_priority as ::core::ffi::c_int,
        fds,
        nfds as ::core::ffi::c_int,
    );
    some_ready = safe_c2rust_g_main_context_check_unlocked(context, max_priority, fds, nfds);
    if dispatch != 0 {
        safe_c2rust_g_main_context_dispatch_unlocked(context);
    }
    safe_c2rust_g_main_context_release_unlocked(context);
    return some_ready;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_pending(
    mut context: *mut GMainContext,
) -> gboolean {
    let mut retval: gboolean = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    retval = safe_c2rust_g_main_context_iterate_unlocked(context, FALSE, FALSE, g_thread_self());
    g_mutex_unlock(&raw mut (*context).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_iteration(
    mut context: *mut GMainContext,
    mut may_block: gboolean,
) -> gboolean {
    let mut retval: gboolean = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    retval = safe_c2rust_g_main_context_iterate_unlocked(context, may_block, TRUE, g_thread_self());
    g_mutex_unlock(&raw mut (*context).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_new(
    mut context: *mut GMainContext,
    mut is_running: gboolean,
) -> *mut GMainLoop {
    let mut loop_0: *mut GMainLoop = ::core::ptr::null_mut::<GMainLoop>();
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    safe_c2rust_g_main_context_ref(context);
    loop_0 = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GMainLoop>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GMainLoop;
    (*loop_0).context = context;
    (*loop_0).is_running = (is_running != FALSE) as ::core::ffi::c_int as gboolean;
    (*loop_0).ref_count = 1 as ::core::ffi::c_int as gint;
    return loop_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_ref(mut loop_0: *mut GMainLoop) -> *mut GMainLoop {
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_122 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_122 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_122
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainLoop>();
    }
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_123 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_123 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_123
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainLoop>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*loop_0).ref_count;
        (*loop_0).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*loop_0).ref_count, 1 as ::core::ffi::c_int);
    return loop_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_unref(mut loop_0: *mut GMainLoop) {
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_124 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_124 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_124
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_125
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*loop_0).ref_count;
            (*loop_0).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*loop_0).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    safe_c2rust_g_main_context_unref((*loop_0).context);
    g_free(loop_0 as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_run(mut loop_0: *mut GMainLoop) {
    let mut self_0: *mut GThread = g_thread_self();
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_126
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*loop_0).ref_count;
        (*loop_0).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*loop_0).ref_count, 1 as ::core::ffi::c_int);
    g_mutex_lock(&raw mut (*(*loop_0).context).mutex);
    if safe_c2rust_g_main_context_acquire_unlocked((*loop_0).context) == 0 {
        let mut got_ownership: gboolean = FALSE;
        let mut gais_temp: gint = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        if 0 as ::core::ffi::c_int != 0 {
            (*loop_0).is_running;
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        } else {
        };
        crate::translated::compat::atomic_store_seqcst(
            &raw mut (*loop_0).is_running as *mut gint,
            *&raw mut gais_temp,
        );
        while ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).is_running;
                (*loop_0).is_running;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).is_running as *mut gint);
            gaig_temp
        }) != 0
            && got_ownership == 0
        {
            got_ownership = safe_c2rust_g_main_context_wait_internal(
                (*loop_0).context,
                &raw mut (*(*loop_0).context).cond,
                &raw mut (*(*loop_0).context).mutex,
            );
        }
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).is_running;
                (*loop_0).is_running;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).is_running as *mut gint);
            gaig_temp
        }) == 0
        {
            if got_ownership != 0 {
                safe_c2rust_g_main_context_release_unlocked((*loop_0).context);
            }
            g_mutex_unlock(&raw mut (*(*loop_0).context).mutex);
            safe_c2rust_g_main_loop_unref(loop_0);
            return;
        }
        if ({
            let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
            if got_ownership != 0 {
                _g_boolean_var_128 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_128 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_128
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmain.c\0" as *const u8 as *const ::core::ffi::c_char,
                4404 as ::core::ffi::c_int,
                G_STRFUNC,
                b"got_ownership\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if (*(*loop_0).context).in_check_or_prepare != 0 {
            _g_boolean_var_129 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_129 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_129
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_main_loop_run(): called recursively from within a source's check() or prepare() member, iteration not possible.\0"
                as *const u8 as *const gchar,
        );
        safe_c2rust_g_main_context_release_unlocked((*loop_0).context);
        g_mutex_unlock(&raw mut (*(*loop_0).context).mutex);
        safe_c2rust_g_main_loop_unref(loop_0);
        return;
    }
    let mut gais_temp_0: gint = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*loop_0).is_running;
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*loop_0).is_running as *mut gint,
        *&raw mut gais_temp_0,
    );
    while ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*loop_0).is_running;
            (*loop_0).is_running;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).is_running as *mut gint);
        gaig_temp
    }) != 0
    {
        safe_c2rust_g_main_context_iterate_unlocked((*loop_0).context, TRUE, TRUE, self_0);
    }
    safe_c2rust_g_main_context_release_unlocked((*loop_0).context);
    g_mutex_unlock(&raw mut (*(*loop_0).context).mutex);
    safe_c2rust_g_main_loop_unref(loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_quit(mut loop_0: *mut GMainLoop) {
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_130 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_130 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_130
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_131 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_131 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_131
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*loop_0).context).mutex);
    let mut gais_temp: gint = 0 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*loop_0).is_running;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*loop_0).is_running as *mut gint,
        *&raw mut gais_temp,
    );
    g_wakeup_signal((*(*loop_0).context).wakeup);
    g_cond_broadcast(&raw mut (*(*loop_0).context).cond);
    g_mutex_unlock(&raw mut (*(*loop_0).context).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_is_running(
    mut loop_0: *mut GMainLoop,
) -> gboolean {
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_132 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_132 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_132
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_133 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_133 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_133
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*loop_0).is_running;
            (*loop_0).is_running;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).is_running as *mut gint);
        gaig_temp
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_loop_get_context(
    mut loop_0: *mut GMainLoop,
) -> *mut GMainContext {
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if !loop_0.is_null() {
            _g_boolean_var_134 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_134 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_134
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"loop != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*loop_0).ref_count;
                (*loop_0).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*loop_0).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_135 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_135 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_135
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&loop->ref_count) > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    return (*loop_0).context;
}
unsafe extern "C" fn safe_c2rust_g_main_context_poll_unlocked(
    mut context: *mut GMainContext,
    mut timeout: ::core::ffi::c_int,
    mut priority: ::core::ffi::c_int,
    mut fds: *mut GPollFD,
    mut n_fds: ::core::ffi::c_int,
) {
    let mut poll_func: GPollFunc = None;
    if n_fds != 0 || timeout != 0 as ::core::ffi::c_int {
        let mut ret: ::core::ffi::c_int = 0;
        let mut errsv: ::core::ffi::c_int = 0;
        poll_func = (*context).poll_func;
        g_mutex_unlock(&raw mut (*context).mutex);
        ret = Some(poll_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(fds, n_fds as guint, timeout as gint)
            as ::core::ffi::c_int;
        g_mutex_lock(&raw mut (*context).mutex);
        errsv = *__errno_location();
        if ret < 0 as ::core::ffi::c_int && errsv != EINTR {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"poll(2) failed due to: %s.\0" as *const u8 as *const gchar,
                g_strerror(errsv as gint),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_add_poll(
    mut context: *mut GMainContext,
    mut fd: *mut GPollFD,
    mut priority: gint,
) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_136 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_136 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_136
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if !fd.is_null() {
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fd\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_main_context_add_poll_unlocked(context, priority, fd);
    g_mutex_unlock(&raw mut (*context).mutex);
}
unsafe extern "C" fn safe_c2rust_g_main_context_add_poll_unlocked(
    mut context: *mut GMainContext,
    mut priority: gint,
    mut fd: *mut GPollFD,
) {
    let mut prevrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut nextrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut newrec: *mut GPollRec =
        g_slice_alloc(::core::mem::size_of::<GPollRec>() as gsize) as *mut GPollRec;
    (*fd).revents = 0 as gushort;
    (*newrec).fd = fd;
    (*newrec).priority = priority;
    prevrec = ::core::ptr::null_mut::<GPollRec>();
    nextrec = (*context).poll_records;
    while !nextrec.is_null() {
        if (*(*nextrec).fd).fd > (*fd).fd {
            break;
        }
        prevrec = nextrec;
        nextrec = (*nextrec).next;
    }
    if !prevrec.is_null() {
        (*prevrec).next = newrec;
    } else {
        (*context).poll_records = newrec;
    }
    (*newrec).prev = prevrec;
    (*newrec).next = nextrec;
    if !nextrec.is_null() {
        (*nextrec).prev = newrec;
    }
    (*context).n_poll_records = (*context).n_poll_records.wrapping_add(1);
    (*context).poll_changed = TRUE as gboolean;
    if fd != &raw mut (*context).wake_up_rec {
        g_wakeup_signal((*context).wakeup);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_remove_poll(
    mut context: *mut GMainContext,
    mut fd: *mut GPollFD,
) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
        if !fd.is_null() {
            _g_boolean_var_139 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_139 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_139
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"fd\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*context).mutex);
    safe_c2rust_g_main_context_remove_poll_unlocked(context, fd);
    g_mutex_unlock(&raw mut (*context).mutex);
}
unsafe extern "C" fn safe_c2rust_g_main_context_remove_poll_unlocked(
    mut context: *mut GMainContext,
    mut fd: *mut GPollFD,
) {
    let mut pollrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut prevrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    let mut nextrec: *mut GPollRec = ::core::ptr::null_mut::<GPollRec>();
    prevrec = ::core::ptr::null_mut::<GPollRec>();
    pollrec = (*context).poll_records;
    while !pollrec.is_null() {
        nextrec = (*pollrec).next;
        if (*pollrec).fd == fd {
            if !prevrec.is_null() {
                (*prevrec).next = nextrec;
            } else {
                (*context).poll_records = nextrec;
            }
            if !nextrec.is_null() {
                (*nextrec).prev = prevrec;
            }
            g_slice_free1(
                ::core::mem::size_of::<GPollRec>() as gsize,
                pollrec as gpointer,
            );
            (*context).n_poll_records = (*context).n_poll_records.wrapping_sub(1);
            break;
        } else {
            prevrec = pollrec;
            pollrec = nextrec;
        }
    }
    (*context).poll_changed = TRUE as gboolean;
    g_wakeup_signal((*context).wakeup);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_current_time(
    mut source: *mut GSource,
    mut timeval: *mut GTimeVal,
) {
    safe_c2rust_g_get_current_time(timeval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_source_get_time(mut source: *mut GSource) -> gint64 {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut result: gint64 = 0;
    if ({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if !source.is_null() {
            _g_boolean_var_140 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_140 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_140
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    if ({
        let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*source).ref_count;
                (*source).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*source).ref_count as *mut gint);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_141 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_141 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_141
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&source->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    if ({
        let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
        if !(*source).context.is_null() {
            _g_boolean_var_142 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_142 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_142
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"source->context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    context = (*source).context;
    g_mutex_lock(&raw mut (*context).mutex);
    if (*context).time_is_fresh == 0 {
        (*context).time = safe_c2rust_g_get_monotonic_time();
        (*context).time_is_fresh = TRUE as gboolean;
    }
    result = (*context).time;
    g_mutex_unlock(&raw mut (*context).mutex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_set_poll_func(
    mut context: *mut GMainContext,
    mut func: GPollFunc,
) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_143 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_143 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_143
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*context).mutex);
    if func.is_some() {
        (*context).poll_func = func;
    } else {
        (*context).poll_func =
            Some(g_poll as unsafe extern "C" fn(*mut GPollFD, guint, gint) -> gint) as GPollFunc;
    }
    g_mutex_unlock(&raw mut (*context).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_get_poll_func(
    mut context: *mut GMainContext,
) -> GPollFunc {
    let mut result: GPollFunc = None;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_144 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_144 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_144
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return None;
    }
    g_mutex_lock(&raw mut (*context).mutex);
    result = (*context).poll_func;
    g_mutex_unlock(&raw mut (*context).mutex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_wakeup(mut context: *mut GMainContext) {
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if ({
        let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*context).ref_count;
                (*context).ref_count;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*context).ref_count);
            gaig_temp
        }) > 0 as ::core::ffi::c_int
        {
            _g_boolean_var_145 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_145 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_145
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_atomic_int_get (&context->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_wakeup_signal((*context).wakeup);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_is_owner(
    mut context: *mut GMainContext,
) -> gboolean {
    let mut is_owner: gboolean = 0;
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    g_mutex_lock(&raw mut (*context).mutex);
    is_owner = ((*context).owner == g_thread_self()) as ::core::ffi::c_int as gboolean;
    g_mutex_unlock(&raw mut (*context).mutex);
    return is_owner;
}
unsafe extern "C" fn safe_c2rust_g_timeout_set_expiration(
    mut timeout_source: *mut GTimeoutSource,
    mut current_time: gint64,
) {
    let mut expiration: gint64 = 0;
    if (*timeout_source).seconds != 0 {
        let mut remainder: gint64 = 0;
        static mut safe_c2rust_timer_perturb: gint = -(1 as gint);
        if safe_c2rust_timer_perturb == -(1 as ::core::ffi::c_int) {
            let mut session_bus_address: *const ::core::ffi::c_char =
                g_getenv(b"DBUS_SESSION_BUS_ADDRESS\0" as *const u8 as *const gchar)
                    as *const ::core::ffi::c_char;
            if session_bus_address.is_null() {
                session_bus_address = g_getenv(b"HOSTNAME\0" as *const u8 as *const gchar)
                    as *const ::core::ffi::c_char;
            }
            if !session_bus_address.is_null() {
                safe_c2rust_timer_perturb =
                    ((if (g_str_hash(session_bus_address as gconstpointer) as gint)
                        < 0 as ::core::ffi::c_int
                    {
                        -(g_str_hash(session_bus_address as gconstpointer) as ::core::ffi::c_int)
                    } else {
                        g_str_hash(session_bus_address as gconstpointer) as ::core::ffi::c_int
                    }) % 1000000 as ::core::ffi::c_int) as gint;
            } else {
                safe_c2rust_timer_perturb = 0 as ::core::ffi::c_int as gint;
            }
        }
        expiration = (current_time as guint64).wrapping_add(
            ((*timeout_source).interval as guint64)
                .wrapping_mul(1000 as guint64)
                .wrapping_mul(1000 as guint64),
        ) as gint64;
        expiration -= safe_c2rust_timer_perturb as gint64;
        remainder = expiration % 1000000 as gint64;
        if remainder >= (1000000 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as gint64 {
            expiration += 1000000 as gint64;
        }
        expiration -= remainder;
        expiration += safe_c2rust_timer_perturb as gint64;
    } else {
        expiration = (current_time as guint64)
            .wrapping_add(((*timeout_source).interval as guint64).wrapping_mul(1000 as guint64))
            as gint64;
    }
    safe_c2rust_g_source_set_ready_time(timeout_source as *mut GSource, expiration);
}
unsafe extern "C" fn safe_c2rust_g_timeout_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut timeout_source: *mut GTimeoutSource = source as *mut GTimeoutSource;
    let mut again: gboolean = 0;
    if callback.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Timeout source dispatched without callback. You must call g_source_set_callback().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    if (*timeout_source).one_shot != 0 {
        let mut once_callback: GSourceOnceFunc =
            ::core::mem::transmute::<GSourceFunc, GSourceOnceFunc>(callback);
        once_callback.expect("non-null function pointer")(user_data);
        again = G_SOURCE_REMOVE as gboolean;
    } else {
        again = callback.expect("non-null function pointer")(user_data);
    }
    if again != 0 {
        safe_c2rust_g_timeout_set_expiration(timeout_source, safe_c2rust_g_source_get_time(source));
    }
    return again;
}
unsafe extern "C" fn safe_c2rust_timeout_source_new(
    mut interval: guint,
    mut seconds: gboolean,
    mut one_shot: gboolean,
) -> *mut GSource {
    let mut source: *mut GSource = safe_c2rust_g_source_new(
        &raw mut safe_c2rust_g_timeout_funcs,
        ::core::mem::size_of::<GTimeoutSource>() as guint,
    );
    let mut timeout_source: *mut GTimeoutSource = source as *mut GTimeoutSource;
    (*timeout_source).interval = interval;
    (*timeout_source).seconds = seconds;
    (*timeout_source).one_shot = one_shot;
    safe_c2rust_g_timeout_set_expiration(timeout_source, safe_c2rust_g_get_monotonic_time());
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_source_new(mut interval: guint) -> *mut GSource {
    return safe_c2rust_timeout_source_new(interval, FALSE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_source_new_seconds(
    mut interval: guint,
) -> *mut GSource {
    return safe_c2rust_timeout_source_new(interval, TRUE, FALSE);
}
unsafe extern "C" fn safe_c2rust_timeout_add_full(
    mut priority: gint,
    mut interval: guint,
    mut seconds: gboolean,
    mut one_shot: gboolean,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
        if function.is_some() {
            _g_boolean_var_146 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_146 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_146
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
    source = safe_c2rust_timeout_source_new(interval, seconds, one_shot);
    if priority != G_PRIORITY_DEFAULT {
        safe_c2rust_g_source_set_priority(source, priority);
    }
    safe_c2rust_g_source_set_callback(source, function, data, notify);
    id = safe_c2rust_g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    safe_c2rust_g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add_full(
    mut priority: gint,
    mut interval: guint,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    return safe_c2rust_timeout_add_full(priority, interval, FALSE, FALSE, function, data, notify);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add(
    mut interval: guint32,
    mut function: GSourceFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_g_timeout_add_full(
        G_PRIORITY_DEFAULT,
        interval as guint,
        function,
        data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add_once(
    mut interval: guint32,
    mut function: GSourceOnceFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_timeout_add_full(
        G_PRIORITY_DEFAULT,
        interval as guint,
        FALSE,
        TRUE,
        ::core::mem::transmute::<GSourceOnceFunc, GSourceFunc>(function),
        data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add_seconds_full(
    mut priority: gint,
    mut interval: guint32,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    return safe_c2rust_timeout_add_full(
        priority,
        interval as guint,
        TRUE,
        FALSE,
        function,
        data,
        notify,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add_seconds(
    mut interval: guint,
    mut function: GSourceFunc,
    mut data: gpointer,
) -> guint {
    if ({
        let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
        if function.is_some() {
            _g_boolean_var_147 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_147 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_147
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
    return safe_c2rust_g_timeout_add_seconds_full(
        G_PRIORITY_DEFAULT,
        interval as guint32,
        function,
        data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_timeout_add_seconds_once(
    mut interval: guint,
    mut function: GSourceOnceFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_timeout_add_full(
        G_PRIORITY_DEFAULT,
        interval,
        TRUE,
        TRUE,
        ::core::mem::transmute::<GSourceOnceFunc, GSourceFunc>(function),
        data,
        None,
    );
}
unsafe extern "C" fn safe_c2rust_siginfo_t_to_wait_status(
    mut info: *const siginfo_t,
) -> ::core::ffi::c_int {
    match (*info).si_code {
        CLD_EXITED_0 => {
            return (*info)._sifields._sigchld.si_status << 8 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int;
        }
        CLD_KILLED_0 => {
            return (0 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | (*info)._sifields._sigchld.si_status;
        }
        CLD_DUMPED_0 => {
            return (0 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | ((*info)._sifields._sigchld.si_status | 0x80 as ::core::ffi::c_int);
        }
        CLD_CONTINUED_0 => return __W_CONTINUED,
        CLD_STOPPED_0 | CLD_TRAPPED_0 | _ => {
            return (*info)._sifields._sigchld.si_status << 8 as ::core::ffi::c_int
                | 0x7f as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_child_watch_prepare(
    mut source: *mut GSource,
    mut timeout: *mut gint,
) -> gboolean {
    let mut child_watch_source: *mut GChildWatchSource =
        ::core::ptr::null_mut::<GChildWatchSource>();
    child_watch_source = source as *mut GChildWatchSource;
    if (*child_watch_source).poll.fd >= 0 as ::core::ffi::c_int {
        return FALSE;
    }
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*child_watch_source).child_maybe_exited;
            (*child_watch_source).child_maybe_exited;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut (*child_watch_source).child_maybe_exited as *mut gint,
        );
        gaig_temp
    });
}
unsafe extern "C" fn safe_c2rust_g_child_watch_check(mut source: *mut GSource) -> gboolean {
    let mut child_watch_source: *mut GChildWatchSource =
        ::core::ptr::null_mut::<GChildWatchSource>();
    let mut child_exited: gboolean = 0;
    child_watch_source = source as *mut GChildWatchSource;
    if (*child_watch_source).poll.fd >= 0 as ::core::ffi::c_int {
        child_exited = ((*child_watch_source).poll.revents as ::core::ffi::c_int
            & G_IO_IN as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int as gboolean;
        return child_exited;
    }
    child_exited = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*child_watch_source).child_maybe_exited;
            (*child_watch_source).child_maybe_exited;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut (*child_watch_source).child_maybe_exited as *mut gint,
        );
        gaig_temp
    }) as gboolean;
    return child_exited;
}
unsafe extern "C" fn safe_c2rust_g_child_watch_finalize(mut source: *mut GSource) {
    let mut child_watch_source: *mut GChildWatchSource = source as *mut GChildWatchSource;
    if (*child_watch_source).poll.fd >= 0 as ::core::ffi::c_int {
        close((*child_watch_source).poll.fd as ::core::ffi::c_int);
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    safe_c2rust_unix_child_watches =
        g_slist_remove(safe_c2rust_unix_child_watches, source as gconstpointer);
    safe_c2rust_unref_unix_signal_handler_unlocked(SIGCHLD);
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
}
unsafe extern "C" fn safe_c2rust_wake_source(mut source: *mut GSource) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    g_mutex_lock(&raw mut safe_c2rust_g__main_context_list_lock);
    context = (*source).context;
    if !context.is_null() {
        g_wakeup_signal((*context).wakeup);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__main_context_list_lock);
}
unsafe extern "C" fn safe_c2rust_dispatch_unix_signals_unlocked() {
    let mut pending: [gboolean; 65] = [0; 65];
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut i: gint = 0;
    let mut gais_temp: gint = 0 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_any_unix_signal_pending;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut safe_c2rust_any_unix_signal_pending as *mut gint,
        *&raw mut gais_temp,
    );
    i = 0 as ::core::ffi::c_int as gint;
    while i < NSIG {
        pending[i as usize] = ({
            let mut gaicae_oldval: gint = 1 as gint;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_unix_signal_pending[i as usize];
            } else {
            };
            let fresh3 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                (&raw mut safe_c2rust_unix_signal_pending as *mut ::core::ffi::c_int)
                    .offset(i as isize) as *mut ::core::ffi::c_int,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int),
                0 as ::core::ffi::c_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int) =
                fresh3.0;
            if fresh3.1 as ::core::ffi::c_int != 0 {
                TRUE
            } else {
                FALSE
            }
        }) as gboolean;
        i += 1;
    }
    if pending[SIGCHLD as usize] != 0 {
        node = safe_c2rust_unix_child_watches;
        while !node.is_null() {
            let mut source: *mut GChildWatchSource = (*node).data as *mut GChildWatchSource;
            if ({
                let mut gaicae_oldval: gint = 0 as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*source).child_maybe_exited;
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
                } else {
                };
                let fresh4 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*source).child_maybe_exited,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gboolean),
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gboolean) = fresh4.0;
                if fresh4.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) != 0
            {
                safe_c2rust_wake_source(source as *mut GSource);
            }
            node = (*node).next;
        }
    }
    node = safe_c2rust_unix_signal_watches;
    while !node.is_null() {
        let mut source_0: *mut GUnixSignalWatchSource = (*node).data as *mut GUnixSignalWatchSource;
        if pending[(*source_0).signum as usize] != 0
            && ({
                let mut gaicae_oldval: gint = 0 as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*source_0).pending;
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
                } else {
                };
                let fresh5 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*source_0).pending,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gboolean),
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gboolean) = fresh5.0;
                (if fresh5.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                })
            }) != 0
        {
            safe_c2rust_wake_source(source_0 as *mut GSource);
        }
        node = (*node).next;
    }
}
unsafe extern "C" fn safe_c2rust_dispatch_unix_signals() {
    g_mutex_lock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    safe_c2rust_dispatch_unix_signals_unlocked();
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
}
unsafe extern "C" fn safe_c2rust_g_unix_signal_watch_prepare(
    mut source: *mut GSource,
    mut timeout: *mut gint,
) -> gboolean {
    let mut unix_signal_source: *mut GUnixSignalWatchSource =
        ::core::ptr::null_mut::<GUnixSignalWatchSource>();
    unix_signal_source = source as *mut GUnixSignalWatchSource;
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*unix_signal_source).pending;
            (*unix_signal_source).pending;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut (*unix_signal_source).pending as *mut gint,
        );
        gaig_temp
    });
}
unsafe extern "C" fn safe_c2rust_g_unix_signal_watch_check(mut source: *mut GSource) -> gboolean {
    let mut unix_signal_source: *mut GUnixSignalWatchSource =
        ::core::ptr::null_mut::<GUnixSignalWatchSource>();
    unix_signal_source = source as *mut GUnixSignalWatchSource;
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*unix_signal_source).pending;
            (*unix_signal_source).pending;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut (*unix_signal_source).pending as *mut gint,
        );
        gaig_temp
    });
}
unsafe extern "C" fn safe_c2rust_g_unix_signal_watch_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut unix_signal_source: *mut GUnixSignalWatchSource =
        ::core::ptr::null_mut::<GUnixSignalWatchSource>();
    let mut again: gboolean = 0;
    unix_signal_source = source as *mut GUnixSignalWatchSource;
    if callback.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Unix signal source dispatched without callback. You must call g_source_set_callback().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    let mut gais_temp: gint = 0 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*unix_signal_source).pending;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*unix_signal_source).pending as *mut gint,
        *&raw mut gais_temp,
    );
    again = callback.expect("non-null function pointer")(user_data);
    return again;
}
unsafe extern "C" fn safe_c2rust_ref_unix_signal_handler_unlocked(mut signum: ::core::ffi::c_int) {
    safe_c2rust_g_get_worker_context();
    safe_c2rust_unix_signal_refcount[signum as usize] =
        safe_c2rust_unix_signal_refcount[signum as usize].wrapping_add(1);
    if safe_c2rust_unix_signal_refcount[signum as usize] == 1 as guint {
        let mut action: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        action.__sigaction_handler.sa_handler = Some(
            safe_c2rust_g_unix_signal_handler as unsafe extern "C" fn(::core::ffi::c_int) -> (),
        ) as __sighandler_t;
        sigemptyset(&raw mut action.sa_mask);
        action.sa_flags = SA_RESTART | SA_NOCLDSTOP;
        sigaction(
            signum,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        );
    }
}
unsafe extern "C" fn safe_c2rust_unref_unix_signal_handler_unlocked(
    mut signum: ::core::ffi::c_int,
) {
    safe_c2rust_unix_signal_refcount[signum as usize] =
        safe_c2rust_unix_signal_refcount[signum as usize].wrapping_sub(1);
    if safe_c2rust_unix_signal_refcount[signum as usize] == 0 as guint {
        let mut action: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &raw mut action as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sigaction>() as size_t,
        );
        action.__sigaction_handler.sa_handler = SIG_DFL;
        sigemptyset(&raw mut action.sa_mask);
        sigaction(
            signum,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        );
    }
}
unsafe extern "C" fn safe_c2rust_signum_to_string(mut signum: ::core::ffi::c_int) -> *const gchar {
    match signum {
        6 => return b"GUnixSignalSource: SIGABRT\0" as *const u8 as *const gchar,
        8 => return b"GUnixSignalSource: SIGFPE\0" as *const u8 as *const gchar,
        4 => return b"GUnixSignalSource: SIGILL\0" as *const u8 as *const gchar,
        2 => return b"GUnixSignalSource: SIGINT\0" as *const u8 as *const gchar,
        11 => return b"GUnixSignalSource: SIGSEGV\0" as *const u8 as *const gchar,
        15 => return b"GUnixSignalSource: SIGTERM\0" as *const u8 as *const gchar,
        14 => return b"GUnixSignalSource: SIGALRM\0" as *const u8 as *const gchar,
        17 => return b"GUnixSignalSource: SIGCHLD\0" as *const u8 as *const gchar,
        1 => return b"GUnixSignalSource: SIGHUP\0" as *const u8 as *const gchar,
        9 => return b"GUnixSignalSource: SIGKILL\0" as *const u8 as *const gchar,
        13 => return b"GUnixSignalSource: SIGPIPE\0" as *const u8 as *const gchar,
        3 => return b"GUnixSignalSource: SIGQUIT\0" as *const u8 as *const gchar,
        19 => return b"GUnixSignalSource: SIGSTOP\0" as *const u8 as *const gchar,
        10 => return b"GUnixSignalSource: SIGUSR1\0" as *const u8 as *const gchar,
        12 => return b"GUnixSignalSource: SIGUSR2\0" as *const u8 as *const gchar,
        29 => return b"GUnixSignalSource: SIGPOLL\0" as *const u8 as *const gchar,
        27 => return b"GUnixSignalSource: SIGPROF\0" as *const u8 as *const gchar,
        5 => return b"GUnixSignalSource: SIGTRAP\0" as *const u8 as *const gchar,
        _ => {
            return b"GUnixSignalSource: Unrecognized signal\0" as *const u8 as *const gchar;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_main_create_unix_signal_watch(
    mut signum: ::core::ffi::c_int,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut unix_signal_source: *mut GUnixSignalWatchSource =
        ::core::ptr::null_mut::<GUnixSignalWatchSource>();
    source = safe_c2rust_g_source_new(
        &raw mut safe_c2rust_g_unix_signal_funcs,
        ::core::mem::size_of::<GUnixSignalWatchSource>() as guint,
    );
    unix_signal_source = source as *mut GUnixSignalWatchSource;
    (*unix_signal_source).signum = signum;
    (*unix_signal_source).pending = FALSE as gboolean;
    safe_c2rust_g_source_set_static_name(
        source,
        safe_c2rust_signum_to_string(signum) as *const ::core::ffi::c_char,
    );
    g_mutex_lock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    safe_c2rust_ref_unix_signal_handler_unlocked(signum);
    safe_c2rust_unix_signal_watches = g_slist_prepend(
        safe_c2rust_unix_signal_watches,
        unix_signal_source as gpointer,
    );
    safe_c2rust_dispatch_unix_signals_unlocked();
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    return source;
}
unsafe extern "C" fn safe_c2rust_g_unix_signal_watch_finalize(mut source: *mut GSource) {
    let mut unix_signal_source: *mut GUnixSignalWatchSource =
        ::core::ptr::null_mut::<GUnixSignalWatchSource>();
    unix_signal_source = source as *mut GUnixSignalWatchSource;
    g_mutex_lock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    safe_c2rust_unref_unix_signal_handler_unlocked((*unix_signal_source).signum);
    safe_c2rust_unix_signal_watches =
        g_slist_remove(safe_c2rust_unix_signal_watches, source as gconstpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
}
unsafe extern "C" fn safe_c2rust_g_child_watch_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut child_watch_source: *mut GChildWatchSource =
        ::core::ptr::null_mut::<GChildWatchSource>();
    let mut child_watch_callback: GChildWatchFunc =
        ::core::mem::transmute::<GSourceFunc, GChildWatchFunc>(callback);
    let mut wait_status: ::core::ffi::c_int = 0;
    child_watch_source = source as *mut GChildWatchSource;
    let mut child_exited: gboolean = FALSE;
    wait_status = -(1 as ::core::ffi::c_int);
    if (*child_watch_source).poll.fd >= 0 as ::core::ffi::c_int {
        let mut child_info: siginfo_t = siginfo_t {
            si_signo: 0 as ::core::ffi::c_int,
            si_errno: 0,
            si_code: 0,
            __pad0: 0,
            _sifields: C2RustUnnamed { _pad: [0; 28] },
        };
        if waitid(
            P_PIDFD,
            (*child_watch_source).poll.fd as __id_t,
            &raw mut child_info,
            WEXITED | WNOHANG,
        ) >= 0 as ::core::ffi::c_int
        {
            if child_info._sifields._kill.si_pid != 0 as ::core::ffi::c_int {
                wait_status = safe_c2rust_siginfo_t_to_wait_status(&raw mut child_info);
                child_exited = TRUE as gboolean;
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"../original/glib/gmain.c:5793: pidfd signaled but pid %i didn't exit\0"
                        as *const u8 as *const gchar,
                    (*child_watch_source).pid,
                );
                return TRUE;
            }
        } else {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/gmain.c:5802: waitid(pid:%i, pidfd=%d) failed: %s (%d). %s\0"
                    as *const u8 as *const gchar,
                (*child_watch_source).pid,
                (*child_watch_source).poll.fd,
                g_strerror(errsv as gint),
                errsv,
                b"See documentation of g_child_watch_source_new() for possible causes.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            child_exited = TRUE as gboolean;
        }
    }
    if child_exited == 0 {
        let mut pid: pid_t = 0;
        let mut wstatus: ::core::ffi::c_int = 0;
        loop {
            let mut gais_temp: gint = 0 as ::core::ffi::c_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*child_watch_source).child_maybe_exited;
            } else {
            };
            crate::translated::compat::atomic_store_seqcst(
                &raw mut (*child_watch_source).child_maybe_exited as *mut gint,
                *&raw mut gais_temp,
            );
            pid = waitpid(
                (*child_watch_source).pid as __pid_t,
                &raw mut wstatus,
                WNOHANG,
            ) as pid_t;
            if !(({
                let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
                if pid < 0 as ::core::ffi::c_int && *__errno_location() == 4 as ::core::ffi::c_int {
                    _g_boolean_var_148 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_148 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_148
            }) as ::core::ffi::c_long
                != 0)
            {
                break;
            }
        }
        if pid == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        if pid > 0 as ::core::ffi::c_int {
            wait_status = wstatus;
        } else {
            let mut errsv_0: ::core::ffi::c_int = *__errno_location();
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/gmain.c:5840: waitpid(pid:%i) failed: %s (%d). %s\0" as *const u8
                    as *const gchar,
                (*child_watch_source).pid,
                g_strerror(errsv_0 as gint),
                errsv_0,
                b"See documentation of g_child_watch_source_new() for possible causes.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if callback.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Child watch source dispatched without callback. You must call g_source_set_callback().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    child_watch_callback.expect("non-null function pointer")(
        (*child_watch_source).pid,
        wait_status as gint,
        user_data,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_unix_signal_handler(mut signum: ::core::ffi::c_int) {
    let mut saved_errno: gint = *__errno_location();
    let mut gais_temp: gint = 1 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_unix_signal_pending[signum as usize];
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        (&raw mut safe_c2rust_unix_signal_pending as *mut ::core::ffi::c_int)
            .offset(signum as isize) as *mut ::core::ffi::c_int as *mut gint,
        *&raw mut gais_temp,
    );
    let mut gais_temp_0: gint = 1 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_any_unix_signal_pending;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut safe_c2rust_any_unix_signal_pending as *mut gint,
        *&raw mut gais_temp_0,
    );
    g_wakeup_signal((*safe_c2rust_glib_worker_context).wakeup);
    *__errno_location() = saved_errno as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_child_watch_source_new(mut pid: GPid) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut child_watch_source: *mut GChildWatchSource =
        ::core::ptr::null_mut::<GChildWatchSource>();
    let mut errsv: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
        if pid > 0 as ::core::ffi::c_int {
            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_149
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pid > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    source = safe_c2rust_g_source_new(
        &raw mut safe_c2rust_g_child_watch_funcs,
        ::core::mem::size_of::<GChildWatchSource>() as guint,
    );
    child_watch_source = source as *mut GChildWatchSource;
    safe_c2rust_g_source_set_static_name(
        source,
        b"GChildWatchSource\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*child_watch_source).pid = pid;
    (*child_watch_source).poll.fd = syscall(
        SYS_pidfd_open as ::core::ffi::c_long,
        pid,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int as gint;
    if (*child_watch_source).poll.fd >= 0 as ::core::ffi::c_int {
        (*child_watch_source).poll.events = G_IO_IN as ::core::ffi::c_int as gushort;
        safe_c2rust_g_source_add_poll(source, &raw mut (*child_watch_source).poll);
        return source;
    }
    errsv = *__errno_location();
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"pidfd_open(%i) failed with error: %s\0" as *const u8 as *const gchar,
        pid,
        g_strerror(errsv as gint),
    );
    (*child_watch_source).child_maybe_exited = TRUE as gboolean;
    (*child_watch_source).poll.fd = -(1 as ::core::ffi::c_int) as gint;
    g_mutex_lock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    safe_c2rust_ref_unix_signal_handler_unlocked(SIGCHLD);
    safe_c2rust_unix_child_watches = g_slist_prepend(
        safe_c2rust_unix_child_watches,
        child_watch_source as gpointer,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_signal_lock_lock);
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_child_watch_add_full(
    mut priority: gint,
    mut pid: GPid,
    mut function: GChildWatchFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if function.is_some() {
            _g_boolean_var_150 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_150 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_150
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
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if pid > 0 as ::core::ffi::c_int {
            _g_boolean_var_151 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_151 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_151
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pid > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    source = safe_c2rust_g_child_watch_source_new(pid);
    if priority != G_PRIORITY_DEFAULT {
        safe_c2rust_g_source_set_priority(source, priority);
    }
    safe_c2rust_g_source_set_callback(
        source,
        ::core::mem::transmute::<GChildWatchFunc, GSourceFunc>(function),
        data,
        notify,
    );
    id = safe_c2rust_g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    safe_c2rust_g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_child_watch_add(
    mut pid: GPid,
    mut function: GChildWatchFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_g_child_watch_add_full(G_PRIORITY_DEFAULT, pid, function, data, None);
}
unsafe extern "C" fn safe_c2rust_g_idle_prepare(
    mut source: *mut GSource,
    mut timeout: *mut gint,
) -> gboolean {
    *timeout = 0 as ::core::ffi::c_int as gint;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_idle_check(mut source: *mut GSource) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_idle_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut idle_source: *mut GIdleSource = source as *mut GIdleSource;
    let mut again: gboolean = 0;
    if callback.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Idle source dispatched without callback. You must call g_source_set_callback().\0"
                as *const u8 as *const gchar,
        );
        return FALSE;
    }
    if (*idle_source).one_shot != 0 {
        let mut once_callback: GSourceOnceFunc =
            ::core::mem::transmute::<GSourceFunc, GSourceOnceFunc>(callback);
        once_callback.expect("non-null function pointer")(user_data);
        again = G_SOURCE_REMOVE as gboolean;
    } else {
        again = callback.expect("non-null function pointer")(user_data);
    }
    return again;
}
unsafe extern "C" fn safe_c2rust_idle_source_new(mut one_shot: gboolean) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut idle_source: *mut GIdleSource = ::core::ptr::null_mut::<GIdleSource>();
    source = safe_c2rust_g_source_new(
        &raw mut safe_c2rust_g_idle_funcs,
        ::core::mem::size_of::<GIdleSource>() as guint,
    );
    idle_source = source as *mut GIdleSource;
    (*idle_source).one_shot = one_shot;
    safe_c2rust_g_source_set_priority(source, G_PRIORITY_DEFAULT_IDLE);
    safe_c2rust_g_source_set_static_name(
        source,
        b"GIdleSource\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_idle_source_new() -> *mut GSource {
    return safe_c2rust_idle_source_new(FALSE);
}
unsafe extern "C" fn safe_c2rust_idle_add_full(
    mut priority: gint,
    mut one_shot: gboolean,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if function.is_some() {
            _g_boolean_var_152 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_152 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_152
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
    source = safe_c2rust_idle_source_new(one_shot);
    if priority != G_PRIORITY_DEFAULT_IDLE {
        safe_c2rust_g_source_set_priority(source, priority);
    }
    safe_c2rust_g_source_set_callback(source, function, data, notify);
    id = safe_c2rust_g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    safe_c2rust_g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_idle_add_full(
    mut priority: gint,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    return safe_c2rust_idle_add_full(priority, FALSE, function, data, notify);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_idle_add(
    mut function: GSourceFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_g_idle_add_full(G_PRIORITY_DEFAULT_IDLE, function, data, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_idle_add_once(
    mut function: GSourceOnceFunc,
    mut data: gpointer,
) -> guint {
    return safe_c2rust_idle_add_full(
        G_PRIORITY_DEFAULT_IDLE,
        TRUE,
        ::core::mem::transmute::<GSourceOnceFunc, GSourceFunc>(function),
        data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_idle_remove_by_data(mut data: gpointer) -> gboolean {
    return safe_c2rust_g_source_remove_by_funcs_user_data(&raw mut safe_c2rust_g_idle_funcs, data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_invoke(
    mut context: *mut GMainContext,
    mut function: GSourceFunc,
    mut data: gpointer,
) {
    safe_c2rust_g_main_context_invoke_full(context, G_PRIORITY_DEFAULT, function, data, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_main_context_invoke_full(
    mut context: *mut GMainContext,
    mut priority: gint,
    mut function: GSourceFunc,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if function.is_some() {
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"function != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if context.is_null() {
        context = safe_c2rust_g_main_context_default();
    }
    if safe_c2rust_g_main_context_is_owner(context) != 0 {
        while function.expect("non-null function pointer")(data) != 0 {}
        if notify.is_some() {
            notify.expect("non-null function pointer")(data);
        }
    } else {
        let mut thread_default: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
        thread_default = safe_c2rust_g_main_context_get_thread_default();
        if thread_default.is_null() {
            thread_default = safe_c2rust_g_main_context_default();
        }
        if thread_default == context && safe_c2rust_g_main_context_acquire(context) != 0 {
            while function.expect("non-null function pointer")(data) != 0 {}
            safe_c2rust_g_main_context_release(context);
            if notify.is_some() {
                notify.expect("non-null function pointer")(data);
            }
        } else {
            let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
            source = safe_c2rust_g_idle_source_new();
            safe_c2rust_g_source_set_priority(source, priority);
            safe_c2rust_g_source_set_callback(source, function, data, notify);
            safe_c2rust_g_source_attach(source, context);
            safe_c2rust_g_source_unref(source);
        }
    };
}
unsafe extern "C" fn safe_c2rust_glib_worker_main(mut data: gpointer) -> gpointer {
    while FALSE == 0 {
        safe_c2rust_g_main_context_iteration(safe_c2rust_glib_worker_context, TRUE);
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_any_unix_signal_pending;
                safe_c2rust_any_unix_signal_pending;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut safe_c2rust_any_unix_signal_pending as *mut gint,
            );
            gaig_temp
        }) != 0
        {
            safe_c2rust_dispatch_unix_signals();
        }
    }
    return NULL_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_worker_context() -> *mut GMainContext {
    static mut safe_c2rust_initialised: gsize = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialised;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut prev_mask: sigset_t = __sigset_t { __val: [0; 16] };
        let mut all: sigset_t = __sigset_t { __val: [0; 16] };
        sigfillset(&raw mut all);
        pthread_sigmask(SIG_SETMASK, &raw mut all, &raw mut prev_mask);
        safe_c2rust_glib_worker_context = safe_c2rust_g_main_context_new();
        g_thread_new(
            b"gmain\0" as *const u8 as *const gchar,
            Some(safe_c2rust_glib_worker_main as unsafe extern "C" fn(gpointer) -> gpointer),
            NULL_0,
        );
        pthread_sigmask(
            SIG_SETMASK,
            &raw mut prev_mask,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
    return safe_c2rust_glib_worker_context;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_main_context_ref\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
