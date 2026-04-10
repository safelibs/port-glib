use std::collections::HashSet;
use std::ffi::{c_void, CStr};
use std::sync::{Mutex, OnceLock};

use crate::abi::{GList, GPtrArray};
use crate::ffi::{
    gboolean, gconstpointer, gint, gpointer, gsize, guint, GDestroyNotify,
};

type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer)>;
type GHRFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;

#[repr(C)]
pub(crate) struct GHashTable {
    size: gsize,
    mod_: gint,
    mask: guint,
    nnodes: guint,
    noccupied: guint,
    flags: guint,
    keys: *mut gpointer,
    hashes: *mut guint,
    values: *mut gpointer,
    hash_func: GHashFunc,
    key_equal_func: GEqualFunc,
    ref_count: gint,
    version: gint,
    key_destroy_func: GDestroyNotify,
    value_destroy_func: GDestroyNotify,
    keys_storage: Vec<gpointer>,
    hashes_storage: Vec<guint>,
    values_storage: Vec<gpointer>,
}

#[repr(C)]
pub(crate) struct GHashTableIter {
    hash_table: *mut GHashTable,
    dummy1: gpointer,
    dummy2: gpointer,
    position: gint,
    dummy3: gboolean,
    version: isize,
}

#[allow(improper_ctypes)]
unsafe extern "C" {
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

const TRUE: gboolean = 1;
const FALSE: gboolean = 0;
const HASH_UNUSED: guint = 0;
const HASH_TOMBSTONE: guint = 1;
const FLAG_HAVE_BIG_KEYS: guint = 1;
const FLAG_HAVE_BIG_VALUES: guint = 2;
const MIN_SIZE: usize = 8;
const PRIME_MOD: [gint; 32] = [
    1, 2, 3, 7, 13, 31, 61, 127, 251, 509, 1021, 2039, 4093, 8191, 16381, 32749,
    65521, 131071, 262139, 524287, 1048573, 2097143, 4194301, 8388593, 16777213,
    33554393, 67108859, 134217689, 268435399, 536870909, 1073741789, 2147483647,
];

fn owned_tables() -> &'static Mutex<HashSet<usize>> {
    static TABLES: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();
    TABLES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn register_owned_table(table: *mut GHashTable) {
    owned_tables()
        .lock()
        .expect("hash table registry mutex poisoned")
        .insert(table as usize);
}

fn unregister_owned_table(table: *mut GHashTable) {
    owned_tables()
        .lock()
        .expect("hash table registry mutex poisoned")
        .remove(&(table as usize));
}

fn is_owned_table(table: *mut GHashTable) -> bool {
    if table.is_null() {
        return false;
    }
    owned_tables()
        .lock()
        .expect("hash table registry mutex poisoned")
        .contains(&(table as usize))
}

fn normalize_hash(hash: guint) -> guint {
    match hash {
        0 => 2,
        1 => 3,
        _ => hash,
    }
}

fn shift_for_size(size: usize) -> usize {
    debug_assert!(size.is_power_of_two());
    size.trailing_zeros() as usize
}

fn iter_index_to_ptr(index: usize) -> gpointer {
    (index + 1) as usize as gpointer
}

fn ptr_to_iter_index(value: gpointer) -> Option<usize> {
    (!value.is_null()).then_some((value as usize) - 1)
}

unsafe fn sync_views(table: *mut GHashTable) {
    (*table).keys = (*table).keys_storage.as_mut_ptr();
    (*table).hashes = (*table).hashes_storage.as_mut_ptr();
    (*table).values = (*table).values_storage.as_mut_ptr();
    (*table).size = (*table).keys_storage.len();
    (*table).mod_ = PRIME_MOD[shift_for_size((*table).size)] as gint;
    (*table).mask = ((*table).size.saturating_sub(1)) as guint;
    (*table).flags = FLAG_HAVE_BIG_KEYS | FLAG_HAVE_BIG_VALUES;
}

unsafe fn resize_storage(table: *mut GHashTable, capacity: usize) {
    (*table).keys_storage.resize(capacity, core::ptr::null_mut());
    (*table).hashes_storage.resize(capacity, HASH_UNUSED);
    (*table).values_storage.resize(capacity, core::ptr::null_mut());
    sync_views(table);
}

unsafe fn make_owned_table(
    hash_func: GHashFunc,
    key_equal_func: GEqualFunc,
    key_destroy_func: GDestroyNotify,
    value_destroy_func: GDestroyNotify,
) -> *mut GHashTable {
    let mut table = Box::new(GHashTable {
        size: 0,
        mod_: 0,
        mask: 0,
        nnodes: 0,
        noccupied: 0,
        flags: FLAG_HAVE_BIG_KEYS | FLAG_HAVE_BIG_VALUES,
        keys: core::ptr::null_mut(),
        hashes: core::ptr::null_mut(),
        values: core::ptr::null_mut(),
        hash_func,
        key_equal_func,
        ref_count: 1,
        version: 0,
        key_destroy_func,
        value_destroy_func,
        keys_storage: Vec::new(),
        hashes_storage: Vec::new(),
        values_storage: Vec::new(),
    });
    resize_storage(core::ptr::from_mut(table.as_mut()), MIN_SIZE);
    let raw = Box::into_raw(table);
    register_owned_table(raw);
    raw
}

unsafe fn table_hash(table: *mut GHashTable, key: gconstpointer) -> guint {
    if let Some(hash_func) = (*table).hash_func {
        normalize_hash(hash_func(key))
    } else {
        normalize_hash(key as usize as guint)
    }
}

unsafe fn table_key_equal(table: *mut GHashTable, left: gconstpointer, right: gconstpointer) -> bool {
    if left == right {
        return true;
    }
    if let Some(equal) = (*table).key_equal_func {
        equal(left, right) != FALSE
    } else {
        false
    }
}

unsafe fn hash_to_index(table: *mut GHashTable, stored_hash: guint) -> usize {
    stored_hash.wrapping_mul(11).wrapping_rem((*table).mod_ as guint) as usize
}

unsafe fn live_indices(table: *mut GHashTable) -> Vec<usize> {
    ((*table).hashes_storage)
        .iter()
        .enumerate()
        .filter_map(|(index, hash)| (*hash >= 2).then_some(index))
        .collect()
}

unsafe fn find_slot(table: *mut GHashTable, key: gconstpointer, stored_hash: guint) -> (Option<usize>, usize) {
    let mut tombstone = None;
    let mut index = hash_to_index(table, stored_hash);
    let mut step = 0usize;
    loop {
        match (&(*table).hashes_storage)[index] {
            HASH_UNUSED => return (None, tombstone.unwrap_or(index)),
            HASH_TOMBSTONE => {
                if tombstone.is_none() {
                    tombstone = Some(index);
                }
            }
            hash if hash == stored_hash => {
                if table_key_equal(table, (&(*table).keys_storage)[index].cast_const(), key) {
                    return (Some(index), index);
                }
            }
            _ => {}
        }
        step += 1;
        index = (index + step) & (*table).mask as usize;
    }
}

unsafe fn insert_at_index(table: *mut GHashTable, index: usize, key: gpointer, value: gpointer, stored_hash: guint) {
    if (&(*table).hashes_storage)[index] == HASH_UNUSED {
        (*table).noccupied += 1;
    }
    (&mut (*table).hashes_storage)[index] = stored_hash;
    (&mut (*table).keys_storage)[index] = key;
    (&mut (*table).values_storage)[index] = value;
    (*table).nnodes += 1;
}

unsafe fn rehash_to(table: *mut GHashTable, new_capacity: usize) {
    let entries: Vec<_> = live_indices(table)
        .into_iter()
        .map(|index| {
            (
                (&(*table).keys_storage)[index],
                (&(*table).values_storage)[index],
                (&(*table).hashes_storage)[index],
            )
        })
        .collect();

    (*table).keys_storage.clear();
    (*table).hashes_storage.clear();
    (*table).values_storage.clear();
    resize_storage(table, new_capacity.max(MIN_SIZE).next_power_of_two());
    (*table).nnodes = 0;
    (*table).noccupied = 0;

    for (key, value, stored_hash) in entries {
        let (_, index) = find_slot(table, key.cast_const(), stored_hash);
        insert_at_index(table, index, key, value, stored_hash);
    }
}

unsafe fn ensure_capacity_for_insert(table: *mut GHashTable) {
    let size = (*table).size.max(1);
    if ((*table).noccupied as usize + 1) * 4 >= size * 3 {
        let should_grow = (*table).nnodes as usize * 8 >= size * 3;
        let target = if should_grow { size * 2 } else { size };
        rehash_to(table, target);
    }
}

struct InsertChange {
    inserted: bool,
    new_key_to_destroy: Option<gpointer>,
    old_key_to_destroy: Option<gpointer>,
    old_value_to_destroy: Option<gpointer>,
}

unsafe fn insert_raw(
    table: *mut GHashTable,
    key: gpointer,
    value: gpointer,
    replace_key: bool,
) -> InsertChange {
    ensure_capacity_for_insert(table);
    let stored_hash = table_hash(table, key.cast_const());
    let (existing, index) = find_slot(table, key.cast_const(), stored_hash);
    if let Some(index) = existing {
        let existing_key = (&(*table).keys_storage)[index];
        let existing_value = (&(*table).values_storage)[index];
        (&mut (*table).values_storage)[index] = value;
        if replace_key {
            (&mut (*table).keys_storage)[index] = key;
            (&mut (*table).hashes_storage)[index] = stored_hash;
        }
        return InsertChange {
            inserted: false,
            new_key_to_destroy: (!replace_key && existing_key != key).then_some(key),
            old_key_to_destroy: (replace_key && existing_key != key).then_some(existing_key),
            old_value_to_destroy: Some(existing_value),
        };
    }

    insert_at_index(table, index, key, value, stored_hash);
    (*table).version += 1;
    InsertChange {
        inserted: true,
        new_key_to_destroy: None,
        old_key_to_destroy: None,
        old_value_to_destroy: None,
    }
}

unsafe fn remove_index_raw(table: *mut GHashTable, index: usize) -> (gpointer, gpointer) {
    let key = (&(*table).keys_storage)[index];
    let value = (&(*table).values_storage)[index];
    (&mut (*table).keys_storage)[index] = core::ptr::null_mut();
    (&mut (*table).values_storage)[index] = core::ptr::null_mut();
    (&mut (*table).hashes_storage)[index] = HASH_TOMBSTONE;
    (*table).nnodes -= 1;
    (*table).version += 1;
    (key, value)
}

unsafe fn clear_table_raw(table: *mut GHashTable) -> Vec<(gpointer, gpointer)> {
    let entries = live_indices(table)
        .into_iter()
        .map(|index| ((&(*table).keys_storage)[index], (&(*table).values_storage)[index]))
        .collect::<Vec<_>>();
    for hash in &mut (*table).hashes_storage {
        *hash = HASH_UNUSED;
    }
    for key in &mut (*table).keys_storage {
        *key = core::ptr::null_mut();
    }
    for value in &mut (*table).values_storage {
        *value = core::ptr::null_mut();
    }
    (*table).nnodes = 0;
    (*table).noccupied = 0;
    (*table).version += 1;
    entries
}

unsafe fn destroy_key(table: *mut GHashTable, key: gpointer) {
    if let Some(func) = (*table).key_destroy_func {
        func(key);
    }
}

unsafe fn destroy_value(table: *mut GHashTable, value: gpointer) {
    if let Some(func) = (*table).value_destroy_func {
        func(value);
    }
}

unsafe fn build_list(table: *mut GHashTable, use_keys: bool) -> *mut GList {
    let mut list = core::ptr::null_mut();
    for index in live_indices(table) {
        let node = if use_keys {
            (&(*table).keys_storage)[index]
        } else {
            (&(*table).values_storage)[index]
        };
        list = g_list_prepend(list, node);
    }
    g_list_reverse(list)
}

unsafe fn build_ptr_array_from_items(items: &[gpointer]) -> *mut GPtrArray {
    let array = g_ptr_array_sized_new(items.len() as guint);
    for &item in items {
        g_ptr_array_add(array, item);
    }
    array
}

unsafe fn build_keys_array(table: *mut GHashTable, length: *mut guint) -> *mut gpointer {
    let items = live_indices(table)
        .into_iter()
        .map(|index| (&(*table).keys_storage)[index])
        .collect::<Vec<_>>();
    if !length.is_null() {
        *length = items.len() as guint;
    }
    let total = (items.len() + 1) * core::mem::size_of::<gpointer>();
    let out = malloc(total.max(core::mem::size_of::<gpointer>())).cast::<gpointer>();
    if out.is_null() {
        return core::ptr::null_mut();
    }
    for (index, item) in items.iter().copied().enumerate() {
        *out.add(index) = item;
    }
    *out.add(items.len()) = core::ptr::null_mut();
    out
}

#[unsafe(export_name = "g_str_equal")]
pub unsafe extern "C" fn str_equal(left: gconstpointer, right: gconstpointer) -> gboolean {
    if left == right {
        return TRUE;
    }
    if left.is_null() || right.is_null() {
        return FALSE;
    }
    (CStr::from_ptr(left.cast::<i8>()).to_bytes() == CStr::from_ptr(right.cast::<i8>()).to_bytes())
        as gboolean
}

#[unsafe(export_name = "g_str_hash")]
pub unsafe extern "C" fn str_hash(value: gconstpointer) -> guint {
    if value.is_null() {
        return 0;
    }
    let mut hash = 5381u32;
    for byte in CStr::from_ptr(value.cast::<i8>()).to_bytes() {
        hash = hash
            .wrapping_shl(5)
            .wrapping_add(hash)
            .wrapping_add((*byte as i8) as i32 as u32);
    }
    hash
}

#[unsafe(export_name = "g_hash_table_new")]
pub unsafe extern "C" fn hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable {
    make_owned_table(hash_func, key_equal_func, None, None)
}

#[unsafe(export_name = "g_hash_table_new_full")]
pub unsafe extern "C" fn hash_table_new_full(
    hash_func: GHashFunc,
    key_equal_func: GEqualFunc,
    key_destroy_func: GDestroyNotify,
    value_destroy_func: GDestroyNotify,
) -> *mut GHashTable {
    make_owned_table(hash_func, key_equal_func, key_destroy_func, value_destroy_func)
}

#[unsafe(export_name = "g_hash_table_new_similar")]
pub unsafe extern "C" fn hash_table_new_similar(other_hash_table: *mut GHashTable) -> *mut GHashTable {
    if !is_owned_table(other_hash_table) {
        return core::ptr::null_mut();
    }
    make_owned_table(
        (*other_hash_table).hash_func,
        (*other_hash_table).key_equal_func,
        (*other_hash_table).key_destroy_func,
        (*other_hash_table).value_destroy_func,
    )
}

#[unsafe(export_name = "g_hash_table_destroy")]
pub unsafe extern "C" fn hash_table_destroy(hash_table: *mut GHashTable) {
    if !is_owned_table(hash_table) {
        return;
    }
    hash_table_remove_all(hash_table);
    hash_table_unref(hash_table);
}

#[unsafe(export_name = "g_hash_table_insert")]
pub unsafe extern "C" fn hash_table_insert(
    hash_table: *mut GHashTable,
    key: gpointer,
    value: gpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    let change = insert_raw(hash_table, key, value, false);
    if let Some(duplicate_key) = change.new_key_to_destroy {
        destroy_key(hash_table, duplicate_key);
    }
    if let Some(old_value) = change.old_value_to_destroy {
        destroy_value(hash_table, old_value);
    }
    change.inserted as gboolean
}

#[unsafe(export_name = "g_hash_table_replace")]
pub unsafe extern "C" fn hash_table_replace(
    hash_table: *mut GHashTable,
    key: gpointer,
    value: gpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    let change = insert_raw(hash_table, key, value, true);
    if let Some(old_key) = change.old_key_to_destroy {
        destroy_key(hash_table, old_key);
    }
    if let Some(old_value) = change.old_value_to_destroy {
        destroy_value(hash_table, old_value);
    }
    change.inserted as gboolean
}

#[unsafe(export_name = "g_hash_table_add")]
pub unsafe extern "C" fn hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    hash_table_replace(hash_table, key, key)
}

