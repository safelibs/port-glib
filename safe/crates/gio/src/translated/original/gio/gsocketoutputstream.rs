extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableOutputStream;
    pub type _GSocketPrivate;
    pub type _GSocketControlMessagePrivate;
    pub type _GFileDescriptorBased;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_source_unref(source: *mut GSource);
    fn g_source_add_child_source(source: *mut GSource, child_source: *mut GSource);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
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
    fn g_source_set_dummy_callback(source: *mut GSource);
    fn g_output_stream_get_type() -> GType;
    fn g_socket_get_type() -> GType;
    fn g_socket_get_fd(socket: *mut GSocket) -> ::core::ffi::c_int;
    fn g_socket_condition_check(socket: *mut GSocket, condition: GIOCondition) -> GIOCondition;
    fn g_socket_create_source(
        socket: *mut GSocket,
        condition: GIOCondition,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_socket_send_with_blocking(
        socket: *mut GSocket,
        buffer: *const gchar,
        size: gsize,
        blocking: gboolean,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_send_message_with_timeout(
        socket: *mut GSocket,
        address: *mut GSocketAddress,
        vectors: *const GOutputVector,
        num_vectors: gint,
        messages: *mut *mut GSocketControlMessage,
        num_messages: gint,
        flags: gint,
        timeout_us: gint64,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> GPollableReturn;
    fn g_pollable_output_stream_get_type() -> GType;
    fn g_pollable_source_new(pollable_stream: *mut GObject) -> *mut GSource;
    fn g_file_descriptor_based_get_type() -> GType;
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
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
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
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
pub type GOutputStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: GOutputStreamSpliceFlags = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: GOutputStreamSpliceFlags = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: GOutputStreamSpliceFlags = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_SOCKET_MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const G_SOCKET_MSG_PEEK: C2RustUnnamed_0 = 2;
pub const G_SOCKET_MSG_OOB: C2RustUnnamed_0 = 1;
pub const G_SOCKET_MSG_NONE: C2RustUnnamed_0 = 0;
pub type GPollableReturn = ::core::ffi::c_int;
pub const G_POLLABLE_RETURN_WOULD_BLOCK: GPollableReturn = -27;
pub const G_POLLABLE_RETURN_OK: GPollableReturn = 1;
pub const G_POLLABLE_RETURN_FAILED: GPollableReturn = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GPollableOutputStream = _GPollableOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStreamClass {
    pub parent_class: GObjectClass,
    pub write_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub splice: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub flush: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub write_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub splice_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub splice_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub flush_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub flush_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub writev_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            *mut gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub writev_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub writev_finish: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GAsyncResult,
            *mut gsize,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
}
pub type GOutputStreamClass = _GOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketOutputStreamPrivate {
    pub socket: *mut GSocket,
    pub buffer: gconstpointer,
    pub count: gsize,
}
pub type GSocketOutputStreamPrivate = _GSocketOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketOutputStreamClass {
    pub parent_class: GOutputStreamClass,
}
pub type GSocketOutputStreamClass = _GSocketOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GSocketOutputStreamPrivate,
}
pub type GSocketOutputStream = _GSocketOutputStream;
pub const PROP_SOCKET: C2RustUnnamed_1 = 1;
pub type GFileDescriptorBasedIface = _GFileDescriptorBasedIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileDescriptorBasedIface {
    pub g_iface: GTypeInterface,
    pub get_fd: Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>,
}
pub type GFileDescriptorBased = _GFileDescriptorBased;
pub type GPollableOutputStreamInterface = _GPollableOutputStreamInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollableOutputStreamInterface {
    pub g_iface: GTypeInterface,
    pub can_poll: Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>,
    pub is_writable: Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>,
    pub create_source:
        Option<unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource>,
    pub write_nonblocking: Option<
        unsafe extern "C" fn(
            *mut GPollableOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub writev_nonblocking: Option<
        unsafe extern "C" fn(
            *mut GPollableOutputStream,
            *const GOutputVector,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GPollableReturn,
    >,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_IOV_MAX: ::core::ffi::c_int = IOV_MAX;
static mut safe_c2rust_g_socket_output_stream_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_socket_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_output_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_output_stream_get_type(),
        g_intern_static_string(b"GSocketOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_output_stream_init
                    as unsafe extern "C" fn(*mut GSocketOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSocketOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSocketOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socket_output_stream_pollable_iface_init
                as unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_pollable_output_stream_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socket_output_stream_file_descriptor_based_iface_init
                as unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_descriptor_based_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socket_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketOutputStream_private_offset,
        );
    }
    safe_c2rust_g_socket_output_stream_class_init(klass as *mut GSocketOutputStreamClass);
}
static mut safe_c2rust_GSocketOutputStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_get_instance_private(
    mut self_0: *mut GSocketOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSocketOutputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GSocketOutputStream =
        object as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*stream).priv_0).socket as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                90 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GSocketOutputStream =
        object as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    match prop_id {
        1 => {
            (*(*stream).priv_0).socket = g_value_dup_object(value) as *mut GSocket;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_finalize(mut object: *mut GObject) {
    let mut stream: *mut GSocketOutputStream =
        object as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    if !(*(*stream).priv_0).socket.is_null() {
        g_object_unref((*(*stream).priv_0).socket as gpointer);
    }
    (*(safe_c2rust_g_socket_output_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut output_stream: *mut GSocketOutputStream =
        stream as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    return g_socket_send_with_blocking(
        (*(*output_stream).priv_0).socket,
        buffer as *const gchar,
        count,
        TRUE,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_writev(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut output_stream: *mut GSocketOutputStream =
        stream as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    let mut res: GPollableReturn = G_POLLABLE_RETURN_FAILED;
    if n_vectors > G_IOV_MAX as gsize {
        n_vectors = G_IOV_MAX as gsize;
    }
    res = g_socket_send_message_with_timeout(
        (*(*output_stream).priv_0).socket,
        ::core::ptr::null_mut::<GSocketAddress>(),
        vectors,
        n_vectors as gint,
        ::core::ptr::null_mut::<*mut GSocketControlMessage>(),
        0 as gint,
        G_SOCKET_MSG_NONE as ::core::ffi::c_int as gint,
        -(1 as ::core::ffi::c_int) as gint64,
        bytes_written,
        cancellable,
        error,
    );
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if res as ::core::ffi::c_int != G_POLLABLE_RETURN_WOULD_BLOCK as ::core::ffi::c_int {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketoutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            162 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res != G_POLLABLE_RETURN_WOULD_BLOCK\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (res as ::core::ffi::c_int == G_POLLABLE_RETURN_OK as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_pollable_is_writable(
    mut pollable: *mut GPollableOutputStream,
) -> gboolean {
    let mut output_stream: *mut GSocketOutputStream =
        pollable as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    return g_socket_condition_check((*(*output_stream).priv_0).socket, G_IO_OUT) as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_pollable_write_nonblocking(
    mut pollable: *mut GPollableOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut size: gsize,
    mut error: *mut *mut GError,
) -> gssize {
    let mut output_stream: *mut GSocketOutputStream =
        pollable as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    return g_socket_send_with_blocking(
        (*(*output_stream).priv_0).socket,
        buffer as *const gchar,
        size,
        FALSE,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_pollable_writev_nonblocking(
    mut pollable: *mut GPollableOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> GPollableReturn {
    let mut output_stream: *mut GSocketOutputStream =
        pollable as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    if n_vectors > G_IOV_MAX as gsize {
        n_vectors = G_IOV_MAX as gsize;
    }
    return g_socket_send_message_with_timeout(
        (*(*output_stream).priv_0).socket,
        ::core::ptr::null_mut::<GSocketAddress>(),
        vectors,
        n_vectors as gint,
        ::core::ptr::null_mut::<*mut GSocketControlMessage>(),
        0 as gint,
        G_SOCKET_MSG_NONE as ::core::ffi::c_int as gint,
        0 as gint64,
        bytes_written,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_pollable_create_source(
    mut pollable: *mut GPollableOutputStream,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut output_stream: *mut GSocketOutputStream =
        pollable as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    let mut socket_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut pollable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    pollable_source =
        g_pollable_source_new(output_stream as *mut ::core::ffi::c_void as *mut GObject);
    socket_source =
        g_socket_create_source((*(*output_stream).priv_0).socket, G_IO_OUT, cancellable);
    g_source_set_dummy_callback(socket_source);
    g_source_add_child_source(pollable_source, socket_source);
    g_source_unref(socket_source);
    return pollable_source;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_get_fd(
    mut fd_based: *mut GFileDescriptorBased,
) -> ::core::ffi::c_int {
    let mut output_stream: *mut GSocketOutputStream =
        fd_based as *mut ::core::ffi::c_void as *mut GSocketOutputStream;
    return g_socket_get_fd((*(*output_stream).priv_0).socket);
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_class_init(
    mut klass: *mut GSocketOutputStreamClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut goutputstream_class: *mut GOutputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GOutputStreamClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_socket_output_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_socket_output_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_socket_output_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*goutputstream_class).write_fn = Some(
        safe_c2rust_g_socket_output_stream_write
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*goutputstream_class).writev_fn = Some(
        safe_c2rust_g_socket_output_stream_writev
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    g_object_class_install_property(
        gobject_class,
        PROP_SOCKET as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"socket\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_file_descriptor_based_iface_init(
    mut iface: *mut GFileDescriptorBasedIface,
) {
    (*iface).get_fd = Some(
        safe_c2rust_g_socket_output_stream_get_fd
            as unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_pollable_iface_init(
    mut iface: *mut GPollableOutputStreamInterface,
) {
    (*iface).is_writable = Some(
        safe_c2rust_g_socket_output_stream_pollable_is_writable
            as unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>;
    (*iface).create_source = Some(
        safe_c2rust_g_socket_output_stream_pollable_create_source
            as unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
        >;
    (*iface).write_nonblocking = Some(
        safe_c2rust_g_socket_output_stream_pollable_write_nonblocking
            as unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*iface).writev_nonblocking = Some(
        safe_c2rust_g_socket_output_stream_pollable_writev_nonblocking
            as unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GPollableReturn,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GPollableReturn,
        >;
}
unsafe extern "C" fn safe_c2rust_g_socket_output_stream_init(mut stream: *mut GSocketOutputStream) {
    (*stream).priv_0 = safe_c2rust_g_socket_output_stream_get_instance_private(stream)
        as *mut GSocketOutputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_socket_output_stream_new(
    mut socket: *mut GSocket,
) -> *mut GSocketOutputStream {
    return g_object_new(
        safe_c2rust__g_socket_output_stream_get_type(),
        b"socket\0" as *const u8 as *const gchar,
        socket,
        NULL,
    ) as *mut GSocketOutputStream;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
