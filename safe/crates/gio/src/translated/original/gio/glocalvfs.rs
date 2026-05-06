extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GCancellablePrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GIcon;
    pub type _GIOExtension;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_get_home_dir() -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_filename_from_utf8(
        utf8string: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_filename_from_uri(
        uri: *const gchar,
        hostname: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_ascii_strncasecmp(s1: *const gchar, s2: *const gchar, n: gsize) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn _g_local_file_new(filename: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_vfs_get_type() -> GType;
    fn _g_dummy_file_new(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_unix_get_passwd_entry(user_name: *const gchar, error: *mut *mut GError) -> *mut passwd;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GVariant = _GVariant;
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
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: GFileAttributeInfoFlags = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: GFileAttributeInfoFlags = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: GFileAttributeInfoFlags = 0;
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfo {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GFileAttributeType,
    pub flags: GFileAttributeInfoFlags,
}
pub type GFileAttributeInfo = _GFileAttributeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfoList {
    pub infos: *mut GFileAttributeInfo,
    pub n_infos: ::core::ffi::c_int,
}
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
pub type GIcon = _GIcon;
pub type GIOExtension = _GIOExtension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalVfs {
    pub parent: GVfs,
}
pub type GLocalVfs = _GLocalVfs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalVfsClass {
    pub parent_class: GVfsClass,
}
pub type GVfsClass = _GVfsClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfsClass {
    pub parent_class: GObjectClass,
    pub is_active: Option<unsafe extern "C" fn(*mut GVfs) -> gboolean>,
    pub get_file_for_path:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_file_for_uri:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_supported_uri_schemes: Option<unsafe extern "C" fn(*mut GVfs) -> *const *const gchar>,
    pub parse_name:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub local_file_add_info: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            guint64,
            *mut GFileAttributeMatcher,
            *mut GFileInfo,
            *mut GCancellable,
            *mut gpointer,
            *mut GDestroyNotify,
        ) -> (),
    >,
    pub add_writable_namespaces:
        Option<unsafe extern "C" fn(*mut GVfs, *mut GFileAttributeInfoList) -> ()>,
    pub local_file_set_attributes: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub local_file_removed:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> ()>,
    pub local_file_moved: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub deserialize_icon: Option<unsafe extern "C" fn(*mut GVfs, *mut GVariant) -> *mut GIcon>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GLocalVfsClass = _GLocalVfsClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
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
static mut safe_c2rust_g_local_vfs_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_vfs_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_vfs_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_local_vfs_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_vfs_get_type(),
        g_intern_static_string(b"GLocalVfs\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalVfsClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_vfs_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalVfs>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalVfs) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_vfs_init as unsafe extern "C" fn(*mut GLocalVfs) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-vfs\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"local\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GLocalVfs_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_local_vfs_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_vfs_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalVfs_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GLocalVfs_private_offset);
    }
    safe_c2rust_g_local_vfs_class_init(klass as *mut GLocalVfsClass);
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_finalize(mut object: *mut GObject) {
    (*(safe_c2rust_g_local_vfs_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_init(mut vfs: *mut GLocalVfs) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_vfs_new() -> *mut GVfs {
    return g_object_new(
        safe_c2rust__g_local_vfs_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GVfs;
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_get_file_for_path(
    mut vfs: *mut GVfs,
    mut path: *const ::core::ffi::c_char,
) -> *mut GFile {
    if *path as ::core::ffi::c_int == '\0' as i32 {
        return _g_dummy_file_new(path);
    } else {
        return _g_local_file_new(path);
    };
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_get_file_for_uri(
    mut vfs: *mut GVfs,
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    path = g_filename_from_uri(
        uri as *const gchar,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    if !path.is_null() {
        file = _g_local_file_new(path);
    } else {
        file = _g_dummy_file_new(uri);
    }
    g_free(path as gpointer);
    return file;
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_get_supported_uri_schemes(
    mut vfs: *mut GVfs,
) -> *const *const gchar {
    static mut safe_c2rust_uri_schemes: [*const gchar; 2] = [
        b"file\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<gchar>(),
    ];
    return &raw mut safe_c2rust_uri_schemes as *mut *const gchar;
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_parse_name(
    mut vfs: *mut GVfs,
    mut parse_name: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user_prefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rest: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !parse_name.is_null() {
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
            b"parse_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if g_ascii_strncasecmp(
        b"file:\0" as *const u8 as *const gchar,
        parse_name as *const gchar,
        5 as gsize,
    ) == 0 as ::core::ffi::c_int
    {
        filename = g_filename_from_uri(
            parse_name as *const gchar,
            ::core::ptr::null_mut::<*mut gchar>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
    } else if *parse_name as ::core::ffi::c_int == '~' as i32 {
        let mut user_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        user_start = parse_name.offset(1 as ::core::ffi::c_int as isize);
        parse_name = parse_name.offset(1);
        while *parse_name as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && *parse_name as ::core::ffi::c_int != '/' as i32
        {
            parse_name = parse_name.offset(1);
        }
        user_end = parse_name;
        if user_end == user_start {
            user_prefix =
                safe_c2rust_g_strdup_inline(g_get_home_dir() as *const ::core::ffi::c_char);
        } else {
            let mut passwd_file_entry: *mut passwd = ::core::ptr::null_mut::<passwd>();
            let mut user_name: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            user_name = g_strndup(
                user_start as *const gchar,
                user_end.offset_from(user_start) as ::core::ffi::c_long as gsize,
            ) as *mut ::core::ffi::c_char;
            passwd_file_entry =
                g_unix_get_passwd_entry(user_name, ::core::ptr::null_mut::<*mut GError>())
                    as *mut passwd;
            g_free(user_name as gpointer);
            if !passwd_file_entry.is_null() && !(*passwd_file_entry).pw_dir.is_null() {
                user_prefix = safe_c2rust_g_strdup_inline((*passwd_file_entry).pw_dir);
            } else {
                user_prefix =
                    safe_c2rust_g_strdup_inline(g_get_home_dir() as *const ::core::ffi::c_char);
            }
            g_free(passwd_file_entry as gpointer);
        }
        rest = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if *user_end as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            rest = g_filename_from_utf8(
                user_end as *const gchar,
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) as *mut ::core::ffi::c_char;
        }
        filename = g_build_filename(user_prefix, rest, NULL_0) as *mut ::core::ffi::c_char;
        g_free(rest as gpointer);
        g_free(user_prefix as gpointer);
    } else {
        filename = g_filename_from_utf8(
            parse_name as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
    }
    if filename.is_null() {
        filename = safe_c2rust_g_strdup_inline(parse_name);
    }
    file = _g_local_file_new(filename);
    g_free(filename as gpointer);
    return file;
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_is_active(mut vfs: *mut GVfs) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_vfs_class_init(mut class: *mut GLocalVfsClass) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut vfs_class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    object_class = class as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_local_vfs_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    vfs_class = class as *mut ::core::ffi::c_void as *mut GVfsClass;
    (*vfs_class).is_active =
        Some(safe_c2rust_g_local_vfs_is_active as unsafe extern "C" fn(*mut GVfs) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GVfs) -> gboolean>;
    (*vfs_class).get_file_for_path = Some(
        safe_c2rust_g_local_vfs_get_file_for_path
            as unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>;
    (*vfs_class).get_file_for_uri = Some(
        safe_c2rust_g_local_vfs_get_file_for_uri
            as unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>;
    (*vfs_class).get_supported_uri_schemes = Some(
        safe_c2rust_g_local_vfs_get_supported_uri_schemes
            as unsafe extern "C" fn(*mut GVfs) -> *const *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GVfs) -> *const *const gchar>;
    (*vfs_class).parse_name = Some(
        safe_c2rust_g_local_vfs_parse_name
            as unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
