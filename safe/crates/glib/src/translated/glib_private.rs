extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GWakeup;
    fn g_wakeup_new() -> *mut GWakeup;
    fn g_wakeup_free(wakeup: *mut GWakeup);
    fn g_wakeup_get_pollfd(wakeup: *mut GWakeup, poll_fd: *mut GPollFD);
    fn g_wakeup_signal(wakeup: *mut GWakeup);
    fn g_wakeup_acknowledge(wakeup: *mut GWakeup);
    fn g_datalist_id_update_atomic(
        datalist: *mut *mut GData,
        key_id: GQuark,
        callback: GDataListUpdateAtomicFunc,
        user_data: gpointer,
    ) -> gpointer;
    fn g_get_worker_context() -> *mut GMainContext;
    fn g_check_setuid() -> gboolean;
    fn g_main_context_new_with_next_id(next_id: guint) -> *mut GMainContext;
    fn g_dir_open_with_errno(path: *const gchar, flags: guint) -> *mut GDir;
    fn g_dir_new_from_dirp(dirp: gpointer) -> *mut GDir;
    fn g_find_program_for_path(
        program: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        working_dir: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_uri_get_default_scheme_port(scheme: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn glib_init();
    fn g_set_prgname_once(prgname: *const gchar) -> gboolean;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
pub type GData = _GData;
pub type GDir = _GDir;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
pub type GMainContext = _GMainContext;
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib__private__() -> *const GLibPrivateVTable {
    static mut safe_c2rust_table: GLibPrivateVTable = unsafe {
        GLibPrivateVTable {
            g_wakeup_new: Some(g_wakeup_new as unsafe extern "C" fn() -> *mut GWakeup),
            g_wakeup_free: Some(g_wakeup_free as unsafe extern "C" fn(*mut GWakeup) -> ()),
            g_wakeup_get_pollfd: Some(
                g_wakeup_get_pollfd as unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> (),
            ),
            g_wakeup_signal: Some(g_wakeup_signal as unsafe extern "C" fn(*mut GWakeup) -> ()),
            g_wakeup_acknowledge: Some(
                g_wakeup_acknowledge as unsafe extern "C" fn(*mut GWakeup) -> (),
            ),
            g_get_worker_context: Some(
                g_get_worker_context as unsafe extern "C" fn() -> *mut GMainContext,
            ),
            g_check_setuid: Some(g_check_setuid as unsafe extern "C" fn() -> gboolean),
            g_main_context_new_with_next_id: Some(
                g_main_context_new_with_next_id as unsafe extern "C" fn(guint) -> *mut GMainContext,
            ),
            g_dir_open_with_errno: Some(
                g_dir_open_with_errno as unsafe extern "C" fn(*const gchar, guint) -> *mut GDir,
            ),
            g_dir_new_from_dirp: Some(
                g_dir_new_from_dirp as unsafe extern "C" fn(gpointer) -> *mut GDir,
            ),
            glib_init: Some(glib_init as unsafe extern "C" fn() -> ()),
            g_win32_push_empty_invalid_parameter_handler: Some(
                safe_c2rust_g_win32_push_empty_invalid_parameter_handler
                    as unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> (),
            ),
            g_win32_pop_invalid_parameter_handler: Some(
                safe_c2rust_g_win32_pop_invalid_parameter_handler
                    as unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> (),
            ),
            g_find_program_for_path: Some(
                g_find_program_for_path
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> *mut ::core::ffi::c_char,
            ),
            g_uri_get_default_scheme_port: Some(
                g_uri_get_default_scheme_port
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            g_set_prgname_once: Some(
                g_set_prgname_once as unsafe extern "C" fn(*const gchar) -> gboolean,
            ),
            g_datalist_id_update_atomic: Some(
                g_datalist_id_update_atomic
                    as unsafe extern "C" fn(
                        *mut *mut GData,
                        GQuark,
                        GDataListUpdateAtomicFunc,
                        gpointer,
                    ) -> gpointer,
            ),
        }
    };
    return &raw const safe_c2rust_table;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_win32_push_empty_invalid_parameter_handler(
    mut handler: *mut GWin32InvalidParameterHandler,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_win32_pop_invalid_parameter_handler(
    mut handler: *mut GWin32InvalidParameterHandler,
) {
}
