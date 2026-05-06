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
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_tls_password_flags_get_type() -> GType;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type _GTlsPasswordFlags = ::core::ffi::c_uint;
pub const G_TLS_PASSWORD_PKCS11_CONTEXT_SPECIFIC: _GTlsPasswordFlags = 64;
pub const G_TLS_PASSWORD_PKCS11_SECURITY_OFFICER: _GTlsPasswordFlags = 32;
pub const G_TLS_PASSWORD_PKCS11_USER: _GTlsPasswordFlags = 16;
pub const G_TLS_PASSWORD_FINAL_TRY: _GTlsPasswordFlags = 8;
pub const G_TLS_PASSWORD_MANY_TRIES: _GTlsPasswordFlags = 4;
pub const G_TLS_PASSWORD_RETRY: _GTlsPasswordFlags = 2;
pub const G_TLS_PASSWORD_NONE: _GTlsPasswordFlags = 0;
pub type GTlsPasswordFlags = _GTlsPasswordFlags;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsPassword {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsPasswordPrivate,
}
pub type GTlsPasswordPrivate = _GTlsPasswordPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsPasswordPrivate {
    pub value: *mut guchar,
    pub length: gsize,
    pub destroy: GDestroyNotify,
    pub flags: GTlsPasswordFlags,
    pub description: *mut gchar,
    pub warning: *mut gchar,
}
pub type GTlsPassword = _GTlsPassword;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsPasswordClass {
    pub parent_class: GObjectClass,
    pub get_value: Option<unsafe extern "C" fn(*mut GTlsPassword, *mut gsize) -> *const guchar>,
    pub set_value:
        Option<unsafe extern "C" fn(*mut GTlsPassword, *mut guchar, gssize, GDestroyNotify) -> ()>,
    pub get_default_warning: Option<unsafe extern "C" fn(*mut GTlsPassword) -> *const gchar>,
    pub padding: [gpointer; 4],
}
pub type GTlsPasswordClass = _GTlsPasswordClass;
pub const PROP_WARNING: C2RustUnnamed_0 = 3;
pub const PROP_DESCRIPTION: C2RustUnnamed_0 = 2;
pub const PROP_FLAGS: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
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
static mut safe_c2rust_g_tls_password_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_tls_password_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_tls_password_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTlsPassword_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GTlsPassword_private_offset);
    }
    safe_c2rust_g_tls_password_class_init(klass as *mut GTlsPasswordClass);
}
static mut safe_c2rust_GTlsPassword_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_tls_password_get_instance_private(
    mut self_0: *mut GTlsPassword,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GTlsPassword_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_tls_password_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTlsPassword\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTlsPasswordClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_password_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTlsPassword>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTlsPassword) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_password_init as unsafe extern "C" fn(*mut GTlsPassword) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GTlsPassword_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GTlsPasswordPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_tls_password_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_tls_password_init(mut password: *mut GTlsPassword) {
    (*password).priv_0 =
        safe_c2rust_g_tls_password_get_instance_private(password) as *mut GTlsPasswordPrivate;
}
unsafe extern "C" fn safe_c2rust_g_tls_password_real_get_value(
    mut password: *mut GTlsPassword,
    mut length: *mut gsize,
) -> *const guchar {
    if !length.is_null() {
        *length = (*(*password).priv_0).length;
    }
    return (*(*password).priv_0).value;
}
unsafe extern "C" fn safe_c2rust_g_tls_password_real_set_value(
    mut password: *mut GTlsPassword,
    mut value: *mut guchar,
    mut length: gssize,
    mut destroy: GDestroyNotify,
) {
    if (*(*password).priv_0).destroy.is_some() {
        (*(*password).priv_0)
            .destroy
            .expect("non-null function pointer")((*(*password).priv_0).value as gpointer);
    }
    (*(*password).priv_0).destroy = None;
    (*(*password).priv_0).value = ::core::ptr::null_mut::<guchar>();
    (*(*password).priv_0).length = 0 as gsize;
    if length < 0 as gssize {
        length = strlen(value as *mut gchar) as gssize;
    }
    (*(*password).priv_0).value = value;
    (*(*password).priv_0).length = length as gsize;
    (*(*password).priv_0).destroy = destroy;
}
unsafe extern "C" fn safe_c2rust_g_tls_password_real_get_default_warning(
    mut password: *mut GTlsPassword,
) -> *const gchar {
    let mut flags: GTlsPasswordFlags = G_TLS_PASSWORD_NONE;
    flags = safe_c2rust_g_tls_password_get_flags(password);
    if flags as ::core::ffi::c_uint
        & G_TLS_PASSWORD_FINAL_TRY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return glib_gettext(
            b"This is the last chance to enter the password correctly before your access is locked out.\0"
                as *const u8 as *const gchar,
        );
    }
    if flags as ::core::ffi::c_uint
        & G_TLS_PASSWORD_MANY_TRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return glib_gettext(
            b"Several passwords entered have been incorrect, and your access will be locked out after further failures.\0"
                as *const u8 as *const gchar,
        );
    }
    if flags as ::core::ffi::c_uint
        & G_TLS_PASSWORD_RETRY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return glib_gettext(b"The password entered is incorrect.\0" as *const u8 as *const gchar);
    }
    return ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn safe_c2rust_g_tls_password_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut password: *mut GTlsPassword = object as *mut ::core::ffi::c_void as *mut GTlsPassword;
    match prop_id {
        1 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_tls_password_get_flags(password) as guint,
            );
        }
        3 => {
            g_value_set_string(value, safe_c2rust_g_tls_password_get_warning(password));
        }
        2 => {
            g_value_set_string(value, safe_c2rust_g_tls_password_get_description(password));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlspassword.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                135 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_password_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut password: *mut GTlsPassword = object as *mut ::core::ffi::c_void as *mut GTlsPassword;
    match prop_id {
        1 => {
            safe_c2rust_g_tls_password_set_flags(
                password,
                g_value_get_flags(value) as GTlsPasswordFlags,
            );
        }
        3 => {
            safe_c2rust_g_tls_password_set_warning(password, g_value_get_string(value));
        }
        2 => {
            safe_c2rust_g_tls_password_set_description(password, g_value_get_string(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlspassword.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_password_finalize(mut object: *mut GObject) {
    let mut password: *mut GTlsPassword = object as *mut ::core::ffi::c_void as *mut GTlsPassword;
    safe_c2rust_g_tls_password_real_set_value(
        password,
        ::core::ptr::null_mut::<guchar>(),
        0 as gssize,
        None,
    );
    g_free((*(*password).priv_0).warning as gpointer);
    g_free((*(*password).priv_0).description as gpointer);
    (*(safe_c2rust_g_tls_password_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_tls_password_class_init(mut klass: *mut GTlsPasswordClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*klass).get_value = Some(
        safe_c2rust_g_tls_password_real_get_value
            as unsafe extern "C" fn(*mut GTlsPassword, *mut gsize) -> *const guchar,
    )
        as Option<unsafe extern "C" fn(*mut GTlsPassword, *mut gsize) -> *const guchar>;
    (*klass).set_value = Some(
        safe_c2rust_g_tls_password_real_set_value
            as unsafe extern "C" fn(*mut GTlsPassword, *mut guchar, gssize, GDestroyNotify) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GTlsPassword, *mut guchar, gssize, GDestroyNotify) -> (),
        >;
    (*klass).get_default_warning = Some(
        safe_c2rust_g_tls_password_real_get_default_warning
            as unsafe extern "C" fn(*mut GTlsPassword) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GTlsPassword) -> *const gchar>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_tls_password_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_tls_password_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_tls_password_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_password_flags_get_type(),
            G_TLS_PASSWORD_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DESCRIPTION as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"description\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_WARNING as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"warning\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_new(
    mut flags: GTlsPasswordFlags,
    mut description: *const gchar,
) -> *mut GTlsPassword {
    return g_object_new(
        safe_c2rust_g_tls_password_get_type(),
        b"flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"description\0" as *const u8 as *const ::core::ffi::c_char,
        description,
        NULL_0,
    ) as *mut GTlsPassword;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_get_value(
    mut password: *mut GTlsPassword,
    mut length: *mut gsize,
) -> *const guchar {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<guchar>();
    }
    return (*((*(password as *mut GTypeInstance)).g_class as *mut GTlsPasswordClass))
        .get_value
        .expect("non-null function pointer")(password, length);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_set_value(
    mut password: *mut GTlsPassword,
    mut value: *const guchar,
    mut length: gssize,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if length < 0 as gssize {
        let mut length_unsigned: gsize = strlen(value as *mut gchar) as gsize;
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if length_unsigned <= 9223372036854775807 as ::core::ffi::c_long as gsize {
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
                b"length_unsigned <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        length = length_unsigned as gssize;
    }
    safe_c2rust_g_tls_password_set_value_full(
        password,
        g_memdup2(value as gconstpointer, length as gsize) as *mut guchar,
        length,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_set_value_full(
    mut password: *mut GTlsPassword,
    mut value: *mut guchar,
    mut length: gssize,
    mut destroy: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(password as *mut GTypeInstance)).g_class as *mut GTlsPasswordClass))
        .set_value
        .expect("non-null function pointer")(password, value, length, destroy);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_get_flags(
    mut password: *mut GTlsPassword,
) -> GTlsPasswordFlags {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_PASSWORD_NONE;
    }
    return (*(*password).priv_0).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_set_flags(
    mut password: *mut GTlsPassword,
    mut flags: GTlsPasswordFlags,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*password).priv_0).flags = flags;
    g_object_notify(
        password as *mut ::core::ffi::c_void as *mut GObject,
        b"flags\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_get_description(
    mut password: *mut GTlsPassword,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*password).priv_0).description;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_set_description(
    mut password: *mut GTlsPassword,
    mut description: *const gchar,
) {
    let mut copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    copy = safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
    g_free((*(*password).priv_0).description as gpointer);
    (*(*password).priv_0).description = copy;
    g_object_notify(
        password as *mut ::core::ffi::c_void as *mut GObject,
        b"description\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_get_warning(
    mut password: *mut GTlsPassword,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if (*(*password).priv_0).warning.is_null() {
        return (*((*(password as *mut GTypeInstance)).g_class as *mut GTlsPasswordClass))
            .get_default_warning
            .expect("non-null function pointer")(password);
    }
    return (*(*password).priv_0).warning;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_set_warning(
    mut password: *mut GTlsPassword,
    mut warning: *const gchar,
) {
    let mut copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    copy = safe_c2rust_g_strdup_inline(warning as *const ::core::ffi::c_char) as *mut gchar;
    g_free((*(*password).priv_0).warning as gpointer);
    (*(*password).priv_0).warning = copy;
    g_object_notify(
        password as *mut ::core::ffi::c_void as *mut GObject,
        b"warning\0" as *const u8 as *const gchar,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