#[unsafe(export_name = "g_hash_table_remove")]
pub unsafe extern "C" fn hash_table_remove(
    hash_table: *mut GHashTable,
    key: gconstpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    let stored_hash = table_hash(hash_table, key);
    let (existing, _) = find_slot(hash_table, key, stored_hash);
    let Some(index) = existing else {
        return FALSE;
    };
    let (removed_key, removed_value) = remove_index_raw(hash_table, index);
    destroy_key(hash_table, removed_key);
    destroy_value(hash_table, removed_value);
    TRUE
}

#[unsafe(export_name = "g_hash_table_remove_all")]
pub unsafe extern "C" fn hash_table_remove_all(hash_table: *mut GHashTable) {
    if !is_owned_table(hash_table) {
        return;
    }
    let entries = clear_table_raw(hash_table);
    for (key, value) in entries {
        destroy_key(hash_table, key);
        destroy_value(hash_table, value);
    }
}

#[unsafe(export_name = "g_hash_table_steal")]
pub unsafe extern "C" fn hash_table_steal(
    hash_table: *mut GHashTable,
    key: gconstpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    let stored_hash = table_hash(hash_table, key);
    let (existing, _) = find_slot(hash_table, key, stored_hash);
    if let Some(index) = existing {
        remove_index_raw(hash_table, index);
        TRUE
    } else {
        FALSE
    }
}

