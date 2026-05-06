extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GDrive;
    pub type _GFile;
    pub type _GIcon;
    pub type _GMount;
    pub type _GMountOperationPrivate;
    pub type _GTask;
    pub type _GVolume;
    pub type _GSubprocess;
    pub type _GUnixVolumeMonitor;
    pub type _GUnixMount;
    pub type _GUnixMountPoint;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_error(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        error: *const GError,
        error_domain: GQuark,
        error_code: ::core::ffi::c_int,
    );
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn _g_unix_volume_monitor_update(monitor: *mut GUnixVolumeMonitor);
    fn g_io_error_quark() -> GQuark;
    fn g_subprocess_newv(
        argv: *const *const gchar,
        flags: GSubprocessFlags,
        error: *mut *mut GError,
    ) -> *mut GSubprocess;
    fn g_subprocess_get_successful(subprocess: *mut GSubprocess) -> gboolean;
    fn g_subprocess_communicate_utf8_async(
        subprocess: *mut GSubprocess,
        stdin_buf: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_subprocess_communicate_utf8_finish(
        subprocess: *mut GSubprocess,
        result: *mut GAsyncResult,
        stdout_buf: *mut *mut ::core::ffi::c_char,
        stderr_buf: *mut *mut ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_volume_get_type() -> GType;
    fn g_unix_mount_point_get_mount_path(
        mount_point: *mut GUnixMountPoint,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_point_get_device_path(
        mount_point: *mut GUnixMountPoint,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_point_get_fs_type(
        mount_point: *mut GUnixMountPoint,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_point_is_user_mountable(mount_point: *mut GUnixMountPoint) -> gboolean;
    fn g_unix_mount_point_is_loopback(mount_point: *mut GUnixMountPoint) -> gboolean;
    fn g_unix_mount_point_guess_can_eject(mount_point: *mut GUnixMountPoint) -> gboolean;
    fn g_unix_mount_point_guess_name(mount_point: *mut GUnixMountPoint)
        -> *mut ::core::ffi::c_char;
    fn g_unix_mount_point_guess_icon(mount_point: *mut GUnixMountPoint) -> *mut GIcon;
    fn g_unix_mount_point_guess_symbolic_icon(mount_point: *mut GUnixMountPoint) -> *mut GIcon;
    fn _g_unix_mount_unset_volume(mount: *mut GUnixMount, volume: *mut GUnixVolume);
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInterface {
    pub g_type: GType,
    pub g_instance_type: GType,
}
pub type GTypeInterface = _GTypeInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GParamFlags = ::core::ffi::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor:
        Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>,
    pub set_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>,
    pub get_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> ()>,
    pub notify: Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
pub type GMountMountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_MOUNT_NONE: GMountMountFlags = 0;
pub type GMountUnmountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_UNMOUNT_FORCE: GMountUnmountFlags = 1;
pub const G_MOUNT_UNMOUNT_NONE: GMountUnmountFlags = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_0 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_0 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_0 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_0 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_0 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_0 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_0 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_0 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_0 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_0 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_0 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_0 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_0 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_0 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_0 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_0 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_0 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_0 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_0 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_0 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_0 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_0 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_0 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_0 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_0 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_0 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_0 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_0 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_0 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_0 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_0 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_0 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_0 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_0 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_0 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_0 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_0 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_0 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_0 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_0 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_0 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_0 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_0 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_0 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_0 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_0 = 0;
pub type GSubprocessFlags = ::core::ffi::c_uint;
pub const G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP: GSubprocessFlags = 256;
pub const G_SUBPROCESS_FLAGS_INHERIT_FDS: GSubprocessFlags = 128;
pub const G_SUBPROCESS_FLAGS_STDERR_MERGE: GSubprocessFlags = 64;
pub const G_SUBPROCESS_FLAGS_STDERR_SILENCE: GSubprocessFlags = 32;
pub const G_SUBPROCESS_FLAGS_STDERR_PIPE: GSubprocessFlags = 16;
pub const G_SUBPROCESS_FLAGS_STDOUT_SILENCE: GSubprocessFlags = 8;
pub const G_SUBPROCESS_FLAGS_STDOUT_PIPE: GSubprocessFlags = 4;
pub const G_SUBPROCESS_FLAGS_STDIN_INHERIT: GSubprocessFlags = 2;
pub const G_SUBPROCESS_FLAGS_STDIN_PIPE: GSubprocessFlags = 1;
pub const G_SUBPROCESS_FLAGS_NONE: GSubprocessFlags = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GDrive = _GDrive;
pub type GFile = _GFile;
pub type GIcon = _GIcon;
pub type GMount = _GMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperation {
    pub parent_instance: GObject,
    pub priv_0: *mut GMountOperationPrivate,
}
pub type GMountOperationPrivate = _GMountOperationPrivate;
pub type GMountOperation = _GMountOperation;
pub type GTask = _GTask;
pub type GVolume = _GVolume;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVolumeMonitor {
    pub parent_instance: GObject,
    pub priv_0: gpointer,
}
pub type GVolumeMonitor = _GVolumeMonitor;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GSubprocess = _GSubprocess;
pub type GUnixVolumeMonitor = _GUnixVolumeMonitor;
pub type GUnixMount = _GUnixMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixVolume {
    pub parent: GObject,
    pub volume_monitor: *mut GVolumeMonitor,
    pub mount: *mut GUnixMount,
    pub device_path: *mut ::core::ffi::c_char,
    pub mount_path: *mut ::core::ffi::c_char,
    pub can_eject: gboolean,
    pub identifier: *mut ::core::ffi::c_char,
    pub identifier_type: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub icon: *mut GIcon,
    pub symbolic_icon: *mut GIcon,
}
pub type GUnixVolume = _GUnixVolume;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVolumeIface {
    pub g_iface: GTypeInterface,
    pub changed: Option<unsafe extern "C" fn(*mut GVolume) -> ()>,
    pub removed: Option<unsafe extern "C" fn(*mut GVolume) -> ()>,
    pub get_name: Option<unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char>,
    pub get_icon: Option<unsafe extern "C" fn(*mut GVolume) -> *mut GIcon>,
    pub get_uuid: Option<unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char>,
    pub get_drive: Option<unsafe extern "C" fn(*mut GVolume) -> *mut GDrive>,
    pub get_mount: Option<unsafe extern "C" fn(*mut GVolume) -> *mut GMount>,
    pub can_mount: Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>,
    pub can_eject: Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>,
    pub mount_fn: Option<
        unsafe extern "C" fn(
            *mut GVolume,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_finish:
        Option<unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject: Option<
        unsafe extern "C" fn(
            *mut GVolume,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_finish:
        Option<unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub get_identifier: Option<
        unsafe extern "C" fn(*mut GVolume, *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
    >,
    pub enumerate_identifiers:
        Option<unsafe extern "C" fn(*mut GVolume) -> *mut *mut ::core::ffi::c_char>,
    pub should_automount: Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>,
    pub get_activation_root: Option<unsafe extern "C" fn(*mut GVolume) -> *mut GFile>,
    pub eject_with_operation: Option<
        unsafe extern "C" fn(
            *mut GVolume,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub get_sort_key: Option<unsafe extern "C" fn(*mut GVolume) -> *const gchar>,
    pub get_symbolic_icon: Option<unsafe extern "C" fn(*mut GVolume) -> *mut GIcon>,
}
pub type GVolumeIface = _GVolumeIface;
pub type GUnixMountPoint = _GUnixMountPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixVolumeClass {
    pub parent_class: GObjectClass,
}
pub type GUnixVolumeClass = _GUnixVolumeClass;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GUnixVolume\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixVolumeClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_volume_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixVolume>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixVolume) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_volume_init as unsafe extern "C" fn(*mut GUnixVolume) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GVolumeIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_unix_volume_volume_iface_init
                as unsafe extern "C" fn(*mut GVolumeIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_volume_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_unix_volume_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_unix_volume_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
static mut safe_c2rust_GUnixVolume_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_unix_volume_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_volume_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixVolume_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GUnixVolume_private_offset);
    }
    safe_c2rust_g_unix_volume_class_init(klass as *mut GUnixVolumeClass);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_finalize(mut object: *mut GObject) {
    let mut volume: *mut GUnixVolume = ::core::ptr::null_mut::<GUnixVolume>();
    volume = object as *mut ::core::ffi::c_void as *mut GUnixVolume;
    if !(*volume).volume_monitor.is_null() {
        g_object_unref((*volume).volume_monitor as gpointer);
    }
    if !(*volume).mount.is_null() {
        _g_unix_mount_unset_volume((*volume).mount, volume);
    }
    g_object_unref((*volume).icon as gpointer);
    g_object_unref((*volume).symbolic_icon as gpointer);
    g_free((*volume).name as gpointer);
    g_free((*volume).mount_path as gpointer);
    g_free((*volume).device_path as gpointer);
    g_free((*volume).identifier as gpointer);
    g_free((*volume).identifier_type as gpointer);
    (*(safe_c2rust_g_unix_volume_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_class_init(mut klass: *mut GUnixVolumeClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_unix_volume_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_init(mut unix_volume: *mut GUnixVolume) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_new(
    mut volume_monitor: *mut GVolumeMonitor,
    mut mountpoint: *mut GUnixMountPoint,
) -> *mut GUnixVolume {
    let mut volume: *mut GUnixVolume = ::core::ptr::null_mut::<GUnixVolume>();
    if !(g_unix_mount_point_is_user_mountable(mountpoint) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char =
                    g_unix_mount_point_get_device_path(mountpoint) as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"/vol/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                g_unix_mount_point_get_device_path(mountpoint) as *const gchar,
                b"/vol/\0" as *const u8 as *const gchar,
            )
        }) != 0)
        || g_unix_mount_point_is_loopback(mountpoint) != 0
    {
        return ::core::ptr::null_mut::<GUnixVolume>();
    }
    volume = g_object_new(
        safe_c2rust__g_unix_volume_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GUnixVolume;
    (*volume).volume_monitor = (if !volume_monitor.is_null() {
        g_object_ref(volume_monitor as gpointer) as *mut GVolumeMonitor
    } else {
        ::core::ptr::null_mut::<GVolumeMonitor>()
    }) as *mut GVolumeMonitor;
    (*volume).mount_path =
        safe_c2rust_g_strdup_inline(g_unix_mount_point_get_mount_path(mountpoint));
    (*volume).device_path =
        safe_c2rust_g_strdup_inline(g_unix_mount_point_get_device_path(mountpoint));
    (*volume).can_eject = g_unix_mount_point_guess_can_eject(mountpoint);
    (*volume).name = g_unix_mount_point_guess_name(mountpoint);
    (*volume).icon = g_unix_mount_point_guess_icon(mountpoint);
    (*volume).symbolic_icon = g_unix_mount_point_guess_symbolic_icon(mountpoint);
    if strcmp(
        g_unix_mount_point_get_fs_type(mountpoint),
        b"nfs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*volume).identifier_type =
            safe_c2rust_g_strdup_inline(b"nfs-mount\0" as *const u8 as *const ::core::ffi::c_char);
        (*volume).identifier = safe_c2rust_g_strdup_inline((*volume).device_path);
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*volume).device_path;
            let __prefix: *const ::core::ffi::c_char =
                b"LABEL=\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            (*volume).device_path,
            b"LABEL=\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        (*volume).identifier_type =
            safe_c2rust_g_strdup_inline(b"label\0" as *const u8 as *const ::core::ffi::c_char);
        (*volume).identifier = safe_c2rust_g_strdup_inline(
            (*volume)
                .device_path
                .offset(6 as ::core::ffi::c_int as isize),
        );
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*volume).device_path;
            let __prefix: *const ::core::ffi::c_char =
                b"UUID=\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_12
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            (*volume).device_path,
            b"UUID=\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        (*volume).identifier_type =
            safe_c2rust_g_strdup_inline(b"uuid\0" as *const u8 as *const ::core::ffi::c_char);
        (*volume).identifier = safe_c2rust_g_strdup_inline(
            (*volume)
                .device_path
                .offset(5 as ::core::ffi::c_int as isize),
        );
    } else if g_path_is_absolute((*volume).device_path) != 0 {
        (*volume).identifier_type = safe_c2rust_g_strdup_inline(
            b"unix-device\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*volume).identifier = safe_c2rust_g_strdup_inline((*volume).device_path);
    }
    return volume;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_disconnected(mut volume: *mut GUnixVolume) {
    if !(*volume).mount.is_null() {
        _g_unix_mount_unset_volume((*volume).mount, volume);
        (*volume).mount = ::core::ptr::null_mut::<GUnixMount>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_set_mount(
    mut volume: *mut GUnixVolume,
    mut mount: *mut GUnixMount,
) {
    if (*volume).mount == mount {
        return;
    }
    if !(*volume).mount.is_null() {
        _g_unix_mount_unset_volume((*volume).mount, volume);
    }
    (*volume).mount = mount;
    g_signal_emit_by_name(
        volume as gpointer,
        b"changed\0" as *const u8 as *const gchar,
    );
    if !(*volume).volume_monitor.is_null() {
        g_signal_emit_by_name(
            (*volume).volume_monitor as gpointer,
            b"volume-changed\0" as *const u8 as *const gchar,
            volume,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_unset_mount(
    mut volume: *mut GUnixVolume,
    mut mount: *mut GUnixMount,
) {
    if (*volume).mount == mount {
        (*volume).mount = ::core::ptr::null_mut::<GUnixMount>();
        g_signal_emit_by_name(
            volume as gpointer,
            b"changed\0" as *const u8 as *const gchar,
        );
        if !(*volume).volume_monitor.is_null() {
            g_signal_emit_by_name(
                (*volume).volume_monitor as gpointer,
                b"volume-changed\0" as *const u8 as *const gchar,
                volume,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_icon(mut volume: *mut GVolume) -> *mut GIcon {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    return g_object_ref((*unix_volume).icon as gpointer) as *mut GIcon;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_symbolic_icon(
    mut volume: *mut GVolume,
) -> *mut GIcon {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    return g_object_ref((*unix_volume).symbolic_icon as gpointer) as *mut GIcon;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_name(
    mut volume: *mut GVolume,
) -> *mut ::core::ffi::c_char {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    return safe_c2rust_g_strdup_inline((*unix_volume).name);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_uuid(
    mut volume: *mut GVolume,
) -> *mut ::core::ffi::c_char {
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_can_mount(mut volume: *mut GVolume) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_can_eject(mut volume: *mut GVolume) -> gboolean {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    return (*unix_volume).can_eject;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_should_automount(
    mut volume: *mut GVolume,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_drive(mut volume: *mut GVolume) -> *mut GDrive {
    return ::core::ptr::null_mut::<GDrive>();
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_mount(mut volume: *mut GVolume) -> *mut GMount {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    if !(*unix_volume).mount.is_null() {
        return g_object_ref(
            (*unix_volume).mount as *mut ::core::ffi::c_void as *mut GMount as gpointer,
        ) as *mut GMount;
    }
    return ::core::ptr::null_mut::<GMount>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_has_mount_path(
    mut volume: *mut GUnixVolume,
    mut mount_path: *const ::core::ffi::c_char,
) -> gboolean {
    return (strcmp((*volume).mount_path, mount_path) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_eject_mount_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut subprocess: *mut GSubprocess = source as *mut ::core::ffi::c_void as *mut GSubprocess;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut stderr_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut unix_volume: *mut GUnixVolume = ::core::ptr::null_mut::<GUnixVolume>();
    if g_subprocess_communicate_utf8_finish(
        subprocess,
        result,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut stderr_str,
        &raw mut error,
    ) == 0
    {
        g_task_return_error(task, error);
        g_error_free(error);
    } else {
        if g_subprocess_get_successful(subprocess) == 0 {
            g_task_return_new_error_literal(
                task,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                stderr_str,
            );
        } else {
            unix_volume = g_task_get_source_object(task) as *mut GUnixVolume;
            _g_unix_volume_monitor_update(
                (*unix_volume).volume_monitor as *mut ::core::ffi::c_void
                    as *mut GUnixVolumeMonitor,
            );
            g_task_return_boolean(task, TRUE);
        }
        g_free(stderr_str as gpointer);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_eject_mount_do(
    mut volume: *mut GVolume,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut argv: *const *const gchar,
    mut task_name: *const gchar,
) {
    let mut subprocess: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(volume as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolume,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                    *const *const gchar,
                    *const gchar,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_eject_mount_do
                as unsafe extern "C" fn(
                    *mut GVolume,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                    *const *const gchar,
                    *const gchar,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"eject_mount_do\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(_task_0, task_name);
    } else {
        g_task_set_name(_task_0, task_name);
    }
    if g_task_return_error_if_cancelled(task) != 0 {
        g_object_unref(task as gpointer);
        return;
    }
    subprocess = g_subprocess_newv(
        argv,
        (G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int
            | G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int) as GSubprocessFlags,
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixvolume.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            328 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    g_subprocess_communicate_utf8_async(
        subprocess,
        ::core::ptr::null::<::core::ffi::c_char>(),
        g_task_get_cancellable(task),
        Some(
            safe_c2rust_eject_mount_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_mount(
    mut volume: *mut GVolume,
    mut flags: GMountMountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    let mut argv: [*const gchar; 3] = [
        b"mount\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
    ];
    if !(*unix_volume).mount_path.is_null() {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_volume).mount_path;
    } else {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_volume).device_path;
    }
    safe_c2rust_eject_mount_do(
        volume,
        cancellable,
        callback,
        user_data,
        &raw mut argv as *mut *const gchar,
        b"[gio] mount volume\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_mount_finish(
    mut volume: *mut GVolume,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, volume as gpointer) != 0 {
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
            b"g_task_is_valid (result, volume)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_eject(
    mut volume: *mut GVolume,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    let mut argv: [*const gchar; 3] = [
        b"eject\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
    ];
    argv[1 as ::core::ffi::c_int as usize] = (*unix_volume).device_path;
    safe_c2rust_eject_mount_do(
        volume,
        cancellable,
        callback,
        user_data,
        &raw mut argv as *mut *const gchar,
        b"[gio] eject volume\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_eject_finish(
    mut volume: *mut GVolume,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, volume as gpointer) != 0 {
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
            b"g_task_is_valid (result, volume)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_get_identifier(
    mut volume: *mut GVolume,
    mut kind: *const gchar,
) -> *mut gchar {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    if !(*unix_volume).identifier_type.is_null()
        && strcmp(
            kind as *const ::core::ffi::c_char,
            (*unix_volume).identifier_type,
        ) == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_strdup_inline((*unix_volume).identifier) as *mut gchar;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_enumerate_identifiers(
    mut volume: *mut GVolume,
) -> *mut *mut gchar {
    let mut unix_volume: *mut GUnixVolume = volume as *mut ::core::ffi::c_void as *mut GUnixVolume;
    let mut res: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if !(*unix_volume).identifier_type.is_null() {
        res = ({
            let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        let ref mut fresh0 = *res.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = safe_c2rust_g_strdup_inline((*unix_volume).identifier_type) as *mut gchar;
        let ref mut fresh1 = *res.offset(1 as ::core::ffi::c_int as isize);
        *fresh1 = ::core::ptr::null_mut::<gchar>();
    } else {
        res = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        let ref mut fresh2 = *res.offset(0 as ::core::ffi::c_int as isize);
        *fresh2 = ::core::ptr::null_mut::<gchar>();
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_volume_iface_init(mut iface: *mut GVolumeIface) {
    (*iface).get_name = Some(
        safe_c2rust_g_unix_volume_get_name
            as unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char>;
    (*iface).get_icon = Some(
        safe_c2rust_g_unix_volume_get_icon as unsafe extern "C" fn(*mut GVolume) -> *mut GIcon,
    ) as Option<unsafe extern "C" fn(*mut GVolume) -> *mut GIcon>;
    (*iface).get_symbolic_icon = Some(
        safe_c2rust_g_unix_volume_get_symbolic_icon
            as unsafe extern "C" fn(*mut GVolume) -> *mut GIcon,
    ) as Option<unsafe extern "C" fn(*mut GVolume) -> *mut GIcon>;
    (*iface).get_uuid = Some(
        safe_c2rust_g_unix_volume_get_uuid
            as unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GVolume) -> *mut ::core::ffi::c_char>;
    (*iface).get_drive = Some(
        safe_c2rust_g_unix_volume_get_drive as unsafe extern "C" fn(*mut GVolume) -> *mut GDrive,
    ) as Option<unsafe extern "C" fn(*mut GVolume) -> *mut GDrive>;
    (*iface).get_mount = Some(
        safe_c2rust_g_unix_volume_get_mount as unsafe extern "C" fn(*mut GVolume) -> *mut GMount,
    ) as Option<unsafe extern "C" fn(*mut GVolume) -> *mut GMount>;
    (*iface).can_mount =
        Some(safe_c2rust_g_unix_volume_can_mount as unsafe extern "C" fn(*mut GVolume) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>;
    (*iface).can_eject =
        Some(safe_c2rust_g_unix_volume_can_eject as unsafe extern "C" fn(*mut GVolume) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>;
    (*iface).should_automount = Some(
        safe_c2rust_g_unix_volume_should_automount
            as unsafe extern "C" fn(*mut GVolume) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GVolume) -> gboolean>;
    (*iface).mount_fn = Some(
        safe_c2rust_g_unix_volume_mount
            as unsafe extern "C" fn(
                *mut GVolume,
                GMountMountFlags,
                *mut GMountOperation,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GVolume,
                GMountMountFlags,
                *mut GMountOperation,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).mount_finish = Some(
        safe_c2rust_g_unix_volume_mount_finish
            as unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).eject = Some(
        safe_c2rust_g_unix_volume_eject
            as unsafe extern "C" fn(
                *mut GVolume,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GVolume,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).eject_finish = Some(
        safe_c2rust_g_unix_volume_eject_finish
            as unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GVolume, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).get_identifier = Some(
        safe_c2rust_g_unix_volume_get_identifier
            as unsafe extern "C" fn(*mut GVolume, *const gchar) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GVolume,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char,
        >;
    (*iface).enumerate_identifiers = Some(
        safe_c2rust_g_unix_volume_enumerate_identifiers
            as unsafe extern "C" fn(*mut GVolume) -> *mut *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GVolume) -> *mut *mut ::core::ffi::c_char>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
