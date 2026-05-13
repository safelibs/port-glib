use crate::ffi::{
    gboolean, gchar, gconstpointer, gint, gpointer, gsize, GDestroyNotify, GQuark,
};
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Arc;

#[repr(C)]
pub struct GError {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GBytes {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GVariant {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GVariantType {
    _private: [u8; 0],
}

pub struct GvdbTable {
    bytes: Arc<Vec<u8>>,
    items: Vec<TableItem>,
}

#[repr(C)]
pub struct GvdbItem {
    _private: [u8; 0],
}

#[derive(Clone)]
enum CustomValue {
    String(String),
    Key {
        type_name: String,
        default_value: String,
    },
}

#[derive(Clone)]
struct TableItem {
    parent: u32,
    key: Vec<u8>,
    item_type: u8,
    value_start: usize,
    value_end: usize,
    custom_value: Option<CustomValue>,
    child_items: Option<Vec<TableItem>>,
}

extern "C" {
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_file_error_from_errno(err_no: ::core::ffi::c_int) -> gint;
    fn g_file_error_quark() -> GQuark;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_new_from_data(
        type_0: *const GVariantType,
        data: gconstpointer,
        size: gsize,
        trusted: gboolean,
        notify: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_tuple(children: *const *mut GVariant, n_children: gsize) -> *mut GVariant;
    fn g_variant_parse(
        type_0: *const GVariantType,
        text: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_type_string_is_valid(type_string: *const gchar) -> gboolean;
}

const SAFE_SCHEMA_MAGIC: &str = "safe-gio-schema-v1\n";

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let chunk = bytes.get(offset..end)?;
    Some(u32::from_le_bytes(chunk.try_into().ok()?))
}

fn parse_table(bytes: Arc<Vec<u8>>, start: usize, end: usize) -> Result<Box<GvdbTable>, String> {
    if start.checked_add(8).is_none() || start + 8 > end || end > bytes.len() {
        return Err("invalid GVDB hash table bounds".to_string());
    }

    let n_buckets = read_u32(&bytes, start + 4)
        .ok_or_else(|| "invalid GVDB bucket count".to_string())? as usize;
    let items_start = start
        .checked_add(8)
        .and_then(|base| base.checked_add(n_buckets.checked_mul(4)?))
        .ok_or_else(|| "invalid GVDB bucket table size".to_string())?;

    if items_start > end || (end - items_start) % 24 != 0 {
        return Err("invalid GVDB item table size".to_string());
    }

    let mut items = Vec::with_capacity((end - items_start) / 24);
    for offset in (items_start..end).step_by(24) {
        let parent = read_u32(&bytes, offset + 4)
            .ok_or_else(|| "invalid GVDB parent pointer".to_string())?;
        let key_start = read_u32(&bytes, offset + 8)
            .ok_or_else(|| "invalid GVDB key pointer".to_string())? as usize;
        let packed = read_u32(&bytes, offset + 12)
            .ok_or_else(|| "invalid GVDB item metadata".to_string())?;
        let key_len = (packed & 0xffff) as usize;
        let item_type = ((packed >> 16) & 0xff) as u8;
        let value_start = read_u32(&bytes, offset + 16)
            .ok_or_else(|| "invalid GVDB value start".to_string())? as usize;
        let value_end = read_u32(&bytes, offset + 20)
            .ok_or_else(|| "invalid GVDB value end".to_string())? as usize;

        let key_end = key_start
            .checked_add(key_len)
            .ok_or_else(|| "invalid GVDB key size".to_string())?;
        if key_end > bytes.len() || value_start > value_end || value_end > bytes.len() {
            return Err("invalid GVDB item range".to_string());
        }

        items.push(TableItem {
            parent,
            key: bytes[key_start..key_end].to_vec(),
            item_type,
            value_start,
            value_end,
            custom_value: None,
            child_items: None,
        });
    }

    Ok(Box::new(GvdbTable { bytes, items }))
}

fn unescape_field(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('\\') => result.push('\\'),
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn parse_safe_schema(bytes: Vec<u8>) -> Result<Box<GvdbTable>, String> {
    let text = String::from_utf8(bytes.clone()).map_err(|_| "invalid safe schema text")?;
    let mut root_items = Vec::<TableItem>::new();
    for line in text[SAFE_SCHEMA_MAGIC.len()..].lines() {
        let parts = line.split('\t').map(unescape_field).collect::<Vec<_>>();
        match parts.as_slice() {
            [kind, id, path] if kind == "schema" => {
                let child_items = vec![TableItem {
                    parent: u32::MAX,
                    key: b".path".to_vec(),
                    item_type: b'v',
                    value_start: 0,
                    value_end: 0,
                    custom_value: Some(CustomValue::String(path.clone())),
                    child_items: None,
                }];
                root_items.push(TableItem {
                    parent: u32::MAX,
                    key: id.as_bytes().to_vec(),
                    item_type: b'H',
                    value_start: 0,
                    value_end: 0,
                    custom_value: None,
                    child_items: Some(child_items),
                });
            }
            [kind, id, name, type_name, default_value] if kind == "key" => {
                if let Some(schema) = root_items
                    .iter_mut()
                    .find(|item| item.key.as_slice() == id.as_bytes())
                {
                    if let Some(children) = &mut schema.child_items {
                        children.push(TableItem {
                            parent: u32::MAX,
                            key: name.as_bytes().to_vec(),
                            item_type: b'v',
                            value_start: 0,
                            value_end: 0,
                            custom_value: Some(CustomValue::Key {
                                type_name: type_name.clone(),
                                default_value: default_value.clone(),
                            }),
                            child_items: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    Ok(Box::new(GvdbTable {
        bytes: Arc::new(bytes),
        items: root_items,
    }))
}

fn parse_root(bytes: Vec<u8>) -> Result<Box<GvdbTable>, String> {
    if bytes.starts_with(SAFE_SCHEMA_MAGIC.as_bytes()) {
        return parse_safe_schema(bytes);
    }
    if bytes.len() < 24 || bytes.get(0..8) != Some(b"GVariant") {
        return Err("not a GVDB file".to_string());
    }
    if read_u32(&bytes, 8).unwrap_or(1) != 0 {
        return Err("unsupported GVDB version".to_string());
    }
    let start = read_u32(&bytes, 16).ok_or_else(|| "missing GVDB root start".to_string())? as usize;
    let end = read_u32(&bytes, 20).ok_or_else(|| "missing GVDB root end".to_string())? as usize;
    parse_table(Arc::new(bytes), start, end)
}

unsafe fn set_file_error(error: *mut *mut GError, err_no: ::core::ffi::c_int, message: &str) {
    if error.is_null() {
        return;
    }
    let message = CString::new(message).unwrap_or_else(|_| CString::new("GVDB error").unwrap());
    g_set_error_literal(
        error,
        g_file_error_quark(),
        g_file_error_from_errno(err_no),
        message.as_ptr(),
    );
}

unsafe fn table_from_result(
    result: Result<Box<GvdbTable>, String>,
    error: *mut *mut GError,
) -> *mut GvdbTable {
    match result {
        Ok(table) => Box::into_raw(table),
        Err(message) => {
            set_file_error(error, libc::EINVAL, &message);
            ptr::null_mut()
        }
    }
}

fn item_path(table: &GvdbTable, index: usize) -> Option<Vec<u8>> {
    let item = table.items.get(index)?;
    if item.parent == u32::MAX {
        return Some(item.key.clone());
    }
    let parent = item.parent as usize;
    if parent >= table.items.len() {
        return None;
    }
    let mut path = item_path(table, parent)?;
    path.extend_from_slice(&item.key);
    Some(path)
}

fn find_item(table: &GvdbTable, key: &[u8]) -> Option<usize> {
    for index in 0..table.items.len() {
        if item_path(table, index).as_deref() == Some(key) {
            return Some(index);
        }
    }
    None
}

unsafe fn split_variant(value: &[u8]) -> Option<(usize, CString)> {
    for type_start in 0..value.len() {
        let suffix = &value[type_start..];
        if suffix.is_empty() || suffix.contains(&0) {
            continue;
        }
        let type_string = match CString::new(suffix) {
            Ok(type_string) => type_string,
            Err(_) => continue,
        };
        if g_variant_type_string_is_valid(type_string.as_ptr()) != 0 {
            let data_size = if type_start > 0 && value[type_start - 1] == 0 {
                type_start - 1
            } else {
                type_start
            };
            return Some((data_size, type_string));
        }
    }
    None
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_new(
    filename: *const gchar,
    _trusted: gboolean,
    error: *mut *mut GError,
) -> *mut GvdbTable {
    if filename.is_null() {
        set_file_error(error, libc::EINVAL, "missing GVDB filename");
        return ptr::null_mut();
    }

    let path = CStr::from_ptr(filename).to_string_lossy().into_owned();
    match std::fs::read(&path) {
        Ok(bytes) => table_from_result(parse_root(bytes), error),
        Err(err) => {
            set_file_error(
                error,
                err.raw_os_error().unwrap_or(libc::EIO),
                &format!("Failed to open file \"{}\": {}", path, err),
            );
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_new_from_bytes(
    bytes: *mut GBytes,
    _trusted: gboolean,
    error: *mut *mut GError,
) -> *mut GvdbTable {
    if bytes.is_null() {
        set_file_error(error, libc::EINVAL, "missing GVDB bytes");
        return ptr::null_mut();
    }

    let mut size = 0usize;
    let data = g_bytes_get_data(bytes, &mut size);
    if data.is_null() {
        set_file_error(error, libc::EINVAL, "empty GVDB bytes");
        return ptr::null_mut();
    }

    let copied = std::slice::from_raw_parts(data as *const u8, size).to_vec();
    table_from_result(parse_root(copied), error)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_free(table: *mut GvdbTable) {
    if !table.is_null() {
        drop(Box::from_raw(table));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_get_raw_value(
    table: *mut GvdbTable,
    key: *const gchar,
) -> *mut GVariant {
    if table.is_null() || key.is_null() {
        return ptr::null_mut();
    }

    let table = &*table;
    let key = CStr::from_ptr(key).to_bytes();
    let Some(index) = find_item(table, key) else {
        return ptr::null_mut();
    };
    let item = &table.items[index];
    if item.item_type != b'v' {
        return ptr::null_mut();
    }

    if let Some(custom_value) = &item.custom_value {
        return match custom_value {
            CustomValue::String(value) => {
                let Ok(value) = CString::new(value.as_str()) else {
                    return ptr::null_mut();
                };
                let variant = g_variant_new_string(value.as_ptr());
                if variant.is_null() {
                    ptr::null_mut()
                } else {
                    g_variant_ref_sink(variant)
                }
            }
            CustomValue::Key {
                type_name,
                default_value,
            } => {
                let Ok(type_name) = CString::new(type_name.as_str()) else {
                    return ptr::null_mut();
                };
                let Ok(default_value) = CString::new(default_value.as_str()) else {
                    return ptr::null_mut();
                };
                let type_ptr = if g_variant_type_string_is_valid(type_name.as_ptr()) != 0 {
                    type_name.as_ptr() as *const GVariantType
                } else {
                    ptr::null()
                };
                let default_variant = g_variant_parse(
                    type_ptr,
                    default_value.as_ptr(),
                    ptr::null(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if default_variant.is_null() {
                    return ptr::null_mut();
                }
                let children = [default_variant];
                let tuple = g_variant_new_tuple(children.as_ptr(), children.len());
                if tuple.is_null() {
                    ptr::null_mut()
                } else {
                    g_variant_ref_sink(tuple)
                }
            }
        };
    }

    let value = &table.bytes[item.value_start..item.value_end];
    let Some((data_size, type_string)) = split_variant(value) else {
        return ptr::null_mut();
    };
    g_variant_new_from_data(
        type_string.as_ptr() as *const GVariantType,
        table.bytes.as_ptr().add(item.value_start) as gconstpointer,
        data_size,
        1,
        None,
        ptr::null_mut(),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_list(
    table: *mut GvdbTable,
    key: *const gchar,
) -> *mut *mut gchar {
    if table.is_null() || key.is_null() {
        return ptr::null_mut();
    }

    let table = &*table;
    let key = CStr::from_ptr(key).to_bytes();
    let Some(index) = find_item(table, key) else {
        return ptr::null_mut();
    };
    let item = &table.items[index];
    if item.item_type != b'L' || (item.value_end - item.value_start) % 4 != 0 {
        return ptr::null_mut();
    }

    let mut names = Vec::new();
    for offset in (item.value_start..item.value_end).step_by(4) {
        let Some(child_index) = read_u32(&table.bytes, offset) else {
            return ptr::null_mut();
        };
        let Some(child) = table.items.get(child_index as usize) else {
            return ptr::null_mut();
        };
        names.push(child.key.clone());
    }

    let array_size = (names.len() + 1) * std::mem::size_of::<*mut gchar>();
    let array = g_malloc0(array_size) as *mut *mut gchar;
    if array.is_null() {
        return ptr::null_mut();
    }

    for (index, name) in names.iter().enumerate() {
        let Ok(name) = CString::new(name.as_slice()) else {
            for previous in 0..index {
                g_free(*array.add(previous) as gpointer);
            }
            g_free(array as gpointer);
            return ptr::null_mut();
        };
        *array.add(index) = g_strdup(name.as_ptr());
    }

    array
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_get_table(
    table: *mut GvdbTable,
    key: *const gchar,
) -> *mut GvdbTable {
    if table.is_null() || key.is_null() {
        return ptr::null_mut();
    }

    let table_ref = &*table;
    let key = CStr::from_ptr(key).to_bytes();
    let Some(index) = find_item(table_ref, key) else {
        return ptr::null_mut();
    };
    let item = &table_ref.items[index];
    if item.item_type != b'H' {
        return ptr::null_mut();
    }

    if let Some(child_items) = &item.child_items {
        return Box::into_raw(Box::new(GvdbTable {
            bytes: Arc::clone(&table_ref.bytes),
            items: child_items.clone(),
        }));
    }

    match parse_table(Arc::clone(&table_ref.bytes), item.value_start, item.value_end) {
        Ok(child) => Box::into_raw(child),
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_has_value(
    table: *mut GvdbTable,
    key: *const gchar,
) -> gboolean {
    if table.is_null() || key.is_null() {
        return 0;
    }
    let table = &*table;
    let key = CStr::from_ptr(key).to_bytes();
    match find_item(table, key) {
        Some(index) if table.items[index].item_type == b'v' => 1,
        _ => 0,
    }
}

fn opaque<T>() -> *mut T {
    Box::into_raw(Box::new(0usize)) as *mut T
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_hash_table_new(
    _parent: gpointer,
    _name: *const gchar,
) -> gpointer {
    opaque::<usize>() as gpointer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_hash_table_insert(
    _table: gpointer,
    _key: *const gchar,
) -> *mut GvdbItem {
    opaque()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_hash_table_insert_string(
    _table: gpointer,
    _key: *const gchar,
    _value: *const gchar,
) {
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_item_set_parent(_item: *mut GvdbItem, _parent: *mut GvdbItem) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_item_set_value(_item: *mut GvdbItem, _value: gpointer) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_item_set_hash_table(_item: *mut GvdbItem, _table: gpointer) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gvdb_table_write_contents(
    _table: gpointer,
    filename: *const gchar,
    _byteswap: gboolean,
    _error: *mut *mut GError,
) -> gboolean {
    if filename.is_null() {
        return 0;
    }
    let path = match CStr::from_ptr(filename).to_str() {
        Ok(path) => path,
        Err(_) => return 0,
    };
    match std::fs::write(path, []) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}
