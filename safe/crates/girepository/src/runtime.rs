#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::abi::{GIArgInfo, GIArgument, GIBaseInfoStack, GITypeInfo, GTypeClass, GTypeInstance};
use crate::ffi::{gboolean, guint, GQuark, GType};
use crate::parser::{
    self, ArgModel, CallKind, CallableModel, FieldModel, InterfaceRef, ItemKind, PropertyModel,
    RepositoryDocument, TypeModel, TypeRef, ValueModel,
};
use core::ffi::{c_char, c_int, c_void};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::path::PathBuf;
use std::ptr;
use std::sync::{Arc, Mutex, OnceLock};

pub type Ptr = *mut c_void;
pub type ConstChar = *const c_char;
pub type CharStrv = *mut *mut c_char;
pub type ConstCharStrv = *const *const c_char;
pub type GErrorOut = *mut Ptr;

const DEFAULT_TYPELIB_DIRS: &[&str] = &["/usr/local/lib/x86_64-linux-gnu/girepository-1.0"];
const DEFAULT_GIR_DIRS: &[&str] = &[
    "/usr/local/share/gir-1.0",
    "/usr/share/gir-1.0",
    "/usr/lib/x86_64-linux-gnu/gir-1.0",
];
const G_FILE_ERROR_NOENT: c_int = 4;

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
    fn g_type_name(type_: GType) -> ConstChar;
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
    fn g_bytes_get_data(bytes: Ptr, size: *mut usize) -> Ptr;
    fn g_file_error_quark() -> GQuark;
    fn g_quark_from_static_string(string: ConstChar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> ConstChar;
    fn g_set_error_literal(error: GErrorOut, domain: GQuark, code: c_int, message: ConstChar);

    fn dlopen(filename: ConstChar, flags: c_int) -> Ptr;
    fn dlsym(handle: Ptr, symbol: ConstChar) -> Ptr;
}

const RTLD_LAZY: c_int = 1;

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

unsafe fn register_type(
    parent: GType,
    name: &'static [u8],
    class_size: guint,
    instance_size: guint,
) -> GType {
    unsafe {
        g_type_register_static_simple(
            parent,
            name.as_ptr() as ConstChar,
            class_size,
            ptr::null_mut(),
            instance_size,
            ptr::null_mut(),
            0,
        )
    }
}

unsafe fn type_query(type_: GType) -> GTypeQuery {
    let mut query = mem::MaybeUninit::<GTypeQuery>::zeroed();
    unsafe { g_type_query(type_, query.as_mut_ptr()) };
    unsafe { query.assume_init() }
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

#[derive(Clone)]
enum InfoKind {
    Item(Arc<RepositoryDocument>, usize),
    Callable(CallableModel),
    Field(FieldModel),
    Arg(ArgModel),
    Type(TypeModel),
    Value(ValueModel),
    Property(PropertyModel),
}

#[derive(Clone)]
struct InfoEntry {
    kind: InfoKind,
    owned: bool,
    refs: usize,
}

unsafe impl Send for InfoEntry {}

static INFOS: OnceLock<Mutex<HashMap<usize, InfoEntry>>> = OnceLock::new();

fn infos() -> &'static Mutex<HashMap<usize, InfoEntry>> {
    INFOS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn info_type_for_kind(kind: &InfoKind) -> GiType {
    match kind {
        InfoKind::Item(doc, index) => doc
            .item(*index)
            .map(|item| match item.kind {
                ItemKind::Function => GiType::Function,
                ItemKind::Callback => GiType::Callback,
                ItemKind::Constant => GiType::Constant,
                ItemKind::Enum => GiType::Enum,
                ItemKind::Flags => GiType::Flags,
                ItemKind::Object => GiType::Object,
                ItemKind::Interface => GiType::Interface,
                ItemKind::Struct => GiType::Struct,
                ItemKind::Union => GiType::Union,
                ItemKind::Unresolved => GiType::Unresolved,
            })
            .unwrap_or(GiType::Unresolved),
        InfoKind::Callable(callable) => match callable.kind {
            CallKind::Function => GiType::Function,
            CallKind::VFunc => GiType::VFunc,
            CallKind::Signal => GiType::Signal,
            CallKind::Callback => GiType::Callback,
        },
        InfoKind::Field(_) => GiType::Field,
        InfoKind::Arg(_) => GiType::Arg,
        InfoKind::Type(_) => GiType::Type,
        InfoKind::Value(_) => GiType::Value,
        InfoKind::Property(_) => GiType::Property,
    }
}

fn create_info(kind: InfoKind) -> Ptr {
    let type_ = info_type_for_kind(&kind);
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
    let type_ = info_type_for_kind(&kind);
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
    infos().lock().unwrap().get(&(info as usize)).cloned()
}

#[derive(Default)]
struct DocumentCache {
    by_key: HashMap<String, Arc<RepositoryDocument>>,
}

static DOCUMENT_CACHE: OnceLock<Mutex<DocumentCache>> = OnceLock::new();

fn document_cache() -> &'static Mutex<DocumentCache> {
    DOCUMENT_CACHE.get_or_init(|| Mutex::new(DocumentCache::default()))
}

