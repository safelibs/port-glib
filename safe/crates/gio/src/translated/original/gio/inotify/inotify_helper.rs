extern "C" {
    pub type _GFile;
    pub type _GFileMonitorSource;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_free(mem: gpointer);
    fn g_get_monotonic_time() -> gint64;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_object_unref(object: gpointer);
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_monitor_source_handle_event(
        fms: *mut GFileMonitorSource,
        event_type: GFileMonitorEvent,
        child: *const gchar,
        rename_to: *const gchar,
        other: *mut GFile,
        event_time: gint64,
    ) -> gboolean;
    fn _im_startup(missing_cb: Option<unsafe extern "C" fn(*mut inotify_sub) -> ()>);
    fn _im_add(sub: *mut inotify_sub);
    fn _im_rm(sub: *mut inotify_sub);
    fn _ip_startup(
        event_cb: Option<
            unsafe extern "C" fn(*mut ik_event_t, *mut inotify_sub, gboolean) -> gboolean,
        >,
    ) -> gboolean;
    fn _ip_start_watching(sub: *mut inotify_sub) -> gboolean;
    fn _ip_stop_watching(sub: *mut inotify_sub) -> gboolean;
    fn _ip_get_path_for_wd(wd: gint32) -> *const ::core::ffi::c_char;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
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
pub type GFileMonitorEvent = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_EVENT_MOVED_OUT: GFileMonitorEvent = 10;
pub const G_FILE_MONITOR_EVENT_MOVED_IN: GFileMonitorEvent = 9;
pub const G_FILE_MONITOR_EVENT_RENAMED: GFileMonitorEvent = 8;
pub const G_FILE_MONITOR_EVENT_MOVED: GFileMonitorEvent = 7;
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: GFileMonitorEvent = 6;
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: GFileMonitorEvent = 5;
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: GFileMonitorEvent = 4;
pub const G_FILE_MONITOR_EVENT_CREATED: GFileMonitorEvent = 3;
pub const G_FILE_MONITOR_EVENT_DELETED: GFileMonitorEvent = 2;
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: GFileMonitorEvent = 1;
pub const G_FILE_MONITOR_EVENT_CHANGED: GFileMonitorEvent = 0;
pub type GFile = _GFile;
pub type GFileMonitorSource = _GFileMonitorSource;
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
pub type ik_event_t = ik_event_s;
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const IN_ACCESS: guint32 = 1 as guint32;
pub const IN_MODIFY: guint32 = 2 as guint32;
pub const IN_ATTRIB: guint32 = 4 as guint32;
pub const IN_CLOSE_WRITE: guint32 = 8 as guint32;
pub const IN_CLOSE_NOWRITE: guint32 = 16 as guint32;
pub const IN_OPEN: guint32 = 32 as guint32;
pub const IN_MOVED_FROM: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const IN_MOVED_TO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const IN_CREATE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const IN_DELETE: guint32 = 512 as guint32;
pub const IN_DELETE_SELF: guint32 = 1024 as guint32;
pub const IN_MOVE_SELF: guint32 = 2048 as guint32;
pub const IN_UNMOUNT: guint32 = 8192 as guint32;
pub const IN_Q_OVERFLOW: guint32 = 16384 as guint32;
pub const IN_IGNORED: guint32 = 32768 as guint32;
pub const IN_MOVE: ::core::ffi::c_int = IN_MOVED_FROM | IN_MOVED_TO;
pub const IN_ISDIR: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_ih_debug_enabled: gboolean = FALSE;
#[no_mangle]
pub static mut safe_c2rust_g__inotify_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ih_startup() -> gboolean {
    static mut safe_c2rust_initialized: gboolean = FALSE;
    static mut safe_c2rust_result: gboolean = FALSE;
    g_mutex_lock(&raw mut safe_c2rust_g__inotify_lock_lock);
    if safe_c2rust_initialized == TRUE {
        g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
        return safe_c2rust_result;
    }
    safe_c2rust_result = _ip_startup(Some(
        safe_c2rust_ih_event_callback
            as unsafe extern "C" fn(*mut ik_event_t, *mut inotify_sub, gboolean) -> gboolean,
    ));
    if safe_c2rust_result == 0 {
        g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
        return FALSE;
    }
    _im_startup(Some(
        safe_c2rust_ih_not_missing_callback as unsafe extern "C" fn(*mut inotify_sub) -> (),
    ));
    if safe_c2rust_ih_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"started gvfs inotify backend\n\0" as *const u8 as *const gchar,
        );
    }
    safe_c2rust_initialized = TRUE as gboolean;
    g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ih_sub_add(mut sub: *mut inotify_sub) -> gboolean {
    g_mutex_lock(&raw mut safe_c2rust_g__inotify_lock_lock);
    if _ip_start_watching(sub) == 0 {
        _im_add(sub);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ih_sub_cancel(mut sub: *mut inotify_sub) -> gboolean {
    g_mutex_lock(&raw mut safe_c2rust_g__inotify_lock_lock);
    if (*sub).cancelled == 0 {
        if safe_c2rust_ih_debug_enabled != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"cancelling %s\n\0" as *const u8 as *const gchar,
                (*sub).dirname,
            );
        }
        (*sub).cancelled = TRUE as gboolean;
        _im_rm(sub);
        _ip_stop_watching(sub);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__inotify_lock_lock);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust__ih_fullpath_from_event(
    mut event: *mut ik_event_t,
    mut dirname: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut fullpath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !filename.is_null() {
        fullpath = g_strdup_printf(b"%s/%s\0" as *const u8 as *const gchar, dirname, filename)
            as *mut ::core::ffi::c_char;
    } else if !(*event).name.is_null() {
        fullpath = g_strdup_printf(
            b"%s/%s\0" as *const u8 as *const gchar,
            dirname,
            (*event).name,
        ) as *mut ::core::ffi::c_char;
    } else {
        fullpath = g_strdup_printf(b"%s/\0" as *const u8 as *const gchar, dirname)
            as *mut ::core::ffi::c_char;
    }
    return fullpath;
}
unsafe extern "C" fn safe_c2rust_ih_event_callback(
    mut event: *mut ik_event_t,
    mut sub: *mut inotify_sub,
    mut file_event: gboolean,
) -> gboolean {
    let mut interesting: gboolean = 0;
    let mut event_flags: GFileMonitorEvent = G_FILE_MONITOR_EVENT_CHANGED;
    event_flags = safe_c2rust_ih_mask_to_EventFlags((*event).mask);
    if (*event).mask & IN_MOVE as guint32 != 0 {
        if !(*event).pair.is_null() && (*(*event).pair).wd == (*event).wd {
            interesting = g_file_monitor_source_handle_event(
                (*sub).user_data as *mut GFileMonitorSource,
                G_FILE_MONITOR_EVENT_RENAMED,
                (*event).name,
                (*(*event).pair).name,
                ::core::ptr::null_mut::<GFile>(),
                (*event).timestamp,
            );
        } else {
            let mut other: *mut GFile = ::core::ptr::null_mut::<GFile>();
            if !(*event).pair.is_null() {
                let mut parent_dir: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut fullpath: *mut gchar = ::core::ptr::null_mut::<gchar>();
                parent_dir = _ip_get_path_for_wd((*(*event).pair).wd);
                fullpath = safe_c2rust__ih_fullpath_from_event(
                    (*event).pair as *mut ik_event_t,
                    parent_dir,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) as *mut gchar;
                other = g_file_new_for_path(fullpath);
                g_free(fullpath as gpointer);
            } else {
                other = ::core::ptr::null_mut::<GFile>();
            }
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if event_flags as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
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
                    b"../original/gio/inotify/inotify-helper.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"(int) event_flags != -1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            interesting = g_file_monitor_source_handle_event(
                (*sub).user_data as *mut GFileMonitorSource,
                event_flags,
                (*event).name,
                ::core::ptr::null::<gchar>(),
                other,
                (*event).timestamp,
            );
            if !other.is_null() {
                g_object_unref(other as gpointer);
            }
        }
    } else if event_flags as ::core::ffi::c_int != -(1 as ::core::ffi::c_int) {
        interesting = g_file_monitor_source_handle_event(
            (*sub).user_data as *mut GFileMonitorSource,
            event_flags,
            (*event).name,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null_mut::<GFile>(),
            (*event).timestamp,
        );
    } else {
        interesting = FALSE as gboolean;
    }
    if (*event).mask & IN_CREATE as guint32 != 0 {
        let mut parent_dir_0: *const gchar = ::core::ptr::null::<gchar>();
        let mut fullname: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut s: gint = 0;
        parent_dir_0 = _ip_get_path_for_wd((*event).wd) as *const gchar;
        fullname = safe_c2rust__ih_fullpath_from_event(
            event,
            parent_dir_0 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) as *mut gchar;
        s = stat(fullname, &raw mut buf) as gint;
        g_free(fullname as gpointer);
        if s != 0 as ::core::ffi::c_int
            || !(buf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            || buf.st_nlink != 1 as __nlink_t
        {
            g_file_monitor_source_handle_event(
                (*sub).user_data as *mut GFileMonitorSource,
                G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
                (*event).name,
                ::core::ptr::null::<gchar>(),
                ::core::ptr::null_mut::<GFile>(),
                (*event).timestamp,
            );
        }
    }
    return interesting;
}
unsafe extern "C" fn safe_c2rust_ih_not_missing_callback(mut sub: *mut inotify_sub) {
    let mut now: gint = g_get_monotonic_time() as gint;
    g_file_monitor_source_handle_event(
        (*sub).user_data as *mut GFileMonitorSource,
        G_FILE_MONITOR_EVENT_CREATED,
        (*sub).filename,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<GFile>(),
        now as gint64,
    );
    g_file_monitor_source_handle_event(
        (*sub).user_data as *mut GFileMonitorSource,
        G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
        (*sub).filename,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<GFile>(),
        now as gint64,
    );
}
unsafe extern "C" fn safe_c2rust_ih_mask_to_EventFlags(mut mask: guint32) -> GFileMonitorEvent {
    mask &= !IN_ISDIR as guint32;
    match mask {
        2 => return G_FILE_MONITOR_EVENT_CHANGED,
        8 => return G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
        4 => return G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED,
        2048 | 512 | 1024 => return G_FILE_MONITOR_EVENT_DELETED,
        256 => return G_FILE_MONITOR_EVENT_CREATED,
        64 => return G_FILE_MONITOR_EVENT_MOVED_OUT,
        128 => return G_FILE_MONITOR_EVENT_MOVED_IN,
        8192 => return G_FILE_MONITOR_EVENT_UNMOUNTED,
        16384 | 32 | 16 | 1 | 32768 | _ => return 4294967295 as GFileMonitorEvent,
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
