use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GDtlsConnection;
    pub type _GTlsCertificatePrivate;
    pub type _GTlsDatabasePrivate;
    pub type _GTlsInteractionPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_type_class_peek_static(type_0: GType) -> gpointer;
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
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_object_interface_install_property(g_iface: gpointer, pspec: *mut GParamSpec);
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_unref(object: gpointer);
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_datagram_based_get_type() -> GType;
    fn g_cancellable_get_type() -> GType;
    fn g_tls_certificate_flags_get_type() -> GType;
    fn g_tls_rehandshake_mode_get_type() -> GType;
    fn g_tls_protocol_version_get_type() -> GType;
    fn g_tls_certificate_get_type() -> GType;
    fn g_tls_channel_binding_error_quark() -> GQuark;
    fn g_tls_database_get_type() -> GType;
    fn g_tls_interaction_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_BOOLEAN__OBJECT_FLAGS(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__OBJECT_FLAGSv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
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
pub type GSignalCVaMarshaller = GVaClosureMarshal;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
pub type GTlsCertificateFlags = ::core::ffi::c_uint;
pub const G_TLS_CERTIFICATE_VALIDATE_ALL: GTlsCertificateFlags = 127;
pub const G_TLS_CERTIFICATE_GENERIC_ERROR: GTlsCertificateFlags = 64;
pub const G_TLS_CERTIFICATE_INSECURE: GTlsCertificateFlags = 32;
pub const G_TLS_CERTIFICATE_REVOKED: GTlsCertificateFlags = 16;
pub const G_TLS_CERTIFICATE_EXPIRED: GTlsCertificateFlags = 8;
pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: GTlsCertificateFlags = 4;
pub const G_TLS_CERTIFICATE_BAD_IDENTITY: GTlsCertificateFlags = 2;
pub const G_TLS_CERTIFICATE_UNKNOWN_CA: GTlsCertificateFlags = 1;
pub const G_TLS_CERTIFICATE_NO_FLAGS: GTlsCertificateFlags = 0;
pub type GTlsChannelBindingType = ::core::ffi::c_uint;
pub const G_TLS_CHANNEL_BINDING_TLS_EXPORTER: GTlsChannelBindingType = 2;
pub const G_TLS_CHANNEL_BINDING_TLS_SERVER_END_POINT: GTlsChannelBindingType = 1;
pub const G_TLS_CHANNEL_BINDING_TLS_UNIQUE: GTlsChannelBindingType = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_TLS_CHANNEL_BINDING_ERROR_GENERAL_ERROR: C2RustUnnamed_0 = 4;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 3;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_AVAILABLE: C2RustUnnamed_0 = 2;
pub const G_TLS_CHANNEL_BINDING_ERROR_INVALID_STATE: C2RustUnnamed_0 = 1;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED: C2RustUnnamed_0 = 0;
pub type GTlsRehandshakeMode = ::core::ffi::c_uint;
pub const G_TLS_REHANDSHAKE_UNSAFELY: GTlsRehandshakeMode = 2;
pub const G_TLS_REHANDSHAKE_SAFELY: GTlsRehandshakeMode = 1;
pub const G_TLS_REHANDSHAKE_NEVER: GTlsRehandshakeMode = 0;
pub type GTlsProtocolVersion = ::core::ffi::c_uint;
pub const G_TLS_PROTOCOL_VERSION_DTLS_1_2: GTlsProtocolVersion = 202;
pub const G_TLS_PROTOCOL_VERSION_DTLS_1_0: GTlsProtocolVersion = 201;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_3: GTlsProtocolVersion = 5;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_2: GTlsProtocolVersion = 4;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_1: GTlsProtocolVersion = 3;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_0: GTlsProtocolVersion = 2;
pub const G_TLS_PROTOCOL_VERSION_SSL_3_0: GTlsProtocolVersion = 1;
pub const G_TLS_PROTOCOL_VERSION_UNKNOWN: GTlsProtocolVersion = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GDtlsConnection = _GDtlsConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificate {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsCertificatePrivate,
}
pub type GTlsCertificatePrivate = _GTlsCertificatePrivate;
pub type GTlsCertificate = _GTlsCertificate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsDatabase {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsDatabasePrivate,
}
pub type GTlsDatabasePrivate = _GTlsDatabasePrivate;
pub type GTlsDatabase = _GTlsDatabase;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsInteraction {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsInteractionPrivate,
}
pub type GTlsInteractionPrivate = _GTlsInteractionPrivate;
pub type GTlsInteraction = _GTlsInteraction;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDtlsConnectionInterface {
    pub g_iface: GTypeInterface,
    pub accept_certificate: Option<
        unsafe extern "C" fn(
            *mut GDtlsConnection,
            *mut GTlsCertificate,
            GTlsCertificateFlags,
        ) -> gboolean,
    >,
    pub handshake: Option<
        unsafe extern "C" fn(*mut GDtlsConnection, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub handshake_async: Option<
        unsafe extern "C" fn(
            *mut GDtlsConnection,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub handshake_finish: Option<
        unsafe extern "C" fn(*mut GDtlsConnection, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub shutdown: Option<
        unsafe extern "C" fn(
            *mut GDtlsConnection,
            gboolean,
            gboolean,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub shutdown_async: Option<
        unsafe extern "C" fn(
            *mut GDtlsConnection,
            gboolean,
            gboolean,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub shutdown_finish: Option<
        unsafe extern "C" fn(*mut GDtlsConnection, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub set_advertised_protocols:
        Option<unsafe extern "C" fn(*mut GDtlsConnection, *const *const gchar) -> ()>,
    pub get_negotiated_protocol: Option<unsafe extern "C" fn(*mut GDtlsConnection) -> *const gchar>,
    pub get_binding_data: Option<
        unsafe extern "C" fn(
            *mut GDtlsConnection,
            GTlsChannelBindingType,
            *mut GByteArray,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type GDtlsConnectionInterface = _GDtlsConnectionInterface;
pub const ACCEPT_CERTIFICATE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_type() -> GType {
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
            g_intern_static_string(b"GDtlsConnection\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDtlsConnectionInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDtlsConnectionInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_dtls_connection_default_init
                        as unsafe extern "C" fn(*mut GDtlsConnectionInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL_0),
            G_TYPE_FLAG_NONE,
        );
        if g_datagram_based_get_type() != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(g_define_type_id, g_datagram_based_get_type());
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
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
unsafe extern "C" fn safe_c2rust_g_dtls_connection_default_init(
    mut iface: *mut GDtlsConnectionInterface,
) {
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"base-socket\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_datagram_based_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"database\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_database_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"interaction\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_interaction_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boolean(
            b"require-close-notify\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_enum(
            b"rehandshake-mode\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_rehandshake_mode_get_type(),
            G_TLS_REHANDSHAKE_NEVER as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_DEPRECATED as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"certificate\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_certificate_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"peer-certificate\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_certificate_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_flags(
            b"peer-certificate-errors\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_certificate_flags_get_type(),
            0 as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boxed(
            b"advertised-protocols\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_strv_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_string(
            b"negotiated-protocol\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_enum(
            b"protocol-version\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_protocol_version_get_type(),
            G_TLS_PROTOCOL_VERSION_UNKNOWN as ::core::ffi::c_int as gint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_string(
            b"ciphersuite-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_signals[ACCEPT_CERTIFICATE as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"accept-certificate\0" as *const u8 as *const gchar),
        safe_c2rust_g_dtls_connection_get_type(),
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL_0,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT_FLAGS
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_tls_certificate_get_type(),
        g_tls_certificate_flags_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[ACCEPT_CERTIFICATE as ::core::ffi::c_int as usize],
        (*(iface as *mut GTypeInterface)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT_FLAGSv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_database(
    mut conn: *mut GDtlsConnection,
    mut database: *mut GTlsDatabase,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if database.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = database as *mut GTypeInstance;
                let mut __t: GType = g_tls_database_get_type();
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
            b"database == NULL || G_IS_TLS_DATABASE (database)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"database\0" as *const u8 as *const gchar,
        database,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_database(
    mut conn: *mut GDtlsConnection,
) -> *mut GTlsDatabase {
    let mut database: *mut GTlsDatabase = ::core::ptr::null_mut::<GTlsDatabase>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsDatabase>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"database\0" as *const u8 as *const gchar,
        &raw mut database,
        NULL_0,
    );
    if !database.is_null() {
        g_object_unref(database as gpointer);
    }
    return database;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_certificate(
    mut conn: *mut GDtlsConnection,
    mut certificate: *mut GTlsCertificate,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = certificate as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (certificate)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"certificate\0" as *const u8 as *const gchar,
        certificate,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_certificate(
    mut conn: *mut GDtlsConnection,
) -> *mut GTlsCertificate {
    let mut certificate: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"certificate\0" as *const u8 as *const gchar,
        &raw mut certificate,
        NULL_0,
    );
    if !certificate.is_null() {
        g_object_unref(certificate as gpointer);
    }
    return certificate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_interaction(
    mut conn: *mut GDtlsConnection,
    mut interaction: *mut GTlsInteraction,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"interaction\0" as *const u8 as *const gchar,
        interaction,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_interaction(
    mut conn: *mut GDtlsConnection,
) -> *mut GTlsInteraction {
    let mut interaction: *mut GTlsInteraction = ::core::ptr::null_mut::<GTlsInteraction>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsInteraction>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"interaction\0" as *const u8 as *const gchar,
        &raw mut interaction,
        NULL_0,
    );
    if !interaction.is_null() {
        g_object_unref(interaction as gpointer);
    }
    return interaction;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_peer_certificate(
    mut conn: *mut GDtlsConnection,
) -> *mut GTlsCertificate {
    let mut peer_certificate: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"peer-certificate\0" as *const u8 as *const gchar,
        &raw mut peer_certificate,
        NULL_0,
    );
    if !peer_certificate.is_null() {
        g_object_unref(peer_certificate as gpointer);
    }
    return peer_certificate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_peer_certificate_errors(
    mut conn: *mut GDtlsConnection,
) -> GTlsCertificateFlags {
    let mut errors: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_NO_FLAGS;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"peer-certificate-errors\0" as *const u8 as *const gchar,
        &raw mut errors,
        NULL_0,
    );
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_require_close_notify(
    mut conn: *mut GDtlsConnection,
    mut require_close_notify: gboolean,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"require-close-notify\0" as *const u8 as *const gchar,
        require_close_notify,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_require_close_notify(
    mut conn: *mut GDtlsConnection,
) -> gboolean {
    let mut require_close_notify: gboolean = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"require-close-notify\0" as *const u8 as *const gchar,
        &raw mut require_close_notify,
        NULL_0,
    );
    return require_close_notify;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_rehandshake_mode(
    mut conn: *mut GDtlsConnection,
    mut mode: GTlsRehandshakeMode,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"rehandshake-mode\0" as *const u8 as *const gchar,
        G_TLS_REHANDSHAKE_SAFELY as ::core::ffi::c_int,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_rehandshake_mode(
    mut conn: *mut GDtlsConnection,
) -> GTlsRehandshakeMode {
    let mut mode: GTlsRehandshakeMode = G_TLS_REHANDSHAKE_NEVER;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_REHANDSHAKE_SAFELY;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"rehandshake-mode\0" as *const u8 as *const gchar,
        &raw mut mode,
        NULL_0,
    );
    return G_TLS_REHANDSHAKE_SAFELY;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_handshake(
    mut conn: *mut GDtlsConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .handshake
        .expect("non-null function pointer")(conn, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_handshake_async(
    mut conn: *mut GDtlsConnection,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .handshake_async
        .expect("non-null function pointer")(
        conn, io_priority, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_handshake_finish(
    mut conn: *mut GDtlsConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .handshake_finish
        .expect("non-null function pointer")(conn, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_shutdown(
    mut conn: *mut GDtlsConnection,
    mut shutdown_read: gboolean,
    mut shutdown_write: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if shutdown_read == 0 && shutdown_write == 0 {
        return TRUE;
    }
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*iface).shutdown.is_some() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdtlsconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            855 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->shutdown != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*iface).shutdown.expect("non-null function pointer")(
        conn,
        shutdown_read,
        shutdown_write,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_shutdown_async(
    mut conn: *mut GDtlsConnection,
    mut shutdown_read: gboolean,
    mut shutdown_write: gboolean,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*iface).shutdown_async.is_some() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdtlsconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            891 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->shutdown_async != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*iface).shutdown_async.expect("non-null function pointer")(
        conn,
        TRUE,
        TRUE,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_shutdown_finish(
    mut conn: *mut GDtlsConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if (*iface).shutdown_finish.is_some() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdtlsconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            922 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->shutdown_finish != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*iface).shutdown_finish.expect("non-null function pointer")(conn, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_close(
    mut conn: *mut GDtlsConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .shutdown
        .expect("non-null function pointer")(conn, TRUE, TRUE, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_close_async(
    mut conn: *mut GDtlsConnection,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .shutdown_async
        .expect("non-null function pointer")(
        conn,
        TRUE,
        TRUE,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_close_finish(
    mut conn: *mut GDtlsConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface))
        .shutdown_finish
        .expect("non-null function pointer")(conn, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_emit_accept_certificate(
    mut conn: *mut GDtlsConnection,
    mut peer_cert: *mut GTlsCertificate,
    mut errors: GTlsCertificateFlags,
) -> gboolean {
    let mut accept: gboolean = FALSE;
    g_signal_emit(
        conn as gpointer,
        safe_c2rust_signals[ACCEPT_CERTIFICATE as ::core::ffi::c_int as usize],
        0 as GQuark,
        peer_cert,
        errors as ::core::ffi::c_uint,
        &raw mut accept,
    );
    return accept;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_set_advertised_protocols(
    mut conn: *mut GDtlsConnection,
    mut protocols: *const *const gchar,
) {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if (*iface).set_advertised_protocols.is_none() {
        return;
    }
    (*iface)
        .set_advertised_protocols
        .expect("non-null function pointer")(conn, protocols);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_negotiated_protocol(
    mut conn: *mut GDtlsConnection,
) -> *const gchar {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if (*iface).get_negotiated_protocol.is_none() {
        return ::core::ptr::null::<gchar>();
    }
    return (*iface)
        .get_negotiated_protocol
        .expect("non-null function pointer")(conn);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_channel_binding_data(
    mut conn: *mut GDtlsConnection,
    mut type_0: GTlsChannelBindingType,
    mut data: *mut GByteArray,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDtlsConnectionInterface =
        ::core::ptr::null_mut::<GDtlsConnectionInterface>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(conn as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dtls_connection_get_type(),
    ) as *mut GDtlsConnectionInterface;
    if (*iface).get_binding_data.is_none() {
        g_set_error_literal(
            error,
            g_tls_channel_binding_error_quark(),
            G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"TLS backend does not implement TLS binding retrieval\0" as *const u8
                    as *const gchar,
            ),
        );
        return FALSE;
    }
    return (*iface)
        .get_binding_data
        .expect("non-null function pointer")(conn, type_0, data, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_protocol_version(
    mut conn: *mut GDtlsConnection,
) -> GTlsProtocolVersion {
    let mut protocol_version: GTlsProtocolVersion = G_TLS_PROTOCOL_VERSION_UNKNOWN;
    let mut enum_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_PROTOCOL_VERSION_UNKNOWN;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"protocol-version\0" as *const u8 as *const gchar,
        &raw mut protocol_version,
        NULL_0,
    );
    enum_class = g_type_class_peek_static(g_tls_protocol_version_get_type()) as *mut GEnumClass;
    enum_value = g_enum_get_value(enum_class, protocol_version as gint);
    return (if !enum_value.is_null() {
        protocol_version as ::core::ffi::c_uint
    } else {
        G_TLS_PROTOCOL_VERSION_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    }) as GTlsProtocolVersion;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_connection_get_ciphersuite_name(
    mut conn: *mut GDtlsConnection,
) -> *mut gchar {
    let mut ciphersuite_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_connection_get_type();
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
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DTLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"ciphersuite-name\0" as *const u8 as *const gchar,
        &raw mut ciphersuite_name,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut ciphersuite_name as gpointer) as *mut gchar;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