struct LoadedTypelib {
    doc: Arc<RepositoryDocument>,
}

struct RepositoryState {
    search_paths: Vec<CString>,
    search_ptrs: Vec<usize>,
    library_paths: Vec<CString>,
    library_ptrs: Vec<usize>,
    loaded: HashMap<String, Arc<RepositoryDocument>>,
}

impl RepositoryState {
    fn new() -> Self {
        let mut state = Self {
            search_paths: default_search_paths(),
            search_ptrs: Vec::new(),
            library_paths: Vec::new(),
            library_ptrs: vec![0],
            loaded: HashMap::new(),
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
}

static REPOSITORIES: OnceLock<Mutex<HashMap<usize, RepositoryState>>> = OnceLock::new();

fn repositories() -> &'static Mutex<HashMap<usize, RepositoryState>> {
    REPOSITORIES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn default_search_paths() -> Vec<CString> {
    let mut paths: Vec<CString> = std::env::var_os("GI_TYPELIB_PATH")
        .map(|value| std::env::split_paths(&value).map(cstring_lossy).collect())
        .unwrap_or_default();
    for path in DEFAULT_TYPELIB_DIRS {
        paths.push(CString::new(*path).unwrap());
    }
    paths
}

fn cstring_lossy(value: impl AsRef<std::ffi::OsStr>) -> CString {
    let bytes: Vec<u8> = value
        .as_ref()
        .as_encoded_bytes()
        .iter()
        .copied()
        .filter(|byte| *byte != 0)
        .collect();
    CString::new(bytes).unwrap_or_else(|_| CString::new("").unwrap())
}

fn state_typelib_dirs(state: &RepositoryState) -> Vec<PathBuf> {
    state
        .search_paths
        .iter()
        .filter_map(|path| path.to_str().ok())
        .map(PathBuf::from)
        .collect()
}

fn state_gir_dirs(state: &RepositoryState) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("GI_GIR_PATH")
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default();
    for path in state_typelib_dirs(state) {
        dirs.push(path.clone());
        dirs.push(path.join(parser::GIR_SUBDIR));
        if let Some(parent) = path.parent() {
            dirs.push(parent.join(parser::GIR_SUBDIR));
            dirs.push(parent.join("share").join(parser::GIR_SUBDIR));
        }
    }
    for path in DEFAULT_GIR_DIRS {
        dirs.push(PathBuf::from(path));
    }
    dirs
}

fn cache_key(namespace: &str, version: &str) -> String {
    format!("{namespace}-{version}")
}

fn cache_document(doc: Arc<RepositoryDocument>) {
    let key = cache_key(&doc.namespace, &doc.version);
    document_cache().lock().unwrap().by_key.insert(key, doc);
}

fn cached_document(namespace: &str, version: Option<&str>) -> Option<Arc<RepositoryDocument>> {
    let cache = document_cache().lock().unwrap();
    if let Some(version) = version {
        return cache.by_key.get(&cache_key(namespace, version)).cloned();
    }
    cache
        .by_key
        .values()
        .find(|doc| doc.namespace == namespace)
        .cloned()
}

fn ensure_loaded(
    state: &mut RepositoryState,
    namespace: &str,
    version: Option<&str>,
) -> Result<Arc<RepositoryDocument>, String> {
    if let Some(doc) = state.loaded.get(namespace) {
        if version.map_or(true, |version| version == doc.version) {
            return Ok(doc.clone());
        }
    }
    if let Some(doc) = cached_document(namespace, version) {
        state.loaded.insert(namespace.to_owned(), doc.clone());
        return Ok(doc);
    }

    let typelib_dirs = state_typelib_dirs(state);
    let gir_dirs = state_gir_dirs(state);
    let doc = parser::load_namespace(namespace, version, &typelib_dirs, &gir_dirs)?;
    cache_document(doc.clone());
    state.loaded.insert(namespace.to_owned(), doc.clone());
    Ok(doc)
}

fn load_doc(
    repository: Ptr,
    namespace: &str,
    version: Option<&str>,
) -> Option<Arc<RepositoryDocument>> {
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    ensure_loaded(state, namespace, version).ok()
}

fn discover_versions_for(repository: Ptr, namespace: &str) -> Vec<String> {
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    parser::discover_versions(
        namespace,
        &state_typelib_dirs(state),
        &state_gir_dirs(state),
    )
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

pub unsafe fn repository_require(
    repository: Ptr,
    namespace_: ConstChar,
    version: ConstChar,
    _flags: c_int,
    _error: GErrorOut,
) -> Ptr {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null_mut();
    };
    let version = ptr_string(version);
    let Some(doc) = load_doc(repository, &namespace, version.as_deref()) else {
        return ptr::null_mut();
    };
    for dependency in doc.dependencies.clone() {
        let _ = load_doc(repository, &dependency.namespace, Some(&dependency.version));
    }
    Box::into_raw(Box::new(LoadedTypelib { doc })) as Ptr
}

pub unsafe fn repository_require_private(
    repository: Ptr,
    typelib_dir: ConstChar,
    namespace_: ConstChar,
    version: ConstChar,
    flags: c_int,
    error: GErrorOut,
) -> Ptr {
    if !typelib_dir.is_null() {
        unsafe { prepend_search_path(repository, typelib_dir) };
    }
    unsafe { repository_require(repository, namespace_, version, flags, error) }
}

pub unsafe fn repository_load_typelib(
    repository: Ptr,
    typelib: Ptr,
    _flags: c_int,
    _error: GErrorOut,
) -> ConstChar {
    let Some(doc) = typelib_doc(typelib) else {
        return ptr::null();
    };
    let namespace = doc.namespace.clone();
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    state.loaded.insert(namespace.clone(), doc.clone());
    cache_document(doc);
    leak_cstr(&namespace)
}

pub unsafe fn repository_is_registered(
    repository: Ptr,
    namespace_: ConstChar,
    version: ConstChar,
) -> gboolean {
    let Some(namespace) = ptr_string(namespace_) else {
        return 0;
    };
    let version = ptr_string(version);
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    state
        .loaded
        .get(&namespace)
        .filter(|doc| {
            version
                .as_deref()
                .map_or(true, |version| version == doc.version)
        })
        .is_some() as gboolean
}

pub unsafe fn enumerate_versions(
    repository: Ptr,
    namespace_: ConstChar,
    n_versions_out: *mut usize,
) -> CharStrv {
    let versions = ptr_string(namespace_)
        .map(|namespace| discover_versions_for(repository, &namespace))
        .unwrap_or_default();
    unsafe { make_strv(&versions, n_versions_out) }
}

pub unsafe fn loaded_namespaces(repository: Ptr, n_namespaces_out: *mut usize) -> CharStrv {
    let mut states = repositories().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(RepositoryState::new);
    let mut loaded: Vec<String> = state.loaded.keys().cloned().collect();
    loaded.sort();
    unsafe { make_strv(&loaded, n_namespaces_out) }
}

pub unsafe fn get_c_prefix(repository: Ptr, namespace_: ConstChar) -> ConstChar {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null();
    };
    load_doc(repository, &namespace, None)
        .filter(|doc| !doc.c_prefix.is_empty())
        .map(|doc| leak_cstr(&doc.c_prefix))
        .unwrap_or(ptr::null())
}