#[unsafe(export_name = "g_hash_table_steal_extended")]
pub unsafe extern "C" fn hash_table_steal_extended(
    hash_table: *mut GHashTable,
    lookup_key: gconstpointer,
    stolen_key: *mut gpointer,
    stolen_value: *mut gpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        if !stolen_key.is_null() {
            *stolen_key = core::ptr::null_mut();
        }
        if !stolen_value.is_null() {
            *stolen_value = core::ptr::null_mut();
        }
        return FALSE;
    }
    let stored_hash = table_hash(hash_table, lookup_key);
    let (existing, _) = find_slot(hash_table, lookup_key, stored_hash);
    let Some(index) = existing else {
        if !stolen_key.is_null() {
            *stolen_key = core::ptr::null_mut();
        }
        if !stolen_value.is_null() {
            *stolen_value = core::ptr::null_mut();
        }
        return FALSE;
    };
    let (key, value) = remove_index_raw(hash_table, index);
    if !stolen_key.is_null() {
        *stolen_key = key;
    }
    if !stolen_value.is_null() {
        *stolen_value = value;
    }
    TRUE
}

#[unsafe(export_name = "g_hash_table_steal_all")]
pub unsafe extern "C" fn hash_table_steal_all(hash_table: *mut GHashTable) {
    if !is_owned_table(hash_table) {
        return;
    }
    let _ = clear_table_raw(hash_table);
}

