#![allow(dead_code)]

use crate::abi::{GIArgInfo, GIArgument, GITypeInfo};
use crate::ffi::{gboolean, guint, GQuark, GType};
use crate::runtime::{ConstChar, ConstCharStrv, GErrorOut, Ptr};
use core::ffi::{c_char, c_int};
use std::ptr;

type CharStrv = *mut *mut c_char;

macro_rules! abi_ret {
    ($name:ident ( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty, $default:expr) => {
        #[export_name = stringify!($name)]
        pub unsafe extern "C" fn $name($($arg: $argty),*) -> $ret {
            $(let _ = $arg;)*
            $default
        }
    };
}

macro_rules! abi_void {
    ($name:ident ( $($arg:ident : $argty:ty),* $(,)? )) => {
        #[export_name = stringify!($name)]
        pub unsafe extern "C" fn $name($($arg: $argty),*) {
            $(let _ = $arg;)*
        }
    };
}

macro_rules! abi_get_type {
    ($($name:ident),+ $(,)?) => {
        $(
            #[export_name = stringify!($name)]
            pub unsafe extern "C" fn $name() -> GType {
                crate::runtime::gtype_for_getter(stringify!($name))
            }
        )+
    };
}

macro_rules! abi_zero_arg_symbols {
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
pub unsafe extern "C" fn gi_repository_new() -> Ptr {
    unsafe { crate::runtime::new_repository() }
}

#[export_name = "gi_repository_prepend_search_path"]
pub unsafe extern "C" fn gi_repository_prepend_search_path(repository: Ptr, path: ConstChar) {
    unsafe { crate::runtime::prepend_search_path(repository, path) }
}

#[export_name = "gi_repository_get_search_path"]
pub unsafe extern "C" fn gi_repository_get_search_path(
    repository: Ptr,
    n_paths_out: *mut usize,
) -> ConstCharStrv {
    unsafe { crate::runtime::get_search_path(repository, n_paths_out) }
}

#[export_name = "gi_repository_prepend_library_path"]
pub unsafe extern "C" fn gi_repository_prepend_library_path(repository: Ptr, path: ConstChar) {
    unsafe { crate::runtime::prepend_library_path(repository, path) }
}

#[export_name = "gi_repository_get_library_path"]
pub unsafe extern "C" fn gi_repository_get_library_path(
    repository: Ptr,
    n_paths_out: *mut usize,
) -> ConstCharStrv {
    unsafe { crate::runtime::get_library_path(repository, n_paths_out) }
}

abi_get_type!(
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

abi_ret!(gi_repository_require(repository: Ptr, namespace_: ConstChar, version: ConstChar, flags: c_int, error: GErrorOut) -> Ptr, unsafe {
    crate::runtime::repository_require(repository, namespace_, version, flags, error)
});
abi_ret!(gi_repository_enumerate_versions(repository: Ptr, namespace_: ConstChar, n_versions_out: *mut usize) -> CharStrv, unsafe {
    crate::runtime::enumerate_versions(repository, namespace_, n_versions_out)
});
abi_ret!(gi_repository_get_loaded_namespaces(repository: Ptr, n_namespaces_out: *mut usize) -> CharStrv, unsafe {
    crate::runtime::loaded_namespaces(repository, n_namespaces_out)
});
abi_ret!(gi_repository_get_c_prefix(repository: Ptr, namespace_: ConstChar) -> ConstChar, unsafe {
    crate::runtime::get_c_prefix(repository, namespace_)
});
abi_ret!(gi_repository_find_by_name(repository: Ptr, namespace_: ConstChar, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::find_by_name(repository, namespace_, name)
});
abi_ret!(gi_repository_find_by_gtype(repository: Ptr, gtype: GType) -> Ptr, unsafe {
    crate::runtime::find_by_gtype(repository, gtype)
});
abi_ret!(gi_repository_find_by_error_domain(repository: Ptr, domain: GQuark) -> Ptr, unsafe {
    crate::runtime::find_by_error_domain(repository, domain)
});
abi_ret!(gi_repository_get_dependencies(repository: Ptr, namespace_: ConstChar, n_dependencies_out: *mut usize) -> CharStrv, unsafe {
    crate::runtime::get_dependencies(repository, namespace_, n_dependencies_out)
});
abi_ret!(gi_repository_get_immediate_dependencies(repository: Ptr, namespace_: ConstChar, n_dependencies_out: *mut usize) -> CharStrv, unsafe {
    crate::runtime::get_dependencies(repository, namespace_, n_dependencies_out)
});
abi_ret!(gi_repository_get_n_infos(repository: Ptr, namespace_: ConstChar) -> guint, unsafe {
    crate::runtime::repository_get_n_infos(repository, namespace_)
});
abi_ret!(gi_repository_get_info(repository: Ptr, namespace_: ConstChar, index: guint) -> Ptr, unsafe {
    crate::runtime::repository_get_info(repository, namespace_, index)
});
#[export_name = "gi_repository_get_object_gtype_interfaces"]
pub unsafe extern "C" fn gi_repository_get_object_gtype_interfaces(
    repository: Ptr,
    gtype: GType,
    n_interfaces_out: *mut usize,
    interfaces_out: *mut *mut Ptr,
) {
    unsafe {
        crate::runtime::get_object_gtype_interfaces(
            repository,
            gtype,
            n_interfaces_out,
            interfaces_out,
        )
    }
}

#[export_name = "gi_base_info_clear"]
pub unsafe extern "C" fn gi_base_info_clear(info: Ptr) {
    unsafe { crate::runtime::base_info_clear(info) }
}
abi_ret!(gi_base_info_ref(info: Ptr) -> Ptr, unsafe { crate::runtime::base_info_ref(info) });
#[export_name = "gi_base_info_unref"]
pub unsafe extern "C" fn gi_base_info_unref(info: Ptr) {
    unsafe { crate::runtime::base_info_unref(info) }
}
abi_ret!(gi_base_info_get_attribute(info: Ptr, name: ConstChar) -> ConstChar, unsafe {
    crate::runtime::base_info_get_attribute(info, name)
});
abi_ret!(gi_base_info_get_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::base_info_get_name(info)
});
abi_ret!(gi_base_info_get_namespace(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::base_info_get_namespace(info)
});

abi_ret!(gi_arg_info_get_closure_index(info: Ptr, out_index: *mut guint) -> gboolean, unsafe {
    crate::runtime::arg_get_closure_index(info, out_index)
});
abi_ret!(gi_arg_info_get_destroy_index(info: Ptr, out_index: *mut guint) -> gboolean, unsafe {
    crate::runtime::arg_get_destroy_index(info, out_index)
});
abi_ret!(gi_arg_info_get_direction(info: Ptr) -> c_int, unsafe {
    crate::runtime::arg_get_direction(info)
});
abi_ret!(gi_arg_info_get_ownership_transfer(info: Ptr) -> c_int, unsafe {
    crate::runtime::arg_get_ownership_transfer(info)
});
abi_ret!(gi_arg_info_get_scope(info: Ptr) -> c_int, unsafe {
    crate::runtime::arg_get_scope(info)
});
abi_ret!(gi_arg_info_get_type_info(info: Ptr) -> Ptr, unsafe {
    crate::runtime::arg_get_type_info(info)
});
abi_ret!(gi_arg_info_is_caller_allocates(info: Ptr) -> gboolean, 0);
abi_ret!(gi_arg_info_is_optional(info: Ptr) -> gboolean, 0);
abi_ret!(gi_arg_info_is_return_value(info: Ptr) -> gboolean, 0);
abi_ret!(gi_arg_info_is_skip(info: Ptr) -> gboolean, 0);
#[export_name = "gi_arg_info_load_type_info"]
pub unsafe extern "C" fn gi_arg_info_load_type_info(info: Ptr, type_info: *mut GITypeInfo) {
    unsafe { crate::runtime::arg_load_type_info(info, type_info) }
}
abi_ret!(gi_arg_info_may_be_null(info: Ptr) -> gboolean, unsafe {
    crate::runtime::arg_may_be_null(info)
});

abi_ret!(gi_callable_info_can_throw_gerror(info: Ptr) -> gboolean, unsafe {
    crate::runtime::callable_can_throw_gerror(info)
});
abi_ret!(gi_callable_info_get_arg(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::callable_get_arg(info, index)
});
abi_ret!(gi_callable_info_get_caller_owns(info: Ptr) -> c_int, 0);
abi_ret!(gi_callable_info_get_instance_ownership_transfer(info: Ptr) -> c_int, unsafe {
    crate::runtime::callable_get_instance_ownership_transfer(info)
});
abi_ret!(gi_callable_info_get_n_args(info: Ptr) -> guint, unsafe {
    crate::runtime::callable_get_n_args(info)
});
abi_ret!(gi_callable_info_get_return_attribute(info: Ptr, name: ConstChar) -> ConstChar, ptr::null());
abi_ret!(gi_callable_info_get_return_type(info: Ptr) -> Ptr, unsafe {
    crate::runtime::callable_get_return_type(info)
});
abi_ret!(gi_callable_info_is_method(info: Ptr) -> gboolean, unsafe {
    crate::runtime::callable_is_method(info)
});
abi_ret!(gi_callable_info_iterate_return_attributes(info: Ptr, iterator: Ptr, name: *mut ConstChar, value: *mut ConstChar) -> gboolean, 0);
#[export_name = "gi_callable_info_load_arg"]
pub unsafe extern "C" fn gi_callable_info_load_arg(
    info: Ptr,
    index: guint,
    arg_info: *mut GIArgInfo,
) {
    unsafe { crate::runtime::callable_load_arg(info, index, arg_info) }
}
#[export_name = "gi_callable_info_load_return_type"]
pub unsafe extern "C" fn gi_callable_info_load_return_type(info: Ptr, type_info: *mut GITypeInfo) {
    unsafe { crate::runtime::callable_load_return_type(info, type_info) }
}
abi_ret!(gi_callable_info_may_return_null(info: Ptr) -> gboolean, unsafe {
    crate::runtime::callable_may_return_null(info)
});
abi_ret!(gi_callable_info_skip_return(info: Ptr) -> gboolean, 0);

abi_ret!(gi_enum_info_get_method(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::enum_get_method(info, index)
});
abi_ret!(gi_enum_info_get_n_methods(info: Ptr) -> guint, unsafe {
    crate::runtime::enum_get_n_methods(info)
});
abi_ret!(gi_enum_info_get_n_values(info: Ptr) -> guint, unsafe {
    crate::runtime::enum_get_n_values(info)
});
abi_ret!(gi_enum_info_get_value(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::enum_get_value(info, index)
});

abi_ret!(gi_field_info_get_type_info(info: Ptr) -> Ptr, unsafe {
    crate::runtime::field_get_type_info(info)
});

abi_ret!(gi_function_info_get_flags(info: Ptr) -> c_int, unsafe {
    crate::runtime::function_get_flags(info)
});
abi_ret!(gi_function_info_get_symbol(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::function_get_symbol(info)
});
abi_ret!(gi_function_info_invoke(info: Ptr, in_args: *const GIArgument, n_in_args: usize, out_args: *mut GIArgument, n_out_args: usize, return_value: *mut GIArgument, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::function_invoke(info, in_args, n_in_args, out_args, n_out_args, return_value, error)
});
abi_ret!(gi_function_info_prep_invoker(info: Ptr, invoker: Ptr, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::function_prep_invoker(info, invoker, error)
});
abi_void!(gi_function_invoker_clear(invoker: Ptr));

abi_ret!(gi_interface_info_find_method(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::interface_find_method(info, name)
});
abi_ret!(gi_interface_info_find_vfunc(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::interface_find_vfunc(info, name)
});

abi_ret!(gi_object_info_find_method(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::object_find_method(info, name)
});
abi_ret!(gi_object_info_find_method_using_interfaces(info: Ptr, name: ConstChar, declarer_out: *mut Ptr) -> Ptr, unsafe {
    crate::runtime::object_find_method_using_interfaces(info, name, declarer_out)
});
abi_ret!(gi_object_info_find_signal(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::object_find_signal(info, name)
});
abi_ret!(gi_object_info_find_vfunc(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::object_find_vfunc(info, name)
});
abi_ret!(gi_object_info_find_vfunc_using_interfaces(info: Ptr, name: ConstChar, declarer_out: *mut Ptr) -> Ptr, unsafe {
    crate::runtime::object_find_vfunc_using_interfaces(info, name, declarer_out)
});
abi_ret!(gi_object_info_get_method(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_method(info, index)
});
abi_ret!(gi_object_info_get_n_methods(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_methods(info)
});
abi_ret!(gi_object_info_get_property(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_property(info, index)
});
abi_ret!(gi_object_info_get_ref_function_pointer(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_ref_function_pointer(info)
});

abi_ret!(gi_registered_type_info_get_g_type(info: Ptr) -> GType, unsafe {
    crate::runtime::registered_get_g_type(info)
});
abi_ret!(gi_registered_type_info_get_type_init_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::registered_get_type_init_function_name(info)
});
abi_ret!(gi_registered_type_info_get_type_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::registered_get_type_name(info)
});
abi_ret!(gi_registered_type_info_is_boxed(info: Ptr) -> gboolean, unsafe {
    crate::runtime::registered_is_boxed(info)
});

