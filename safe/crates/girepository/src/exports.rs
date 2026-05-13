#![allow(dead_code)]

use crate::abi::{GIArgInfo, GIArgument, GIAttributeIter, GITypeInfo};
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
abi_ret!(gi_repository_require_private(repository: Ptr, typelib_dir: ConstChar, namespace_: ConstChar, version: ConstChar, flags: c_int, error: GErrorOut) -> Ptr, unsafe {
    crate::runtime::repository_require_private(repository, typelib_dir, namespace_, version, flags, error)
});
abi_ret!(gi_repository_load_typelib(repository: Ptr, typelib: Ptr, flags: c_int, error: GErrorOut) -> ConstChar, unsafe {
    crate::runtime::repository_load_typelib(repository, typelib, flags, error)
});
abi_ret!(gi_repository_is_registered(repository: Ptr, namespace_: ConstChar, version: ConstChar) -> gboolean, unsafe {
    crate::runtime::repository_is_registered(repository, namespace_, version)
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
abi_ret!(gi_repository_get_version(repository: Ptr, namespace_: ConstChar) -> ConstChar, unsafe {
    crate::runtime::get_version(repository, namespace_)
});
abi_ret!(gi_repository_get_shared_libraries(repository: Ptr, namespace_: ConstChar, out_n_elements: *mut usize) -> ConstCharStrv, unsafe {
    crate::runtime::get_shared_libraries(repository, namespace_, out_n_elements)
});
abi_ret!(gi_repository_get_typelib_path(repository: Ptr, namespace_: ConstChar) -> ConstChar, unsafe {
    crate::runtime::get_typelib_path(repository, namespace_)
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
abi_ret!(gi_repository_error_quark() -> GQuark, crate::runtime::repository_error_quark());

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
abi_ret!(gi_invoke_error_quark() -> GQuark, crate::runtime::invoke_error_quark());

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
#[export_name = "gi_typelib_unref"]
pub unsafe extern "C" fn gi_typelib_unref(typelib: Ptr) {
    unsafe { crate::runtime::typelib_unref(typelib) }
}
abi_ret!(gi_typelib_new_from_bytes(bytes: Ptr, error: GErrorOut) -> Ptr, unsafe {
    crate::runtime::typelib_new_from_bytes(bytes, error)
});
abi_ret!(gi_typelib_get_namespace(typelib: Ptr) -> ConstChar, unsafe {
    crate::runtime::typelib_get_namespace(typelib)
});
abi_ret!(gi_typelib_symbol(typelib: Ptr, symbol_name: ConstChar, symbol: *mut Ptr) -> gboolean, unsafe {
    crate::runtime::typelib_symbol(typelib, symbol_name, symbol)
});
abi_ret!(gi_typelib_validate(typelib: Ptr, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::typelib_validate(typelib, error)
});

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

abi_ret!(gi_base_info_equal(info1: Ptr, info2: Ptr) -> gboolean, unsafe {
    crate::runtime::base_info_equal(info1, info2)
});
abi_ret!(gi_base_info_get_container(info: Ptr) -> Ptr, unsafe {
    crate::runtime::base_info_get_container(info)
});
abi_ret!(gi_base_info_get_typelib(info: Ptr) -> Ptr, unsafe {
    crate::runtime::base_info_get_typelib(info)
});
abi_ret!(gi_base_info_is_deprecated(info: Ptr) -> gboolean, unsafe {
    crate::runtime::base_info_is_deprecated(info)
});
abi_ret!(gi_base_info_iterate_attributes(info: Ptr, iterator: *mut GIAttributeIter, name: *mut ConstChar, value: *mut ConstChar) -> gboolean, unsafe {
    crate::runtime::base_info_iterate_attributes(info, iterator, name, value)
});

abi_ret!(gi_callable_info_create_closure(info: Ptr, cif: Ptr, callback: Ptr, user_data: Ptr) -> Ptr, unsafe {
    crate::runtime::callable_create_closure(info, cif, callback, user_data)
});
#[export_name = "gi_callable_info_destroy_closure"]
pub unsafe extern "C" fn gi_callable_info_destroy_closure(info: Ptr, closure: Ptr) {
    unsafe { crate::runtime::callable_destroy_closure(info, closure) }
}
abi_ret!(gi_callable_info_get_closure_native_address(info: Ptr, closure: Ptr) -> *mut Ptr, unsafe {
    crate::runtime::callable_get_closure_native_address(info, closure)
});
abi_ret!(gi_callable_info_invoke(info: Ptr, function: Ptr, in_args: *const GIArgument, n_in_args: usize, out_args: *mut GIArgument, n_out_args: usize, return_value: *mut GIArgument, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::callable_invoke(info, function, in_args, n_in_args, out_args, n_out_args, return_value, error)
});
#[export_name = "gi_cclosure_marshal_generic"]
pub unsafe extern "C" fn gi_cclosure_marshal_generic(
    closure: Ptr,
    return_value: Ptr,
    n_param_values: guint,
    param_values: Ptr,
    invocation_hint: Ptr,
    marshal_data: Ptr,
) {
    unsafe {
        crate::runtime::cclosure_marshal_generic(
            closure,
            return_value,
            n_param_values,
            param_values,
            invocation_hint,
            marshal_data,
        )
    }
}

#[export_name = "gi_constant_info_free_value"]
pub unsafe extern "C" fn gi_constant_info_free_value(info: Ptr, value: *mut GIArgument) {
    unsafe { crate::runtime::constant_free_value(info, value) }
}
abi_ret!(gi_constant_info_get_type_info(info: Ptr) -> Ptr, unsafe {
    crate::runtime::constant_get_type_info(info)
});
abi_ret!(gi_constant_info_get_value(info: Ptr, value: *mut GIArgument) -> usize, unsafe {
    crate::runtime::constant_get_value(info, value)
});

abi_ret!(gi_enum_info_get_error_domain(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::enum_get_error_domain(info)
});
abi_ret!(gi_enum_info_get_storage_type(info: Ptr) -> c_int, unsafe {
    crate::runtime::enum_get_storage_type(info)
});

abi_ret!(gi_field_info_get_field(info: Ptr, mem: Ptr, value: *mut GIArgument) -> gboolean, unsafe {
    crate::runtime::field_get_field(info, mem, value)
});
abi_ret!(gi_field_info_get_flags(info: Ptr) -> c_int, unsafe {
    crate::runtime::field_get_flags(info)
});
abi_ret!(gi_field_info_get_offset(info: Ptr) -> usize, unsafe {
    crate::runtime::field_get_offset(info)
});
abi_ret!(gi_field_info_get_size(info: Ptr) -> usize, unsafe {
    crate::runtime::field_get_size(info)
});
abi_ret!(gi_field_info_set_field(info: Ptr, mem: Ptr, value: *const GIArgument) -> gboolean, unsafe {
    crate::runtime::field_set_field(info, mem, value)
});

abi_ret!(gi_function_info_get_property(info: Ptr) -> Ptr, unsafe {
    crate::runtime::function_get_property(info)
});
abi_ret!(gi_function_info_get_vfunc(info: Ptr) -> Ptr, unsafe {
    crate::runtime::function_get_vfunc(info)
});
abi_ret!(gi_function_invoker_new_for_address(address: Ptr, info: Ptr, invoker: Ptr, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::function_invoker_new_for_address(address, info, invoker, error)
});

abi_ret!(gi_interface_info_find_signal(info: Ptr, name: ConstChar) -> Ptr, unsafe {
    crate::runtime::interface_find_signal(info, name)
});
abi_ret!(gi_interface_info_get_constant(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_constant(info, index)
});
abi_ret!(gi_interface_info_get_iface_struct(info: Ptr) -> Ptr, unsafe {
    crate::runtime::interface_get_iface_struct(info)
});
abi_ret!(gi_interface_info_get_method(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_method(info, index)
});
abi_ret!(gi_interface_info_get_n_constants(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_constants(info)
});
abi_ret!(gi_interface_info_get_n_methods(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_methods(info)
});
abi_ret!(gi_interface_info_get_n_prerequisites(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_prerequisites(info)
});
abi_ret!(gi_interface_info_get_n_properties(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_properties(info)
});
abi_ret!(gi_interface_info_get_n_signals(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_signals(info)
});
abi_ret!(gi_interface_info_get_n_vfuncs(info: Ptr) -> guint, unsafe {
    crate::runtime::interface_get_n_vfuncs(info)
});
abi_ret!(gi_interface_info_get_prerequisite(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_prerequisite(info, index)
});
abi_ret!(gi_interface_info_get_property(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_property(info, index)
});
abi_ret!(gi_interface_info_get_signal(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_signal(info, index)
});
abi_ret!(gi_interface_info_get_vfunc(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::interface_get_vfunc(info, index)
});

abi_ret!(gi_object_info_get_abstract(info: Ptr) -> gboolean, unsafe {
    crate::runtime::object_get_abstract(info)
});
abi_ret!(gi_object_info_get_class_struct(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_class_struct(info)
});
abi_ret!(gi_object_info_get_constant(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_constant(info, index)
});
abi_ret!(gi_object_info_get_field(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_field(info, index)
});
abi_ret!(gi_object_info_get_final(info: Ptr) -> gboolean, unsafe {
    crate::runtime::object_get_final(info)
});
abi_ret!(gi_object_info_get_fundamental(info: Ptr) -> gboolean, unsafe {
    crate::runtime::object_get_fundamental(info)
});
abi_ret!(gi_object_info_get_get_value_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_get_value_function_name(info)
});
abi_ret!(gi_object_info_get_get_value_function_pointer(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_get_value_function_pointer(info)
});
abi_ret!(gi_object_info_get_interface(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_interface(info, index)
});
abi_ret!(gi_object_info_get_n_constants(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_constants(info)
});
abi_ret!(gi_object_info_get_n_fields(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_fields(info)
});
abi_ret!(gi_object_info_get_n_interfaces(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_interfaces(info)
});
abi_ret!(gi_object_info_get_n_properties(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_properties(info)
});
abi_ret!(gi_object_info_get_n_signals(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_signals(info)
});
abi_ret!(gi_object_info_get_n_vfuncs(info: Ptr) -> guint, unsafe {
    crate::runtime::object_get_n_vfuncs(info)
});
abi_ret!(gi_object_info_get_parent(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_parent(info)
});
abi_ret!(gi_object_info_get_ref_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_ref_function_name(info)
});
abi_ret!(gi_object_info_get_set_value_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_set_value_function_name(info)
});
abi_ret!(gi_object_info_get_set_value_function_pointer(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_set_value_function_pointer(info)
});
abi_ret!(gi_object_info_get_signal(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_signal(info, index)
});
abi_ret!(gi_object_info_get_type_init_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_type_init_function_name(info)
});
abi_ret!(gi_object_info_get_type_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_type_name(info)
});
abi_ret!(gi_object_info_get_unref_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::object_get_unref_function_name(info)
});
abi_ret!(gi_object_info_get_unref_function_pointer(info: Ptr) -> Ptr, unsafe {
    crate::runtime::object_get_unref_function_pointer(info)
});
abi_ret!(gi_object_info_get_vfunc(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::object_get_vfunc(info, index)
});

