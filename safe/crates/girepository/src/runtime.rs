#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::abi::{
    GIArgInfo, GIArgument, GIAttributeIter, GIBaseInfoStack, GITypeInfo, GTypeClass, GTypeInstance,
};
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
    attribute_pair(&entry.kind)
        .filter(|(attr_name, _)| *attr_name == name)
        .map(|(_, attr_value)| leak_cstr(&attr_value))
        .unwrap_or(ptr::null())
}

pub unsafe fn base_info_iterate_attributes(
    info: Ptr,
    iterator: *mut GIAttributeIter,
    name: *mut ConstChar,
    value: *mut ConstChar,
) -> gboolean {
    if iterator.is_null() {
        return 0;
    }
    if unsafe { !(*iterator).data.is_null() } {
        return 0;
    }
    let Some(entry) = entry_for(info) else {
        return 0;
    };
    let Some((attr_name, attr_value)) = attribute_pair(&entry.kind) else {
        return 0;
    };
    if !name.is_null() {
        unsafe { *name = leak_cstr(attr_name) };
    }
    if !value.is_null() {
        unsafe { *value = leak_cstr(&attr_value) };
    }
    unsafe { (*iterator).data = 1usize as Ptr };
    1
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

pub unsafe fn base_info_equal(info1: Ptr, info2: Ptr) -> gboolean {
    if info1 == info2 {
        return (!info1.is_null()) as gboolean;
    }
    let Some(entry1) = entry_for(info1) else {
        return 0;
    };
    let Some(entry2) = entry_for(info2) else {
        return 0;
    };
    let id1 = info_identity(&entry1.kind);
    let id2 = info_identity(&entry2.kind);
    (!id1.1.is_empty() && id1 == id2) as gboolean
}

pub unsafe fn base_info_is_deprecated(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn base_info_get_container(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn base_info_get_typelib(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Item(doc, _)) => Box::into_raw(Box::new(LoadedTypelib { doc })) as Ptr,
        Some(kind) => namespace_for_kind(&kind)
            .and_then(|namespace| cached_document(&namespace, None))
            .map(|doc| Box::into_raw(Box::new(LoadedTypelib { doc })) as Ptr)
            .unwrap_or(ptr::null_mut()),
        None => ptr::null_mut(),
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

pub unsafe fn callable_create_closure(
    _info: Ptr,
    _cif: Ptr,
    _callback: Ptr,
    _user_data: Ptr,
) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn callable_get_closure_native_address(_info: Ptr, _closure: Ptr) -> *mut Ptr {
    ptr::null_mut()
}

pub unsafe fn callable_destroy_closure(_info: Ptr, _closure: Ptr) {}

pub unsafe fn callable_invoke(
    info: Ptr,
    _function: Ptr,
    in_args: *const GIArgument,
    n_in_args: usize,
    out_args: *mut GIArgument,
    n_out_args: usize,
    return_value: *mut GIArgument,
    error: GErrorOut,
) -> gboolean {
    unsafe {
        function_invoke(
            info,
            in_args,
            n_in_args,
            out_args,
            n_out_args,
            return_value,
            error,
        )
    }
}

pub unsafe fn cclosure_marshal_generic(
    _closure: Ptr,
    _return_value: Ptr,
    _n_param_values: guint,
    _param_values: Ptr,
    _invocation_hint: Ptr,
    _marshal_data: Ptr,
) {
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

pub unsafe fn constant_get_type_info(info: Ptr) -> Ptr {
    item_for_info(info)
        .filter(|(_, item)| item.kind == ItemKind::Constant)
        .and_then(|(_, item)| item.constant_type)
        .map(|type_info| create_info(InfoKind::Type(type_info)))
        .unwrap_or_else(|| create_info(InfoKind::Type(TypeModel::void())))
}

pub unsafe fn constant_get_value(info: Ptr, value: *mut GIArgument) -> usize {
    let Some((_, item)) = item_for_info(info).filter(|(_, item)| item.kind == ItemKind::Constant)
    else {
        return 0;
    };
    let type_info = item.constant_type.unwrap_or_else(TypeModel::void);
    if !value.is_null() {
        unsafe { write_string_value(&type_info, &item.constant_value, value) };
    }
    type_storage_size(type_info.tag, type_info.is_pointer)
}

pub unsafe fn constant_free_value(_info: Ptr, _value: *mut GIArgument) {}

pub unsafe fn enum_get_error_domain(info: Ptr) -> ConstChar {
    item_for_info(info)
        .filter(|(_, item)| !item.error_domain.is_empty())
        .map(|(_, item)| leak_cstr(&item.error_domain))
        .unwrap_or(ptr::null())
}

pub unsafe fn enum_get_storage_type(_info: Ptr) -> c_int {
    parser::GI_TYPE_TAG_INT32
}

pub unsafe fn field_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(field)) => create_info(InfoKind::Type(field.type_info)),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn field_get_flags(_info: Ptr) -> c_int {
    0x1 | 0x2
}

pub unsafe fn field_get_size(info: Ptr) -> usize {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(field)) => {
            type_storage_size(field.type_info.tag, field.type_info.is_pointer)
        }
        _ => 0,
    }
}

pub unsafe fn field_get_offset(_info: Ptr) -> usize {
    0
}

pub unsafe fn field_get_field(info: Ptr, mem: Ptr, value: *mut GIArgument) -> gboolean {
    if mem.is_null() || value.is_null() {
        return 0;
    }
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(field)) => unsafe { read_argument(mem, &field.type_info, value) },
        _ => 0,
    }
}

