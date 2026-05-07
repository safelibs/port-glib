#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::abi::{GIArgInfo, GIArgument, GIBaseInfoStack, GITypeInfo, GTypeClass, GTypeInstance};
use crate::ffi::{gboolean, guint, GQuark, GType};
use core::ffi::{c_char, c_int, c_void};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::sync::{Mutex, OnceLock};

pub type Ptr = *mut c_void;
pub type ConstChar = *const c_char;
pub type CharStrv = *mut *mut c_char;
pub type ConstCharStrv = *const *const c_char;
pub type GErrorOut = *mut Ptr;

const DEFAULT_TYPELIB_DIR: &str = "/usr/local/lib/x86_64-linux-gnu/girepository-1.0";

const GI_TRANSFER_NOTHING: c_int = 0;
const GI_TRANSFER_EVERYTHING: c_int = 2;
const GI_DIRECTION_IN: c_int = 0;
const GI_DIRECTION_OUT: c_int = 1;
const GI_SCOPE_TYPE_INVALID: c_int = 0;

const GI_TYPE_TAG_VOID: c_int = 0;
const GI_TYPE_TAG_INT8: c_int = 2;
const GI_TYPE_TAG_UINT8: c_int = 3;
const GI_TYPE_TAG_UINT32: c_int = 7;
const GI_TYPE_TAG_UTF8: c_int = 13;
const GI_TYPE_TAG_ARRAY: c_int = 15;
const GI_TYPE_TAG_INTERFACE: c_int = 16;
const GI_ARRAY_TYPE_C: c_int = 0;

const G_FILE_ERROR_NOENT: c_int = 4;
const G_SIGNAL_NOTIFY_FLAGS: c_int = 1 | 8 | 16 | 32 | 64;

const GLIB: &[u8] = b"GLib\0";
const GOBJECT: &[u8] = b"GObject\0";
const GIO: &[u8] = b"Gio\0";
const GIREPOSITORY: &[u8] = b"GIRepository\0";

#[repr(C)]
struct GTypeQuery {
    type_: GType,
    type_name: *const c_char,
    class_size: guint,
    instance_size: guint,
}

unsafe extern "C" {
    fn g_object_get_type() -> GType;
    fn g_object_new_with_properties(
        object_type: GType,
        n_properties: guint,
        names: *const *const c_char,
        values: *const c_void,
    ) -> Ptr;
    fn g_type_class_ref(type_: GType) -> *mut GTypeClass;
    fn g_type_query(type_: GType, query: *mut GTypeQuery);
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const c_char,
        class_size: guint,
        class_init: Ptr,
        instance_size: guint,
        instance_init: Ptr,
        flags: guint,
    ) -> GType;

    fn g_malloc(n_bytes: usize) -> Ptr;
    fn g_strdup(str: ConstChar) -> *mut c_char;
    fn g_file_error_quark() -> GQuark;
    fn g_set_error_literal(error: GErrorOut, domain: GQuark, code: c_int, message: ConstChar);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GiType {
    Arg,
    Base,
    Callable,
    Callback,
    Constant,
    Enum,
    Field,
    Flags,
    Function,
    Interface,
    Object,
    Property,
    RegisteredType,
    Repository,
    Signal,
    Struct,
    Type,
    Typelib,
    Union,
    Unresolved,
    Value,
    VFunc,
}

struct TypeRegistry {
    arg: GType,
    base: GType,
    callable: GType,
    callback: GType,
    constant: GType,
    enum_: GType,
    field: GType,
    flags: GType,
    function: GType,
    interface: GType,
    object: GType,
    property: GType,
    registered_type: GType,
    repository: GType,
    signal: GType,
    struct_: GType,
    type_: GType,
    typelib: GType,
    union_: GType,
    unresolved: GType,
    value: GType,
    vfunc: GType,
}

static TYPE_REGISTRY: OnceLock<TypeRegistry> = OnceLock::new();

unsafe impl Sync for TypeRegistry {}
unsafe impl Send for TypeRegistry {}