#[unsafe(export_name = "g_hash_table_steal_all_keys")]
pub unsafe extern "C" fn hash_table_steal_all_keys(hash_table: *mut GHashTable) -> *mut GPtrArray {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let entries = clear_table_raw(hash_table);
    let keys = entries.iter().map(|(key, _)| *key).collect::<Vec<_>>();
    for (_, value) in entries {
        destroy_value(hash_table, value);
    }
    build_ptr_array_from_items(&keys)
}

#[unsafe(export_name = "g_hash_table_steal_all_values")]
pub unsafe extern "C" fn hash_table_steal_all_values(hash_table: *mut GHashTable) -> *mut GPtrArray {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let entries = clear_table_raw(hash_table);
    let values = entries.iter().map(|(_, value)| *value).collect::<Vec<_>>();
    for (key, _) in entries {
        destroy_key(hash_table, key);
    }
    build_ptr_array_from_items(&values)
}

#[unsafe(export_name = "g_hash_table_lookup")]
pub unsafe extern "C" fn hash_table_lookup(
    hash_table: *mut GHashTable,
    key: gconstpointer,
) -> gpointer {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let stored_hash = table_hash(hash_table, key);
    match find_slot(hash_table, key, stored_hash).0 {
        Some(index) => (&(*hash_table).values_storage)[index],
        None => core::ptr::null_mut(),
    }
}