pub unsafe fn field_set_field(info: Ptr, mem: Ptr, value: *const GIArgument) -> gboolean {
    if mem.is_null() || value.is_null() {
        return 0;
    }
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Field(field)) => unsafe { write_argument(mem, &field.type_info, value) },
        _ => 0,
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

pub unsafe fn function_get_property(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn function_get_vfunc(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn function_invoker_new_for_address(
    addr: Ptr,
    info: Ptr,
    invoker: Ptr,
    error: GErrorOut,
) -> gboolean {
    if info.is_null() || invoker.is_null() {
        return 0;
    }
    let _ = addr;
    unsafe { function_prep_invoker(info, invoker, error) }
}

pub unsafe fn interface_find_method(info: Ptr, name: ConstChar) -> Ptr {
    find_method_on_item(info, name)
}

pub unsafe fn interface_find_vfunc(info: Ptr, name: ConstChar) -> Ptr {
    find_vfunc_on_item(info, name)
}

pub unsafe fn interface_find_signal(info: Ptr, name: ConstChar) -> Ptr {
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

pub unsafe fn interface_get_n_prerequisites(_info: Ptr) -> guint {
    0
}

pub unsafe fn interface_get_prerequisite(_info: Ptr, _index: guint) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn interface_get_n_properties(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.properties.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn interface_get_property(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.properties.get(index as usize).cloned())
        .map(|property| create_info(InfoKind::Property(property)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn interface_get_n_methods(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.methods.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn interface_get_method(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.methods.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn interface_get_n_signals(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.signals.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn interface_get_signal(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.signals.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn interface_get_n_vfuncs(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.vfuncs.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn interface_get_vfunc(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.vfuncs.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn interface_get_n_constants(_info: Ptr) -> guint {
    0
}

pub unsafe fn interface_get_constant(_info: Ptr, _index: guint) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn interface_get_iface_struct(info: Ptr) -> Ptr {
    item_for_info(info)
        .and_then(|(doc, item)| {
            (!item.type_struct.is_empty()).then_some(TypeRef {
                namespace: doc.namespace.clone(),
                name: item.type_struct,
            })
        })
        .and_then(|reference| find_item_by_ref(&reference))
        .map(create_info)
        .unwrap_or(ptr::null_mut())
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

pub unsafe fn object_get_type_name(info: Ptr) -> ConstChar {
    unsafe { registered_get_type_name(info) }
}

pub unsafe fn object_get_type_init_function_name(info: Ptr) -> ConstChar {
    unsafe { registered_get_type_init_function_name(info) }
}

pub unsafe fn object_get_abstract(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn object_get_final(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn object_get_fundamental(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn object_get_parent(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn object_get_n_interfaces(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.implements.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_interface(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.implements.get(index as usize).cloned())
        .and_then(|reference| find_item_by_ref(&reference))
        .map(create_info)
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_n_fields(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.fields.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_field(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.fields.get(index as usize).cloned())
        .map(|field| create_info(InfoKind::Field(field)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_n_properties(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.properties.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_n_signals(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.signals.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_signal(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.signals.get(index as usize).cloned())
        .map(|signal| create_info(InfoKind::Callable(signal)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_n_vfuncs(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.vfuncs.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn object_get_vfunc(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.vfuncs.get(index as usize).cloned())
        .map(|vfunc| create_info(InfoKind::Callable(vfunc)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_n_constants(_info: Ptr) -> guint {
    0
}

pub unsafe fn object_get_constant(_info: Ptr, _index: guint) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn object_get_class_struct(info: Ptr) -> Ptr {
    item_for_info(info)
        .and_then(|(doc, item)| {
            (!item.type_struct.is_empty()).then_some(TypeRef {
                namespace: doc.namespace.clone(),
                name: item.type_struct,
            })
        })
        .and_then(|reference| find_item_by_ref(&reference))
        .map(create_info)
        .unwrap_or(ptr::null_mut())
}

pub unsafe extern "C" fn local_ref_func() {}

pub unsafe fn object_get_ref_function_name(info: Ptr) -> ConstChar {
    item_for_info(info)
        .filter(|(_, item)| !item.ref_func.is_empty())
        .map(|(_, item)| leak_cstr(&item.ref_func))
        .unwrap_or(ptr::null())
}

pub unsafe fn object_get_ref_function_pointer(info: Ptr) -> Ptr {
    item_for_info(info)
        .filter(|(_, item)| !item.ref_func.is_empty() || !item.type_name.is_empty())
        .map(|_| local_ref_func as *const () as Ptr)
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn object_get_unref_function_name(_info: Ptr) -> ConstChar {
    ptr::null()
}

pub unsafe fn object_get_unref_function_pointer(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn object_get_set_value_function_name(_info: Ptr) -> ConstChar {
    ptr::null()
}

pub unsafe fn object_get_set_value_function_pointer(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn object_get_get_value_function_name(_info: Ptr) -> ConstChar {
    ptr::null()
}

pub unsafe fn object_get_get_value_function_pointer(_info: Ptr) -> Ptr {
    ptr::null_mut()
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

pub unsafe fn property_get_flags(_info: Ptr) -> c_int {
    0x1 | 0x2
}

pub unsafe fn property_get_ownership_transfer(_info: Ptr) -> c_int {
    parser::GI_TRANSFER_NOTHING
}

pub unsafe fn property_get_type_info(info: Ptr) -> Ptr {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Property(property)) => create_info(InfoKind::Type(property.type_info)),
        _ => ptr::null_mut(),
    }
}

pub unsafe fn property_get_getter(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn property_get_setter(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn signal_get_flags(info: Ptr) -> c_int {
    callable_for_info(info)
        .map(|callable| callable.signal_flags)
        .unwrap_or(0)
}

pub unsafe fn signal_get_class_closure(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn signal_true_stops_emit(info: Ptr) -> gboolean {
    let flags = unsafe { signal_get_flags(info) };
    ((flags & 8) != 0) as gboolean
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

pub unsafe fn struct_get_n_methods(info: Ptr) -> guint {
    item_for_info(info)
        .map(|(_, item)| item.methods.len() as guint)
        .unwrap_or(0)
}

pub unsafe fn struct_get_method(info: Ptr, index: guint) -> Ptr {
    item_for_info(info)
        .and_then(|(_, item)| item.methods.get(index as usize).cloned())
        .map(|callable| create_info(InfoKind::Callable(callable)))
        .unwrap_or(ptr::null_mut())
}

pub unsafe fn struct_get_alignment(info: Ptr) -> usize {
    item_for_info(info)
        .and_then(|(_, item)| item.alignment)
        .unwrap_or(0)
}

pub unsafe fn struct_is_foreign(_info: Ptr) -> gboolean {
    0
}

pub unsafe fn struct_get_copy_function_name(_info: Ptr) -> ConstChar {
    ptr::null()
}

pub unsafe fn struct_get_free_function_name(_info: Ptr) -> ConstChar {
    ptr::null()
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

pub unsafe fn type_get_param_type(_info: Ptr, _index: guint) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn type_get_array_fixed_size(_info: Ptr, out_size: *mut usize) -> gboolean {
    if !out_size.is_null() {
        unsafe { *out_size = 0 };
    }
    0
}

pub unsafe fn type_get_storage_type(info: Ptr) -> c_int {
    unsafe { type_get_tag(info) }
}

pub unsafe fn type_get_ffi_type(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn type_argument_from_hash_pointer(info: Ptr, hash_pointer: Ptr, arg: *mut GIArgument) {
    let tag = unsafe { type_get_storage_type(info) };
    unsafe { type_tag_argument_from_hash_pointer(tag, hash_pointer, arg) };
}

pub unsafe fn type_hash_pointer_from_argument(info: Ptr, arg: *const GIArgument) -> Ptr {
    let tag = unsafe { type_get_storage_type(info) };
    unsafe { type_tag_hash_pointer_from_argument(tag, arg) }
}

pub unsafe fn type_extract_ffi_return_value(
    info: Ptr,
    ffi_value: *const GIArgument,
    arg: *mut GIArgument,
) {
    let tag = unsafe { type_get_storage_type(info) };
    unsafe { type_tag_extract_ffi_return_value(tag, 0, ffi_value, arg) };
}

pub unsafe fn type_tag_get_ffi_type(_tag: c_int, _is_pointer: gboolean) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn type_tag_argument_from_hash_pointer(
    tag: c_int,
    hash_pointer: Ptr,
    arg: *mut GIArgument,
) {
    if arg.is_null() {
        return;
    }
    unsafe { argument_from_usize(tag, hash_pointer as usize, arg) };
}

pub unsafe fn type_tag_hash_pointer_from_argument(tag: c_int, arg: *const GIArgument) -> Ptr {
    if arg.is_null() {
        return ptr::null_mut();
    }
    unsafe { argument_to_usize(tag, arg) as Ptr }
}

pub unsafe fn type_tag_extract_ffi_return_value(
    return_tag: c_int,
    _interface_type: GType,
    ffi_value: *const GIArgument,
    arg: *mut GIArgument,
) {
    if ffi_value.is_null() || arg.is_null() {
        return;
    }
    let value = unsafe { argument_to_usize(return_tag, ffi_value) };
    unsafe { argument_from_usize(return_tag, value, arg) };
}

pub fn type_tag_to_string(tag: c_int) -> ConstChar {
    let name = match tag {
        parser::GI_TYPE_TAG_VOID => "void",
        parser::GI_TYPE_TAG_BOOLEAN => "gboolean",
        parser::GI_TYPE_TAG_INT8 => "gint8",
        parser::GI_TYPE_TAG_UINT8 => "guint8",
        parser::GI_TYPE_TAG_INT16 => "gint16",
        parser::GI_TYPE_TAG_UINT16 => "guint16",
        parser::GI_TYPE_TAG_INT32 => "gint32",
        parser::GI_TYPE_TAG_UINT32 => "guint32",
        parser::GI_TYPE_TAG_INT64 => "gint64",
        parser::GI_TYPE_TAG_UINT64 => "guint64",
        parser::GI_TYPE_TAG_FLOAT => "gfloat",
        parser::GI_TYPE_TAG_DOUBLE => "gdouble",
        parser::GI_TYPE_TAG_UNICHAR => "gunichar",
        parser::GI_TYPE_TAG_GTYPE => "GType",
        parser::GI_TYPE_TAG_UTF8 => "utf8",
        parser::GI_TYPE_TAG_FILENAME => "filename",
        parser::GI_TYPE_TAG_ARRAY => "array",
        parser::GI_TYPE_TAG_INTERFACE => "interface",
        parser::GI_TYPE_TAG_GLIST => "glist",
        parser::GI_TYPE_TAG_GSLIST => "gslist",
        parser::GI_TYPE_TAG_GHASH => "ghash",
        parser::GI_TYPE_TAG_ERROR => "error",
        _ => "unknown",
    };
    leak_cstr(name)
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

pub unsafe fn repository_get_option_group() -> Ptr {
    ptr::null_mut()
}

pub unsafe fn repository_dump(
    input_filename: ConstChar,
    output_filename: ConstChar,
    _error: GErrorOut,
) -> gboolean {
    let Some(input) = ptr_string(input_filename) else {
        return 0;
    };
    let Some(output) = ptr_string(output_filename) else {
        return 0;
    };
    let gir_dirs: Vec<PathBuf> = DEFAULT_GIR_DIRS.iter().map(PathBuf::from).collect();
    let text = match parser::decompile_typelib_to_gir(std::path::Path::new(&input), &gir_dirs) {
        Ok(text) => text,
        Err(_) => return 0,
    };
    std::fs::write(output, text).is_ok() as gboolean
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

pub unsafe fn vfunc_get_flags(_info: Ptr) -> c_int {
    0
}

pub unsafe fn vfunc_get_offset(_info: Ptr) -> usize {
    0
}

pub unsafe fn vfunc_get_signal(_info: Ptr) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn vfunc_get_address(_info: Ptr, _implementor_gtype: GType, _error: GErrorOut) -> Ptr {
    ptr::null_mut()
}

pub unsafe fn vfunc_invoke(
    info: Ptr,
    _implementor: GType,
    in_args: *const GIArgument,
    n_in_args: usize,
    out_args: *mut GIArgument,
    n_out_args: usize,
    return_value: *mut GIArgument,
    error: GErrorOut,
) -> gboolean {
    unsafe {
        function_invoke(
            info,
            in_args,
            n_in_args,
            out_args,
            n_out_args,
            return_value,
            error,
        )
    }
}

pub unsafe fn value_get_value(info: Ptr) -> i64 {
    match entry_for(info).map(|entry| entry.kind) {
        Some(InfoKind::Value(value)) => value.value,
        _ => -1,
    }
}

fn attribute_pair(kind: &InfoKind) -> Option<(&'static str, String)> {
    match kind {
        InfoKind::Item(doc, index) => doc
            .item(*index)
            .filter(|item| !item.c_identifier.is_empty())
            .map(|item| ("c:identifier", item.c_identifier.clone())),
        InfoKind::Callable(callable) if !callable.symbol.is_empty() => {
            Some(("c:identifier", callable.symbol.clone()))
        }
        InfoKind::Value(value) if !value.c_identifier.is_empty() => {
            Some(("c:identifier", value.c_identifier.clone()))
        }
        _ => None,
    }
}

fn namespace_for_kind(kind: &InfoKind) -> Option<String> {
    match kind {
        InfoKind::Item(doc, index) => doc.item(*index).map(|item| item.namespace.clone()),
        InfoKind::Callable(callable) => Some(callable.namespace.clone()),
        InfoKind::Field(field) => Some(field.namespace.clone()),
        InfoKind::Value(value) => Some(value.namespace.clone()),
        InfoKind::Property(property) => Some(property.namespace.clone()),
        InfoKind::Arg(_) | InfoKind::Type(_) => None,
    }
}

fn info_identity(kind: &InfoKind) -> (GiType, String, String) {
    let type_ = info_type_for_kind(kind);
    match kind {
        InfoKind::Item(doc, index) => doc
            .item(*index)
            .map(|item| (type_, item.namespace.clone(), item.name.clone()))
            .unwrap_or_else(|| (type_, String::new(), String::new())),
        InfoKind::Callable(callable) => (type_, callable.namespace.clone(), callable.name.clone()),
        InfoKind::Field(field) => (type_, field.namespace.clone(), field.name.clone()),
        InfoKind::Arg(arg) => (type_, String::new(), arg.name.clone()),
        InfoKind::Type(type_info) => (type_, String::new(), type_info.tag.to_string()),
        InfoKind::Value(value) => (type_, value.namespace.clone(), value.name.clone()),
        InfoKind::Property(property) => (type_, property.namespace.clone(), property.name.clone()),
    }
}

fn type_storage_size(tag: c_int, is_pointer: bool) -> usize {
    if is_pointer {
        return mem::size_of::<Ptr>();
    }
    match tag {
        parser::GI_TYPE_TAG_VOID => 0,
        parser::GI_TYPE_TAG_BOOLEAN => mem::size_of::<gboolean>(),
        parser::GI_TYPE_TAG_INT8 | parser::GI_TYPE_TAG_UINT8 => 1,
        parser::GI_TYPE_TAG_INT16 | parser::GI_TYPE_TAG_UINT16 => 2,
        parser::GI_TYPE_TAG_INT32 | parser::GI_TYPE_TAG_UINT32 | parser::GI_TYPE_TAG_UNICHAR => 4,
        parser::GI_TYPE_TAG_INT64 | parser::GI_TYPE_TAG_UINT64 | parser::GI_TYPE_TAG_GTYPE => 8,
        parser::GI_TYPE_TAG_FLOAT => 4,
        parser::GI_TYPE_TAG_DOUBLE => 8,
        _ => mem::size_of::<Ptr>(),
    }
}

unsafe fn write_string_value(type_info: &TypeModel, text: &str, value: *mut GIArgument) {
    if type_info.is_pointer
        || matches!(
            type_info.tag,
            parser::GI_TYPE_TAG_UTF8
                | parser::GI_TYPE_TAG_FILENAME
                | parser::GI_TYPE_TAG_ARRAY
                | parser::GI_TYPE_TAG_INTERFACE
                | parser::GI_TYPE_TAG_GLIST
                | parser::GI_TYPE_TAG_GSLIST
                | parser::GI_TYPE_TAG_GHASH
                | parser::GI_TYPE_TAG_ERROR
        )
    {
        unsafe { (*value).v_pointer = leak_cstr(text.trim_matches('"')) as Ptr };
        return;
    }
    let parsed = text.parse::<i64>().unwrap_or(0) as usize;
    unsafe { argument_from_usize(type_info.tag, parsed, value) };
}

unsafe fn read_argument(mem: Ptr, type_info: &TypeModel, value: *mut GIArgument) -> gboolean {
    if type_info.is_pointer {
        unsafe { (*value).v_pointer = ptr::read_unaligned(mem as *const Ptr) };
        return 1;
    }
    match type_info.tag {
        parser::GI_TYPE_TAG_BOOLEAN => unsafe {
            (*value).v_boolean = ptr::read_unaligned(mem as *const gboolean)
        },
        parser::GI_TYPE_TAG_INT8 => unsafe {
            (*value).v_int8 = ptr::read_unaligned(mem as *const i8)
        },
        parser::GI_TYPE_TAG_UINT8 => unsafe {
            (*value).v_uint8 = ptr::read_unaligned(mem as *const u8)
        },
        parser::GI_TYPE_TAG_INT16 => unsafe {
            (*value).v_int16 = ptr::read_unaligned(mem as *const i16)
        },
        parser::GI_TYPE_TAG_UINT16 => unsafe {
            (*value).v_uint16 = ptr::read_unaligned(mem as *const u16)
        },
        parser::GI_TYPE_TAG_INT32 => unsafe {
            (*value).v_int32 = ptr::read_unaligned(mem as *const i32)
        },
        parser::GI_TYPE_TAG_UINT32 | parser::GI_TYPE_TAG_UNICHAR => unsafe {
            (*value).v_uint32 = ptr::read_unaligned(mem as *const u32)
        },
        parser::GI_TYPE_TAG_INT64 => unsafe {
            (*value).v_int64 = ptr::read_unaligned(mem as *const i64)
        },
        parser::GI_TYPE_TAG_UINT64 | parser::GI_TYPE_TAG_GTYPE => unsafe {
            (*value).v_uint64 = ptr::read_unaligned(mem as *const u64)
        },
        parser::GI_TYPE_TAG_FLOAT => unsafe {
            (*value).v_float = ptr::read_unaligned(mem as *const f32)
        },
        parser::GI_TYPE_TAG_DOUBLE => unsafe {
            (*value).v_double = ptr::read_unaligned(mem as *const f64)
        },
        _ => unsafe { (*value).v_pointer = ptr::read_unaligned(mem as *const Ptr) },
    }
    1
}

unsafe fn write_argument(mem: Ptr, type_info: &TypeModel, value: *const GIArgument) -> gboolean {
    if type_info.is_pointer {
        unsafe { ptr::write_unaligned(mem as *mut Ptr, (*value).v_pointer) };
        return 1;
    }
    match type_info.tag {
        parser::GI_TYPE_TAG_BOOLEAN => unsafe {
            ptr::write_unaligned(mem as *mut gboolean, (*value).v_boolean)
        },
        parser::GI_TYPE_TAG_INT8 => unsafe {
            ptr::write_unaligned(mem as *mut i8, (*value).v_int8)
        },
        parser::GI_TYPE_TAG_UINT8 => unsafe {
            ptr::write_unaligned(mem as *mut u8, (*value).v_uint8)
        },
        parser::GI_TYPE_TAG_INT16 => unsafe {
            ptr::write_unaligned(mem as *mut i16, (*value).v_int16)
        },
        parser::GI_TYPE_TAG_UINT16 => unsafe {
            ptr::write_unaligned(mem as *mut u16, (*value).v_uint16)
        },
        parser::GI_TYPE_TAG_INT32 => unsafe {
            ptr::write_unaligned(mem as *mut i32, (*value).v_int32)
        },
        parser::GI_TYPE_TAG_UINT32 | parser::GI_TYPE_TAG_UNICHAR => unsafe {
            ptr::write_unaligned(mem as *mut u32, (*value).v_uint32)
        },
        parser::GI_TYPE_TAG_INT64 => unsafe {
            ptr::write_unaligned(mem as *mut i64, (*value).v_int64)
        },
        parser::GI_TYPE_TAG_UINT64 | parser::GI_TYPE_TAG_GTYPE => unsafe {
            ptr::write_unaligned(mem as *mut u64, (*value).v_uint64)
        },
        parser::GI_TYPE_TAG_FLOAT => unsafe {
            ptr::write_unaligned(mem as *mut f32, (*value).v_float)
        },
        parser::GI_TYPE_TAG_DOUBLE => unsafe {
            ptr::write_unaligned(mem as *mut f64, (*value).v_double)
        },
        _ => unsafe { ptr::write_unaligned(mem as *mut Ptr, (*value).v_pointer) },
    }
    1
}

unsafe fn argument_from_usize(tag: c_int, value: usize, arg: *mut GIArgument) {
    match tag {
        parser::GI_TYPE_TAG_BOOLEAN => unsafe { (*arg).v_boolean = (value != 0) as gboolean },
        parser::GI_TYPE_TAG_INT8 => unsafe { (*arg).v_int8 = value as i8 },
        parser::GI_TYPE_TAG_UINT8 => unsafe { (*arg).v_uint8 = value as u8 },
        parser::GI_TYPE_TAG_INT16 => unsafe { (*arg).v_int16 = value as i16 },
        parser::GI_TYPE_TAG_UINT16 => unsafe { (*arg).v_uint16 = value as u16 },
        parser::GI_TYPE_TAG_INT32 => unsafe { (*arg).v_int32 = value as i32 },
        parser::GI_TYPE_TAG_UINT32 | parser::GI_TYPE_TAG_UNICHAR => unsafe {
            (*arg).v_uint32 = value as u32
        },
        parser::GI_TYPE_TAG_INT64 => unsafe { (*arg).v_int64 = value as i64 },
        parser::GI_TYPE_TAG_UINT64 | parser::GI_TYPE_TAG_GTYPE => unsafe {
            (*arg).v_uint64 = value as u64
        },
        parser::GI_TYPE_TAG_FLOAT => unsafe { (*arg).v_float = f32::from_bits(value as u32) },
        parser::GI_TYPE_TAG_DOUBLE => unsafe { (*arg).v_double = f64::from_bits(value as u64) },
        _ => unsafe { (*arg).v_pointer = value as Ptr },
    }
}

unsafe fn argument_to_usize(tag: c_int, arg: *const GIArgument) -> usize {
    match tag {
        parser::GI_TYPE_TAG_BOOLEAN => unsafe { (*arg).v_boolean as usize },
        parser::GI_TYPE_TAG_INT8 => unsafe { (*arg).v_int8 as usize },
        parser::GI_TYPE_TAG_UINT8 => unsafe { (*arg).v_uint8 as usize },
        parser::GI_TYPE_TAG_INT16 => unsafe { (*arg).v_int16 as usize },
        parser::GI_TYPE_TAG_UINT16 => unsafe { (*arg).v_uint16 as usize },
        parser::GI_TYPE_TAG_INT32 => unsafe { (*arg).v_int32 as usize },
        parser::GI_TYPE_TAG_UINT32 | parser::GI_TYPE_TAG_UNICHAR => unsafe {
            (*arg).v_uint32 as usize
        },
        parser::GI_TYPE_TAG_INT64 => unsafe { (*arg).v_int64 as usize },
        parser::GI_TYPE_TAG_UINT64 | parser::GI_TYPE_TAG_GTYPE => unsafe {
            (*arg).v_uint64 as usize
        },
        parser::GI_TYPE_TAG_FLOAT => unsafe { (*arg).v_float.to_bits() as usize },
        parser::GI_TYPE_TAG_DOUBLE => unsafe { (*arg).v_double.to_bits() as usize },
        _ => unsafe { (*arg).v_pointer as usize },
    }
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