abi_ret!(gi_signal_info_get_flags(info: Ptr) -> c_int, unsafe {
    crate::runtime::signal_get_flags(info)
});

abi_ret!(gi_struct_info_find_field(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::struct_find_field(info, name)
});
abi_ret!(gi_struct_info_find_method(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::struct_find_method(info, name)
});
abi_ret!(gi_struct_info_get_field(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::struct_get_field(info, index)
});
abi_ret!(gi_struct_info_get_n_fields(info: Ptr) -> guint, unsafe {
    crate::runtime::struct_get_n_fields(info)
});
abi_ret!(gi_struct_info_get_size(info: Ptr) -> usize, unsafe {
    crate::runtime::struct_get_size(info)
});
abi_ret!(gi_struct_info_is_gtype_struct(info: Ptr) -> gboolean, unsafe {
    crate::runtime::struct_is_gtype_struct(info)
});

abi_ret!(gi_type_info_get_array_length_index(info: Ptr, out_index: *mut guint) -> gboolean, unsafe {
    crate::runtime::type_get_array_length_index(info, out_index)
});
abi_ret!(gi_type_info_get_array_type(info: Ptr) -> c_int, unsafe {
    crate::runtime::type_get_array_type(info)
});
abi_ret!(gi_type_info_get_interface(info: Ptr) -> Ptr, unsafe {
    crate::runtime::type_get_interface(info)
});
abi_ret!(gi_type_info_get_tag(info: Ptr) -> c_int, unsafe {
    crate::runtime::type_get_tag(info)
});
abi_ret!(gi_type_info_is_pointer(info: Ptr) -> gboolean, unsafe {
    crate::runtime::type_is_pointer(info)
});
abi_ret!(gi_type_info_is_zero_terminated(info: Ptr) -> gboolean, unsafe {
    crate::runtime::type_is_zero_terminated(info)
});

