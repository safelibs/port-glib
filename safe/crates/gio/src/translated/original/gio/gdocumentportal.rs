extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GCancellablePrivate;
    pub type _GUnixFDListPrivate;
    pub type _GDBusConnection;
    pub type _GXdpDocuments;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_filename_from_uri(
        uri: *const gchar,
        hostname: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_object_unref(object: gpointer);
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_unix_fd_list_new() -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn g_unix_fd_list_get_length(list: *mut GUnixFDList) -> gint;
    fn gxdp_documents_call_get_mount_point_sync(
        proxy: *mut GXdpDocuments,
        out_path: *mut *mut gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_documents_call_add_full_sync(
        proxy: *mut GXdpDocuments,
        arg_o_path_fds: *mut GVariant,
        arg_flags: guint,
        arg_app_id: *const gchar,
        arg_permissions: *const *const gchar,
        fd_list: *mut GUnixFDList,
        out_doc_ids: *mut *mut *mut gchar,
        out_extra_out: *mut *mut GVariant,
        out_fd_list: *mut *mut GUnixFDList,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_documents_proxy_new_sync(
        connection: *mut GDBusConnection,
        flags: GDBusProxyFlags,
        name: *const gchar,
        object_path: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GXdpDocuments;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
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
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
pub type GType = gsize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
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
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusConnection = _GDBusConnection;
pub type GXdpDocuments = _GXdpDocuments;
pub const XDP_ADD_FLAGS_AS_NEEDED_BY_APP: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const XDP_ADD_FLAGS_FLAGS_ALL: C2RustUnnamed_1 = 7;
pub const XDP_ADD_FLAGS_PERSISTENT: C2RustUnnamed_1 = 2;
pub const XDP_ADD_FLAGS_REUSE_EXISTING: C2RustUnnamed_1 = 1;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
unsafe extern "C" fn safe_c2rust_get_document_portal(
    mut documents: *mut *mut GXdpDocuments,
    mut documents_mountpoint: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    *documents = ::core::ptr::null_mut::<GXdpDocuments>();
    *documents_mountpoint = ::core::ptr::null_mut::<::core::ffi::c_char>();
    connection = g_bus_get_sync(
        G_BUS_TYPE_SESSION,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    );
    if connection.is_null() {
        g_prefix_error(
            error,
            b"Cannot connect to session bus when initializing document portal: \0" as *const u8
                as *const gchar,
        );
    } else {
        *documents = gxdp_documents_proxy_new_sync(
            connection,
            (G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int
                | G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS as ::core::ffi::c_int)
                as GDBusProxyFlags,
            b"org.freedesktop.portal.Documents\0" as *const u8 as *const gchar,
            b"/org/freedesktop/portal/documents\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        );
        if (*documents).is_null() {
            g_prefix_error(
                error,
                b"Cannot create document portal proxy: \0" as *const u8 as *const gchar,
            );
        } else if gxdp_documents_call_get_mount_point_sync(
            *documents,
            documents_mountpoint as *mut *mut gchar,
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        ) == 0
        {
            let mut _pp: *mut *mut GXdpDocuments = documents as *mut *mut GXdpDocuments;
            let mut _ptr: *mut GXdpDocuments = *_pp;
            *_pp = ::core::ptr::null_mut::<GXdpDocuments>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
            g_prefix_error(
                error,
                b"Cannot get document portal mount point: \0" as *const u8 as *const gchar,
            );
        }
    }
    let mut _pp_0: *mut *mut GDBusConnection = &raw mut connection;
    let mut _ptr_0: *mut GDBusConnection = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDBusConnection>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    return (*documents != NULL_0 as *mut GXdpDocuments) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_document_portal_add_documents(
    mut uris: *mut GList,
    mut app_id: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut documents: *mut GXdpDocuments = ::core::ptr::null_mut::<GXdpDocuments>();
    let mut documents_mountpoint: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: ::core::ffi::c_int = 0;
    let mut ruris: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut as_is: *mut gboolean = ::core::ptr::null_mut::<gboolean>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: gsize = 0;
    let mut j: gsize = 0;
    let mut permissions: [*const ::core::ffi::c_char; 3] = [
        b"read\0" as *const u8 as *const ::core::ffi::c_char,
        b"write\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut doc_ids: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut extra_out: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if safe_c2rust_get_document_portal(&raw mut documents, &raw mut documents_mountpoint, error)
        == 0
    {
        return ::core::ptr::null_mut::<GList>();
    }
    length = g_list_length(uris) as ::core::ffi::c_int;
    as_is = ({
        let mut __n: gsize = length as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gboolean>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut gboolean;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"ah\0" as *const u8 as *const gchar),
    );
    fd_list = g_unix_fd_list_new();
    l = uris;
    i = 0 as gsize;
    while !l.is_null() {
        let mut uri: *const ::core::ffi::c_char = (*l).data as *const ::core::ffi::c_char;
        let mut idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        path = g_filename_from_uri(
            uri as *const gchar,
            ::core::ptr::null_mut::<*mut gchar>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        if !path.is_null() {
            let mut fd: ::core::ffi::c_int = 0;
            fd = open(path, O_CLOEXEC | O_RDWR);
            if fd == -(1 as ::core::ffi::c_int)
                && (*__errno_location() == EACCES || *__errno_location() == EISDIR)
            {
                fd = open(path, O_CLOEXEC | O_RDONLY);
                permissions[1 as ::core::ffi::c_int as usize] =
                    ::core::ptr::null::<::core::ffi::c_char>();
            }
            if fd >= 0 as ::core::ffi::c_int {
                idx = g_unix_fd_list_append(
                    fd_list,
                    fd as gint,
                    ::core::ptr::null_mut::<*mut GError>(),
                ) as ::core::ffi::c_int;
                close(fd);
            }
        }
        g_free(path as gpointer);
        if idx != -(1 as ::core::ffi::c_int) {
            g_variant_builder_add(&raw mut builder, b"h\0" as *const u8 as *const gchar, idx);
        } else {
            *as_is.offset(i as isize) = TRUE as gboolean;
        }
        l = (*l).next;
        i = i.wrapping_add(1);
    }
    if g_unix_fd_list_get_length(fd_list) > 0 as ::core::ffi::c_int {
        if !(gxdp_documents_call_add_full_sync(
            documents,
            g_variant_builder_end(&raw mut builder),
            XDP_ADD_FLAGS_AS_NEEDED_BY_APP as ::core::ffi::c_int as guint,
            app_id as *const gchar,
            &raw mut permissions as *mut *const ::core::ffi::c_char as *const *const gchar,
            fd_list,
            &raw mut doc_ids,
            &raw mut extra_out,
            ::core::ptr::null_mut::<*mut GUnixFDList>(),
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        ) == 0)
        {
            l = uris;
            i = 0 as gsize;
            j = 0 as gsize;
            while !l.is_null() {
                let mut uri_0: *const ::core::ffi::c_char = (*l).data as *const ::core::ffi::c_char;
                let mut ruri: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if *as_is.offset(i as isize) != 0 {
                    ruri = safe_c2rust_g_strdup_inline(uri_0);
                } else if strcmp(
                    *doc_ids.offset(j as isize),
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    ruri = safe_c2rust_g_strdup_inline(uri_0);
                    j = j.wrapping_add(1);
                } else {
                    let mut basename: *mut ::core::ffi::c_char = g_path_get_basename(uri_0.offset(
                        strlen(b"file:\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                    ))
                        as *mut ::core::ffi::c_char;
                    let mut doc_path: *mut ::core::ffi::c_char = g_build_filename(
                        documents_mountpoint,
                        *doc_ids.offset(j as isize),
                        basename,
                        NULL_0,
                    )
                        as *mut ::core::ffi::c_char;
                    ruri = g_strconcat(b"file:\0" as *const u8 as *const gchar, doc_path, NULL_0)
                        as *mut ::core::ffi::c_char;
                    g_free(basename as gpointer);
                    g_free(doc_path as gpointer);
                    j = j.wrapping_add(1);
                }
                ruris = g_list_prepend(ruris, ruri as gpointer);
                l = (*l).next;
                i = i.wrapping_add(1);
            }
            ruris = g_list_reverse(ruris);
        }
    } else {
        ruris = g_list_copy_deep(
            uris,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const gchar) -> *mut gchar>,
                GCopyFunc,
            >(Some(
                g_strdup as unsafe extern "C" fn(*const gchar) -> *mut gchar,
            )),
            NULL_0,
        );
        g_variant_builder_clear(&raw mut builder);
    }
    let mut _pp: *mut *mut GXdpDocuments = &raw mut documents;
    let mut _ptr: *mut GXdpDocuments = *_pp;
    *_pp = ::core::ptr::null_mut::<GXdpDocuments>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut ::core::ffi::c_char = &raw mut documents_mountpoint;
    let mut _ptr_0: *mut ::core::ffi::c_char = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GUnixFDList = &raw mut fd_list;
    let mut _ptr_1: *mut GUnixFDList = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GUnixFDList>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    let mut _pp_2: *mut *mut GVariant = &raw mut extra_out;
    let mut _ptr_2: *mut GVariant = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GVariant>();
    if !_ptr_2.is_null() {
        g_variant_unref(_ptr_2 as *mut GVariant);
    }
    let mut _pp_3: *mut *mut *mut ::core::ffi::c_char = &raw mut doc_ids;
    let mut _ptr_3: *mut *mut ::core::ffi::c_char = *_pp_3;
    *_pp_3 = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !_ptr_3.is_null() {
        g_strfreev(_ptr_3 as *mut *mut gchar);
    }
    g_free(as_is as gpointer);
    return ruris;
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