#[unsafe(export_name = "g_hash_table_contains")]
pub unsafe extern "C" fn hash_table_contains(
    hash_table: *mut GHashTable,
    key: gconstpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        return FALSE;
    }
    let stored_hash = table_hash(hash_table, key);
    find_slot(hash_table, key, stored_hash).0.is_some() as gboolean
}

#[unsafe(export_name = "g_hash_table_lookup_extended")]
pub unsafe extern "C" fn hash_table_lookup_extended(
    hash_table: *mut GHashTable,
    lookup_key: gconstpointer,
    orig_key: *mut gpointer,
    value: *mut gpointer,
) -> gboolean {
    if !is_owned_table(hash_table) {
        if !orig_key.is_null() {
            *orig_key = core::ptr::null_mut();
        }
        if !value.is_null() {
            *value = core::ptr::null_mut();
        }
        return FALSE;
    }
    let stored_hash = table_hash(hash_table, lookup_key);
    let Some(index) = find_slot(hash_table, lookup_key, stored_hash).0 else {
        if !orig_key.is_null() {
            *orig_key = core::ptr::null_mut();
        }
        if !value.is_null() {
            *value = core::ptr::null_mut();
        }
        return FALSE;
    };
    if !orig_key.is_null() {
        *orig_key = (&(*hash_table).keys_storage)[index];
    }
    if !value.is_null() {
        *value = (&(*hash_table).values_storage)[index];
    }
    TRUE
}

