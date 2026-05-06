extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GInputStreamPrivate;
    pub type _GFile;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_get_environ() -> *mut *mut gchar;
    fn g_get_current_dir() -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_dup_bytestring(value: *mut GVariant, length: *mut gsize) -> *mut gchar;
    fn g_variant_dup_bytestring_array(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_loop(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_dict_new(from_asv: *mut GVariant) -> *mut GVariantDict;
    fn g_variant_dict_unref(dict: *mut GVariantDict);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_printerr(format: *const gchar, ...);
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_variant(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        type_0: *const GVariantType,
        default_value: *mut GVariant,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_dup_variant(value: *const GValue) -> *mut GVariant;
    fn g_file_new_for_commandline_arg(arg: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_new_for_commandline_arg_and_cwd(arg: *const gchar, cwd: *const gchar) -> *mut GFile;
    fn g_unix_input_stream_new(fd: gint, close_fd: gboolean) -> *mut GInputStream;
}
pub type __builtin_va_list = [__va_list_tag; 1];
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
pub type va_list = __builtin_va_list;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
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
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub struct _GApplicationCommandLine {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationCommandLinePrivate,
}
pub type GApplicationCommandLinePrivate = _GApplicationCommandLinePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationCommandLinePrivate {
    pub platform_data: *mut GVariant,
    pub arguments: *mut GVariant,
    pub options: *mut GVariant,
    pub options_dict: *mut GVariantDict,
    pub cwd: *mut gchar,
    pub environ: *mut *mut gchar,
    pub exit_status: gint,
    pub done: gboolean,
}
pub type GApplicationCommandLine = _GApplicationCommandLine;
pub type GFile = _GFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationCommandLineClass {
    pub parent_class: GObjectClass,
    pub print_literal:
        Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>,
    pub printerr_literal:
        Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>,
    pub get_stdin: Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream>,
    pub done: Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> ()>,
    pub padding: [gpointer; 10],
}
pub type GApplicationCommandLineClass = _GApplicationCommandLineClass;
pub const PROP_IS_REMOTE: C2RustUnnamed_2 = 4;
pub const PROP_PLATFORM_DATA: C2RustUnnamed_2 = 3;
pub const PROP_OPTIONS: C2RustUnnamed_2 = 2;
pub const PROP_ARGUMENTS: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING_ARRAY: *const GVariantType =
    b"aay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_VARDICT: *const GVariantType =
    b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_application_command_line_get_instance_private(
    mut self_0: *mut GApplicationCommandLine,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GApplicationCommandLine_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GApplicationCommandLine_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_application_command_line_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_application_command_line_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GApplicationCommandLine_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GApplicationCommandLine_private_offset,
        );
    }
    safe_c2rust_g_application_command_line_class_init(klass as *mut GApplicationCommandLineClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_application_command_line_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_application_command_line_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GApplicationCommandLine\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GApplicationCommandLineClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_command_line_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GApplicationCommandLine>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_command_line_init
                    as unsafe extern "C" fn(*mut GApplicationCommandLine) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GApplicationCommandLine_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GApplicationCommandLinePrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_application_command_line_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_grok_platform_data(mut cmdline: *mut GApplicationCommandLine) {
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_variant_iter_init(&raw mut iter, (*(*cmdline).priv_0).platform_data);
    while g_variant_iter_loop(
        &raw mut iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        &raw mut value,
    ) != 0
    {
        if strcmp(
            key as *const ::core::ffi::c_char,
            b"cwd\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_variant_is_of_type(value, G_VARIANT_TYPE_BYTESTRING) != 0
        {
            if (*(*cmdline).priv_0).cwd.is_null() {
                (*(*cmdline).priv_0).cwd =
                    g_variant_dup_bytestring(value, ::core::ptr::null_mut::<gsize>());
            }
        } else if strcmp(
            key as *const ::core::ffi::c_char,
            b"environ\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_variant_is_of_type(value, G_VARIANT_TYPE_BYTESTRING_ARRAY) != 0
        {
            if (*(*cmdline).priv_0).environ.is_null() {
                (*(*cmdline).priv_0).environ =
                    g_variant_dup_bytestring_array(value, ::core::ptr::null_mut::<gsize>());
            }
        } else if strcmp(
            key as *const ::core::ffi::c_char,
            b"options\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_variant_is_of_type(value, G_VARIANT_TYPE_VARDICT) != 0
        {
            if (*(*cmdline).priv_0).options.is_null() {
                (*(*cmdline).priv_0).options = g_variant_ref(value);
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_real_print_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    g_print(b"%s\0" as *const u8 as *const gchar, message);
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_real_printerr_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    g_printerr(b"%s\0" as *const u8 as *const gchar, message);
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_real_get_stdin(
    mut cmdline: *mut GApplicationCommandLine,
) -> *mut GInputStream {
    return g_unix_input_stream_new(0 as gint, FALSE);
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_real_done(
    mut cmdline: *mut GApplicationCommandLine,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut cmdline: *mut GApplicationCommandLine =
        object as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
    match prop_id {
        1 => {
            g_value_set_variant(value, (*(*cmdline).priv_0).arguments);
        }
        3 => {
            g_value_set_variant(value, (*(*cmdline).priv_0).platform_data);
        }
        4 => {
            g_value_set_boolean(
                value,
                ((*(*(cmdline as *mut GTypeInstance)).g_class).g_type
                    != safe_c2rust_g_application_command_line_get_type())
                    as ::core::ffi::c_int,
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationcommandline.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                333 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut cmdline: *mut GApplicationCommandLine =
        object as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
    match prop_id {
        1 => {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if (*(*cmdline).priv_0).arguments.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationcommandline.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"cmdline->priv->arguments == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*(*cmdline).priv_0).arguments = g_value_dup_variant(value);
        }
        2 => {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if (*(*cmdline).priv_0).options.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationcommandline.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    353 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"cmdline->priv->options == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*(*cmdline).priv_0).options = g_value_dup_variant(value);
        }
        3 => {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if (*(*cmdline).priv_0).platform_data.is_null() {
                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_12
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationcommandline.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    358 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"cmdline->priv->platform_data == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*(*cmdline).priv_0).platform_data = g_value_dup_variant(value);
            if !(*(*cmdline).priv_0).platform_data.is_null() {
                safe_c2rust_grok_platform_data(cmdline);
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationcommandline.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                365 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_dispose(mut object: *mut GObject) {
    let mut cmdline: *mut GApplicationCommandLine =
        object as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
    safe_c2rust_g_application_command_line_done(cmdline);
    (*(safe_c2rust_g_application_command_line_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_finalize(mut object: *mut GObject) {
    let mut cmdline: *mut GApplicationCommandLine =
        object as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
    if !(*(*cmdline).priv_0).options_dict.is_null() {
        g_variant_dict_unref((*(*cmdline).priv_0).options_dict);
    }
    if !(*(*cmdline).priv_0).options.is_null() {
        g_variant_unref((*(*cmdline).priv_0).options);
    }
    if !(*(*cmdline).priv_0).platform_data.is_null() {
        g_variant_unref((*(*cmdline).priv_0).platform_data);
    }
    if !(*(*cmdline).priv_0).arguments.is_null() {
        g_variant_unref((*(*cmdline).priv_0).arguments);
    }
    g_free((*(*cmdline).priv_0).cwd as gpointer);
    g_strfreev((*(*cmdline).priv_0).environ);
    (*(safe_c2rust_g_application_command_line_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_init(
    mut cmdline: *mut GApplicationCommandLine,
) {
    (*cmdline).priv_0 = safe_c2rust_g_application_command_line_get_instance_private(cmdline)
        as *mut GApplicationCommandLinePrivate;
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_constructed(mut object: *mut GObject) {
    let mut cmdline: *mut GApplicationCommandLine =
        object as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
    if (*(*(cmdline as *mut GTypeInstance)).g_class).g_type
        != safe_c2rust_g_application_command_line_get_type()
    {
        return;
    }
    if (*(*cmdline).priv_0).cwd.is_null() {
        (*(*cmdline).priv_0).cwd = g_get_current_dir();
    }
    if (*(*cmdline).priv_0).environ.is_null() {
        (*(*cmdline).priv_0).environ = g_get_environ();
    }
}
unsafe extern "C" fn safe_c2rust_g_application_command_line_class_init(
    mut class: *mut GApplicationCommandLineClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_application_command_line_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_application_command_line_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_application_command_line_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).dispose = Some(
        safe_c2rust_g_application_command_line_dispose as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).constructed = Some(
        safe_c2rust_g_application_command_line_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).printerr_literal = Some(
        safe_c2rust_g_application_command_line_real_printerr_literal
            as unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>;
    (*class).print_literal = Some(
        safe_c2rust_g_application_command_line_real_print_literal
            as unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>;
    (*class).get_stdin = Some(
        safe_c2rust_g_application_command_line_real_get_stdin
            as unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream,
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream>;
    (*class).done = Some(
        safe_c2rust_g_application_command_line_real_done
            as unsafe extern "C" fn(*mut GApplicationCommandLine) -> (),
    ) as Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_ARGUMENTS as ::core::ffi::c_int as guint,
        g_param_spec_variant(
            b"arguments\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_VARIANT_TYPE_BYTESTRING_ARRAY,
            ::core::ptr::null_mut::<GVariant>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_OPTIONS as ::core::ffi::c_int as guint,
        g_param_spec_variant(
            b"options\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_VARIANT_TYPE_VARDICT,
            ::core::ptr::null_mut::<GVariant>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PLATFORM_DATA as ::core::ffi::c_int as guint,
        g_param_spec_variant(
            b"platform-data\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
            ::core::ptr::null_mut::<GVariant>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_REMOTE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-remote\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_arguments(
    mut cmdline: *mut GApplicationCommandLine,
    mut argc: *mut ::core::ffi::c_int,
) -> *mut *mut gchar {
    let mut argv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    argv = g_variant_dup_bytestring_array((*(*cmdline).priv_0).arguments, &raw mut len);
    if !argc.is_null() {
        *argc = len as ::core::ffi::c_int;
    }
    return argv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_options_dict(
    mut cmdline: *mut GApplicationCommandLine,
) -> *mut GVariantDict {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantDict>();
    }
    if (*(*cmdline).priv_0).options_dict.is_null() {
        (*(*cmdline).priv_0).options_dict = g_variant_dict_new((*(*cmdline).priv_0).options);
    }
    return (*(*cmdline).priv_0).options_dict;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_stdin(
    mut cmdline: *mut GApplicationCommandLine,
) -> *mut GInputStream {
    return (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .get_stdin
        .expect("non-null function pointer")(cmdline);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_cwd(
    mut cmdline: *mut GApplicationCommandLine,
) -> *const gchar {
    return (*(*cmdline).priv_0).cwd;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_environ(
    mut cmdline: *mut GApplicationCommandLine,
) -> *const *const gchar {
    return (*(*cmdline).priv_0).environ as *mut *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_getenv(
    mut cmdline: *mut GApplicationCommandLine,
    mut name: *const gchar,
) -> *const gchar {
    let mut length: gint = strlen(name as *const ::core::ffi::c_char) as gint;
    let mut i: gint = 0;
    if !(*(*cmdline).priv_0).environ.is_null() {
        i = 0 as ::core::ffi::c_int as gint;
        while !(*(*(*cmdline).priv_0).environ.offset(i as isize)).is_null() {
            if strncmp(
                *(*(*cmdline).priv_0).environ.offset(i as isize),
                name as *const ::core::ffi::c_char,
                length as size_t,
            ) == 0 as ::core::ffi::c_int
                && *(*(*(*cmdline).priv_0).environ.offset(i as isize)).offset(length as isize)
                    as ::core::ffi::c_int
                    == '=' as i32
            {
                return (*(*(*cmdline).priv_0).environ.offset(i as isize))
                    .offset(length as isize)
                    .offset(1 as ::core::ffi::c_int as isize);
            }
            i += 1;
        }
    }
    return ::core::ptr::null::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_is_remote(
    mut cmdline: *mut GApplicationCommandLine,
) -> gboolean {
    return ((*(*(cmdline as *mut GTypeInstance)).g_class).g_type
        != safe_c2rust_g_application_command_line_get_type()) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_print_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !message.is_null() {
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
            b"message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .print_literal
        .expect("non-null function pointer")(cmdline, message);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_printerr_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !message.is_null() {
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
            b"message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .printerr_literal
        .expect("non-null function pointer")(cmdline, message);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_print(
    mut cmdline: *mut GApplicationCommandLine,
    mut format: *const gchar,
    mut args: ...
) {
    let mut message: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    ap = args.clone();
    message = g_strdup_vprintf(format, ap);
    (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .print_literal
        .expect("non-null function pointer")(cmdline, message);
    g_free(message as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_printerr(
    mut cmdline: *mut GApplicationCommandLine,
    mut format: *const gchar,
    mut args: ...
) {
    let mut message: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    ap = args.clone();
    message = g_strdup_vprintf(format, ap);
    (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .printerr_literal
        .expect("non-null function pointer")(cmdline, message);
    g_free(message as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_set_exit_status(
    mut cmdline: *mut GApplicationCommandLine,
    mut exit_status: ::core::ffi::c_int,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*cmdline).priv_0).done != 0 {
        return;
    }
    (*(*cmdline).priv_0).exit_status = exit_status as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_exit_status(
    mut cmdline: *mut GApplicationCommandLine,
) -> ::core::ffi::c_int {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return (*(*cmdline).priv_0).exit_status as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_get_platform_data(
    mut cmdline: *mut GApplicationCommandLine,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if !(*(*cmdline).priv_0).platform_data.is_null() {
        return g_variant_ref((*(*cmdline).priv_0).platform_data);
    } else {
        return ::core::ptr::null_mut::<GVariant>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_create_file_for_arg(
    mut cmdline: *mut GApplicationCommandLine,
    mut arg: *const gchar,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !arg.is_null() {
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
            b"arg != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if !(*(*cmdline).priv_0).cwd.is_null() {
        return g_file_new_for_commandline_arg_and_cwd(arg, (*(*cmdline).priv_0).cwd);
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"Requested creation of GFile for commandline invocation that did not send cwd. Using cwd of local process to resolve relative path names.\0"
            as *const u8 as *const gchar,
    );
    return g_file_new_for_commandline_arg(arg as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_command_line_done(
    mut cmdline: *mut GApplicationCommandLine,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cmdline as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_command_line_get_type();
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
            b"G_IS_APPLICATION_COMMAND_LINE (cmdline)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*cmdline).priv_0).done != 0 {
        return;
    }
    (*((*(cmdline as *mut GTypeInstance)).g_class as *mut GApplicationCommandLineClass))
        .done
        .expect("non-null function pointer")(cmdline);
    (*(*cmdline).priv_0).done = TRUE as gboolean;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
