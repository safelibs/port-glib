extern "C" {
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_set_size(array: *mut GArray, length: guint) -> *mut GArray;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_slist_free_1(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slist_copy(list: *mut GSList) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_thread_unref(thread: *mut GThread);
    fn g_thread_yield();
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_trylock(mutex: *mut GMutex) -> gboolean;
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_rec_mutex_init(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_clear(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_trylock(rec_mutex: *mut GRecMutex) -> gboolean;
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_cond_wait_until(cond: *mut GCond, mutex: *mut GMutex, end_time: gint64) -> gboolean;
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_get_monotonic_time() -> gint64;
    fn g_get_real_time() -> gint64;
    fn g_thread_new_internal(
        name: *const gchar,
        proxy: GThreadFunc,
        func: GThreadFunc,
        data: gpointer,
        stack_size: gsize,
        error: *mut *mut GError,
    ) -> *mut GThread;
    fn g_thread_proxy(thread: gpointer) -> gpointer;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
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
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub struct _GThreadFunctions {
    pub mutex_new: Option<unsafe extern "C" fn() -> *mut GMutex>,
    pub mutex_lock: Option<unsafe extern "C" fn(*mut GMutex) -> ()>,
    pub mutex_trylock: Option<unsafe extern "C" fn(*mut GMutex) -> gboolean>,
    pub mutex_unlock: Option<unsafe extern "C" fn(*mut GMutex) -> ()>,
    pub mutex_free: Option<unsafe extern "C" fn(*mut GMutex) -> ()>,
    pub cond_new: Option<unsafe extern "C" fn() -> *mut GCond>,
    pub cond_signal: Option<unsafe extern "C" fn(*mut GCond) -> ()>,
    pub cond_broadcast: Option<unsafe extern "C" fn(*mut GCond) -> ()>,
    pub cond_wait: Option<unsafe extern "C" fn(*mut GCond, *mut GMutex) -> ()>,
    pub cond_timed_wait:
        Option<unsafe extern "C" fn(*mut GCond, *mut GMutex, *mut GTimeVal) -> gboolean>,
    pub cond_free: Option<unsafe extern "C" fn(*mut GCond) -> ()>,
    pub private_new: Option<unsafe extern "C" fn(GDestroyNotify) -> *mut GPrivate>,
    pub private_get: Option<unsafe extern "C" fn(*mut GPrivate) -> gpointer>,
    pub private_set: Option<unsafe extern "C" fn(*mut GPrivate, gpointer) -> ()>,
    pub thread_create: Option<
        unsafe extern "C" fn(
            GThreadFunc,
            gpointer,
            gulong,
            gboolean,
            gboolean,
            GThreadPriority,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub thread_yield: Option<unsafe extern "C" fn() -> ()>,
    pub thread_join: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub thread_exit: Option<unsafe extern "C" fn() -> ()>,
    pub thread_set_priority: Option<unsafe extern "C" fn(gpointer, GThreadPriority) -> ()>,
    pub thread_self: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub thread_equal: Option<unsafe extern "C" fn(gpointer, gpointer) -> gboolean>,
}
pub type GThreadFunctions = _GThreadFunctions;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GStaticMutex {
    pub mutex: *mut GMutex,
    pub unused: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStaticRecMutex {
    pub mutex: GStaticMutex,
    pub depth: guint,
    pub unused: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub owner: pthread_t,
    pub dummy: gdouble,
}
pub type GStaticRecMutex = _GStaticRecMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStaticRWLock {
    pub mutex: GStaticMutex,
    pub read_cond: *mut GCond,
    pub write_cond: *mut GCond,
    pub read_counter: guint,
    pub have_writer: gboolean,
    pub want_to_read: guint,
    pub want_to_write: guint,
}
pub type GStaticRWLock = _GStaticRWLock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStaticPrivate {
    pub index: guint,
}
pub type GStaticPrivate = _GStaticPrivate;
pub type GStaticPrivateNode = _GStaticPrivateNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStaticPrivateNode {
    pub data: gpointer,
    pub destroy: GDestroyNotify,
    pub owner: *mut GStaticPrivate,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut safe_c2rust_g_thread_use_default_impl: gboolean = FALSE;
#[no_mangle]
pub static mut safe_c2rust_g_thread_functions_for_glib_use: GThreadFunctions = unsafe {
    _GThreadFunctions {
        mutex_new: Some(safe_c2rust_g_mutex_new as unsafe extern "C" fn() -> *mut GMutex),
        mutex_lock: Some(g_mutex_lock as unsafe extern "C" fn(*mut GMutex) -> ()),
        mutex_trylock: Some(g_mutex_trylock as unsafe extern "C" fn(*mut GMutex) -> gboolean),
        mutex_unlock: Some(g_mutex_unlock as unsafe extern "C" fn(*mut GMutex) -> ()),
        mutex_free: Some(safe_c2rust_g_mutex_free as unsafe extern "C" fn(*mut GMutex) -> ()),
        cond_new: Some(safe_c2rust_g_cond_new as unsafe extern "C" fn() -> *mut GCond),
        cond_signal: Some(g_cond_signal as unsafe extern "C" fn(*mut GCond) -> ()),
        cond_broadcast: Some(g_cond_broadcast as unsafe extern "C" fn(*mut GCond) -> ()),
        cond_wait: Some(g_cond_wait as unsafe extern "C" fn(*mut GCond, *mut GMutex) -> ()),
        cond_timed_wait: Some(
            safe_c2rust_g_cond_timed_wait
                as unsafe extern "C" fn(*mut GCond, *mut GMutex, *mut GTimeVal) -> gboolean,
        ),
        cond_free: Some(safe_c2rust_g_cond_free as unsafe extern "C" fn(*mut GCond) -> ()),
        private_new: Some(
            safe_c2rust_g_private_new as unsafe extern "C" fn(GDestroyNotify) -> *mut GPrivate,
        ),
        private_get: Some(g_private_get as unsafe extern "C" fn(*mut GPrivate) -> gpointer),
        private_set: Some(g_private_set as unsafe extern "C" fn(*mut GPrivate, gpointer) -> ()),
        thread_create: None,
        thread_yield: Some(g_thread_yield as unsafe extern "C" fn() -> ()),
        thread_join: None,
        thread_exit: None,
        thread_set_priority: None,
        thread_self: None,
        thread_equal: None,
    }
};
unsafe extern "C" fn safe_c2rust_gettime() -> guint64 {
    return (g_get_monotonic_time() * 1000 as gint64) as guint64;
}
#[no_mangle]
pub static mut safe_c2rust_g_thread_gettime: Option<unsafe extern "C" fn() -> guint64> =
    unsafe { Some(safe_c2rust_gettime as unsafe extern "C" fn() -> guint64) };
#[no_mangle]
pub static mut safe_c2rust_g_threads_got_initialized: gboolean = TRUE;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_get_initialized() -> gboolean {
    return 1 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_init_glib() {}
static mut safe_c2rust_g_thread_all_threads: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_g_thread_free_indices: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_g__g_static_mutex_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_g__g_thread_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_set_priority(
    mut thread: *mut GThread,
    mut priority: GThreadPriority,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_foreach(
    mut thread_func: GFunc,
    mut user_data: gpointer,
) {
    let mut slist: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut thread: *mut GRealThread = ::core::ptr::null_mut::<GRealThread>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if thread_func.is_some() {
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
            b"thread_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
    slist = g_slist_copy(safe_c2rust_g_thread_all_threads);
    g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
    while !slist.is_null() {
        let mut node: *mut GSList = slist;
        slist = (*node).next;
        g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
        if !g_slist_find(
            safe_c2rust_g_thread_all_threads,
            (*node).data as gconstpointer,
        )
        .is_null()
        {
            thread = (*node).data as *mut GRealThread;
        } else {
            thread = ::core::ptr::null_mut::<GRealThread>();
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
        if !thread.is_null() {
            thread_func.expect("non-null function pointer")(thread as gpointer, user_data);
        }
        g_slist_free_1(node);
    }
}
unsafe extern "C" fn safe_c2rust_g_enumerable_thread_remove(mut data: gpointer) {
    let mut thread: *mut GRealThread = data as *mut GRealThread;
    g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
    safe_c2rust_g_thread_all_threads =
        g_slist_remove(safe_c2rust_g_thread_all_threads, thread as gconstpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
}
#[no_mangle]
pub static mut safe_c2rust_enumerable_thread_private: GPrivate = unsafe {
    _GPrivate {
        p: NULL,
        notify: Some(
            safe_c2rust_g_enumerable_thread_remove as unsafe extern "C" fn(gpointer) -> (),
        ),
        future: [NULL, NULL],
    }
};
unsafe extern "C" fn safe_c2rust_g_enumerable_thread_add(mut thread: *mut GRealThread) {
    g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
    safe_c2rust_g_thread_all_threads =
        g_slist_prepend(safe_c2rust_g_thread_all_threads, thread as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
    g_private_set(
        &raw mut safe_c2rust_enumerable_thread_private,
        thread as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_deprecated_thread_proxy(mut data: gpointer) -> gpointer {
    let mut real: *mut GRealThread = data as *mut GRealThread;
    safe_c2rust_g_enumerable_thread_add(real);
    return g_thread_proxy(data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_create(
    mut func: GThreadFunc,
    mut data: gpointer,
    mut joinable: gboolean,
    mut error: *mut *mut GError,
) -> *mut GThread {
    return safe_c2rust_g_thread_create_full(
        func,
        data,
        0 as gulong,
        joinable,
        0 as gboolean,
        G_THREAD_PRIORITY_LOW,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_create_full(
    mut func: GThreadFunc,
    mut data: gpointer,
    mut stack_size: gulong,
    mut joinable: gboolean,
    mut bound: gboolean,
    mut priority: GThreadPriority,
    mut error: *mut *mut GError,
) -> *mut GThread {
    let mut thread: *mut GThread = ::core::ptr::null_mut::<GThread>();
    thread = g_thread_new_internal(
        ::core::ptr::null::<gchar>(),
        Some(safe_c2rust_g_deprecated_thread_proxy as unsafe extern "C" fn(gpointer) -> gpointer),
        func,
        data,
        stack_size as gsize,
        error,
    );
    if !thread.is_null() && joinable == 0 {
        (*thread).joinable = FALSE as gboolean;
        g_thread_unref(thread);
    }
    return thread;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_once_init_enter_impl(mut location: *mut gsize) -> gboolean {
    return g_once_init_enter(location as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_mutex_init(mut mutex: *mut GStaticMutex) {
    static mut safe_c2rust_init_mutex: GStaticMutex = GStaticMutex {
        mutex: ::core::ptr::null::<GMutex>() as *mut GMutex,
        unused: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0 as ::core::ffi::c_int,
                __count: 0 as ::core::ffi::c_uint,
                __owner: 0 as ::core::ffi::c_int,
                __nusers: 0 as ::core::ffi::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                __spins: 0 as ::core::ffi::c_short,
                __elision: 0 as ::core::ffi::c_short,
                __list: __pthread_internal_list {
                    __prev: ::core::ptr::null::<__pthread_internal_list>()
                        as *mut __pthread_internal_list,
                    __next: ::core::ptr::null::<__pthread_internal_list>()
                        as *mut __pthread_internal_list,
                },
            },
        },
    };
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !mutex.is_null() {
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
            b"mutex\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *mutex = safe_c2rust_init_mutex;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_mutex_get_mutex_impl(
    mut mutex: *mut GStaticMutex,
) -> *mut GMutex {
    let mut result: *mut GMutex = ::core::ptr::null_mut::<GMutex>();
    if 1 as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<GMutex>();
    }
    result = ({
        let mut gapg_temp_newval: *mut GMutex = ::core::ptr::null_mut::<GMutex>();
        let mut gapg_temp_atomic: *mut *mut GMutex = &raw mut (*mutex).mutex;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut GMutex;
    if result.is_null() {
        g_mutex_lock(&raw mut safe_c2rust_g__g_static_mutex_lock);
        result = (*mutex).mutex;
        if result.is_null() {
            result = safe_c2rust_g_mutex_new();
            let mut gaps_temp_atomic: *mut *mut GMutex = &raw mut (*mutex).mutex;
            let mut gaps_temp_newval: *mut GMutex = result as *mut GMutex;
            if 0 as ::core::ffi::c_int != 0 {
                (*mutex).mutex;
            } else {
            };
            crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__g_static_mutex_lock);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_mutex_free(mut mutex: *mut GStaticMutex) {
    let mut runtime_mutex: *mut *mut GMutex = ::core::ptr::null_mut::<*mut GMutex>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !mutex.is_null() {
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
            b"mutex\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    runtime_mutex = mutex as *mut *mut GMutex;
    if !(*runtime_mutex).is_null() {
        safe_c2rust_g_mutex_free(*runtime_mutex);
    }
    *runtime_mutex = ::core::ptr::null_mut::<GMutex>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_init(mut mutex: *mut GStaticRecMutex) {
    static mut safe_c2rust_init_mutex: GStaticRecMutex = _GStaticRecMutex {
        mutex: GStaticMutex {
            mutex: ::core::ptr::null::<GMutex>() as *mut GMutex,
            unused: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0 as ::core::ffi::c_int,
                    __count: 0 as ::core::ffi::c_uint,
                    __owner: 0 as ::core::ffi::c_int,
                    __nusers: 0 as ::core::ffi::c_uint,
                    __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                    __spins: 0 as ::core::ffi::c_short,
                    __elision: 0 as ::core::ffi::c_short,
                    __list: __pthread_internal_list {
                        __prev: ::core::ptr::null::<__pthread_internal_list>()
                            as *mut __pthread_internal_list,
                        __next: ::core::ptr::null::<__pthread_internal_list>()
                            as *mut __pthread_internal_list,
                    },
                },
            },
        },
        depth: 0 as guint,
        unused: C2RustUnnamed_0 {
            owner: 0 as ::core::ffi::c_int as pthread_t,
        },
    };
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !mutex.is_null() {
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
            b"mutex\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *mutex = safe_c2rust_init_mutex;
}
unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(
    mut mutex: *mut GStaticRecMutex,
) -> *mut GRecMutex {
    let mut result: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    if 1 as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<GRecMutex>();
    }
    result = ({
        let mut gapg_temp_newval: *mut GMutex = ::core::ptr::null_mut::<GMutex>();
        let mut gapg_temp_atomic: *mut *mut GMutex = &raw mut (*mutex).mutex.mutex;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut GRecMutex;
    if result.is_null() {
        g_mutex_lock(&raw mut safe_c2rust_g__g_static_mutex_lock);
        result = (*mutex).mutex.mutex as *mut GRecMutex;
        if result.is_null() {
            result = g_slice_alloc(::core::mem::size_of::<GRecMutex>() as gsize) as *mut GRecMutex;
            g_rec_mutex_init(result);
            let mut gaps_temp_atomic: *mut *mut GMutex = &raw mut (*mutex).mutex.mutex;
            let mut gaps_temp_newval: *mut GMutex = result as *mut GMutex;
            if 0 as ::core::ffi::c_int != 0 {
                (*mutex).mutex.mutex;
            } else {
            };
            crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__g_static_mutex_lock);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_lock(mut mutex: *mut GStaticRecMutex) {
    let mut rm: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    rm = safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(mutex);
    g_rec_mutex_lock(rm);
    (*mutex).depth = (*mutex).depth.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_trylock(
    mut mutex: *mut GStaticRecMutex,
) -> gboolean {
    let mut rm: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    rm = safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(mutex);
    if g_rec_mutex_trylock(rm) != 0 {
        (*mutex).depth = (*mutex).depth.wrapping_add(1);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_unlock(mut mutex: *mut GStaticRecMutex) {
    let mut rm: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    rm = safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(mutex);
    (*mutex).depth = (*mutex).depth.wrapping_sub(1);
    g_rec_mutex_unlock(rm);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_lock_full(
    mut mutex: *mut GStaticRecMutex,
    mut depth: guint,
) {
    let mut rm: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    rm = safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(mutex);
    loop {
        let fresh0 = depth;
        depth = depth.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        g_rec_mutex_lock(rm);
        (*mutex).depth = (*mutex).depth.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_unlock_full(
    mut mutex: *mut GStaticRecMutex,
) -> guint {
    let mut rm: *mut GRecMutex = ::core::ptr::null_mut::<GRecMutex>();
    let mut depth: gint = 0;
    let mut i: gint = 0;
    rm = safe_c2rust_g_static_rec_mutex_get_rec_mutex_impl(mutex);
    depth = (*mutex).depth as gint;
    i = (*mutex).depth as gint;
    (*mutex).depth = 0 as guint;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(fresh1 != 0) {
            break;
        }
        g_rec_mutex_unlock(rm);
    }
    return depth as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rec_mutex_free(mut mutex: *mut GStaticRecMutex) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !mutex.is_null() {
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
            b"mutex\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*mutex).mutex.mutex.is_null() {
        let mut rm: *mut GRecMutex = (*mutex).mutex.mutex as *mut GRecMutex;
        g_rec_mutex_clear(rm);
        g_slice_free1(::core::mem::size_of::<GRecMutex>() as gsize, rm as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_init(mut lock: *mut GStaticRWLock) {
    static mut safe_c2rust_init_lock: GStaticRWLock = _GStaticRWLock {
        mutex: GStaticMutex {
            mutex: ::core::ptr::null::<GMutex>() as *mut GMutex,
            unused: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0 as ::core::ffi::c_int,
                    __count: 0 as ::core::ffi::c_uint,
                    __owner: 0 as ::core::ffi::c_int,
                    __nusers: 0 as ::core::ffi::c_uint,
                    __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                    __spins: 0 as ::core::ffi::c_short,
                    __elision: 0 as ::core::ffi::c_short,
                    __list: __pthread_internal_list {
                        __prev: ::core::ptr::null::<__pthread_internal_list>()
                            as *mut __pthread_internal_list,
                        __next: ::core::ptr::null::<__pthread_internal_list>()
                            as *mut __pthread_internal_list,
                    },
                },
            },
        },
        read_cond: ::core::ptr::null::<GCond>() as *mut GCond,
        write_cond: ::core::ptr::null::<GCond>() as *mut GCond,
        read_counter: 0 as guint,
        have_writer: FALSE,
        want_to_read: 0 as guint,
        want_to_write: 0 as guint,
    };
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *lock = safe_c2rust_init_lock;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_static_rw_lock_wait(
    mut cond: *mut *mut GCond,
    mut mutex: *mut GStaticMutex,
) {
    if (*cond).is_null() {
        *cond = safe_c2rust_g_cond_new();
    }
    g_cond_wait(*cond, safe_c2rust_g_static_mutex_get_mutex_impl(mutex));
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_static_rw_lock_signal(mut lock: *mut GStaticRWLock) {
    if (*lock).want_to_write != 0 && !(*lock).write_cond.is_null() {
        g_cond_signal((*lock).write_cond);
    } else if (*lock).want_to_read != 0 && !(*lock).read_cond.is_null() {
        g_cond_broadcast((*lock).read_cond);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_reader_lock(mut lock: *mut GStaticRWLock) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    (*lock).want_to_read = (*lock).want_to_read.wrapping_add(1);
    while (*lock).have_writer != 0 || (*lock).want_to_write != 0 {
        safe_c2rust_g_static_rw_lock_wait(&raw mut (*lock).read_cond, &raw mut (*lock).mutex);
    }
    (*lock).want_to_read = (*lock).want_to_read.wrapping_sub(1);
    (*lock).read_counter = (*lock).read_counter.wrapping_add(1);
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_reader_trylock(
    mut lock: *mut GStaticRWLock,
) -> gboolean {
    let mut ret_val: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return TRUE;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    if (*lock).have_writer == 0 && (*lock).want_to_write == 0 {
        (*lock).read_counter = (*lock).read_counter.wrapping_add(1);
        ret_val = TRUE as gboolean;
    }
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_reader_unlock(mut lock: *mut GStaticRWLock) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    (*lock).read_counter = (*lock).read_counter.wrapping_sub(1);
    if (*lock).read_counter == 0 as guint {
        safe_c2rust_g_static_rw_lock_signal(lock);
    }
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_writer_lock(mut lock: *mut GStaticRWLock) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    (*lock).want_to_write = (*lock).want_to_write.wrapping_add(1);
    while (*lock).have_writer != 0 || (*lock).read_counter != 0 {
        safe_c2rust_g_static_rw_lock_wait(&raw mut (*lock).write_cond, &raw mut (*lock).mutex);
    }
    (*lock).want_to_write = (*lock).want_to_write.wrapping_sub(1);
    (*lock).have_writer = TRUE as gboolean;
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_writer_trylock(
    mut lock: *mut GStaticRWLock,
) -> gboolean {
    let mut ret_val: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return TRUE;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    if (*lock).have_writer == 0 && (*lock).read_counter == 0 {
        (*lock).have_writer = TRUE as gboolean;
        ret_val = TRUE as gboolean;
    }
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_writer_unlock(mut lock: *mut GStaticRWLock) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_threads_got_initialized == 0 {
        return;
    }
    g_mutex_lock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
    (*lock).have_writer = FALSE as gboolean;
    safe_c2rust_g_static_rw_lock_signal(lock);
    g_mutex_unlock(safe_c2rust_g_static_mutex_get_mutex_impl(
        &raw mut (*lock).mutex,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_rw_lock_free(mut lock: *mut GStaticRWLock) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !lock.is_null() {
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
            b"lock\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*lock).read_cond.is_null() {
        safe_c2rust_g_cond_free((*lock).read_cond);
        (*lock).read_cond = ::core::ptr::null_mut::<GCond>();
    }
    if !(*lock).write_cond.is_null() {
        safe_c2rust_g_cond_free((*lock).write_cond);
        (*lock).write_cond = ::core::ptr::null_mut::<GCond>();
    }
    safe_c2rust_g_static_mutex_free(&raw mut (*lock).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_private_new(mut notify: GDestroyNotify) -> *mut GPrivate {
    let mut tmp: GPrivate = _GPrivate {
        p: NULL,
        notify: notify,
        future: [NULL, NULL],
    };
    let mut key: *mut GPrivate = ::core::ptr::null_mut::<GPrivate>();
    key = g_slice_alloc(::core::mem::size_of::<GPrivate>() as gsize) as *mut GPrivate;
    *key = tmp;
    return key;
}
unsafe extern "C" fn safe_c2rust_g_static_private_cleanup(mut data: gpointer) {
    let mut array: *mut GArray = data as *mut GArray;
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*array).len {
        let mut node: *mut GStaticPrivateNode =
            ((*array).data as *mut ::core::ffi::c_void as *mut GStaticPrivateNode)
                .offset(i as isize) as *mut GStaticPrivateNode;
        if (*node).destroy.is_some() {
            (*node).destroy.expect("non-null function pointer")((*node).data);
        }
        i = i.wrapping_add(1);
    }
    g_array_free(array, TRUE);
}
#[no_mangle]
pub static mut safe_c2rust_static_private_private: GPrivate = unsafe {
    _GPrivate {
        p: NULL,
        notify: Some(safe_c2rust_g_static_private_cleanup as unsafe extern "C" fn(gpointer) -> ()),
        future: [NULL, NULL],
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_private_init(mut private_key: *mut GStaticPrivate) {
    (*private_key).index = 0 as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_private_get(
    mut private_key: *mut GStaticPrivate,
) -> gpointer {
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut ret: gpointer = NULL;
    array = g_private_get(&raw mut safe_c2rust_static_private_private) as *mut GArray;
    if !array.is_null()
        && (*private_key).index != 0 as guint
        && (*private_key).index <= (*array).len
    {
        let mut node: *mut GStaticPrivateNode = ::core::ptr::null_mut::<GStaticPrivateNode>();
        node = ((*array).data as *mut ::core::ffi::c_void as *mut GStaticPrivateNode)
            .offset((*private_key).index.wrapping_sub(1 as guint) as isize)
            as *mut GStaticPrivateNode;
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if (*node).owner != private_key {
                _g_boolean_var_21 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_21 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_21
        }) as ::core::ffi::c_long
            != 0
        {
            if (*node).destroy.is_some() {
                (*node).destroy.expect("non-null function pointer")((*node).data);
            }
            (*node).destroy = None;
            (*node).data = NULL as gpointer;
            (*node).owner = ::core::ptr::null_mut::<GStaticPrivate>();
        }
        ret = (*node).data;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_private_set(
    mut private_key: *mut GStaticPrivate,
    mut data: gpointer,
    mut notify: GDestroyNotify,
) {
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    static mut safe_c2rust_next_index: guint = 0 as guint;
    let mut node: *mut GStaticPrivateNode = ::core::ptr::null_mut::<GStaticPrivateNode>();
    if (*private_key).index == 0 {
        g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
        if (*private_key).index == 0 {
            if !safe_c2rust_g_thread_free_indices.is_null() {
                (*private_key).index = (*safe_c2rust_g_thread_free_indices).data as gulong as guint;
                safe_c2rust_g_thread_free_indices = g_slist_delete_link(
                    safe_c2rust_g_thread_free_indices,
                    safe_c2rust_g_thread_free_indices,
                );
            } else {
                safe_c2rust_next_index = safe_c2rust_next_index.wrapping_add(1);
                (*private_key).index = safe_c2rust_next_index;
            }
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
    }
    array = g_private_get(&raw mut safe_c2rust_static_private_private) as *mut GArray;
    if array.is_null() {
        array = g_array_new(
            FALSE,
            TRUE,
            ::core::mem::size_of::<GStaticPrivateNode>() as guint,
        );
        g_private_set(
            &raw mut safe_c2rust_static_private_private,
            array as gpointer,
        );
    }
    if (*private_key).index > (*array).len {
        g_array_set_size(array, (*private_key).index);
    }
    node = ((*array).data as *mut ::core::ffi::c_void as *mut GStaticPrivateNode)
        .offset((*private_key).index.wrapping_sub(1 as guint) as isize)
        as *mut GStaticPrivateNode;
    if (*node).destroy.is_some() {
        (*node).destroy.expect("non-null function pointer")((*node).data);
    }
    (*node).data = data;
    (*node).destroy = notify;
    (*node).owner = private_key;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_private_free(mut private_key: *mut GStaticPrivate) {
    let mut idx: guint = (*private_key).index;
    if idx == 0 {
        return;
    }
    (*private_key).index = 0 as guint;
    g_mutex_lock(&raw mut safe_c2rust_g__g_thread_lock);
    safe_c2rust_g_thread_free_indices =
        g_slist_prepend(safe_c2rust_g_thread_free_indices, idx as gulong as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__g_thread_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_new() -> *mut GMutex {
    let mut mutex: *mut GMutex = ::core::ptr::null_mut::<GMutex>();
    mutex = g_slice_alloc(::core::mem::size_of::<GMutex>() as gsize) as *mut GMutex;
    g_mutex_init(mutex);
    return mutex;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mutex_free(mut mutex: *mut GMutex) {
    g_mutex_clear(mutex);
    g_slice_free1(::core::mem::size_of::<GMutex>() as gsize, mutex as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_new() -> *mut GCond {
    let mut cond: *mut GCond = ::core::ptr::null_mut::<GCond>();
    cond = g_slice_alloc(::core::mem::size_of::<GCond>() as gsize) as *mut GCond;
    g_cond_init(cond);
    return cond;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_free(mut cond: *mut GCond) {
    g_cond_clear(cond);
    g_slice_free1(::core::mem::size_of::<GCond>() as gsize, cond as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cond_timed_wait(
    mut cond: *mut GCond,
    mut mutex: *mut GMutex,
    mut abs_time: *mut GTimeVal,
) -> gboolean {
    let mut end_time: gint64 = 0;
    if abs_time.is_null() {
        g_cond_wait(cond, mutex);
        return TRUE;
    }
    end_time = (*abs_time).tv_sec as gint64;
    end_time *= 1000000 as gint64;
    end_time += (*abs_time).tv_usec as ::core::ffi::c_long;
    end_time += g_get_monotonic_time() - g_get_real_time();
    return g_cond_wait_until(cond, mutex, end_time);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_thread_foreach\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
