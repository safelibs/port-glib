extern "C" {
    pub type _GData;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GIOExtension;
    pub type _GUnixMountMonitor;
    pub type _GFileMonitorSource;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn g_local_file_monitor_get_type() -> GType;
    fn _ih_sub_new(
        dirname: *const gchar,
        basename: *const gchar,
        filename: *const gchar,
        user_data: gpointer,
    ) -> *mut inotify_sub;
    fn _ih_sub_free(sub: *mut inotify_sub);
    fn _ih_startup() -> gboolean;
    fn _ih_sub_add(sub: *mut inotify_sub) -> gboolean;
    fn _ih_sub_cancel(sub: *mut inotify_sub) -> gboolean;
}
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
pub type GData = _GData;
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
pub type GFileMonitorEvent = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_EVENT_MOVED_OUT: GFileMonitorEvent = 10;
pub const G_FILE_MONITOR_EVENT_MOVED_IN: GFileMonitorEvent = 9;
pub const G_FILE_MONITOR_EVENT_RENAMED: GFileMonitorEvent = 8;
pub const G_FILE_MONITOR_EVENT_MOVED: GFileMonitorEvent = 7;
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: GFileMonitorEvent = 6;
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: GFileMonitorEvent = 5;
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: GFileMonitorEvent = 4;
pub const G_FILE_MONITOR_EVENT_CREATED: GFileMonitorEvent = 3;
pub const G_FILE_MONITOR_EVENT_DELETED: GFileMonitorEvent = 2;
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: GFileMonitorEvent = 1;
pub const G_FILE_MONITOR_EVENT_CHANGED: GFileMonitorEvent = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GIOExtension = _GIOExtension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitorClass {
    pub parent_class: GObjectClass,
    pub changed: Option<
        unsafe extern "C" fn(*mut GFileMonitor, *mut GFile, *mut GFile, GFileMonitorEvent) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileMonitorClass = _GFileMonitorClass;
pub type GUnixMountMonitor = _GUnixMountMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileMonitor {
    pub parent_instance: GFileMonitor,
    pub source: *mut GFileMonitorSource,
    pub mount_monitor: *mut GUnixMountMonitor,
    pub was_mounted: gboolean,
}
pub type GFileMonitorSource = _GFileMonitorSource;
pub type GLocalFileMonitor = _GLocalFileMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileMonitorClass {
    pub parent_class: GFileMonitorClass,
    pub is_supported: Option<unsafe extern "C" fn() -> gboolean>,
    pub start: Option<
        unsafe extern "C" fn(
            *mut GLocalFileMonitor,
            *const gchar,
            *const gchar,
            *const gchar,
            *mut GFileMonitorSource,
        ) -> (),
    >,
    pub mount_notify: gboolean,
}
pub type GLocalFileMonitorClass = _GLocalFileMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInotifyFileMonitor {
    pub parent_instance: GLocalFileMonitor,
    pub sub: *mut inotify_sub,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_sub {
    pub dirname: *mut gchar,
    pub filename: *mut gchar,
    pub cancelled: gboolean,
    pub user_data: gpointer,
    pub pair_moves: gboolean,
    pub hardlinks: gboolean,
}
pub type GInotifyFileMonitor = _GInotifyFileMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInotifyFileMonitorClass {
    pub parent_class: GLocalFileMonitorClass,
}
pub type GInotifyFileMonitorClass = _GInotifyFileMonitorClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_inotify_file_monitor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_local_file_monitor_get_type(),
        g_intern_static_string(b"GInotifyFileMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GInotifyFileMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inotify_file_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GInotifyFileMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInotifyFileMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inotify_file_monitor_init
                    as unsafe extern "C" fn(*mut GInotifyFileMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    g_io_extension_point_implement(
        b"gio-local-file-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"inotify\0" as *const u8 as *const ::core::ffi::c_char,
        20 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GInotifyFileMonitor_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_inotify_file_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GInotifyFileMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GInotifyFileMonitor_private_offset,
        );
    }
    safe_c2rust_g_inotify_file_monitor_class_init(klass as *mut GInotifyFileMonitorClass);
}
static mut safe_c2rust_g_inotify_file_monitor_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_is_supported() -> gboolean {
    return _ih_startup();
}
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_start(
    mut local_monitor: *mut GLocalFileMonitor,
    mut dirname: *const gchar,
    mut basename: *const gchar,
    mut filename: *const gchar,
    mut source: *mut GFileMonitorSource,
) {
    let mut inotify_monitor: *mut GInotifyFileMonitor =
        local_monitor as *mut ::core::ffi::c_void as *mut GInotifyFileMonitor;
    let mut success: gboolean = 0;
    success = _ih_startup();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if success != 0 {
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
            b"../original/gio/inotify/ginotifyfilemonitor.c\0" as *const u8
                as *const ::core::ffi::c_char,
            62 as ::core::ffi::c_int,
            G_STRFUNC,
            b"success\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*inotify_monitor).sub = _ih_sub_new(dirname, basename, filename, source as gpointer);
    _ih_sub_add((*inotify_monitor).sub);
}
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_cancel(
    mut monitor: *mut GFileMonitor,
) -> gboolean {
    let mut inotify_monitor: *mut GInotifyFileMonitor =
        monitor as *mut ::core::ffi::c_void as *mut GInotifyFileMonitor;
    if !(*inotify_monitor).sub.is_null() {
        _ih_sub_cancel((*inotify_monitor).sub);
        _ih_sub_free((*inotify_monitor).sub);
        (*inotify_monitor).sub = ::core::ptr::null_mut::<inotify_sub>();
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_finalize(mut object: *mut GObject) {
    let mut inotify_monitor: *mut GInotifyFileMonitor =
        object as *mut ::core::ffi::c_void as *mut GInotifyFileMonitor;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*inotify_monitor).sub.is_null() {
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
            b"../original/gio/inotify/ginotifyfilemonitor.c\0" as *const u8
                as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!inotify_monitor->sub\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(safe_c2rust_g_inotify_file_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_init(
    mut monitor: *mut GInotifyFileMonitor,
) {
}
unsafe extern "C" fn safe_c2rust_g_inotify_file_monitor_class_init(
    mut klass: *mut GInotifyFileMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut file_monitor_class: *mut GFileMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GFileMonitorClass;
    let mut local_file_monitor_class: *mut GLocalFileMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GLocalFileMonitorClass;
    (*local_file_monitor_class).is_supported =
        Some(safe_c2rust_g_inotify_file_monitor_is_supported as unsafe extern "C" fn() -> gboolean)
            as Option<unsafe extern "C" fn() -> gboolean>;
    (*local_file_monitor_class).start = Some(
        safe_c2rust_g_inotify_file_monitor_start
            as unsafe extern "C" fn(
                *mut GLocalFileMonitor,
                *const gchar,
                *const gchar,
                *const gchar,
                *mut GFileMonitorSource,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLocalFileMonitor,
                *const gchar,
                *const gchar,
                *const gchar,
                *mut GFileMonitorSource,
            ) -> (),
        >;
    (*local_file_monitor_class).mount_notify = TRUE as gboolean;
    (*file_monitor_class).cancel = Some(
        safe_c2rust_g_inotify_file_monitor_cancel
            as unsafe extern "C" fn(*mut GFileMonitor) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_inotify_file_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
