extern "C" {
    pub type _GAsyncQueue;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_get_prgname() -> *const gchar;
    fn g_snprintf(string: *mut gchar, n: gulong, format: *const gchar, ...) -> gint;
    fn g_thread_unref(thread: *mut GThread);
    fn g_thread_new(name: *const gchar, func: GThreadFunc, data: gpointer) -> *mut GThread;
    fn g_thread_try_new(
        name: *const gchar,
        func: GThreadFunc,
        data: gpointer,
        error: *mut *mut GError,
    ) -> *mut GThread;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_async_queue_new() -> *mut GAsyncQueue;
    fn g_async_queue_new_full(item_free_func: GDestroyNotify) -> *mut GAsyncQueue;
    fn g_async_queue_lock(queue: *mut GAsyncQueue);
    fn g_async_queue_unlock(queue: *mut GAsyncQueue);
    fn g_async_queue_unref(queue: *mut GAsyncQueue);
    fn g_async_queue_push(queue: *mut GAsyncQueue, data: gpointer);
    fn g_async_queue_push_unlocked(queue: *mut GAsyncQueue, data: gpointer);
    fn g_async_queue_push_sorted_unlocked(
        queue: *mut GAsyncQueue,
        data: gpointer,
        func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_async_queue_pop(queue: *mut GAsyncQueue) -> gpointer;
    fn g_async_queue_pop_unlocked(queue: *mut GAsyncQueue) -> gpointer;
    fn g_async_queue_timeout_pop(queue: *mut GAsyncQueue, timeout: guint64) -> gpointer;
    fn g_async_queue_timeout_pop_unlocked(queue: *mut GAsyncQueue, timeout: guint64) -> gpointer;
    fn g_async_queue_length(queue: *mut GAsyncQueue) -> gint;
    fn g_async_queue_length_unlocked(queue: *mut GAsyncQueue) -> gint;
    fn g_async_queue_sort_unlocked(
        queue: *mut GAsyncQueue,
        func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_async_queue_remove(queue: *mut GAsyncQueue, item: gpointer) -> gboolean;
    fn g_async_queue_remove_unlocked(queue: *mut GAsyncQueue, item: gpointer) -> gboolean;
    fn g_async_queue_push_front_unlocked(queue: *mut GAsyncQueue, item: gpointer);
    fn _g_async_queue_get_mutex(queue: *mut GAsyncQueue) -> *mut GMutex;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_usleep(microseconds: gulong);
    fn __lsan_ignore_object(p: *const ::core::ffi::c_void);
}
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
pub struct _GThreadPool {
    pub func: GFunc,
    pub user_data: gpointer,
    pub exclusive: gboolean,
}
pub type GThreadPool = _GThreadPool;
pub type GRealThreadPool = _GRealThreadPool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealThreadPool {
    pub pool: GThreadPool,
    pub queue: *mut GAsyncQueue,
    pub cond: GCond,
    pub max_threads: gint,
    pub num_threads: guint,
    pub running: gboolean,
    pub immediate: gboolean,
    pub waiting: gboolean,
    pub sort_func: GCompareDataFunc,
    pub sort_user_data: gpointer,
}
pub type GAsyncQueue = _GAsyncQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpawnThreadData {
    pub pool: *mut GThreadPool,
    pub thread: *mut GThread,
    pub error: *mut GError,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_0 as gpointer;
    return ref_0;
}
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_ignore_leak(mut p: gconstpointer) {
    if !p.is_null()
        && Some(__lsan_ignore_object as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ())
            .is_some()
    {
        __lsan_ignore_object(p as *const ::core::ffi::c_void);
    }
}
static mut safe_c2rust_wakeup_thread_marker: gpointer = unsafe {
    ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                GFunc,
                gpointer,
                gint,
                gboolean,
                *mut *mut GError,
            ) -> *mut GThreadPool,
        >,
        gpointer,
    >(Some(
        safe_c2rust_g_thread_pool_new
            as unsafe extern "C" fn(
                GFunc,
                gpointer,
                gint,
                gboolean,
                *mut *mut GError,
            ) -> *mut GThreadPool,
    ))
};
static mut safe_c2rust_wakeup_thread_serial: gint = 0 as gint;
static mut safe_c2rust_unused_thread_queue: *mut GAsyncQueue =
    ::core::ptr::null::<GAsyncQueue>() as *mut GAsyncQueue;
static mut safe_c2rust_unused_threads: gint = 0 as gint;
static mut safe_c2rust_max_unused_threads: gint = 2 as gint;
static mut safe_c2rust_kill_unused_threads: gint = 0 as gint;
static mut safe_c2rust_max_idle_time: guint =
    (15 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int) as guint;
static mut safe_c2rust_spawn_thread_cond: GCond = _GCond {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_spawn_thread_queue: *mut GAsyncQueue =
    ::core::ptr::null::<GAsyncQueue>() as *mut GAsyncQueue;
unsafe extern "C" fn safe_c2rust_g_thread_pool_queue_push_unlocked(
    mut pool: *mut GRealThreadPool,
    mut data: gpointer,
) {
    if (*pool).sort_func.is_some() {
        g_async_queue_push_sorted_unlocked(
            (*pool).queue,
            data,
            (*pool).sort_func,
            (*pool).sort_user_data,
        );
    } else {
        g_async_queue_push_unlocked((*pool).queue, data);
    };
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_wait_for_new_pool() -> *mut GRealThreadPool {
    let mut pool: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut local_wakeup_thread_serial: gint = 0;
    let mut local_max_unused_threads: guint = 0;
    let mut local_max_idle_time: gint = 0;
    let mut last_wakeup_thread_serial: gint = 0;
    let mut have_relayed_thread_marker: gboolean = FALSE;
    local_max_unused_threads = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_max_unused_threads;
            safe_c2rust_max_unused_threads;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_max_unused_threads);
        gaig_temp
    }) as guint;
    local_max_idle_time = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_max_idle_time;
            safe_c2rust_max_idle_time;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_max_idle_time as *mut gint);
        gaig_temp
    });
    last_wakeup_thread_serial = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_wakeup_thread_serial;
            safe_c2rust_wakeup_thread_serial;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_wakeup_thread_serial);
        gaig_temp
    });
    loop {
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_unused_threads;
                safe_c2rust_unused_threads;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_unused_threads);
            gaig_temp
        }) as guint
            >= local_max_unused_threads
        {
            pool = ::core::ptr::null_mut::<GRealThreadPool>();
        } else if local_max_idle_time > 0 as ::core::ffi::c_int {
            pool = g_async_queue_timeout_pop(
                safe_c2rust_unused_thread_queue,
                (local_max_idle_time as ::core::ffi::c_int * 1000 as ::core::ffi::c_int) as guint64,
            ) as *mut GRealThreadPool;
        } else {
            pool = g_async_queue_pop(safe_c2rust_unused_thread_queue) as *mut GRealThreadPool;
        }
        if pool == safe_c2rust_wakeup_thread_marker as *mut GRealThreadPool {
            local_wakeup_thread_serial = ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_wakeup_thread_serial;
                    safe_c2rust_wakeup_thread_serial;
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut safe_c2rust_wakeup_thread_serial,
                );
                gaig_temp
            });
            if last_wakeup_thread_serial == local_wakeup_thread_serial {
                if have_relayed_thread_marker == 0 {
                    g_async_queue_push(
                        safe_c2rust_unused_thread_queue,
                        safe_c2rust_wakeup_thread_marker,
                    );
                    have_relayed_thread_marker = TRUE as gboolean;
                    g_usleep(100 as gulong);
                }
            } else if ({
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_kill_unused_threads;
                    -(1 as ::core::ffi::c_int);
                } else {
                };
                crate::translated::compat::atomic_xadd_seqcst(
                    &raw mut safe_c2rust_kill_unused_threads,
                    -(1 as ::core::ffi::c_int),
                )
            }) > 0 as ::core::ffi::c_int
            {
                pool = ::core::ptr::null_mut::<GRealThreadPool>();
                break;
            } else {
                local_max_unused_threads = ({
                    let mut gaig_temp: gint = 0;
                    if 0 as ::core::ffi::c_int != 0 {
                        safe_c2rust_max_unused_threads;
                        safe_c2rust_max_unused_threads;
                    } else {
                    };
                    *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                        &raw mut safe_c2rust_max_unused_threads,
                    );
                    gaig_temp
                }) as guint;
                local_max_idle_time = ({
                    let mut gaig_temp: gint = 0;
                    if 0 as ::core::ffi::c_int != 0 {
                        safe_c2rust_max_idle_time;
                        safe_c2rust_max_idle_time;
                    } else {
                    };
                    *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                        &raw mut safe_c2rust_max_idle_time as *mut gint,
                    );
                    gaig_temp
                });
                last_wakeup_thread_serial = local_wakeup_thread_serial;
                have_relayed_thread_marker = FALSE as gboolean;
            }
        }
        if !(pool == safe_c2rust_wakeup_thread_marker as *mut GRealThreadPool) {
            break;
        }
    }
    return pool;
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_wait_for_new_task(
    mut pool: *mut GRealThreadPool,
) -> gpointer {
    let mut task: gpointer = NULL;
    if (*pool).running != 0
        || (*pool).immediate == 0
            && g_async_queue_length_unlocked((*pool).queue) > 0 as ::core::ffi::c_int
    {
        if !((*pool).max_threads != -(1 as ::core::ffi::c_int)
            && (*pool).num_threads > (*pool).max_threads as guint)
        {
            if (*pool).pool.exclusive != 0 {
                task = g_async_queue_pop_unlocked((*pool).queue);
            } else {
                task = g_async_queue_timeout_pop_unlocked(
                    (*pool).queue,
                    (G_USEC_PER_SEC / 2 as ::core::ffi::c_int) as guint64,
                );
            }
        }
    }
    return task;
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_spawn_thread(mut data: gpointer) -> gpointer {
    while FALSE == 0 {
        let mut spawn_thread_data: *mut SpawnThreadData =
            ::core::ptr::null_mut::<SpawnThreadData>();
        let mut thread: *mut GThread = ::core::ptr::null_mut::<GThread>();
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut prgname: *const gchar = g_get_prgname();
        let mut name: [gchar; 16] =
            ::core::mem::transmute::<[u8; 16], [gchar; 16]>(*b"pool\0\0\0\0\0\0\0\0\0\0\0\0");
        if !prgname.is_null() {
            g_snprintf(
                &raw mut name as *mut gchar,
                ::core::mem::size_of::<[gchar; 16]>() as gulong,
                b"pool-%s\0" as *const u8 as *const gchar,
                prgname,
            );
        }
        g_async_queue_lock(safe_c2rust_spawn_thread_queue);
        spawn_thread_data =
            g_async_queue_pop_unlocked(safe_c2rust_spawn_thread_queue) as *mut SpawnThreadData;
        thread = g_thread_try_new(
            &raw mut name as *mut gchar,
            Some(
                safe_c2rust_g_thread_pool_thread_proxy
                    as unsafe extern "C" fn(gpointer) -> gpointer,
            ),
            (*spawn_thread_data).pool as gpointer,
            &raw mut error,
        );
        (*spawn_thread_data).thread = safe_c2rust_g_steal_pointer(&raw mut thread as gpointer)
            as *mut GThread as *mut GThread;
        (*spawn_thread_data).error =
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError as *mut GError;
        g_cond_broadcast(&raw mut safe_c2rust_spawn_thread_cond);
        g_async_queue_unlock(safe_c2rust_spawn_thread_queue);
    }
    return NULL;
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_thread_proxy(mut data: gpointer) -> gpointer {
    let mut pool: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    pool = data as *mut GRealThreadPool;
    g_async_queue_lock((*pool).queue);
    while FALSE == 0 {
        let mut task: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        task = safe_c2rust_g_thread_pool_wait_for_new_task(pool);
        if !task.is_null() {
            if (*pool).running != 0 || (*pool).immediate == 0 {
                g_async_queue_unlock((*pool).queue);
                (*pool).pool.func.expect("non-null function pointer")(task, (*pool).pool.user_data);
                g_async_queue_lock((*pool).queue);
            }
        } else {
            let mut free_pool: gboolean = FALSE;
            (*pool).num_threads = (*pool).num_threads.wrapping_sub(1);
            if (*pool).running == 0 {
                if (*pool).waiting == 0 {
                    if (*pool).num_threads == 0 as guint {
                        free_pool = TRUE as gboolean;
                    } else if g_async_queue_length_unlocked((*pool).queue)
                        == (*pool).num_threads.wrapping_neg() as gint
                    {
                        safe_c2rust_g_thread_pool_wakeup_and_stop_all(pool);
                    }
                } else if (*pool).immediate != 0
                    || g_async_queue_length_unlocked((*pool).queue) <= 0 as ::core::ffi::c_int
                {
                    g_cond_broadcast(&raw mut (*pool).cond);
                }
            }
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_unused_threads;
                safe_c2rust_unused_threads;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut safe_c2rust_unused_threads,
                1 as ::core::ffi::c_int,
            );
            g_async_queue_unlock((*pool).queue);
            if free_pool != 0 {
                safe_c2rust_g_thread_pool_free_internal(pool);
            }
            pool = safe_c2rust_g_thread_pool_wait_for_new_pool();
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_unused_threads;
                -(1 as ::core::ffi::c_int);
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut safe_c2rust_unused_threads,
                -(1 as ::core::ffi::c_int),
            );
            if pool.is_null() {
                break;
            }
            g_async_queue_lock((*pool).queue);
        }
    }
    return NULL;
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_start_thread(
    mut pool: *mut GRealThreadPool,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut success: gboolean = FALSE;
    if (*pool).max_threads != -(1 as ::core::ffi::c_int)
        && (*pool).num_threads >= (*pool).max_threads as guint
    {
        return TRUE;
    }
    g_async_queue_lock(safe_c2rust_unused_thread_queue);
    if g_async_queue_length_unlocked(safe_c2rust_unused_thread_queue) < 0 as ::core::ffi::c_int {
        g_async_queue_push_unlocked(safe_c2rust_unused_thread_queue, pool as gpointer);
        success = TRUE as gboolean;
    }
    g_async_queue_unlock(safe_c2rust_unused_thread_queue);
    if success == 0 {
        let mut prgname: *const gchar = g_get_prgname();
        let mut name: [gchar; 16] =
            ::core::mem::transmute::<[u8; 16], [gchar; 16]>(*b"pool\0\0\0\0\0\0\0\0\0\0\0\0");
        let mut thread: *mut GThread = ::core::ptr::null_mut::<GThread>();
        if !prgname.is_null() {
            g_snprintf(
                &raw mut name as *mut gchar,
                ::core::mem::size_of::<[gchar; 16]>() as gulong,
                b"pool-%s\0" as *const u8 as *const gchar,
                prgname,
            );
        }
        if (*pool).pool.exclusive != 0 {
            thread = g_thread_try_new(
                &raw mut name as *mut gchar,
                Some(
                    safe_c2rust_g_thread_pool_thread_proxy
                        as unsafe extern "C" fn(gpointer) -> gpointer,
                ),
                pool as gpointer,
                error,
            );
        } else {
            let mut spawn_thread_data: SpawnThreadData = SpawnThreadData {
                pool: pool as *mut GThreadPool,
                thread: ::core::ptr::null_mut::<GThread>(),
                error: ::core::ptr::null_mut::<GError>(),
            };
            g_async_queue_lock(safe_c2rust_spawn_thread_queue);
            g_async_queue_push_unlocked(
                safe_c2rust_spawn_thread_queue,
                &raw mut spawn_thread_data as gpointer,
            );
            while spawn_thread_data.thread.is_null() && spawn_thread_data.error.is_null() {
                g_cond_wait(
                    &raw mut safe_c2rust_spawn_thread_cond,
                    _g_async_queue_get_mutex(safe_c2rust_spawn_thread_queue),
                );
            }
            thread = spawn_thread_data.thread;
            if thread.is_null() {
                g_propagate_error(
                    error,
                    safe_c2rust_g_steal_pointer(&raw mut spawn_thread_data.error as gpointer)
                        as *mut GError,
                );
            }
            g_async_queue_unlock(safe_c2rust_spawn_thread_queue);
        }
        if thread.is_null() {
            return FALSE;
        }
        g_thread_unref(thread);
    }
    (*pool).num_threads = (*pool).num_threads.wrapping_add(1);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_new(
    mut func: GFunc,
    mut user_data: gpointer,
    mut max_threads: gint,
    mut exclusive: gboolean,
    mut error: *mut *mut GError,
) -> *mut GThreadPool {
    return safe_c2rust_g_thread_pool_new_full(
        func,
        user_data,
        None,
        max_threads,
        exclusive,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_new_full(
    mut func: GFunc,
    mut user_data: gpointer,
    mut item_free_func: GDestroyNotify,
    mut max_threads: gint,
    mut exclusive: gboolean,
    mut error: *mut *mut GError,
) -> *mut GThreadPool {
    let mut retval: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    static mut safe_c2rust_g__init_lock: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GThreadPool>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if exclusive == 0 || max_threads != -(1 as ::core::ffi::c_int) {
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
            b"!exclusive || max_threads != -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GThreadPool>();
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if max_threads >= -(1 as ::core::ffi::c_int) {
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
            b"max_threads >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GThreadPool>();
    }
    retval = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GRealThreadPool>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GRealThreadPool;
    (*retval).pool.func = func;
    (*retval).pool.user_data = user_data;
    (*retval).pool.exclusive = exclusive;
    (*retval).queue = g_async_queue_new_full(item_free_func);
    g_cond_init(&raw mut (*retval).cond);
    (*retval).max_threads = max_threads;
    (*retval).num_threads = 0 as guint;
    (*retval).running = TRUE as gboolean;
    (*retval).immediate = FALSE as gboolean;
    (*retval).waiting = FALSE as gboolean;
    (*retval).sort_func = None;
    (*retval).sort_user_data = NULL as gpointer;
    g_mutex_lock(&raw mut safe_c2rust_g__init_lock);
    if safe_c2rust_unused_thread_queue.is_null() {
        safe_c2rust_unused_thread_queue = g_async_queue_new();
    }
    if exclusive == 0 && safe_c2rust_spawn_thread_queue.is_null() {
        let mut pool_spawner: *mut GThread = ::core::ptr::null_mut::<GThread>();
        safe_c2rust_spawn_thread_queue = g_async_queue_new();
        g_cond_init(&raw mut safe_c2rust_spawn_thread_cond);
        pool_spawner = g_thread_new(
            b"pool-spawner\0" as *const u8 as *const gchar,
            Some(
                safe_c2rust_g_thread_pool_spawn_thread
                    as unsafe extern "C" fn(gpointer) -> gpointer,
            ),
            NULL,
        );
        safe_c2rust_g_ignore_leak(pool_spawner as gconstpointer);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__init_lock);
    if (*retval).pool.exclusive != 0 {
        g_async_queue_lock((*retval).queue);
        while (*retval).num_threads < (*retval).max_threads as guint {
            let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
            if !(safe_c2rust_g_thread_pool_start_thread(retval, &raw mut local_error) == 0) {
                continue;
            }
            g_propagate_error(error, local_error);
            break;
        }
        g_async_queue_unlock((*retval).queue);
    }
    return retval as *mut GThreadPool;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_push(
    mut pool: *mut GThreadPool,
    mut data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut result: gboolean = 0;
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    result = TRUE as gboolean;
    g_async_queue_lock((*real).queue);
    if g_async_queue_length_unlocked((*real).queue) >= 0 as ::core::ffi::c_int {
        let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
        if safe_c2rust_g_thread_pool_start_thread(real, &raw mut local_error) == 0 {
            g_propagate_error(error, local_error);
            result = FALSE as gboolean;
        }
    }
    safe_c2rust_g_thread_pool_queue_push_unlocked(real, data);
    g_async_queue_unlock((*real).queue);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_set_max_threads(
    mut pool: *mut GThreadPool,
    mut max_threads: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut to_start: gint = 0;
    let mut result: gboolean = 0;
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*real).pool.exclusive == 0 || max_threads != -(1 as ::core::ffi::c_int) {
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
            b"!real->pool.exclusive || max_threads != -1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if max_threads >= -(1 as ::core::ffi::c_int) {
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
            b"max_threads >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    result = TRUE as gboolean;
    g_async_queue_lock((*real).queue);
    (*real).max_threads = max_threads;
    if (*pool).exclusive != 0 {
        to_start = ((*real).max_threads as guint).wrapping_sub((*real).num_threads) as gint;
    } else {
        to_start = g_async_queue_length_unlocked((*real).queue);
    }
    while to_start > 0 as ::core::ffi::c_int {
        let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
        if safe_c2rust_g_thread_pool_start_thread(real, &raw mut local_error) == 0 {
            g_propagate_error(error, local_error);
            result = FALSE as gboolean;
            break;
        } else {
            to_start -= 1;
        }
    }
    g_async_queue_unlock((*real).queue);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_get_max_threads(
    mut pool: *mut GThreadPool,
) -> gint {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut retval: gint = 0;
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    g_async_queue_lock((*real).queue);
    retval = (*real).max_threads;
    g_async_queue_unlock((*real).queue);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_get_num_threads(
    mut pool: *mut GThreadPool,
) -> guint {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut retval: guint = 0;
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_async_queue_lock((*real).queue);
    retval = (*real).num_threads;
    g_async_queue_unlock((*real).queue);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_unprocessed(
    mut pool: *mut GThreadPool,
) -> guint {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    let mut unprocessed: gint = 0;
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    unprocessed = g_async_queue_length((*real).queue);
    return (if unprocessed > 0 as ::core::ffi::c_int {
        unprocessed as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_free(
    mut pool: *mut GThreadPool,
    mut immediate: gboolean,
    mut wait_: gboolean,
) {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if immediate != 0
            || (*real).max_threads != 0 as ::core::ffi::c_int
            || g_async_queue_length((*real).queue) == 0 as ::core::ffi::c_int
        {
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
            b"immediate || real->max_threads != 0 || g_async_queue_length (real->queue) == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_queue_lock((*real).queue);
    (*real).running = FALSE as gboolean;
    (*real).immediate = immediate;
    (*real).waiting = wait_;
    if wait_ != 0 {
        while g_async_queue_length_unlocked((*real).queue)
            != (*real).num_threads.wrapping_neg() as gint
            && !(immediate != 0 && (*real).num_threads == 0 as guint)
        {
            g_cond_wait(
                &raw mut (*real).cond,
                _g_async_queue_get_mutex((*real).queue),
            );
        }
    }
    if immediate != 0
        || g_async_queue_length_unlocked((*real).queue)
            == (*real).num_threads.wrapping_neg() as gint
    {
        if (*real).num_threads == 0 as guint {
            g_async_queue_unlock((*real).queue);
            safe_c2rust_g_thread_pool_free_internal(real);
            return;
        }
        safe_c2rust_g_thread_pool_wakeup_and_stop_all(real);
    }
    (*real).waiting = FALSE as gboolean;
    g_async_queue_unlock((*real).queue);
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_free_internal(mut pool: *mut GRealThreadPool) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !pool.is_null() {
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
            b"pool\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*pool).running == 0 as ::core::ffi::c_int {
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
            b"pool->running == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if (*pool).num_threads == 0 as guint {
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
            b"pool->num_threads == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_queue_remove((*pool).queue, 1 as ::core::ffi::c_int as gulong as gpointer);
    g_async_queue_unref((*pool).queue);
    g_cond_clear(&raw mut (*pool).cond);
    g_free(pool as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_thread_pool_wakeup_and_stop_all(mut pool: *mut GRealThreadPool) {
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !pool.is_null() {
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
            b"pool\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if (*pool).running == 0 as ::core::ffi::c_int {
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
            b"pool->running == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*pool).num_threads != 0 as guint {
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
            b"pool->num_threads != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*pool).immediate = TRUE as gboolean;
    i = 0 as guint;
    while i < (*pool).num_threads {
        g_async_queue_push_unlocked((*pool).queue, 1 as ::core::ffi::c_int as gulong as gpointer);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_set_max_unused_threads(mut max_threads: gint) {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if max_threads >= -(1 as ::core::ffi::c_int) {
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
            b"max_threads >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut gais_temp: gint = max_threads;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_max_unused_threads;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut safe_c2rust_max_unused_threads,
        *&raw mut gais_temp,
    );
    if max_threads != -(1 as ::core::ffi::c_int) {
        max_threads -= ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_unused_threads;
                safe_c2rust_unused_threads;
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_unused_threads);
            gaig_temp
        });
        if max_threads < 0 as ::core::ffi::c_int {
            let mut gais_temp_0: gint = -max_threads;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_kill_unused_threads;
                -(max_threads as ::core::ffi::c_int);
            } else {
            };
            crate::translated::compat::atomic_store_seqcst(
                &raw mut safe_c2rust_kill_unused_threads,
                *&raw mut gais_temp_0,
            );
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_wakeup_thread_serial;
                safe_c2rust_wakeup_thread_serial;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut safe_c2rust_wakeup_thread_serial,
                1 as ::core::ffi::c_int,
            );
            g_async_queue_lock(safe_c2rust_unused_thread_queue);
            loop {
                g_async_queue_push_unlocked(
                    safe_c2rust_unused_thread_queue,
                    safe_c2rust_wakeup_thread_marker,
                );
                max_threads += 1;
                if !(max_threads != 0) {
                    break;
                }
            }
            g_async_queue_unlock(safe_c2rust_unused_thread_queue);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_get_max_unused_threads() -> gint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_max_unused_threads;
            safe_c2rust_max_unused_threads;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_max_unused_threads);
        gaig_temp
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_get_num_unused_threads() -> guint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_unused_threads;
            safe_c2rust_unused_threads;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_unused_threads);
        gaig_temp
    }) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_stop_unused_threads() {
    let mut oldval: guint = 0;
    oldval = safe_c2rust_g_thread_pool_get_max_unused_threads() as guint;
    safe_c2rust_g_thread_pool_set_max_unused_threads(0 as gint);
    safe_c2rust_g_thread_pool_set_max_unused_threads(oldval as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_set_sort_function(
    mut pool: *mut GThreadPool,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut real: *mut GRealThreadPool = ::core::ptr::null_mut::<GRealThreadPool>();
    real = pool as *mut GRealThreadPool;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !real.is_null() {
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
            b"real\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*real).running != 0 {
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
            b"real->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_queue_lock((*real).queue);
    (*real).sort_func = func;
    (*real).sort_user_data = user_data;
    if func.is_some() {
        g_async_queue_sort_unlocked((*real).queue, (*real).sort_func, (*real).sort_user_data);
    }
    g_async_queue_unlock((*real).queue);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_move_to_front(
    mut pool: *mut GThreadPool,
    mut data: gpointer,
) -> gboolean {
    let mut real: *mut GRealThreadPool = pool as *mut GRealThreadPool;
    let mut found: gboolean = 0;
    g_async_queue_lock((*real).queue);
    found = g_async_queue_remove_unlocked((*real).queue, data);
    if found != 0 {
        g_async_queue_push_front_unlocked((*real).queue, data);
    }
    g_async_queue_unlock((*real).queue);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_set_max_idle_time(mut interval: guint) {
    let mut i: guint = 0;
    let mut gais_temp: gint = interval as gint;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_max_idle_time;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut safe_c2rust_max_idle_time as *mut gint,
        *&raw mut gais_temp,
    );
    i = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_unused_threads;
            safe_c2rust_unused_threads;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_unused_threads);
        gaig_temp
    }) as guint;
    if i > 0 as guint {
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_wakeup_thread_serial;
            safe_c2rust_wakeup_thread_serial;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut safe_c2rust_wakeup_thread_serial,
            1 as ::core::ffi::c_int,
        );
        g_async_queue_lock(safe_c2rust_unused_thread_queue);
        loop {
            g_async_queue_push_unlocked(
                safe_c2rust_unused_thread_queue,
                safe_c2rust_wakeup_thread_marker,
            );
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        g_async_queue_unlock(safe_c2rust_unused_thread_queue);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_thread_pool_get_max_idle_time() -> guint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_max_idle_time;
            safe_c2rust_max_idle_time;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut safe_c2rust_max_idle_time as *mut gint);
        gaig_temp
    }) as guint;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_thread_pool_free_internal\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
