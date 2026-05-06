use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileInfo;
    pub type _GFileInputStreamPrivate;
    pub type _GFileDescriptorBased;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_file_input_stream_get_type() -> GType;
    fn _g_local_file_info_get_from_fd(
        fd: ::core::ffi::c_int,
        attributes: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_file_descriptor_based_get_type() -> GType;
}
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GFileInputStreamPrivate,
}
pub type GFileInputStreamPrivate = _GFileInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStreamClass {
    pub parent_class: GObjectClass,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub skip: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub skip_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub skip_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GInputStreamClass = _GInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStreamClass {
    pub parent_class: GInputStreamClass,
    pub tell: Option<unsafe extern "C" fn(*mut GFileInputStream) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GFileInputStream) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileInputStreamClass = _GFileInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileInputStream {
    pub parent_instance: GFileInputStream,
    pub priv_0: *mut GLocalFileInputStreamPrivate,
}
pub type GLocalFileInputStreamPrivate = _GLocalFileInputStreamPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GLocalFileInputStreamPrivate {
    pub fd: ::core::ffi::c_int,
    #[bitfield(name = "do_close", ty = "guint", bits = "0..=0")]
    pub do_close: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GLocalFileInputStream = _GLocalFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileInputStreamClass {
    pub parent_class: GFileInputStreamClass,
}
pub type GLocalFileInputStreamClass = _GLocalFileInputStreamClass;
pub type GFileDescriptorBasedIface = _GFileDescriptorBasedIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileDescriptorBasedIface {
    pub g_iface: GTypeInterface,
    pub get_fd: Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>,
}
pub type GFileDescriptorBased = _GFileDescriptorBased;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ESPIPE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_input_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_input_stream_get_type(),
        g_intern_static_string(b"GLocalFileInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFileInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFileInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_input_stream_init
                    as unsafe extern "C" fn(*mut GLocalFileInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GLocalFileInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GLocalFileInputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_file_descriptor_based_iface_init
                as unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_descriptor_based_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFileInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GLocalFileInputStream_private_offset,
        );
    }
    safe_c2rust_g_local_file_input_stream_class_init(klass as *mut GLocalFileInputStreamClass);
}
static mut safe_c2rust_GLocalFileInputStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_get_instance_private(
    mut self_0: *mut GLocalFileInputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GLocalFileInputStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_local_file_input_stream_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_input_stream_set_do_close(
    mut in_0: *mut GLocalFileInputStream,
    mut do_close: gboolean,
) {
    (*(*in_0).priv_0).set_do_close(do_close as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_class_init(
    mut klass: *mut GLocalFileInputStreamClass,
) {
    let mut stream_class: *mut GInputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GInputStreamClass;
    let mut file_stream_class: *mut GFileInputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GFileInputStreamClass;
    (*stream_class).read_fn = Some(
        safe_c2rust_g_local_file_input_stream_read
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*stream_class).close_fn = Some(
        safe_c2rust_g_local_file_input_stream_close
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).tell = Some(
        safe_c2rust_g_local_file_input_stream_tell
            as unsafe extern "C" fn(*mut GFileInputStream) -> goffset,
    )
        as Option<unsafe extern "C" fn(*mut GFileInputStream) -> goffset>;
    (*file_stream_class).can_seek = Some(
        safe_c2rust_g_local_file_input_stream_can_seek
            as unsafe extern "C" fn(*mut GFileInputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileInputStream) -> gboolean>;
    (*file_stream_class).seek = Some(
        safe_c2rust_g_local_file_input_stream_seek
            as unsafe extern "C" fn(
                *mut GFileInputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileInputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).query_info = Some(
        safe_c2rust_g_local_file_input_stream_query_info
            as unsafe extern "C" fn(
                *mut GFileInputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileInputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
}
unsafe extern "C" fn safe_c2rust_g_file_descriptor_based_iface_init(
    mut iface: *mut GFileDescriptorBasedIface,
) {
    (*iface).get_fd = Some(
        safe_c2rust_g_local_file_input_stream_get_fd
            as unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_init(
    mut info: *mut GLocalFileInputStream,
) {
    (*info).priv_0 = safe_c2rust_g_local_file_input_stream_get_instance_private(info)
        as *mut GLocalFileInputStreamPrivate;
    (*(*info).priv_0).set_do_close(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_input_stream_new(
    mut fd: ::core::ffi::c_int,
) -> *mut GFileInputStream {
    let mut stream: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    stream = g_object_new(
        safe_c2rust__g_local_file_input_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFileInputStream;
    (*(*stream).priv_0).fd = fd;
    return stream as *mut ::core::ffi::c_void as *mut GFileInputStream;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    let mut res: gssize = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    res = -(1 as ::core::ffi::c_int) as gssize;
    while !(g_cancellable_set_error_if_cancelled(cancellable, error) != 0) {
        res = read((*(*file).priv_0).fd, buffer, count as size_t) as gssize;
        if !(res == -(1 as ::core::ffi::c_int) as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EINTR {
            continue;
        }
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error reading from file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        break;
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_close(
    mut stream: *mut GInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    if (*(*file).priv_0).do_close() == 0 {
        return TRUE;
    }
    if (*(*file).priv_0).fd == -(1 as ::core::ffi::c_int) {
        return TRUE;
    }
    if g_close(
        (*(*file).priv_0).fd as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error closing file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_tell(
    mut stream: *mut GFileInputStream,
) -> goffset {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    pos = lseek((*(*file).priv_0).fd, 0 as __off64_t, SEEK_CUR) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t {
        return 0 as goffset;
    }
    return pos as goffset;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_can_seek(
    mut stream: *mut GFileInputStream,
) -> gboolean {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    pos = lseek((*(*file).priv_0).fd, 0 as __off64_t, SEEK_CUR) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t && *__errno_location() == ESPIPE {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_seek_type_to_lseek(mut type_0: GSeekType) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_uint {
        1 => return SEEK_SET,
        2 => return SEEK_END,
        0 | _ => return SEEK_CUR,
    };
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_seek(
    mut stream: *mut GFileInputStream,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    pos = lseek(
        (*(*file).priv_0).fd,
        offset as __off64_t,
        safe_c2rust_seek_type_to_lseek(type_0),
    ) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error seeking in file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_query_info(
    mut stream: *mut GFileInputStream,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut file: *mut GLocalFileInputStream = ::core::ptr::null_mut::<GLocalFileInputStream>();
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return _g_local_file_info_get_from_fd((*(*file).priv_0).fd, attributes, error);
}
unsafe extern "C" fn safe_c2rust_g_local_file_input_stream_get_fd(
    mut fd_based: *mut GFileDescriptorBased,
) -> ::core::ffi::c_int {
    let mut stream: *mut GLocalFileInputStream =
        fd_based as *mut ::core::ffi::c_void as *mut GLocalFileInputStream;
    return (*(*stream).priv_0).fd;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