fn type_registry() -> &'static TypeRegistry {
    TYPE_REGISTRY.get_or_init(|| unsafe {
        let gobject = g_object_get_type();
        let class_size = type_query(gobject).class_size;
        let repository_instance_size = type_query(gobject).instance_size;

        let repository = register_type(
            gobject,
            b"GIRepository\0",
            class_size,
            repository_instance_size,
        );
        let base = register_type(
            gobject,
            b"GIBaseInfo\0",
            class_size,
            mem::size_of::<GIBaseInfoStack>() as guint,
        );
        let registered_type = register_type(
            base,
            b"GIRegisteredTypeInfo\0",
            class_size,
            mem::size_of::<GIBaseInfoStack>() as guint,
        );
        let callable = register_type(
            base,
            b"GICallableInfo\0",
            class_size,
            mem::size_of::<GIBaseInfoStack>() as guint,
        );

        TypeRegistry {
            arg: register_type(
                base,
                b"GIArgInfo\0",
                class_size,
                mem::size_of::<GIArgInfo>() as guint,
            ),
            base,
            callable,
            callback: register_type(
                callable,
                b"GICallbackInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            constant: register_type(
                base,
                b"GIConstantInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            enum_: register_type(
                registered_type,
                b"GIEnumInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            field: register_type(
                base,
                b"GIFieldInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            flags: register_type(
                registered_type,
                b"GIFlagsInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            function: register_type(
                callable,
                b"GIFunctionInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            interface: register_type(
                registered_type,
                b"GIInterfaceInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            object: register_type(
                registered_type,
                b"GIObjectInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            property: register_type(
                base,
                b"GIPropertyInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            registered_type,
            repository,
            signal: register_type(
                callable,
                b"GISignalInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            struct_: register_type(
                registered_type,
                b"GIStructInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            type_: register_type(
                base,
                b"GITypeInfo\0",
                class_size,
                mem::size_of::<GITypeInfo>() as guint,
            ),
            typelib: register_type(
                base,
                b"GITypelib\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            union_: register_type(
                registered_type,
                b"GIUnionInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            unresolved: register_type(
                base,
                b"GIUnresolvedInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            value: register_type(
                base,
                b"GIValueInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
            vfunc: register_type(
                callable,
                b"GIVFuncInfo\0",
                class_size,
                mem::size_of::<GIBaseInfoStack>() as guint,
            ),
        }
    })
}

unsafe fn type_query(type_: GType) -> GTypeQuery {
    let mut query = MaybeUninit::<GTypeQuery>::zeroed();
    unsafe { g_type_query(type_, query.as_mut_ptr()) };
    unsafe { query.assume_init() }
}

unsafe fn register_type(
    parent: GType,
    name: &'static [u8],
    class_size: guint,
    instance_size: guint,
) -> GType {
    unsafe {
        g_type_register_static_simple(
            parent,
            c(name),
            class_size,
            ptr::null_mut(),
            instance_size,
            ptr::null_mut(),
            0,
        )
    }
}

pub fn gtype_for_getter(name: &str) -> GType {
    let registry = type_registry();
    match name {
        "gi_arg_info_get_type" => registry.arg,
        "gi_base_info_get_type" => registry.base,
        "gi_callable_info_get_type" => registry.callable,
        "gi_callback_info_get_type" => registry.callback,
        "gi_constant_info_get_type" => registry.constant,
        "gi_enum_info_get_type" => registry.enum_,
        "gi_field_info_get_type" => registry.field,
        "gi_flags_info_get_type" => registry.flags,
        "gi_function_info_get_type" => registry.function,
        "gi_interface_info_get_type" => registry.interface,
        "gi_object_info_get_type" => registry.object,
        "gi_property_info_get_type" => registry.property,
        "gi_registered_type_info_get_type" => registry.registered_type,
        "gi_repository_get_type" => registry.repository,
        "gi_signal_info_get_type" => registry.signal,
        "gi_struct_info_get_type" => registry.struct_,
        "gi_type_info_get_type" => registry.type_,
        "gi_typelib_get_type" => registry.typelib,
        "gi_union_info_get_type" => registry.union_,
        "gi_unresolved_info_get_type" => registry.unresolved,
        "gi_value_info_get_type" => registry.value,
        "gi_vfunc_info_get_type" => registry.vfunc,
        _ => 0,
    }
}

fn gtype_for_info_type(type_: GiType) -> GType {
    let registry = type_registry();
    match type_ {
        GiType::Arg => registry.arg,
        GiType::Base => registry.base,
        GiType::Callable => registry.callable,
        GiType::Callback => registry.callback,
        GiType::Constant => registry.constant,
        GiType::Enum => registry.enum_,
        GiType::Field => registry.field,
        GiType::Flags => registry.flags,
        GiType::Function => registry.function,
        GiType::Interface => registry.interface,
        GiType::Object => registry.object,
        GiType::Property => registry.property,
        GiType::RegisteredType => registry.registered_type,
        GiType::Repository => registry.repository,
        GiType::Signal => registry.signal,
        GiType::Struct => registry.struct_,
        GiType::Type => registry.type_,
        GiType::Typelib => registry.typelib,
        GiType::Union => registry.union_,
        GiType::Unresolved => registry.unresolved,
        GiType::Value => registry.value,
        GiType::VFunc => registry.vfunc,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Named {
    GObjectObject,
    GObjectObjectClass,
    GObjectObjectFinalizeFunc,
    GObjectValue,
    GObjectBookmarkFile,
    GObjectClosure,
    GObjectCClosure,
    GObjectTypeCValue,
    GObjectInitiallyUnownedClass,
    GObjectParamSpec,
    GLibVariant,
    GLibUnicodeScript,
    GLibDoubleIEEE754,
    GLibMutex,
    GioResolver,
    GioDBusProxy,
    GioApplication,
    GioAppInfo,
    GioAppInfoIface,
    GioDBusMethodInvocation,
    GioSettings,
    GioFile,
    GioAppLaunchContext,
    GioInitable,
    GioAsyncInitable,
    GioResolverError,
    GioAsyncReadyCallback,
    GioDbusInvocationHandled,
    GioDbusError,
    GioActionEntry,
    GioAppInfoCreateFlags,
    GioBufferedInputStream,
    GioSrvTarget,
    GioCancellable,
    GioDbusAnnotationInfo,
    GioZlibCompressorFormat,
    GioAction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Callable {
    GLibGetLocaleVariants,
    GLibFileReadLink,
    GObjectGetProperty,
    GObjectGetQData,
    GObjectNewv,
    GObjectClassListProperties,
    GObjectValueGetUchar,
    GObjectValueGetSchar,
    GLibVariantEqual,
    GLibUnicodeScriptToIso15924,
    GLibMutexClear,
    GLibMutexTrylock,
    GioAppInfoLaunch,
    GioAppInfoLaunchCallback,
    GioDbusInvocationGetConnection,
    GioDbusInvocationReturnErrorLiteral,
    GioAppLaunchContextGetDisplay,
    GioFileReadAsync,
    GioDbusProxyInit,
    GioTlsServerConnectionNew,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VFunc {
    GObjectDispose,
    GioAppInfoLaunch,
    GioFileReadAsync,
    GioAppLaunchContextGetDisplay,
    GioApplicationAfterEmit,
    GioActionActivate,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Signal {
    GObjectNotify,
    GioSettingsChangeEvent,
    GioCancellableCancelled,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Field {
    GObjectObjectClassConstructor,
    GObjectObjectClassSetProperty,
    GLibDoubleVDouble,
    GioAppInfoIfaceLaunch,
    GioActionEntryName,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Arg {
    GObjectGetPropertyName,
    GObjectGetPropertyValue,
    GObjectGetQDataQuark,
    GObjectClassListPropertiesNProperties,
    GLibVariantEqualValue,
    GioSettingsKeys,
    GioSettingsNKeys,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TypeSpec {
    VoidPointer,
    Void,
    Utf8Pointer,
    Uint8,
    Int8,
    Uint32,
    ArrayKeys,
    InterfaceNamed(Named),
    InterfaceCallable(Callable),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Value {
    GioZlibGzip,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Property {
    GioBufferedInputStreamBaseStream,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum InfoKind {
    Named(Named),
    Callable(Callable),
    VFunc(VFunc),
    Signal(Signal),
    Field(Field),
    Arg(Arg),
    Type(TypeSpec),
    Value(Value),
    Property(Property),
}

#[derive(Copy, Clone)]
struct InfoEntry {
    kind: InfoKind,
    owned: bool,
    refs: usize,
}

static INFOS: OnceLock<Mutex<HashMap<usize, InfoEntry>>> = OnceLock::new();

fn infos() -> &'static Mutex<HashMap<usize, InfoEntry>> {
    INFOS.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe impl Send for InfoEntry {}

fn info_type_for_kind(kind: InfoKind) -> GiType {
    match kind {
        InfoKind::Named(named) => info_type_for_named(named),
        InfoKind::Callable(_) => GiType::Function,
        InfoKind::VFunc(_) => GiType::VFunc,
        InfoKind::Signal(_) => GiType::Signal,
        InfoKind::Field(_) => GiType::Field,
        InfoKind::Arg(_) => GiType::Arg,
        InfoKind::Type(_) => GiType::Type,
        InfoKind::Value(_) => GiType::Value,
        InfoKind::Property(_) => GiType::Property,
    }
}

fn info_type_for_named(named: Named) -> GiType {
    match named {
        Named::GObjectObject
        | Named::GObjectParamSpec
        | Named::GioDBusProxy
        | Named::GioApplication
        | Named::GioDBusMethodInvocation
        | Named::GioSettings
        | Named::GioAppLaunchContext
        | Named::GioBufferedInputStream
        | Named::GioCancellable => GiType::Object,
        Named::GioAppInfo
        | Named::GioFile
        | Named::GioInitable
        | Named::GioAsyncInitable
        | Named::GioAction => GiType::Interface,
        Named::GLibUnicodeScript
        | Named::GioResolverError
        | Named::GioDbusError
        | Named::GioZlibCompressorFormat => GiType::Enum,
        Named::GioAppInfoCreateFlags => GiType::Flags,
        Named::GLibDoubleIEEE754 | Named::GLibMutex | Named::GObjectTypeCValue => GiType::Union,
        Named::GObjectObjectFinalizeFunc | Named::GioAsyncReadyCallback => GiType::Callback,
        Named::GioDbusInvocationHandled => GiType::Constant,
        Named::GObjectObjectClass
        | Named::GObjectValue
        | Named::GObjectBookmarkFile
        | Named::GObjectClosure
        | Named::GObjectCClosure
        | Named::GObjectInitiallyUnownedClass
        | Named::GLibVariant
        | Named::GioResolver
        | Named::GioAppInfoIface
        | Named::GioActionEntry
        | Named::GioSrvTarget
        | Named::GioDbusAnnotationInfo => GiType::Struct,
    }
}

fn create_info(kind: InfoKind) -> Ptr {
    let type_ = info_type_for_kind(kind);
    let class = unsafe { g_type_class_ref(gtype_for_info_type(type_)) };
    let mut boxed = Box::new(GIBaseInfoStack {
        parent_instance: GTypeInstance { g_class: class },
        dummy0: 1,
        dummy1: [ptr::null_mut(); 3],
        dummy2: [0; 2],
        dummy3: [ptr::null_mut(); 6],
    });
    let ptr = boxed.as_mut() as *mut GIBaseInfoStack as Ptr;
    infos().lock().unwrap().insert(
        ptr as usize,
        InfoEntry {
            kind,
            owned: true,
            refs: 1,
        },
    );
    Box::into_raw(boxed) as Ptr
}

fn load_stack_info(base: *mut GIBaseInfoStack, kind: InfoKind) {
    if base.is_null() {
        return;
    }
    let type_ = info_type_for_kind(kind);
    let class = unsafe { g_type_class_ref(gtype_for_info_type(type_)) };
    unsafe {
        ptr::write_bytes(base as *mut u8, 0, mem::size_of::<GIBaseInfoStack>());
        (*base).parent_instance.g_class = class;
    }
    infos().lock().unwrap().insert(
        base as usize,
        InfoEntry {
            kind,
            owned: false,
            refs: 1,
        },
    );
}

fn entry_for(info: Ptr) -> Option<InfoEntry> {
    if info.is_null() {
        return None;
    }
    infos().lock().unwrap().get(&(info as usize)).copied()
}

fn c(bytes: &'static [u8]) -> ConstChar {
    bytes.as_ptr() as ConstChar
}

unsafe fn ptr_str(ptr: ConstChar) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    let owned = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    Some(Box::leak(owned.into_boxed_str()))
}

fn cstring_lossy(value: impl AsRef<std::ffi::OsStr>) -> CString {
    CString::new(value.as_ref().as_encoded_bytes()).unwrap_or_else(|_| CString::new("").unwrap())
}

fn default_search_paths() -> Vec<CString> {
    let mut paths: Vec<CString> = std::env::var_os("GI_TYPELIB_PATH")
        .map(|value| std::env::split_paths(&value).map(cstring_lossy).collect())
        .unwrap_or_default();
    paths.push(CString::new(DEFAULT_TYPELIB_DIR).unwrap());
    paths
}

struct RepositoryState {
    search_paths: Vec<CString>,
    search_ptrs: Vec<usize>,
    library_paths: Vec<CString>,
    library_ptrs: Vec<usize>,
    loaded: Vec<&'static [u8]>,
}

impl RepositoryState {
    fn new() -> Self {
        let mut state = Self {
            search_paths: default_search_paths(),
            search_ptrs: Vec::new(),
            library_paths: Vec::new(),
            library_ptrs: vec![0],
            loaded: Vec::new(),
        };
        state.refresh_search_ptrs();
        state
    }

    fn refresh_search_ptrs(&mut self) {
        self.search_ptrs = self
            .search_paths
            .iter()
            .map(|path| path.as_ptr() as usize)
            .chain([0])
            .collect();
    }

    fn refresh_library_ptrs(&mut self) {
        self.library_ptrs = self
            .library_paths
            .iter()
            .map(|path| path.as_ptr() as usize)
            .chain([0])
            .collect();
    }

    fn mark_loaded(&mut self, namespace_: &'static [u8]) {
        if !self.loaded.contains(&namespace_) {
            self.loaded.push(namespace_);
        }
    }
}

static REPOSITORIES: OnceLock<Mutex<HashMap<usize, RepositoryState>>> = OnceLock::new();

fn repositories() -> &'static Mutex<HashMap<usize, RepositoryState>> {
    REPOSITORIES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub unsafe fn new_repository() -> Ptr {
    let repository = unsafe {
        g_object_new_with_properties(
            gtype_for_info_type(GiType::Repository),
            0,
            ptr::null(),
            ptr::null(),
        )
    };
    if !repository.is_null() {
        repositories()
            .lock()
            .unwrap()
            .insert(repository as usize, RepositoryState::new());
    }
    repository
}

pub unsafe fn prepend_search_path(repository: Ptr, path: ConstChar) {
    if path.is_null() {
        return;
    }
    let copy = unsafe { CStr::from_ptr(path) }.to_owned();
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    state.search_paths.insert(0, copy);
    state.refresh_search_ptrs();
}

pub unsafe fn get_search_path(repository: Ptr, n_paths_out: *mut usize) -> ConstCharStrv {
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    if !n_paths_out.is_null() {
        unsafe { *n_paths_out = state.search_paths.len() };
    }
    state.search_ptrs.as_ptr() as ConstCharStrv
}

pub unsafe fn prepend_library_path(repository: Ptr, path: ConstChar) {
    if path.is_null() {
        return;
    }
    let copy = unsafe { CStr::from_ptr(path) }.to_owned();
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    state.library_paths.insert(0, copy);
    state.refresh_library_ptrs();
}

pub unsafe fn get_library_path(repository: Ptr, n_paths_out: *mut usize) -> ConstCharStrv {
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    if !n_paths_out.is_null() {
        unsafe { *n_paths_out = state.library_paths.len() };
    }
    state.library_ptrs.as_ptr() as ConstCharStrv
}

fn namespace_from_str(namespace_: &str) -> Option<&'static [u8]> {
    match namespace_ {
        "GLib" => Some(GLIB),
        "GObject" => Some(GOBJECT),
        "Gio" => Some(GIO),
        "GIRepository" => Some(GIREPOSITORY),
        _ => None,
    }
}

pub unsafe fn repository_require(
    repository: Ptr,
    namespace_: ConstChar,
    version: ConstChar,
    _flags: c_int,
    _error: GErrorOut,
) -> Ptr {
    let Some(namespace) = (unsafe { ptr_str(namespace_) }) else {
        return ptr::null_mut();
    };
    let expected_version = if namespace == "GIRepository" {
        "3.0"
    } else {
        "2.0"
    };
    if !version.is_null() && unsafe { ptr_str(version) } != Some(expected_version) {
        return ptr::null_mut();
    }
    let Some(static_namespace) = namespace_from_str(namespace) else {
        return ptr::null_mut();
    };
    repositories()
        .lock()
        .unwrap()
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new)
        .mark_loaded(static_namespace);

    Box::into_raw(Box::new(0_u8)) as Ptr
}

pub unsafe fn enumerate_versions(
    _repository: Ptr,
    namespace_: ConstChar,
    n_versions_out: *mut usize,
) -> CharStrv {
    let version = match unsafe { ptr_str(namespace_) } {
        Some("GIRepository") => Some(b"3.0\0".as_slice()),
        Some("GLib" | "GObject" | "Gio") => Some(b"2.0\0".as_slice()),
        _ => None,
    };
    match version {
        Some(value) => unsafe { make_strv(&[value], n_versions_out) },
        None => unsafe { make_strv(&[], n_versions_out) },
    }
}

pub unsafe fn loaded_namespaces(repository: Ptr, n_namespaces_out: *mut usize) -> CharStrv {
    let loaded = repositories()
        .lock()
        .unwrap()
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new)
        .loaded
        .clone();
    unsafe { make_strv(&loaded, n_namespaces_out) }
}

pub unsafe fn get_c_prefix(_repository: Ptr, namespace_: ConstChar) -> ConstChar {
    match unsafe { ptr_str(namespace_) } {
        Some("GIRepository") => c(b"GI\0"),
        Some("GLib" | "GObject" | "Gio") => c(b"G\0"),
        _ => ptr::null(),
    }
}

pub unsafe fn get_dependencies(
    _repository: Ptr,
    namespace_: ConstChar,
    n_dependencies_out: *mut usize,
) -> CharStrv {
    match unsafe { ptr_str(namespace_) } {
        Some("GObject") => unsafe { make_strv(&[b"GLib-2.0\0"], n_dependencies_out) },
        Some("Gio") => unsafe {
            make_strv(
                &[b"GLib-2.0\0", b"GObject-2.0\0", b"GModule-2.0\0"],
                n_dependencies_out,
            )
        },
        Some("GIRepository") => unsafe {
            make_strv(
                &[b"GLib-2.0\0", b"GObject-2.0\0", b"Gio-2.0\0"],
                n_dependencies_out,
            )
        },
        _ => unsafe { make_strv(&[], n_dependencies_out) },
    }
}

unsafe fn make_strv(values: &[&'static [u8]], n_out: *mut usize) -> CharStrv {
    if !n_out.is_null() {
        unsafe { *n_out = values.len() };
    }
    let bytes = (values.len() + 1) * mem::size_of::<*mut c_char>();
    let array = unsafe { g_malloc(bytes) } as *mut *mut c_char;
    if array.is_null() {
        return ptr::null_mut();
    }
    for (index, value) in values.iter().enumerate() {
        unsafe { *array.add(index) = g_strdup(c(value)) };
    }
    unsafe { *array.add(values.len()) = ptr::null_mut() };
    array
}

pub unsafe fn find_by_name(_repository: Ptr, namespace_: ConstChar, name: ConstChar) -> Ptr {
    let Some(namespace) = (unsafe { ptr_str(namespace_) }) else {
        return ptr::null_mut();
    };
    let Some(name) = (unsafe { ptr_str(name) }) else {
        return ptr::null_mut();
    };
    let kind = match (namespace, name) {
        ("GLib", "get_locale_variants") => InfoKind::Callable(Callable::GLibGetLocaleVariants),
        ("GLib", "file_read_link") => InfoKind::Callable(Callable::GLibFileReadLink),
        ("GLib", "Variant") => InfoKind::Named(Named::GLibVariant),
        ("GLib", "UnicodeScript") => InfoKind::Named(Named::GLibUnicodeScript),
        ("GLib", "DoubleIEEE754") => InfoKind::Named(Named::GLibDoubleIEEE754),
        ("GLib", "Mutex") => InfoKind::Named(Named::GLibMutex),

        ("GObject", "Object") => InfoKind::Named(Named::GObjectObject),
        ("GObject", "ObjectClass") => InfoKind::Named(Named::GObjectObjectClass),
        ("GObject", "ObjectFinalizeFunc") => InfoKind::Named(Named::GObjectObjectFinalizeFunc),
        ("GObject", "Value") => InfoKind::Named(Named::GObjectValue),
        ("GObject", "BookmarkFile") => InfoKind::Named(Named::GObjectBookmarkFile),
        ("GObject", "Closure") => InfoKind::Named(Named::GObjectClosure),
        ("GObject", "CClosure") => InfoKind::Named(Named::GObjectCClosure),
        ("GObject", "TypeCValue") => InfoKind::Named(Named::GObjectTypeCValue),
        ("GObject", "InitiallyUnownedClass") => {
            InfoKind::Named(Named::GObjectInitiallyUnownedClass)
        }
        ("GObject", "ParamSpec") => InfoKind::Named(Named::GObjectParamSpec),

        ("Gio", "Resolver") => InfoKind::Named(Named::GioResolver),
        ("Gio", "DBusProxy") => InfoKind::Named(Named::GioDBusProxy),
        ("Gio", "Application") => InfoKind::Named(Named::GioApplication),
        ("Gio", "AppInfo") => InfoKind::Named(Named::GioAppInfo),
        ("Gio", "AppInfoIface") => InfoKind::Named(Named::GioAppInfoIface),
        ("Gio", "DBusMethodInvocation") => InfoKind::Named(Named::GioDBusMethodInvocation),
        ("Gio", "Settings") => InfoKind::Named(Named::GioSettings),
        ("Gio", "File") => InfoKind::Named(Named::GioFile),
        ("Gio", "AppLaunchContext") => InfoKind::Named(Named::GioAppLaunchContext),
        ("Gio", "Initable") => InfoKind::Named(Named::GioInitable),
        ("Gio", "AsyncInitable") => InfoKind::Named(Named::GioAsyncInitable),
        ("Gio", "AsyncReadyCallback") => InfoKind::Named(Named::GioAsyncReadyCallback),
        ("Gio", "DBUS_METHOD_INVOCATION_HANDLED") => {
            InfoKind::Named(Named::GioDbusInvocationHandled)
        }
        ("Gio", "DBusError") => InfoKind::Named(Named::GioDbusError),
        ("Gio", "ActionEntry") => InfoKind::Named(Named::GioActionEntry),
        ("Gio", "AppInfoCreateFlags") => InfoKind::Named(Named::GioAppInfoCreateFlags),
        ("Gio", "BufferedInputStream") => InfoKind::Named(Named::GioBufferedInputStream),
        ("Gio", "SrvTarget") => InfoKind::Named(Named::GioSrvTarget),
        ("Gio", "Cancellable") => InfoKind::Named(Named::GioCancellable),
        ("Gio", "DBusAnnotationInfo") => InfoKind::Named(Named::GioDbusAnnotationInfo),
        ("Gio", "ZlibCompressorFormat") => InfoKind::Named(Named::GioZlibCompressorFormat),
        ("Gio", "Action") => InfoKind::Named(Named::GioAction),
        ("Gio", "tls_server_connection_new") => {
            InfoKind::Callable(Callable::GioTlsServerConnectionNew)
        }
        _ => return ptr::null_mut(),
    };
    create_info(kind)
}

pub unsafe fn find_by_gtype(_repository: Ptr, gtype: GType) -> Ptr {
    if gtype == 0 {
        ptr::null_mut()
    } else {
        create_info(InfoKind::Named(Named::GObjectObject))
    }
}

pub unsafe fn find_by_error_domain(_repository: Ptr, domain: GQuark) -> Ptr {
    if domain == 0 {
        ptr::null_mut()
    } else {
        create_info(InfoKind::Named(Named::GioResolverError))
    }
}

pub unsafe fn get_object_gtype_interfaces(
    _repository: Ptr,
    _gtype: GType,
    n_interfaces_out: *mut usize,
    interfaces_out: *mut *mut Ptr,
) {
    if !n_interfaces_out.is_null() {
        unsafe { *n_interfaces_out = 2 };
    }
    if !interfaces_out.is_null() {
        let array = unsafe { g_malloc(2 * mem::size_of::<Ptr>()) } as *mut Ptr;
        if !array.is_null() {
            unsafe {
                *array.add(0) = create_info(InfoKind::Named(Named::GioInitable));
                *array.add(1) = create_info(InfoKind::Named(Named::GioAsyncInitable));
                *interfaces_out = array;
            }
        }
    }
}

pub unsafe fn base_info_ref(info: Ptr) -> Ptr {
    if !info.is_null() {
        if let Some(entry) = infos().lock().unwrap().get_mut(&(info as usize)) {
            entry.refs += 1;
        }
    }
    info
}

pub unsafe fn base_info_unref(info: Ptr) {
    if info.is_null() {
        return;
    }
    let mut map = infos().lock().unwrap();
    let key = info as usize;
    let Some(entry) = map.get_mut(&key) else {
        return;
    };
    if entry.refs > 1 {
        entry.refs -= 1;
        return;
    }
    let owned = entry.owned;
    map.remove(&key);
    drop(map);
    if owned {
        unsafe {
            drop(Box::from_raw(info as *mut GIBaseInfoStack));
        }
    }
}

pub unsafe fn base_info_clear(info: Ptr) {
    if info.is_null() {
        return;
    }
    let mut map = infos().lock().unwrap();
    if let Some(entry) = map.remove(&(info as usize)) {
        if entry.owned {
            drop(map);
            unsafe {
                drop(Box::from_raw(info as *mut GIBaseInfoStack));
            }
            return;
        }
    }
    unsafe {
        ptr::write_bytes(info as *mut u8, 0, mem::size_of::<GIBaseInfoStack>());
    }
}

pub unsafe fn base_info_get_attribute(info: Ptr, name: ConstChar) -> ConstChar {
    let Some(entry) = entry_for(info) else {
        return ptr::null();
    };
    match (entry.kind, unsafe { ptr_str(name) }) {
        (InfoKind::Value(Value::GioZlibGzip), Some("c:identifier")) => {
            c(b"G_ZLIB_COMPRESSOR_FORMAT_GZIP\0")
        }
        _ => ptr::null(),
    }
}

pub unsafe fn base_info_get_name(info: Ptr) -> ConstChar {
    let Some(entry) = entry_for(info) else {
        return ptr::null();
    };
    match entry.kind {
        InfoKind::Named(named) => named_name(named),
        InfoKind::Callable(callable) => callable_name(callable),
        InfoKind::VFunc(vfunc) => vfunc_name(vfunc),
        InfoKind::Signal(signal) => signal_name(signal),
        InfoKind::Field(field) => field_name(field),
        InfoKind::Arg(arg) => arg_name(arg),
        InfoKind::Type(_) => ptr::null(),
        InfoKind::Value(Value::GioZlibGzip) => c(b"gzip\0"),
        InfoKind::Property(Property::GioBufferedInputStreamBaseStream) => c(b"base-stream\0"),
    }
}

pub unsafe fn base_info_get_namespace(info: Ptr) -> ConstChar {
    let Some(entry) = entry_for(info) else {
        return ptr::null();
    };
    match entry.kind {
        InfoKind::Named(named) => named_namespace(named),
        InfoKind::Callable(callable) => callable_namespace(callable),
        InfoKind::VFunc(vfunc) => vfunc_namespace(vfunc),
        InfoKind::Signal(signal) => signal_namespace(signal),
        InfoKind::Field(field) => field_namespace(field),
        InfoKind::Arg(_) | InfoKind::Type(_) | InfoKind::Value(_) | InfoKind::Property(_) => {
            ptr::null()
        }
    }
}

fn named_namespace(named: Named) -> ConstChar {
    match named {
        Named::GObjectObject
        | Named::GObjectObjectClass
        | Named::GObjectObjectFinalizeFunc
        | Named::GObjectValue
        | Named::GObjectBookmarkFile
        | Named::GObjectClosure
        | Named::GObjectCClosure
        | Named::GObjectTypeCValue
        | Named::GObjectInitiallyUnownedClass
        | Named::GObjectParamSpec => c(GOBJECT),
        Named::GLibVariant
        | Named::GLibUnicodeScript
        | Named::GLibDoubleIEEE754
        | Named::GLibMutex => c(GLIB),
        _ => c(GIO),
    }
}

fn named_name(named: Named) -> ConstChar {
    c(match named {
        Named::GObjectObject => b"Object\0",
        Named::GObjectObjectClass => b"ObjectClass\0",
        Named::GObjectObjectFinalizeFunc => b"ObjectFinalizeFunc\0",
        Named::GObjectValue => b"Value\0",
        Named::GObjectBookmarkFile => b"BookmarkFile\0",
        Named::GObjectClosure => b"Closure\0",
        Named::GObjectCClosure => b"CClosure\0",
        Named::GObjectTypeCValue => b"TypeCValue\0",
        Named::GObjectInitiallyUnownedClass => b"InitiallyUnownedClass\0",
        Named::GObjectParamSpec => b"ParamSpec\0",
        Named::GLibVariant => b"Variant\0",
        Named::GLibUnicodeScript => b"UnicodeScript\0",
        Named::GLibDoubleIEEE754 => b"DoubleIEEE754\0",
        Named::GLibMutex => b"Mutex\0",
        Named::GioResolver => b"Resolver\0",
        Named::GioDBusProxy => b"DBusProxy\0",
        Named::GioApplication => b"Application\0",
        Named::GioAppInfo => b"AppInfo\0",
        Named::GioAppInfoIface => b"AppInfoIface\0",
        Named::GioDBusMethodInvocation => b"DBusMethodInvocation\0",
        Named::GioSettings => b"Settings\0",
        Named::GioFile => b"File\0",
        Named::GioAppLaunchContext => b"AppLaunchContext\0",
        Named::GioInitable => b"Initable\0",
        Named::GioAsyncInitable => b"AsyncInitable\0",
        Named::GioResolverError => b"ResolverError\0",
        Named::GioAsyncReadyCallback => b"AsyncReadyCallback\0",
        Named::GioDbusInvocationHandled => b"DBUS_METHOD_INVOCATION_HANDLED\0",
        Named::GioDbusError => b"DBusError\0",
        Named::GioActionEntry => b"ActionEntry\0",
        Named::GioAppInfoCreateFlags => b"AppInfoCreateFlags\0",
        Named::GioBufferedInputStream => b"BufferedInputStream\0",
        Named::GioSrvTarget => b"SrvTarget\0",
        Named::GioCancellable => b"Cancellable\0",
        Named::GioDbusAnnotationInfo => b"DBusAnnotationInfo\0",
        Named::GioZlibCompressorFormat => b"ZlibCompressorFormat\0",
        Named::GioAction => b"Action\0",
    })
}

fn callable_namespace(callable: Callable) -> ConstChar {
    match callable {
        Callable::GLibGetLocaleVariants
        | Callable::GLibFileReadLink
        | Callable::GLibVariantEqual
        | Callable::GLibUnicodeScriptToIso15924
        | Callable::GLibMutexClear
        | Callable::GLibMutexTrylock => c(GLIB),
        Callable::GObjectGetProperty
        | Callable::GObjectGetQData
        | Callable::GObjectNewv
        | Callable::GObjectClassListProperties
        | Callable::GObjectValueGetUchar
        | Callable::GObjectValueGetSchar => c(GOBJECT),
        _ => c(GIO),
    }
}

fn callable_name(callable: Callable) -> ConstChar {
    c(match callable {
        Callable::GLibGetLocaleVariants => b"get_locale_variants\0",
        Callable::GLibFileReadLink => b"file_read_link\0",
        Callable::GObjectGetProperty => b"get_property\0",
        Callable::GObjectGetQData => b"get_qdata\0",
        Callable::GObjectNewv => b"newv\0",
        Callable::GObjectClassListProperties => b"list_properties\0",
        Callable::GObjectValueGetUchar => b"get_uchar\0",
        Callable::GObjectValueGetSchar => b"get_schar\0",
        Callable::GLibVariantEqual => b"equal\0",
        Callable::GLibUnicodeScriptToIso15924 => b"to_iso15924\0",
        Callable::GLibMutexClear => b"clear\0",
        Callable::GLibMutexTrylock => b"trylock\0",
        Callable::GioAppInfoLaunch => b"launch\0",
        Callable::GioAppInfoLaunchCallback => b"launch\0",
        Callable::GioDbusInvocationGetConnection => b"get_connection\0",
        Callable::GioDbusInvocationReturnErrorLiteral => b"return_error_literal\0",
        Callable::GioAppLaunchContextGetDisplay => b"get_display\0",
        Callable::GioFileReadAsync => b"read_async\0",
        Callable::GioDbusProxyInit => b"init\0",
        Callable::GioTlsServerConnectionNew => b"tls_server_connection_new\0",
    })
}

fn vfunc_namespace(vfunc: VFunc) -> ConstChar {
    match vfunc {
        VFunc::GObjectDispose => c(GOBJECT),
        _ => c(GIO),
    }
}

fn vfunc_name(vfunc: VFunc) -> ConstChar {
    c(match vfunc {
        VFunc::GObjectDispose => b"dispose\0",
        VFunc::GioAppInfoLaunch => b"launch\0",
        VFunc::GioFileReadAsync => b"read_async\0",
        VFunc::GioAppLaunchContextGetDisplay => b"get_display\0",
        VFunc::GioApplicationAfterEmit => b"after_emit\0",
        VFunc::GioActionActivate => b"activate\0",
    })
}

fn signal_namespace(signal: Signal) -> ConstChar {
    match signal {
        Signal::GObjectNotify => c(GOBJECT),
        _ => c(GIO),
    }
}

fn signal_name(signal: Signal) -> ConstChar {
    c(match signal {
        Signal::GObjectNotify => b"notify\0",
        Signal::GioSettingsChangeEvent => b"change-event\0",
        Signal::GioCancellableCancelled => b"cancelled\0",
    })
}

fn field_namespace(field: Field) -> ConstChar {
    match field {
        Field::GObjectObjectClassConstructor | Field::GObjectObjectClassSetProperty => c(GOBJECT),
        Field::GLibDoubleVDouble => c(GLIB),
        Field::GioAppInfoIfaceLaunch | Field::GioActionEntryName => c(GIO),
    }
}

fn field_name(field: Field) -> ConstChar {
    c(match field {
        Field::GObjectObjectClassConstructor => b"constructor\0",
        Field::GObjectObjectClassSetProperty => b"set_property\0",
        Field::GLibDoubleVDouble => b"v_double\0",
        Field::GioAppInfoIfaceLaunch => b"launch\0",
        Field::GioActionEntryName => b"name\0",
    })
}

fn arg_name(arg: Arg) -> ConstChar {
    c(match arg {
        Arg::GObjectGetPropertyName => b"property_name\0",
        Arg::GObjectGetPropertyValue => b"value\0",
        Arg::GObjectGetQDataQuark => b"quark\0",
        Arg::GObjectClassListPropertiesNProperties => b"n_properties\0",
        Arg::GLibVariantEqualValue => b"two\0",
        Arg::GioSettingsKeys => b"keys\0",
        Arg::GioSettingsNKeys => b"n_keys\0",
    })
}

pub unsafe fn arg_get_closure_index(_info: Ptr, out_index: *mut guint) -> gboolean {
    if !out_index.is_null() {
        unsafe { *out_index = 0 };
    }
    0
}

pub unsafe fn arg_get_destroy_index(info: Ptr, out_index: *mut guint) -> gboolean {
    unsafe { arg_get_closure_index(info, out_index) }
}

struct ArgSpecData {
    direction: c_int,
    transfer: c_int,
    type_: TypeSpec,
}

fn arg_spec(arg: Arg) -> ArgSpecData {
    match arg {
        Arg::GObjectClassListPropertiesNProperties => ArgSpecData {
            direction: GI_DIRECTION_OUT,
            transfer: GI_TRANSFER_EVERYTHING,
            type_: TypeSpec::Uint32,
        },
        Arg::GioSettingsKeys => ArgSpecData {
            direction: GI_DIRECTION_IN,
            transfer: GI_TRANSFER_NOTHING,
            type_: TypeSpec::ArrayKeys,
        },
        Arg::GioSettingsNKeys => ArgSpecData {
            direction: GI_DIRECTION_IN,
            transfer: GI_TRANSFER_NOTHING,
            type_: TypeSpec::Uint32,
        },
        Arg::GObjectGetPropertyName => ArgSpecData {
            direction: GI_DIRECTION_IN,
            transfer: GI_TRANSFER_NOTHING,
            type_: TypeSpec::Utf8Pointer,
        },
        Arg::GLibVariantEqualValue | Arg::GObjectGetPropertyValue | Arg::GObjectGetQDataQuark => {
            ArgSpecData {
                direction: GI_DIRECTION_IN,
                transfer: GI_TRANSFER_NOTHING,
                type_: TypeSpec::VoidPointer,
            }
        }
    }
}

pub unsafe fn arg_get_direction(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg_spec(arg).direction,
        _ => GI_DIRECTION_IN,
    }
}

pub unsafe fn arg_get_ownership_transfer(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg_spec(arg).transfer,
        _ => GI_TRANSFER_NOTHING,
    }
}

pub unsafe fn arg_get_scope(_info: Ptr) -> c_int {
    GI_SCOPE_TYPE_INVALID
}

pub unsafe fn arg_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => create_info(InfoKind::Type(arg_spec(arg).type_)),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn arg_load_type_info(info: Ptr, type_info: *mut GITypeInfo) {
    if let Some(InfoKind::Arg(arg)) = entry_for(info).map(|entry| entry.kind) {
        load_stack_info(
            type_info as *mut GIBaseInfoStack,
            InfoKind::Type(arg_spec(arg).type_),
        );
    }
}

pub unsafe fn arg_may_be_null(_info: Ptr) -> gboolean {
    0
}

const OBJECT_GET_PROPERTY_ARGS: [Arg; 2] =
    [Arg::GObjectGetPropertyName, Arg::GObjectGetPropertyValue];
const OBJECT_GET_QDATA_ARGS: [Arg; 1] = [Arg::GObjectGetQDataQuark];
const OBJECT_LIST_PROPERTIES_ARGS: [Arg; 1] = [Arg::GObjectClassListPropertiesNProperties];
const VARIANT_EQUAL_ARGS: [Arg; 1] = [Arg::GLibVariantEqualValue];
const SETTINGS_CHANGE_ARGS: [Arg; 2] = [Arg::GioSettingsKeys, Arg::GioSettingsNKeys];
const NO_ARGS: [Arg; 0] = [];

fn callable_args(callable: Callable) -> &'static [Arg] {
    match callable {
        Callable::GObjectGetProperty => &OBJECT_GET_PROPERTY_ARGS,
        Callable::GObjectGetQData | Callable::GObjectNewv => &OBJECT_GET_QDATA_ARGS,
        Callable::GObjectClassListProperties => &OBJECT_LIST_PROPERTIES_ARGS,
        Callable::GLibVariantEqual => &VARIANT_EQUAL_ARGS,
        _ => &NO_ARGS,
    }
}

fn signal_args(signal: Signal) -> &'static [Arg] {
    match signal {
        Signal::GioSettingsChangeEvent => &SETTINGS_CHANGE_ARGS,
        _ => &NO_ARGS,
    }
}

fn callable_return_type(callable: Callable) -> TypeSpec {
    match callable {
        Callable::GObjectGetQData => TypeSpec::VoidPointer,
        Callable::GObjectNewv => TypeSpec::InterfaceNamed(Named::GObjectObject),
        Callable::GObjectValueGetUchar => TypeSpec::Uint8,
        Callable::GObjectValueGetSchar => TypeSpec::Int8,
        Callable::GioFileReadAsync => TypeSpec::VoidPointer,
        _ => TypeSpec::Void,
    }
}

fn vfunc_return_type(vfunc: VFunc) -> TypeSpec {
    match vfunc {
        VFunc::GioFileReadAsync => TypeSpec::VoidPointer,
        _ => TypeSpec::Void,
    }
}

pub unsafe fn callable_can_throw_gerror(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(Callable::GLibFileReadLink))
        | Some(InfoKind::Callable(Callable::GioAppInfoLaunch))
        | Some(InfoKind::Callable(Callable::GioAppInfoLaunchCallback))
        | Some(InfoKind::VFunc(VFunc::GioAppInfoLaunch)) => 1,
        _ => 0,
    }
}

pub unsafe fn callable_get_arg(info: Ptr, index: guint) -> Ptr {
    let args = match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(callable)) => callable_args(callable),
        Some(InfoKind::Signal(signal)) => signal_args(signal),
        _ => &NO_ARGS,
    };
    args.get(index as usize)
        .map(|arg| create_info(InfoKind::Arg(*arg)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn callable_get_instance_ownership_transfer(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(Callable::GioDbusInvocationReturnErrorLiteral)) => {
            GI_TRANSFER_EVERYTHING
        }
        _ => GI_TRANSFER_NOTHING,
    }
}

pub unsafe fn callable_get_n_args(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(callable)) => callable_args(callable).len() as guint,
        Some(InfoKind::Signal(signal)) => signal_args(signal).len() as guint,
        _ => 0,
    }
}

