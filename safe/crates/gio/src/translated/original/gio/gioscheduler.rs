extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GTask;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOSchedulerJob {
    pub active_link: *mut GList,
    pub task: *mut GTask,
    pub job_func: GIOSchedulerJobFunc,
    pub data: gpointer,
    pub destroy_notify: GDestroyNotify,
    pub cancellable: *mut GCancellable,
    pub cancellable_id: gulong,
    pub context: *mut GMainContext,
}
pub type GIOSchedulerJobFunc =
    Option<unsafe extern "C" fn(*mut GIOSchedulerJob, *mut GCancellable, gpointer) -> gboolean>;
pub type GIOSchedulerJob = _GIOSchedulerJob;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MainLoopProxy {
    pub func: GSourceFunc,
    pub ret_val: gboolean,
    pub data: gpointer,
    pub notify: GDestroyNotify,
    pub ack_lock: GMutex,
    pub ack_condition: GCond,
    pub ack: gboolean,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut safe_c2rust_g__active_jobs_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_active_jobs: *mut GList = ::core::ptr::null::<GList>() as *mut GList;
unsafe extern "C" fn safe_c2rust_g_io_job_free(mut job: *mut GIOSchedulerJob) {
    if (*job).destroy_notify.is_some() {
        (*job).destroy_notify.expect("non-null function pointer")((*job).data);
    }
    g_mutex_lock(&raw mut safe_c2rust_g__active_jobs_lock);
    safe_c2rust_active_jobs = g_list_delete_link(safe_c2rust_active_jobs, (*job).active_link);
    g_mutex_unlock(&raw mut safe_c2rust_g__active_jobs_lock);
    if !(*job).cancellable.is_null() {
        g_object_unref((*job).cancellable as gpointer);
    }
    g_main_context_unref((*job).context);
    g_slice_free1(
        ::core::mem::size_of::<GIOSchedulerJob>() as gsize,
        job as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_io_job_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut job: *mut GIOSchedulerJob = task_data as *mut GIOSchedulerJob;
    let mut result: gboolean = 0;
    if !(*job).cancellable.is_null() {
        g_cancellable_push_current((*job).cancellable);
    }
    loop {
        result = (*job).job_func.expect("non-null function pointer")(
            job,
            (*job).cancellable,
            (*job).data,
        );
        if !(result != 0) {
            break;
        }
    }
    if !(*job).cancellable.is_null() {
        g_cancellable_pop_current((*job).cancellable);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_scheduler_push_job(
    mut job_func: GIOSchedulerJobFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
) {
    let mut job: *mut GIOSchedulerJob = ::core::ptr::null_mut::<GIOSchedulerJob>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if job_func.is_some() {
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
            b"job_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    job = ({
        let mut __s: gsize = ::core::mem::size_of::<GIOSchedulerJob>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GIOSchedulerJob;
    (*job).job_func = job_func;
    (*job).data = user_data;
    (*job).destroy_notify = notify;
    if !cancellable.is_null() {
        (*job).cancellable =
            g_object_ref(cancellable as gpointer) as *mut GCancellable as *mut GCancellable;
    }
    (*job).context = g_main_context_ref_thread_default();
    g_mutex_lock(&raw mut safe_c2rust_g__active_jobs_lock);
    safe_c2rust_active_jobs = g_list_prepend(safe_c2rust_active_jobs, job as gpointer);
    (*job).active_link = safe_c2rust_active_jobs;
    g_mutex_unlock(&raw mut safe_c2rust_g__active_jobs_lock);
    task = g_task_new(NULL, cancellable, None, NULL);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    GIOSchedulerJobFunc,
                    gpointer,
                    GDestroyNotify,
                    gint,
                    *mut GCancellable,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_io_scheduler_push_job
                as unsafe extern "C" fn(
                    GIOSchedulerJobFunc,
                    gpointer,
                    GDestroyNotify,
                    gint,
                    *mut GCancellable,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_io_scheduler_push_job\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        job as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIOSchedulerJob) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_io_job_free as unsafe extern "C" fn(*mut GIOSchedulerJob) -> (),
        )),
    );
    g_task_set_priority(task, io_priority);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_io_job_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_scheduler_cancel_all_jobs() {
    let mut cancellable_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    g_mutex_lock(&raw mut safe_c2rust_g__active_jobs_lock);
    cancellable_list = ::core::ptr::null_mut::<GList>();
    l = safe_c2rust_active_jobs;
    while !l.is_null() {
        let mut job: *mut GIOSchedulerJob = (*l).data as *mut GIOSchedulerJob;
        if !(*job).cancellable.is_null() {
            cancellable_list = g_list_prepend(
                cancellable_list,
                g_object_ref((*job).cancellable as gpointer) as *mut GCancellable as gpointer,
            );
        }
        l = (*l).next;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__active_jobs_lock);
    l = cancellable_list;
    while !l.is_null() {
        let mut c: *mut GCancellable = (*l).data as *mut GCancellable;
        g_cancellable_cancel(c);
        g_object_unref(c as gpointer);
        l = (*l).next;
    }
    g_list_free(cancellable_list);
}
unsafe extern "C" fn safe_c2rust_mainloop_proxy_func(mut data: gpointer) -> gboolean {
    let mut proxy: *mut MainLoopProxy = data as *mut MainLoopProxy;
    (*proxy).ret_val = (*proxy).func.expect("non-null function pointer")((*proxy).data);
    if (*proxy).notify.is_some() {
        (*proxy).notify.expect("non-null function pointer")((*proxy).data);
    }
    g_mutex_lock(&raw mut (*proxy).ack_lock);
    (*proxy).ack = TRUE as gboolean;
    g_cond_signal(&raw mut (*proxy).ack_condition);
    g_mutex_unlock(&raw mut (*proxy).ack_lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_mainloop_proxy_free(mut proxy: *mut MainLoopProxy) {
    g_mutex_clear(&raw mut (*proxy).ack_lock);
    g_cond_clear(&raw mut (*proxy).ack_condition);
    g_free(proxy as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_scheduler_job_send_to_mainloop(
    mut job: *mut GIOSchedulerJob,
    mut func: GSourceFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) -> gboolean {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut proxy: *mut MainLoopProxy = ::core::ptr::null_mut::<MainLoopProxy>();
    let mut ret_val: gboolean = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !job.is_null() {
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
            b"job != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
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
        return 0 as gboolean;
    }
    proxy = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<MainLoopProxy>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut MainLoopProxy;
    (*proxy).func = func;
    (*proxy).data = user_data;
    (*proxy).notify = notify;
    g_mutex_init(&raw mut (*proxy).ack_lock);
    g_cond_init(&raw mut (*proxy).ack_condition);
    g_mutex_lock(&raw mut (*proxy).ack_lock);
    source = g_idle_source_new();
    g_source_set_priority(source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        source,
        Some(safe_c2rust_mainloop_proxy_func as unsafe extern "C" fn(gpointer) -> gboolean),
        proxy as gpointer,
        None,
    );
    g_source_set_static_name(
        source,
        b"[gio] mainloop_proxy_func\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(source, (*job).context);
    g_source_unref(source);
    while (*proxy).ack == 0 {
        g_cond_wait(&raw mut (*proxy).ack_condition, &raw mut (*proxy).ack_lock);
    }
    g_mutex_unlock(&raw mut (*proxy).ack_lock);
    ret_val = (*proxy).ret_val;
    safe_c2rust_mainloop_proxy_free(proxy);
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_scheduler_job_send_to_mainloop_async(
    mut job: *mut GIOSchedulerJob,
    mut func: GSourceFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut proxy: *mut MainLoopProxy = ::core::ptr::null_mut::<MainLoopProxy>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !job.is_null() {
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
            b"job != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    proxy = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<MainLoopProxy>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut MainLoopProxy;
    (*proxy).func = func;
    (*proxy).data = user_data;
    (*proxy).notify = notify;
    g_mutex_init(&raw mut (*proxy).ack_lock);
    g_cond_init(&raw mut (*proxy).ack_condition);
    source = g_idle_source_new();
    g_source_set_priority(source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        source,
        Some(safe_c2rust_mainloop_proxy_func as unsafe extern "C" fn(gpointer) -> gboolean),
        proxy as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut MainLoopProxy) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_mainloop_proxy_free as unsafe extern "C" fn(*mut MainLoopProxy) -> (),
        )),
    );
    g_source_set_static_name(
        source,
        b"[gio] mainloop_proxy_func\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(source, (*job).context);
    g_source_unref(source);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
