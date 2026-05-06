extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFile;
    pub type _GTask;
    pub type _GUnixFDListPrivate;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    pub type _GXdpOpenURI;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_random_int_range(begin: gint32, end: gint32) -> gint32;
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_object_unref(object: gpointer);
    fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
    fn g_object_set_data_full(
        object: *mut GObject,
        key: *const gchar,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_get_unique_name(connection: *mut GDBusConnection) -> *const gchar;
    fn g_dbus_connection_signal_subscribe(
        connection: *mut GDBusConnection,
        sender: *const gchar,
        interface_name: *const gchar,
        member: *const gchar,
        object_path: *const gchar,
        arg0: *const gchar,
        flags: GDBusSignalFlags,
        callback: GDBusSignalCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_connection_signal_unsubscribe(
        connection: *mut GDBusConnection,
        subscription_id: guint,
    );
    fn g_dbus_proxy_get_connection(proxy: *mut GDBusProxy) -> *mut GDBusConnection;
    fn g_file_new_for_uri(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_is_native(file: *mut GFile) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_report_new_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_unix_fd_list_new_from_array(fds: *const gint, n_fds: gint) -> *mut GUnixFDList;
    fn gxdp_open_uri_call_open_uri(
        proxy: *mut GXdpOpenURI,
        arg_parent_window: *const gchar,
        arg_uri: *const gchar,
        arg_options: *mut GVariant,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn gxdp_open_uri_call_open_uri_finish(
        proxy: *mut GXdpOpenURI,
        out_handle: *mut *mut gchar,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_open_uri_call_open_uri_sync(
        proxy: *mut GXdpOpenURI,
        arg_parent_window: *const gchar,
        arg_uri: *const gchar,
        arg_options: *mut GVariant,
        out_handle: *mut *mut gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_open_uri_call_open_file(
        proxy: *mut GXdpOpenURI,
        arg_parent_window: *const gchar,
        arg_fd: *mut GVariant,
        arg_options: *mut GVariant,
        fd_list: *mut GUnixFDList,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn gxdp_open_uri_call_open_file_finish(
        proxy: *mut GXdpOpenURI,
        out_handle: *mut *mut gchar,
        out_fd_list: *mut *mut GUnixFDList,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_open_uri_call_open_file_sync(
        proxy: *mut GXdpOpenURI,
        arg_parent_window: *const gchar,
        arg_fd: *mut GVariant,
        arg_options: *mut GVariant,
        fd_list: *mut GUnixFDList,
        out_handle: *mut *mut gchar,
        out_fd_list: *mut *mut GUnixFDList,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_open_uri_proxy_new_sync(
        connection: *mut GDBusConnection,
        flags: GDBusProxyFlags,
        name: *const gchar,
        object_path: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GXdpOpenURI;
}
pub type size_t = usize;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
pub type GDBusSignalCallback = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        gpointer,
    ) -> (),
>;
pub type GXdpOpenURI = _GXdpOpenURI;
pub const XDG_DESKTOP_PORTAL_FAILED: C2RustUnnamed_1 = 2;
pub const XDG_DESKTOP_PORTAL_CANCELLED: C2RustUnnamed_1 = 1;
pub const XDG_DESKTOP_PORTAL_SUCCESS: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
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
pub const G_VARIANT_TYPE_VARDICT: *const GVariantType =
    b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
static mut safe_c2rust_openuri: *mut GXdpOpenURI =
    ::core::ptr::null::<GXdpOpenURI>() as *mut GXdpOpenURI;
unsafe extern "C" fn safe_c2rust_init_openuri_portal() -> gboolean {
    static mut safe_c2rust_openuri_inited: gsize = 0 as gsize;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_openuri_inited;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_openuri_inited;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_openuri_inited as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut connection: *mut GDBusConnection = g_bus_get_sync(
            G_BUS_TYPE_SESSION,
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut error,
        );
        if !connection.is_null() {
            safe_c2rust_openuri = gxdp_open_uri_proxy_new_sync(
                connection,
                G_DBUS_PROXY_FLAGS_NONE,
                b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
                b"/org/freedesktop/portal/desktop\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<GCancellable>(),
                &raw mut error,
            );
            if safe_c2rust_openuri.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Cannot create document portal proxy: %s\0" as *const u8 as *const gchar,
                    (*error).message,
                );
                g_error_free(error);
            }
            g_object_unref(connection as gpointer);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Cannot connect to session bus when initializing document portal: %s\0"
                    as *const u8 as *const gchar,
                (*error).message,
            );
            g_error_free(error);
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_openuri_inited = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_openuri_inited as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
    return (safe_c2rust_openuri != NULL_0 as *mut GXdpOpenURI) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_openuri_portal_open_uri(
    mut uri: *const ::core::ffi::c_char,
    mut parent_window: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut opt_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut res: gboolean = 0;
    if safe_c2rust_init_openuri_portal() == 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            b"OpenURI portal is not available\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    g_variant_builder_init(&raw mut opt_builder, G_VARIANT_TYPE_VARDICT);
    file = g_file_new_for_uri(uri);
    if g_file_is_native(file) != 0 {
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
        let mut fd: ::core::ffi::c_int = 0;
        let mut fd_id: ::core::ffi::c_int = 0;
        let mut errsv: ::core::ffi::c_int = 0;
        path = g_file_get_path(file);
        fd = open(path, O_RDONLY | O_CLOEXEC);
        errsv = *__errno_location();
        if fd == -(1 as ::core::ffi::c_int) {
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                b"Failed to open '%s'\0" as *const u8 as *const gchar,
                path,
            );
            g_free(path as gpointer);
            g_variant_builder_clear(&raw mut opt_builder);
            return FALSE;
        }
        fd_list = g_unix_fd_list_new_from_array(&raw mut fd, 1 as gint);
        fd = -(1 as ::core::ffi::c_int);
        fd_id = 0 as ::core::ffi::c_int;
        res = gxdp_open_uri_call_open_file_sync(
            safe_c2rust_openuri,
            if !parent_window.is_null() {
                parent_window as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
            g_variant_new(b"h\0" as *const u8 as *const gchar, fd_id),
            g_variant_builder_end(&raw mut opt_builder),
            fd_list,
            ::core::ptr::null_mut::<*mut gchar>(),
            ::core::ptr::null_mut::<*mut GUnixFDList>(),
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        );
        g_free(path as gpointer);
        g_object_unref(fd_list as gpointer);
    } else {
        res = gxdp_open_uri_call_open_uri_sync(
            safe_c2rust_openuri,
            if !parent_window.is_null() {
                parent_window as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
            uri as *const gchar,
            g_variant_builder_end(&raw mut opt_builder),
            ::core::ptr::null_mut::<*mut gchar>(),
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        );
    }
    g_object_unref(file as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_response_received(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const ::core::ffi::c_char,
    mut object_path: *const ::core::ffi::c_char,
    mut interface_name: *const ::core::ffi::c_char,
    mut signal_name: *const ::core::ffi::c_char,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut response: guint32 = 0;
    let mut signal_id: guint = 0;
    signal_id = g_object_get_data(
        task as *mut ::core::ffi::c_void as *mut GObject,
        b"signal-id\0" as *const u8 as *const gchar,
    ) as gulong as guint;
    g_dbus_connection_signal_unsubscribe(connection, signal_id);
    g_variant_get(
        parameters,
        b"(u@a{sv})\0" as *const u8 as *const gchar,
        &raw mut response,
        NULL_0,
    );
    match response {
        0 => {
            g_task_return_boolean(task, TRUE);
        }
        1 => {
            g_task_return_new_error_literal(
                task,
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
                b"Launch cancelled\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        2 | _ => {
            g_task_return_new_error_literal(
                task,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Launch failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_open_call_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut openuri: *mut GXdpOpenURI = source as *mut ::core::ffi::c_void as *mut GXdpOpenURI;
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut open_file: gboolean = 0;
    let mut res: gboolean = 0;
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut signal_id: guint = 0;
    connection =
        g_dbus_proxy_get_connection(openuri as *mut ::core::ffi::c_void as *mut GDBusProxy);
    open_file = g_object_get_data(
        task as *mut ::core::ffi::c_void as *mut GObject,
        b"open-file\0" as *const u8 as *const gchar,
    ) as glong as gint as gboolean;
    if open_file != 0 {
        res = gxdp_open_uri_call_open_file_finish(
            openuri,
            &raw mut path,
            ::core::ptr::null_mut::<*mut GUnixFDList>(),
            result,
            &raw mut error,
        );
    } else {
        res = gxdp_open_uri_call_open_uri_finish(openuri, &raw mut path, result, &raw mut error);
    }
    if res == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        g_free(path as gpointer);
        return;
    }
    handle = g_object_get_data(
        task as *mut ::core::ffi::c_void as *mut GObject,
        b"handle\0" as *const u8 as *const gchar,
    ) as *const ::core::ffi::c_char;
    if g_strcmp0(handle, path) != 0 as ::core::ffi::c_int {
        signal_id = g_object_get_data(
            task as *mut ::core::ffi::c_void as *mut GObject,
            b"signal-id\0" as *const u8 as *const gchar,
        ) as gulong as guint;
        g_dbus_connection_signal_unsubscribe(connection, signal_id);
        signal_id = g_dbus_connection_signal_subscribe(
            connection,
            b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Request\0" as *const u8 as *const gchar,
            b"Response\0" as *const u8 as *const gchar,
            path,
            ::core::ptr::null::<gchar>(),
            G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE,
            Some(
                safe_c2rust_response_received
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            task as gpointer,
            None,
        );
        g_object_set_data(
            task as *mut ::core::ffi::c_void as *mut GObject,
            b"signal-id\0" as *const u8 as *const gchar,
            signal_id as glong as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_openuri_portal_open_uri_async(
    mut uri: *const ::core::ffi::c_char,
    mut parent_window: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut opts: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut i: ::core::ffi::c_int = 0;
    let mut signal_id: guint = 0;
    if safe_c2rust_init_openuri_portal() == 0 {
        g_task_report_new_error(
            NULL_0,
            callback,
            user_data,
            NULL_0,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            b"OpenURI portal is not available\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    connection = g_dbus_proxy_get_connection(
        safe_c2rust_openuri as *mut ::core::ffi::c_void as *mut GDBusProxy,
    );
    if callback.is_some() {
        let mut opt_builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sender: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut handle: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        task = g_task_new(NULL_0, cancellable, callback, user_data);
        token = g_strdup_printf(
            b"gio%d\0" as *const u8 as *const gchar,
            g_random_int_range(0 as gint32, G_MAXINT),
        ) as *mut ::core::ffi::c_char;
        sender = safe_c2rust_g_strdup_inline(
            g_dbus_connection_get_unique_name(connection).offset(1 as ::core::ffi::c_int as isize),
        );
        i = 0 as ::core::ffi::c_int;
        while *sender.offset(i as isize) != 0 {
            if *sender.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
                *sender.offset(i as isize) = '_' as i32 as ::core::ffi::c_char;
            }
            i += 1;
        }
        handle = g_strdup_printf(
            b"/org/freedesktop/portal/desktop/request/%s/%s\0" as *const u8 as *const gchar,
            sender,
            token,
        ) as *mut ::core::ffi::c_char;
        g_object_set_data_full(
            task as *mut ::core::ffi::c_void as *mut GObject,
            b"handle\0" as *const u8 as *const gchar,
            handle as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_free(sender as gpointer);
        signal_id = g_dbus_connection_signal_subscribe(
            connection,
            b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Request\0" as *const u8 as *const gchar,
            b"Response\0" as *const u8 as *const gchar,
            handle,
            ::core::ptr::null::<gchar>(),
            G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE,
            Some(
                safe_c2rust_response_received
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            task as gpointer,
            None,
        );
        g_object_set_data(
            task as *mut ::core::ffi::c_void as *mut GObject,
            b"signal-id\0" as *const u8 as *const gchar,
            signal_id as glong as gpointer,
        );
        g_variant_builder_init(&raw mut opt_builder, G_VARIANT_TYPE_VARDICT);
        g_variant_builder_add(
            &raw mut opt_builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"handle_token\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string(token),
        );
        g_free(token as gpointer);
        opts = g_variant_builder_end(&raw mut opt_builder);
    } else {
        task = ::core::ptr::null_mut::<GTask>();
    }
    file = g_file_new_for_uri(uri);
    if g_file_is_native(file) != 0 {
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
        let mut fd: ::core::ffi::c_int = 0;
        let mut fd_id: ::core::ffi::c_int = 0;
        let mut errsv: ::core::ffi::c_int = 0;
        if !task.is_null() {
            g_object_set_data(
                task as *mut ::core::ffi::c_void as *mut GObject,
                b"open-file\0" as *const u8 as *const gchar,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as glong as gpointer,
            );
        }
        path = g_file_get_path(file);
        fd = open(path, O_RDONLY | O_CLOEXEC);
        errsv = *__errno_location();
        if fd == -(1 as ::core::ffi::c_int) {
            g_task_report_new_error(
                NULL_0,
                callback,
                user_data,
                NULL_0,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                b"OpenURI portal is not available\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        fd_list = g_unix_fd_list_new_from_array(&raw mut fd, 1 as gint);
        fd = -(1 as ::core::ffi::c_int);
        fd_id = 0 as ::core::ffi::c_int;
        gxdp_open_uri_call_open_file(
            safe_c2rust_openuri,
            if !parent_window.is_null() {
                parent_window as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
            g_variant_new(b"h\0" as *const u8 as *const gchar, fd_id),
            opts,
            fd_list,
            cancellable,
            if !task.is_null() {
                Some(
                    safe_c2rust_open_call_done
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                )
            } else {
                None
            },
            task as gpointer,
        );
        g_object_unref(fd_list as gpointer);
        g_free(path as gpointer);
    } else {
        gxdp_open_uri_call_open_uri(
            safe_c2rust_openuri,
            if !parent_window.is_null() {
                parent_window as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
            uri as *const gchar,
            opts,
            cancellable,
            if !task.is_null() {
                Some(
                    safe_c2rust_open_call_done
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                )
            } else {
                None
            },
            task as gpointer,
        );
    }
    g_object_unref(file as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_openuri_portal_open_uri_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