pub unsafe fn callable_get_return_type(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(callable)) => {
            create_info(InfoKind::Type(callable_return_type(callable)))
        }
        Some(InfoKind::VFunc(vfunc)) => create_info(InfoKind::Type(vfunc_return_type(vfunc))),
        _ => create_info(InfoKind::Type(TypeSpec::Void)),
    }
}

pub unsafe fn callable_is_method(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(
            Callable::GObjectGetProperty
            | Callable::GObjectGetQData
            | Callable::GObjectNewv
            | Callable::GObjectClassListProperties
            | Callable::GObjectValueGetUchar
            | Callable::GObjectValueGetSchar
            | Callable::GLibVariantEqual
            | Callable::GLibMutexClear
            | Callable::GLibMutexTrylock
            | Callable::GioAppInfoLaunch
            | Callable::GioDbusInvocationGetConnection
            | Callable::GioDbusInvocationReturnErrorLiteral
            | Callable::GioAppLaunchContextGetDisplay
            | Callable::GioFileReadAsync
            | Callable::GioDbusProxyInit,
        )) => 1,
        _ => 0,
    }
}

pub unsafe fn callable_load_arg(info: Ptr, index: guint, arg_info: *mut GIArgInfo) {
    let args = match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(callable)) => callable_args(callable),
        Some(InfoKind::Signal(signal)) => signal_args(signal),
        _ => &NO_ARGS,
    };
    if let Some(arg) = args.get(index as usize) {
        load_stack_info(arg_info as *mut GIBaseInfoStack, InfoKind::Arg(*arg));
    }
}

