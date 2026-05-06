extern "C" {
    pub type _GData;
    pub type _GSocketControlMessagePrivate;
    pub type _GCredentials;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
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
    fn g_credentials_new() -> *mut GCredentials;
    fn g_credentials_get_native(
        credentials: *mut GCredentials,
        native_type: GCredentialsType,
    ) -> gpointer;
    fn g_credentials_set_native(
        credentials: *mut GCredentials,
        native_type: GCredentialsType,
        native: gpointer,
    );
    fn g_credentials_get_unix_user(
        credentials: *mut GCredentials,
        error: *mut *mut GError,
    ) -> uid_t;
    fn g_socket_control_message_get_type() -> GType;
}
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type size_t = usize;
pub type pid_t = __pid_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
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
pub type GCredentialsType = ::core::ffi::c_uint;
pub const G_CREDENTIALS_TYPE_WIN32_PID: GCredentialsType = 7;
pub const G_CREDENTIALS_TYPE_APPLE_XUCRED: GCredentialsType = 6;
pub const G_CREDENTIALS_TYPE_NETBSD_UNPCBID: GCredentialsType = 5;
pub const G_CREDENTIALS_TYPE_SOLARIS_UCRED: GCredentialsType = 4;
pub const G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED: GCredentialsType = 3;
pub const G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED: GCredentialsType = 2;
pub const G_CREDENTIALS_TYPE_LINUX_UCRED: GCredentialsType = 1;
pub const G_CREDENTIALS_TYPE_INVALID: GCredentialsType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
pub type GCredentials = _GCredentials;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixCredentialsMessage {
    pub parent_instance: GSocketControlMessage,
    pub priv_0: *mut GUnixCredentialsMessagePrivate,
}
pub type GUnixCredentialsMessagePrivate = _GUnixCredentialsMessagePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixCredentialsMessagePrivate {
    pub credentials: *mut GCredentials,
}
pub type GUnixCredentialsMessage = _GUnixCredentialsMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessageClass {
    pub parent_class: GObjectClass,
    pub get_size: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize>,
    pub get_level: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>,
    pub get_type: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>,
    pub serialize: Option<unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> ()>,
    pub deserialize: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            gsize,
            gpointer,
        ) -> *mut GSocketControlMessage,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketControlMessageClass = _GSocketControlMessageClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixCredentialsMessageClass {
    pub parent_class: GSocketControlMessageClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
}
pub type GUnixCredentialsMessageClass = _GUnixCredentialsMessageClass;
pub const PROP_CREDENTIALS: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucred {
    pub pid: pid_t,
    pub uid: uid_t,
    pub gid: gid_t,
}
pub const SCM_CREDENTIALS: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCM_PIDFD: C2RustUnnamed_0 = 4;
pub const SCM_SECURITY: C2RustUnnamed_0 = 3;
pub const SCM_RIGHTS: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_CREDENTIALS_NATIVE_SIZE: usize = ::core::mem::size_of::<ucred>();
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_credentials_message_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixCredentialsMessage_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixCredentialsMessage_private_offset,
        );
    }
    safe_c2rust_g_unix_credentials_message_class_init(klass as *mut GUnixCredentialsMessageClass);
}
static mut safe_c2rust_GUnixCredentialsMessage_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_control_message_get_type(),
        g_intern_static_string(b"GUnixCredentialsMessage\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixCredentialsMessageClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_credentials_message_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixCredentialsMessage>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixCredentialsMessage) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_credentials_message_init
                    as unsafe extern "C" fn(*mut GUnixCredentialsMessage) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GUnixCredentialsMessage_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GUnixCredentialsMessagePrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_credentials_message_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_instance_private(
    mut self_0: *mut GUnixCredentialsMessage,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GUnixCredentialsMessage_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_unix_credentials_message_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_size(
    mut message: *mut GSocketControlMessage,
) -> gsize {
    return G_CREDENTIALS_NATIVE_SIZE as gsize;
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_level(
    mut message: *mut GSocketControlMessage,
) -> ::core::ffi::c_int {
    return SOL_SOCKET;
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_msg_type(
    mut message: *mut GSocketControlMessage,
) -> ::core::ffi::c_int {
    return SCM_CREDENTIALS as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_deserialize(
    mut level: gint,
    mut type_0: gint,
    mut size: gsize,
    mut data: gpointer,
) -> *mut GSocketControlMessage {
    let mut message: *mut GSocketControlMessage = ::core::ptr::null_mut::<GSocketControlMessage>();
    let mut credentials: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    if level != SOL_SOCKET
        || type_0
            != safe_c2rust_g_unix_credentials_message_get_msg_type(::core::ptr::null_mut::<
                GSocketControlMessage,
            >())
    {
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    if size as usize != G_CREDENTIALS_NATIVE_SIZE {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Expected a credentials struct of %lu bytes but got %lu bytes of data\0" as *const u8
                as *const gchar,
            ::core::mem::size_of::<ucred>(),
            size,
        );
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    credentials = g_credentials_new();
    g_credentials_set_native(credentials, G_CREDENTIALS_TYPE_LINUX_UCRED, data);
    if g_credentials_get_unix_user(credentials, ::core::ptr::null_mut::<*mut GError>())
        == -(1 as ::core::ffi::c_int) as uid_t
    {
        g_object_unref(credentials as gpointer);
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    message = safe_c2rust_g_unix_credentials_message_new_with_credentials(credentials);
    g_object_unref(credentials as gpointer);
    return message;
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_serialize(
    mut _message: *mut GSocketControlMessage,
    mut data: gpointer,
) {
    let mut message: *mut GUnixCredentialsMessage =
        _message as *mut ::core::ffi::c_void as *mut GUnixCredentialsMessage;
    memcpy(
        data as *mut ::core::ffi::c_void,
        g_credentials_get_native(
            (*(*message).priv_0).credentials,
            G_CREDENTIALS_TYPE_LINUX_UCRED,
        ) as *const ::core::ffi::c_void,
        G_CREDENTIALS_NATIVE_SIZE,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_finalize(mut object: *mut GObject) {
    let mut message: *mut GUnixCredentialsMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixCredentialsMessage;
    if !(*(*message).priv_0).credentials.is_null() {
        g_object_unref((*(*message).priv_0).credentials as gpointer);
    }
    (*(safe_c2rust_g_unix_credentials_message_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_init(
    mut message: *mut GUnixCredentialsMessage,
) {
    (*message).priv_0 = safe_c2rust_g_unix_credentials_message_get_instance_private(message)
        as *mut GUnixCredentialsMessagePrivate;
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut message: *mut GUnixCredentialsMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixCredentialsMessage;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*message).priv_0).credentials as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixcredentialsmessage.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut message: *mut GUnixCredentialsMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixCredentialsMessage;
    match prop_id {
        1 => {
            (*(*message).priv_0).credentials = g_value_dup_object(value) as *mut GCredentials;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixcredentialsmessage.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_constructed(mut object: *mut GObject) {
    let mut message: *mut GUnixCredentialsMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixCredentialsMessage;
    if (*(*message).priv_0).credentials.is_null() {
        (*(*message).priv_0).credentials = g_credentials_new();
    }
    if (*(safe_c2rust_g_unix_credentials_message_parent_class as *mut GObjectClass))
        .constructed
        .is_some()
    {
        (*(safe_c2rust_g_unix_credentials_message_parent_class as *mut GObjectClass))
            .constructed
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_class_init(
    mut class: *mut GUnixCredentialsMessageClass,
) {
    let mut scm_class: *mut GSocketControlMessageClass =
        ::core::ptr::null_mut::<GSocketControlMessageClass>();
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_unix_credentials_message_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_unix_credentials_message_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_unix_credentials_message_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).constructed = Some(
        safe_c2rust_g_unix_credentials_message_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    scm_class = class as *mut ::core::ffi::c_void as *mut GSocketControlMessageClass;
    (*scm_class).get_size = Some(
        safe_c2rust_g_unix_credentials_message_get_size
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize>;
    (*scm_class).get_level = Some(
        safe_c2rust_g_unix_credentials_message_get_level
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>;
    (*scm_class).get_type = Some(
        safe_c2rust_g_unix_credentials_message_get_msg_type
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>;
    (*scm_class).serialize = Some(
        safe_c2rust_g_unix_credentials_message_serialize
            as unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> ()>;
    (*scm_class).deserialize = Some(
        safe_c2rust_g_unix_credentials_message_deserialize
            as unsafe extern "C" fn(gint, gint, gsize, gpointer) -> *mut GSocketControlMessage,
    )
        as Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                gsize,
                gpointer,
            ) -> *mut GSocketControlMessage,
        >;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_is_supported() -> gboolean {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_new() -> *mut GSocketControlMessage
{
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if safe_c2rust_g_unix_credentials_message_is_supported() != 0 {
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
            b"g_unix_credentials_message_is_supported ()\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    return g_object_new(
        safe_c2rust_g_unix_credentials_message_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSocketControlMessage;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_new_with_credentials(
    mut credentials: *mut GCredentials,
) -> *mut GSocketControlMessage {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = credentials as *mut GTypeInstance;
            let mut __t: GType = g_credentials_get_type();
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
            b"G_IS_CREDENTIALS (credentials)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if safe_c2rust_g_unix_credentials_message_is_supported() != 0 {
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
            b"g_unix_credentials_message_is_supported ()\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    return g_object_new(
        safe_c2rust_g_unix_credentials_message_get_type(),
        b"credentials\0" as *const u8 as *const gchar,
        credentials,
        NULL,
    ) as *mut GSocketControlMessage;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_credentials_message_get_credentials(
    mut message: *mut GUnixCredentialsMessage,
) -> *mut GCredentials {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_credentials_message_get_type();
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
            b"G_IS_UNIX_CREDENTIALS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    return (*(*message).priv_0).credentials;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