abi_ret!(gi_property_info_get_flags(info: Ptr) -> c_int, unsafe {
    crate::runtime::property_get_flags(info)
});
abi_ret!(gi_property_info_get_getter(info: Ptr) -> Ptr, unsafe {
    crate::runtime::property_get_getter(info)
});
abi_ret!(gi_property_info_get_ownership_transfer(info: Ptr) -> c_int, unsafe {
    crate::runtime::property_get_ownership_transfer(info)
});
abi_ret!(gi_property_info_get_setter(info: Ptr) -> Ptr, unsafe {
    crate::runtime::property_get_setter(info)
});
abi_ret!(gi_property_info_get_type_info(info: Ptr) -> Ptr, unsafe {
    crate::runtime::property_get_type_info(info)
});

abi_ret!(gi_repository_dump(input_filename: ConstChar, output_filename: ConstChar, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::repository_dump(input_filename, output_filename, error)
});
abi_ret!(gi_repository_get_option_group() -> Ptr, unsafe {
    crate::runtime::repository_get_option_group()
});

abi_ret!(gi_signal_info_get_class_closure(info: Ptr) -> Ptr, unsafe {
    crate::runtime::signal_get_class_closure(info)
});
abi_ret!(gi_signal_info_true_stops_emit(info: Ptr) -> gboolean, unsafe {
    crate::runtime::signal_true_stops_emit(info)
});

