extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GCancellablePrivate;
    pub type _GDatagramBased;
    pub type _GSocketControlMessage;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_cancellable_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
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
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GDatagramBased = _GDatagramBased;
pub type GSocketControlMessage = _GSocketControlMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputVector {
    pub buffer: gpointer,
    pub size: gsize,
}
pub type GInputVector = _GInputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputMessage {
    pub address: *mut *mut GSocketAddress,
    pub vectors: *mut GInputVector,
    pub num_vectors: guint,
    pub bytes_received: gsize,
    pub flags: gint,
    pub control_messages: *mut *mut *mut GSocketControlMessage,
    pub num_control_messages: *mut guint,
}
pub type GInputMessage = _GInputMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputMessage {
    pub address: *mut GSocketAddress,
    pub vectors: *mut GOutputVector,
    pub num_vectors: guint,
    pub bytes_sent: guint,
    pub control_messages: *mut *mut GSocketControlMessage,
    pub num_control_messages: guint,
}
pub type GOutputMessage = _GOutputMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDatagramBasedInterface {
    pub g_iface: GTypeInterface,
    pub receive_messages: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            *mut GInputMessage,
            guint,
            gint,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gint,
    >,
    pub send_messages: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            *mut GOutputMessage,
            guint,
            gint,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gint,
    >,
    pub create_source: Option<
        unsafe extern "C" fn(*mut GDatagramBased, GIOCondition, *mut GCancellable) -> *mut GSource,
    >,
    pub condition_check:
        Option<unsafe extern "C" fn(*mut GDatagramBased, GIOCondition) -> GIOCondition>,
    pub condition_wait: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            GIOCondition,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type GDatagramBasedInterface = _GDatagramBasedInterface;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GDatagramBased\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDatagramBasedInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDatagramBasedInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_datagram_based_default_init
                        as unsafe extern "C" fn(*mut GDatagramBasedInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_g_datagram_based_default_init(
    mut iface: *mut GDatagramBasedInterface,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_receive_messages(
    mut datagram_based: *mut GDatagramBased,
    mut messages: *mut GInputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    let mut iface: *mut GDatagramBasedInterface =
        ::core::ptr::null_mut::<GDatagramBasedInterface>();
    let mut retval: gint = 0;
    let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = datagram_based as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_datagram_based_get_type();
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
            b"G_IS_DATAGRAM_BASED (datagram_based)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if num_messages == 0 as guint || !messages.is_null() {
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
            b"num_messages == 0 || messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    iface = g_type_interface_peek(
        (*(datagram_based as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_datagram_based_get_type(),
    ) as *mut GDatagramBasedInterface;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*iface).receive_messages.is_some() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatagrambased.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            188 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->receive_messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    retval = (*iface)
        .receive_messages
        .expect("non-null function pointer")(
        datagram_based,
        messages,
        num_messages,
        flags,
        timeout,
        cancellable,
        &raw mut child_error,
    );
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (retval < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            == (child_error != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GError)
                as ::core::ffi::c_int
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
            b"(retval < 0) == (child_error != NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if timeout == 0 as gint64
            || g_error_matches(
                child_error,
                g_io_error_quark(),
                G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
            ) == 0
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
            b"timeout == 0 || !g_error_matches (child_error, G_IO_ERROR, G_IO_ERROR_WOULD_BLOCK)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if timeout > 0 as gint64
            || g_error_matches(
                child_error,
                g_io_error_quark(),
                G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
            ) == 0
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
            b"timeout > 0 || !g_error_matches (child_error, G_IO_ERROR, G_IO_ERROR_TIMED_OUT)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if retval < 0 as ::core::ffi::c_int || retval as guint <= num_messages {
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
            b"retval < 0 || (guint) retval <= num_messages\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if !child_error.is_null() {
        g_propagate_error(error, child_error);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_send_messages(
    mut datagram_based: *mut GDatagramBased,
    mut messages: *mut GOutputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    let mut iface: *mut GDatagramBasedInterface =
        ::core::ptr::null_mut::<GDatagramBasedInterface>();
    let mut retval: gint = 0;
    let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = datagram_based as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_datagram_based_get_type();
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
            b"G_IS_DATAGRAM_BASED (datagram_based)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if num_messages == 0 as guint || !messages.is_null() {
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
            b"num_messages == 0 || messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    iface = g_type_interface_peek(
        (*(datagram_based as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_datagram_based_get_type(),
    ) as *mut GDatagramBasedInterface;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*iface).send_messages.is_some() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatagrambased.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->send_messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    retval = (*iface).send_messages.expect("non-null function pointer")(
        datagram_based,
        messages,
        num_messages,
        flags,
        timeout,
        cancellable,
        &raw mut child_error,
    );
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (retval < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            == (child_error != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GError)
                as ::core::ffi::c_int
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
            b"(retval < 0) == (child_error != NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if timeout == 0 as gint64
            || g_error_matches(
                child_error,
                g_io_error_quark(),
                G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
            ) == 0
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
            b"timeout == 0 || !g_error_matches (child_error, G_IO_ERROR, G_IO_ERROR_WOULD_BLOCK)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if timeout > 0 as gint64
            || g_error_matches(
                child_error,
                g_io_error_quark(),
                G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
            ) == 0
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
            b"timeout > 0 || !g_error_matches (child_error, G_IO_ERROR, G_IO_ERROR_TIMED_OUT)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if retval < 0 as ::core::ffi::c_int || retval as guint <= num_messages {
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
            b"retval < 0 || (guint) retval <= num_messages\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !(timeout < 0 as gint64 && num_messages > 0 as guint)
            || retval != 0 as ::core::ffi::c_int
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
            b"!(timeout < 0 && num_messages > 0) || retval != 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if !child_error.is_null() {
        g_propagate_error(error, child_error);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_create_source(
    mut datagram_based: *mut GDatagramBased,
    mut condition: GIOCondition,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut iface: *mut GDatagramBasedInterface =
        ::core::ptr::null_mut::<GDatagramBasedInterface>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = datagram_based as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_datagram_based_get_type();
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
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATAGRAM_BASED (datagram_based)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    iface = g_type_interface_peek(
        (*(datagram_based as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_datagram_based_get_type(),
    ) as *mut GDatagramBasedInterface;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*iface).create_source.is_some() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatagrambased.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            348 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->create_source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*iface).create_source.expect("non-null function pointer")(
        datagram_based,
        condition,
        cancellable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_condition_check(
    mut datagram_based: *mut GDatagramBased,
    mut condition: GIOCondition,
) -> GIOCondition {
    let mut iface: *mut GDatagramBasedInterface =
        ::core::ptr::null_mut::<GDatagramBasedInterface>();
    let mut out: GIOCondition = 0 as GIOCondition;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = datagram_based as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_datagram_based_get_type();
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
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATAGRAM_BASED (datagram_based)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GIOCondition;
    }
    iface = g_type_interface_peek(
        (*(datagram_based as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_datagram_based_get_type(),
    ) as *mut GDatagramBasedInterface;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*iface).condition_check.is_some() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatagrambased.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            409 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->condition_check != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    out = (*iface).condition_check.expect("non-null function pointer")(datagram_based, condition);
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if out as ::core::ffi::c_uint
            & (G_IO_OUT as ::core::ffi::c_int | G_IO_HUP as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_IO_OUT as ::core::ffi::c_int | G_IO_HUP as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(out & (G_IO_OUT | G_IO_HUP)) != (G_IO_OUT | G_IO_HUP)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return (out as ::core::ffi::c_uint
            & !(G_IO_OUT as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as GIOCondition;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if out as ::core::ffi::c_uint
            & (G_IO_IN as ::core::ffi::c_int | G_IO_HUP as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_IO_IN as ::core::ffi::c_int | G_IO_HUP as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(out & (G_IO_IN | G_IO_HUP)) != (G_IO_IN | G_IO_HUP)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return (out as ::core::ffi::c_uint
            & !(G_IO_IN as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as GIOCondition;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if out as ::core::ffi::c_uint
            & !(condition as ::core::ffi::c_uint
                | G_IO_ERR as ::core::ffi::c_int as ::core::ffi::c_uint
                | G_IO_HUP as ::core::ffi::c_int as ::core::ffi::c_uint)
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(out & ~(condition | G_IO_ERR | G_IO_HUP)) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return (out as ::core::ffi::c_uint
            & (condition as ::core::ffi::c_uint
                | G_IO_ERR as ::core::ffi::c_int as ::core::ffi::c_uint
                | G_IO_HUP as ::core::ffi::c_int as ::core::ffi::c_uint))
            as GIOCondition;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_datagram_based_condition_wait(
    mut datagram_based: *mut GDatagramBased,
    mut condition: GIOCondition,
    mut timeout: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GDatagramBasedInterface =
        ::core::ptr::null_mut::<GDatagramBasedInterface>();
    let mut out: gboolean = 0;
    let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = datagram_based as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_datagram_based_get_type();
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
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATAGRAM_BASED (datagram_based)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(datagram_based as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_datagram_based_get_type(),
    ) as *mut GDatagramBasedInterface;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if (*iface).condition_wait.is_some() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatagrambased.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            463 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->condition_wait != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    out = (*iface).condition_wait.expect("non-null function pointer")(
        datagram_based,
        condition,
        timeout,
        cancellable,
        &raw mut child_error,
    );
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if out
            == (child_error == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GError)
                as ::core::ffi::c_int
        {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"out == (child_error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !child_error.is_null() {
        g_propagate_error(error, child_error);
    }
    return out;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