pub unsafe fn callable_load_return_type(info: Ptr, type_info: *mut GITypeInfo) {
    let spec = match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(callable)) => callable_return_type(callable),
        Some(InfoKind::VFunc(vfunc)) => vfunc_return_type(vfunc),
        _ => TypeSpec::Void,
    };
    load_stack_info(type_info as *mut GIBaseInfoStack, InfoKind::Type(spec));
}

pub unsafe fn callable_may_return_null(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(Callable::GObjectGetQData)) => 1,
        _ => 0,
    }
}

pub unsafe fn enum_get_n_methods(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GLibUnicodeScript)) => 1,
        _ => 0,
    }
}

pub unsafe fn enum_get_method(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GLibUnicodeScript)), 0) => {
            create_info(InfoKind::Callable(Callable::GLibUnicodeScriptToIso15924))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn enum_get_n_values(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GioZlibCompressorFormat)) => 1,
        _ => 0,
    }
}

pub unsafe fn enum_get_value(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GioZlibCompressorFormat)), 0) => {
            create_info(InfoKind::Value(Value::GioZlibGzip))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn field_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(Field::GioAppInfoIfaceLaunch)) => create_info(InfoKind::Type(
            TypeSpec::InterfaceCallable(Callable::GioAppInfoLaunchCallback),
        )),
        Some(InfoKind::Field(Field::GioActionEntryName)) => {
            create_info(InfoKind::Type(TypeSpec::Utf8Pointer))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn function_get_flags(_info: Ptr) -> c_int {
    0
}

pub unsafe fn function_get_symbol(info: Ptr) -> ConstChar {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(Callable::GLibUnicodeScriptToIso15924)) => {
            c(b"g_unicode_script_to_iso15924\0")
        }
        Some(InfoKind::Callable(Callable::GioTlsServerConnectionNew)) => {
            c(b"g_tls_server_connection_new\0")
        }
        _ => ptr::null(),
    }
}

