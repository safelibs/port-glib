extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GKeyFile;
    pub type _GWakeup;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_contains(strv: *const *const gchar, str: *const gchar) -> gboolean;
    fn g_key_file_new() -> *mut GKeyFile;
    fn g_key_file_unref(key_file: *mut GKeyFile);
    fn g_key_file_load_from_file(
        key_file: *mut GKeyFile,
        file: *const gchar,
        flags: GKeyFileFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_get_string(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_get_string_list(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_spawn_sync(
        working_directory: *const gchar,
        argv: *mut *mut gchar,
        envp: *mut *mut gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        standard_output: *mut *mut gchar,
        standard_error: *mut *mut gchar,
        wait_status: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_spawn_check_wait_status(wait_status: gint, error: *mut *mut GError) -> gboolean;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn glib_get_sandbox_type() -> GSandboxType;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
pub type GKeyFile = _GKeyFile;
pub type GKeyFileFlags = ::core::ffi::c_uint;
pub const G_KEY_FILE_KEEP_TRANSLATIONS: GKeyFileFlags = 2;
pub const G_KEY_FILE_KEEP_COMMENTS: GKeyFileFlags = 1;
pub const G_KEY_FILE_NONE: GKeyFileFlags = 0;
pub type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GSpawnFlags = ::core::ffi::c_uint;
pub const G_SPAWN_STDIN_FROM_DEV_NULL: GSpawnFlags = 2048;
pub const G_SPAWN_CHILD_INHERITS_STDERR: GSpawnFlags = 1024;
pub const G_SPAWN_CHILD_INHERITS_STDOUT: GSpawnFlags = 512;
pub const G_SPAWN_CLOEXEC_PIPES: GSpawnFlags = 256;
pub const G_SPAWN_SEARCH_PATH_FROM_ENVP: GSpawnFlags = 128;
pub const G_SPAWN_FILE_AND_ARGV_ZERO: GSpawnFlags = 64;
pub const G_SPAWN_CHILD_INHERITS_STDIN: GSpawnFlags = 32;
pub const G_SPAWN_STDERR_TO_DEV_NULL: GSpawnFlags = 16;
pub const G_SPAWN_STDOUT_TO_DEV_NULL: GSpawnFlags = 8;
pub const G_SPAWN_SEARCH_PATH: GSpawnFlags = 4;
pub const G_SPAWN_DO_NOT_REAP_CHILD: GSpawnFlags = 2;
pub const G_SPAWN_LEAVE_DESCRIPTORS_OPEN: GSpawnFlags = 1;
pub const G_SPAWN_DEFAULT: GSpawnFlags = 0;
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
pub const G_SANDBOX_TYPE_SNAP: GSandboxType = 2;
pub type GSandboxType = ::core::ffi::c_uint;
pub const G_SANDBOX_TYPE_FLATPAK: GSandboxType = 1;
pub const G_SANDBOX_TYPE_UNKNOWN: GSandboxType = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_sandbox_type: GSandboxType = G_SANDBOX_TYPE_UNKNOWN;
static mut safe_c2rust_use_portal: gboolean = 0;
static mut safe_c2rust_network_available: gboolean = 0;
static mut safe_c2rust_dconf_access: gboolean = 0;
static mut safe_c2rust_snapctl: *const ::core::ffi::c_char =
    b"/usr/bin/snapctl\0" as *const u8 as *const ::core::ffi::c_char;
unsafe extern "C" fn safe_c2rust_snap_plug_is_connected(mut plug_name: *const gchar) -> gboolean {
    let mut wait_status: gint = 0;
    let mut argv: [*const gchar; 4] = [
        safe_c2rust_snapctl,
        b"is-connected\0" as *const u8 as *const ::core::ffi::c_char,
        plug_name,
        ::core::ptr::null::<gchar>(),
    ];
    if (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")()
        != 0
    {
        return FALSE;
    }
    if g_spawn_sync(
        ::core::ptr::null::<gchar>(),
        &raw mut argv as *mut *const gchar as *mut *mut gchar,
        ::core::ptr::null_mut::<*mut gchar>(),
        (G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int) as GSpawnFlags,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        &raw mut wait_status,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        return FALSE;
    }
    return g_spawn_check_wait_status(wait_status, ::core::ptr::null_mut::<*mut GError>());
}
unsafe extern "C" fn safe_c2rust_sandbox_info_read() {
    static mut safe_c2rust_sandbox_info_is_read: gsize = 0 as gsize;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_sandbox_info_is_read;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_sandbox_info_is_read;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &raw mut safe_c2rust_sandbox_info_is_read as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    safe_c2rust_sandbox_type = glib_get_sandbox_type();
    match safe_c2rust_sandbox_type as ::core::ffi::c_uint {
        1 => {
            let mut keyfile: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
            let mut keyfile_path: *const ::core::ffi::c_char =
                b"/.flatpak-info\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_use_portal = TRUE as gboolean;
            safe_c2rust_network_available = FALSE as gboolean;
            safe_c2rust_dconf_access = FALSE as gboolean;
            keyfile = g_key_file_new();
            if g_key_file_load_from_file(
                keyfile,
                keyfile_path as *const gchar,
                G_KEY_FILE_NONE,
                ::core::ptr::null_mut::<*mut GError>(),
            ) != 0
            {
                let mut shared: *mut *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                let mut dconf_policy: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                shared = g_key_file_get_string_list(
                    keyfile,
                    b"Context\0" as *const u8 as *const gchar,
                    b"shared\0" as *const u8 as *const gchar,
                    ::core::ptr::null_mut::<gsize>(),
                    ::core::ptr::null_mut::<*mut GError>(),
                ) as *mut *mut ::core::ffi::c_char;
                if !shared.is_null() {
                    safe_c2rust_network_available = g_strv_contains(
                        shared as *const *const gchar,
                        b"network\0" as *const u8 as *const gchar,
                    );
                    g_strfreev(shared as *mut *mut gchar);
                }
                dconf_policy = g_key_file_get_string(
                    keyfile,
                    b"Session Bus Policy\0" as *const u8 as *const gchar,
                    b"ca.desrt.dconf\0" as *const u8 as *const gchar,
                    ::core::ptr::null_mut::<*mut GError>(),
                ) as *mut ::core::ffi::c_char;
                if !dconf_policy.is_null() {
                    if strcmp(
                        dconf_policy,
                        b"talk\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        safe_c2rust_dconf_access = TRUE as gboolean;
                    }
                    g_free(dconf_policy as gpointer);
                }
            }
            g_key_file_unref(keyfile);
        }
        0 => {
            let mut var: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            var = g_getenv(b"GIO_USE_PORTALS\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char;
            if !var.is_null()
                && *var.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '1' as i32
            {
                safe_c2rust_use_portal = TRUE as gboolean;
            }
            safe_c2rust_network_available = TRUE as gboolean;
            safe_c2rust_dconf_access = TRUE as gboolean;
        }
        2 | _ => {}
    }
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_sandbox_info_is_read = 1 as gsize;
    } else {
    };
    g_once_init_leave(
        &raw mut safe_c2rust_sandbox_info_is_read as *mut ::core::ffi::c_void,
        1 as ::core::ffi::c_int as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_should_use_portal() -> gboolean {
    safe_c2rust_sandbox_info_read();
    if safe_c2rust_sandbox_type as ::core::ffi::c_uint
        == G_SANDBOX_TYPE_SNAP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_snap_plug_is_connected(b"desktop\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_use_portal;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_network_available_in_sandbox() -> gboolean {
    safe_c2rust_sandbox_info_read();
    if safe_c2rust_sandbox_type as ::core::ffi::c_uint
        == G_SANDBOX_TYPE_SNAP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (safe_c2rust_snap_plug_is_connected(b"desktop\0" as *const u8 as *const gchar) != 0
            || safe_c2rust_snap_plug_is_connected(b"network-status\0" as *const u8 as *const gchar)
                != 0) as ::core::ffi::c_int;
    }
    return safe_c2rust_network_available;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_has_dconf_access_in_sandbox() -> gboolean {
    safe_c2rust_sandbox_info_read();
    if safe_c2rust_sandbox_type as ::core::ffi::c_uint
        == G_SANDBOX_TYPE_SNAP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_snap_plug_is_connected(b"gsettings\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_dconf_access;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
