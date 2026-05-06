extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn g_file_monitor_get_type() -> GType;
    fn g_file_monitor_is_cancelled(monitor: *mut GFileMonitor) -> gboolean;
    fn g_file_monitor_emit_event(
        monitor: *mut GFileMonitor,
        child: *mut GFile,
        other_file: *mut GFile,
        event_type: GFileMonitorEvent,
    );
    fn g_file_query_info_async(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_query_info(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_query_info_finish(
        file: *mut GFile,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_info_has_attribute(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_info_get_size(info: *mut GFileInfo) -> goffset;
    fn g_file_info_get_etag(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_file_info_get_attribute_uint64(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> guint64;
    fn g_file_info_get_attribute_uint32(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> guint32;
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
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFileMonitor {
    pub parent_instance: GFileMonitor,
    pub file: *mut GFile,
    pub last_info: *mut GFileInfo,
    pub timeout: *mut GSource,
}
pub type GPollFileMonitor = _GPollFileMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFileMonitorClass {
    pub parent_class: GFileMonitorClass,
}
pub type GPollFileMonitorClass = _GPollFileMonitorClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
pub const G_FILE_ATTRIBUTE_STANDARD_SIZE: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::size\0") };
pub const G_FILE_ATTRIBUTE_ETAG_VALUE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"etag::value\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"time::modified\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-nsec\0")
};
pub const POLL_TIME_SECS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_poll_file_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GPollFileMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GPollFileMonitor_private_offset,
        );
    }
    safe_c2rust_g_poll_file_monitor_class_init(klass as *mut GPollFileMonitorClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_monitor_get_type(),
        g_intern_static_string(b"GPollFileMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GPollFileMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_poll_file_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GPollFileMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPollFileMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_poll_file_monitor_init
                    as unsafe extern "C" fn(*mut GPollFileMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GPollFileMonitor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_poll_file_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_poll_file_monitor_get_type_once();
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
static mut safe_c2rust_g_poll_file_monitor_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_finalize(mut object: *mut GObject) {
    let mut poll_monitor: *mut GPollFileMonitor = ::core::ptr::null_mut::<GPollFileMonitor>();
    poll_monitor = object as *mut ::core::ffi::c_void as *mut GPollFileMonitor;
    safe_c2rust_g_poll_file_monitor_cancel(
        poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor,
    );
    g_object_unref((*poll_monitor).file as gpointer);
    let mut _pp: *mut *mut GFileInfo = &raw mut (*poll_monitor).last_info;
    let mut _ptr: *mut GFileInfo = *_pp;
    *_pp = ::core::ptr::null_mut::<GFileInfo>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_poll_file_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_class_init(
    mut klass: *mut GPollFileMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut file_monitor_class: *mut GFileMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GFileMonitorClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_poll_file_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*file_monitor_class).cancel = Some(
        safe_c2rust_g_poll_file_monitor_cancel
            as unsafe extern "C" fn(*mut GFileMonitor) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>;
}
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_init(mut poll_monitor: *mut GPollFileMonitor) {
}
unsafe extern "C" fn safe_c2rust_calc_event_type(
    mut last: *mut GFileInfo,
    mut new: *mut GFileInfo,
) -> ::core::ffi::c_int {
    if last.is_null() && new.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if last.is_null() && !new.is_null() {
        return G_FILE_MONITOR_EVENT_CREATED as ::core::ffi::c_int;
    }
    if !last.is_null() && new.is_null() {
        return G_FILE_MONITOR_EVENT_DELETED as ::core::ffi::c_int;
    }
    if g_file_info_has_attribute(last, G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr()) != 0
        && g_file_info_has_attribute(new, G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr()) != 0
        && g_strcmp0(g_file_info_get_etag(last), g_file_info_get_etag(new))
            != 0 as ::core::ffi::c_int
    {
        return G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int;
    }
    if g_file_info_has_attribute(last, G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr()) != 0
        && g_file_info_has_attribute(new, G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr()) != 0
        && g_file_info_get_size(last) != g_file_info_get_size(new)
    {
        return G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int;
    }
    if g_file_info_has_attribute(last, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr()) != 0
        && g_file_info_has_attribute(new, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr()) != 0
        && g_file_info_get_attribute_uint64(last, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr())
            != g_file_info_get_attribute_uint64(new, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr())
    {
        return G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int;
    }
    if g_file_info_has_attribute(last, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr()) != 0
        && g_file_info_has_attribute(new, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr()) != 0
        && g_file_info_get_attribute_uint32(last, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr())
            != g_file_info_get_attribute_uint32(new, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr())
    {
        return G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int;
    }
    if g_file_info_has_attribute(last, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr()) != 0
        && g_file_info_has_attribute(new, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr()) != 0
        && g_file_info_get_attribute_uint32(last, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr())
            != g_file_info_get_attribute_uint32(new, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr())
    {
        return G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn safe_c2rust_got_new_info(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut poll_monitor: *mut GPollFileMonitor = user_data as *mut GPollFileMonitor;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut event: ::core::ffi::c_int = 0;
    info = g_file_query_info_finish(
        (*poll_monitor).file,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if g_file_monitor_is_cancelled(poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor)
        == 0
    {
        event = safe_c2rust_calc_event_type((*poll_monitor).last_info, info);
        if event != -(1 as ::core::ffi::c_int) {
            g_file_monitor_emit_event(
                poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor,
                (*poll_monitor).file,
                ::core::ptr::null_mut::<GFile>(),
                event as GFileMonitorEvent,
            );
            if g_file_monitor_is_cancelled(
                poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor,
            ) == 0
                && (event == G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int
                    || event == G_FILE_MONITOR_EVENT_CREATED as ::core::ffi::c_int)
            {
                g_file_monitor_emit_event(
                    poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor,
                    (*poll_monitor).file,
                    ::core::ptr::null_mut::<GFile>(),
                    G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
                );
            }
        }
        if !(*poll_monitor).last_info.is_null() {
            g_object_unref((*poll_monitor).last_info as gpointer);
            (*poll_monitor).last_info = ::core::ptr::null_mut::<GFileInfo>();
        }
        if !info.is_null() {
            (*poll_monitor).last_info =
                g_object_ref(info as gpointer) as *mut GFileInfo as *mut GFileInfo;
        }
        safe_c2rust_schedule_poll_timeout(poll_monitor);
    }
    if !info.is_null() {
        g_object_unref(info as gpointer);
    }
    g_object_unref(poll_monitor as gpointer);
}
unsafe extern "C" fn safe_c2rust_poll_file_timeout(mut data: gpointer) -> gboolean {
    let mut poll_monitor: *mut GPollFileMonitor = data as *mut GPollFileMonitor;
    g_source_unref((*poll_monitor).timeout);
    (*poll_monitor).timeout = ::core::ptr::null_mut::<GSource>();
    g_file_query_info_async(
        (*poll_monitor).file,
        b"etag::value,standard::size,time::modified,time::modified-usec,time::modified-nsec\0"
            as *const u8 as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<GCancellable>(),
        Some(
            safe_c2rust_got_new_info
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(poll_monitor as gpointer) as *mut GPollFileMonitor as gpointer,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_schedule_poll_timeout(mut poll_monitor: *mut GPollFileMonitor) {
    (*poll_monitor).timeout = g_timeout_source_new_seconds(POLL_TIME_SECS as guint);
    g_source_set_callback(
        (*poll_monitor).timeout,
        Some(safe_c2rust_poll_file_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
        poll_monitor as gpointer,
        None,
    );
    g_source_attach((*poll_monitor).timeout, g_main_context_get_thread_default());
}
unsafe extern "C" fn safe_c2rust_got_initial_info(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut poll_monitor: *mut GPollFileMonitor = user_data as *mut GPollFileMonitor;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    info = g_file_query_info_finish(
        (*poll_monitor).file,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    (*poll_monitor).last_info = info;
    if g_file_monitor_is_cancelled(poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor)
        == 0
    {
        safe_c2rust_schedule_poll_timeout(poll_monitor);
    }
    g_object_unref(poll_monitor as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_poll_file_monitor_new(
    mut file: *mut GFile,
) -> *mut GFileMonitor {
    let mut poll_monitor: *mut GPollFileMonitor = ::core::ptr::null_mut::<GPollFileMonitor>();
    poll_monitor = g_object_new(
        safe_c2rust__g_poll_file_monitor_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GPollFileMonitor;
    (*poll_monitor).file = g_object_ref(file as gpointer) as *mut GFile as *mut GFile;
    (*poll_monitor).last_info = g_file_query_info(
        file,
        b"etag::value,standard::size,time::modified,time::modified-usec,time::modified-nsec\0"
            as *const u8 as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if g_file_monitor_is_cancelled(poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor)
        == 0
    {
        safe_c2rust_schedule_poll_timeout(poll_monitor);
    }
    return poll_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor;
}
unsafe extern "C" fn safe_c2rust_g_poll_file_monitor_cancel(
    mut monitor: *mut GFileMonitor,
) -> gboolean {
    let mut poll_monitor: *mut GPollFileMonitor =
        monitor as *mut ::core::ffi::c_void as *mut GPollFileMonitor;
    if !(*poll_monitor).timeout.is_null() {
        g_source_destroy((*poll_monitor).timeout);
        g_source_unref((*poll_monitor).timeout);
        (*poll_monitor).timeout = ::core::ptr::null_mut::<GSource>();
    }
    return TRUE;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