abi_ret!(gi_typelib_ref(typelib: Ptr) -> Ptr, unsafe { crate::runtime::typelib_ref(typelib) });
abi_void!(gi_typelib_unref(typelib: Ptr));

abi_ret!(gi_union_info_find_method(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::union_find_method(info, name)
});
abi_ret!(gi_union_info_get_alignment(info: Ptr) -> usize, unsafe {
    crate::runtime::union_get_alignment(info)
});
abi_ret!(gi_union_info_get_copy_function_name(info: Ptr) -> ConstChar, ptr::null());
abi_ret!(gi_union_info_get_discriminator(info: Ptr, index: guint) -> Ptr, ptr::null_mut());
abi_ret!(gi_union_info_get_discriminator_offset(info: Ptr, out_offset: *mut usize) -> gboolean, unsafe {
    crate::runtime::union_get_discriminator_offset(info, out_offset)
});
abi_ret!(gi_union_info_get_discriminator_type(info: Ptr) -> Ptr, ptr::null_mut());
abi_ret!(gi_union_info_get_field(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::union_get_field(info, index)
});
abi_ret!(gi_union_info_get_free_function_name(info: Ptr) -> ConstChar, ptr::null());
abi_ret!(gi_union_info_get_method(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::union_get_method(info, index)
});
abi_ret!(gi_union_info_get_n_fields(info: Ptr) -> guint, unsafe {
    crate::runtime::union_get_n_fields(info)
});
abi_ret!(gi_union_info_get_n_methods(info: Ptr) -> guint, unsafe {
    crate::runtime::union_get_n_methods(info)
});
abi_ret!(gi_union_info_get_size(info: Ptr) -> usize, unsafe {
    crate::runtime::union_get_size(info)
});
abi_ret!(gi_union_info_is_discriminated(info: Ptr) -> gboolean, 0);

abi_ret!(gi_vfunc_info_get_invoker(info: Ptr) -> Ptr, unsafe {
    crate::runtime::vfunc_get_invoker(info)
});

abi_zero_arg_symbols!(
    gi_base_info_equal,
    gi_base_info_get_container,
    gi_base_info_get_typelib,
    gi_base_info_is_deprecated,
    gi_base_info_iterate_attributes,
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
    gi_typelib_validate,
    gi_value_info_get_value,
    gi_vfunc_info_get_address,
    gi_vfunc_info_get_flags,
    gi_vfunc_info_get_offset,
    gi_vfunc_info_get_signal,
    gi_vfunc_info_invoke,
);
