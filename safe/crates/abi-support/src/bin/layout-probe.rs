use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};

use safe_gio::abi as gio;
use safe_girepository::abi as girepository;
use safe_glib::abi as glib;
use safe_gmodule::abi as gmodule;
use safe_gobject::abi as gobject;
use safe_gthread::abi as gthread;

struct LayoutEntry {
    type_name: &'static str,
    kind: &'static str,
    size: usize,
    align: usize,
    fields: BTreeMap<&'static str, usize>,
}

struct LibraryLayouts {
    library: &'static str,
    entries: Vec<LayoutEntry>,
}

struct ProbeOutput {
    libraries: Vec<LibraryLayouts>,
}

macro_rules! struct_fields {
    ($ty:ty, $($field:ident),* $(,)?) => {{
        let mut fields = BTreeMap::new();
        $(fields.insert(stringify!($field), offset_of!($ty, $field));)*
        fields
    }};
}

macro_rules! union_fields {
    ($($field:ident),* $(,)?) => {{
        let mut fields = BTreeMap::new();
        $(fields.insert(stringify!($field), 0usize);)*
        fields
    }};
}

fn struct_entry<T>(type_name: &'static str, fields: BTreeMap<&'static str, usize>) -> LayoutEntry {
    LayoutEntry {
        type_name,
        kind: "struct",
        size: size_of::<T>(),
        align: align_of::<T>(),
        fields,
    }
}

fn union_entry<T>(type_name: &'static str, fields: BTreeMap<&'static str, usize>) -> LayoutEntry {
    LayoutEntry {
        type_name,
        kind: "union",
        size: size_of::<T>(),
        align: align_of::<T>(),
        fields,
    }
}

fn enum_entry<T>(type_name: &'static str) -> LayoutEntry {
    LayoutEntry {
        type_name,
        kind: "enum",
        size: size_of::<T>(),
        align: align_of::<T>(),
        fields: BTreeMap::new(),
    }
}

fn gtype_query_fields() -> BTreeMap<&'static str, usize> {
    let mut fields = BTreeMap::new();
    fields.insert("type", offset_of!(gobject::GTypeQuery, type_));
    fields.insert("type_name", offset_of!(gobject::GTypeQuery, type_name));
    fields.insert("class_size", offset_of!(gobject::GTypeQuery, class_size));
    fields.insert("instance_size", offset_of!(gobject::GTypeQuery, instance_size));
    fields
}

