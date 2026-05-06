extern "C" {
    pub type _GData;
    pub type _GConverter;
    pub type _GFileInfo;
    pub type internal_state;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
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
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_converter_get_type() -> GType;
    fn g_file_info_get_type() -> GType;
    fn g_file_info_new() -> *mut GFileInfo;
    fn g_file_info_set_attribute_byte_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: *const ::core::ffi::c_char,
    );
    fn g_file_info_set_attribute_uint32(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: guint32,
    );
    fn g_file_info_set_attribute_uint64(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: guint64,
    );
    fn inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateGetHeader(strm: z_streamp, head: gz_headerp) -> ::core::ffi::c_int;
    fn inflateInit_(
        strm: z_streamp,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn g_io_error_quark() -> GQuark;
    fn g_zlib_compressor_format_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
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
pub type GConverterFlags = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSH: GConverterFlags = 2;
pub const G_CONVERTER_INPUT_AT_END: GConverterFlags = 1;
pub const G_CONVERTER_NO_FLAGS: GConverterFlags = 0;
pub type GConverterResult = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSHED: GConverterResult = 3;
pub const G_CONVERTER_FINISHED: GConverterResult = 2;
pub const G_CONVERTER_CONVERTED: GConverterResult = 1;
pub const G_CONVERTER_ERROR: GConverterResult = 0;
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
pub type GZlibCompressorFormat = ::core::ffi::c_uint;
pub const G_ZLIB_COMPRESSOR_FORMAT_RAW: GZlibCompressorFormat = 2;
pub const G_ZLIB_COMPRESSOR_FORMAT_GZIP: GZlibCompressorFormat = 1;
pub const G_ZLIB_COMPRESSOR_FORMAT_ZLIB: GZlibCompressorFormat = 0;
pub type GConverter = _GConverter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GZlibDecompressor {
    pub parent_instance: GObject,
    pub format: GZlibCompressorFormat,
    pub zstream: z_stream,
    pub header_data: *mut HeaderData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeaderData {
    pub gzheader: gz_header,
    pub filename: [::core::ffi::c_char; 257],
    pub file_info: *mut GFileInfo,
}
pub type GFileInfo = _GFileInfo;
pub type gz_header = gz_header_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: ::core::ffi::c_int,
    pub time: uLong,
    pub xflags: ::core::ffi::c_int,
    pub os: ::core::ffi::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: ::core::ffi::c_int,
    pub done: ::core::ffi::c_int,
}
pub type uInt = ::core::ffi::c_uint;
pub type Bytef = Byte;
pub type Byte = ::core::ffi::c_uchar;
pub type uLong = ::core::ffi::c_ulong;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type voidpf = *mut ::core::ffi::c_void;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type GZlibDecompressor = _GZlibDecompressor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GConverterIface {
    pub g_iface: GTypeInterface,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut GConverter,
            *const ::core::ffi::c_void,
            gsize,
            *mut ::core::ffi::c_void,
            gsize,
            GConverterFlags,
            *mut gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GConverterResult,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut GConverter) -> ()>,
}
pub type GConverterIface = _GConverterIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GZlibDecompressorClass {
    pub parent_class: GObjectClass,
}
pub type GZlibDecompressorClass = _GZlibDecompressorClass;
pub const PROP_FILE_INFO: C2RustUnnamed_1 = 2;
pub const PROP_FORMAT: C2RustUnnamed_1 = 1;
pub type gz_headerp = *mut gz_header;
pub type z_streamp = *mut z_stream;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_FILE_ATTRIBUTE_STANDARD_NAME: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::name\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"time::modified\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-nsec\0")
};
pub const ZLIB_VERSION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"1.3\0") };
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_NEED_DICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Z_STREAM_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const Z_DATA_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const Z_MEM_ERROR: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const Z_BUF_ERROR: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_set_gzheader(
    mut decompressor: *mut GZlibDecompressor,
) {
    if (*decompressor).format as ::core::ffi::c_uint
        != G_ZLIB_COMPRESSOR_FORMAT_GZIP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if !(*decompressor).header_data.is_null() {
        if !(*(*decompressor).header_data).file_info.is_null() {
            g_object_unref((*(*decompressor).header_data).file_info as gpointer);
        }
        memset(
            (*decompressor).header_data as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<HeaderData>() as size_t,
        );
    } else {
        (*decompressor).header_data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<HeaderData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut HeaderData;
    }
    (*(*decompressor).header_data).gzheader.name =
        &raw mut (*(*decompressor).header_data).filename as *mut Bytef;
    (*(*decompressor).header_data).gzheader.name_max = 256 as uInt;
    if inflateGetHeader(
        &raw mut (*decompressor).zstream,
        &raw mut (*(*decompressor).header_data).gzheader,
    ) != Z_OK
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"unexpected zlib error: %s\0" as *const u8 as *const gchar,
            (*decompressor).zstream.msg,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GZlibDecompressor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GZlibDecompressorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_zlib_decompressor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GZlibDecompressor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GZlibDecompressor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_zlib_decompressor_init
                    as unsafe extern "C" fn(*mut GZlibDecompressor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GConverterIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_zlib_decompressor_iface_init
                as unsafe extern "C" fn(*mut GConverterIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_converter_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GZlibDecompressor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_zlib_decompressor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_zlib_decompressor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GZlibDecompressor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GZlibDecompressor_private_offset,
        );
    }
    safe_c2rust_g_zlib_decompressor_class_init(klass as *mut GZlibDecompressorClass);
}
static mut safe_c2rust_g_zlib_decompressor_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_finalize(mut object: *mut GObject) {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    decompressor = object as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    inflateEnd(&raw mut (*decompressor).zstream);
    if !(*decompressor).header_data.is_null() {
        if !(*(*decompressor).header_data).file_info.is_null() {
            g_object_unref((*(*decompressor).header_data).file_info as gpointer);
        }
        g_free((*decompressor).header_data as gpointer);
    }
    (*(safe_c2rust_g_zlib_decompressor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    decompressor = object as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    match prop_id {
        1 => {
            (*decompressor).format = g_value_get_enum(value) as GZlibCompressorFormat;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gzlibdecompressor.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    decompressor = object as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*decompressor).format as gint);
        }
        2 => {
            if !(*decompressor).header_data.is_null() {
                g_value_set_object(value, (*(*decompressor).header_data).file_info as gpointer);
            } else {
                g_value_set_object(value, NULL);
            }
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gzlibdecompressor.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_init(
    mut decompressor: *mut GZlibDecompressor,
) {
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_constructed(mut object: *mut GObject) {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    let mut res: ::core::ffi::c_int = 0;
    decompressor = object as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    if (*decompressor).format as ::core::ffi::c_uint
        == G_ZLIB_COMPRESSOR_FORMAT_GZIP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        res = inflateInit2_(
            &raw mut (*decompressor).zstream,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        );
    } else if (*decompressor).format as ::core::ffi::c_uint
        == G_ZLIB_COMPRESSOR_FORMAT_RAW as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        res = inflateInit2_(
            &raw mut (*decompressor).zstream,
            -(15 as ::core::ffi::c_int),
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        );
    } else {
        res = inflateInit_(
            &raw mut (*decompressor).zstream,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        );
    }
    if res == Z_MEM_ERROR {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"GZlibDecompressor: Not enough memory for zlib use\0" as *const u8 as *const gchar,
        );
        loop {}
    }
    if res != Z_OK {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"unexpected zlib error: %s\0" as *const u8 as *const gchar,
            (*decompressor).zstream.msg,
        );
    }
    safe_c2rust_g_zlib_decompressor_set_gzheader(decompressor);
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_class_init(
    mut klass: *mut GZlibDecompressorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_zlib_decompressor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).constructed = Some(
        safe_c2rust_g_zlib_decompressor_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_zlib_decompressor_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_zlib_decompressor_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FORMAT as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"format\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_zlib_compressor_format_get_type(),
            G_ZLIB_COMPRESSOR_FORMAT_ZLIB as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FILE_INFO as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"file-info\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_file_info_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_new(
    mut format: GZlibCompressorFormat,
) -> *mut GZlibDecompressor {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    decompressor = g_object_new(
        safe_c2rust_g_zlib_decompressor_get_type(),
        b"format\0" as *const u8 as *const gchar,
        format as ::core::ffi::c_uint,
        NULL,
    ) as *mut GZlibDecompressor;
    return decompressor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_get_file_info(
    mut decompressor: *mut GZlibDecompressor,
) -> *mut GFileInfo {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = decompressor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_zlib_decompressor_get_type();
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
            b"G_IS_ZLIB_DECOMPRESSOR (decompressor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if !(*decompressor).header_data.is_null() {
        return (*(*decompressor).header_data).file_info;
    }
    return ::core::ptr::null_mut::<GFileInfo>();
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_reset(mut converter: *mut GConverter) {
    let mut decompressor: *mut GZlibDecompressor =
        converter as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    let mut res: ::core::ffi::c_int = 0;
    res = inflateReset(&raw mut (*decompressor).zstream);
    if res != Z_OK {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"unexpected zlib error: %s\0" as *const u8 as *const gchar,
            (*decompressor).zstream.msg,
        );
    }
    safe_c2rust_g_zlib_decompressor_set_gzheader(decompressor);
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_convert(
    mut converter: *mut GConverter,
    mut inbuf: *const ::core::ffi::c_void,
    mut inbuf_size: gsize,
    mut outbuf: *mut ::core::ffi::c_void,
    mut outbuf_size: gsize,
    mut flags: GConverterFlags,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> GConverterResult {
    let mut decompressor: *mut GZlibDecompressor = ::core::ptr::null_mut::<GZlibDecompressor>();
    let mut res: ::core::ffi::c_int = 0;
    decompressor = converter as *mut ::core::ffi::c_void as *mut GZlibDecompressor;
    (*decompressor).zstream.next_in = inbuf as *mut ::core::ffi::c_void as *mut Bytef;
    (*decompressor).zstream.avail_in = inbuf_size as uInt;
    (*decompressor).zstream.next_out = outbuf as *mut Bytef;
    (*decompressor).zstream.avail_out = outbuf_size as uInt;
    res = inflate(&raw mut (*decompressor).zstream, Z_NO_FLUSH);
    if res == Z_DATA_ERROR || res == Z_NEED_DICT {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_DATA as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid compressed data\0" as *const u8 as *const gchar),
        );
        return G_CONVERTER_ERROR;
    }
    if res == Z_MEM_ERROR {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Not enough memory\0" as *const u8 as *const gchar),
        );
        return G_CONVERTER_ERROR;
    }
    if res == Z_STREAM_ERROR {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Internal error: %s\0" as *const u8 as *const gchar),
            (*decompressor).zstream.msg,
        );
        return G_CONVERTER_ERROR;
    }
    if res == Z_BUF_ERROR {
        if flags as ::core::ffi::c_uint
            & G_CONVERTER_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            return G_CONVERTER_FLUSHED;
        }
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
            glib_gettext(b"Need more input\0" as *const u8 as *const gchar),
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if res == 0 as ::core::ffi::c_int || res == 1 as ::core::ffi::c_int {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gzlibdecompressor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            370 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res == Z_OK || res == Z_STREAM_END\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *bytes_read = inbuf_size.wrapping_sub((*decompressor).zstream.avail_in as gsize);
    *bytes_written = outbuf_size.wrapping_sub((*decompressor).zstream.avail_out as gsize);
    if !(*decompressor).header_data.is_null()
        && (*(*decompressor).header_data).gzheader.done == 1 as ::core::ffi::c_int
    {
        let mut data: *mut HeaderData = (*decompressor).header_data;
        (*data).gzheader.done = 2 as ::core::ffi::c_int;
        (*data).file_info = g_file_info_new();
        g_file_info_set_attribute_uint64(
            (*data).file_info,
            G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr(),
            (*data).gzheader.time as guint64,
        );
        g_file_info_set_attribute_uint32(
            (*data).file_info,
            G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr(),
            0 as guint32,
        );
        g_file_info_set_attribute_uint32(
            (*data).file_info,
            G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr(),
            0 as guint32,
        );
        if (*data).filename[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '\0' as i32 {
            g_file_info_set_attribute_byte_string(
                (*data).file_info,
                G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr(),
                &raw mut (*data).filename as *mut ::core::ffi::c_char,
            );
        }
        g_object_notify(
            decompressor as *mut ::core::ffi::c_void as *mut GObject,
            b"file-info\0" as *const u8 as *const gchar,
        );
    }
    if res == Z_STREAM_END {
        return G_CONVERTER_FINISHED;
    }
    return G_CONVERTER_CONVERTED;
}
unsafe extern "C" fn safe_c2rust_g_zlib_decompressor_iface_init(mut iface: *mut GConverterIface) {
    (*iface).convert = Some(
        safe_c2rust_g_zlib_decompressor_convert
            as unsafe extern "C" fn(
                *mut GConverter,
                *const ::core::ffi::c_void,
                gsize,
                *mut ::core::ffi::c_void,
                gsize,
                GConverterFlags,
                *mut gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GConverterResult,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GConverter,
                *const ::core::ffi::c_void,
                gsize,
                *mut ::core::ffi::c_void,
                gsize,
                GConverterFlags,
                *mut gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GConverterResult,
        >;
    (*iface).reset =
        Some(safe_c2rust_g_zlib_decompressor_reset as unsafe extern "C" fn(*mut GConverter) -> ())
            as Option<unsafe extern "C" fn(*mut GConverter) -> ()>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