abi_ret!(gi_struct_info_get_alignment(info: Ptr) -> usize, unsafe {
    crate::runtime::struct_get_alignment(info)
});
abi_ret!(gi_struct_info_get_copy_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::struct_get_copy_function_name(info)
});
abi_ret!(gi_struct_info_get_free_function_name(info: Ptr) -> ConstChar, unsafe {
    crate::runtime::struct_get_free_function_name(info)
});
abi_ret!(gi_struct_info_get_method(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::struct_get_method(info, index)
});
abi_ret!(gi_struct_info_get_n_methods(info: Ptr) -> guint, unsafe {
    crate::runtime::struct_get_n_methods(info)
});
abi_ret!(gi_struct_info_is_foreign(info: Ptr) -> gboolean, unsafe {
    crate::runtime::struct_is_foreign(info)
});

#[export_name = "gi_type_info_argument_from_hash_pointer"]
pub unsafe extern "C" fn gi_type_info_argument_from_hash_pointer(
    info: Ptr,
    hash_pointer: Ptr,
    arg: *mut GIArgument,
) {
    unsafe { crate::runtime::type_argument_from_hash_pointer(info, hash_pointer, arg) }
}
#[export_name = "gi_type_info_extract_ffi_return_value"]
pub unsafe extern "C" fn gi_type_info_extract_ffi_return_value(
    info: Ptr,
    ffi_value: *const GIArgument,
    arg: *mut GIArgument,
) {
    unsafe { crate::runtime::type_extract_ffi_return_value(info, ffi_value, arg) }
}
abi_ret!(gi_type_info_get_array_fixed_size(info: Ptr, out_size: *mut usize) -> gboolean, unsafe {
    crate::runtime::type_get_array_fixed_size(info, out_size)
});
abi_ret!(gi_type_info_get_ffi_type(info: Ptr) -> Ptr, unsafe {
    crate::runtime::type_get_ffi_type(info)
});
abi_ret!(gi_type_info_get_param_type(info: Ptr, index: guint) -> Ptr, unsafe {
    crate::runtime::type_get_param_type(info, index)
});
abi_ret!(gi_type_info_get_storage_type(info: Ptr) -> c_int, unsafe {
    crate::runtime::type_get_storage_type(info)
});
abi_ret!(gi_type_info_hash_pointer_from_argument(info: Ptr, arg: *const GIArgument) -> Ptr, unsafe {
    crate::runtime::type_hash_pointer_from_argument(info, arg)
});

