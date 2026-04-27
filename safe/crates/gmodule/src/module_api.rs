use std::ffi::CStr;
use std::ptr;

use crate::ffi::{gboolean, gchar, gint, gpointer, GQuark};
use crate::runtime::{
    close_module, duplicate_string, module_error_ptr, module_error_quark, open_main_module, open_named_module, GModule,
};

const TRUE: gboolean = 1;
const FALSE: gboolean = 0;

#[repr(C)]
pub struct GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}

#[unsafe(export_name = "g_module_supported")]
pub unsafe extern "C" fn module_supported() -> gboolean {
    TRUE
}

#[unsafe(export_name = "g_module_open")]
pub unsafe extern "C" fn module_open(file_name: *const gchar, flags: gint) -> *mut GModule {
    module_open_full(file_name, flags, ptr::null_mut())
}

#[unsafe(export_name = "g_module_open_full")]
pub unsafe extern "C" fn module_open_full(
    file_name: *const gchar,
    flags: gint,
    error: *mut *mut GError,
) -> *mut GModule {
    if file_name.is_null() {
        return open_main_module(flags, error.cast());
    }

    let file_name = CStr::from_ptr(file_name);
    if file_name.to_bytes().is_empty() {
        return open_main_module(flags, error.cast());
    }

    open_named_module(file_name, flags, error.cast())
}

#[unsafe(export_name = "g_module_close")]
pub unsafe extern "C" fn module_close(module: *mut GModule) -> gboolean {
    close_module(module)
}

#[unsafe(export_name = "g_module_make_resident")]
pub unsafe extern "C" fn module_make_resident(module: *mut GModule) {
    if !module.is_null() {
        (*module).is_resident = true;
    }
}

#[unsafe(export_name = "g_module_error")]
pub unsafe extern "C" fn module_error() -> *const gchar {
    module_error_ptr()
}

#[unsafe(export_name = "g_module_symbol")]
pub unsafe extern "C" fn module_symbol(
    module: *mut GModule,
    symbol_name: *const gchar,
    symbol: *mut gpointer,
) -> gboolean {
    if !symbol.is_null() {
        *symbol = ptr::null_mut();
    }
    if module.is_null() || symbol_name.is_null() || symbol.is_null() {
        return FALSE;
    }

    match crate::runtime::module_symbol((*module).handle, symbol_name) {
        Ok(resolved) => {
            *symbol = resolved;
            crate::runtime::set_module_error(None);
            TRUE
        }
        Err(error) => {
            let name = CStr::from_ptr(symbol_name).to_string_lossy();
            crate::runtime::set_module_error(Some(format!("'{}': {}", name, error)));
            *symbol = ptr::null_mut();
            FALSE
        }
    }
}

#[unsafe(export_name = "g_module_name")]
pub unsafe extern "C" fn module_name(module: *mut GModule) -> *const gchar {
    if module.is_null() {
        return ptr::null();
    }
    if (*module).is_main {
        return c"main".as_ptr().cast();
    }
    (*module)
        .file_name
        .as_ref()
        .map_or(ptr::null(), |file_name| file_name.as_ptr().cast())
}

#[unsafe(export_name = "g_module_build_path")]
pub unsafe extern "C" fn module_build_path(
    directory: *const gchar,
    module_name: *const gchar,
) -> *mut gchar {
    if module_name.is_null() {
        return ptr::null_mut();
    }

    let module_name = CStr::from_ptr(module_name).to_string_lossy();
    let directory = if directory.is_null() {
        None
    } else {
        Some(CStr::from_ptr(directory).to_string_lossy())
    };

    let result = if let Some(directory) = directory {
        if directory.is_empty() {
            if module_name.starts_with("lib") {
                module_name.into_owned()
            } else {
                format!("lib{}.so", module_name)
            }
        } else if module_name.starts_with("lib") {
            format!("{directory}/{module_name}")
        } else {
            format!("{directory}/lib{module_name}.so")
        }
    } else if module_name.starts_with("lib") {
        module_name.into_owned()
    } else {
        format!("lib{module_name}.so")
    };

    duplicate_string(&result)
}

#[unsafe(export_name = "g_module_error_quark")]
pub unsafe extern "C" fn module_error_quark_export() -> GQuark {
    module_error_quark()
}