#[unsafe(export_name = "g_hash_table_foreach")]
pub unsafe extern "C" fn hash_table_foreach(
    hash_table: *mut GHashTable,
    func: GHFunc,
    user_data: gpointer,
) {
    if !is_owned_table(hash_table) {
        return;
    }
    let Some(func) = func else {
        return;
    };
    for index in live_indices(hash_table) {
        func(
            (&(*hash_table).keys_storage)[index],
            (&(*hash_table).values_storage)[index],
            user_data,
        );
    }
}

#[unsafe(export_name = "g_hash_table_find")]
pub unsafe extern "C" fn hash_table_find(
    hash_table: *mut GHashTable,
    predicate: GHRFunc,
    user_data: gpointer,
) -> gpointer {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let Some(predicate) = predicate else {
        return core::ptr::null_mut();
    };
    for index in live_indices(hash_table) {
        let key = (&(*hash_table).keys_storage)[index];
        let value = (&(*hash_table).values_storage)[index];
        if predicate(key, value, user_data) != FALSE {
            return value;
        }
    }
    core::ptr::null_mut()
}

#[unsafe(export_name = "g_hash_table_foreach_remove")]
pub unsafe extern "C" fn hash_table_foreach_remove(
    hash_table: *mut GHashTable,
    func: GHRFunc,
    user_data: gpointer,
) -> guint {
    if !is_owned_table(hash_table) {
        return 0;
    }
    let Some(func) = func else {
        return 0;
    };
    let mut removed = 0;
    for index in live_indices(hash_table) {
        if (&(*hash_table).hashes_storage)[index] < 2 {
            continue;
        }
        let key = (&(*hash_table).keys_storage)[index];
        let value = (&(*hash_table).values_storage)[index];
        if func(key, value, user_data) != FALSE {
            let (removed_key, removed_value) = remove_index_raw(hash_table, index);
            destroy_key(hash_table, removed_key);
            destroy_value(hash_table, removed_value);
            removed += 1;
        }
    }
    removed
}