#[export_name = "gi_type_tag_argument_from_hash_pointer"]
pub unsafe extern "C" fn gi_type_tag_argument_from_hash_pointer(
    tag: c_int,
    hash_pointer: Ptr,
    arg: *mut GIArgument,
) {
    unsafe { crate::runtime::type_tag_argument_from_hash_pointer(tag, hash_pointer, arg) }
}
#[export_name = "gi_type_tag_extract_ffi_return_value"]
pub unsafe extern "C" fn gi_type_tag_extract_ffi_return_value(
    return_tag: c_int,
    interface_type: GType,
    ffi_value: *const GIArgument,
    arg: *mut GIArgument,
) {
    unsafe {
        crate::runtime::type_tag_extract_ffi_return_value(
            return_tag,
            interface_type,
            ffi_value,
            arg,
        )
    }
}
abi_ret!(gi_type_tag_get_ffi_type(tag: c_int, is_pointer: gboolean) -> Ptr, unsafe {
    crate::runtime::type_tag_get_ffi_type(tag, is_pointer)
});
abi_ret!(gi_type_tag_hash_pointer_from_argument(tag: c_int, arg: *const GIArgument) -> Ptr, unsafe {
    crate::runtime::type_tag_hash_pointer_from_argument(tag, arg)
});
abi_ret!(gi_type_tag_to_string(tag: c_int) -> ConstChar, crate::runtime::type_tag_to_string(tag));

abi_ret!(gi_value_info_get_value(info: Ptr) -> i64, unsafe {
    crate::runtime::value_get_value(info)
});

abi_ret!(gi_vfunc_info_get_address(info: Ptr, implementor_gtype: GType, error: GErrorOut) -> Ptr, unsafe {
    crate::runtime::vfunc_get_address(info, implementor_gtype, error)
});
abi_ret!(gi_vfunc_info_get_flags(info: Ptr) -> c_int, unsafe {
    crate::runtime::vfunc_get_flags(info)
});
abi_ret!(gi_vfunc_info_get_offset(info: Ptr) -> usize, unsafe {
    crate::runtime::vfunc_get_offset(info)
});
abi_ret!(gi_vfunc_info_get_signal(info: Ptr) -> Ptr, unsafe {
    crate::runtime::vfunc_get_signal(info)
});
abi_ret!(gi_vfunc_info_invoke(info: Ptr, implementor: GType, in_args: *const GIArgument, n_in_args: usize, out_args: *mut GIArgument, n_out_args: usize, return_value: *mut GIArgument, error: GErrorOut) -> gboolean, unsafe {
    crate::runtime::vfunc_invoke(info, implementor, in_args, n_in_args, out_args, n_out_args, return_value, error)
});