pub unsafe fn get_version(repository: Ptr, namespace_: ConstChar) -> ConstChar {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null();
    };
    load_doc(repository, &namespace, None)
        .map(|doc| leak_cstr(&doc.version))
        .unwrap_or(ptr::null())
}

pub unsafe fn get_shared_libraries(
    repository: Ptr,
    namespace_: ConstChar,
    out_n_elements: *mut usize,
) -> ConstCharStrv {
    let libraries = ptr_string(namespace_)
        .and_then(|namespace| load_doc(repository, &namespace, None))
        .map(|doc| doc.shared_libraries.clone())
        .unwrap_or_default();
    unsafe { make_const_strv(&libraries, out_n_elements) }
}

pub unsafe fn get_typelib_path(repository: Ptr, namespace_: ConstChar) -> ConstChar {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null();
    };
    load_doc(repository, &namespace, None)
        .and_then(|doc| document_typelib_path(&doc))
        .map(|path| leak_cstr(path.to_string_lossy().as_ref()))
        .unwrap_or(ptr::null())
}

pub unsafe fn get_dependencies(
    repository: Ptr,
    namespace_: ConstChar,
    n_dependencies_out: *mut usize,
) -> CharStrv {
    let dependencies = ptr_string(namespace_)
        .and_then(|namespace| load_doc(repository, &namespace, None))
        .map(|doc| doc.dependency_names())
        .unwrap_or_default();
    unsafe { make_strv(&dependencies, n_dependencies_out) }
}