#[unsafe(export_name = "g_hash_table_foreach_steal")]
pub unsafe extern "C" fn hash_table_foreach_steal(
    hash_table: *mut GHashTable,
    func: GHRFunc,
    user_data: gpointer,
) -> guint {
    if !is_owned_table(hash_table) {
        return 0;
    }
    let Some(func) = func else {
        return 0;
    };
    let mut removed = 0;
    for index in live_indices(hash_table) {
        if (&(*hash_table).hashes_storage)[index] < 2 {
            continue;
        }
        if func(
            (&(*hash_table).keys_storage)[index],
            (&(*hash_table).values_storage)[index],
            user_data,
        ) != FALSE
        {
            remove_index_raw(hash_table, index);
            removed += 1;
        }
    }
    removed
}

#[unsafe(export_name = "g_hash_table_size")]
pub unsafe extern "C" fn hash_table_size(hash_table: *mut GHashTable) -> guint {
    if !is_owned_table(hash_table) {
        return 0;
    }
    (*hash_table).nnodes
}

#[unsafe(export_name = "g_hash_table_get_keys")]
pub unsafe extern "C" fn hash_table_get_keys(hash_table: *mut GHashTable) -> *mut GList {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    build_list(hash_table, true)
}

#[unsafe(export_name = "g_hash_table_get_values")]
pub unsafe extern "C" fn hash_table_get_values(hash_table: *mut GHashTable) -> *mut GList {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    build_list(hash_table, false)
}

#[unsafe(export_name = "g_hash_table_get_keys_as_array")]
pub unsafe extern "C" fn hash_table_get_keys_as_array(
    hash_table: *mut GHashTable,
    length: *mut guint,
) -> *mut gpointer {
    if !is_owned_table(hash_table) {
        if !length.is_null() {
            *length = 0;
        }
        return core::ptr::null_mut();
    }
    build_keys_array(hash_table, length)
}

#[unsafe(export_name = "g_hash_table_get_keys_as_ptr_array")]
pub unsafe extern "C" fn hash_table_get_keys_as_ptr_array(
    hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let keys = live_indices(hash_table)
        .into_iter()
        .map(|index| (&(*hash_table).keys_storage)[index])
        .collect::<Vec<_>>();
    build_ptr_array_from_items(&keys)
}

#[unsafe(export_name = "g_hash_table_get_values_as_ptr_array")]
pub unsafe extern "C" fn hash_table_get_values_as_ptr_array(
    hash_table: *mut GHashTable,
) -> *mut GPtrArray {
    if !is_owned_table(hash_table) {
        return core::ptr::null_mut();
    }
    let values = live_indices(hash_table)
        .into_iter()
        .map(|index| (&(*hash_table).values_storage)[index])
        .collect::<Vec<_>>();
    build_ptr_array_from_items(&values)
}

#[unsafe(export_name = "g_hash_table_iter_init")]
pub unsafe extern "C" fn hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable) {
    if !is_owned_table(hash_table) {
        (*iter).hash_table = core::ptr::null_mut();
        (*iter).dummy1 = core::ptr::null_mut();
        (*iter).dummy2 = core::ptr::null_mut();
        (*iter).position = 0;
        (*iter).dummy3 = FALSE;
        (*iter).version = 0;
        return;
    }
    (*iter).hash_table = hash_table;
    (*iter).dummy1 = core::ptr::null_mut();
    (*iter).dummy2 = core::ptr::null_mut();
    (*iter).position = -1;
    (*iter).dummy3 = FALSE;
    (*iter).version = (*hash_table).version as isize;
}

