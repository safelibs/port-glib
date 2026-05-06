extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileInfo;
    pub type _GFileInputStreamPrivate;
    pub type _GFileOutputStreamPrivate;
    pub type _GFileIOStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GLocalFileOutputStreamPrivate;
    pub type _GLocalFileInputStreamPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_input_stream_close(
        stream: *mut GInputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_close(
        stream: *mut GOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_io_stream_get_type() -> GType;
    fn _g_local_file_output_stream_set_do_close(
        out: *mut GLocalFileOutputStream,
        do_close: gboolean,
    );
    fn _g_local_file_output_stream_really_close(
        out: *mut GLocalFileOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_local_file_output_stream_get_fd(output_stream: *mut GLocalFileOutputStream) -> gint;
    fn _g_local_file_input_stream_new(fd: ::core::ffi::c_int) -> *mut GFileInputStream;
    fn _g_local_file_input_stream_set_do_close(
        in_0: *mut GLocalFileInputStream,
        do_close: gboolean,
    );
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GFileInputStreamPrivate,
}
pub type GFileInputStreamPrivate = _GFileInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStream {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GFileIOStreamPrivate,
}
pub type GFileIOStreamPrivate = _GFileIOStreamPrivate;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GFileIOStream = _GFileIOStream;
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
pub struct _GFileIOStreamClass {
    pub parent_class: GIOStreamClass,
    pub tell: Option<unsafe extern "C" fn(*mut GFileIOStream) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_truncate: Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>,
    pub truncate_fn: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            goffset,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub get_etag: Option<unsafe extern "C" fn(*mut GFileIOStream) -> *mut ::core::ffi::c_char>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileIOStreamClass = _GFileIOStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileOutputStream {
    pub parent_instance: GFileOutputStream,
    pub priv_0: *mut GLocalFileOutputStreamPrivate,
}
pub type GLocalFileOutputStreamPrivate = _GLocalFileOutputStreamPrivate;
pub type GLocalFileOutputStream = _GLocalFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileIOStream {
    pub parent_instance: GFileIOStream,
    pub input_stream: *mut GInputStream,
    pub output_stream: *mut GOutputStream,
}
pub type GLocalFileIOStream = _GLocalFileIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileIOStreamClass {
    pub parent_class: GFileIOStreamClass,
}
pub type GLocalFileIOStreamClass = _GLocalFileIOStreamClass;
pub type GLocalFileInputStream = _GLocalFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileInputStream {
    pub parent_instance: GFileInputStream,
    pub priv_0: *mut GLocalFileInputStreamPrivate,
}
pub type GLocalFileInputStreamPrivate = _GLocalFileInputStreamPrivate;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_io_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_io_stream_get_type_once();
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
static mut safe_c2rust_g_local_file_io_stream_parent_class: gpointer = NULL;
static mut safe_c2rust_GLocalFileIOStream_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_io_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFileIOStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GLocalFileIOStream_private_offset,
        );
    }
    safe_c2rust_g_local_file_io_stream_class_init(klass as *mut GLocalFileIOStreamClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_io_stream_get_type(),
        g_intern_static_string(b"GLocalFileIOStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileIOStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_io_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFileIOStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFileIOStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_io_stream_init
                    as unsafe extern "C" fn(*mut GLocalFileIOStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_finalize(mut object: *mut GObject) {
    let mut file: *mut GLocalFileIOStream = ::core::ptr::null_mut::<GLocalFileIOStream>();
    file = object as *mut ::core::ffi::c_void as *mut GLocalFileIOStream;
    g_object_unref((*file).input_stream as gpointer);
    g_object_unref((*file).output_stream as gpointer);
    (*(safe_c2rust_g_local_file_io_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_io_stream_new(
    mut output_stream: *mut GLocalFileOutputStream,
) -> *mut GFileIOStream {
    let mut stream: *mut GLocalFileIOStream = ::core::ptr::null_mut::<GLocalFileIOStream>();
    let mut fd: ::core::ffi::c_int = 0;
    stream = g_object_new(
        safe_c2rust__g_local_file_io_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFileIOStream;
    (*stream).output_stream =
        g_object_ref(output_stream as *mut ::core::ffi::c_void as *mut GOutputStream as gpointer)
            as *mut GOutputStream as *mut GOutputStream;
    _g_local_file_output_stream_set_do_close(output_stream, FALSE);
    fd = _g_local_file_output_stream_get_fd(output_stream) as ::core::ffi::c_int;
    (*stream).input_stream = _g_local_file_input_stream_new(fd) as *mut GInputStream;
    _g_local_file_input_stream_set_do_close(
        (*stream).input_stream as *mut ::core::ffi::c_void as *mut GLocalFileInputStream,
        FALSE,
    );
    return stream as *mut ::core::ffi::c_void as *mut GFileIOStream;
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_get_input_stream(
    mut stream: *mut GIOStream,
) -> *mut GInputStream {
    return (*(stream as *mut ::core::ffi::c_void as *mut GLocalFileIOStream)).input_stream;
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_get_output_stream(
    mut stream: *mut GIOStream,
) -> *mut GOutputStream {
    return (*(stream as *mut ::core::ffi::c_void as *mut GLocalFileIOStream)).output_stream;
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_close(
    mut stream: *mut GIOStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileIOStream =
        stream as *mut ::core::ffi::c_void as *mut GLocalFileIOStream;
    g_output_stream_close(
        (*file).output_stream,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_input_stream_close(
        (*file).input_stream,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    return _g_local_file_output_stream_really_close(
        (*file).output_stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_class_init(
    mut klass: *mut GLocalFileIOStreamClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut stream_class: *mut GIOStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GIOStreamClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_local_file_io_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*stream_class).get_input_stream = Some(
        safe_c2rust_g_local_file_io_stream_get_input_stream
            as unsafe extern "C" fn(*mut GIOStream) -> *mut GInputStream,
    )
        as Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GInputStream>;
    (*stream_class).get_output_stream = Some(
        safe_c2rust_g_local_file_io_stream_get_output_stream
            as unsafe extern "C" fn(*mut GIOStream) -> *mut GOutputStream,
    )
        as Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GOutputStream>;
    (*stream_class).close_fn = Some(
        safe_c2rust_g_local_file_io_stream_close
            as unsafe extern "C" fn(
                *mut GIOStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GIOStream, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_local_file_io_stream_init(mut stream: *mut GLocalFileIOStream) {}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
