use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFileEnumeratorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_filename_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_uri_escape_string(
        unescaped: *const ::core::ffi::c_char,
        reserved_chars_allowed: *const ::core::ffi::c_char,
        allow_utf8: gboolean,
    ) -> *mut ::core::ffi::c_char;
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_file_enumerator_next_files_async(
        enumerator: *mut GFileEnumerator,
        num_files: ::core::ffi::c_int,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_enumerator_next_files_finish(
        enumerator: *mut GFileEnumerator,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_file_enumerator_close_async(
        enumerator: *mut GFileEnumerator,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_parse_name(parse_name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_equal(file1: *mut GFile, file2: *mut GFile) -> gboolean;
    fn g_file_get_basename(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_get_parent(file: *mut GFile) -> *mut GFile;
    fn g_file_enumerate_children_async(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_enumerate_children_finish(
        file: *mut GFile,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GFileEnumerator;
    fn g_file_info_get_file_type(info: *mut GFileInfo) -> GFileType;
    fn g_file_info_get_name(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
}
pub type size_t = usize;
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub type gunichar = guint32;
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
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GFileType = ::core::ffi::c_uint;
pub const G_FILE_TYPE_MOUNTABLE: GFileType = 6;
pub const G_FILE_TYPE_SHORTCUT: GFileType = 5;
pub const G_FILE_TYPE_SPECIAL: GFileType = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: GFileType = 3;
pub const G_FILE_TYPE_DIRECTORY: GFileType = 2;
pub const G_FILE_TYPE_REGULAR: GFileType = 1;
pub const G_FILE_TYPE_UNKNOWN: GFileType = 0;
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
pub struct _GFileEnumerator {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileEnumeratorPrivate,
}
pub type GFileEnumeratorPrivate = _GFileEnumeratorPrivate;
pub type GFileEnumerator = _GFileEnumerator;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilenameCompleter {
    pub parent: GObject,
    pub basenames_dir: *mut GFile,
    pub basenames_are_escaped: gboolean,
    pub dirs_only: gboolean,
    pub basenames: *mut GList,
    pub basename_loader: *mut LoadBasenamesData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadBasenamesData {
    pub completer: *mut GFilenameCompleter,
    pub enumerator: *mut GFileEnumerator,
    pub cancellable: *mut GCancellable,
    pub should_escape: gboolean,
    pub dir: *mut GFile,
    pub basenames: *mut GList,
    pub dirs_only: gboolean,
}
pub type GFilenameCompleter = _GFilenameCompleter;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilenameCompleterClass {
    pub parent_class: GObjectClass,
    pub got_completion_data: Option<unsafe extern "C" fn(*mut GFilenameCompleter) -> ()>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFilenameCompleterClass = _GFilenameCompleterClass;
pub const GOT_COMPLETION_DATA: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_0 = 1;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_g_filename_completer_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GFilenameCompleter_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_filename_completer_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_filename_completer_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GFilenameCompleter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GFilenameCompleter_private_offset,
        );
    }
    safe_c2rust_g_filename_completer_class_init(klass as *mut GFilenameCompleterClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_filename_completer_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GFilenameCompleter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GFilenameCompleterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_filename_completer_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GFilenameCompleter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GFilenameCompleter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_filename_completer_init
                    as unsafe extern "C" fn(*mut GFilenameCompleter) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_completer_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_filename_completer_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_filename_completer_finalize(mut object: *mut GObject) {
    let mut completer: *mut GFilenameCompleter = ::core::ptr::null_mut::<GFilenameCompleter>();
    completer = object as *mut ::core::ffi::c_void as *mut GFilenameCompleter;
    safe_c2rust_cancel_load_basenames(completer);
    if !(*completer).basenames_dir.is_null() {
        g_object_unref((*completer).basenames_dir as gpointer);
    }
    g_list_free_full(
        (*completer).basenames,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(safe_c2rust_g_filename_completer_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_filename_completer_class_init(
    mut klass: *mut GFilenameCompleterClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_filename_completer_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_signals[GOT_COMPLETION_DATA as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"got-completion-data\0" as *const u8 as *const gchar),
        safe_c2rust_g_filename_completer_get_type(),
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
}
unsafe extern "C" fn safe_c2rust_g_filename_completer_init(mut completer: *mut GFilenameCompleter) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_completer_new() -> *mut GFilenameCompleter {
    return g_object_new(
        safe_c2rust_g_filename_completer_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GFilenameCompleter;
}
unsafe extern "C" fn safe_c2rust_longest_common_prefix(
    mut a: *mut ::core::ffi::c_char,
    mut b: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    start = a;
    while g_utf8_get_char(a) == g_utf8_get_char(b) {
        a = a.offset(
            *safe_c2rust_g_utf8_skip.offset(*(a as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        );
        b = b.offset(
            *safe_c2rust_g_utf8_skip.offset(*(b as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        );
    }
    return g_strndup(start, a.offset_from(start) as ::core::ffi::c_long as gsize)
        as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_load_basenames_data_free(mut data: *mut LoadBasenamesData) {
    if !(*data).enumerator.is_null() {
        g_object_unref((*data).enumerator as gpointer);
    }
    g_object_unref((*data).cancellable as gpointer);
    g_object_unref((*data).dir as gpointer);
    g_list_free_full(
        (*data).basenames,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_got_more_files(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut LoadBasenamesData = user_data as *mut LoadBasenamesData;
    let mut infos: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut append_slash: gboolean = 0;
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*data).completer.is_null() {
        safe_c2rust_load_basenames_data_free(data);
        return;
    }
    infos = g_file_enumerator_next_files_finish(
        (*data).enumerator,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    l = infos;
    while !l.is_null() {
        info = (*l).data as *mut GFileInfo;
        if (*data).dirs_only != 0
            && g_file_info_get_file_type(info) as ::core::ffi::c_uint
                != G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_object_unref(info as gpointer);
        } else {
            append_slash = (g_file_info_get_file_type(info) as ::core::ffi::c_uint
                == G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int as gboolean;
            name = g_file_info_get_name(info);
            if name.is_null() {
                g_object_unref(info as gpointer);
            } else {
                if (*data).should_escape != 0 {
                    basename = g_uri_escape_string(
                        name,
                        b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
                        TRUE,
                    );
                } else {
                    basename = g_filename_to_utf8(
                        name as *const gchar,
                        -(1 as ::core::ffi::c_int) as gssize,
                        ::core::ptr::null_mut::<gsize>(),
                        ::core::ptr::null_mut::<gsize>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    ) as *mut ::core::ffi::c_char;
                }
                if !basename.is_null() {
                    if append_slash != 0 {
                        t = basename;
                        basename = g_strconcat(
                            basename,
                            b"/\0" as *const u8 as *const ::core::ffi::c_char,
                            NULL_0,
                        ) as *mut ::core::ffi::c_char;
                        g_free(t as gpointer);
                    }
                    (*data).basenames = g_list_prepend((*data).basenames, basename as gpointer);
                }
                g_object_unref(info as gpointer);
            }
        }
        l = (*l).next;
    }
    g_list_free(infos);
    if !infos.is_null() {
        g_file_enumerator_next_files_async(
            (*data).enumerator,
            100 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*data).cancellable,
            Some(
                safe_c2rust_got_more_files
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
    } else {
        (*(*data).completer).basename_loader = ::core::ptr::null_mut::<LoadBasenamesData>();
        if !(*(*data).completer).basenames_dir.is_null() {
            g_object_unref((*(*data).completer).basenames_dir as gpointer);
        }
        g_list_free_full(
            (*(*data).completer).basenames,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*(*data).completer).basenames_dir =
            g_object_ref((*data).dir as gpointer) as *mut GFile as *mut GFile;
        (*(*data).completer).basenames = (*data).basenames;
        (*(*data).completer).basenames_are_escaped = (*data).should_escape;
        (*data).basenames = ::core::ptr::null_mut::<GList>();
        g_file_enumerator_close_async(
            (*data).enumerator,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_0,
        );
        g_signal_emit(
            (*data).completer as gpointer,
            safe_c2rust_signals[GOT_COMPLETION_DATA as ::core::ffi::c_int as usize],
            0 as GQuark,
        );
        safe_c2rust_load_basenames_data_free(data);
    };
}
unsafe extern "C" fn safe_c2rust_got_enum(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut LoadBasenamesData = user_data as *mut LoadBasenamesData;
    if (*data).completer.is_null() {
        safe_c2rust_load_basenames_data_free(data);
        return;
    }
    (*data).enumerator = g_file_enumerate_children_finish(
        source_object as *mut ::core::ffi::c_void as *mut GFile,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if (*data).enumerator.is_null() {
        (*(*data).completer).basename_loader = ::core::ptr::null_mut::<LoadBasenamesData>();
        if !(*(*data).completer).basenames_dir.is_null() {
            g_object_unref((*(*data).completer).basenames_dir as gpointer);
        }
        g_list_free_full(
            (*(*data).completer).basenames,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*(*data).completer).basenames_dir =
            g_object_ref((*data).dir as gpointer) as *mut GFile as *mut GFile;
        (*(*data).completer).basenames = ::core::ptr::null_mut::<GList>();
        (*(*data).completer).basenames_are_escaped = (*data).should_escape;
        safe_c2rust_load_basenames_data_free(data);
        return;
    }
    g_file_enumerator_next_files_async(
        (*data).enumerator,
        100 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*data).cancellable,
        Some(
            safe_c2rust_got_more_files
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_schedule_load_basenames(
    mut completer: *mut GFilenameCompleter,
    mut dir: *mut GFile,
    mut should_escape: gboolean,
) {
    let mut data: *mut LoadBasenamesData = ::core::ptr::null_mut::<LoadBasenamesData>();
    safe_c2rust_cancel_load_basenames(completer);
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LoadBasenamesData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LoadBasenamesData;
    (*data).completer = completer;
    (*data).cancellable = g_cancellable_new();
    (*data).dir = g_object_ref(dir as gpointer) as *mut GFile as *mut GFile;
    (*data).should_escape = should_escape;
    (*data).dirs_only = (*completer).dirs_only;
    (*completer).basename_loader = data;
    g_file_enumerate_children_async(
        dir,
        b"standard::name,standard::type\0" as *const u8 as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        0 as ::core::ffi::c_int,
        (*data).cancellable,
        Some(
            safe_c2rust_got_enum
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_cancel_load_basenames(mut completer: *mut GFilenameCompleter) {
    let mut loader: *mut LoadBasenamesData = ::core::ptr::null_mut::<LoadBasenamesData>();
    if !(*completer).basename_loader.is_null() {
        loader = (*completer).basename_loader;
        (*loader).completer = ::core::ptr::null_mut::<GFilenameCompleter>();
        g_cancellable_cancel((*loader).cancellable);
        (*completer).basename_loader = ::core::ptr::null_mut::<LoadBasenamesData>();
    }
}
unsafe extern "C" fn safe_c2rust_init_completion(
    mut completer: *mut GFilenameCompleter,
    mut initial_text: *const ::core::ffi::c_char,
    mut basename_out: *mut *mut ::core::ffi::c_char,
) -> *mut GList {
    let mut should_escape: gboolean = 0;
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut parent: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    *basename_out = ::core::ptr::null_mut::<::core::ffi::c_char>();
    should_escape = !(g_path_is_absolute(initial_text as *const gchar) != 0
        || *initial_text as ::core::ffi::c_int == '~' as i32)
        as ::core::ffi::c_int as gboolean;
    len = strlen(initial_text) as ::core::ffi::c_int;
    if len > 0 as ::core::ffi::c_int
        && *initial_text.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        return ::core::ptr::null_mut::<GList>();
    }
    file = g_file_parse_name(initial_text);
    parent = g_file_get_parent(file);
    if parent.is_null() {
        g_object_unref(file as gpointer);
        return ::core::ptr::null_mut::<GList>();
    }
    if (*completer).basenames_dir.is_null()
        || (*completer).basenames_are_escaped != should_escape
        || g_file_equal(parent, (*completer).basenames_dir) == 0
    {
        safe_c2rust_schedule_load_basenames(completer, parent, should_escape);
        g_object_unref(file as gpointer);
        return ::core::ptr::null_mut::<GList>();
    }
    basename = g_file_get_basename(file);
    if should_escape != 0 {
        t = basename;
        basename = g_uri_escape_string(
            basename,
            b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
            TRUE,
        );
        g_free(t as gpointer);
    } else {
        t = basename;
        basename = g_filename_to_utf8(
            basename,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        g_free(t as gpointer);
        if basename.is_null() {
            return ::core::ptr::null_mut::<GList>();
        }
    }
    *basename_out = basename;
    return (*completer).basenames;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_completer_get_completion_suffix(
    mut completer: *mut GFilenameCompleter,
    mut initial_text: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut possible_matches: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut prefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut possible_match: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lcp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = completer as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_filename_completer_get_type();
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
            b"G_IS_FILENAME_COMPLETER (completer)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !initial_text.is_null() {
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
            b"initial_text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    possible_matches = safe_c2rust_init_completion(completer, initial_text, &raw mut prefix);
    suffix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    l = possible_matches;
    while !l.is_null() {
        possible_match = (*l).data as *mut ::core::ffi::c_char;
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = possible_match;
                let __prefix: *const ::core::ffi::c_char = prefix;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(possible_match, prefix)
        } != 0
        {
            if suffix.is_null() {
                suffix =
                    safe_c2rust_g_strdup_inline(possible_match.offset(strlen(prefix) as isize));
            } else {
                lcp = safe_c2rust_longest_common_prefix(
                    suffix,
                    possible_match.offset(strlen(prefix) as isize),
                );
                g_free(suffix as gpointer);
                suffix = lcp;
                if *suffix as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        l = (*l).next;
    }
    g_free(prefix as gpointer);
    return suffix;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_completer_get_completions(
    mut completer: *mut GFilenameCompleter,
    mut initial_text: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut possible_matches: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut prefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut possible_match: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = completer as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_filename_completer_get_type();
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
            b"G_IS_FILENAME_COMPLETER (completer)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !initial_text.is_null() {
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
            b"initial_text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    possible_matches = safe_c2rust_init_completion(completer, initial_text, &raw mut prefix);
    res = g_ptr_array_new();
    l = possible_matches;
    while !l.is_null() {
        possible_match = (*l).data as *mut ::core::ffi::c_char;
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = possible_match;
                let __prefix: *const ::core::ffi::c_char = prefix;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_15
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(possible_match, prefix)
        } != 0
        {
            g_ptr_array_add(
                res,
                g_strconcat(
                    initial_text as *const gchar,
                    possible_match.offset(strlen(prefix) as isize),
                    NULL_0,
                ) as gpointer,
            );
        }
        l = (*l).next;
    }
    g_free(prefix as gpointer);
    g_ptr_array_add(res, NULL_0);
    return g_ptr_array_free(res, FALSE) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filename_completer_set_dirs_only(
    mut completer: *mut GFilenameCompleter,
    mut dirs_only: gboolean,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = completer as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_filename_completer_get_type();
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
            b"G_IS_FILENAME_COMPLETER (completer)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*completer).dirs_only = dirs_only;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
