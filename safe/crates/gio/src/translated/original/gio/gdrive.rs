use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GDrive;
    pub type _GIcon;
    pub type _GMountOperation;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
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
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_themed_icon_new_with_default_fallbacks(iconname: *const ::core::ffi::c_char)
        -> *mut GIcon;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GMountUnmountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_UNMOUNT_FORCE: GMountUnmountFlags = 1;
pub const G_MOUNT_UNMOUNT_NONE: GMountUnmountFlags = 0;
pub type GDriveStartFlags = ::core::ffi::c_uint;
pub const G_DRIVE_START_NONE: GDriveStartFlags = 0;
pub type GDriveStartStopType = ::core::ffi::c_uint;
pub const G_DRIVE_START_STOP_TYPE_PASSWORD: GDriveStartStopType = 4;
pub const G_DRIVE_START_STOP_TYPE_MULTIDISK: GDriveStartStopType = 3;
pub const G_DRIVE_START_STOP_TYPE_NETWORK: GDriveStartStopType = 2;
pub const G_DRIVE_START_STOP_TYPE_SHUTDOWN: GDriveStartStopType = 1;
pub const G_DRIVE_START_STOP_TYPE_UNKNOWN: GDriveStartStopType = 0;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GDrive = _GDrive;
pub type GIcon = _GIcon;
pub type GMountOperation = _GMountOperation;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDriveIface {
    pub g_iface: GTypeInterface,
    pub changed: Option<unsafe extern "C" fn(*mut GDrive) -> ()>,
    pub disconnected: Option<unsafe extern "C" fn(*mut GDrive) -> ()>,
    pub eject_button: Option<unsafe extern "C" fn(*mut GDrive) -> ()>,
    pub get_name: Option<unsafe extern "C" fn(*mut GDrive) -> *mut ::core::ffi::c_char>,
    pub get_icon: Option<unsafe extern "C" fn(*mut GDrive) -> *mut GIcon>,
    pub has_volumes: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub get_volumes: Option<unsafe extern "C" fn(*mut GDrive) -> *mut GList>,
    pub is_media_removable: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub has_media: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub is_media_check_automatic: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub can_eject: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub can_poll_for_media: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub eject: Option<
        unsafe extern "C" fn(
            *mut GDrive,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_finish:
        Option<unsafe extern "C" fn(*mut GDrive, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub poll_for_media: Option<
        unsafe extern "C" fn(*mut GDrive, *mut GCancellable, GAsyncReadyCallback, gpointer) -> (),
    >,
    pub poll_for_media_finish:
        Option<unsafe extern "C" fn(*mut GDrive, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub get_identifier: Option<
        unsafe extern "C" fn(*mut GDrive, *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
    >,
    pub enumerate_identifiers:
        Option<unsafe extern "C" fn(*mut GDrive) -> *mut *mut ::core::ffi::c_char>,
    pub get_start_stop_type: Option<unsafe extern "C" fn(*mut GDrive) -> GDriveStartStopType>,
    pub can_start: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub can_start_degraded: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub start: Option<
        unsafe extern "C" fn(
            *mut GDrive,
            GDriveStartFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub start_finish:
        Option<unsafe extern "C" fn(*mut GDrive, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub can_stop: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
    pub stop: Option<
        unsafe extern "C" fn(
            *mut GDrive,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub stop_finish:
        Option<unsafe extern "C" fn(*mut GDrive, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub stop_button: Option<unsafe extern "C" fn(*mut GDrive) -> ()>,
    pub eject_with_operation: Option<
        unsafe extern "C" fn(
            *mut GDrive,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GDrive, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub get_sort_key: Option<unsafe extern "C" fn(*mut GDrive) -> *const gchar>,
    pub get_symbolic_icon: Option<unsafe extern "C" fn(*mut GDrive) -> *mut GIcon>,
    pub is_removable: Option<unsafe extern "C" fn(*mut GDrive) -> gboolean>,
}
pub type GDriveIface = _GDriveIface;
pub type GDriveInterface = GDriveIface;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GDrive\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDriveInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDriveInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_drive_default_init
                        as unsafe extern "C" fn(*mut GDriveInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_g_drive_default_init(mut iface: *mut GDriveInterface) {
    g_signal_new(
        g_intern_static_string(b"changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_drive_get_type(),
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    g_signal_new(
        g_intern_static_string(b"disconnected\0" as *const u8 as *const gchar),
        safe_c2rust_g_drive_get_type(),
        G_SIGNAL_RUN_LAST,
        24 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    g_signal_new(
        g_intern_static_string(b"eject-button\0" as *const u8 as *const gchar),
        safe_c2rust_g_drive_get_type(),
        G_SIGNAL_RUN_LAST,
        32 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    g_signal_new(
        g_intern_static_string(b"stop-button\0" as *const u8 as *const gchar),
        safe_c2rust_g_drive_get_type(),
        G_SIGNAL_RUN_LAST,
        224 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_name(
    mut drive: *mut GDrive,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).get_name.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_icon(mut drive: *mut GDrive) -> *mut GIcon {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).get_icon.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_symbolic_icon(
    mut drive: *mut GDrive,
) -> *mut GIcon {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    let mut ret: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).get_symbolic_icon.is_some() {
        ret = (*iface)
            .get_symbolic_icon
            .expect("non-null function pointer")(drive);
    } else {
        ret = g_themed_icon_new_with_default_fallbacks(
            b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_has_volumes(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).has_volumes.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_volumes(mut drive: *mut GDrive) -> *mut GList {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).get_volumes.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_is_media_check_automatic(
    mut drive: *mut GDrive,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some(
        (*iface)
            .is_media_check_automatic
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_is_removable(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).is_removable.is_some() {
        return (*iface).is_removable.expect("non-null function pointer")(drive);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_is_media_removable(
    mut drive: *mut GDrive,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some(
        (*iface)
            .is_media_removable
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_has_media(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).has_media.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_can_eject(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).can_eject.is_none() {
        return FALSE;
    }
    return Some((*iface).can_eject.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_can_poll_for_media(
    mut drive: *mut GDrive,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).poll_for_media.is_none() {
        return FALSE;
    }
    return Some(
        (*iface)
            .can_poll_for_media
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_eject(
    mut drive: *mut GDrive,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).eject.is_none() {
        g_task_report_new_error(
            drive as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDrive,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_drive_eject_with_operation
                    as unsafe extern "C" fn(
                        *mut GDrive,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"drive doesn\xE2\x80\x99t implement eject\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).eject.expect("non-null function pointer")).expect("non-null function pointer")(
        drive,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_eject_finish(
    mut drive: *mut GDrive,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDrive,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_drive_eject_with_operation
                as unsafe extern "C" fn(
                    *mut GDrive,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).eject_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_eject_with_operation(
    mut drive: *mut GDrive,
    mut flags: GMountUnmountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).eject.is_none() && (*iface).eject_with_operation.is_none() {
        g_task_report_new_error(
            drive as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDrive,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_drive_eject_with_operation
                    as unsafe extern "C" fn(
                        *mut GDrive,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"drive doesn\xE2\x80\x99t implement eject or eject_with_operation\0" as *const u8
                    as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*iface).eject_with_operation.is_some() {
        Some(
            (*iface)
                .eject_with_operation
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            drive,
            flags,
            mount_operation,
            cancellable,
            callback,
            user_data,
        );
    } else {
        Some((*iface).eject.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            drive, flags, cancellable, callback, user_data
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_eject_with_operation_finish(
    mut drive: *mut GDrive,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDrive,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_drive_eject_with_operation
                as unsafe extern "C" fn(
                    *mut GDrive,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).eject_with_operation_finish.is_some() {
        return Some(
            (*iface)
                .eject_with_operation_finish
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(drive, result, error);
    } else {
        return Some((*iface).eject_finish.expect("non-null function pointer"))
            .expect("non-null function pointer")(drive, result, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_poll_for_media(
    mut drive: *mut GDrive,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).poll_for_media.is_none() {
        g_task_report_new_error(
            drive as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDrive,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_drive_poll_for_media
                    as unsafe extern "C" fn(
                        *mut GDrive,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"drive doesn\xE2\x80\x99t implement polling for media\0" as *const u8
                    as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).poll_for_media.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_poll_for_media_finish(
    mut drive: *mut GDrive,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDrive,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_drive_poll_for_media
                as unsafe extern "C" fn(
                    *mut GDrive,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some(
        (*iface)
            .poll_for_media_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_identifier(
    mut drive: *mut GDrive,
    mut kind: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !kind.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"kind != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).get_identifier.is_none() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return Some((*iface).get_identifier.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive, kind);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_enumerate_identifiers(
    mut drive: *mut GDrive,
) -> *mut *mut ::core::ffi::c_char {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).enumerate_identifiers.is_none() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    return Some(
        (*iface)
            .enumerate_identifiers
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_start_stop_type(
    mut drive: *mut GDrive,
) -> GDriveStartStopType {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DRIVE_START_STOP_TYPE_UNKNOWN;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).get_start_stop_type.is_none() {
        return G_DRIVE_START_STOP_TYPE_UNKNOWN;
    }
    return Some(
        (*iface)
            .get_start_stop_type
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_can_start(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).can_start.is_none() {
        return FALSE;
    }
    return Some((*iface).can_start.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_can_start_degraded(
    mut drive: *mut GDrive,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).can_start_degraded.is_none() {
        return FALSE;
    }
    return Some(
        (*iface)
            .can_start_degraded
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_start(
    mut drive: *mut GDrive,
    mut flags: GDriveStartFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).start.is_none() {
        g_task_report_new_error(
            drive as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDrive,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_drive_start
                    as unsafe extern "C" fn(
                        *mut GDrive,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"drive doesn\xE2\x80\x99t implement start\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).start.expect("non-null function pointer")).expect("non-null function pointer")(
        drive,
        flags,
        mount_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_start_finish(
    mut drive: *mut GDrive,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDrive,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_drive_start
                as unsafe extern "C" fn(
                    *mut GDrive,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).start_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_can_stop(mut drive: *mut GDrive) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).can_stop.is_none() {
        return FALSE;
    }
    return Some((*iface).can_stop.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_stop(
    mut drive: *mut GDrive,
    mut flags: GMountUnmountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).stop.is_none() {
        g_task_report_new_error(
            drive as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDrive,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_drive_start
                    as unsafe extern "C" fn(
                        *mut GDrive,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"drive doesn\xE2\x80\x99t implement stop\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).stop.expect("non-null function pointer")).expect("non-null function pointer")(
        drive,
        flags,
        mount_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_stop_finish(
    mut drive: *mut GDrive,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDrive,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_drive_start
                as unsafe extern "C" fn(
                    *mut GDrive,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    return Some((*iface).stop_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(drive, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_get_sort_key(mut drive: *mut GDrive) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    let mut iface: *mut GDriveIface = ::core::ptr::null_mut::<GDriveIface>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = drive as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_drive_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DRIVE (drive)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    iface = g_type_interface_peek(
        (*(drive as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_drive_get_type(),
    ) as *mut GDriveIface;
    if (*iface).get_sort_key.is_some() {
        ret = (*iface).get_sort_key.expect("non-null function pointer")(drive);
    }
    return ret;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
