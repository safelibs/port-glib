extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_freeze_notify(object: *mut GObject);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_thaw_notify(object: *mut GObject);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_io_error_quark() -> GQuark;
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
}
pub type guint8 = ::core::ffi::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPermission {
    pub parent_instance: GObject,
    pub priv_0: *mut GPermissionPrivate,
}
pub type GPermissionPrivate = _GPermissionPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPermissionPrivate {
    pub allowed: gboolean,
    pub can_acquire: gboolean,
    pub can_release: gboolean,
}
pub type GPermission = _GPermission;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPermissionClass {
    pub parent_class: GObjectClass,
    pub acquire: Option<
        unsafe extern "C" fn(*mut GPermission, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub acquire_async: Option<
        unsafe extern "C" fn(
            *mut GPermission,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub acquire_finish: Option<
        unsafe extern "C" fn(*mut GPermission, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub release: Option<
        unsafe extern "C" fn(*mut GPermission, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub release_async: Option<
        unsafe extern "C" fn(
            *mut GPermission,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub release_finish: Option<
        unsafe extern "C" fn(*mut GPermission, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub reserved: [gpointer; 16],
}
pub type GPermissionClass = _GPermissionClass;
pub const PROP_CAN_RELEASE: C2RustUnnamed_1 = 3;
pub const PROP_CAN_ACQUIRE: C2RustUnnamed_1 = 2;
pub const PROP_ALLOWED: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_GPermission_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_permission_get_instance_private(
    mut self_0: *mut GPermission,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GPermission_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_permission_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_permission_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GPermission\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GPermissionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_permission_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GPermission>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPermission) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_permission_init as unsafe extern "C" fn(*mut GPermission) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GPermission_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GPermissionPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_permission_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_permission_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GPermission_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GPermission_private_offset);
    }
    safe_c2rust_g_permission_class_init(klass as *mut GPermissionClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_permission_get_type_once();
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_acquire(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .acquire
        .expect("non-null function pointer")(permission, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_acquire_async(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .acquire_async
        .expect("non-null function pointer")(permission, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_acquire_finish(
    mut permission: *mut GPermission,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .acquire_finish
        .expect("non-null function pointer")(permission, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_release(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .release
        .expect("non-null function pointer")(permission, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_release_async(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .release_async
        .expect("non-null function pointer")(permission, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_release_finish(
    mut permission: *mut GPermission,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(permission as *mut GTypeInstance)).g_class as *mut GPermissionClass))
        .release_finish
        .expect("non-null function pointer")(permission, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_get_allowed(
    mut permission: *mut GPermission,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*permission).priv_0).allowed;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_get_can_acquire(
    mut permission: *mut GPermission,
) -> gboolean {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*permission).priv_0).can_acquire;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_get_can_release(
    mut permission: *mut GPermission,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*permission).priv_0).can_release;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_permission_impl_update(
    mut permission: *mut GPermission,
    mut allowed: gboolean,
    mut can_acquire: gboolean,
    mut can_release: gboolean,
) {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = permission as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_permission_get_type();
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
            b"G_IS_PERMISSION (permission)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    object = permission as *mut ::core::ffi::c_void as *mut GObject;
    g_object_freeze_notify(object);
    allowed = (allowed != FALSE) as ::core::ffi::c_int as gboolean;
    if allowed != (*(*permission).priv_0).allowed {
        (*(*permission).priv_0).allowed = allowed;
        g_object_notify(object, b"allowed\0" as *const u8 as *const gchar);
    }
    can_acquire = (can_acquire != FALSE) as ::core::ffi::c_int as gboolean;
    if can_acquire != (*(*permission).priv_0).can_acquire {
        (*(*permission).priv_0).can_acquire = can_acquire;
        g_object_notify(object, b"can-acquire\0" as *const u8 as *const gchar);
    }
    can_release = (can_release != FALSE) as ::core::ffi::c_int as gboolean;
    if can_release != (*(*permission).priv_0).can_release {
        (*(*permission).priv_0).can_release = can_release;
        g_object_notify(object, b"can-release\0" as *const u8 as *const gchar);
    }
    g_object_thaw_notify(object);
}
unsafe extern "C" fn safe_c2rust_g_permission_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut permission: *mut GPermission = object as *mut ::core::ffi::c_void as *mut GPermission;
    match prop_id {
        1 => {
            g_value_set_boolean(value, (*(*permission).priv_0).allowed);
        }
        2 => {
            g_value_set_boolean(value, (*(*permission).priv_0).can_acquire);
        }
        3 => {
            g_value_set_boolean(value, (*(*permission).priv_0).can_release);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpermission.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                373 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_permission_init(mut permission: *mut GPermission) {
    (*permission).priv_0 =
        safe_c2rust_g_permission_get_instance_private(permission) as *mut GPermissionPrivate;
}
unsafe extern "C" fn safe_c2rust_acquire_or_release(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"Can't acquire or release permission\0" as *const u8 as *const gchar,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_acquire_or_release_async(
    mut permission: *mut GPermission,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_task_report_new_error(
        permission as gpointer,
        callback,
        user_data,
        NULL,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"Can't acquire or release permission\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_acquire_or_release_finish(
    mut permission: *mut GPermission,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_permission_class_init(mut class: *mut GPermissionClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_permission_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*class).acquire = Some(
        safe_c2rust_acquire_or_release
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GPermission, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*class).release = Some(
        safe_c2rust_acquire_or_release
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GPermission, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*class).acquire_async = Some(
        safe_c2rust_acquire_or_release_async
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*class).release_async = Some(
        safe_c2rust_acquire_or_release_async
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPermission,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*class).acquire_finish = Some(
        safe_c2rust_acquire_or_release_finish
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GPermission, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*class).release_finish = Some(
        safe_c2rust_acquire_or_release_finish
            as unsafe extern "C" fn(
                *mut GPermission,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GPermission, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    g_object_class_install_property(
        object_class,
        PROP_ALLOWED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"allowed\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_READABLE as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_CAN_ACQUIRE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"can-acquire\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_READABLE as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_CAN_RELEASE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"can-release\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_READABLE as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
