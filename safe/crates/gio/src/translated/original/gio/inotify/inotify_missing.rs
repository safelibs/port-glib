extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GWakeup;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_list_free(list: *mut GList);
    fn g_list_free_1(list: *mut GList);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
    fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn _ip_start_watching(sub: *mut inotify_sub) -> gboolean;
    fn glib__private__() -> *const GLibPrivateVTable;
    static mut safe_c2rust_g__inotify_lock_lock: GMutex;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
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
pub struct inotify_sub {
    pub dirname: *mut gchar,
    pub filename: *mut gchar,
    pub cancelled: gboolean,
    pub user_data: gpointer,
    pub pair_moves: gboolean,
    pub hardlinks: gboolean,
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
pub const SCAN_MISSING_TIME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut safe_c2rust_im_debug_enabled: gboolean = FALSE;
static mut safe_c2rust_missing_sub_list: *mut GList = ::core::ptr::null::<GList>() as *mut GList;
static mut safe_c2rust_scan_missing_running: gboolean = FALSE;
static mut safe_c2rust_missing_cb: Option<unsafe extern "C" fn(*mut inotify_sub) -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__im_startup(
    mut callback: Option<unsafe extern "C" fn(*mut inotify_sub) -> ()>,
) {
    static mut safe_c2rust_initialized: gboolean = FALSE;
    if safe_c2rust_initialized == 0 {
        safe_c2rust_missing_cb = callback;
        safe_c2rust_initialized = TRUE as gboolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__im_add(mut sub: *mut inotify_sub) {
    if !g_list_find(safe_c2rust_missing_sub_list, sub as gconstpointer).is_null() {
        if safe_c2rust_im_debug_enabled != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"asked to add %s to missing list but it's already on the list!\n\0" as *const u8
                    as *const gchar,
                (*sub).dirname,
            );
        }
        return;
    }
    if safe_c2rust_im_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"adding %s to missing list\n\0" as *const u8 as *const gchar,
            (*sub).dirname,
        );
    }
    safe_c2rust_missing_sub_list = g_list_prepend(safe_c2rust_missing_sub_list, sub as gpointer);
    if safe_c2rust_scan_missing_running == 0 {
        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        safe_c2rust_scan_missing_running = TRUE as gboolean;
        source = g_timeout_source_new_seconds(SCAN_MISSING_TIME as guint);
        g_source_set_callback(
            source,
            Some(safe_c2rust_im_scan_missing as unsafe extern "C" fn(gpointer) -> gboolean),
            NULL,
            None,
        );
        g_source_attach(
            source,
            (*glib__private__())
                .g_get_worker_context
                .expect("non-null function pointer")(),
        );
        g_source_unref(source);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__im_rm(mut sub: *mut inotify_sub) {
    let mut link: *mut GList = ::core::ptr::null_mut::<GList>();
    link = g_list_find(safe_c2rust_missing_sub_list, sub as gconstpointer);
    if link.is_null() {
        if safe_c2rust_im_debug_enabled != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"asked to remove %s from missing list but it isn't on the list!\n\0" as *const u8
                    as *const gchar,
                (*sub).dirname,
            );
        }
        return;
    }
    if safe_c2rust_im_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"removing %s from missing list\n\0" as *const u8 as *const gchar,
            (*sub).dirname,
        );
    }
    safe_c2rust_missing_sub_list = g_list_remove_link(safe_c2rust_missing_sub_list, link);
    g_list_free_1(link);
}
unsafe extern "C" fn safe_c2rust_im_scan_missing(mut user_data: gpointer) -> gboolean {
    let mut nolonger_missing: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    g_mutex_lock(&raw mut safe_c2rust_g__inotify_lock_lock);
    if safe_c2rust_im_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"scanning missing list with %d items\n\0" as *const u8 as *const gchar,
            g_list_length(safe_c2rust_missing_sub_list),
        );
    }
    l = safe_c2rust_missing_sub_list;
    while !l.is_null() {
        let mut sub: *mut inotify_sub = (*l).data as *mut inotify_sub;
        let mut not_m: gboolean = FALSE;
        if safe_c2rust_im_debug_enabled != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"checking %p\n\0" as *const u8 as *const gchar,
                sub,
            );
        }
        if ({
            let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
            if !sub.is_null() {
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
                b"../original/gio/inotify/inotify-missing.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int,
                G_STRFUNC,
                b"sub\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
            if !(*sub).dirname.is_null() {
                _g_boolean_var_9 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_9 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_9
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/gio/inotify/inotify-missing.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                G_STRFUNC,
                b"sub->dirname\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        not_m = _ip_start_watching(sub);
        if not_m != 0 {
            safe_c2rust_missing_cb.expect("non-null function pointer")(sub);
            if safe_c2rust_im_debug_enabled != 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"removed %s from missing list\n\0" as *const u8 as *const gchar,
                    (*sub).dirname,
                );
            }
            nolonger_missing = g_list_prepend(nolonger_missing, l as gpointer);
        }
        l = (*l).next;
    }
    l = nolonger_missing;
    while !l.is_null() {
        let mut llink: *mut GList = (*l).data as *mut GList;
        safe_c2rust_missing_sub_list = g_list_remove_link(safe_c2rust_missing_sub_list, llink);
        g_list_free_1(llink);
        l = (*l).next;
    }
    g_list_free(nolonger_missing);
    if safe_c2rust_missing_sub_list.is_null() {
        safe_c2rust_scan_missing_running = FALSE as gboolean;
        g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
        return FALSE;
    } else {
        g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
        return TRUE;
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"im_scan_missing\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
