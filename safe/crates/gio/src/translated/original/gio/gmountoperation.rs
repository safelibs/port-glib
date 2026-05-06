use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_idle_add_full(
        priority: gint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    ) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
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
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_strv_get_type() -> GType;
    fn g_array_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
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
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_ask_password_flags_get_type() -> GType;
    fn g_password_save_get_type() -> GType;
    fn g_mount_operation_result_get_type() -> GType;
    fn _g_cclosure_marshal_VOID__STRING_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__STRING_BOXED_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_BOXED_BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__STRING_INT64_INT64(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_INT64_INT64v(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGS(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGSv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GQuark = guint32;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
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
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
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
pub type GSignalCVaMarshaller = GVaClosureMarshal;
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
pub type GAskPasswordFlags = ::core::ffi::c_uint;
pub const G_ASK_PASSWORD_TCRYPT: GAskPasswordFlags = 32;
pub const G_ASK_PASSWORD_ANONYMOUS_SUPPORTED: GAskPasswordFlags = 16;
pub const G_ASK_PASSWORD_SAVING_SUPPORTED: GAskPasswordFlags = 8;
pub const G_ASK_PASSWORD_NEED_DOMAIN: GAskPasswordFlags = 4;
pub const G_ASK_PASSWORD_NEED_USERNAME: GAskPasswordFlags = 2;
pub const G_ASK_PASSWORD_NEED_PASSWORD: GAskPasswordFlags = 1;
pub type GPasswordSave = ::core::ffi::c_uint;
pub const G_PASSWORD_SAVE_PERMANENTLY: GPasswordSave = 2;
pub const G_PASSWORD_SAVE_FOR_SESSION: GPasswordSave = 1;
pub const G_PASSWORD_SAVE_NEVER: GPasswordSave = 0;
pub type GMountOperationResult = ::core::ffi::c_uint;
pub const G_MOUNT_OPERATION_UNHANDLED: GMountOperationResult = 2;
pub const G_MOUNT_OPERATION_ABORTED: GMountOperationResult = 1;
pub const G_MOUNT_OPERATION_HANDLED: GMountOperationResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperation {
    pub parent_instance: GObject,
    pub priv_0: *mut GMountOperationPrivate,
}
pub type GMountOperationPrivate = _GMountOperationPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperationPrivate {
    pub password: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub domain: *mut ::core::ffi::c_char,
    pub anonymous: gboolean,
    pub password_save: GPasswordSave,
    pub choice: ::core::ffi::c_int,
    pub hidden_volume: gboolean,
    pub system_volume: gboolean,
    pub pim: guint,
}
pub type GMountOperation = _GMountOperation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperationClass {
    pub parent_class: GObjectClass,
    pub ask_password: Option<
        unsafe extern "C" fn(
            *mut GMountOperation,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            GAskPasswordFlags,
        ) -> (),
    >,
    pub ask_question: Option<
        unsafe extern "C" fn(
            *mut GMountOperation,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub reply: Option<unsafe extern "C" fn(*mut GMountOperation, GMountOperationResult) -> ()>,
    pub aborted: Option<unsafe extern "C" fn(*mut GMountOperation) -> ()>,
    pub show_processes: Option<
        unsafe extern "C" fn(
            *mut GMountOperation,
            *const gchar,
            *mut GArray,
            *mut *const gchar,
        ) -> (),
    >,
    pub show_unmount_progress:
        Option<unsafe extern "C" fn(*mut GMountOperation, *const gchar, gint64, gint64) -> ()>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved9: Option<unsafe extern "C" fn() -> ()>,
}
pub type GMountOperationClass = _GMountOperationClass;
pub const PROP_PIM: C2RustUnnamed_1 = 9;
pub const PROP_IS_TCRYPT_SYSTEM_VOLUME: C2RustUnnamed_1 = 8;
pub const PROP_IS_TCRYPT_HIDDEN_VOLUME: C2RustUnnamed_1 = 7;
pub const PROP_CHOICE: C2RustUnnamed_1 = 6;
pub const PROP_PASSWORD_SAVE: C2RustUnnamed_1 = 5;
pub const PROP_DOMAIN: C2RustUnnamed_1 = 4;
pub const PROP_ANONYMOUS: C2RustUnnamed_1 = 3;
pub const PROP_PASSWORD: C2RustUnnamed_1 = 2;
pub const PROP_USERNAME: C2RustUnnamed_1 = 1;
pub const SHOW_UNMOUNT_PROGRESS: C2RustUnnamed_0 = 5;
pub const SHOW_PROCESSES: C2RustUnnamed_0 = 4;
pub const ABORTED: C2RustUnnamed_0 = 3;
pub const REPLY: C2RustUnnamed_0 = 2;
pub const ASK_QUESTION: C2RustUnnamed_0 = 1;
pub const ASK_PASSWORD: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_0 = 6;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_PRIORITY_DEFAULT_IDLE: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
pub const G_TYPE_INT64: GType = ((10 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 6] = [0 as ::core::ffi::c_int as guint, 0, 0, 0, 0, 0];
static mut safe_c2rust_g_mount_operation_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_mount_operation_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMountOperation\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMountOperationClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_mount_operation_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMountOperation>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMountOperation) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_mount_operation_init
                    as unsafe extern "C" fn(*mut GMountOperation) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GMountOperation_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GMountOperationPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_mount_operation_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_mount_operation_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_mount_operation_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMountOperation_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMountOperation_private_offset,
        );
    }
    safe_c2rust_g_mount_operation_class_init(klass as *mut GMountOperationClass);
}
static mut safe_c2rust_GMountOperation_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_mount_operation_get_instance_private(
    mut self_0: *mut GMountOperation,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GMountOperation_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_mount_operation_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut operation: *mut GMountOperation = ::core::ptr::null_mut::<GMountOperation>();
    operation = object as *mut ::core::ffi::c_void as *mut GMountOperation;
    match prop_id {
        1 => {
            safe_c2rust_g_mount_operation_set_username(
                operation,
                g_value_get_string(value) as *const ::core::ffi::c_char,
            );
        }
        2 => {
            safe_c2rust_g_mount_operation_set_password(
                operation,
                g_value_get_string(value) as *const ::core::ffi::c_char,
            );
        }
        3 => {
            safe_c2rust_g_mount_operation_set_anonymous(operation, g_value_get_boolean(value));
        }
        4 => {
            safe_c2rust_g_mount_operation_set_domain(
                operation,
                g_value_get_string(value) as *const ::core::ffi::c_char,
            );
        }
        5 => {
            safe_c2rust_g_mount_operation_set_password_save(
                operation,
                g_value_get_enum(value) as GPasswordSave,
            );
        }
        6 => {
            safe_c2rust_g_mount_operation_set_choice(
                operation,
                g_value_get_int(value) as ::core::ffi::c_int,
            );
        }
        7 => {
            safe_c2rust_g_mount_operation_set_is_tcrypt_hidden_volume(
                operation,
                g_value_get_boolean(value),
            );
        }
        8 => {
            safe_c2rust_g_mount_operation_set_is_tcrypt_system_volume(
                operation,
                g_value_get_boolean(value),
            );
        }
        9 => {
            safe_c2rust_g_mount_operation_set_pim(operation, g_value_get_uint(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmountoperation.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_mount_operation_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut operation: *mut GMountOperation = ::core::ptr::null_mut::<GMountOperation>();
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    operation = object as *mut ::core::ffi::c_void as *mut GMountOperation;
    priv_0 = (*operation).priv_0;
    match prop_id {
        1 => {
            g_value_set_string(value, (*priv_0).user);
        }
        2 => {
            g_value_set_string(value, (*priv_0).password);
        }
        3 => {
            g_value_set_boolean(value, (*priv_0).anonymous);
        }
        4 => {
            g_value_set_string(value, (*priv_0).domain);
        }
        5 => {
            g_value_set_enum(value, (*priv_0).password_save as gint);
        }
        6 => {
            g_value_set_int(value, (*priv_0).choice as gint);
        }
        7 => {
            g_value_set_boolean(value, (*priv_0).hidden_volume);
        }
        8 => {
            g_value_set_boolean(value, (*priv_0).system_volume);
        }
        9 => {
            g_value_set_uint(value, (*priv_0).pim);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmountoperation.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                214 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_mount_operation_finalize(mut object: *mut GObject) {
    let mut operation: *mut GMountOperation = ::core::ptr::null_mut::<GMountOperation>();
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    operation = object as *mut ::core::ffi::c_void as *mut GMountOperation;
    priv_0 = (*operation).priv_0;
    g_free((*priv_0).password as gpointer);
    g_free((*priv_0).user as gpointer);
    g_free((*priv_0).domain as gpointer);
    (*(safe_c2rust_g_mount_operation_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_reply_non_handled_in_idle(mut data: gpointer) -> gboolean {
    let mut op: *mut GMountOperation = data as *mut GMountOperation;
    safe_c2rust_g_mount_operation_reply(op, G_MOUNT_OPERATION_UNHANDLED);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_ask_password(
    mut op: *mut GMountOperation,
    mut message: *const ::core::ffi::c_char,
    mut default_user: *const ::core::ffi::c_char,
    mut default_domain: *const ::core::ffi::c_char,
    mut flags: GAskPasswordFlags,
) {
    g_idle_add_full(
        G_PRIORITY_DEFAULT_IDLE,
        Some(safe_c2rust_reply_non_handled_in_idle as unsafe extern "C" fn(gpointer) -> gboolean),
        g_object_ref(op as gpointer) as *mut GMountOperation as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_ask_question(
    mut op: *mut GMountOperation,
    mut message: *const ::core::ffi::c_char,
    mut choices: *mut *const ::core::ffi::c_char,
) {
    g_idle_add_full(
        G_PRIORITY_DEFAULT_IDLE,
        Some(safe_c2rust_reply_non_handled_in_idle as unsafe extern "C" fn(gpointer) -> gboolean),
        g_object_ref(op as gpointer) as *mut GMountOperation as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_show_processes(
    mut op: *mut GMountOperation,
    mut message: *const gchar,
    mut processes: *mut GArray,
    mut choices: *mut *const gchar,
) {
    g_idle_add_full(
        G_PRIORITY_DEFAULT_IDLE,
        Some(safe_c2rust_reply_non_handled_in_idle as unsafe extern "C" fn(gpointer) -> gboolean),
        g_object_ref(op as gpointer) as *mut GMountOperation as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_show_unmount_progress(
    mut op: *mut GMountOperation,
    mut message: *const gchar,
    mut time_left: gint64,
    mut bytes_left: gint64,
) {
}
unsafe extern "C" fn safe_c2rust_g_mount_operation_class_init(
    mut klass: *mut GMountOperationClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_mount_operation_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_mount_operation_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_mount_operation_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*klass).ask_password = Some(
        safe_c2rust_ask_password
            as unsafe extern "C" fn(
                *mut GMountOperation,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                GAskPasswordFlags,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMountOperation,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                GAskPasswordFlags,
            ) -> (),
        >;
    (*klass).ask_question = Some(
        safe_c2rust_ask_question
            as unsafe extern "C" fn(
                *mut GMountOperation,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMountOperation,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> (),
        >;
    (*klass).show_processes = Some(
        safe_c2rust_show_processes
            as unsafe extern "C" fn(
                *mut GMountOperation,
                *const gchar,
                *mut GArray,
                *mut *const gchar,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMountOperation,
                *const gchar,
                *mut GArray,
                *mut *const gchar,
            ) -> (),
        >;
    (*klass).show_unmount_progress = Some(
        safe_c2rust_show_unmount_progress
            as unsafe extern "C" fn(*mut GMountOperation, *const gchar, gint64, gint64) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMountOperation, *const gchar, gint64, gint64) -> ()>;
    safe_c2rust_signals[ASK_PASSWORD as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"ask-password\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGS
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        4 as guint,
        G_TYPE_STRING,
        G_TYPE_STRING,
        G_TYPE_STRING,
        g_ask_password_flags_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[ASK_PASSWORD as ::core::ffi::c_int as usize],
        (*(object_class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGSv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[ASK_QUESTION as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"ask-question\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        144 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOXED
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        2 as guint,
        G_TYPE_STRING,
        g_strv_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[ASK_QUESTION as ::core::ffi::c_int as usize],
        (*(object_class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOXEDv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[REPLY as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"reply\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        152 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_mount_operation_result_get_type(),
    );
    safe_c2rust_signals[ABORTED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"aborted\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        160 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    safe_c2rust_signals[SHOW_PROCESSES as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"show-processes\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        168 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOXED_BOXED
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        3 as guint,
        G_TYPE_STRING,
        g_array_get_type(),
        g_strv_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[SHOW_PROCESSES as ::core::ffi::c_int as usize],
        (*(object_class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOXED_BOXEDv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[SHOW_UNMOUNT_PROGRESS as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"show-unmount-progress\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        176 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__STRING_INT64_INT64
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        3 as guint,
        G_TYPE_STRING,
        G_TYPE_INT64,
        G_TYPE_INT64,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[SHOW_UNMOUNT_PROGRESS as ::core::ffi::c_int as usize],
        (*(object_class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_INT64_INT64v
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_USERNAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"username\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PASSWORD as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"password\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_ANONYMOUS as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"anonymous\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_DOMAIN as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"domain\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PASSWORD_SAVE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"password-save\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_password_save_get_type(),
            G_PASSWORD_SAVE_NEVER as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_CHOICE as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"choice\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as gint,
            G_MAXINT,
            0 as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_TCRYPT_HIDDEN_VOLUME as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-tcrypt-hidden-volume\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_TCRYPT_SYSTEM_VOLUME as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-tcrypt-system-volume\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PIM as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"pim\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_mount_operation_init(mut operation: *mut GMountOperation) {
    (*operation).priv_0 = safe_c2rust_g_mount_operation_get_instance_private(operation)
        as *mut GMountOperationPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_new() -> *mut GMountOperation {
    return g_object_new(
        safe_c2rust_g_mount_operation_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GMountOperation;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_username(
    mut op: *mut GMountOperation,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(*op).priv_0).user;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_username(
    mut op: *mut GMountOperation,
    mut username: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*op).priv_0).user as gpointer);
    (*(*op).priv_0).user = safe_c2rust_g_strdup_inline(username);
    g_object_notify(
        op as *mut ::core::ffi::c_void as *mut GObject,
        b"username\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_password(
    mut op: *mut GMountOperation,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(*op).priv_0).password;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_password(
    mut op: *mut GMountOperation,
    mut password: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*op).priv_0).password as gpointer);
    (*(*op).priv_0).password = safe_c2rust_g_strdup_inline(password);
    g_object_notify(
        op as *mut ::core::ffi::c_void as *mut GObject,
        b"password\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_anonymous(
    mut op: *mut GMountOperation,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*op).priv_0).anonymous;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_anonymous(
    mut op: *mut GMountOperation,
    mut anonymous: gboolean,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).anonymous != anonymous {
        (*priv_0).anonymous = anonymous;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"anonymous\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_domain(
    mut op: *mut GMountOperation,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(*op).priv_0).domain;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_domain(
    mut op: *mut GMountOperation,
    mut domain: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*op).priv_0).domain as gpointer);
    (*(*op).priv_0).domain = safe_c2rust_g_strdup_inline(domain);
    g_object_notify(
        op as *mut ::core::ffi::c_void as *mut GObject,
        b"domain\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_password_save(
    mut op: *mut GMountOperation,
) -> GPasswordSave {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_PASSWORD_SAVE_NEVER;
    }
    return (*(*op).priv_0).password_save;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_password_save(
    mut op: *mut GMountOperation,
    mut save: GPasswordSave,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).password_save as ::core::ffi::c_uint != save as ::core::ffi::c_uint {
        (*priv_0).password_save = save;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"password-save\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_choice(
    mut op: *mut GMountOperation,
) -> ::core::ffi::c_int {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return (*(*op).priv_0).choice;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_choice(
    mut op: *mut GMountOperation,
    mut choice: ::core::ffi::c_int,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).choice != choice {
        (*priv_0).choice = choice;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"choice\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_is_tcrypt_hidden_volume(
    mut op: *mut GMountOperation,
) -> gboolean {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*op).priv_0).hidden_volume;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_is_tcrypt_hidden_volume(
    mut op: *mut GMountOperation,
    mut hidden_volume: gboolean,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).hidden_volume != hidden_volume {
        (*priv_0).hidden_volume = hidden_volume;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"is-tcrypt-hidden-volume\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_is_tcrypt_system_volume(
    mut op: *mut GMountOperation,
) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*op).priv_0).system_volume;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_is_tcrypt_system_volume(
    mut op: *mut GMountOperation,
    mut system_volume: gboolean,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).system_volume != system_volume {
        (*priv_0).system_volume = system_volume;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"is-tcrypt-system-volume\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_get_pim(
    mut op: *mut GMountOperation,
) -> guint {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(*op).priv_0).pim;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_set_pim(
    mut op: *mut GMountOperation,
    mut pim: guint,
) {
    let mut priv_0: *mut GMountOperationPrivate = ::core::ptr::null_mut::<GMountOperationPrivate>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*op).priv_0;
    if (*priv_0).pim != pim {
        (*priv_0).pim = pim;
        g_object_notify(
            op as *mut ::core::ffi::c_void as *mut GObject,
            b"pim\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_reply(
    mut op: *mut GMountOperation,
    mut result: GMountOperationResult,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = op as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_mount_operation_get_type();
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
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_MOUNT_OPERATION (op)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_emit(
        op as gpointer,
        safe_c2rust_signals[REPLY as ::core::ffi::c_int as usize],
        0 as GQuark,
        result as ::core::ffi::c_uint,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
