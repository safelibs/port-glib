use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GOutputStreamPrivate;
    pub type _GInputStream;
    pub type _GSeekable;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_output_stream_get_type() -> GType;
    fn g_output_stream_write_all(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_filter_output_stream_get_type() -> GType;
    fn g_seekable_get_type() -> GType;
    fn g_seekable_tell(seekable: *mut GSeekable) -> goffset;
    fn g_seekable_can_seek(seekable: *mut GSeekable) -> gboolean;
    fn g_seekable_seek(
        seekable: *mut GSeekable,
        offset: goffset,
        type_0: GSeekType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_seekable_can_truncate(seekable: *mut GSeekable) -> gboolean;
    fn g_seekable_truncate(
        seekable: *mut GSeekable,
        offset: goffset,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_data_stream_byte_order_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
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
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
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
pub type GDataStreamByteOrder = ::core::ffi::c_uint;
pub const G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN: GDataStreamByteOrder = 2;
pub const G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN: GDataStreamByteOrder = 1;
pub const G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN: GDataStreamByteOrder = 0;
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
pub type GOutputStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: GOutputStreamSpliceFlags = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: GOutputStreamSpliceFlags = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: GOutputStreamSpliceFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterOutputStream {
    pub parent_instance: GOutputStream,
    pub base_stream: *mut GOutputStream,
}
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
pub type GFilterOutputStream = _GFilterOutputStream;
pub type GInputStream = _GInputStream;
pub type GSeekable = _GSeekable;
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
pub struct _GFilterOutputStreamClass {
    pub parent_class: GOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFilterOutputStreamClass = _GFilterOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataOutputStream {
    pub parent_instance: GFilterOutputStream,
    pub priv_0: *mut GDataOutputStreamPrivate,
}
pub type GDataOutputStreamPrivate = _GDataOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataOutputStreamPrivate {
    pub byte_order: GDataStreamByteOrder,
}
pub type GDataOutputStream = _GDataOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataOutputStreamClass {
    pub parent_class: GFilterOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GDataOutputStreamClass = _GDataOutputStreamClass;
pub const PROP_BYTE_ORDER: C2RustUnnamed_1 = 1;
pub type GSeekableIface = _GSeekableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSeekableIface {
    pub g_iface: GTypeInterface,
    pub tell: Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GSeekable,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_truncate: Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>,
    pub truncate_fn: Option<
        unsafe extern "C" fn(
            *mut GSeekable,
            goffset,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_data_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_data_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDataOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDataOutputStream_private_offset,
        );
    }
    safe_c2rust_g_data_output_stream_class_init(klass as *mut GDataOutputStreamClass);
}
static mut safe_c2rust_GDataOutputStream_private_offset: gint = 0;
static mut safe_c2rust_g_data_output_stream_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_data_output_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_data_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_filter_output_stream_get_type(),
        g_intern_static_string(b"GDataOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDataOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_data_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDataOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDataOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_data_output_stream_init
                    as unsafe extern "C" fn(*mut GDataOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDataOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDataOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_data_output_stream_seekable_iface_init
                as unsafe extern "C" fn(*mut GSeekableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_seekable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_data_output_stream_get_instance_private(
    mut self_0: *mut GDataOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDataOutputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_class_init(
    mut klass: *mut GDataOutputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_data_output_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_data_output_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_BYTE_ORDER as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"byte-order\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_data_stream_byte_order_get_type(),
            G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut dstream: *mut GDataOutputStream = ::core::ptr::null_mut::<GDataOutputStream>();
    dstream = object as *mut ::core::ffi::c_void as *mut GDataOutputStream;
    match prop_id {
        1 => {
            safe_c2rust_g_data_output_stream_set_byte_order(
                dstream,
                g_value_get_enum(value) as GDataStreamByteOrder,
            );
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdataoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GDataOutputStreamPrivate =
        ::core::ptr::null_mut::<GDataOutputStreamPrivate>();
    let mut dstream: *mut GDataOutputStream = ::core::ptr::null_mut::<GDataOutputStream>();
    dstream = object as *mut ::core::ffi::c_void as *mut GDataOutputStream;
    priv_0 = (*dstream).priv_0;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*priv_0).byte_order as gint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdataoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_init(mut stream: *mut GDataOutputStream) {
    (*stream).priv_0 = safe_c2rust_g_data_output_stream_get_instance_private(stream)
        as *mut GDataOutputStreamPrivate;
    (*(*stream).priv_0).byte_order = G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN;
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_data_output_stream_tell as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_data_output_stream_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_data_output_stream_seek
            as unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).can_truncate = Some(
        safe_c2rust_g_data_output_stream_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_data_output_stream_truncate
            as unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_new(
    mut base_stream: *mut GOutputStream,
) -> *mut GDataOutputStream {
    let mut stream: *mut GDataOutputStream = ::core::ptr::null_mut::<GDataOutputStream>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
            let mut __t: GType = g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDataOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_data_output_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        NULL,
    ) as *mut GDataOutputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_set_byte_order(
    mut stream: *mut GDataOutputStream,
    mut order: GDataStreamByteOrder,
) {
    let mut priv_0: *mut GDataOutputStreamPrivate =
        ::core::ptr::null_mut::<GDataOutputStreamPrivate>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    if (*priv_0).byte_order as ::core::ffi::c_uint != order as ::core::ffi::c_uint {
        (*priv_0).byte_order = order;
        g_object_notify(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            b"byte-order\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_get_byte_order(
    mut stream: *mut GDataOutputStream,
) -> GDataStreamByteOrder {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN;
    }
    return (*(*stream).priv_0).byte_order;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_byte(
    mut stream: *mut GDataOutputStream,
    mut data: guchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        1 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_int16(
    mut stream: *mut GDataOutputStream,
    mut data: gint16,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ((data as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int
                | ((data as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int) as guint16 as gint16;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        2 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_uint16(
    mut stream: *mut GDataOutputStream,
    mut data: guint16,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ((data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int
                | ((data as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int) as guint16;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        2 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_int32(
    mut stream: *mut GDataOutputStream,
    mut data: gint32,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = data as guint32;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh0 = &mut __v;
                    let fresh1;
                    let fresh2 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                }
                __v
            }) as gint32;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        4 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_uint32(
    mut stream: *mut GDataOutputStream,
    mut data: guint32,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = data;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh3 = &mut __v;
                    let fresh4;
                    let fresh5 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                }
                __v
            });
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        4 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_int64(
    mut stream: *mut GDataOutputStream,
    mut data: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint64 = 0;
                let mut __x: guint64 = data as guint64;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                        | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                            >> 24 as ::core::ffi::c_int
                        | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                            >> 40 as ::core::ffi::c_int
                        | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                            >> 56 as ::core::ffi::c_int;
                } else {
                    let fresh6 = &mut __v;
                    let fresh7;
                    let fresh8 = __x;
                    asm!(
                        "bswapq {0}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
                }
                __v
            }) as gint64;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        8 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_uint64(
    mut stream: *mut GDataOutputStream,
    mut data: guint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint64 = 0;
                let mut __x: guint64 = data;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                        | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                            >> 24 as ::core::ffi::c_int
                        | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                            >> 40 as ::core::ffi::c_int
                        | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                            >> 56 as ::core::ffi::c_int;
                } else {
                    let fresh9 = &mut __v;
                    let fresh10;
                    let fresh11 = __x;
                    asm!(
                        "bswapq {0}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) => fresh10,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
                }
                __v
            });
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        &raw mut data as *const ::core::ffi::c_void,
        8 as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_output_stream_put_string(
    mut stream: *mut GDataOutputStream,
    mut str: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes_written: gsize = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_output_stream_get_type();
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
            b"G_IS_DATA_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_output_stream_write_all(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        str as *const ::core::ffi::c_void,
        strlen(str) as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        return 0 as goffset;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_tell(base_stream_seekable);
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    return (({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
        && g_seekable_can_seek(base_stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Seek not supported on base stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_seek(base_stream_seekable, offset, type_0, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    return (({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
        && g_seekable_can_truncate(base_stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_data_output_stream_truncate(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Truncate not supported on base stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_truncate(base_stream_seekable, offset, cancellable, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
