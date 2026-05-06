extern "C" {
    pub type _GIConv;
    pub type _GData;
    pub type _GCancellable;
    pub type _GConverter;
    pub type _GInitable;
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
    fn g_iconv_open(to_codeset: *const gchar, from_codeset: *const gchar) -> GIConv;
    fn g_iconv(
        converter: GIConv,
        inbuf: *mut *mut gchar,
        inbytes_left: *mut gsize,
        outbuf: *mut *mut gchar,
        outbytes_left: *mut gsize,
    ) -> gsize;
    fn g_iconv_close(converter: GIConv) -> gint;
    fn g_free(mem: gpointer);
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_converter_get_type() -> GType;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_io_error_quark() -> GQuark;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GIConv = *mut _GIConv;
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
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCharsetConverter {
    pub parent_instance: GObject,
    pub from: *mut ::core::ffi::c_char,
    pub to: *mut ::core::ffi::c_char,
    pub iconv: GIConv,
    pub use_fallback: gboolean,
    pub n_fallback_errors: guint,
}
pub type GCharsetConverter = _GCharsetConverter;
pub type GConverter = _GConverter;
pub type GInitable = _GInitable;
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
pub struct _GCharsetConverterClass {
    pub parent_class: GObjectClass,
}
pub type GCharsetConverterClass = _GCharsetConverterClass;
pub const PROP_USE_FALLBACK: C2RustUnnamed_1 = 3;
pub const PROP_FROM_CHARSET: C2RustUnnamed_1 = 1;
pub const PROP_TO_CHARSET: C2RustUnnamed_1 = 2;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EILSEQ: ::core::ffi::c_int = 84;
pub const E2BIG: ::core::ffi::c_int = 7;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
static mut safe_c2rust_g_charset_converter_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_charset_converter_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_charset_converter_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_charset_converter_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_charset_converter_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GCharsetConverter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GCharsetConverter_private_offset,
        );
    }
    safe_c2rust_g_charset_converter_class_init(klass as *mut GCharsetConverterClass);
}
static mut safe_c2rust_GCharsetConverter_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_charset_converter_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GCharsetConverter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GCharsetConverterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_charset_converter_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GCharsetConverter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCharsetConverter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_charset_converter_init
                    as unsafe extern "C" fn(*mut GCharsetConverter) -> (),
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
            safe_c2rust_g_charset_converter_iface_init
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
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_charset_converter_initable_iface_init
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
unsafe extern "C" fn safe_c2rust_g_charset_converter_finalize(mut object: *mut GObject) {
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    conv = object as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    g_free((*conv).from as gpointer);
    g_free((*conv).to as gpointer);
    if !(*conv).iconv.is_null() {
        g_iconv_close((*conv).iconv);
    }
    (*(safe_c2rust_g_charset_converter_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    conv = object as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    match prop_id {
        2 => {
            g_free((*conv).to as gpointer);
            (*conv).to = g_value_dup_string(value) as *mut ::core::ffi::c_char;
        }
        1 => {
            g_free((*conv).from as gpointer);
            (*conv).from = g_value_dup_string(value) as *mut ::core::ffi::c_char;
        }
        3 => {
            (*conv).use_fallback = g_value_get_boolean(value);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcharsetconverter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    conv = object as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    match prop_id {
        2 => {
            g_value_set_string(value, (*conv).to);
        }
        1 => {
            g_value_set_string(value, (*conv).from);
        }
        3 => {
            g_value_set_boolean(value, (*conv).use_fallback);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcharsetconverter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                141 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_class_init(
    mut klass: *mut GCharsetConverterClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_charset_converter_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_charset_converter_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_charset_converter_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_TO_CHARSET as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"to-charset\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FROM_CHARSET as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"from-charset\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_USE_FALLBACK as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"use-fallback\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_init(mut local: *mut GCharsetConverter) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_charset_converter_new(
    mut to_charset: *const gchar,
    mut from_charset: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GCharsetConverter {
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    conv = g_initable_new(
        safe_c2rust_g_charset_converter_get_type(),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"to-charset\0" as *const u8 as *const gchar,
        to_charset,
        b"from-charset\0" as *const u8 as *const ::core::ffi::c_char,
        from_charset,
        NULL,
    ) as *mut GCharsetConverter;
    return conv;
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_reset(mut converter: *mut GConverter) {
    let mut conv: *mut GCharsetConverter =
        converter as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    if (*conv).iconv.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Invalid object, not initialized\0" as *const u8 as *const gchar,
        );
        return;
    }
    g_iconv(
        (*conv).iconv,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<gsize>(),
    );
    (*conv).n_fallback_errors = 0 as guint;
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_convert(
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
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    let mut res: gsize = 0;
    let mut ret: GConverterResult = G_CONVERTER_ERROR;
    let mut inbufp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut outbufp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut in_left: gsize = 0;
    let mut out_left: gsize = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    let mut reset: gboolean = 0;
    conv = converter as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    if (*conv).iconv.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid object, not initialized\0" as *const u8 as *const gchar),
        );
        return G_CONVERTER_ERROR;
    }
    inbufp = inbuf as *mut ::core::ffi::c_char as *mut gchar;
    outbufp = outbuf as *mut ::core::ffi::c_char as *mut gchar;
    in_left = inbuf_size;
    out_left = outbuf_size;
    reset = FALSE as gboolean;
    if inbuf_size == 0 as gsize {
        if flags as ::core::ffi::c_uint
            & G_CONVERTER_INPUT_AT_END as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || flags as ::core::ffi::c_uint
                & G_CONVERTER_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            reset = TRUE as gboolean;
        } else {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Incomplete multibyte sequence in input\0" as *const u8 as *const gchar,
                ),
            );
            return G_CONVERTER_ERROR;
        }
    }
    if reset != 0 {
        res = g_iconv(
            (*conv).iconv,
            ::core::ptr::null_mut::<*mut gchar>(),
            &raw mut in_left,
            &raw mut outbufp,
            &raw mut out_left,
        );
    } else {
        res = g_iconv(
            (*conv).iconv,
            &raw mut inbufp,
            &raw mut in_left,
            &raw mut outbufp,
            &raw mut out_left,
        );
    }
    *bytes_read =
        inbufp.offset_from(inbuf as *mut ::core::ffi::c_char) as ::core::ffi::c_long as gsize;
    *bytes_written =
        outbufp.offset_from(outbuf as *mut ::core::ffi::c_char) as ::core::ffi::c_long as gsize;
    let mut current_block_41: u64;
    if res == -(1 as ::core::ffi::c_int) as gsize && *bytes_read == 0 as gsize {
        errsv = *__errno_location();
        match errsv {
            EINVAL => {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Incomplete multibyte sequence in input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block_41 = 7828949454673616476;
            }
            E2BIG => {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
                    glib_gettext(b"Not enough space in destination\0" as *const u8 as *const gchar),
                );
                current_block_41 = 7828949454673616476;
            }
            EILSEQ => {
                if (*conv).use_fallback != 0 {
                    if outbuf_size < 3 as gsize {
                        g_set_error_literal(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Not enough space in destination\0" as *const u8 as *const gchar,
                            ),
                        );
                        current_block_41 = 7828949454673616476;
                    } else {
                        let hex: [::core::ffi::c_char; 17] =
                            ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                *b"0123456789ABCDEF\0",
                            );
                        let mut v: guint8 = *(inbuf as *mut guint8);
                        let mut out: *mut guint8 = outbuf as *mut guint8;
                        *out.offset(0 as ::core::ffi::c_int as isize) = '\\' as i32 as guint8;
                        *out.offset(1 as ::core::ffi::c_int as isize) =
                            hex[((v as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                                >> 4 as ::core::ffi::c_int)
                                as usize] as guint8;
                        *out.offset(2 as ::core::ffi::c_int as isize) =
                            hex[((v as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                >> 0 as ::core::ffi::c_int)
                                as usize] as guint8;
                        *bytes_read = 1 as gsize;
                        *bytes_written = 3 as gsize;
                        in_left = in_left.wrapping_sub(1);
                        (*conv).n_fallback_errors = (*conv).n_fallback_errors.wrapping_add(1);
                        current_block_41 = 14421636780160048408;
                    }
                } else {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_DATA as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Invalid byte sequence in conversion input\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    current_block_41 = 7828949454673616476;
                }
            }
            _ => {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(b"Error during conversion: %s\0" as *const u8 as *const gchar),
                    g_strerror(errsv as gint),
                );
                current_block_41 = 7828949454673616476;
            }
        }
        match current_block_41 {
            14421636780160048408 => {}
            _ => {
                ret = G_CONVERTER_ERROR;
                current_block_41 = 12199444798915819164;
            }
        }
    } else {
        current_block_41 = 14421636780160048408;
    }
    match current_block_41 {
        14421636780160048408 => {
            ret = G_CONVERTER_CONVERTED;
            if reset != 0
                && flags as ::core::ffi::c_uint
                    & G_CONVERTER_INPUT_AT_END as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
            {
                ret = G_CONVERTER_FINISHED;
            } else if reset != 0
                && flags as ::core::ffi::c_uint
                    & G_CONVERTER_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
            {
                ret = G_CONVERTER_FLUSHED;
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_charset_converter_set_use_fallback(
    mut converter: *mut GCharsetConverter,
    mut use_fallback: gboolean,
) {
    use_fallback = (use_fallback != 0) as ::core::ffi::c_int as gboolean;
    if (*converter).use_fallback != use_fallback {
        (*converter).use_fallback = use_fallback;
        g_object_notify(
            converter as *mut ::core::ffi::c_void as *mut GObject,
            b"use-fallback\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_charset_converter_get_use_fallback(
    mut converter: *mut GCharsetConverter,
) -> gboolean {
    return (*converter).use_fallback;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_charset_converter_get_num_fallbacks(
    mut converter: *mut GCharsetConverter,
) -> guint {
    return (*converter).n_fallback_errors;
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_iface_init(mut iface: *mut GConverterIface) {
    (*iface).convert = Some(
        safe_c2rust_g_charset_converter_convert
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
        Some(safe_c2rust_g_charset_converter_reset as unsafe extern "C" fn(*mut GConverter) -> ())
            as Option<unsafe extern "C" fn(*mut GConverter) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut conv: *mut GCharsetConverter = ::core::ptr::null_mut::<GCharsetConverter>();
    let mut errsv: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = initable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_charset_converter_get_type();
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
            b"G_IS_CHARSET_CONVERTER (initable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    conv = initable as *mut ::core::ffi::c_void as *mut GCharsetConverter;
    if !cancellable.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cancellable initialization not supported\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    (*conv).iconv = g_iconv_open((*conv).to, (*conv).from);
    errsv = *__errno_location();
    if (*conv).iconv == -(1 as ::core::ffi::c_int) as GIConv {
        if errsv == EINVAL {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Conversion from character set \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D is not supported\0"
                        as *const u8 as *const gchar,
                ),
                (*conv).from,
                (*conv).to,
            );
        } else {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Could not open converter from \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                (*conv).from,
                (*conv).to,
            );
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_charset_converter_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_charset_converter_initable_init
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
