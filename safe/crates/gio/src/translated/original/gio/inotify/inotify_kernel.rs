extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GWakeup;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_source_add_unix_fd(source: *mut GSource, fd: gint, events: GIOCondition) -> gpointer;
    fn g_source_modify_unix_fd(source: *mut GSource, tag: gpointer, new_events: GIOCondition);
    fn g_source_query_unix_fd(source: *mut GSource, tag: gpointer) -> GIOCondition;
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    fn g_queue_peek_head(queue: *mut GQueue) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_error(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        error: *const GError,
        error_domain: GQuark,
        error_code: ::core::ffi::c_int,
    );
    fn inotify_init() -> ::core::ffi::c_int;
    fn inotify_init1(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inotify_add_watch(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __mask: uint32_t,
    ) -> ::core::ffi::c_int;
    fn inotify_rm_watch(__fd: ::core::ffi::c_int, __wd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_unix_set_fd_nonblocking(fd: gint, nonblock: gboolean, error: *mut *mut GError)
        -> gboolean;
    fn glib__private__() -> *const GLibPrivateVTable;
    static mut safe_c2rust_g__inotify_lock_lock: GMutex;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GDir = _GDir;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
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
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GQueue = _GQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ik_event_s {
    pub wd: gint32,
    pub mask: guint32,
    pub original_mask: guint32,
    pub cookie: guint32,
    pub len: guint32,
    pub name: *mut ::core::ffi::c_char,
    pub is_second_in_pair: gboolean,
    pub pair: *mut ik_event_s,
    pub timestamp: gint64,
}
pub type ik_event_t = ik_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InotifyKernelSource {
    pub source: GSource,
    pub queue: GQueue,
    pub fd_tag: gpointer,
    pub fd: gint,
    pub unmatched_moves: *mut GHashTable,
    pub is_bored: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWakeup = _GWakeup;
pub const IN_NONBLOCK: C2RustUnnamed = 2048;
pub const IN_CLOEXEC: C2RustUnnamed = 524288;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: ::core::ffi::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [::core::ffi::c_char; 0],
}
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"ik_source_new\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_TIME_SPAN_MILLISECOND: ::core::ffi::c_long = 1000 as ::core::ffi::c_long;
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
pub const IN_MOVED_FROM: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const IN_MOVED_TO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MAX_EVENT_SIZE: usize = (::core::mem::size_of::<inotify_event>() as usize)
    .wrapping_add(NAME_MAX as usize)
    .wrapping_add(1 as usize);
pub const BOREDOM_SLEEP_TIME: ::core::ffi::c_long =
    100 as ::core::ffi::c_long * G_TIME_SPAN_MILLISECOND;
pub const MOVE_PAIR_DELAY: ::core::ffi::c_long =
    10 as ::core::ffi::c_long * G_TIME_SPAN_MILLISECOND;
pub const MOVE_PAIR_DISTANCE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_ik_event_new(
    mut kevent: *mut inotify_event,
    mut now: gint64,
) -> *mut ik_event_t {
    let mut event: *mut ik_event_t = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ik_event_t>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ik_event_t;
    (*event).wd = (*kevent).wd as gint32;
    (*event).mask = (*kevent).mask as guint32;
    (*event).cookie = (*kevent).cookie as guint32;
    (*event).len = (*kevent).len as guint32;
    (*event).timestamp = now;
    if (*event).len != 0 {
        (*event).name =
            safe_c2rust_g_strdup_inline(&raw mut (*kevent).name as *mut ::core::ffi::c_char);
    } else {
        (*event).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return event;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ik_event_free(mut event: *mut ik_event_t) {
    if !(*event).pair.is_null() {
        (*(*event).pair).pair = ::core::ptr::null_mut::<ik_event_s>();
        safe_c2rust__ik_event_free((*event).pair as *mut ik_event_t);
    }
    g_free((*event).name as gpointer);
    g_free(event as gpointer);
}
static mut safe_c2rust_inotify_source: *mut InotifyKernelSource =
    ::core::ptr::null::<InotifyKernelSource>() as *mut InotifyKernelSource;
unsafe extern "C" fn safe_c2rust_ik_source_get_dispatch_time(
    mut iks: *mut InotifyKernelSource,
) -> gint64 {
    let mut head: *mut ik_event_t = ::core::ptr::null_mut::<ik_event_t>();
    head = g_queue_peek_head(&raw mut (*iks).queue) as *mut ik_event_t;
    if head.is_null() {
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    if !(*head).mask & IN_MOVED_FROM as guint32 != 0 || !(*head).pair.is_null() {
        return 0 as gint64;
    }
    if (*iks).queue.length > MOVE_PAIR_DISTANCE as guint {
        return 0 as gint64;
    }
    return (*head).timestamp + MOVE_PAIR_DELAY;
}
unsafe extern "C" fn safe_c2rust_ik_source_can_dispatch_now(
    mut iks: *mut InotifyKernelSource,
    mut now: gint64,
) -> gboolean {
    let mut dispatch_time: gint64 = 0;
    dispatch_time = safe_c2rust_ik_source_get_dispatch_time(iks);
    return (0 as gint64 <= dispatch_time && dispatch_time <= now) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_ik_source_read_some_events(
    mut iks: *mut InotifyKernelSource,
    mut buffer: *mut gchar,
    mut buffer_len: gsize,
) -> gsize {
    let mut result: gssize = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    loop {
        result = read(
            (*iks).fd as ::core::ffi::c_int,
            buffer as *mut ::core::ffi::c_void,
            buffer_len as size_t,
        ) as gssize;
        errsv = *__errno_location();
        if result < 0 as gssize {
            if errsv == EINTR {
                continue;
            }
            if errsv == EAGAIN {
                return 0 as gsize;
            }
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"inotify read(): %s\0" as *const u8 as *const gchar,
                g_strerror(errsv as gint),
            );
            loop {}
        } else {
            if result == 0 as gssize {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"inotify unexpectedly hit eof\0" as *const u8 as *const gchar,
                );
                loop {}
            }
            return result as gsize;
        }
    }
}
unsafe extern "C" fn safe_c2rust_ik_source_read_all_the_events(
    mut iks: *mut InotifyKernelSource,
    mut buffer: *mut gchar,
    mut buffer_len: gsize,
    mut length_out: *mut gsize,
) -> *mut gchar {
    let mut n_read: gsize = 0;
    n_read = safe_c2rust_ik_source_read_some_events(iks, buffer, buffer_len);
    if (n_read as usize).wrapping_add(MAX_EVENT_SIZE) > buffer_len as usize {
        let mut new_buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut n_readable: guint = 0;
        let mut result: gint = 0;
        let mut errsv: ::core::ffi::c_int = 0;
        result = ioctl(
            (*iks).fd as ::core::ffi::c_int,
            FIONREAD as ::core::ffi::c_ulong,
            &raw mut n_readable,
        ) as gint;
        errsv = *__errno_location();
        if result != 0 as ::core::ffi::c_int {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"inotify ioctl(FIONREAD): %s\0" as *const u8 as *const gchar,
                g_strerror(errsv as gint),
            );
            loop {}
        }
        if n_readable != 0 as guint {
            new_buffer = g_malloc(n_read.wrapping_add(n_readable as gsize)) as *mut gchar;
            memcpy(
                new_buffer as *mut ::core::ffi::c_void,
                buffer as *const ::core::ffi::c_void,
                n_read as size_t,
            );
            n_read = n_read.wrapping_add(safe_c2rust_ik_source_read_some_events(
                iks,
                new_buffer.offset(n_read as isize),
                n_readable as gsize,
            ));
            buffer = new_buffer;
        }
    }
    *length_out = n_read;
    return buffer;
}
unsafe extern "C" fn safe_c2rust_ik_source_dispatch(
    mut source: *mut GSource,
    mut func: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut iks: *mut InotifyKernelSource = source as *mut InotifyKernelSource;
    let mut user_callback: Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean> =
        ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean>,
        >(::core::mem::transmute::<
            GSourceFunc,
            *mut ::core::ffi::c_void,
        >(func));
    let mut interesting: gboolean = FALSE;
    let mut now: gint64 = 0;
    now = g_source_get_time(source);
    if (*iks).is_bored != 0
        || g_source_query_unix_fd(source, (*iks).fd_tag) as ::core::ffi::c_uint != 0
    {
        let mut stack_buffer: [gchar; 4096] = [0; 4096];
        let mut buffer_len: gsize = 0;
        let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut offset: gsize = 0;
        buffer = safe_c2rust_ik_source_read_all_the_events(
            iks,
            &raw mut stack_buffer as *mut gchar,
            ::core::mem::size_of::<[gchar; 4096]>() as gsize,
            &raw mut buffer_len,
        );
        offset = 0 as gsize;
        while offset < buffer_len {
            let mut kevent: *mut inotify_event =
                buffer.offset(offset as isize) as *mut inotify_event;
            let mut event: *mut ik_event_t = ::core::ptr::null_mut::<ik_event_t>();
            event = safe_c2rust_ik_event_new(kevent, now);
            offset = (offset as ::core::ffi::c_ulong).wrapping_add(
                (::core::mem::size_of::<inotify_event>() as usize)
                    .wrapping_add((*event).len as usize) as ::core::ffi::c_ulong,
            ) as gsize as gsize;
            if (*event).mask & IN_MOVED_TO as guint32 != 0 {
                let mut pair: *mut ik_event_t = ::core::ptr::null_mut::<ik_event_t>();
                pair = g_hash_table_lookup(
                    (*iks).unmatched_moves,
                    (*event).cookie as gulong as gpointer as gconstpointer,
                ) as *mut ik_event_t;
                if !pair.is_null() {
                    if ({
                        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
                        if (*pair).pair.is_null() {
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
                            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            273 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"!pair->pair\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    g_hash_table_remove(
                        (*iks).unmatched_moves,
                        (*event).cookie as gulong as gpointer as gconstpointer,
                    );
                    (*event).is_second_in_pair = TRUE as gboolean;
                    (*event).pair = pair as *mut ik_event_s;
                    (*pair).pair = event as *mut ik_event_s;
                    continue;
                } else {
                    interesting = TRUE as gboolean;
                }
            } else if (*event).mask & IN_MOVED_FROM as guint32 != 0 {
                let mut new: gboolean = 0;
                new = g_hash_table_insert(
                    (*iks).unmatched_moves,
                    (*event).cookie as gulong as gpointer,
                    event as gpointer,
                );
                if ({
                    let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
                    if new == 0 {
                        _g_boolean_var_9 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_9 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_9
                }) as ::core::ffi::c_long
                    != 0
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"inotify: got IN_MOVED_FROM event with already-pending cookie %#x\0"
                            as *const u8 as *const gchar,
                        (*event).cookie,
                    );
                }
                interesting = TRUE as gboolean;
            }
            g_queue_push_tail(&raw mut (*iks).queue, event as gpointer);
        }
        if buffer_len == 0 as gsize {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if (*iks).is_bored != 0 {
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
                    b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    307 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"iks->is_bored\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            interesting = TRUE as gboolean;
        }
        if buffer != &raw mut stack_buffer as *mut gchar {
            g_free(buffer as gpointer);
        }
    }
    while safe_c2rust_ik_source_can_dispatch_now(iks, now) != 0 {
        let mut event_0: *mut ik_event_t = ::core::ptr::null_mut::<ik_event_t>();
        event_0 = g_queue_pop_head(&raw mut (*iks).queue) as *mut ik_event_t;
        if (*event_0).mask & IN_MOVED_FROM as guint32 != 0 && (*event_0).pair.is_null() {
            g_hash_table_remove(
                (*iks).unmatched_moves,
                (*event_0).cookie as gulong as gpointer as gconstpointer,
            );
        }
        g_mutex_lock(&raw mut safe_c2rust_g__inotify_lock_lock);
        interesting |= Some(user_callback.expect("non-null function pointer"))
            .expect("non-null function pointer")(event_0);
        g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ((*iks).queue.length > 0 as guint) as ::core::ffi::c_int
            == (g_hash_table_size((*iks).unmatched_moves) > 0 as guint) as ::core::ffi::c_int
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            333 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(iks->queue.length > 0) == (g_hash_table_size (iks->unmatched_moves) > 0)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if interesting != 0 {
        if (*iks).is_bored != 0 {
            g_source_modify_unix_fd(source, (*iks).fd_tag, G_IO_IN);
            (*iks).is_bored = FALSE as gboolean;
        }
        g_source_set_ready_time(source, safe_c2rust_ik_source_get_dispatch_time(iks));
    } else {
        let mut dispatch_time: guint64 = safe_c2rust_ik_source_get_dispatch_time(iks) as guint64;
        let mut boredom_time: guint64 =
            (now as ::core::ffi::c_long + BOREDOM_SLEEP_TIME) as guint64;
        if (*iks).is_bored == 0 {
            g_source_modify_unix_fd(source, (*iks).fd_tag, 0 as GIOCondition);
            (*iks).is_bored = TRUE as gboolean;
        }
        g_source_set_ready_time(
            source,
            (if dispatch_time < boredom_time {
                dispatch_time
            } else {
                boredom_time
            }) as gint64,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_ik_source_new(
    mut callback: Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean>,
) -> *mut InotifyKernelSource {
    static mut safe_c2rust_source_funcs: GSourceFuncs = unsafe {
        _GSourceFuncs {
            prepare: None,
            check: None,
            dispatch: Some(
                safe_c2rust_ik_source_dispatch
                    as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
            ),
            finalize: None,
            closure_callback: None,
            closure_marshal: None,
        }
    };
    let mut iks: *mut InotifyKernelSource = ::core::ptr::null_mut::<InotifyKernelSource>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut should_set_nonblock: gboolean = FALSE;
    source = g_source_new(
        &raw mut safe_c2rust_source_funcs,
        ::core::mem::size_of::<InotifyKernelSource>() as guint,
    );
    iks = source as *mut InotifyKernelSource;
    g_source_set_static_name(
        source,
        b"inotify kernel source\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*iks).unmatched_moves = g_hash_table_new(None, None);
    (*iks).fd =
        inotify_init1(IN_CLOEXEC as ::core::ffi::c_int | IN_NONBLOCK as ::core::ffi::c_int) as gint;
    if (*iks).fd < 0 as ::core::ffi::c_int {
        should_set_nonblock = TRUE as gboolean;
        (*iks).fd = inotify_init() as gint;
    }
    if (*iks).fd >= 0 as ::core::ffi::c_int {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        if should_set_nonblock != 0 {
            g_unix_set_fd_nonblocking((*iks).fd, TRUE, &raw mut error);
            if !error.is_null() {
                g_assertion_message_error(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    403 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char,
                    error,
                    0 as GQuark,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        (*iks).fd_tag = g_source_add_unix_fd(source, (*iks).fd, G_IO_IN);
    }
    g_source_set_callback(
        source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean>,
            GSourceFunc,
        >(callback),
        NULL_0,
        None,
    );
    g_source_attach(
        source,
        (*glib__private__())
            .g_get_worker_context
            .expect("non-null function pointer")(),
    );
    return iks;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ik_startup(
    mut cb: Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean>,
) -> gboolean {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_inotify_source;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut InotifyKernelSource =
                ::core::ptr::null_mut::<InotifyKernelSource>();
            let mut gapg_temp_atomic: *mut *mut InotifyKernelSource =
                &raw mut safe_c2rust_inotify_source;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_inotify_source as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_inotify_source = safe_c2rust_ik_source_new(cb);
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_inotify_source as *mut ::core::ffi::c_void,
            safe_c2rust_ik_source_new(cb) as guintptr as gpointer,
        );
    }
    return ((*safe_c2rust_inotify_source).fd >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ik_watch(
    mut path: *const ::core::ffi::c_char,
    mut mask: guint32,
    mut err: *mut ::core::ffi::c_int,
) -> gint32 {
    let mut wd: gint32 = -(1 as gint32);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !path.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            432 as ::core::ffi::c_int,
            G_STRFUNC,
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !safe_c2rust_inotify_source.is_null()
            && (*safe_c2rust_inotify_source).fd >= 0 as ::core::ffi::c_int
        {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            433 as ::core::ffi::c_int,
            G_STRFUNC,
            b"inotify_source && inotify_source->fd >= 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    wd = inotify_add_watch(
        (*safe_c2rust_inotify_source).fd as ::core::ffi::c_int,
        path,
        mask as uint32_t,
    ) as gint32;
    if wd < 0 as ::core::ffi::c_int {
        let mut e: ::core::ffi::c_int = *__errno_location();
        if !err.is_null() {
            *err = e;
        }
        return wd;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int {
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
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            446 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return wd;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ik_ignore(
    mut path: *const ::core::ffi::c_char,
    mut wd: gint32,
) -> ::core::ffi::c_int {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            454 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !safe_c2rust_inotify_source.is_null()
            && (*safe_c2rust_inotify_source).fd >= 0 as ::core::ffi::c_int
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-kernel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int,
            G_STRFUNC,
            b"inotify_source && inotify_source->fd >= 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if inotify_rm_watch(
        (*safe_c2rust_inotify_source).fd as ::core::ffi::c_int,
        wd as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