pub unsafe fn function_invoke(
    info: Ptr,
    _in_args: *const GIArgument,
    _n_in_args: usize,
    _out_args: *mut GIArgument,
    _n_out_args: usize,
    _return_value: *mut GIArgument,
    error: GErrorOut,
) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Callable(Callable::GLibFileReadLink)) => {
            unsafe {
                g_set_error_literal(
                    error,
                    g_file_error_quark(),
                    G_FILE_ERROR_NOENT,
                    c(b"No such file or directory\0"),
                );
            }
            0
        }
        _ => 0,
    }
}

pub unsafe fn function_prep_invoker(_info: Ptr, _invoker: Ptr, _error: GErrorOut) -> gboolean {
    1
}

pub unsafe fn interface_find_method(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GioAppInfo)), Some("launch")) => {
            create_info(InfoKind::Callable(Callable::GioAppInfoLaunch))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn interface_find_vfunc(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GioAppInfo)), Some("launch")) => {
            create_info(InfoKind::VFunc(VFunc::GioAppInfoLaunch))
        }
        (Some(InfoKind::Named(Named::GioFile)), Some("read_async")) => {
            create_info(InfoKind::VFunc(VFunc::GioFileReadAsync))
        }
        (Some(InfoKind::Named(Named::GioAction)), Some("activate")) => {
            create_info(InfoKind::VFunc(VFunc::GioActionActivate))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_find_method(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GObjectObject)), Some("get_property")) => {
            create_info(InfoKind::Callable(Callable::GObjectGetProperty))
        }
        (Some(InfoKind::Named(Named::GObjectObject)), Some("get_qdata")) => {
            create_info(InfoKind::Callable(Callable::GObjectGetQData))
        }
        (Some(InfoKind::Named(Named::GObjectObject)), Some("newv")) => {
            create_info(InfoKind::Callable(Callable::GObjectNewv))
        }
        (Some(InfoKind::Named(Named::GioDBusMethodInvocation)), Some("get_connection")) => {
            create_info(InfoKind::Callable(Callable::GioDbusInvocationGetConnection))
        }
        (Some(InfoKind::Named(Named::GioDBusMethodInvocation)), Some("return_error_literal")) => {
            create_info(InfoKind::Callable(
                Callable::GioDbusInvocationReturnErrorLiteral,
            ))
        }
        (Some(InfoKind::Named(Named::GioAppLaunchContext)), Some("get_display")) => {
            create_info(InfoKind::Callable(Callable::GioAppLaunchContextGetDisplay))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_find_method_using_interfaces(
    info: Ptr,
    name: ConstChar,
    declarer_out: *mut Ptr,
) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GioDBusProxy)), Some("init")) => {
            if !declarer_out.is_null() {
                unsafe { *declarer_out = create_info(InfoKind::Named(Named::GioInitable)) };
            }
            create_info(InfoKind::Callable(Callable::GioDbusProxyInit))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_find_signal(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GObjectObject)), Some("notify")) => {
            create_info(InfoKind::Signal(Signal::GObjectNotify))
        }
        (Some(InfoKind::Named(Named::GioSettings)), Some("change-event")) => {
            create_info(InfoKind::Signal(Signal::GioSettingsChangeEvent))
        }
        (Some(InfoKind::Named(Named::GioCancellable)), Some("cancelled")) => {
            create_info(InfoKind::Signal(Signal::GioCancellableCancelled))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_find_vfunc(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GObjectObject)), Some("dispose")) => {
            create_info(InfoKind::VFunc(VFunc::GObjectDispose))
        }
        (Some(InfoKind::Named(Named::GioAppLaunchContext)), Some("get_display")) => {
            create_info(InfoKind::VFunc(VFunc::GioAppLaunchContextGetDisplay))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_find_vfunc_using_interfaces(
    info: Ptr,
    name: ConstChar,
    declarer_out: *mut Ptr,
) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GioApplication)), Some("after_emit")) => {
            if !declarer_out.is_null() {
                unsafe { *declarer_out = create_info(InfoKind::Named(Named::GioApplication)) };
            }
            create_info(InfoKind::VFunc(VFunc::GioApplicationAfterEmit))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_get_n_methods(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectObject)) => 3,
        _ => 0,
    }
}

