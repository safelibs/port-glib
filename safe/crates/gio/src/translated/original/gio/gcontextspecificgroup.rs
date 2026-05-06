extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GWakeup;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_default() -> *mut GMainContext;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_main_context_invoke(context: *mut GMainContext, function: GSourceFunc, data: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_queue_init(queue: *mut GQueue);
    fn g_queue_clear(queue: *mut GQueue);
    fn g_queue_is_empty(queue: *mut GQueue) -> gboolean;
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    fn g_queue_remove(queue: *mut GQueue, data: gconstpointer) -> gboolean;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn glib__private__() -> *const GLibPrivateVTable;
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
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
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GContextSpecificGroup {
    pub table: *mut GHashTable,
    pub lock: GMutex,
    pub cond: GCond,
    pub requested_state: gboolean,
    pub requested_func: GCallback,
    pub effective_state: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GContextSpecificSource {
    pub source: GSource,
    pub lock: GMutex,
    pub instance: gpointer,
    pub pending: GQueue,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_context_specific_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut css: *mut GContextSpecificSource = source as *mut GContextSpecificSource;
    let mut signal_id: guint = 0;
    g_mutex_lock(&raw mut (*css).lock);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_queue_is_empty(&raw mut (*css).pending) == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcontextspecificgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!g_queue_is_empty (&css->pending)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    signal_id = g_queue_pop_head(&raw mut (*css).pending) as gulong as guint;
    if g_queue_is_empty(&raw mut (*css).pending) != 0 {
        g_source_set_ready_time(source, -(1 as ::core::ffi::c_int) as gint64);
    }
    g_mutex_unlock(&raw mut (*css).lock);
    g_signal_emit((*css).instance, signal_id, 0 as GQuark);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_context_specific_source_finalize(mut source: *mut GSource) {
    let mut css: *mut GContextSpecificSource = source as *mut GContextSpecificSource;
    g_mutex_clear(&raw mut (*css).lock);
    g_queue_clear(&raw mut (*css).pending);
}
unsafe extern "C" fn safe_c2rust_g_context_specific_source_new(
    mut name: *const gchar,
    mut instance: gpointer,
) -> *mut GContextSpecificSource {
    static mut safe_c2rust_source_funcs: GSourceFuncs = unsafe {
        _GSourceFuncs {
            prepare: None,
            check: None,
            dispatch: Some(
                safe_c2rust_g_context_specific_source_dispatch
                    as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
            ),
            finalize: Some(
                safe_c2rust_g_context_specific_source_finalize
                    as unsafe extern "C" fn(*mut GSource) -> (),
            ),
            closure_callback: None,
            closure_marshal: None,
        }
    };
    let mut css: *mut GContextSpecificSource = ::core::ptr::null_mut::<GContextSpecificSource>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source = g_source_new(
        &raw mut safe_c2rust_source_funcs,
        ::core::mem::size_of::<GContextSpecificSource>() as guint,
    );
    css = source as *mut GContextSpecificSource;
    g_source_set_name(source, name as *const ::core::ffi::c_char);
    g_mutex_init(&raw mut (*css).lock);
    g_queue_init(&raw mut (*css).pending);
    (*css).instance = instance;
    return css;
}
unsafe extern "C" fn safe_c2rust_g_context_specific_group_change_state(
    mut user_data: gpointer,
) -> gboolean {
    let mut group: *mut GContextSpecificGroup = user_data as *mut GContextSpecificGroup;
    g_mutex_lock(&raw mut (*group).lock);
    if (*group).requested_state != (*group).effective_state {
        Some((*group).requested_func.expect("non-null function pointer"))
            .expect("non-null function pointer")();
        (*group).effective_state = (*group).requested_state;
        (*group).requested_func = None;
        g_cond_broadcast(&raw mut (*group).cond);
    }
    g_mutex_unlock(&raw mut (*group).lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_context_specific_group_request_state(
    mut group: *mut GContextSpecificGroup,
    mut requested_state: gboolean,
    mut requested_func: GCallback,
) {
    if requested_state != (*group).requested_state {
        if (*group).effective_state != (*group).requested_state {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if (*group).effective_state == requested_state {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcontextspecificgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"group->effective_state == requested_state\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*group).requested_state = requested_state;
            (*group).requested_func = None;
        } else {
            (*group).requested_state = requested_state;
            (*group).requested_func = requested_func;
            g_main_context_invoke(
                (*glib__private__())
                    .g_get_worker_context
                    .expect("non-null function pointer")(),
                Some(
                    safe_c2rust_g_context_specific_group_change_state
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
                group as gpointer,
            );
        }
    }
    while (*group).requested_state != (*group).effective_state {
        g_cond_wait(&raw mut (*group).cond, &raw mut (*group).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_context_specific_group_get(
    mut group: *mut GContextSpecificGroup,
    mut type_0: GType,
    mut context_offset: goffset,
    mut start_func: GCallback,
) -> gpointer {
    let mut css: *mut GContextSpecificSource = ::core::ptr::null_mut::<GContextSpecificSource>();
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    context = g_main_context_get_thread_default();
    if context.is_null() {
        context = g_main_context_default();
    }
    g_mutex_lock(&raw mut (*group).lock);
    if (*group).table.is_null() {
        (*group).table = g_hash_table_new(None, None);
    }
    css = g_hash_table_lookup((*group).table, context as gconstpointer)
        as *mut GContextSpecificSource;
    if css.is_null() {
        let mut instance: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        instance = g_object_new(type_0, ::core::ptr::null::<gchar>());
        css = safe_c2rust_g_context_specific_source_new(g_type_name(type_0), instance);
        let ref mut fresh0 = *((instance as *mut guint8).offset(context_offset as isize) as gpointer
            as *mut *mut GMainContext);
        *fresh0 = g_main_context_ref(context);
        g_source_attach(css as *mut GSource, context);
        g_hash_table_insert((*group).table, context as gpointer, css as gpointer);
    } else {
        g_object_ref((*css).instance);
    }
    if start_func.is_some() {
        safe_c2rust_g_context_specific_group_request_state(group, TRUE, start_func);
    }
    g_mutex_unlock(&raw mut (*group).lock);
    return (*css).instance;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_context_specific_group_remove(
    mut group: *mut GContextSpecificGroup,
    mut context: *mut GMainContext,
    mut instance: gpointer,
    mut stop_func: GCallback,
) {
    let mut css: *mut GContextSpecificSource = ::core::ptr::null_mut::<GContextSpecificSource>();
    if context.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Removing %s with NULL context.  This object was probably directly constructed from a dynamic language.  This is not a valid use of the API.\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(instance as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    g_mutex_lock(&raw mut (*group).lock);
    css = g_hash_table_lookup((*group).table, context as gconstpointer)
        as *mut GContextSpecificSource;
    g_hash_table_remove((*group).table, context as gconstpointer);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !css.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcontextspecificgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            220 as ::core::ffi::c_int,
            G_STRFUNC,
            b"css\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if stop_func.is_some() && g_hash_table_size((*group).table) == 0 as guint {
        safe_c2rust_g_context_specific_group_request_state(group, FALSE, stop_func);
    }
    g_mutex_unlock(&raw mut (*group).lock);
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*css).instance == instance {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcontextspecificgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            228 as ::core::ffi::c_int,
            G_STRFUNC,
            b"css->instance == instance\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_source_destroy(css as *mut GSource);
    g_source_unref(css as *mut GSource);
    g_main_context_unref(context);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_context_specific_group_emit(
    mut group: *mut GContextSpecificGroup,
    mut signal_id: guint,
) {
    g_mutex_lock(&raw mut (*group).lock);
    if !(*group).table.is_null() {
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ptr = signal_id as gulong as gpointer;
        g_hash_table_iter_init(&raw mut iter, (*group).table);
        while g_hash_table_iter_next(
            &raw mut iter,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut value,
        ) != 0
        {
            let mut css: *mut GContextSpecificSource = value as *mut GContextSpecificSource;
            g_mutex_lock(&raw mut (*css).lock);
            g_queue_remove(&raw mut (*css).pending, ptr as gconstpointer);
            g_queue_push_tail(&raw mut (*css).pending, ptr);
            g_source_set_ready_time(css as *mut GSource, 0 as gint64);
            g_mutex_unlock(&raw mut (*css).lock);
        }
    }
    g_mutex_unlock(&raw mut (*group).lock);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
