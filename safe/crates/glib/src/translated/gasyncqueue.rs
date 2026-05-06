extern "C" {
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_cond_wait_until(cond: *mut GCond, mutex: *mut GMutex, end_time: gint64) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_get_monotonic_time() -> gint64;
    fn g_get_real_time() -> gint64;
    fn g_queue_init(queue: *mut GQueue);
    fn g_queue_clear(queue: *mut GQueue);
    fn g_queue_foreach(queue: *mut GQueue, func: GFunc, user_data: gpointer);
    fn g_queue_sort(queue: *mut GQueue, compare_func: GCompareDataFunc, user_data: gpointer);
    fn g_queue_push_head(queue: *mut GQueue, data: gpointer);
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_tail(queue: *mut GQueue) -> gpointer;
    fn g_queue_remove(queue: *mut GQueue, data: gconstpointer) -> gboolean;
    fn g_queue_insert_sorted(
        queue: *mut GQueue,
        data: gpointer,
        func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_queue_peek_tail_link(queue: *mut GQueue) -> *mut GList;
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
}
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
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
pub struct _GAsyncQueue {
    pub mutex: GMutex,
    pub cond: GCond,
    pub queue: GQueue,
    pub item_free_func: GDestroyNotify,
    pub waiting_threads: guint,
    pub ref_count: gint,
}
pub type GQueue = _GQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GAsyncQueue = _GAsyncQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortData {
    pub func: GCompareDataFunc,
    pub user_data: gpointer,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_new() -> *mut GAsyncQueue {
    return safe_c2rust_g_async_queue_new_full(None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_new_full(
    mut item_free_func: GDestroyNotify,
) -> *mut GAsyncQueue {
    let mut queue: *mut GAsyncQueue = ::core::ptr::null_mut::<GAsyncQueue>();
    queue = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GAsyncQueue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GAsyncQueue;
    g_mutex_init(&raw mut (*queue).mutex);
    g_cond_init(&raw mut (*queue).cond);
    g_queue_init(&raw mut (*queue).queue);
    (*queue).waiting_threads = 0 as guint;
    (*queue).ref_count = 1 as ::core::ffi::c_int as gint;
    (*queue).item_free_func = item_free_func;
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_ref(
    mut queue: *mut GAsyncQueue,
) -> *mut GAsyncQueue {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAsyncQueue>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*queue).ref_count;
        (*queue).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*queue).ref_count, 1 as ::core::ffi::c_int);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_ref_unlocked(mut queue: *mut GAsyncQueue) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*queue).ref_count;
        (*queue).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*queue).ref_count, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_unref_and_unlock(mut queue: *mut GAsyncQueue) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_unlock(&raw mut (*queue).mutex);
    safe_c2rust_g_async_queue_unref(queue);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_unref(mut queue: *mut GAsyncQueue) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*queue).ref_count;
            (*queue).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*queue).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if (*queue).waiting_threads == 0 as guint {
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
                b"queue->waiting_threads == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        g_mutex_clear(&raw mut (*queue).mutex);
        g_cond_clear(&raw mut (*queue).cond);
        if (*queue).item_free_func.is_some() {
            g_queue_foreach(
                &raw mut (*queue).queue,
                ::core::mem::transmute::<GDestroyNotify, GFunc>((*queue).item_free_func),
                NULL,
            );
        }
        g_queue_clear(&raw mut (*queue).queue);
        g_free(queue as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_lock(mut queue: *mut GAsyncQueue) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_unlock(mut queue: *mut GAsyncQueue) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_unlock(&raw mut (*queue).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push(
    mut queue: *mut GAsyncQueue,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !data.is_null() {
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
            b"data\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    safe_c2rust_g_async_queue_push_unlocked(queue, data);
    g_mutex_unlock(&raw mut (*queue).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push_unlocked(
    mut queue: *mut GAsyncQueue,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !data.is_null() {
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
            b"data\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_queue_push_head(&raw mut (*queue).queue, data);
    if (*queue).waiting_threads > 0 as guint {
        g_cond_signal(&raw mut (*queue).cond);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push_sorted(
    mut queue: *mut GAsyncQueue,
    mut data: gpointer,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    safe_c2rust_g_async_queue_push_sorted_unlocked(queue, data, func, user_data);
    g_mutex_unlock(&raw mut (*queue).mutex);
}
unsafe extern "C" fn safe_c2rust_g_async_queue_invert_compare(
    mut v1: gpointer,
    mut v2: gpointer,
    mut sd: *mut SortData,
) -> gint {
    return -(*sd).func.expect("non-null function pointer")(
        v1 as gconstpointer,
        v2 as gconstpointer,
        (*sd).user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push_sorted_unlocked(
    mut queue: *mut GAsyncQueue,
    mut data: gpointer,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut sd: SortData = SortData {
        func: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    sd.func = func;
    sd.user_data = user_data;
    g_queue_insert_sorted(
        &raw mut (*queue).queue,
        data,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(gpointer, gpointer, *mut SortData) -> gint>,
            GCompareDataFunc,
        >(Some(
            safe_c2rust_g_async_queue_invert_compare
                as unsafe extern "C" fn(gpointer, gpointer, *mut SortData) -> gint,
        )),
        &raw mut sd as gpointer,
    );
    if (*queue).waiting_threads > 0 as guint {
        g_cond_signal(&raw mut (*queue).cond);
    }
}
unsafe extern "C" fn safe_c2rust_g_async_queue_pop_intern_unlocked(
    mut queue: *mut GAsyncQueue,
    mut wait: gboolean,
    mut end_time: gint64,
) -> gpointer {
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if g_queue_peek_tail_link(&raw mut (*queue).queue).is_null() && wait != 0 {
        (*queue).waiting_threads = (*queue).waiting_threads.wrapping_add(1);
        while g_queue_peek_tail_link(&raw mut (*queue).queue).is_null() {
            if end_time == -(1 as ::core::ffi::c_int) as gint64 {
                g_cond_wait(&raw mut (*queue).cond, &raw mut (*queue).mutex);
            } else if g_cond_wait_until(&raw mut (*queue).cond, &raw mut (*queue).mutex, end_time)
                == 0
            {
                break;
            }
        }
        (*queue).waiting_threads = (*queue).waiting_threads.wrapping_sub(1);
    }
    retval = g_queue_pop_tail(&raw mut (*queue).queue);
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !retval.is_null() || wait == 0 || end_time > 0 as gint64 {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gasyncqueue.c\0" as *const u8 as *const ::core::ffi::c_char,
            387 as ::core::ffi::c_int,
            G_STRFUNC,
            b"retval || !wait || end_time > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_pop(mut queue: *mut GAsyncQueue) -> gpointer {
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    retval = safe_c2rust_g_async_queue_pop_intern_unlocked(
        queue,
        TRUE,
        -(1 as ::core::ffi::c_int) as gint64,
    );
    g_mutex_unlock(&raw mut (*queue).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_pop_unlocked(
    mut queue: *mut GAsyncQueue,
) -> gpointer {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return safe_c2rust_g_async_queue_pop_intern_unlocked(
        queue,
        TRUE,
        -(1 as ::core::ffi::c_int) as gint64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_try_pop(
    mut queue: *mut GAsyncQueue,
) -> gpointer {
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    retval = safe_c2rust_g_async_queue_pop_intern_unlocked(
        queue,
        FALSE,
        -(1 as ::core::ffi::c_int) as gint64,
    );
    g_mutex_unlock(&raw mut (*queue).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_try_pop_unlocked(
    mut queue: *mut GAsyncQueue,
) -> gpointer {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return safe_c2rust_g_async_queue_pop_intern_unlocked(
        queue,
        FALSE,
        -(1 as ::core::ffi::c_int) as gint64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_timeout_pop(
    mut queue: *mut GAsyncQueue,
    mut timeout: guint64,
) -> gpointer {
    let mut end_time: gint64 = (g_get_monotonic_time() as guint64).wrapping_add(timeout) as gint64;
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    retval = safe_c2rust_g_async_queue_pop_intern_unlocked(queue, TRUE, end_time);
    g_mutex_unlock(&raw mut (*queue).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_timeout_pop_unlocked(
    mut queue: *mut GAsyncQueue,
    mut timeout: guint64,
) -> gpointer {
    let mut end_time: gint64 = (g_get_monotonic_time() as guint64).wrapping_add(timeout) as gint64;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return safe_c2rust_g_async_queue_pop_intern_unlocked(queue, TRUE, end_time);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_timed_pop(
    mut queue: *mut GAsyncQueue,
    mut end_time: *mut GTimeVal,
) -> gpointer {
    let mut m_end_time: gint64 = 0;
    let mut retval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !end_time.is_null() {
        m_end_time = g_get_monotonic_time()
            + ((*end_time).tv_sec * G_USEC_PER_SEC as gint64 + (*end_time).tv_usec as gint64
                - g_get_real_time());
    } else {
        m_end_time = -(1 as ::core::ffi::c_int) as gint64;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    retval = safe_c2rust_g_async_queue_pop_intern_unlocked(queue, TRUE, m_end_time);
    g_mutex_unlock(&raw mut (*queue).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_timed_pop_unlocked(
    mut queue: *mut GAsyncQueue,
    mut end_time: *mut GTimeVal,
) -> gpointer {
    let mut m_end_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !end_time.is_null() {
        m_end_time = g_get_monotonic_time()
            + ((*end_time).tv_sec * G_USEC_PER_SEC as gint64 + (*end_time).tv_usec as gint64
                - g_get_real_time());
    } else {
        m_end_time = -(1 as ::core::ffi::c_int) as gint64;
    }
    return safe_c2rust_g_async_queue_pop_intern_unlocked(queue, TRUE, m_end_time);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_length(mut queue: *mut GAsyncQueue) -> gint {
    let mut retval: gint = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    retval = (*queue).queue.length.wrapping_sub((*queue).waiting_threads) as gint;
    g_mutex_unlock(&raw mut (*queue).mutex);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_length_unlocked(
    mut queue: *mut GAsyncQueue,
) -> gint {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*queue).queue.length.wrapping_sub((*queue).waiting_threads) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_sort(
    mut queue: *mut GAsyncQueue,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    safe_c2rust_g_async_queue_sort_unlocked(queue, func, user_data);
    g_mutex_unlock(&raw mut (*queue).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_sort_unlocked(
    mut queue: *mut GAsyncQueue,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut sd: SortData = SortData {
        func: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    sd.func = func;
    sd.user_data = user_data;
    g_queue_sort(
        &raw mut (*queue).queue,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(gpointer, gpointer, *mut SortData) -> gint>,
            GCompareDataFunc,
        >(Some(
            safe_c2rust_g_async_queue_invert_compare
                as unsafe extern "C" fn(gpointer, gpointer, *mut SortData) -> gint,
        )),
        &raw mut sd as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_remove(
    mut queue: *mut GAsyncQueue,
    mut item: gpointer,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    ret = safe_c2rust_g_async_queue_remove_unlocked(queue, item);
    g_mutex_unlock(&raw mut (*queue).mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_remove_unlocked(
    mut queue: *mut GAsyncQueue,
    mut item: gpointer,
) -> gboolean {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_queue_remove(&raw mut (*queue).queue, item as gconstpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push_front(
    mut queue: *mut GAsyncQueue,
    mut item: gpointer,
) {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*queue).mutex);
    safe_c2rust_g_async_queue_push_front_unlocked(queue, item);
    g_mutex_unlock(&raw mut (*queue).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_async_queue_push_front_unlocked(
    mut queue: *mut GAsyncQueue,
    mut item: gpointer,
) {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_queue_push_tail(&raw mut (*queue).queue, item);
    if (*queue).waiting_threads > 0 as guint {
        g_cond_signal(&raw mut (*queue).cond);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_async_queue_get_mutex(
    mut queue: *mut GAsyncQueue,
) -> *mut GMutex {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMutex>();
    }
    return &raw mut (*queue).mutex;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_async_queue_lock\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