pub unsafe fn object_get_method(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GObjectObject)), 0) => {
            create_info(InfoKind::Callable(Callable::GObjectGetProperty))
        }
        (Some(InfoKind::Named(Named::GObjectObject)), 1) => {
            create_info(InfoKind::Callable(Callable::GObjectGetQData))
        }
        (Some(InfoKind::Named(Named::GObjectObject)), 2) => {
            create_info(InfoKind::Callable(Callable::GObjectNewv))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn object_get_property(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GioBufferedInputStream)), 0) => create_info(
            InfoKind::Property(Property::GioBufferedInputStreamBaseStream),
        ),
        _ => ptr::null_mut(),
    }
}

pub unsafe extern "C" fn local_ref_func() {}

pub unsafe fn object_get_ref_function_pointer(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectParamSpec)) => local_ref_func as *const () as Ptr,
        _ => ptr::null_mut(),
    }
}

pub unsafe fn registered_get_g_type(info: Ptr) -> GType {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectObject)) => unsafe { g_object_get_type() },
        Some(InfoKind::Named(
            Named::GObjectBookmarkFile
            | Named::GObjectClosure
            | Named::GObjectParamSpec
            | Named::GioSrvTarget
            | Named::GioBufferedInputStream,
        )) => 1,
        _ => 0,
    }
}

