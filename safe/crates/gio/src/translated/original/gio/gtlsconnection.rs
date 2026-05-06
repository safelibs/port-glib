use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GTlsCertificatePrivate;
    pub type _GTlsConnectionPrivate;
    pub type _GTlsDatabasePrivate;
    pub type _GTlsInteractionPrivate;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
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
    fn g_type_class_peek_static(type_0: GType) -> gpointer;
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
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
    fn g_io_stream_get_type() -> GType;
    fn g_tls_certificate_flags_get_type() -> GType;
    fn g_tls_rehandshake_mode_get_type() -> GType;
    fn g_tls_protocol_version_get_type() -> GType;
    fn g_tls_certificate_get_type() -> GType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GIOStream = _GIOStream;
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
pub struct _GTlsConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GTlsConnectionPrivate,
}
pub type GTlsConnectionPrivate = _GTlsConnectionPrivate;
pub type GTlsConnection = _GTlsConnection;
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
pub struct _GIOStreamClass {
    pub parent_class: GObjectClass,
    pub get_input_stream: Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GInputStream>,
    pub get_output_stream: Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GOutputStream>,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GIOStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GIOStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GIOStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved9: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved10: Option<unsafe extern "C" fn() -> ()>,
}
pub type GIOStreamClass = _GIOStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsConnectionClass {
    pub parent_class: GIOStreamClass,
    pub accept_certificate: Option<
        unsafe extern "C" fn(
            *mut GTlsConnection,
            *mut GTlsCertificate,
            GTlsCertificateFlags,
        ) -> gboolean,
    >,
    pub handshake: Option<
        unsafe extern "C" fn(*mut GTlsConnection, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub handshake_async: Option<
        unsafe extern "C" fn(
            *mut GTlsConnection,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub handshake_finish: Option<
        unsafe extern "C" fn(*mut GTlsConnection, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub get_binding_data: Option<
        unsafe extern "C" fn(
            *mut GTlsConnection,
            GTlsChannelBindingType,
            *mut GByteArray,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub get_negotiated_protocol: Option<unsafe extern "C" fn(*mut GTlsConnection) -> *const gchar>,
    pub padding: [gpointer; 6],
}
pub type GTlsConnectionClass = _GTlsConnectionClass;
pub const ACCEPT_CERTIFICATE: C2RustUnnamed_1 = 0;
pub const PROP_CIPHERSUITE_NAME: C2RustUnnamed_2 = 13;
pub const PROP_PROTOCOL_VERSION: C2RustUnnamed_2 = 12;
pub const PROP_NEGOTIATED_PROTOCOL: C2RustUnnamed_2 = 11;
pub const PROP_ADVERTISED_PROTOCOLS: C2RustUnnamed_2 = 10;
pub const PROP_PEER_CERTIFICATE_ERRORS: C2RustUnnamed_2 = 9;
pub const PROP_PEER_CERTIFICATE: C2RustUnnamed_2 = 8;
pub const PROP_CERTIFICATE: C2RustUnnamed_2 = 7;
pub const PROP_REHANDSHAKE_MODE: C2RustUnnamed_2 = 3;
pub const PROP_REQUIRE_CLOSE_NOTIFY: C2RustUnnamed_2 = 2;
pub const PROP_INTERACTION: C2RustUnnamed_2 = 6;
pub const PROP_DATABASE: C2RustUnnamed_2 = 5;
pub const PROP_USE_SYSTEM_CERTDB: C2RustUnnamed_2 = 4;
pub const PROP_BASE_IO_STREAM: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
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
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_tls_connection_get_type_once();
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
static mut safe_c2rust_GTlsConnection_private_offset: gint = 0;
static mut safe_c2rust_g_tls_connection_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_tls_connection_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_io_stream_get_type(),
        g_intern_static_string(b"GTlsConnection\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTlsConnectionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_connection_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTlsConnection>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTlsConnection) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_connection_init
                    as unsafe extern "C" fn(*mut GTlsConnection) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_tls_connection_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_tls_connection_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTlsConnection_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GTlsConnection_private_offset,
        );
    }
    safe_c2rust_g_tls_connection_class_init(klass as *mut GTlsConnectionClass);
}
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
unsafe extern "C" fn safe_c2rust_g_tls_connection_class_init(mut klass: *mut GTlsConnectionClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_tls_connection_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_tls_connection_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_BASE_IO_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"base-io-stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_io_stream_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_USE_SYSTEM_CERTDB as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"use-system-certdb\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_DEPRECATED as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DATABASE as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_INTERACTION as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_REQUIRE_CLOSE_NOTIFY as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_REHANDSHAKE_MODE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"rehandshake-mode\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_rehandshake_mode_get_type(),
            G_TLS_REHANDSHAKE_SAFELY as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_DEPRECATED as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CERTIFICATE as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_PEER_CERTIFICATE as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_PEER_CERTIFICATE_ERRORS as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_ADVERTISED_PROTOCOLS as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_NEGOTIATED_PROTOCOL as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_PROTOCOL_VERSION as ::core::ffi::c_int as guint,
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
    g_object_class_install_property(
        gobject_class,
        PROP_CIPHERSUITE_NAME as ::core::ffi::c_int as guint,
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
        safe_c2rust_g_tls_connection_get_type(),
        G_SIGNAL_RUN_LAST,
        256 as ::core::ffi::c_ulong as glong as guint,
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
        (*(klass as *mut GTypeClass)).g_type,
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
unsafe extern "C" fn safe_c2rust_g_tls_connection_init(mut conn: *mut GTlsConnection) {}
unsafe extern "C" fn safe_c2rust_g_tls_connection_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut _glib__object: *mut GObject = object;
    let mut _glib__pspec: *mut GParamSpec = pspec;
    let mut _glib__property_id: guint = prop_id;
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8 as *const gchar,
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsconnection.c\0" as *const u8
            as *const ::core::ffi::c_char,
        393 as ::core::ffi::c_int,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
        _glib__property_id,
        (*_glib__pspec).name,
        g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
        g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
    );
}
unsafe extern "C" fn safe_c2rust_g_tls_connection_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut _glib__object: *mut GObject = object;
    let mut _glib__pspec: *mut GParamSpec = pspec;
    let mut _glib__property_id: guint = prop_id;
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8 as *const gchar,
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsconnection.c\0" as *const u8
            as *const ::core::ffi::c_char,
        402 as ::core::ffi::c_int,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
        _glib__property_id,
        (*_glib__pspec).name,
        g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
        g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_use_system_certdb(
    mut conn: *mut GTlsConnection,
    mut use_system_certdb: gboolean,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"use-system-certdb\0" as *const u8 as *const gchar,
        use_system_certdb,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_use_system_certdb(
    mut conn: *mut GTlsConnection,
) -> gboolean {
    let mut use_system_certdb: gboolean = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"use-system-certdb\0" as *const u8 as *const gchar,
        &raw mut use_system_certdb,
        NULL_0,
    );
    return use_system_certdb;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_database(
    mut conn: *mut GTlsConnection,
    mut database: *mut GTlsDatabase,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_database(
    mut conn: *mut GTlsConnection,
) -> *mut GTlsDatabase {
    let mut database: *mut GTlsDatabase = ::core::ptr::null_mut::<GTlsDatabase>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_certificate(
    mut conn: *mut GTlsConnection,
    mut certificate: *mut GTlsCertificate,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_certificate(
    mut conn: *mut GTlsConnection,
) -> *mut GTlsCertificate {
    let mut certificate: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_interaction(
    mut conn: *mut GTlsConnection,
    mut interaction: *mut GTlsInteraction,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_interaction(
    mut conn: *mut GTlsConnection,
) -> *mut GTlsInteraction {
    let mut interaction: *mut GTlsInteraction = ::core::ptr::null_mut::<GTlsInteraction>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_peer_certificate(
    mut conn: *mut GTlsConnection,
) -> *mut GTlsCertificate {
    let mut peer_certificate: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_peer_certificate_errors(
    mut conn: *mut GTlsConnection,
) -> GTlsCertificateFlags {
    let mut errors: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_require_close_notify(
    mut conn: *mut GTlsConnection,
    mut require_close_notify: gboolean,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_require_close_notify(
    mut conn: *mut GTlsConnection,
) -> gboolean {
    let mut require_close_notify: gboolean = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_rehandshake_mode(
    mut conn: *mut GTlsConnection,
    mut mode: GTlsRehandshakeMode,
) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_rehandshake_mode(
    mut conn: *mut GTlsConnection,
) -> GTlsRehandshakeMode {
    let mut mode: GTlsRehandshakeMode = G_TLS_REHANDSHAKE_NEVER;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_set_advertised_protocols(
    mut conn: *mut GTlsConnection,
    mut protocols: *const *const gchar,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"advertised-protocols\0" as *const u8 as *const gchar,
        protocols,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_negotiated_protocol(
    mut conn: *mut GTlsConnection,
) -> *const gchar {
    let mut class: *mut GTlsConnectionClass = ::core::ptr::null_mut::<GTlsConnectionClass>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    class = (*(conn as *mut GTypeInstance)).g_class as *mut GTlsConnectionClass;
    if (*class).get_negotiated_protocol.is_none() {
        return ::core::ptr::null::<gchar>();
    }
    return (*class)
        .get_negotiated_protocol
        .expect("non-null function pointer")(conn);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_channel_binding_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(
            b"g-tls-channel-binding-error-quark\0" as *const u8 as *const gchar,
        );
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_channel_binding_data(
    mut conn: *mut GTlsConnection,
    mut type_0: GTlsChannelBindingType,
    mut data: *mut GByteArray,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GTlsConnectionClass = ::core::ptr::null_mut::<GTlsConnectionClass>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(conn as *mut GTypeInstance)).g_class as *mut GTlsConnectionClass;
    if (*class).get_binding_data.is_none() {
        g_set_error_literal(
            error,
            safe_c2rust_g_tls_channel_binding_error_quark(),
            G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"TLS backend does not implement TLS binding retrieval\0" as *const u8
                    as *const gchar,
            ),
        );
        return FALSE;
    }
    return (*class)
        .get_binding_data
        .expect("non-null function pointer")(conn, type_0, data, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_handshake(
    mut conn: *mut GTlsConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(conn as *mut GTypeInstance)).g_class as *mut GTlsConnectionClass))
        .handshake
        .expect("non-null function pointer")(conn, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_handshake_async(
    mut conn: *mut GTlsConnection,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(conn as *mut GTypeInstance)).g_class as *mut GTlsConnectionClass))
        .handshake_async
        .expect("non-null function pointer")(
        conn, io_priority, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_handshake_finish(
    mut conn: *mut GTlsConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(conn as *mut GTypeInstance)).g_class as *mut GTlsConnectionClass))
        .handshake_finish
        .expect("non-null function pointer")(conn, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_protocol_version(
    mut conn: *mut GTlsConnection,
) -> GTlsProtocolVersion {
    let mut protocol_version: GTlsProtocolVersion = G_TLS_PROTOCOL_VERSION_UNKNOWN;
    let mut enum_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_get_ciphersuite_name(
    mut conn: *mut GTlsConnection,
) -> *mut gchar {
    let mut ciphersuite_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-tls-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_connection_emit_accept_certificate(
    mut conn: *mut GTlsConnection,
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
