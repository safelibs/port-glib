extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
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
    pub type _GUnixVolume;
    pub type _GUnixMountEntry;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_source_unref(source: *mut GSource);
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
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
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_io_error_quark() -> GQuark;
    fn g_mount_get_type() -> GType;
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
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_attach_source(task: *mut GTask, source: *mut GSource, callback: GSourceFunc);
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
    fn g_volume_get_drive(volume: *mut GVolume) -> *mut GDrive;
    fn g_unix_mount_get_mount_path(mount_entry: *mut GUnixMountEntry)
        -> *const ::core::ffi::c_char;
    fn g_unix_mount_get_device_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_guess_can_eject(mount_entry: *mut GUnixMountEntry) -> gboolean;
    fn g_unix_mount_guess_should_display(mount_entry: *mut GUnixMountEntry) -> gboolean;
    fn g_unix_mount_guess_name(mount_entry: *mut GUnixMountEntry) -> *mut ::core::ffi::c_char;
    fn g_unix_mount_guess_icon(mount_entry: *mut GUnixMountEntry) -> *mut GIcon;
    fn g_unix_mount_guess_symbolic_icon(mount_entry: *mut GUnixMountEntry) -> *mut GIcon;
    fn _g_unix_volume_set_mount(volume: *mut GUnixVolume, mount: *mut GUnixMount);
    fn _g_unix_volume_unset_mount(volume: *mut GUnixVolume, mount: *mut GUnixMount);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMount {
    pub parent: GObject,
    pub volume_monitor: *mut GVolumeMonitor,
    pub volume: *mut GUnixVolume,
    pub name: *mut ::core::ffi::c_char,
    pub icon: *mut GIcon,
    pub symbolic_icon: *mut GIcon,
    pub device_path: *mut ::core::ffi::c_char,
    pub mount_path: *mut ::core::ffi::c_char,
    pub can_eject: gboolean,
}
pub type GUnixVolume = _GUnixVolume;
pub type GUnixMount = _GUnixMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountIface {
    pub g_iface: GTypeInterface,
    pub changed: Option<unsafe extern "C" fn(*mut GMount) -> ()>,
    pub unmounted: Option<unsafe extern "C" fn(*mut GMount) -> ()>,
    pub get_root: Option<unsafe extern "C" fn(*mut GMount) -> *mut GFile>,
    pub get_name: Option<unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char>,
    pub get_icon: Option<unsafe extern "C" fn(*mut GMount) -> *mut GIcon>,
    pub get_uuid: Option<unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char>,
    pub get_volume: Option<unsafe extern "C" fn(*mut GMount) -> *mut GVolume>,
    pub get_drive: Option<unsafe extern "C" fn(*mut GMount) -> *mut GDrive>,
    pub can_unmount: Option<unsafe extern "C" fn(*mut GMount) -> gboolean>,
    pub can_eject: Option<unsafe extern "C" fn(*mut GMount) -> gboolean>,
    pub unmount: Option<
        unsafe extern "C" fn(
            *mut GMount,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_finish:
        Option<unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject: Option<
        unsafe extern "C" fn(
            *mut GMount,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_finish:
        Option<unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub remount: Option<
        unsafe extern "C" fn(
            *mut GMount,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub remount_finish:
        Option<unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub guess_content_type: Option<
        unsafe extern "C" fn(
            *mut GMount,
            gboolean,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub guess_content_type_finish: Option<
        unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> *mut *mut gchar,
    >,
    pub guess_content_type_sync: Option<
        unsafe extern "C" fn(
            *mut GMount,
            gboolean,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut *mut gchar,
    >,
    pub pre_unmount: Option<unsafe extern "C" fn(*mut GMount) -> ()>,
    pub unmount_with_operation: Option<
        unsafe extern "C" fn(
            *mut GMount,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_with_operation: Option<
        unsafe extern "C" fn(
            *mut GMount,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub get_default_location: Option<unsafe extern "C" fn(*mut GMount) -> *mut GFile>,
    pub get_sort_key: Option<unsafe extern "C" fn(*mut GMount) -> *const gchar>,
    pub get_symbolic_icon: Option<unsafe extern "C" fn(*mut GMount) -> *mut GIcon>,
}
pub type GMountIface = _GMountIface;
pub type GUnixMountEntry = _GUnixMountEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMountClass {
    pub parent_class: GObjectClass,
}
pub type GUnixMountClass = _GUnixMountClass;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
unsafe extern "C" fn safe_c2rust_g_unix_mount_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_mount_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixMount_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GUnixMount_private_offset);
    }
    safe_c2rust_g_unix_mount_class_init(klass as *mut GUnixMountClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_mount_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_mount_get_type_once();
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
static mut safe_c2rust_GUnixMount_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GUnixMount\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixMountClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_mount_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixMount>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixMount) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_mount_init as unsafe extern "C" fn(*mut GUnixMount) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GMountIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_unix_mount_mount_iface_init
                as unsafe extern "C" fn(*mut GMountIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_mount_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_unix_mount_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_unix_mount_finalize(mut object: *mut GObject) {
    let mut mount: *mut GUnixMount = ::core::ptr::null_mut::<GUnixMount>();
    mount = object as *mut ::core::ffi::c_void as *mut GUnixMount;
    if !(*mount).volume_monitor.is_null() {
        g_object_unref((*mount).volume_monitor as gpointer);
    }
    if !(*mount).volume.is_null() {
        _g_unix_volume_unset_mount((*mount).volume, mount);
    }
    g_object_unref((*mount).icon as gpointer);
    g_object_unref((*mount).symbolic_icon as gpointer);
    g_free((*mount).name as gpointer);
    g_free((*mount).device_path as gpointer);
    g_free((*mount).mount_path as gpointer);
    (*(safe_c2rust_g_unix_mount_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_class_init(mut klass: *mut GUnixMountClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_unix_mount_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_init(mut unix_mount: *mut GUnixMount) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_mount_new(
    mut volume_monitor: *mut GVolumeMonitor,
    mut mount_entry: *mut GUnixMountEntry,
    mut volume: *mut GUnixVolume,
) -> *mut GUnixMount {
    let mut mount: *mut GUnixMount = ::core::ptr::null_mut::<GUnixMount>();
    if volume.is_null() && g_unix_mount_guess_should_display(mount_entry) == 0 {
        return ::core::ptr::null_mut::<GUnixMount>();
    }
    mount = g_object_new(
        safe_c2rust__g_unix_mount_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GUnixMount;
    (*mount).volume_monitor = (if !volume_monitor.is_null() {
        g_object_ref(volume_monitor as gpointer) as *mut GVolumeMonitor
    } else {
        ::core::ptr::null_mut::<GVolumeMonitor>()
    }) as *mut GVolumeMonitor;
    (*mount).device_path = safe_c2rust_g_strdup_inline(g_unix_mount_get_device_path(mount_entry));
    (*mount).mount_path = safe_c2rust_g_strdup_inline(g_unix_mount_get_mount_path(mount_entry));
    (*mount).can_eject = g_unix_mount_guess_can_eject(mount_entry);
    (*mount).name = g_unix_mount_guess_name(mount_entry);
    (*mount).icon = g_unix_mount_guess_icon(mount_entry);
    (*mount).symbolic_icon = g_unix_mount_guess_symbolic_icon(mount_entry);
    (*mount).volume = volume;
    if !volume.is_null() {
        _g_unix_volume_set_mount(volume, mount);
    }
    return mount;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_mount_unmounted(mut mount: *mut GUnixMount) {
    if !(*mount).volume.is_null() {
        _g_unix_volume_unset_mount((*mount).volume, mount);
        (*mount).volume = ::core::ptr::null_mut::<GUnixVolume>();
        g_signal_emit_by_name(mount as gpointer, b"changed\0" as *const u8 as *const gchar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_mount_unset_volume(
    mut mount: *mut GUnixMount,
    mut volume: *mut GUnixVolume,
) {
    if (*mount).volume == volume {
        (*mount).volume = ::core::ptr::null_mut::<GUnixVolume>();
        g_signal_emit_by_name(mount as gpointer, b"changed\0" as *const u8 as *const gchar);
        if !(*mount).volume_monitor.is_null() {
            g_signal_emit_by_name(
                (*mount).volume_monitor as gpointer,
                b"mount-changed\0" as *const u8 as *const gchar,
                mount,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_root(mut mount: *mut GMount) -> *mut GFile {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    return g_file_new_for_path((*unix_mount).mount_path);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_icon(mut mount: *mut GMount) -> *mut GIcon {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    return g_object_ref((*unix_mount).icon as gpointer) as *mut GIcon;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_symbolic_icon(
    mut mount: *mut GMount,
) -> *mut GIcon {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    return g_object_ref((*unix_mount).symbolic_icon as gpointer) as *mut GIcon;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_uuid(
    mut mount: *mut GMount,
) -> *mut ::core::ffi::c_char {
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_name(
    mut mount: *mut GMount,
) -> *mut ::core::ffi::c_char {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    return safe_c2rust_g_strdup_inline((*unix_mount).name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_mount_has_mount_path(
    mut mount: *mut GUnixMount,
    mut mount_path: *const ::core::ffi::c_char,
) -> gboolean {
    return (strcmp((*mount).mount_path, mount_path) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_drive(mut mount: *mut GMount) -> *mut GDrive {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    if !(*unix_mount).volume.is_null() {
        return g_volume_get_drive(
            (*unix_mount).volume as *mut ::core::ffi::c_void as *mut GVolume,
        );
    }
    return ::core::ptr::null_mut::<GDrive>();
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_get_volume(mut mount: *mut GMount) -> *mut GVolume {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    if !(*unix_mount).volume.is_null() {
        return g_object_ref((*unix_mount).volume as gpointer) as *mut GUnixVolume
            as *mut ::core::ffi::c_void as *mut GVolume;
    }
    return ::core::ptr::null_mut::<GVolume>();
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_can_unmount(mut mount: *mut GMount) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_can_eject(mut mount: *mut GMount) -> gboolean {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    return (*unix_mount).can_eject;
}
unsafe extern "C" fn safe_c2rust_eject_unmount_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut subprocess: *mut GSubprocess = source as *mut ::core::ffi::c_void as *mut GSubprocess;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut stderr_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
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
            g_task_return_boolean(task, TRUE);
        }
        g_free(stderr_str as gpointer);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_eject_unmount_do_cb(mut user_data: gpointer) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut subprocess: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut argv: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    argv = g_task_get_task_data(task) as *mut *const gchar;
    if g_task_return_error_if_cancelled(task) != 0 {
        g_object_unref(task as gpointer);
        return G_SOURCE_REMOVE;
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixmount.c\0" as *const u8
                as *const ::core::ffi::c_char,
            293 as ::core::ffi::c_int,
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
            safe_c2rust_eject_unmount_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_eject_unmount_do(
    mut mount: *mut GMount,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut task_name: *const gchar,
) {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut timeout: *mut GSource = ::core::ptr::null_mut::<GSource>();
    task = g_task_new(mount as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GMount,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                    *mut *mut ::core::ffi::c_char,
                    *const gchar,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_eject_unmount_do
                as unsafe extern "C" fn(
                    *mut GMount,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                    *mut *mut ::core::ffi::c_char,
                    *const gchar,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"eject_unmount_do\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(_task_0, task_name);
    } else {
        g_task_set_name(_task_0, task_name);
    }
    g_task_set_task_data(
        task,
        g_strdupv(argv as *mut *mut gchar) as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut *mut gchar) -> ()>, GDestroyNotify>(
            Some(g_strfreev as unsafe extern "C" fn(*mut *mut gchar) -> ()),
        ),
    );
    if !(*unix_mount).volume_monitor.is_null() {
        g_signal_emit_by_name(
            (*unix_mount).volume_monitor as gpointer,
            b"mount-pre-unmount\0" as *const u8 as *const gchar,
            mount,
        );
    }
    g_signal_emit_by_name(
        mount as gpointer,
        b"pre-unmount\0" as *const u8 as *const gchar,
        0 as ::core::ffi::c_int,
    );
    timeout = g_timeout_source_new(500 as guint);
    g_task_attach_source(
        task,
        timeout,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gboolean>, GSourceFunc>(
            Some(safe_c2rust_eject_unmount_do_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        ),
    );
    g_source_unref(timeout);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_unmount(
    mut mount: *mut GMount,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    let mut argv: [*mut ::core::ffi::c_char; 3] = [
        b"umount\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    if !(*unix_mount).mount_path.is_null() {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_mount).mount_path;
    } else {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_mount).device_path;
    }
    safe_c2rust_eject_unmount_do(
        mount,
        cancellable,
        callback,
        user_data,
        &raw mut argv as *mut *mut ::core::ffi::c_char,
        b"[gio] unmount mount\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_unmount_finish(
    mut mount: *mut GMount,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_eject(
    mut mount: *mut GMount,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut unix_mount: *mut GUnixMount = mount as *mut ::core::ffi::c_void as *mut GUnixMount;
    let mut argv: [*mut ::core::ffi::c_char; 3] = [
        b"eject\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    if !(*unix_mount).mount_path.is_null() {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_mount).mount_path;
    } else {
        argv[1 as ::core::ffi::c_int as usize] = (*unix_mount).device_path;
    }
    safe_c2rust_eject_unmount_do(
        mount,
        cancellable,
        callback,
        user_data,
        &raw mut argv as *mut *mut ::core::ffi::c_char,
        b"[gio] eject mount\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_eject_finish(
    mut mount: *mut GMount,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_mount_iface_init(mut iface: *mut GMountIface) {
    (*iface).get_root =
        Some(safe_c2rust_g_unix_mount_get_root as unsafe extern "C" fn(*mut GMount) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GMount) -> *mut GFile>;
    (*iface).get_name = Some(
        safe_c2rust_g_unix_mount_get_name
            as unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char>;
    (*iface).get_icon =
        Some(safe_c2rust_g_unix_mount_get_icon as unsafe extern "C" fn(*mut GMount) -> *mut GIcon)
            as Option<unsafe extern "C" fn(*mut GMount) -> *mut GIcon>;
    (*iface).get_symbolic_icon = Some(
        safe_c2rust_g_unix_mount_get_symbolic_icon
            as unsafe extern "C" fn(*mut GMount) -> *mut GIcon,
    ) as Option<unsafe extern "C" fn(*mut GMount) -> *mut GIcon>;
    (*iface).get_uuid = Some(
        safe_c2rust_g_unix_mount_get_uuid
            as unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GMount) -> *mut ::core::ffi::c_char>;
    (*iface).get_drive = Some(
        safe_c2rust_g_unix_mount_get_drive as unsafe extern "C" fn(*mut GMount) -> *mut GDrive,
    ) as Option<unsafe extern "C" fn(*mut GMount) -> *mut GDrive>;
    (*iface).get_volume = Some(
        safe_c2rust_g_unix_mount_get_volume as unsafe extern "C" fn(*mut GMount) -> *mut GVolume,
    ) as Option<unsafe extern "C" fn(*mut GMount) -> *mut GVolume>;
    (*iface).can_unmount =
        Some(safe_c2rust_g_unix_mount_can_unmount as unsafe extern "C" fn(*mut GMount) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GMount) -> gboolean>;
    (*iface).can_eject =
        Some(safe_c2rust_g_unix_mount_can_eject as unsafe extern "C" fn(*mut GMount) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GMount) -> gboolean>;
    (*iface).unmount = Some(
        safe_c2rust_g_unix_mount_unmount
            as unsafe extern "C" fn(
                *mut GMount,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMount,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).unmount_finish = Some(
        safe_c2rust_g_unix_mount_unmount_finish
            as unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).eject = Some(
        safe_c2rust_g_unix_mount_eject
            as unsafe extern "C" fn(
                *mut GMount,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMount,
                GMountUnmountFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).eject_finish = Some(
        safe_c2rust_g_unix_mount_eject_finish
            as unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GMount, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