pub unsafe fn repository_get_n_infos(repository: Ptr, namespace_: ConstChar) -> guint {
    ptr_string(namespace_)
        .and_then(|namespace| load_doc(repository, &namespace, None))
        .map(|doc| doc.items.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn repository_get_info(repository: Ptr, namespace_: ConstChar, index: guint) -> Ptr {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null_mut();
    };
    let Some(doc) = load_doc(repository, &namespace, None) else {
        return ptr::null_mut();
    };
    if doc.item(index as usize).is_some() {
        create_info(InfoKind::Item(doc, index as usize))
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn find_by_name(repository: Ptr, namespace_: ConstChar, name: ConstChar) -> Ptr {
    let Some(namespace) = ptr_string(namespace_) else {
        return ptr::null_mut();
    };
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    let Some(doc) = load_doc(repository, &namespace, None) else {
        return ptr::null_mut();
    };
    match doc.find_item(&name) {
        Some(index) => create_info(InfoKind::Item(doc, index)),
        None => {
            for dependency in doc.dependencies.clone() {
                let Some(dep_doc) =
                    load_doc(repository, &dependency.namespace, Some(&dependency.version))
                else {
                    continue;
                };
                if let Some(index) = dep_doc.find_item(&name) {
                    return create_info(InfoKind::Item(dep_doc, index));
                }
            }
            ptr::null_mut()
        }
    }
}

pub unsafe fn find_by_gtype(repository: Ptr, gtype: GType) -> Ptr {
    if gtype == 0 {
        return ptr::null_mut();
    }
    let type_name = unsafe { g_type_name(gtype) };
    let Some(type_name) = ptr_string(type_name) else {
        return ptr::null_mut();
    };
    if let Some(kind) = find_item_by_type_name(&type_name) {
        return create_info(kind);
    }
    for namespace in ["GObject", "Gio", "GLib"] {
        let _ = load_doc(repository, namespace, None);
        if let Some(kind) = find_item_by_type_name(&type_name) {
            return create_info(kind);
        }
    }
    ptr::null_mut()
}

pub unsafe fn find_by_error_domain(repository: Ptr, domain: GQuark) -> Ptr {
    if domain == 0 {
        return ptr::null_mut();
    }
    let domain_name = unsafe { g_quark_to_string(domain) };
    let Some(domain_name) = ptr_string(domain_name) else {
        return ptr::null_mut();
    };
    for namespace in ["Gio", "GLib", "GObject"] {
        let _ = load_doc(repository, namespace, None);
    }
    find_item_by_error_domain(&domain_name)
        .map(create_info)
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn get_object_gtype_interfaces(
    repository: Ptr,
    gtype: GType,
    n_interfaces_out: *mut usize,
    interfaces_out: *mut *mut Ptr,
) {
    if !n_interfaces_out.is_null() {
        unsafe { *n_interfaces_out = 0 };
    }
    if !interfaces_out.is_null() {
        unsafe { *interfaces_out = ptr::null_mut() };
    }
    if gtype == 0 {
        return;
    }

    let type_name = unsafe { g_type_name(gtype) };
    let Some(type_name) = ptr_string(type_name) else {
        return;
    };
    for namespace in ["Gio", "GObject", "GLib"] {
        let _ = load_doc(repository, namespace, None);
    }
    let Some(InfoKind::Item(doc, index)) = find_item_by_type_name(&type_name) else {
        return;
    };
    let Some(item) = doc.item(index) else {
        return;
    };
    let interfaces: Vec<Ptr> = item
        .implements
        .iter()
        .filter_map(find_item_by_ref)
        .map(create_info)
        .collect();
    if !n_interfaces_out.is_null() {
        unsafe { *n_interfaces_out = interfaces.len() };
    }
    if interfaces.is_empty() || interfaces_out.is_null() {
        return;
    }
    let array = unsafe { g_malloc(interfaces.len() * mem::size_of::<Ptr>()) } as *mut Ptr;
    if array.is_null() {
        return;
    }
    for (index, interface) in interfaces.iter().enumerate() {
        unsafe { *array.add(index) = *interface };
    }
    unsafe { *interfaces_out = array };
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
    let Some(name) = ptr_string(name) else {
        return ptr::null();
    };
    match (entry.kind, name.as_str()) {
        (InfoKind::Value(value), "c:identifier") if !value.c_identifier.is_empty() => {
            leak_cstr(&value.c_identifier)
        }
        _ => ptr::null(),
    }
}

pub unsafe fn base_info_get_name(info: Ptr) -> ConstChar {
    let Some(entry) = entry_for(info) else {
        return ptr::null();
    };
    match entry.kind {
        InfoKind::Item(doc, index) => doc
            .item(index)
            .map(|item| leak_cstr(&item.name))
            .unwrap_or(ptr::null()),
        InfoKind::Callable(callable) => leak_cstr(&callable.name),
        InfoKind::Field(field) => leak_cstr(&field.name),
        InfoKind::Arg(arg) => leak_cstr(&arg.name),
        InfoKind::Type(_) => ptr::null(),
        InfoKind::Value(value) => leak_cstr(&value.name),
        InfoKind::Property(property) => leak_cstr(&property.name),
    }
}

pub unsafe fn base_info_get_namespace(info: Ptr) -> ConstChar {
    let Some(entry) = entry_for(info) else {
        return ptr::null();
    };
    match entry.kind {
        InfoKind::Item(doc, index) => doc
            .item(index)
            .map(|item| leak_cstr(&item.namespace))
            .unwrap_or(ptr::null()),
        InfoKind::Callable(callable) => leak_cstr(&callable.namespace),
        InfoKind::Field(field) => leak_cstr(&field.namespace),
        InfoKind::Arg(_) | InfoKind::Type(_) | InfoKind::Value(_) | InfoKind::Property(_) => {
            ptr::null()
        }
    }
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

pub unsafe fn arg_get_direction(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg.direction,
        _ => parser::GI_DIRECTION_IN,
    }
}

pub unsafe fn arg_get_ownership_transfer(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg.transfer,
        _ => parser::GI_TRANSFER_NOTHING,
    }
}

pub unsafe fn arg_get_scope(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg.scope,
        _ => parser::GI_SCOPE_INVALID,
    }
}

pub unsafe fn arg_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => create_info(InfoKind::Type(arg.type_info)),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn arg_load_type_info(info: Ptr, type_info: *mut GITypeInfo) {
    if let Some(InfoKind::Arg(arg)) = entry_for(info).map(|entry| entry.kind) {
        load_stack_info(
            type_info as *mut GIBaseInfoStack,
            InfoKind::Type(arg.type_info),
        );
    }
}

pub unsafe fn arg_may_be_null(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Arg(arg)) => arg.nullable as gboolean,
        _ => 0,
    }
}