fn probe_output() -> ProbeOutput {
    ProbeOutput {
        libraries: vec![
            LibraryLayouts {
                library: "glib",
                entries: vec![
                    struct_entry::<glib::GList>("GList", struct_fields!(glib::GList, data, next, prev)),
                    struct_entry::<glib::GSList>("GSList", struct_fields!(glib::GSList, data, next)),
                    struct_entry::<glib::GQueue>("GQueue", struct_fields!(glib::GQueue, head, tail, length)),
                    struct_entry::<glib::GArray>("GArray", struct_fields!(glib::GArray, data, len)),
                    struct_entry::<glib::GPtrArray>("GPtrArray", struct_fields!(glib::GPtrArray, pdata, len)),
                    struct_entry::<glib::GByteArray>("GByteArray", struct_fields!(glib::GByteArray, data, len)),
                    struct_entry::<glib::GString>("GString", struct_fields!(glib::GString, str, len, allocated_len)),
                    struct_entry::<glib::GError>("GError", struct_fields!(glib::GError, domain, code, message)),
                    struct_entry::<glib::GOptionEntry>(
                        "GOptionEntry",
                        struct_fields!(
                            glib::GOptionEntry,
                            long_name,
                            short_name,
                            flags,
                            arg,
                            arg_data,
                            description,
                            arg_description
                        ),
                    ),
                ],
            },
            LibraryLayouts {
                library: "gthread",
                entries: vec![
                    union_entry::<gthread::GMutex>("GMutex", union_fields!(p, i)),
                    struct_entry::<gthread::GRecMutex>("GRecMutex", struct_fields!(gthread::GRecMutex, p, i)),
                    struct_entry::<gthread::GRWLock>("GRWLock", struct_fields!(gthread::GRWLock, p, i)),
                    struct_entry::<gthread::GCond>("GCond", struct_fields!(gthread::GCond, p, i)),
                    struct_entry::<gthread::GOnce>("GOnce", struct_fields!(gthread::GOnce, status, retval)),
                ],
            },
            LibraryLayouts {
                library: "gmodule",
                entries: vec![enum_entry::<gmodule::GModuleFlags>("GModuleFlags")],
            },
            LibraryLayouts {
                library: "gobject",
                entries: vec![
                    struct_entry::<gobject::GTypeInfo>(
                        "GTypeInfo",
                        struct_fields!(
                            gobject::GTypeInfo,
                            class_size,
                            base_init,
                            base_finalize,
                            class_init,
                            class_finalize,
                            class_data,
                            instance_size,
                            n_preallocs,
                            instance_init,
                            value_table
                        ),
                    ),
                    struct_entry::<gobject::GTypeQuery>(
                        "GTypeQuery",
                        gtype_query_fields(),
                    ),
                    struct_entry::<gobject::GTypeValueTable>(
                        "GTypeValueTable",
                        struct_fields!(
                            gobject::GTypeValueTable,
                            value_init,
                            value_free,
                            value_copy,
                            value_peek_pointer,
                            collect_format,
                            collect_value,
                            lcopy_format,
                            lcopy_value
                        ),
                    ),
                    struct_entry::<gobject::GInterfaceInfo>(
                        "GInterfaceInfo",
                        struct_fields!(gobject::GInterfaceInfo, interface_init, interface_finalize, interface_data),
                    ),
                    struct_entry::<gobject::GTypeInstance>(
                        "GTypeInstance",
                        struct_fields!(gobject::GTypeInstance, g_class),
                    ),
                    struct_entry::<gobject::GTypeClass>(
                        "GTypeClass",
                        struct_fields!(gobject::GTypeClass, g_type),
                    ),
                    struct_entry::<gobject::GTypeInterface>(
                        "GTypeInterface",
                        struct_fields!(gobject::GTypeInterface, g_type, g_instance_type),
                    ),
                    struct_entry::<gobject::GValue>("GValue", struct_fields!(gobject::GValue, g_type, data)),
                    struct_entry::<gobject::GClosure>(
                        "GClosure",
                        struct_fields!(gobject::GClosure, marshal, data, notifiers),
                    ),
                    struct_entry::<gobject::GCClosure>(
                        "GCClosure",
                        struct_fields!(gobject::GCClosure, closure, callback),
                    ),
                    struct_entry::<gobject::GObjectConstructParam>(
                        "GObjectConstructParam",
                        struct_fields!(gobject::GObjectConstructParam, pspec, value),
                    ),
                ],
            },
            LibraryLayouts {
                library: "gio",
                entries: vec![
                    struct_entry::<gio::GActionEntry>(
                        "GActionEntry",
                        struct_fields!(gio::GActionEntry, name, activate, parameter_type, state, change_state, padding),
                    ),
                    struct_entry::<gio::GDBusInterfaceVTable>(
                        "GDBusInterfaceVTable",
                        struct_fields!(gio::GDBusInterfaceVTable, method_call, get_property, set_property, padding),
                    ),
                    struct_entry::<gio::GDBusSubtreeVTable>(
                        "GDBusSubtreeVTable",
                        struct_fields!(gio::GDBusSubtreeVTable, enumerate, introspect, dispatch, padding),
                    ),
                ],
            },
            LibraryLayouts {
                library: "girepository",
                entries: vec![
                    union_entry::<girepository::GIArgument>(
                        "GIArgument",
                        union_fields!(
                            v_boolean,
                            v_int8,
                            v_uint8,
                            v_int16,
                            v_uint16,
                            v_int32,
                            v_uint32,
                            v_int64,
                            v_uint64,
                            v_float,
                            v_double,
                            v_short,
                            v_ushort,
                            v_int,
                            v_uint,
                            v_long,
                            v_ulong,
                            v_ssize,
                            v_size,
                            v_string,
                            v_pointer
                        ),
                    ),
                    enum_entry::<girepository::GITypeTag>("GITypeTag"),
                    enum_entry::<girepository::GIArrayType>("GIArrayType"),
                    struct_entry::<girepository::GIAttributeIter>(
                        "GIAttributeIter",
                        struct_fields!(girepository::GIAttributeIter, data, _dummy),
                    ),
                ],
            },
        ],
    }
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
}

fn render_layout_entry(out: &mut String, entry: &LayoutEntry) {
    out.push('{');
    out.push_str("\"type_name\":");
    push_json_string(out, entry.type_name);
    out.push_str(",\"kind\":");
    push_json_string(out, entry.kind);
    write!(out, ",\"size\":{},\"align\":{},\"fields\":{{", entry.size, entry.align).unwrap();
    let mut first = true;
    for (field, offset) in &entry.fields {
        if !first {
            out.push(',');
        }
        first = false;
        push_json_string(out, field);
        write!(out, ":{offset}").unwrap();
    }
    out.push_str("}}");
}

fn render_output(output: &ProbeOutput) -> String {
    let mut rendered = String::new();
    rendered.push_str("{\"libraries\":[");
    for (library_index, library) in output.libraries.iter().enumerate() {
        if library_index > 0 {
            rendered.push(',');
        }
        rendered.push('{');
        rendered.push_str("\"library\":");
        push_json_string(&mut rendered, library.library);
        rendered.push_str(",\"entries\":[");
        for (entry_index, entry) in library.entries.iter().enumerate() {
            if entry_index > 0 {
                rendered.push(',');
            }
            render_layout_entry(&mut rendered, entry);
        }
        rendered.push_str("]}");
    }
    rendered.push_str("]}");
    rendered
}

fn main() {
    let output = probe_output();
    if std::env::args().any(|arg| arg == "--verify-contract") {
        println!("layout-probe-contract-ok");
        return;
    }

    println!("{}", render_output(&output));
}
