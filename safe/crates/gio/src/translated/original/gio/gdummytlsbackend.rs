extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInputStreamPrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GOutputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketConnectable;
    pub type _GTlsCertificatePrivate;
    pub type _GTlsConnectionPrivate;
    pub type _GTlsDatabasePrivate;
    pub type _GTlsInteraction;
    pub type _GTlsBackend;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_initable_get_type() -> GType;
    fn g_dtls_connection_get_type() -> GType;
    fn g_dtls_client_connection_get_type() -> GType;
    fn g_dtls_server_connection_get_type() -> GType;
    fn g_tls_backend_get_type() -> GType;
    fn g_tls_certificate_get_type() -> GType;
    fn g_tls_connection_get_type() -> GType;
    fn g_tls_error_quark() -> GQuark;
    fn g_tls_client_connection_get_type() -> GType;
    fn g_tls_database_get_type() -> GType;
    fn g_tls_file_database_get_type() -> GType;
    fn g_tls_server_connection_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_TLS_ERROR_BAD_CERTIFICATE_PASSWORD: C2RustUnnamed_0 = 8;
pub const G_TLS_ERROR_INAPPROPRIATE_FALLBACK: C2RustUnnamed_0 = 7;
pub const G_TLS_ERROR_EOF: C2RustUnnamed_0 = 6;
pub const G_TLS_ERROR_CERTIFICATE_REQUIRED: C2RustUnnamed_0 = 5;
pub const G_TLS_ERROR_HANDSHAKE: C2RustUnnamed_0 = 4;
pub const G_TLS_ERROR_NOT_TLS: C2RustUnnamed_0 = 3;
pub const G_TLS_ERROR_BAD_CERTIFICATE: C2RustUnnamed_0 = 2;
pub const G_TLS_ERROR_MISC: C2RustUnnamed_0 = 1;
pub const G_TLS_ERROR_UNAVAILABLE: C2RustUnnamed_0 = 0;
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
pub type GTlsDatabaseVerifyFlags = ::core::ffi::c_uint;
pub const G_TLS_DATABASE_VERIFY_NONE: GTlsDatabaseVerifyFlags = 0;
pub type GTlsDatabaseLookupFlags = ::core::ffi::c_uint;
pub const G_TLS_DATABASE_LOOKUP_KEYPAIR: GTlsDatabaseLookupFlags = 1;
pub const G_TLS_DATABASE_LOOKUP_NONE: GTlsDatabaseLookupFlags = 0;
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
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
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
pub type GSocketConnectable = _GSocketConnectable;
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
pub type GTlsInteraction = _GTlsInteraction;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsBackend {
    pub parent_instance: GObject,
    pub database: *mut GTlsDatabase,
}
pub type GDummyTlsBackend = _GDummyTlsBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsBackendClass {
    pub parent_class: GObjectClass,
}
pub type GDummyTlsBackendClass = _GDummyTlsBackendClass;
pub type GTlsBackendInterface = _GTlsBackendInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsBackendInterface {
    pub g_iface: GTypeInterface,
    pub supports_tls: Option<unsafe extern "C" fn(*mut GTlsBackend) -> gboolean>,
    pub get_certificate_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_client_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_server_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_file_database_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_default_database: Option<unsafe extern "C" fn(*mut GTlsBackend) -> *mut GTlsDatabase>,
    pub supports_dtls: Option<unsafe extern "C" fn(*mut GTlsBackend) -> gboolean>,
    pub get_dtls_client_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_dtls_server_connection_type: Option<unsafe extern "C" fn() -> GType>,
}
pub type GTlsBackend = _GTlsBackend;
pub type GDummyTlsDatabase = _GDummyTlsDatabase;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsDatabase {
    pub parent_instance: GTlsDatabase,
}
pub type GDummyTlsDatabaseClass = _GDummyTlsDatabaseClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsDatabaseClass {
    pub parent_class: GTlsDatabaseClass,
}
pub type GTlsDatabaseClass = _GTlsDatabaseClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsDatabaseClass {
    pub parent_class: GObjectClass,
    pub verify_chain: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *const gchar,
            *mut GSocketConnectable,
            *mut GTlsInteraction,
            GTlsDatabaseVerifyFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> GTlsCertificateFlags,
    >,
    pub verify_chain_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *const gchar,
            *mut GSocketConnectable,
            *mut GTlsInteraction,
            GTlsDatabaseVerifyFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub verify_chain_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> GTlsCertificateFlags,
    >,
    pub create_certificate_handle:
        Option<unsafe extern "C" fn(*mut GTlsDatabase, *mut GTlsCertificate) -> *mut gchar>,
    pub lookup_certificate_for_handle: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *const gchar,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_for_handle_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *const gchar,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificate_for_handle_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_issuer: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_issuer_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificate_issuer_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificates_issued_by: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GByteArray,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub lookup_certificates_issued_by_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GByteArray,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificates_issued_by_finish: Option<
        unsafe extern "C" fn(*mut GTlsDatabase, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub padding: [gpointer; 16],
}
pub const PROP_ANCHORS: C2RustUnnamed_4 = 1;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GTlsFileDatabaseInterface = _GTlsFileDatabaseInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsFileDatabaseInterface {
    pub g_iface: GTypeInterface,
    pub padding: [gpointer; 8],
}
pub type GDummyDtlsConnection = _GDummyDtlsConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyDtlsConnection {
    pub parent_instance: GObject,
}
pub type GDummyDtlsConnectionClass = _GDummyDtlsConnectionClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyDtlsConnectionClass {
    pub parent_class: GObjectClass,
}
pub const PROP_DTLS_CONN_AUTHENTICATION_MODE: C2RustUnnamed_3 = 13;
pub const PROP_DTLS_CONN_ACCEPTED_CAS: C2RustUnnamed_3 = 12;
pub const PROP_DTLS_CONN_SERVER_IDENTITY: C2RustUnnamed_3 = 10;
pub const PROP_DTLS_CONN_VALIDATION_FLAGS: C2RustUnnamed_3 = 9;
pub const PROP_DTLS_CONN_PEER_CERTIFICATE_ERRORS: C2RustUnnamed_3 = 8;
pub const PROP_DTLS_CONN_PEER_CERTIFICATE: C2RustUnnamed_3 = 7;
pub const PROP_DTLS_CONN_INTERACTION: C2RustUnnamed_3 = 6;
pub const PROP_DTLS_CONN_DATABASE: C2RustUnnamed_3 = 5;
pub const PROP_DTLS_CONN_CERTIFICATE: C2RustUnnamed_3 = 4;
pub const PROP_DTLS_CONN_REHANDSHAKE_MODE: C2RustUnnamed_3 = 3;
pub const PROP_DTLS_CONN_REQUIRE_CLOSE_NOTIFY: C2RustUnnamed_3 = 2;
pub const PROP_DTLS_CONN_BASE_SOCKET: C2RustUnnamed_3 = 1;
pub type GDummyTlsConnection = _GDummyTlsConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsConnection {
    pub parent_instance: GTlsConnection,
}
pub type GDummyTlsConnectionClass = _GDummyTlsConnectionClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsConnectionClass {
    pub parent_class: GTlsConnectionClass,
}
pub type GTlsConnectionClass = _GTlsConnectionClass;
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
pub type GIOStreamClass = _GIOStreamClass;
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
pub const PROP_CONN_NEGOTIATED_PROTOCOL: C2RustUnnamed_2 = 16;
pub const PROP_CONN_ADVERTISED_PROTOCOLS: C2RustUnnamed_2 = 15;
pub const PROP_CONN_AUTHENTICATION_MODE: C2RustUnnamed_2 = 14;
pub const PROP_CONN_ACCEPTED_CAS: C2RustUnnamed_2 = 13;
pub const PROP_CONN_USE_SSL3: C2RustUnnamed_2 = 12;
pub const PROP_CONN_SERVER_IDENTITY: C2RustUnnamed_2 = 11;
pub const PROP_CONN_VALIDATION_FLAGS: C2RustUnnamed_2 = 10;
pub const PROP_CONN_PEER_CERTIFICATE_ERRORS: C2RustUnnamed_2 = 9;
pub const PROP_CONN_PEER_CERTIFICATE: C2RustUnnamed_2 = 8;
pub const PROP_CONN_INTERACTION: C2RustUnnamed_2 = 7;
pub const PROP_CONN_DATABASE: C2RustUnnamed_2 = 6;
pub const PROP_CONN_CERTIFICATE: C2RustUnnamed_2 = 5;
pub const PROP_CONN_REHANDSHAKE_MODE: C2RustUnnamed_2 = 4;
pub const PROP_CONN_REQUIRE_CLOSE_NOTIFY: C2RustUnnamed_2 = 3;
pub const PROP_CONN_USE_SYSTEM_CERTDB: C2RustUnnamed_2 = 2;
pub const PROP_CONN_BASE_IO_STREAM: C2RustUnnamed_2 = 1;
pub type GDummyTlsCertificate = _GDummyTlsCertificate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsCertificate {
    pub parent_instance: GTlsCertificate,
}
pub type GDummyTlsCertificateClass = _GDummyTlsCertificateClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyTlsCertificateClass {
    pub parent_class: GTlsCertificateClass,
}
pub type GTlsCertificateClass = _GTlsCertificateClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificateClass {
    pub parent_class: GObjectClass,
    pub verify: Option<
        unsafe extern "C" fn(
            *mut GTlsCertificate,
            *mut GSocketConnectable,
            *mut GTlsCertificate,
        ) -> GTlsCertificateFlags,
    >,
    pub padding: [gpointer; 8],
}
pub const PROP_CERT_ISSUER: C2RustUnnamed_1 = 5;
pub const PROP_CERT_PRIVATE_KEY_PEM: C2RustUnnamed_1 = 4;
pub const PROP_CERT_PRIVATE_KEY: C2RustUnnamed_1 = 3;
pub const PROP_CERT_CERTIFICATE_PEM: C2RustUnnamed_1 = 2;
pub const PROP_CERT_CERTIFICATE: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_CERTIFICATE_0: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_CONNECTION_0: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const PROP_DTLS_CONN_ENABLE_NEGOTIATION: C2RustUnnamed_3 = 11;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const PROP_DATABASE_0: C2RustUnnamed_4 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDummyTlsBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyTlsBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyTlsBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyTlsBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_backend_init
                    as unsafe extern "C" fn(*mut GDummyTlsBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GTlsBackendInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_tls_backend_iface_init
                as unsafe extern "C" fn(*mut GTlsBackendInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_tls_backend_get_type(),
        &raw const g_implement_interface_info,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-tls-backend\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"dummy\0" as *const u8 as *const ::core::ffi::c_char,
        -(100 as gint),
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDummyTlsBackend_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dummy_tls_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_tls_backend_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_tls_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyTlsBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDummyTlsBackend_private_offset,
        );
    }
    safe_c2rust_g_dummy_tls_backend_class_init(klass as *mut GDummyTlsBackendClass);
}
static mut safe_c2rust_g_dummy_tls_backend_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_init(mut dummy: *mut GDummyTlsBackend) {}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_finalize(mut object: *mut GObject) {
    let mut dummy: *mut GDummyTlsBackend =
        object as *mut ::core::ffi::c_void as *mut GDummyTlsBackend;
    let mut _pp: *mut *mut GTlsDatabase = &raw mut (*dummy).database;
    let mut _ptr: *mut GTlsDatabase = *_pp;
    *_pp = ::core::ptr::null_mut::<GTlsDatabase>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_dummy_tls_backend_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_class_init(
    mut backend_class: *mut GDummyTlsBackendClass,
) {
    let mut object_class: *mut GObjectClass =
        backend_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_dummy_tls_backend_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_get_default_database(
    mut backend: *mut GTlsBackend,
) -> *mut GTlsDatabase {
    let mut dummy: *mut GDummyTlsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDummyTlsBackend;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*dummy).database;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GTlsDatabase = ::core::ptr::null_mut::<GTlsDatabase>();
            let mut gapg_temp_atomic: *mut *mut GTlsDatabase = &raw mut (*dummy).database;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut (*dummy).database as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut tlsdb: *mut GTlsDatabase = ::core::ptr::null_mut::<GTlsDatabase>();
        tlsdb = g_object_new(
            safe_c2rust__g_dummy_tls_database_get_type(),
            ::core::ptr::null::<gchar>(),
        ) as *mut GTlsDatabase;
        if 0 as ::core::ffi::c_int != 0 {
            (*dummy).database = tlsdb;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut (*dummy).database as *mut ::core::ffi::c_void,
            tlsdb as guintptr as gpointer,
        );
    }
    return g_object_ref((*dummy).database as gpointer) as *mut GTlsDatabase;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_backend_iface_init(
    mut iface: *mut GTlsBackendInterface,
) {
    (*iface).get_certificate_type =
        Some(safe_c2rust__g_dummy_tls_certificate_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_client_connection_type =
        Some(safe_c2rust__g_dummy_tls_connection_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_server_connection_type =
        Some(safe_c2rust__g_dummy_tls_connection_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_dtls_client_connection_type =
        Some(safe_c2rust__g_dummy_dtls_connection_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_dtls_server_connection_type =
        Some(safe_c2rust__g_dummy_dtls_connection_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_file_database_type =
        Some(safe_c2rust__g_dummy_tls_database_get_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
    (*iface).get_default_database = Some(
        safe_c2rust_g_dummy_tls_backend_get_default_database
            as unsafe extern "C" fn(*mut GTlsBackend) -> *mut GTlsDatabase,
    )
        as Option<unsafe extern "C" fn(*mut GTlsBackend) -> *mut GTlsDatabase>;
}
static mut safe_c2rust_g_dummy_tls_certificate_parent_class: gpointer = NULL;
static mut safe_c2rust_GDummyTlsCertificate_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_tls_certificate_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyTlsCertificate_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDummyTlsCertificate_private_offset,
        );
    }
    safe_c2rust_g_dummy_tls_certificate_class_init(klass as *mut GDummyTlsCertificateClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_tls_certificate_get_type(),
        g_intern_static_string(b"GDummyTlsCertificate\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyTlsCertificateClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_certificate_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyTlsCertificate>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyTlsCertificate) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_certificate_init
                    as unsafe extern "C" fn(*mut GDummyTlsCertificate) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_tls_certificate_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust__g_dummy_tls_certificate_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_tls_certificate_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_class_init(
    mut certificate_class: *mut GDummyTlsCertificateClass,
) {
    let mut gobject_class: *mut GObjectClass =
        certificate_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dummy_tls_certificate_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dummy_tls_certificate_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_CERT_CERTIFICATE as ::core::ffi::c_int as guint,
        b"certificate\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CERT_CERTIFICATE_PEM as ::core::ffi::c_int as guint,
        b"certificate-pem\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CERT_PRIVATE_KEY as ::core::ffi::c_int as guint,
        b"private-key\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CERT_PRIVATE_KEY_PEM as ::core::ffi::c_int as guint,
        b"private-key-pem\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CERT_ISSUER as ::core::ffi::c_int as guint,
        b"issuer\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_init(
    mut certificate: *mut GDummyTlsCertificate,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_tls_error_quark(),
        G_TLS_ERROR_UNAVAILABLE as ::core::ffi::c_int as gint,
        glib_gettext(b"TLS support is not available\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_certificate_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_dummy_tls_certificate_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_tls_connection_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyTlsConnection_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDummyTlsConnection_private_offset,
        );
    }
    safe_c2rust_g_dummy_tls_connection_class_init(klass as *mut GDummyTlsConnectionClass);
}
unsafe extern "C" fn safe_c2rust__g_dummy_tls_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_tls_connection_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_tls_connection_get_type(),
        g_intern_static_string(b"GDummyTlsConnection\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyTlsConnectionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_connection_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyTlsConnection>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyTlsConnection) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_connection_init
                    as unsafe extern "C" fn(*mut GDummyTlsConnection) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>())),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_tls_client_connection_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>())),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_tls_server_connection_get_type(),
        &raw const g_implement_interface_info_0,
    );
    let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_tls_connection_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info_1,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDummyTlsConnection_private_offset: gint = 0;
static mut safe_c2rust_g_dummy_tls_connection_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_close(
    mut stream: *mut GIOStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_class_init(
    mut connection_class: *mut GDummyTlsConnectionClass,
) {
    let mut gobject_class: *mut GObjectClass =
        connection_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut io_stream_class: *mut GIOStreamClass =
        connection_class as *mut ::core::ffi::c_void as *mut GIOStreamClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dummy_tls_connection_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dummy_tls_connection_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*io_stream_class).close_fn = Some(
        safe_c2rust_g_dummy_tls_connection_close
            as unsafe extern "C" fn(
                *mut GIOStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GIOStream, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_BASE_IO_STREAM as ::core::ffi::c_int as guint,
        b"base-io-stream\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_USE_SYSTEM_CERTDB as ::core::ffi::c_int as guint,
        b"use-system-certdb\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_REQUIRE_CLOSE_NOTIFY as ::core::ffi::c_int as guint,
        b"require-close-notify\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_REHANDSHAKE_MODE as ::core::ffi::c_int as guint,
        b"rehandshake-mode\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_CERTIFICATE as ::core::ffi::c_int as guint,
        b"certificate\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_DATABASE as ::core::ffi::c_int as guint,
        b"database\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_INTERACTION as ::core::ffi::c_int as guint,
        b"interaction\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_PEER_CERTIFICATE as ::core::ffi::c_int as guint,
        b"peer-certificate\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_PEER_CERTIFICATE_ERRORS as ::core::ffi::c_int as guint,
        b"peer-certificate-errors\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_VALIDATION_FLAGS as ::core::ffi::c_int as guint,
        b"validation-flags\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_SERVER_IDENTITY as ::core::ffi::c_int as guint,
        b"server-identity\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_USE_SSL3 as ::core::ffi::c_int as guint,
        b"use-ssl3\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_ACCEPTED_CAS as ::core::ffi::c_int as guint,
        b"accepted-cas\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_AUTHENTICATION_MODE as ::core::ffi::c_int as guint,
        b"authentication-mode\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_ADVERTISED_PROTOCOLS as ::core::ffi::c_int as guint,
        b"advertised-protocols\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONN_NEGOTIATED_PROTOCOL as ::core::ffi::c_int as guint,
        b"negotiated-protocol\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_init(
    mut connection: *mut GDummyTlsConnection,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_tls_error_quark(),
        G_TLS_ERROR_UNAVAILABLE as ::core::ffi::c_int as gint,
        glib_gettext(b"TLS support is not available\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_connection_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_dummy_tls_connection_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
static mut safe_c2rust_g_dummy_dtls_connection_parent_class: gpointer = NULL;
static mut safe_c2rust_GDummyDtlsConnection_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust__g_dummy_dtls_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_dtls_connection_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDummyDtlsConnection\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyDtlsConnectionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_dtls_connection_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyDtlsConnection>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyDtlsConnection) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_dtls_connection_init
                    as unsafe extern "C" fn(*mut GDummyDtlsConnection) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>())),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dtls_connection_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>())),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dtls_client_connection_get_type(),
        &raw const g_implement_interface_info_0,
    );
    let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>())),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dtls_server_connection_get_type(),
        &raw const g_implement_interface_info_1,
    );
    let g_implement_interface_info_2: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_dtls_connection_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info_2,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_dtls_connection_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyDtlsConnection_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDummyDtlsConnection_private_offset,
        );
    }
    safe_c2rust_g_dummy_dtls_connection_class_init(klass as *mut GDummyDtlsConnectionClass);
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_class_init(
    mut connection_class: *mut GDummyDtlsConnectionClass,
) {
    let mut gobject_class: *mut GObjectClass =
        connection_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dummy_dtls_connection_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dummy_dtls_connection_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_BASE_SOCKET as ::core::ffi::c_int as guint,
        b"base-socket\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_REQUIRE_CLOSE_NOTIFY as ::core::ffi::c_int as guint,
        b"require-close-notify\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_REHANDSHAKE_MODE as ::core::ffi::c_int as guint,
        b"rehandshake-mode\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_CERTIFICATE as ::core::ffi::c_int as guint,
        b"certificate\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_DATABASE as ::core::ffi::c_int as guint,
        b"database\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_INTERACTION as ::core::ffi::c_int as guint,
        b"interaction\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_PEER_CERTIFICATE as ::core::ffi::c_int as guint,
        b"peer-certificate\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_PEER_CERTIFICATE_ERRORS as ::core::ffi::c_int as guint,
        b"peer-certificate-errors\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_VALIDATION_FLAGS as ::core::ffi::c_int as guint,
        b"validation-flags\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_SERVER_IDENTITY as ::core::ffi::c_int as guint,
        b"server-identity\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_ACCEPTED_CAS as ::core::ffi::c_int as guint,
        b"accepted-cas\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DTLS_CONN_AUTHENTICATION_MODE as ::core::ffi::c_int as guint,
        b"authentication-mode\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_init(
    mut connection: *mut GDummyDtlsConnection,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_tls_error_quark(),
        G_TLS_ERROR_UNAVAILABLE as ::core::ffi::c_int as gint,
        glib_gettext(b"DTLS support is not available\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_dtls_connection_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_dummy_dtls_connection_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust__g_dummy_tls_database_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_tls_database_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_tls_database_get_type(),
        g_intern_static_string(b"GDummyTlsDatabase\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyTlsDatabaseClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_database_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyTlsDatabase>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyTlsDatabase) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_tls_database_init
                    as unsafe extern "C" fn(*mut GDummyTlsDatabase) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GTlsFileDatabaseInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_tls_database_file_database_iface_init
                as unsafe extern "C" fn(*mut GTlsFileDatabaseInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_tls_file_database_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_tls_database_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_tls_database_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyTlsDatabase_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDummyTlsDatabase_private_offset,
        );
    }
    safe_c2rust_g_dummy_tls_database_class_init(klass as *mut GDummyTlsDatabaseClass);
}
static mut safe_c2rust_g_dummy_tls_database_parent_class: gpointer = NULL;
static mut safe_c2rust_GDummyTlsDatabase_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_class_init(
    mut database_class: *mut GDummyTlsDatabaseClass,
) {
    let mut gobject_class: *mut GObjectClass =
        database_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dummy_tls_database_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dummy_tls_database_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_ANCHORS as ::core::ffi::c_int as guint,
        b"anchors\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_init(mut database: *mut GDummyTlsDatabase) {}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_file_database_iface_init(
    mut iface: *mut GTlsFileDatabaseInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_tls_error_quark(),
        G_TLS_ERROR_UNAVAILABLE as ::core::ffi::c_int as gint,
        glib_gettext(b"TLS support is not available\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_tls_database_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_dummy_tls_database_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
