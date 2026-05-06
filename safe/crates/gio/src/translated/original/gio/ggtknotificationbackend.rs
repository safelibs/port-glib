extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GApplicationPrivate;
    pub type _GNotification;
    pub type _GIOExtension;
    pub type _GDBusConnection;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_unref(object: gpointer);
    fn g_notification_backend_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_sync(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_application_get_application_id(application: *mut GApplication) -> *const gchar;
    fn g_notification_serialize(notification: *mut GNotification) -> *mut GVariant;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplication {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationPrivate,
}
pub type GApplicationPrivate = _GApplicationPrivate;
pub type GApplication = _GApplication;
pub type GNotification = _GNotification;
pub type GIOExtension = _GIOExtension;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotificationBackend {
    pub parent_instance: GObject,
    pub application: *mut GApplication,
    pub dbus_connection: *mut GDBusConnection,
}
pub type GNotificationBackend = _GNotificationBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotificationBackendClass {
    pub parent_class: GObjectClass,
    pub is_supported: Option<unsafe extern "C" fn() -> gboolean>,
    pub send_notification: Option<
        unsafe extern "C" fn(*mut GNotificationBackend, *const gchar, *mut GNotification) -> (),
    >,
    pub withdraw_notification:
        Option<unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> ()>,
}
pub type GNotificationBackendClass = _GNotificationBackendClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GGtkNotificationBackend {
    pub parent: GNotificationBackend,
}
pub type GGtkNotificationBackend = _GGtkNotificationBackend;
pub type GGtkNotificationBackendClass = GNotificationBackendClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_TYPE_UNIT: *const GVariantType =
    b"()\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_gtk_notification_backend_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_notification_backend_get_type(),
        g_intern_static_string(b"GGtkNotificationBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GGtkNotificationBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_gtk_notification_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GGtkNotificationBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GGtkNotificationBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_gtk_notification_backend_init
                    as unsafe extern "C" fn(*mut GGtkNotificationBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gnotification-backend\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"gtk\0" as *const u8 as *const ::core::ffi::c_char,
        100 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_gtk_notification_backend_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_gtk_notification_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GGtkNotificationBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GGtkNotificationBackend_private_offset,
        );
    }
    safe_c2rust_g_gtk_notification_backend_class_init(klass as *mut GGtkNotificationBackendClass);
}
static mut safe_c2rust_GGtkNotificationBackend_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_is_supported() -> gboolean {
    let mut session_bus: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    session_bus = g_bus_get_sync(
        G_BUS_TYPE_SESSION,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if session_bus.is_null() {
        return FALSE;
    }
    reply = g_dbus_connection_call_sync(
        session_bus,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"GetNameOwner\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s)\0" as *const u8 as *const gchar,
            b"org.gtk.Notifications\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(session_bus as gpointer);
    if !reply.is_null() {
        g_variant_unref(reply);
        return TRUE;
    } else {
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_send_notification(
    mut backend: *mut GNotificationBackend,
    mut id: *const gchar,
    mut notification: *mut GNotification,
) {
    let mut params: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    params = g_variant_new(
        b"(ss@a{sv})\0" as *const u8 as *const gchar,
        g_application_get_application_id((*backend).application),
        id,
        g_notification_serialize(notification),
    );
    g_dbus_connection_call(
        (*backend).dbus_connection,
        b"org.gtk.Notifications\0" as *const u8 as *const gchar,
        b"/org/gtk/Notifications\0" as *const u8 as *const gchar,
        b"org.gtk.Notifications\0" as *const u8 as *const gchar,
        b"AddNotification\0" as *const u8 as *const gchar,
        params,
        G_VARIANT_TYPE_UNIT,
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL,
    );
}
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_withdraw_notification(
    mut backend: *mut GNotificationBackend,
    mut id: *const gchar,
) {
    let mut params: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    params = g_variant_new(
        b"(ss)\0" as *const u8 as *const gchar,
        g_application_get_application_id((*backend).application),
        id,
    );
    g_dbus_connection_call(
        (*backend).dbus_connection,
        b"org.gtk.Notifications\0" as *const u8 as *const gchar,
        b"/org/gtk/Notifications\0" as *const u8 as *const gchar,
        b"org.gtk.Notifications\0" as *const u8 as *const gchar,
        b"RemoveNotification\0" as *const u8 as *const gchar,
        params,
        G_VARIANT_TYPE_UNIT,
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL,
    );
}
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_init(
    mut backend: *mut GGtkNotificationBackend,
) {
}
unsafe extern "C" fn safe_c2rust_g_gtk_notification_backend_class_init(
    mut class: *mut GGtkNotificationBackendClass,
) {
    let mut backend_class: *mut GNotificationBackendClass =
        class as *mut ::core::ffi::c_void as *mut GNotificationBackendClass;
    (*backend_class).is_supported = Some(
        safe_c2rust_g_gtk_notification_backend_is_supported as unsafe extern "C" fn() -> gboolean,
    ) as Option<unsafe extern "C" fn() -> gboolean>;
    (*backend_class).send_notification = Some(
        safe_c2rust_g_gtk_notification_backend_send_notification
            as unsafe extern "C" fn(
                *mut GNotificationBackend,
                *const gchar,
                *mut GNotification,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GNotificationBackend, *const gchar, *mut GNotification) -> (),
        >;
    (*backend_class).withdraw_notification = Some(
        safe_c2rust_g_gtk_notification_backend_withdraw_notification
            as unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> ()>;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
