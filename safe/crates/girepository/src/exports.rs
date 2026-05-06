#![allow(dead_code)]

use crate::abi::{GIArgInfo, GIArgument, GITypeInfo};
use crate::ffi::{GQuark, GType, gboolean, guint};
use core::ffi::{c_char, c_int, c_void};
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::sync::{Mutex, OnceLock};

type Ptr = *mut c_void;
type ConstPtr = *const c_void;
type ConstChar = *const c_char;
type CharStrv = *mut *mut c_char;
type ConstCharStrv = *const *const c_char;
type GErrorOut = *mut Ptr;

const RTLD_LAZY: c_int = 1;
const RTLD_LOCAL: c_int = 0;
const DEFAULT_TYPELIB_DIR: &str = "/usr/local/lib/x86_64-linux-gnu/girepository-1.0";

unsafe extern "C" {
    fn dlopen(filename: ConstChar, flags: c_int) -> Ptr;
    fn dlsym(handle: Ptr, symbol: ConstChar) -> Ptr;
}

struct RepositoryState {
    backend: bool,
    search_paths: Vec<CString>,
    search_ptrs: Vec<usize>,
    library_paths: Vec<CString>,
    library_ptrs: Vec<usize>,
}

impl RepositoryState {
    fn new(backend: bool) -> Self {
        let mut state = Self {
            backend,
            search_paths: default_search_paths(),
            search_ptrs: Vec::new(),
            library_paths: Vec::new(),
            library_ptrs: vec![0],
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

static BACKEND_HANDLE: OnceLock<usize> = OnceLock::new();
static REPOSITORIES: OnceLock<Mutex<HashMap<usize, RepositoryState>>> = OnceLock::new();

fn repository_states() -> &'static Mutex<HashMap<usize, RepositoryState>> {
    REPOSITORIES.get_or_init(|| Mutex::new(HashMap::new()))
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

unsafe fn backend_handle() -> Ptr {
    *BACKEND_HANDLE.get_or_init(|| {
        let env_path = std::env::var("SAFE_GIREPOSITORY_BACKEND").ok();
        let candidates = [
            env_path.as_deref(),
            Some("/lib/x86_64-linux-gnu/libgirepository-2.0.so.0"),
            Some("/usr/lib/x86_64-linux-gnu/libgirepository-2.0.so.0"),
        ];
        for candidate in candidates.into_iter().flatten() {
            let path = CString::new(candidate).unwrap();
            let handle = unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
            if !handle.is_null() {
                return handle as usize;
            }
        }
        0
    }) as Ptr
}

unsafe fn resolve<T: Copy>(symbol: &'static [u8]) -> Option<T> {
    let handle = unsafe { backend_handle() };
    if handle.is_null() {
        return None;
    }
    let pointer = unsafe { dlsym(handle, symbol.as_ptr().cast()) };
    (!pointer.is_null()).then(|| unsafe { core::mem::transmute_copy::<Ptr, T>(&pointer) })
}

fn repository_uses_backend(repository: Ptr) -> bool {
    repository_states()
        .lock()
        .unwrap()
        .get(&(repository as usize))
        .map(|state| state.backend)
        .unwrap_or(true)
}

macro_rules! forward_ret {
    ($name:ident ( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty, $default:expr) => {
        #[export_name = stringify!($name)]
        pub unsafe extern "C" fn $name($($arg: $argty),*) -> $ret {
            type Backend = unsafe extern "C" fn($($argty),*) -> $ret;
            if let Some(function) = unsafe { resolve::<Backend>(concat!(stringify!($name), "\0").as_bytes()) } {
                unsafe { function($($arg),*) }
            } else {
                $default
            }
        }
    };
}

macro_rules! forward_void {
    ($name:ident ( $($arg:ident : $argty:ty),* $(,)? )) => {
        #[export_name = stringify!($name)]
        pub unsafe extern "C" fn $name($($arg: $argty),*) {
            type Backend = unsafe extern "C" fn($($argty),*);
            if let Some(function) = unsafe { resolve::<Backend>(concat!(stringify!($name), "\0").as_bytes()) } {
                unsafe { function($($arg),*) };
            }
        }
    };
}

macro_rules! forward_get_type {
    ($($name:ident),+ $(,)?) => {
        $(forward_ret!($name() -> GType, 0);)+
    };
}

macro_rules! stub_symbols {
    ($($name:ident),+ $(,)?) => {
        $(
            #[export_name = stringify!($name)]
            pub unsafe extern "C" fn $name() -> usize {
                0
            }
        )+
    };
}

#[export_name = "gi_repository_new"]
pub unsafe extern "C" fn export_gi_repository_new() -> Ptr {
    type Backend = unsafe extern "C" fn() -> Ptr;
    if let Some(function) = unsafe { resolve::<Backend>(b"gi_repository_new\0") } {
        if function as usize != export_gi_repository_new as *const () as usize {
            let repository = unsafe { function() };
            if !repository.is_null() {
                repository_states()
                    .lock()
                    .unwrap()
                    .insert(repository as usize, RepositoryState::new(true));
                return repository;
            }
        }
    }

    let repository = Box::into_raw(Box::new(crate::runtime::GIRepositoryHandle { _opaque: 1 })) as Ptr;
    repository_states()
        .lock()
        .unwrap()
        .insert(repository as usize, RepositoryState::new(false));
    repository
}

#[export_name = "gi_repository_prepend_search_path"]
pub unsafe extern "C" fn gi_repository_prepend_search_path(repository: Ptr, path: ConstChar) {
    if repository_uses_backend(repository) {
        type Backend = unsafe extern "C" fn(Ptr, ConstChar);
        if let Some(function) = unsafe { resolve::<Backend>(b"gi_repository_prepend_search_path\0") } {
            unsafe { function(repository, path) };
        }
    }
    if path.is_null() {
        return;
    }
    let copy = unsafe { std::ffi::CStr::from_ptr(path) }.to_owned();
    let mut states = repository_states().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(|| RepositoryState::new(false));
    state.search_paths.insert(0, copy);
    state.refresh_search_ptrs();
}

#[export_name = "gi_repository_get_search_path"]
pub unsafe extern "C" fn gi_repository_get_search_path(repository: Ptr, n_paths_out: *mut usize) -> ConstCharStrv {
    let mut states = repository_states().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(|| RepositoryState::new(false));
    if !n_paths_out.is_null() {
        unsafe { *n_paths_out = state.search_paths.len() };
    }
    state.search_ptrs.as_ptr() as ConstCharStrv
}

#[export_name = "gi_repository_prepend_library_path"]
pub unsafe extern "C" fn gi_repository_prepend_library_path(repository: Ptr, path: ConstChar) {
    if repository_uses_backend(repository) {
        type Backend = unsafe extern "C" fn(Ptr, ConstChar);
        if let Some(function) = unsafe { resolve::<Backend>(b"gi_repository_prepend_library_path\0") } {
            unsafe { function(repository, path) };
        }
    }
    if path.is_null() {
        return;
    }
    let copy = unsafe { std::ffi::CStr::from_ptr(path) }.to_owned();
    let mut states = repository_states().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(|| RepositoryState::new(false));
    state.library_paths.insert(0, copy);
    state.refresh_library_ptrs();
}

#[export_name = "gi_repository_get_library_path"]
pub unsafe extern "C" fn gi_repository_get_library_path(repository: Ptr, n_paths_out: *mut usize) -> ConstCharStrv {
    let mut states = repository_states().lock().unwrap();
    let state = states
        .entry(repository as usize)
        .or_insert_with(|| RepositoryState::new(false));
    if !n_paths_out.is_null() {
        unsafe { *n_paths_out = state.library_paths.len() };
    }
    state.library_ptrs.as_ptr() as ConstCharStrv
}

forward_get_type!(
    gi_arg_info_get_type,
    gi_base_info_get_type,
    gi_callable_info_get_type,
    gi_callback_info_get_type,
    gi_constant_info_get_type,
    gi_enum_info_get_type,
    gi_field_info_get_type,
    gi_flags_info_get_type,
    gi_function_info_get_type,
    gi_interface_info_get_type,
    gi_object_info_get_type,
    gi_property_info_get_type,
    gi_registered_type_info_get_type,
    gi_repository_get_type,
    gi_signal_info_get_type,
    gi_struct_info_get_type,
    gi_type_info_get_type,
    gi_typelib_get_type,
    gi_union_info_get_type,
    gi_unresolved_info_get_type,
    gi_value_info_get_type,
    gi_vfunc_info_get_type,
);

forward_ret!(gi_repository_require(repository: Ptr, namespace_: ConstChar, version: ConstChar, flags: c_int, error: GErrorOut) -> Ptr, ptr::null_mut());
forward_ret!(gi_repository_enumerate_versions(repository: Ptr, namespace_: ConstChar, n_versions_out: *mut usize) -> CharStrv, ptr::null_mut());
forward_ret!(gi_repository_get_loaded_namespaces(repository: Ptr, n_namespaces_out: *mut usize) -> CharStrv, ptr::null_mut());
forward_ret!(gi_repository_get_c_prefix(repository: Ptr, namespace_: ConstChar) -> ConstChar, ptr::null());
forward_ret!(gi_repository_find_by_name(repository: Ptr, namespace_: ConstChar, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_repository_find_by_gtype(repository: Ptr, gtype: GType) -> Ptr, ptr::null_mut());
forward_ret!(gi_repository_find_by_error_domain(repository: Ptr, domain: GQuark) -> Ptr, ptr::null_mut());
forward_ret!(gi_repository_get_dependencies(repository: Ptr, namespace_: ConstChar, n_dependencies_out: *mut usize) -> CharStrv, ptr::null_mut());
forward_ret!(gi_repository_get_n_infos(repository: Ptr, namespace_: ConstChar) -> guint, 0);
forward_ret!(gi_repository_get_info(repository: Ptr, namespace_: ConstChar, index: guint) -> Ptr, ptr::null_mut());
forward_void!(gi_repository_get_object_gtype_interfaces(repository: Ptr, gtype: GType, n_interfaces_out: *mut usize, interfaces_out: *mut *mut Ptr));

forward_void!(gi_base_info_clear(info: Ptr));
forward_void!(gi_base_info_unref(info: Ptr));
forward_ret!(gi_base_info_get_attribute(info: Ptr, name: ConstChar) -> ConstChar, ptr::null());
forward_ret!(gi_base_info_get_name(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_base_info_get_namespace(info: Ptr) -> ConstChar, ptr::null());

forward_ret!(gi_arg_info_get_closure_index(info: Ptr, out_index: *mut guint) -> gboolean, 0);
forward_ret!(gi_arg_info_get_destroy_index(info: Ptr, out_index: *mut guint) -> gboolean, 0);
forward_ret!(gi_arg_info_get_direction(info: Ptr) -> c_int, 0);
forward_ret!(gi_arg_info_get_ownership_transfer(info: Ptr) -> c_int, 0);
forward_ret!(gi_arg_info_get_scope(info: Ptr) -> c_int, 0);
forward_ret!(gi_arg_info_get_type_info(info: Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_arg_info_is_caller_allocates(info: Ptr) -> gboolean, 0);
forward_ret!(gi_arg_info_is_optional(info: Ptr) -> gboolean, 0);
forward_ret!(gi_arg_info_is_return_value(info: Ptr) -> gboolean, 0);
forward_ret!(gi_arg_info_is_skip(info: Ptr) -> gboolean, 0);
forward_void!(gi_arg_info_load_type_info(info: Ptr, type_info: *mut GITypeInfo));
forward_ret!(gi_arg_info_may_be_null(info: Ptr) -> gboolean, 0);

forward_ret!(gi_callable_info_can_throw_gerror(info: Ptr) -> gboolean, 0);
forward_ret!(gi_callable_info_get_arg(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_callable_info_get_caller_owns(info: Ptr) -> c_int, 0);
forward_ret!(gi_callable_info_get_instance_ownership_transfer(info: Ptr) -> c_int, 0);
forward_ret!(gi_callable_info_get_n_args(info: Ptr) -> guint, 0);
forward_ret!(gi_callable_info_get_return_attribute(info: Ptr, name: ConstChar) -> ConstChar, ptr::null());
forward_ret!(gi_callable_info_get_return_type(info: Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_callable_info_is_method(info: Ptr) -> gboolean, 0);
forward_ret!(gi_callable_info_iterate_return_attributes(info: Ptr, iterator: Ptr, name: *mut ConstChar, value: *mut ConstChar) -> gboolean, 0);
forward_void!(gi_callable_info_load_arg(info: Ptr, index: guint, arg_info: *mut GIArgInfo));
forward_void!(gi_callable_info_load_return_type(info: Ptr, type_info: *mut GITypeInfo));
forward_ret!(gi_callable_info_may_return_null(info: Ptr) -> gboolean, 0);
forward_ret!(gi_callable_info_skip_return(info: Ptr) -> gboolean, 0);

forward_ret!(gi_enum_info_get_method(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_enum_info_get_n_methods(info: Ptr) -> guint, 0);
forward_ret!(gi_enum_info_get_n_values(info: Ptr) -> guint, 0);
forward_ret!(gi_enum_info_get_value(info: Ptr, index: guint) -> Ptr, ptr::null_mut());

forward_ret!(gi_field_info_get_type_info(info: Ptr) -> Ptr, ptr::null_mut());

forward_ret!(gi_function_info_get_flags(info: Ptr) -> c_int, 0);
forward_ret!(gi_function_info_get_symbol(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_function_info_invoke(info: Ptr, in_args: *const GIArgument, n_in_args: usize, out_args: *mut GIArgument, n_out_args: usize, return_value: *mut GIArgument, error: GErrorOut) -> gboolean, 0);
forward_ret!(gi_function_info_prep_invoker(info: Ptr, invoker: Ptr, error: GErrorOut) -> gboolean, 0);
forward_void!(gi_function_invoker_clear(invoker: Ptr));

forward_ret!(gi_interface_info_find_method(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_interface_info_find_vfunc(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());

forward_ret!(gi_object_info_find_method(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_find_method_using_interfaces(info: Ptr, name: ConstChar, declarer_out: *mut Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_find_signal(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_find_vfunc(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_find_vfunc_using_interfaces(info: Ptr, name: ConstChar, declarer_out: *mut Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_get_method(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_get_n_methods(info: Ptr) -> guint, 0);
forward_ret!(gi_object_info_get_property(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_object_info_get_ref_function_pointer(info: Ptr) -> Ptr, ptr::null_mut());

forward_ret!(gi_registered_type_info_get_g_type(info: Ptr) -> GType, 0);
forward_ret!(gi_registered_type_info_get_type_init_function_name(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_registered_type_info_get_type_name(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_registered_type_info_is_boxed(info: Ptr) -> gboolean, 0);

forward_ret!(gi_signal_info_get_flags(info: Ptr) -> c_int, 0);

forward_ret!(gi_struct_info_find_field(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_struct_info_find_method(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_struct_info_get_field(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_struct_info_get_n_fields(info: Ptr) -> guint, 0);
forward_ret!(gi_struct_info_get_size(info: Ptr) -> usize, 0);
forward_ret!(gi_struct_info_is_gtype_struct(info: Ptr) -> gboolean, 0);

forward_ret!(gi_type_info_get_array_length_index(info: Ptr, out_index: *mut guint) -> gboolean, 0);
forward_ret!(gi_type_info_get_array_type(info: Ptr) -> c_int, 0);
forward_ret!(gi_type_info_get_interface(info: Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_type_info_get_tag(info: Ptr) -> c_int, 0);
forward_ret!(gi_type_info_is_pointer(info: Ptr) -> gboolean, 0);
forward_ret!(gi_type_info_is_zero_terminated(info: Ptr) -> gboolean, 0);

forward_ret!(gi_typelib_ref(typelib: Ptr) -> Ptr, ptr::null_mut());

forward_ret!(gi_union_info_find_method(info: Ptr, name: ConstChar) -> Ptr, ptr::null_mut());
forward_ret!(gi_union_info_get_alignment(info: Ptr) -> usize, 0);
forward_ret!(gi_union_info_get_copy_function_name(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_union_info_get_discriminator(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_union_info_get_discriminator_offset(info: Ptr, out_offset: *mut usize) -> gboolean, 0);
forward_ret!(gi_union_info_get_discriminator_type(info: Ptr) -> Ptr, ptr::null_mut());
forward_ret!(gi_union_info_get_field(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_union_info_get_free_function_name(info: Ptr) -> ConstChar, ptr::null());
forward_ret!(gi_union_info_get_method(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
forward_ret!(gi_union_info_get_n_fields(info: Ptr) -> guint, 0);
forward_ret!(gi_union_info_get_n_methods(info: Ptr) -> guint, 0);
forward_ret!(gi_union_info_get_size(info: Ptr) -> usize, 0);
forward_ret!(gi_union_info_is_discriminated(info: Ptr) -> gboolean, 0);

forward_ret!(gi_vfunc_info_get_invoker(info: Ptr) -> Ptr, ptr::null_mut());

stub_symbols!(
    gi_base_info_equal,
    gi_base_info_get_container,
    gi_base_info_get_typelib,
    gi_base_info_is_deprecated,
    gi_base_info_iterate_attributes,
    gi_base_info_ref,
    gi_callable_info_create_closure,
    gi_callable_info_destroy_closure,
    gi_callable_info_get_closure_native_address,
    gi_callable_info_invoke,
    gi_cclosure_marshal_generic,
    gi_constant_info_free_value,
    gi_constant_info_get_type_info,
    gi_constant_info_get_value,
    gi_enum_info_get_error_domain,
    gi_enum_info_get_storage_type,
    gi_field_info_get_field,
    gi_field_info_get_flags,
    gi_field_info_get_offset,
    gi_field_info_get_size,
    gi_field_info_set_field,
    gi_function_info_get_property,
    gi_function_info_get_vfunc,
    gi_function_invoker_new_for_address,
    gi_interface_info_find_signal,
    gi_interface_info_get_constant,
    gi_interface_info_get_iface_struct,
    gi_interface_info_get_method,
    gi_interface_info_get_n_constants,
    gi_interface_info_get_n_methods,
    gi_interface_info_get_n_prerequisites,
    gi_interface_info_get_n_properties,
    gi_interface_info_get_n_signals,
    gi_interface_info_get_n_vfuncs,
    gi_interface_info_get_prerequisite,
    gi_interface_info_get_property,
    gi_interface_info_get_signal,
    gi_interface_info_get_vfunc,
    gi_invoke_error_quark,
    gi_object_info_get_abstract,
    gi_object_info_get_class_struct,
    gi_object_info_get_constant,
    gi_object_info_get_field,
    gi_object_info_get_final,
    gi_object_info_get_fundamental,
    gi_object_info_get_get_value_function_name,
    gi_object_info_get_get_value_function_pointer,
    gi_object_info_get_interface,
    gi_object_info_get_n_constants,
    gi_object_info_get_n_fields,
    gi_object_info_get_n_interfaces,
    gi_object_info_get_n_properties,
    gi_object_info_get_n_signals,
    gi_object_info_get_n_vfuncs,
    gi_object_info_get_parent,
    gi_object_info_get_ref_function_name,
    gi_object_info_get_set_value_function_name,
    gi_object_info_get_set_value_function_pointer,
    gi_object_info_get_signal,
    gi_object_info_get_type_init_function_name,
    gi_object_info_get_type_name,
    gi_object_info_get_unref_function_name,
    gi_object_info_get_unref_function_pointer,
    gi_object_info_get_vfunc,
    gi_property_info_get_flags,
    gi_property_info_get_getter,
    gi_property_info_get_ownership_transfer,
    gi_property_info_get_setter,
    gi_property_info_get_type_info,
    gi_repository_dump,
    gi_repository_error_quark,
    gi_repository_get_immediate_dependencies,
    gi_repository_get_option_group,
    gi_repository_get_shared_libraries,
    gi_repository_get_typelib_path,
    gi_repository_get_version,
    gi_repository_is_registered,
    gi_repository_load_typelib,
    gi_repository_require_private,
    gi_signal_info_get_class_closure,
    gi_signal_info_true_stops_emit,
    gi_struct_info_get_alignment,
    gi_struct_info_get_copy_function_name,
    gi_struct_info_get_free_function_name,
    gi_struct_info_get_method,
    gi_struct_info_get_n_methods,
    gi_struct_info_is_foreign,
    gi_type_info_argument_from_hash_pointer,
    gi_type_info_extract_ffi_return_value,
    gi_type_info_get_array_fixed_size,
    gi_type_info_get_ffi_type,
    gi_type_info_get_param_type,
    gi_type_info_get_storage_type,
    gi_type_info_hash_pointer_from_argument,
    gi_type_tag_argument_from_hash_pointer,
    gi_type_tag_extract_ffi_return_value,
    gi_type_tag_get_ffi_type,
    gi_type_tag_hash_pointer_from_argument,
    gi_type_tag_to_string,
    gi_typelib_get_namespace,
    gi_typelib_new_from_bytes,
    gi_typelib_symbol,
    gi_typelib_unref,
    gi_typelib_validate,
    gi_value_info_get_value,
    gi_vfunc_info_get_address,
    gi_vfunc_info_get_flags,
    gi_vfunc_info_get_offset,
    gi_vfunc_info_get_signal,
    gi_vfunc_info_invoke,
);