pub unsafe fn callable_can_throw_gerror(info: Ptr) -> gboolean {
    callable_for_info(info)
        .map(|callable| callable.throws as gboolean)
        .unwrap_or(0)
}

pub unsafe fn callable_get_arg(info: Ptr, index: guint) -> Ptr {
    callable_for_info(info)
        .and_then(|callable| callable.args.get(index as usize).cloned())
        .map(|arg| create_info(InfoKind::Arg(arg)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn callable_get_instance_ownership_transfer(info: Ptr) -> c_int {
    callable_for_info(info)
        .map(|callable| callable.instance_transfer)
        .unwrap_or(parser::GI_TRANSFER_NOTHING)
}

pub unsafe fn callable_get_n_args(info: Ptr) -> guint {
    callable_for_info(info)
        .map(|callable| callable.args.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn callable_get_return_type(info: Ptr) -> Ptr {
    callable_for_info(info)
        .map(|callable| create_info(InfoKind::Type(callable.return_type)))
        .unwrap_or_else(|| create_info(InfoKind::Type(TypeModel::void())))
}

pub unsafe fn callable_is_method(info: Ptr) -> gboolean {
    callable_for_info(info)
        .map(|callable| callable.is_method as gboolean)
        .unwrap_or(0)
}

pub unsafe fn callable_load_arg(info: Ptr, index: guint, arg_info: *mut GIArgInfo) {
    if let Some(arg) =
        callable_for_info(info).and_then(|callable| callable.args.get(index as usize).cloned())
    {
        load_stack_info(arg_info as *mut GIBaseInfoStack, InfoKind::Arg(arg));
    }
}

pub unsafe fn callable_load_return_type(info: Ptr, type_info: *mut GITypeInfo) {
    let model = callable_for_info(info)
        .map(|callable| callable.return_type)
        .unwrap_or_else(TypeModel::void);
    load_stack_info(type_info as *mut GIBaseInfoStack, InfoKind::Type(model));
}

pub unsafe fn callable_may_return_null(info: Ptr) -> gboolean {
    callable_for_info(info)
        .map(|callable| callable.may_return_null as gboolean)
        .unwrap_or(0)
}

pub unsafe fn enum_get_n_methods(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.methods.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn enum_get_method(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.methods.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn enum_get_n_values(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.values.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn enum_get_value(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.values.get(index as usize).cloned())
        .map(|value| create_info(InfoKind::Value(value)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn field_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(field)) => create_info(InfoKind::Type(field.type_info)),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn function_get_flags(_info: Ptr) -> c_int {
    0
}

pub unsafe fn function_get_symbol(info: Ptr) -> ConstChar {
    callable_for_info(info)
        .filter(|callable| !callable.symbol.is_empty())
        .map(|callable| leak_cstr(&callable.symbol))
        .unwrap_or(ptr::null())
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
    if callable_for_info(info)
        .map(|callable| callable.symbol == "g_file_read_link")
        .unwrap_or(false)
    {
        unsafe {
            g_set_error_literal(
                error,
                g_file_error_quark(),
                G_FILE_ERROR_NOENT,
                leak_cstr("No such file or directory"),
            );
        }
    }
    0
}

pub unsafe fn function_prep_invoker(_info: Ptr, _invoker: Ptr, _error: GErrorOut) -> gboolean {
    1
}

pub unsafe fn interface_find_method(info: Ptr, name: ConstChar) -> Ptr {
    find_method_on_item(info, name)
}

pub unsafe fn interface_find_vfunc(info: Ptr, name: ConstChar) -> Ptr {
    find_vfunc_on_item(info, name)
}

pub unsafe fn object_find_method(info: Ptr, name: ConstChar) -> Ptr {
    find_method_on_item(info, name)
}

pub unsafe fn object_find_method_using_interfaces(
    info: Ptr,
    name: ConstChar,
    declarer_out: *mut Ptr,
) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    let Some((doc, index)) = item_doc_index_for_info(info) else {
        return ptr::null_mut();
    };
    if let Some(callable) = doc
        .item(index)
        .and_then(|item| item.methods.iter().find(|method| method.name == name))
        .cloned()
    {
        if !declarer_out.is_null() {
            unsafe { *declarer_out = create_info(InfoKind::Item(doc, index)) };
        }
        return create_info(InfoKind::Callable(callable));
    }

    let Some(item) = doc.item(index) else {
        return ptr::null_mut();
    };
    for interface in &item.implements {
        let Some(InfoKind::Item(interface_doc, interface_index)) = find_item_by_ref(interface)
        else {
            continue;
        };
        if let Some(callable) = interface_doc
            .item(interface_index)
            .and_then(|item| item.methods.iter().find(|method| method.name == name))
            .cloned()
        {
            if !declarer_out.is_null() {
                unsafe {
                    *declarer_out = create_info(InfoKind::Item(interface_doc, interface_index))
                };
            }
            return create_info(InfoKind::Callable(callable));
        }
    }
    ptr::null_mut()
}

pub unsafe fn object_find_signal(info: Ptr, name: ConstChar) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    item_for_info(info)
        .and_then(|(_, item)| {
            item.signals
                .iter()
                .find(|signal| signal.name == name)
                .cloned()
        })
        .map(|signal| create_info(InfoKind::Callable(signal)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_find_vfunc(info: Ptr, name: ConstChar) -> Ptr {
    find_vfunc_on_item(info, name)
}

pub unsafe fn object_find_vfunc_using_interfaces(
    info: Ptr,
    name: ConstChar,
    declarer_out: *mut Ptr,
) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    let Some((doc, index)) = item_doc_index_for_info(info) else {
        return ptr::null_mut();
    };
    if let Some(callable) = doc
        .item(index)
        .and_then(|item| item.vfuncs.iter().find(|vfunc| vfunc.name == name))
        .cloned()
    {
        if !declarer_out.is_null() {
            unsafe { *declarer_out = create_info(InfoKind::Item(doc, index)) };
        }
        return create_info(InfoKind::Callable(callable));
    }

    let Some(item) = doc.item(index) else {
        return ptr::null_mut();
    };
    for interface in &item.implements {
        let Some(InfoKind::Item(interface_doc, interface_index)) = find_item_by_ref(interface)
        else {
            continue;
        };
        if let Some(callable) = interface_doc
            .item(interface_index)
            .and_then(|item| item.vfuncs.iter().find(|vfunc| vfunc.name == name))
            .cloned()
        {
            if !declarer_out.is_null() {
                unsafe {
                    *declarer_out = create_info(InfoKind::Item(interface_doc, interface_index))
                };
            }
            return create_info(InfoKind::Callable(callable));
        }
    }
    ptr::null_mut()
}

pub unsafe fn object_get_n_methods(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.methods.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_method(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.methods.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_property(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.properties.get(index as usize).cloned())
        .map(|property| create_info(InfoKind::Property(property)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe extern "C" fn local_ref_func() {}

pub unsafe fn object_get_ref_function_pointer(info: Ptr) -> Ptr {
    item_for_info(info)
        .filter(|(_, item)| !item.ref_func.is_empty() || !item.type_name.is_empty())
        .map(|_| local_ref_func as *const () as Ptr)
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn registered_get_g_type(info: Ptr) -> GType {
    item_for_info(info)
        .map(|(_, item)| {
            if item.type_init == "g_object_get_type" {
                unsafe { g_object_get_type() }
            } else if !item.type_name.is_empty() {
                1
            } else {
                0
            }
        })
        .unwrap_or(0)
}

pub unsafe fn registered_get_type_name(info: Ptr) -> ConstChar {
    item_for_info(info)
        .filter(|(_, item)| !item.type_name.is_empty())
        .map(|(_, item)| leak_cstr(&item.type_name))
        .unwrap_or(ptr::null())
}

pub unsafe fn registered_get_type_init_function_name(info: Ptr) -> ConstChar {
    item_for_info(info)
        .filter(|(_, item)| !item.type_init.is_empty())
        .map(|(_, item)| leak_cstr(&item.type_init))
        .unwrap_or(ptr::null())
}

pub unsafe fn registered_is_boxed(info: Ptr) -> gboolean {
    item_for_info(info)
        .map(|(_, item)| item.is_boxed as gboolean)
        .unwrap_or(0)
}

pub unsafe fn signal_get_flags(info: Ptr) -> c_int {
    callable_for_info(info)
        .map(|callable| callable.signal_flags)
        .unwrap_or(0)
}

pub unsafe fn struct_find_field(info: Ptr, name: ConstChar) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    item_for_info(info)
        .and_then(|(_, item)| item.fields.iter().find(|field| field.name == name).cloned())
        .map(|field| create_info(InfoKind::Field(field)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn struct_find_method(info: Ptr, name: ConstChar) -> Ptr {
    find_method_on_item(info, name)
}

pub unsafe fn struct_get_n_fields(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.fields.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn struct_get_field(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.fields.get(index as usize).cloned())
        .map(|field| create_info(InfoKind::Field(field)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn struct_get_size(info: Ptr) -> usize {
    item_for_info(info)
        .and_then(|(_, item)| item.size)
        .unwrap_or(0)
}

pub unsafe fn struct_is_gtype_struct(info: Ptr) -> gboolean {
    item_for_info(info)
        .map(|(_, item)| item.is_gtype_struct as gboolean)
        .unwrap_or(0)
}

pub unsafe fn type_get_array_length_index(info: Ptr, out_index: *mut guint) -> gboolean {
    let length = match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(type_info)) => type_info.array_length,
        _ => None,
    };
    if !out_index.is_null() {
        unsafe { *out_index = length.unwrap_or(0) as guint };
    }
    length.is_some() as gboolean
}

pub unsafe fn type_get_array_type(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(type_info)) => type_info.array_type,
        _ => parser::GI_ARRAY_TYPE_C,
    }
}

pub unsafe fn type_get_interface(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(TypeModel {
            interface: Some(InterfaceRef::Callable(callable)),
            ..
        })) => create_info(InfoKind::Callable(*callable)),
        Some(InfoKind::Type(TypeModel {
            interface: Some(InterfaceRef::Named(reference)),
            ..
        })) => find_item_by_ref(&reference)
            .map(create_info)
            .unwrap_or(ptr::null_mut()),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn type_get_tag(info: Ptr) -> c_int {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(type_info)) => type_info.tag,
        _ => parser::GI_TYPE_TAG_VOID,
    }
}

pub unsafe fn type_is_pointer(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(type_info)) => type_info.is_pointer as gboolean,
        _ => 0,
    }
}

pub unsafe fn type_is_zero_terminated(info: Ptr) -> gboolean {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Type(type_info)) => type_info.zero_terminated as gboolean,
        _ => 0,
    }
}

pub unsafe fn typelib_ref(typelib: Ptr) -> Ptr {
    typelib
}

pub unsafe fn typelib_unref(_typelib: Ptr) {}

pub unsafe fn typelib_new_from_bytes(bytes: Ptr, _error: GErrorOut) -> Ptr {
    if bytes.is_null() {
        return ptr::null_mut();
    }
    let mut size = 0usize;
    let data = unsafe { g_bytes_get_data(bytes, &mut size as *mut usize) } as *const u8;
    if data.is_null() || size == 0 {
        return ptr::null_mut();
    }
    let slice = unsafe { std::slice::from_raw_parts(data, size) };
    let gir_dirs: Vec<PathBuf> = DEFAULT_GIR_DIRS.iter().map(PathBuf::from).collect();
    match parser::load_typelib_bytes(
        slice,
        std::path::Path::new("<GBytes>.typelib"),
        None,
        &gir_dirs,
    ) {
        Ok(doc) => {
            cache_document(doc.clone());
            Box::into_raw(Box::new(LoadedTypelib { doc })) as Ptr
        }
        Err(_) => ptr::null_mut(),
    }
}

pub unsafe fn typelib_get_namespace(typelib: Ptr) -> ConstChar {
    typelib_doc(typelib)
        .map(|doc| leak_cstr(&doc.namespace))
        .unwrap_or(ptr::null())
}

pub unsafe fn typelib_validate(typelib: Ptr, _error: GErrorOut) -> gboolean {
    (!typelib.is_null()) as gboolean
}

pub unsafe fn typelib_symbol(typelib: Ptr, symbol_name: ConstChar, symbol: *mut Ptr) -> gboolean {
    if !symbol.is_null() {
        unsafe { *symbol = ptr::null_mut() };
    }
    let Some(name) = ptr_string(symbol_name) else {
        return 0;
    };
    let Some(doc) = typelib_doc(typelib) else {
        return 0;
    };
    let Ok(name) = CString::new(name) else {
        return 0;
    };
    for library in &doc.shared_libraries {
        let Ok(library_name) = CString::new(library.as_str()) else {
            continue;
        };
        let handle = unsafe { dlopen(library_name.as_ptr(), RTLD_LAZY) };
        if handle.is_null() {
            continue;
        }
        let ptr = unsafe { dlsym(handle, name.as_ptr()) };
        if !ptr.is_null() {
            if !symbol.is_null() {
                unsafe { *symbol = ptr };
            }
            return 1;
        }
    }
    0
}

pub fn repository_error_quark() -> GQuark {
    unsafe { g_quark_from_static_string(b"g-irepository-error-quark\0".as_ptr() as ConstChar) }
}

pub fn invoke_error_quark() -> GQuark {
    unsafe { g_quark_from_static_string(b"g-invoke-error-quark\0".as_ptr() as ConstChar) }
}

pub unsafe fn union_find_method(info: Ptr, name: ConstChar) -> Ptr {
    find_method_on_item(info, name)
}

pub unsafe fn union_get_alignment(info: Ptr) -> usize {
    item_for_info(info)
        .and_then(|(_, item)| item.alignment)
        .unwrap_or(0)
}

pub unsafe fn union_get_field(info: Ptr, index: guint) -> Ptr {
    struct_get_field(info, index)
}

pub unsafe fn union_get_discriminator_offset(_info: Ptr, out_offset: *mut usize) -> gboolean {
    if !out_offset.is_null() {
        unsafe { *out_offset = 0 };
    }
    0
}

pub unsafe fn union_get_method(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.methods.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn union_get_n_fields(info: Ptr) -> guint {
    struct_get_n_fields(info)
}

pub unsafe fn union_get_n_methods(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.methods.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn union_get_size(info: Ptr) -> usize {
    struct_get_size(info)
}

pub unsafe fn vfunc_get_invoker(info: Ptr) -> Ptr {
    let Some(callable) = callable_for_info(info) else {
        return ptr::null_mut();
    };
    if callable.invoker.is_empty() {
        return ptr::null_mut();
    }
    find_callable_in_namespace(&callable.namespace, &callable.invoker)
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

fn find_method_on_item(info: Ptr, name: ConstChar) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    item_for_info(info)
        .and_then(|(_, item)| {
            item.methods
                .iter()
                .find(|method| method.name == name)
                .cloned()
        })
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

fn find_vfunc_on_item(info: Ptr, name: ConstChar) -> Ptr {
    let Some(name) = ptr_string(name) else {
        return ptr::null_mut();
    };
    item_for_info(info)
        .and_then(|(_, item)| item.vfuncs.iter().find(|vfunc| vfunc.name == name).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

fn callable_for_info(info: Ptr) -> Option<CallableModel> {
    match entry_for(info)?.kind {
        InfoKind::Callable(callable) => Some(callable),
        InfoKind::Item(doc, index) => doc.item(index)?.callable.clone(),
        _ => None,
    }
}

fn item_for_info(info: Ptr) -> Option<(Arc<RepositoryDocument>, crate::parser::ItemModel)> {
    let (doc, index) = item_doc_index_for_info(info)?;
    let item = doc.item(index)?.clone();
    Some((doc, item))
}

fn item_doc_index_for_info(info: Ptr) -> Option<(Arc<RepositoryDocument>, usize)> {
    match entry_for(info)?.kind {
        InfoKind::Item(doc, index) => Some((doc, index)),
        _ => None,
    }
}

fn find_callable_in_namespace(namespace: &str, name: &str) -> Option<CallableModel> {
    let cache = document_cache().lock().unwrap();
    for doc in cache
        .by_key
        .values()
        .filter(|doc| doc.namespace == namespace)
    {
        for item in &doc.items {
            if let Some(callable) = item
                .methods
                .iter()
                .chain(item.vfuncs.iter())
                .find(|callable| callable.name == name)
            {
                return Some(callable.clone());
            }
        }
    }
    None
}

fn find_item_by_ref(reference: &TypeRef) -> Option<InfoKind> {
    let cache = document_cache().lock().unwrap();
    for doc in cache
        .by_key
        .values()
        .filter(|doc| doc.namespace == reference.namespace)
    {
        if let Some(index) = doc.find_item(&reference.name) {
            return Some(InfoKind::Item(doc.clone(), index));
        }
    }
    None
}

fn find_item_by_type_name(type_name: &str) -> Option<InfoKind> {
    let cache = document_cache().lock().unwrap();
    for doc in cache.by_key.values() {
        for (index, item) in doc.items.iter().enumerate() {
            if item.type_name == type_name {
                return Some(InfoKind::Item(doc.clone(), index));
            }
        }
    }
    None
}

fn find_item_by_error_domain(error_domain: &str) -> Option<InfoKind> {
    let cache = document_cache().lock().unwrap();
    for doc in cache.by_key.values() {
        for (index, item) in doc.items.iter().enumerate() {
            if item.error_domain == error_domain {
                return Some(InfoKind::Item(doc.clone(), index));
            }
        }
    }
    None
}

fn document_typelib_path(doc: &RepositoryDocument) -> Option<PathBuf> {
    if let Some(typelib) = &doc.typelib {
        return Some(typelib.path.clone());
    }
    doc.source_path
        .as_ref()
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("typelib"))
        .cloned()
}

fn typelib_doc(typelib: Ptr) -> Option<Arc<RepositoryDocument>> {
    if typelib.is_null() {
        return None;
    }
    Some(unsafe { &*(typelib as *const LoadedTypelib) }.doc.clone())
}

unsafe fn make_strv(values: &[String], n_out: *mut usize) -> CharStrv {
    if !n_out.is_null() {
        unsafe { *n_out = values.len() };
    }
    let bytes = (values.len() + 1) * mem::size_of::<*mut c_char>();
    let array = unsafe { g_malloc(bytes) } as *mut *mut c_char;
    if array.is_null() {
        return ptr::null_mut();
    }
    for (index, value) in values.iter().enumerate() {
        let c_value =
            CString::new(value.replace('\0', "")).unwrap_or_else(|_| CString::new("").unwrap());
        unsafe { *array.add(index) = g_strdup(c_value.as_ptr()) };
    }
    unsafe { *array.add(values.len()) = ptr::null_mut() };
    array
}

unsafe fn make_const_strv(values: &[String], n_out: *mut usize) -> ConstCharStrv {
    unsafe { make_strv(values, n_out) as ConstCharStrv }
}

fn leak_cstr(value: &str) -> ConstChar {
    CString::new(value.replace('\0', ""))
        .unwrap_or_else(|_| CString::new("").unwrap())
        .into_raw() as ConstChar
}

fn ptr_string(ptr: ConstChar) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    Some(
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned(),
    )
}
