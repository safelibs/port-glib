extern "C" {
    pub type _GData;
    pub type _GIOStreamPrivate;
    pub type _GCredentials;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
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
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_credentials_get_type() -> GType;
    fn g_io_stream_get_type() -> GType;
}
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GDBusConnectionFlags = ::core::ffi::c_uint;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: GDBusConnectionFlags = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusConnectionFlags = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: GDBusConnectionFlags = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: GDBusConnectionFlags = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusConnectionFlags = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: GDBusConnectionFlags = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: GDBusConnectionFlags = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: GDBusConnectionFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GIOStream = _GIOStream;
pub type GCredentials = _GCredentials;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanism {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthMechanismPrivate,
}
pub type GDBusAuthMechanismPrivate = _GDBusAuthMechanismPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismPrivate {
    pub stream: *mut GIOStream,
    pub credentials: *mut GCredentials,
}
pub type GDBusAuthMechanism = _GDBusAuthMechanism;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismClass {
    pub parent_class: GObjectClass,
    pub get_priority: Option<unsafe extern "C" fn() -> gint>,
    pub get_name: Option<unsafe extern "C" fn() -> *const gchar>,
    pub is_supported: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean>,
    pub encode_data: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            *const gchar,
            gsize,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub decode_data: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            *const gchar,
            gsize,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub server_get_state:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>,
    pub server_initiate:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub server_data_receive:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub server_data_send:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>,
    pub server_get_reject_reason:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar>,
    pub server_shutdown: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>,
    pub client_get_state:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>,
    pub client_initiate: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            GDBusConnectionFlags,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub client_data_receive:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub client_data_send:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>,
    pub client_shutdown: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>,
}
pub type GDBusAuthMechanismState = ::core::ffi::c_uint;
pub const G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED: GDBusAuthMechanismState = 4;
pub const G_DBUS_AUTH_MECHANISM_STATE_REJECTED: GDBusAuthMechanismState = 3;
pub const G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND: GDBusAuthMechanismState = 2;
pub const G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA: GDBusAuthMechanismState = 1;
pub const G_DBUS_AUTH_MECHANISM_STATE_INVALID: GDBusAuthMechanismState = 0;
pub type GDBusAuthMechanismClass = _GDBusAuthMechanismClass;
pub const PROP_CREDENTIALS: C2RustUnnamed_0 = 2;
pub const PROP_STREAM: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust__g_dbus_auth_mechanism_parent_class: gpointer = NULL;
#[inline]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_instance_private(
    mut self_0: *mut GDBusAuthMechanism,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusAuthMechanism_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_class_intern_init(mut klass: gpointer) {
    safe_c2rust__g_dbus_auth_mechanism_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusAuthMechanism_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusAuthMechanism_private_offset,
        );
    }
    safe_c2rust__g_dbus_auth_mechanism_class_init(klass as *mut GDBusAuthMechanismClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_dbus_auth_mechanism_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusAuthMechanism\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusAuthMechanismClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusAuthMechanism>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_init
                    as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GDBusAuthMechanism_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusAuthMechanismPrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDBusAuthMechanism_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_finalize(mut object: *mut GObject) {
    let mut mechanism: *mut GDBusAuthMechanism =
        object as *mut ::core::ffi::c_void as *mut GDBusAuthMechanism;
    if !(*(*mechanism).priv_0).stream.is_null() {
        g_object_unref((*(*mechanism).priv_0).stream as gpointer);
    }
    if !(*(*mechanism).priv_0).credentials.is_null() {
        g_object_unref((*(*mechanism).priv_0).credentials as gpointer);
    }
    (*(safe_c2rust__g_dbus_auth_mechanism_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut mechanism: *mut GDBusAuthMechanism =
        object as *mut ::core::ffi::c_void as *mut GDBusAuthMechanism;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*mechanism).priv_0).stream as gpointer);
        }
        2 => {
            g_value_set_object(value, (*(*mechanism).priv_0).credentials as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanism.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut mechanism: *mut GDBusAuthMechanism =
        object as *mut ::core::ffi::c_void as *mut GDBusAuthMechanism;
    match prop_id {
        1 => {
            (*(*mechanism).priv_0).stream = g_value_dup_object(value) as *mut GIOStream;
        }
        2 => {
            (*(*mechanism).priv_0).credentials = g_value_dup_object(value) as *mut GCredentials;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanism.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_class_init(
    mut klass: *mut GDBusAuthMechanismClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust__g_dbus_auth_mechanism_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust__g_dbus_auth_mechanism_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust__g_dbus_auth_mechanism_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_io_stream_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CREDENTIALS as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"credentials\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_credentials_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_init(
    mut mechanism: *mut GDBusAuthMechanism,
) {
    (*mechanism).priv_0 = safe_c2rust__g_dbus_auth_mechanism_get_instance_private(mechanism)
        as *mut GDBusAuthMechanismPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_stream(
    mut mechanism: *mut GDBusAuthMechanism,
) -> *mut GIOStream {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    return (*(*mechanism).priv_0).stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_credentials(
    mut mechanism: *mut GDBusAuthMechanism,
) -> *mut GCredentials {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    return (*(*mechanism).priv_0).credentials;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_name(
    mut mechanism_type: GType,
) -> *const gchar {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut klass: *mut GDBusAuthMechanismClass =
        ::core::ptr::null_mut::<GDBusAuthMechanismClass>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if mechanism_type == safe_c2rust__g_dbus_auth_mechanism_get_type()
            || g_type_is_a(
                mechanism_type,
                safe_c2rust__g_dbus_auth_mechanism_get_type(),
            ) != 0
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
            b"g_type_is_a (mechanism_type, G_TYPE_DBUS_AUTH_MECHANISM)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    klass = g_type_class_ref(mechanism_type) as *mut GDBusAuthMechanismClass;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !klass.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanism.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            188 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    name = (*klass).get_name.expect("non-null function pointer")();
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_get_priority(
    mut mechanism_type: GType,
) -> gint {
    let mut priority: gint = 0;
    let mut klass: *mut GDBusAuthMechanismClass =
        ::core::ptr::null_mut::<GDBusAuthMechanismClass>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if mechanism_type == safe_c2rust__g_dbus_auth_mechanism_get_type()
            || g_type_is_a(
                mechanism_type,
                safe_c2rust__g_dbus_auth_mechanism_get_type(),
            ) != 0
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
            b"g_type_is_a (mechanism_type, G_TYPE_DBUS_AUTH_MECHANISM)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    klass = g_type_class_ref(mechanism_type) as *mut GDBusAuthMechanismClass;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !klass.is_null() {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanism.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            204 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    priority = (*klass).get_priority.expect("non-null function pointer")();
    return priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_is_supported(
    mut mechanism: *mut GDBusAuthMechanism,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .is_supported
        .expect("non-null function pointer")(mechanism);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_encode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .encode_data
        .expect("non-null function pointer")(mechanism, data, data_len, out_data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_decode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .decode_data
        .expect("non-null function pointer")(mechanism, data, data_len, out_data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_get_state
        .expect("non-null function pointer")(mechanism);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut initial_response: *const gchar,
    mut initial_response_len: gsize,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_initiate
        .expect("non-null function pointer")(mechanism, initial_response, initial_response_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_data_receive
        .expect("non-null function pointer")(mechanism, data, data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_data_send
        .expect("non-null function pointer")(mechanism, out_data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_get_reject_reason(
    mut mechanism: *mut GDBusAuthMechanism,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_get_reject_reason
        .expect("non-null function pointer")(mechanism);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_server_shutdown(
    mut mechanism: *mut GDBusAuthMechanism,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .server_shutdown
        .expect("non-null function pointer")(mechanism);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_client_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .client_get_state
        .expect("non-null function pointer")(mechanism);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_client_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut conn_flags: GDBusConnectionFlags,
    mut out_initial_response_len: *mut gsize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .client_initiate
        .expect("non-null function pointer")(
        mechanism, conn_flags, out_initial_response_len
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_client_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .client_data_receive
        .expect("non-null function pointer")(mechanism, data, data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_client_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .client_data_send
        .expect("non-null function pointer")(mechanism, out_data_len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_client_shutdown(
    mut mechanism: *mut GDBusAuthMechanism,
) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM (mechanism)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(mechanism as *mut GTypeInstance)).g_class as *mut GDBusAuthMechanismClass))
        .client_shutdown
        .expect("non-null function pointer")(mechanism);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