pub unsafe fn registered_get_type_name(info: Ptr) -> ConstChar {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectObject)) => c(b"GObject\0"),
        Some(InfoKind::Named(Named::GObjectBookmarkFile)) => c(b"GBookmarkFile\0"),
        Some(InfoKind::Named(Named::GObjectClosure)) => c(b"GClosure\0"),
        Some(InfoKind::Named(Named::GObjectParamSpec)) => c(b"GParamSpec\0"),
        Some(InfoKind::Named(Named::GioSrvTarget)) => c(b"GSrvTarget\0"),
        Some(InfoKind::Named(Named::GioBufferedInputStream)) => c(b"GBufferedInputStream\0"),
        _ => ptr::null(),
    }
}

pub unsafe fn registered_get_type_init_function_name(info: Ptr) -> ConstChar {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectObject)) => c(b"g_object_get_type\0"),
        Some(InfoKind::Named(Named::GObjectBookmarkFile)) => c(b"g_bookmark_file_get_type\0"),
        Some(InfoKind::Named(Named::GObjectClosure)) => c(b"g_closure_get_type\0"),
        Some(InfoKind::Named(Named::GObjectParamSpec)) => c(b"g_param_spec_get_type\0"),
        Some(InfoKind::Named(Named::GioSrvTarget)) => c(b"g_srv_target_get_type\0"),
        Some(InfoKind::Named(Named::GioBufferedInputStream)) => {
            c(b"g_buffered_input_stream_get_type\0")
        }
        _ => ptr::null(),
    }
}

