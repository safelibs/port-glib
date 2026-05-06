extern "C" {
    pub type _GData;
    pub type _GDBusAuthMechanismPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn _g_dbus_auth_mechanism_get_type() -> GType;
}
pub type size_t = usize;
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
pub struct _GDBusAuthMechanism {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthMechanismPrivate,
}
pub type GDBusAuthMechanismPrivate = _GDBusAuthMechanismPrivate;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismAnon {
    pub parent_instance: GDBusAuthMechanism,
    pub priv_0: *mut GDBusAuthMechanismAnonPrivate,
}
pub type GDBusAuthMechanismAnonPrivate = _GDBusAuthMechanismAnonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismAnonPrivate {
    pub is_client: gboolean,
    pub is_server: gboolean,
    pub state: GDBusAuthMechanismState,
}
pub type GDBusAuthMechanismAnon = _GDBusAuthMechanismAnon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismAnonClass {
    pub parent_class: GDBusAuthMechanismClass,
}
pub type GDBusAuthMechanismAnonClass = _GDBusAuthMechanismAnonClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_dbus_auth_mechanism_anon_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusAuthMechanismAnon_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusAuthMechanismAnon_private_offset,
        );
    }
    safe_c2rust__g_dbus_auth_mechanism_anon_class_init(klass as *mut GDBusAuthMechanismAnonClass);
}
static mut safe_c2rust_GDBusAuthMechanismAnon_private_offset: gint = 0;
static mut safe_c2rust__g_dbus_auth_mechanism_anon_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_get_instance_private(
    mut self_0: *mut GDBusAuthMechanismAnon,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusAuthMechanismAnon_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        _g_dbus_auth_mechanism_get_type(),
        g_intern_static_string(b"GDBusAuthMechanismAnon\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusAuthMechanismAnonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_anon_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusAuthMechanismAnon>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAuthMechanismAnon) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_anon_init
                    as unsafe extern "C" fn(*mut GDBusAuthMechanismAnon) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusAuthMechanismAnon_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusAuthMechanismAnonPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_finalize(mut object: *mut GObject) {
    if (*(safe_c2rust__g_dbus_auth_mechanism_anon_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust__g_dbus_auth_mechanism_anon_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_class_init(
    mut klass: *mut GDBusAuthMechanismAnonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut mechanism_class: *mut GDBusAuthMechanismClass =
        ::core::ptr::null_mut::<GDBusAuthMechanismClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust__g_dbus_auth_mechanism_anon_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    mechanism_class = klass as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismClass;
    (*mechanism_class).get_priority =
        Some(safe_c2rust_mechanism_get_priority as unsafe extern "C" fn() -> gint)
            as Option<unsafe extern "C" fn() -> gint>;
    (*mechanism_class).get_name =
        Some(safe_c2rust_mechanism_get_name as unsafe extern "C" fn() -> *const gchar)
            as Option<unsafe extern "C" fn() -> *const gchar>;
    (*mechanism_class).is_supported = Some(
        safe_c2rust_mechanism_is_supported
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean>;
    (*mechanism_class).encode_data = Some(
        safe_c2rust_mechanism_encode_data
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).decode_data = Some(
        safe_c2rust_mechanism_decode_data
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).server_get_state = Some(
        safe_c2rust_mechanism_server_get_state
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>;
    (*mechanism_class).server_initiate = Some(
        safe_c2rust_mechanism_server_initiate
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).server_data_receive = Some(
        safe_c2rust_mechanism_server_data_receive
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).server_data_send = Some(
        safe_c2rust_mechanism_server_data_send
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>;
    (*mechanism_class).server_get_reject_reason = Some(
        safe_c2rust_mechanism_server_get_reject_reason
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar>;
    (*mechanism_class).server_shutdown = Some(
        safe_c2rust_mechanism_server_shutdown
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>;
    (*mechanism_class).client_get_state = Some(
        safe_c2rust_mechanism_client_get_state
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>;
    (*mechanism_class).client_initiate = Some(
        safe_c2rust_mechanism_client_initiate
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                GDBusConnectionFlags,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                GDBusConnectionFlags,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).client_data_receive = Some(
        safe_c2rust_mechanism_client_data_receive
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).client_data_send = Some(
        safe_c2rust_mechanism_client_data_send
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>;
    (*mechanism_class).client_shutdown = Some(
        safe_c2rust_mechanism_client_shutdown
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_anon_init(
    mut mechanism: *mut GDBusAuthMechanismAnon,
) {
    (*mechanism).priv_0 = safe_c2rust__g_dbus_auth_mechanism_anon_get_instance_private(mechanism)
        as *mut GDBusAuthMechanismAnonPrivate;
}
unsafe extern "C" fn safe_c2rust_mechanism_get_priority() -> gint {
    return 50 as gint;
}
unsafe extern "C" fn safe_c2rust_mechanism_get_name() -> *const gchar {
    return b"ANONYMOUS\0" as *const u8 as *const gchar;
}
unsafe extern "C" fn safe_c2rust_mechanism_is_supported(
    mut mechanism: *mut GDBusAuthMechanism,
) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_mechanism_encode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_mechanism_decode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_mechanism_server_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*(*m).priv_0).state;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut initial_response: *const gchar,
    mut initial_response_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server == 0 && (*(*m).priv_0).is_client == 0 {
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
            b"!m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_server = TRUE as gboolean;
    (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA as ::core::ffi::c_int
                as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismanon.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        205 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_server_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND as ::core::ffi::c_int
                as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismanon.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        219 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_server_get_reject_reason(
    mut mechanism: *mut GDBusAuthMechanism,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_REJECTED as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_REJECTED\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismanon.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        234 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_server_shutdown(mut mechanism: *mut GDBusAuthMechanism) {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_server = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
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
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*(*m).priv_0).state;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut conn_flags: GDBusConnectionFlags,
    mut out_initial_response_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server == 0 && (*(*m).priv_0).is_client == 0 {
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
            b"!m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    (*(*m).priv_0).is_client = TRUE as gboolean;
    (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED;
    result = safe_c2rust_g_strdup_inline(b"GDBus 0.1\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut gchar;
    *out_initial_response_len = strlen(result) as gsize;
    return result;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
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
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA as ::core::ffi::c_int
                as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismanon.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        296 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_client_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
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
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND as ::core::ffi::c_int
                as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismanon.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        310 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_client_shutdown(mut mechanism: *mut GDBusAuthMechanism) {
    let mut m: *mut GDBusAuthMechanismAnon =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismAnon;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_anon_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_ANON (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
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
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_client = FALSE as gboolean;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
