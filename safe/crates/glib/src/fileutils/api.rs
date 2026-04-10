#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use crate::backend;
use crate::ffi::gchar;

#[cfg(unix)]
unsafe extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
}

#[cfg(unix)]
fn is_absolute(path: &[u8]) -> bool {
    matches!(path.first(), Some(b'/'))
}

#[cfg(unix)]
fn build_canonical_path(base: &[u8], filename: &[u8]) -> Vec<u8> {
    let mut joined = Vec::new();
    if is_absolute(filename) {
        joined.extend_from_slice(filename);
    } else {
        joined.extend_from_slice(base);
        if !joined.ends_with(b"/") {
            joined.push(b'/');
        }
        joined.extend_from_slice(filename);
    }

    let prefix_len = if joined.starts_with(b"//") && !joined.starts_with(b"///") {
        2
    } else if joined.starts_with(b"/") {
        1
    } else {
        0
    };

    let mut components = Vec::<&[u8]>::new();
    for part in joined.split(|byte| *byte == b'/') {
        match part {
            [] | b"." => {}
            b".." => {
                components.pop();
            }
            _ => components.push(part),
        }
    }

    let mut canonical = Vec::new();
    for _ in 0..prefix_len {
        canonical.push(b'/');
    }
    for component in components.iter() {
        if !canonical.is_empty() && canonical.last() != Some(&b'/') {
            canonical.push(b'/');
        }
        canonical.extend_from_slice(component);
    }
    if canonical.is_empty() {
        canonical.push(b'/');
    }
    canonical
}

#[unsafe(export_name = "g_canonicalize_filename")]
pub unsafe extern "C" fn canonicalize_filename(
    filename: *const gchar,
    relative_to: *const gchar,
) -> *mut gchar {
    #[cfg(unix)]
    {
        if filename.is_null() {
            return backend::canonicalize_filename(filename, relative_to);
        }

        let filename = std::ffi::CStr::from_ptr(filename.cast()).to_bytes();
        let base = if is_absolute(filename) {
            Vec::new()
        } else if relative_to.is_null() {
            match std::env::current_dir() {
                Ok(path) => path.as_os_str().as_bytes().to_vec(),
                Err(_) => return backend::canonicalize_filename(filename.as_ptr().cast(), relative_to),
            }
        } else {
            let relative_to = std::ffi::CStr::from_ptr(relative_to.cast()).to_bytes();
            if !is_absolute(relative_to) {
                return backend::canonicalize_filename(filename.as_ptr().cast(), relative_to.as_ptr().cast());
            }
            relative_to.to_vec()
        };

        let canonical = build_canonical_path(&base, filename);
        let out = malloc(canonical.len() + 1).cast::<u8>();
        if out.is_null() {
            return core::ptr::null_mut();
        }
        core::ptr::copy_nonoverlapping(canonical.as_ptr(), out, canonical.len());
        *out.add(canonical.len()) = 0;
        return out.cast();
    }

    #[cfg(not(unix))]
    {
        backend::canonicalize_filename(filename, relative_to)
    }
}