pub unsafe fn registered_is_boxed(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectBookmarkFile | Named::GObjectClosure)) => 1,
        _ => 0,
    }
}

pub unsafe fn signal_get_flags(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Signal(Signal::GObjectNotify)) => G_SIGNAL_NOTIFY_FLAGS,
        _ => 0,
    }
}

pub unsafe fn struct_find_field(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GObjectObjectClass)), Some("constructor")) => {
            create_info(InfoKind::Field(Field::GObjectObjectClassConstructor))
        }
        (Some(InfoKind::Named(Named::GObjectObjectClass)), Some("set_property")) => {
            create_info(InfoKind::Field(Field::GObjectObjectClassSetProperty))
        }
        (Some(InfoKind::Named(Named::GioAppInfoIface)), Some("launch")) => {
            create_info(InfoKind::Field(Field::GioAppInfoIfaceLaunch))
        }
        (Some(InfoKind::Named(Named::GioActionEntry)), Some("name")) => {
            create_info(InfoKind::Field(Field::GioActionEntryName))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn struct_find_method(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GObjectObjectClass)), Some("list_properties")) => {
            create_info(InfoKind::Callable(Callable::GObjectClassListProperties))
        }
        (Some(InfoKind::Named(Named::GObjectValue)), Some("get_uchar")) => {
            create_info(InfoKind::Callable(Callable::GObjectValueGetUchar))
        }
        (Some(InfoKind::Named(Named::GObjectValue)), Some("get_schar")) => {
            create_info(InfoKind::Callable(Callable::GObjectValueGetSchar))
        }
        (Some(InfoKind::Named(Named::GLibVariant)), Some("equal")) => {
            create_info(InfoKind::Callable(Callable::GLibVariantEqual))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn struct_get_n_fields(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectObjectClass)) => 2,
        _ => 0,
    }
}

pub unsafe fn struct_get_field(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GObjectObjectClass)), 0) => {
            create_info(InfoKind::Field(Field::GObjectObjectClassConstructor))
        }
        (Some(InfoKind::Named(Named::GObjectObjectClass)), 1) => {
            create_info(InfoKind::Field(Field::GObjectObjectClassSetProperty))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn struct_get_size(info: Ptr) -> usize {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectValue)) => 24,
        _ => 0,
    }
}

pub unsafe fn struct_is_gtype_struct(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GObjectInitiallyUnownedClass)) => 1,
        _ => 0,
    }
}

pub unsafe fn type_get_array_length_index(info: Ptr, out_index: *mut guint) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(TypeSpec::ArrayKeys)) => {
            if !out_index.is_null() {
                unsafe { *out_index = 1 };
            }
            1
        }
        _ => {
            if !out_index.is_null() {
                unsafe { *out_index = 0 };
            }
            0
        }
    }
}

pub unsafe fn type_get_array_type(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(TypeSpec::ArrayKeys)) => GI_ARRAY_TYPE_C,
        _ => 0,
    }
}

pub unsafe fn type_get_interface(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(TypeSpec::InterfaceNamed(named))) => {
            create_info(InfoKind::Named(named))
        }
        Some(InfoKind::Type(TypeSpec::InterfaceCallable(callable))) => {
            create_info(InfoKind::Callable(callable))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn type_get_tag(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(TypeSpec::Utf8Pointer)) => GI_TYPE_TAG_UTF8,
        Some(InfoKind::Type(TypeSpec::Uint8)) => GI_TYPE_TAG_UINT8,
        Some(InfoKind::Type(TypeSpec::Int8)) => GI_TYPE_TAG_INT8,
        Some(InfoKind::Type(TypeSpec::Uint32)) => GI_TYPE_TAG_UINT32,
        Some(InfoKind::Type(TypeSpec::ArrayKeys)) => GI_TYPE_TAG_ARRAY,
        Some(InfoKind::Type(TypeSpec::InterfaceNamed(_) | TypeSpec::InterfaceCallable(_))) => {
            GI_TYPE_TAG_INTERFACE
        }
        _ => GI_TYPE_TAG_VOID,
    }
}

pub unsafe fn type_is_pointer(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(
            TypeSpec::VoidPointer
            | TypeSpec::Utf8Pointer
            | TypeSpec::ArrayKeys
            | TypeSpec::InterfaceNamed(_)
            | TypeSpec::InterfaceCallable(_),
        )) => 1,
        _ => 0,
    }
}

pub unsafe fn type_is_zero_terminated(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn typelib_ref(typelib: Ptr) -> Ptr {
    typelib
}

pub unsafe fn union_find_method(info: Ptr, name: ConstChar) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), unsafe {
        ptr_str(name)
    }) {
        (Some(InfoKind::Named(Named::GLibMutex)), Some("trylock")) => {
            create_info(InfoKind::Callable(Callable::GLibMutexTrylock))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn union_get_alignment(info: Ptr) -> usize {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GLibDoubleIEEE754)) => 8,
        _ => 0,
    }
}

pub unsafe fn union_get_field(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GLibDoubleIEEE754)), 0) => {
            create_info(InfoKind::Field(Field::GLibDoubleVDouble))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn union_get_discriminator_offset(_info: Ptr, out_offset: *mut usize) -> gboolean {
    if !out_offset.is_null() {
        unsafe { *out_offset = 0 };
    }
    0
}

pub unsafe fn union_get_method(info: Ptr, index: guint) -> Ptr {
    match (entry_for(info).map(|entry| entry.kind), index) {
        (Some(InfoKind::Named(Named::GLibMutex)), 0) => {
            create_info(InfoKind::Callable(Callable::GLibMutexClear))
        }
        _ => ptr::null_mut(),
    }
}

pub unsafe fn union_get_n_fields(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GLibDoubleIEEE754)) => 1,
        _ => 0,
    }
}

pub unsafe fn union_get_n_methods(info: Ptr) -> guint {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GLibMutex)) => 5,
        _ => 0,
    }
}

pub unsafe fn union_get_size(info: Ptr) -> usize {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Named(Named::GLibDoubleIEEE754)) => 8,
        _ => 0,
    }
}

pub unsafe fn vfunc_get_invoker(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::VFunc(VFunc::GioFileReadAsync)) => {
            create_info(InfoKind::Callable(Callable::GioFileReadAsync))
        }
        Some(InfoKind::VFunc(VFunc::GioAppLaunchContextGetDisplay)) => {
            create_info(InfoKind::Callable(Callable::GioAppLaunchContextGetDisplay))
        }
        _ => ptr::null_mut(),
    }
}
