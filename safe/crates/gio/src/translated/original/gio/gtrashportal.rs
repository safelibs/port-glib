extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GCancellablePrivate;
    pub type _GFile;
    pub type _GUnixFDListPrivate;
    pub type _GDBusConnection;
    pub type _GXdpTrash;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_variant_new_handle(value: gint32) -> *mut GVariant;
    fn g_object_unref(object: gpointer);
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_unix_fd_list_new() -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn gxdp_trash_call_trash_file_sync(
        proxy: *mut GXdpTrash,
        arg_fd: *mut GVariant,
        fd_list: *mut GUnixFDList,
        out_result: *mut guint,
        out_fd_list: *mut *mut GUnixFDList,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_trash_proxy_new_sync(
        connection: *mut GDBusConnection,
        flags: GDBusProxyFlags,
        name: *const gchar,
        object_path: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GXdpTrash;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
}
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
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
pub type GVariant = _GVariant;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusConnection = _GDBusConnection;
pub type GXdpTrash = _GXdpTrash;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __O_PATH: ::core::ffi::c_int = 0o10000000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_PATH: ::core::ffi::c_int = __O_PATH;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_ensure_trash_portal() -> *mut GXdpTrash {
    static mut safe_c2rust_trash: *mut GXdpTrash =
        ::core::ptr::null::<GXdpTrash>() as *mut GXdpTrash;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_trash;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GXdpTrash = ::core::ptr::null_mut::<GXdpTrash>();
            let mut gapg_temp_atomic: *mut *mut GXdpTrash = &raw mut safe_c2rust_trash;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_trash as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut connection: *mut GDBusConnection = g_bus_get_sync(
            G_BUS_TYPE_SESSION,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        let mut proxy: *mut GXdpTrash = ::core::ptr::null_mut::<GXdpTrash>();
        if !connection.is_null() {
            proxy = gxdp_trash_proxy_new_sync(
                connection,
                G_DBUS_PROXY_FLAGS_NONE,
                b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
                b"/org/freedesktop/portal/desktop\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(connection as gpointer);
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_trash = proxy;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_trash as *mut ::core::ffi::c_void,
            proxy as guintptr as gpointer,
        );
    }
    return safe_c2rust_trash;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trash_portal_trash_file(
    mut file: *mut GFile,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    let mut fd: ::core::ffi::c_int = 0;
    let mut fd_in: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    let mut ret: gboolean = FALSE;
    let mut portal_result: guint = 0 as guint;
    let mut proxy: *mut GXdpTrash = ::core::ptr::null_mut::<GXdpTrash>();
    proxy = safe_c2rust_ensure_trash_portal();
    if proxy.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            b"Trash portal is not available\0" as *const u8 as *const gchar,
        );
    } else {
        path = g_file_get_path(file);
        fd = open(path, O_RDWR | O_CLOEXEC | O_NOFOLLOW);
        if fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EISDIR {
            fd = open(path, O_PATH | O_CLOEXEC | O_RDONLY);
        }
        errsv = *__errno_location();
        if fd == -(1 as ::core::ffi::c_int) {
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                b"Failed to open %s\0" as *const u8 as *const gchar,
                path,
            );
        } else {
            fd_list = g_unix_fd_list_new();
            fd_in = g_unix_fd_list_append(fd_list, fd as gint, error) as ::core::ffi::c_int;
            g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
            if !(fd_in == -(1 as ::core::ffi::c_int)) {
                ret = gxdp_trash_call_trash_file_sync(
                    proxy,
                    g_variant_new_handle(fd_in as gint32),
                    fd_list,
                    &raw mut portal_result,
                    ::core::ptr::null_mut::<*mut GUnixFDList>(),
                    ::core::ptr::null_mut::<GCancellable>(),
                    error,
                );
                if ret != 0 && portal_result != 1 as guint {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                        b"Trash portal failed on %s\0" as *const u8 as *const gchar,
                        path,
                    );
                    ret = FALSE as gboolean;
                }
            }
        }
    }
    let mut _pp: *mut *mut GUnixFDList = &raw mut fd_list;
    let mut _ptr: *mut GUnixFDList = *_pp;
    *_pp = ::core::ptr::null_mut::<GUnixFDList>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free(path as gpointer);
    return ret;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