#[unsafe(export_name = "g_hash_table_iter_next")]
pub unsafe extern "C" fn hash_table_iter_next(
    iter: *mut GHashTableIter,
    key: *mut gpointer,
    value: *mut gpointer,
) -> gboolean {
    if iter.is_null() || !is_owned_table((*iter).hash_table) {
        return FALSE;
    }
    let table = (*iter).hash_table;
    let mut index = (*iter).position + 1;
    while index < (*table).size as gint {
        let slot = index as usize;
        if (&(*table).hashes_storage)[slot] >= 2 {
            (*iter).dummy1 = iter_index_to_ptr(slot);
            (*iter).position = index;
            (*iter).dummy3 = TRUE;
            if !key.is_null() {
                *key = (&(*table).keys_storage)[slot];
            }
            if !value.is_null() {
                *value = (&(*table).values_storage)[slot];
            }
            return TRUE;
        }
        index += 1;
    }
    (*iter).position = index;
    (*iter).dummy3 = FALSE;
    FALSE
}

#[unsafe(export_name = "g_hash_table_iter_get_hash_table")]
pub unsafe extern "C" fn hash_table_iter_get_hash_table(iter: *mut GHashTableIter) -> *mut GHashTable {
    if iter.is_null() || !is_owned_table((*iter).hash_table) {
        return core::ptr::null_mut();
    }
    (*iter).hash_table
}

#[unsafe(export_name = "g_hash_table_iter_remove")]
pub unsafe extern "C" fn hash_table_iter_remove(iter: *mut GHashTableIter) {
    if iter.is_null() || !is_owned_table((*iter).hash_table) {
        return;
    }
    let Some(index) = ptr_to_iter_index((*iter).dummy1) else {
        return;
    };
    if (&(*(*iter).hash_table).hashes_storage)[index] >= 2 {
        let (key, value) = remove_index_raw((*iter).hash_table, index);
        destroy_key((*iter).hash_table, key);
        destroy_value((*iter).hash_table, value);
    }
    (*iter).dummy3 = FALSE;
}

#[unsafe(export_name = "g_hash_table_iter_replace")]
pub unsafe extern "C" fn hash_table_iter_replace(iter: *mut GHashTableIter, value: gpointer) {
    if iter.is_null() || !is_owned_table((*iter).hash_table) {
        return;
    }
    let Some(index) = ptr_to_iter_index((*iter).dummy1) else {
        return;
    };
    if (&(*(*iter).hash_table).hashes_storage)[index] >= 2 {
        let old_value = (&(*(*iter).hash_table).values_storage)[index];
        (&mut (*(*iter).hash_table).values_storage)[index] = value;
        destroy_value((*iter).hash_table, old_value);
    }
}

#[unsafe(export_name = "g_hash_table_iter_steal")]
pub unsafe extern "C" fn hash_table_iter_steal(iter: *mut GHashTableIter) {
    if iter.is_null() || !is_owned_table((*iter).hash_table) {
        return;
    }
    let Some(index) = ptr_to_iter_index((*iter).dummy1) else {
        return;
    };
    if (&(*(*iter).hash_table).hashes_storage)[index] >= 2 {
        remove_index_raw((*iter).hash_table, index);
    }
    (*iter).dummy3 = FALSE;
}

#[unsafe(export_name = "g_hash_table_ref")]
pub unsafe extern "C" fn hash_table_ref(hash_table: *mut GHashTable) -> *mut GHashTable {
    if !is_owned_table(hash_table) {
        return hash_table;
    }
    (*hash_table).ref_count += 1;
    hash_table
}

#[unsafe(export_name = "g_hash_table_unref")]
pub unsafe extern "C" fn hash_table_unref(hash_table: *mut GHashTable) {
    if !is_owned_table(hash_table) {
        return;
    }
    (*hash_table).ref_count -= 1;
    if (*hash_table).ref_count > 0 {
        return;
    }
    let entries = clear_table_raw(hash_table);
    for (key, value) in entries {
        destroy_key(hash_table, key);
        destroy_value(hash_table, value);
    }
    unregister_owned_table(hash_table);
    drop(Box::from_raw(hash_table));
}
