extern "C" {
    pub type _GData;
    pub type _GSocketControlMessagePrivate;
    pub type _GUnixFDListPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_strerror(errnum: gint) -> *const gchar;
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
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_socket_control_message_get_type() -> GType;
    fn g_unix_fd_list_get_type() -> GType;
    fn g_unix_fd_list_new() -> *mut GUnixFDList;
    fn g_unix_fd_list_new_from_array(fds: *const gint, n_fds: gint) -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn g_unix_fd_list_get_length(list: *mut GUnixFDList) -> gint;
    fn g_unix_fd_list_peek_fds(list: *mut GUnixFDList, length: *mut gint) -> *const gint;
    fn g_unix_fd_list_steal_fds(list: *mut GUnixFDList, length: *mut gint) -> *mut gint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessageClass {
    pub parent_class: GObjectClass,
    pub get_size: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize>,
    pub get_level: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>,
    pub get_type: Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>,
    pub serialize: Option<unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> ()>,
    pub deserialize: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            gsize,
            gpointer,
        ) -> *mut GSocketControlMessage,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketControlMessageClass = _GSocketControlMessageClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDMessagePrivate {
    pub list: *mut GUnixFDList,
}
pub type GUnixFDMessagePrivate = _GUnixFDMessagePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDMessageClass {
    pub parent_class: GSocketControlMessageClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
}
pub type GUnixFDMessageClass = _GUnixFDMessageClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDMessage {
    pub parent_instance: GSocketControlMessage,
    pub priv_0: *mut GUnixFDMessagePrivate,
}
pub type GUnixFDMessage = _GUnixFDMessage;
pub const SCM_RIGHTS: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCM_PIDFD: C2RustUnnamed_0 = 4;
pub const SCM_SECURITY: C2RustUnnamed_0 = 3;
pub const SCM_CREDENTIALS: C2RustUnnamed_0 = 2;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut safe_c2rust_GUnixFDMessage_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_instance_private(
    mut self_0: *mut GUnixFDMessage,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GUnixFDMessage_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_unix_fd_message_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_control_message_get_type(),
        g_intern_static_string(b"GUnixFDMessage\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixFDMessageClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_fd_message_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixFDMessage>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixFDMessage) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_fd_message_init
                    as unsafe extern "C" fn(*mut GUnixFDMessage) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GUnixFDMessage_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GUnixFDMessagePrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_fd_message_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_fd_message_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixFDMessage_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixFDMessage_private_offset,
        );
    }
    safe_c2rust_g_unix_fd_message_class_init(klass as *mut GUnixFDMessageClass);
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_size(
    mut message: *mut GSocketControlMessage,
) -> gsize {
    let mut fd_message: *mut GUnixFDMessage =
        message as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
    return (g_unix_fd_list_get_length((*(*fd_message).priv_0).list) as gsize)
        .wrapping_mul(::core::mem::size_of::<gint>() as gsize);
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_level(
    mut message: *mut GSocketControlMessage,
) -> ::core::ffi::c_int {
    return SOL_SOCKET;
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_msg_type(
    mut message: *mut GSocketControlMessage,
) -> ::core::ffi::c_int {
    return SCM_RIGHTS as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_deserialize(
    mut level: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut size: gsize,
    mut data: gpointer,
) -> *mut GSocketControlMessage {
    let mut message: *mut GSocketControlMessage = ::core::ptr::null_mut::<GSocketControlMessage>();
    let mut list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    let mut n: gint = 0;
    let mut s: gint = 0;
    let mut i: gint = 0;
    let mut fds: *mut gint = ::core::ptr::null_mut::<gint>();
    if level != SOL_SOCKET || type_0 != SCM_RIGHTS as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    if size.wrapping_rem(4 as gsize) > 0 as gsize {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Kernel returned non-integral number of fds\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GSocketControlMessage>();
    }
    fds = data as *mut gint;
    n = (size as usize).wrapping_div(::core::mem::size_of::<gint>() as usize) as gint;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n {
        let mut errsv: ::core::ffi::c_int = 0;
        loop {
            s = fcntl(
                *fds.offset(i as isize) as ::core::ffi::c_int,
                F_SETFD,
                FD_CLOEXEC,
            ) as gint;
            errsv = *__errno_location();
            if !(s < 0 as ::core::ffi::c_int && errsv == EINTR) {
                break;
            }
        }
        if s < 0 as ::core::ffi::c_int {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Error setting close-on-exec flag on incoming fd: %s\0" as *const u8
                    as *const gchar,
                g_strerror(errsv as gint),
            );
            return ::core::ptr::null_mut::<GSocketControlMessage>();
        }
        i += 1;
    }
    list = g_unix_fd_list_new_from_array(fds, n);
    message = safe_c2rust_g_unix_fd_message_new_with_fd_list(list);
    g_object_unref(list as gpointer);
    return message;
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_serialize(
    mut message: *mut GSocketControlMessage,
    mut data: gpointer,
) {
    let mut fd_message: *mut GUnixFDMessage =
        message as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
    let mut fds: *const gint = ::core::ptr::null::<gint>();
    let mut n_fds: gint = 0;
    fds = g_unix_fd_list_peek_fds((*(*fd_message).priv_0).list, &raw mut n_fds);
    memcpy(
        data as *mut ::core::ffi::c_void,
        fds as *const ::core::ffi::c_void,
        (::core::mem::size_of::<gint>() as size_t).wrapping_mul(n_fds as size_t),
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut message: *mut GUnixFDMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(*message).priv_0).list.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixfdmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            146 as ::core::ffi::c_int,
            G_STRFUNC,
            b"message->priv->list == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut __n1: gint64 = prop_id as gint64;
    let mut __n2: gint64 = 1 as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixfdmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            147 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id == 1\0" as *const u8 as *const ::core::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const ::core::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as ::core::ffi::c_char,
        );
    }
    (*(*message).priv_0).list = g_value_dup_object(value) as *mut GUnixFDList;
    if (*(*message).priv_0).list.is_null() {
        (*(*message).priv_0).list = g_unix_fd_list_new();
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_fd_list(
    mut message: *mut GUnixFDMessage,
) -> *mut GUnixFDList {
    return (*(*message).priv_0).list;
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut message: *mut GUnixFDMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
    let mut __n1: gint64 = prop_id as gint64;
    let mut __n2: gint64 = 1 as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixfdmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            179 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id == 1\0" as *const u8 as *const ::core::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const ::core::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as ::core::ffi::c_char,
        );
    }
    g_value_set_object(
        value,
        safe_c2rust_g_unix_fd_message_get_fd_list(message) as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_init(mut message: *mut GUnixFDMessage) {
    (*message).priv_0 =
        safe_c2rust_g_unix_fd_message_get_instance_private(message) as *mut GUnixFDMessagePrivate;
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_finalize(mut object: *mut GObject) {
    let mut message: *mut GUnixFDMessage =
        object as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
    g_object_unref((*(*message).priv_0).list as gpointer);
    (*(safe_c2rust_g_unix_fd_message_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_fd_message_class_init(mut class: *mut GUnixFDMessageClass) {
    let mut scm_class: *mut GSocketControlMessageClass =
        class as *mut ::core::ffi::c_void as *mut GSocketControlMessageClass;
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*scm_class).get_size = Some(
        safe_c2rust_g_unix_fd_message_get_size
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> gsize>;
    (*scm_class).get_level = Some(
        safe_c2rust_g_unix_fd_message_get_level
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>;
    (*scm_class).get_type = Some(
        safe_c2rust_g_unix_fd_message_get_msg_type
            as unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage) -> ::core::ffi::c_int>;
    (*scm_class).serialize = Some(
        safe_c2rust_g_unix_fd_message_serialize
            as unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSocketControlMessage, gpointer) -> ()>;
    (*scm_class).deserialize = Some(
        safe_c2rust_g_unix_fd_message_deserialize
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                gsize,
                gpointer,
            ) -> *mut GSocketControlMessage,
    )
        as Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                gsize,
                gpointer,
            ) -> *mut GSocketControlMessage,
        >;
    (*object_class).finalize =
        Some(safe_c2rust_g_unix_fd_message_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_unix_fd_message_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_unix_fd_message_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        object_class,
        1 as guint,
        g_param_spec_object(
            b"fd-list\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_unix_fd_list_get_type(),
            (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_new() -> *mut GSocketControlMessage {
    return g_object_new(
        safe_c2rust_g_unix_fd_message_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSocketControlMessage;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_new_with_fd_list(
    mut fd_list: *mut GUnixFDList,
) -> *mut GSocketControlMessage {
    return g_object_new(
        safe_c2rust_g_unix_fd_message_get_type(),
        b"fd-list\0" as *const u8 as *const gchar,
        fd_list,
        NULL,
    ) as *mut GSocketControlMessage;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_steal_fds(
    mut message: *mut GUnixFDMessage,
    mut length: *mut gint,
) -> *mut gint {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !(message as *mut ::core::ffi::c_void as *mut GUnixFDMessage).is_null() {
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
            b"G_UNIX_FD_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gint>();
    }
    return g_unix_fd_list_steal_fds((*(*message).priv_0).list, length);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_fd_message_append_fd(
    mut message: *mut GUnixFDMessage,
    mut fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !(message as *mut ::core::ffi::c_void as *mut GUnixFDMessage).is_null() {
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
            b"G_UNIX_FD_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (g_unix_fd_list_append((*(*message).priv_0).list, fd, error) >= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
