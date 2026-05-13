use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub const GIR_SUBDIR: &str = "gir-1.0";
pub const TYPELIB_SUBDIR: &str = "girepository-1.0";

const TYPELIB_MAGIC: &[u8; 16] = b"GOBJ\nMETADATA\r\n\x1a";
const SAFE_TYPELIB_MAGIC: &[u8] = b"SAFE-GIREPOSITORY-TYPELIB\n";

pub const GI_DIRECTION_IN: i32 = 0;
pub const GI_DIRECTION_OUT: i32 = 1;
pub const GI_TRANSFER_NOTHING: i32 = 0;
pub const GI_TRANSFER_CONTAINER: i32 = 1;
pub const GI_TRANSFER_EVERYTHING: i32 = 2;
pub const GI_SCOPE_INVALID: i32 = 0;
pub const GI_TYPE_TAG_VOID: i32 = 0;
pub const GI_TYPE_TAG_BOOLEAN: i32 = 1;
pub const GI_TYPE_TAG_INT8: i32 = 2;
pub const GI_TYPE_TAG_UINT8: i32 = 3;
pub const GI_TYPE_TAG_INT16: i32 = 4;
pub const GI_TYPE_TAG_UINT16: i32 = 5;
pub const GI_TYPE_TAG_INT32: i32 = 6;
pub const GI_TYPE_TAG_UINT32: i32 = 7;
pub const GI_TYPE_TAG_INT64: i32 = 8;
pub const GI_TYPE_TAG_UINT64: i32 = 9;
pub const GI_TYPE_TAG_FLOAT: i32 = 10;
pub const GI_TYPE_TAG_DOUBLE: i32 = 11;
pub const GI_TYPE_TAG_GTYPE: i32 = 12;
pub const GI_TYPE_TAG_UTF8: i32 = 13;
pub const GI_TYPE_TAG_FILENAME: i32 = 14;
pub const GI_TYPE_TAG_ARRAY: i32 = 15;
pub const GI_TYPE_TAG_INTERFACE: i32 = 16;
pub const GI_TYPE_TAG_GLIST: i32 = 17;
pub const GI_TYPE_TAG_GSLIST: i32 = 18;
pub const GI_TYPE_TAG_GHASH: i32 = 19;
pub const GI_TYPE_TAG_ERROR: i32 = 20;
pub const GI_TYPE_TAG_UNICHAR: i32 = 21;
pub const GI_ARRAY_TYPE_C: i32 = 0;

#[derive(Clone, Debug)]
pub struct Dependency {
    pub namespace: String,
    pub version: String,
}

impl Dependency {
    pub fn typelib_name(&self) -> String {
        format!("{}-{}", self.namespace, self.version)
    }
}

#[derive(Clone, Debug)]
pub struct RepositoryDocument {
    pub namespace: String,
    pub version: String,
    pub c_prefix: String,
    pub shared_libraries: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub items: Vec<ItemModel>,
    pub raw_gir: Option<String>,
    pub source_path: Option<PathBuf>,
    pub typelib: Option<TypelibMetadata>,
}

impl RepositoryDocument {
    pub fn find_item(&self, name: &str) -> Option<usize> {
        self.items.iter().position(|item| item.name == name)
    }

    pub fn item(&self, index: usize) -> Option<&ItemModel> {
        self.items.get(index)
    }

    pub fn dependency_names(&self) -> Vec<String> {
        self.dependencies
            .iter()
            .map(Dependency::typelib_name)
            .collect()
    }

    pub fn to_gir(&self) -> String {
        if let Some(raw) = &self.raw_gir {
            return raw.clone();
        }

        let mut out = String::from("<?xml version=\"1.0\"?>\n<repository version=\"1.2\">\n");
        for dependency in &self.dependencies {
            out.push_str(&format!(
                "  <include name=\"{}\" version=\"{}\"/>\n",
                escape_xml(&dependency.namespace),
                escape_xml(&dependency.version)
            ));
        }
        out.push_str(&format!(
            "  <namespace name=\"{}\" version=\"{}\" shared-library=\"{}\" c:identifier-prefixes=\"{}\">\n",
            escape_xml(&self.namespace),
            escape_xml(&self.version),
            escape_xml(&self.shared_libraries.join(",")),
            escape_xml(&self.c_prefix)
        ));
        for item in &self.items {
            let tag = match item.kind {
                ItemKind::Function => "function",
                ItemKind::Callback => "callback",
                ItemKind::Constant => "constant",
                ItemKind::Enum => "enumeration",
                ItemKind::Flags => "bitfield",
                ItemKind::Object => "class",
                ItemKind::Interface => "interface",
                ItemKind::Struct => "record",
                ItemKind::Union => "union",
                ItemKind::Unresolved => "alias",
            };
            out.push_str(&format!(
                "    <{} name=\"{}\" c:type=\"{}\"",
                tag,
                escape_xml(&item.name),
                escape_xml(&item.c_type)
            ));
            if !item.type_name.is_empty() {
                out.push_str(&format!(
                    " glib:type-name=\"{}\"",
                    escape_xml(&item.type_name)
                ));
            }
            if !item.type_init.is_empty() {
                out.push_str(&format!(
                    " glib:get-type=\"{}\"",
                    escape_xml(&item.type_init)
                ));
            }
            if !item.error_domain.is_empty() {
                out.push_str(&format!(
                    " glib:error-domain=\"{}\"",
                    escape_xml(&item.error_domain)
                ));
            }
            out.push_str("/>\n");
        }
        out.push_str("  </namespace>\n</repository>\n");
        out
    }
}

#[derive(Clone, Debug)]
pub struct TypelibMetadata {
    pub namespace: String,
    pub version: String,
    pub c_prefix: String,
    pub shared_libraries: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub entries: Vec<TypelibEntry>,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct TypelibEntry {
    pub name: String,
    pub kind: ItemKind,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ItemKind {
    Function,
    Callback,
    Constant,
    Enum,
    Flags,
    Object,
    Interface,
    Struct,
    Union,
    Unresolved,
}

#[derive(Clone, Debug)]
pub struct ItemModel {
    pub namespace: String,
    pub name: String,
    pub kind: ItemKind,
    pub c_identifier: String,
    pub c_type: String,
    pub type_name: String,
    pub type_init: String,
    pub type_struct: String,
    pub ref_func: String,
    pub is_boxed: bool,
    pub is_gtype_struct: bool,
    pub error_domain: String,
    pub size: Option<usize>,
    pub alignment: Option<usize>,
    pub callable: Option<CallableModel>,
    pub methods: Vec<CallableModel>,
    pub vfuncs: Vec<CallableModel>,
    pub signals: Vec<CallableModel>,
    pub fields: Vec<FieldModel>,
    pub properties: Vec<PropertyModel>,
    pub values: Vec<ValueModel>,
    pub implements: Vec<TypeRef>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CallKind {
    Function,
    VFunc,
    Signal,
    Callback,
}

#[derive(Clone, Debug)]
pub struct CallableModel {
    pub namespace: String,
    pub name: String,
    pub symbol: String,
    pub kind: CallKind,
    pub throws: bool,
    pub is_method: bool,
    pub args: Vec<ArgModel>,
    pub return_type: TypeModel,
    pub instance_transfer: i32,
    pub may_return_null: bool,
    pub invoker: String,
    pub signal_flags: i32,
}

#[derive(Clone, Debug)]
pub struct ArgModel {
    pub name: String,
    pub direction: i32,
    pub transfer: i32,
    pub nullable: bool,
    pub optional: bool,
    pub caller_allocates: bool,
    pub scope: i32,
    pub type_info: TypeModel,
}

#[derive(Clone, Debug)]
pub struct TypeModel {
    pub tag: i32,
    pub is_pointer: bool,
    pub zero_terminated: bool,
    pub array_type: i32,
    pub array_length: Option<usize>,
    pub interface: Option<InterfaceRef>,
}

impl TypeModel {
    pub fn void() -> Self {
        Self {
            tag: GI_TYPE_TAG_VOID,
            is_pointer: false,
            zero_terminated: false,
            array_type: GI_ARRAY_TYPE_C,
            array_length: None,
            interface: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum InterfaceRef {
    Named(TypeRef),
    Callable(Box<CallableModel>),
}

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub namespace: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct FieldModel {
    pub namespace: String,
    pub name: String,
    pub type_info: TypeModel,
}

#[derive(Clone, Debug)]
pub struct PropertyModel {
    pub namespace: String,
    pub name: String,
    pub type_info: TypeModel,
}

#[derive(Clone, Debug)]
pub struct ValueModel {
    pub namespace: String,
    pub name: String,
    pub c_identifier: String,
}

#[derive(Clone, Debug)]
struct XmlNode {
    name: String,
    attrs: HashMap<String, String>,
    children: Vec<XmlNode>,
}

impl XmlNode {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attrs: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn attr(&self, name: &str) -> &str {
        self.attrs.get(name).map(String::as_str).unwrap_or("")
    }

    fn attr_bool(&self, name: &str) -> bool {
        matches!(self.attr(name), "1" | "true" | "yes")
    }

    fn child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|child| child.name == name)
    }
}

pub fn parse_gir_file(path: &Path) -> Result<Arc<RepositoryDocument>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut document = parse_gir_text(&text)?;
    document.source_path = Some(path.to_path_buf());
    Ok(Arc::new(document))
}

pub fn parse_gir_text(text: &str) -> Result<RepositoryDocument, String> {
    let root = parse_xml(text)?;
    let repository = root
        .children
        .iter()
        .find(|child| child.name == "repository")
        .ok_or_else(|| "GIR has no repository node".to_owned())?;
    let namespace_node = repository
        .children
        .iter()
        .find(|child| child.name == "namespace")
        .ok_or_else(|| "GIR has no namespace node".to_owned())?;

    let namespace = namespace_node.attr("name").to_owned();
    if namespace.is_empty() {
        return Err("GIR namespace has no name".to_owned());
    }
    let version = namespace_node.attr("version").to_owned();
    let dependencies = repository
        .children
        .iter()
        .filter(|child| child.name == "include")
        .filter_map(|child| {
            let namespace = child.attr("name");
            let version = child.attr("version");
            (!namespace.is_empty() && !version.is_empty()).then(|| Dependency {
                namespace: namespace.to_owned(),
                version: version.to_owned(),
            })
        })
        .collect();
    let shared_libraries = split_list(namespace_node.attr("shared-library"));
    let c_prefix = first_nonempty(&[
        namespace_node.attr("c:identifier-prefixes"),
        namespace_node.attr("c:symbol-prefixes"),
        namespace_node.attr("c:prefix"),
    ])
    .to_owned();

    let mut items = Vec::new();
    for child in &namespace_node.children {
        if let Some(item) = parse_item(child, &namespace) {
            items.push(item);
        }
    }

    Ok(RepositoryDocument {
        namespace,
        version,
        c_prefix,
        shared_libraries,
        dependencies,
        items,
        raw_gir: Some(text.to_owned()),
        source_path: None,
        typelib: None,
    })
}

pub fn compile_gir_to_typelib(input: &Path, output: &Path) -> Result<(), String> {
    let text =
        std::fs::read_to_string(input).map_err(|error| format!("{}: {error}", input.display()))?;
    let document = parse_gir_text(&text)?;
    let header = format!(
        "{}namespace={}\nversion={}\nentries={}\n\n",
        String::from_utf8_lossy(SAFE_TYPELIB_MAGIC),
        document.namespace,
        document.version,
        document.items.len()
    );
    let mut bytes = Vec::with_capacity(header.len() + text.len());
    bytes.extend_from_slice(header.as_bytes());
    bytes.extend_from_slice(text.as_bytes());
    std::fs::write(output, bytes).map_err(|error| format!("{}: {error}", output.display()))
}

pub fn load_typelib_document(
    input: &Path,
    gir_dirs: &[PathBuf],
) -> Result<Arc<RepositoryDocument>, String> {
    let bytes = std::fs::read(input).map_err(|error| format!("{}: {error}", input.display()))?;
    load_typelib_bytes(&bytes, input, input.parent(), gir_dirs)
}

pub fn load_typelib_bytes(
    bytes: &[u8],
    source_path: &Path,
    typelib_dir: Option<&Path>,
    gir_dirs: &[PathBuf],
) -> Result<Arc<RepositoryDocument>, String> {
    if let Some(gir_text) = read_safe_typelib_gir(&bytes) {
        let mut document = parse_gir_text(gir_text)?;
        document.source_path = Some(source_path.to_path_buf());
        return Ok(Arc::new(document));
    }

    let metadata = parse_binary_typelib_metadata(source_path, bytes)?;
    let gir_path = find_gir_for_typelib(&metadata, typelib_dir, gir_dirs);
    if let Some(gir_path) = gir_path {
        let mut document = (*parse_gir_file(&gir_path)?).clone();
        document.typelib = Some(metadata);
        return Ok(Arc::new(document));
    }

    Ok(Arc::new(document_from_typelib_metadata(metadata)))
}

pub fn decompile_typelib_to_gir(input: &Path, gir_dirs: &[PathBuf]) -> Result<String, String> {
    Ok(load_typelib_document(input, gir_dirs)?.to_gir())
}

pub fn load_namespace(
    namespace: &str,
    version: Option<&str>,
    typelib_dirs: &[PathBuf],
    gir_dirs: &[PathBuf],
) -> Result<Arc<RepositoryDocument>, String> {
    let version = version
        .map(str::to_owned)
        .or_else(|| {
            discover_versions(namespace, typelib_dirs, gir_dirs)
                .into_iter()
                .next()
        })
        .ok_or_else(|| format!("no version found for namespace {namespace}"))?;
    let typelib_name = format!("{namespace}-{version}.typelib");
    if let Some(path) = find_file(&typelib_name, typelib_dirs) {
        return load_typelib_document(&path, gir_dirs);
    }

    let gir_name = format!("{namespace}-{version}.gir");
    if let Some(path) = find_file(&gir_name, gir_dirs) {
        return parse_gir_file(&path);
    }

    Err(format!("namespace {namespace}-{version} not found"))
}

pub fn discover_versions(
    namespace: &str,
    typelib_dirs: &[PathBuf],
    gir_dirs: &[PathBuf],
) -> Vec<String> {
    let mut versions = BTreeSet::new();
    let prefix = format!("{namespace}-");
    for dir in typelib_dirs.iter().chain(gir_dirs.iter()) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Some(file_name) = entry.file_name().to_str().map(str::to_owned) else {
                continue;
            };
            let version = file_name.strip_prefix(&prefix).and_then(|rest| {
                rest.strip_suffix(".typelib")
                    .or_else(|| rest.strip_suffix(".gir"))
            });
            if let Some(version) = version {
                versions.insert(version.to_owned());
            }
        }
    }
    versions.into_iter().collect()
}

fn document_from_typelib_metadata(metadata: TypelibMetadata) -> RepositoryDocument {
    let items = metadata
        .entries
        .iter()
        .map(|entry| ItemModel {
            namespace: metadata.namespace.clone(),
            name: entry.name.clone(),
            kind: entry.kind,
            c_identifier: String::new(),
            c_type: String::new(),
            type_name: String::new(),
            type_init: String::new(),
            type_struct: String::new(),
            ref_func: String::new(),
            is_boxed: false,
            is_gtype_struct: false,
            error_domain: String::new(),
            size: None,
            alignment: None,
            callable: None,
            methods: Vec::new(),
            vfuncs: Vec::new(),
            signals: Vec::new(),
            fields: Vec::new(),
            properties: Vec::new(),
            values: Vec::new(),
            implements: Vec::new(),
        })
        .collect();

    RepositoryDocument {
        namespace: metadata.namespace.clone(),
        version: metadata.version.clone(),
        c_prefix: metadata.c_prefix.clone(),
        shared_libraries: metadata.shared_libraries.clone(),
        dependencies: metadata.dependencies.clone(),
        items,
        raw_gir: None,
        source_path: Some(metadata.path.clone()),
        typelib: Some(metadata),
    }
}

fn parse_item(node: &XmlNode, namespace: &str) -> Option<ItemModel> {
    if node.attr("introspectable") == "0" {
        return None;
    }
    let kind = match node.name.as_str() {
        "function" => ItemKind::Function,
        "callback" => ItemKind::Callback,
        "constant" => ItemKind::Constant,
        "enumeration" => ItemKind::Enum,
        "bitfield" => ItemKind::Flags,
        "class" => ItemKind::Object,
        "interface" => ItemKind::Interface,
        "record" => ItemKind::Struct,
        "union" => ItemKind::Union,
        _ => return None,
    };

    let name = node.attr("name").to_owned();
    if name.is_empty() {
        return None;
    }

    let mut item = ItemModel {
        namespace: namespace.to_owned(),
        name,
        kind,
        c_identifier: node.attr("c:identifier").to_owned(),
        c_type: node.attr("c:type").to_owned(),
        type_name: node.attr("glib:type-name").to_owned(),
        type_init: node.attr("glib:get-type").to_owned(),
        type_struct: node.attr("glib:type-struct").to_owned(),
        ref_func: node.attr("glib:ref-func").to_owned(),
        is_boxed: false,
        is_gtype_struct: !node.attr("glib:is-gtype-struct-for").is_empty(),
        error_domain: node.attr("glib:error-domain").to_owned(),
        size: parse_usize(node.attr("size")),
        alignment: parse_usize(node.attr("alignment")),
        callable: None,
        methods: Vec::new(),
        vfuncs: Vec::new(),
        signals: Vec::new(),
        fields: Vec::new(),
        properties: Vec::new(),
        values: Vec::new(),
        implements: Vec::new(),
    };

    let (size, alignment) = layout_guess(&item.c_type);
    item.size = item.size.or(size);
    item.alignment = item.alignment.or(alignment);
    item.is_boxed = kind == ItemKind::Struct
        && !item.type_name.is_empty()
        && !item.type_init.is_empty()
        && !item.is_gtype_struct;

    if matches!(kind, ItemKind::Function | ItemKind::Callback) {
        item.callable = Some(parse_callable(
            node,
            namespace,
            if kind == ItemKind::Callback {
                CallKind::Callback
            } else {
                CallKind::Function
            },
            false,
        ));
    }

    for child in &node.children {
        if child.attr("introspectable") == "0" {
            continue;
        }
        match child.name.as_str() {
            "constructor" | "method" => {
                item.methods
                    .push(parse_callable(child, namespace, CallKind::Function, true));
            }
            "function" => {
                let is_method = matches!(
                    kind,
                    ItemKind::Object | ItemKind::Interface | ItemKind::Struct | ItemKind::Union
                );
                item.methods.push(parse_callable(
                    child,
                    namespace,
                    CallKind::Function,
                    is_method,
                ));
            }
            "virtual-method" => {
                item.vfuncs
                    .push(parse_callable(child, namespace, CallKind::VFunc, false));
            }
            "glib:signal" => {
                item.signals
                    .push(parse_callable(child, namespace, CallKind::Signal, false));
            }
            "field" => item.fields.push(parse_field(child, namespace)),
            "property" => item.properties.push(parse_property(child, namespace)),
            "member" => {
                let value_name = child.attr("name");
                if !value_name.is_empty() {
                    item.values.push(ValueModel {
                        namespace: namespace.to_owned(),
                        name: value_name.to_owned(),
                        c_identifier: child.attr("c:identifier").to_owned(),
                    });
                }
            }
            "implements" => {
                let name = child.attr("name");
                if !name.is_empty() {
                    item.implements.push(resolve_type_ref(namespace, name));
                }
            }
            _ => {}
        }
    }

    Some(item)
}

fn parse_callable(
    node: &XmlNode,
    namespace: &str,
    kind: CallKind,
    is_method: bool,
) -> CallableModel {
    let return_node = node.child("return-value");
    let return_type = return_node
        .map(|node| parse_type_from_container(node, namespace))
        .unwrap_or_else(TypeModel::void);
    let may_return_null = return_node
        .map(|node| node.attr_bool("nullable") || node.attr_bool("allow-none"))
        .unwrap_or(false);
    let instance_transfer = node
        .child("parameters")
        .and_then(|parameters| {
            parameters
                .children
                .iter()
                .find(|child| child.name == "instance-parameter")
        })
        .map(|node| transfer_from_attr(node.attr("transfer-ownership")))
        .unwrap_or(GI_TRANSFER_NOTHING);

    let mut args = Vec::new();
    if let Some(parameters) = node.child("parameters") {
        for parameter in parameters
            .children
            .iter()
            .filter(|child| child.name == "parameter")
        {
            args.push(parse_arg(parameter, namespace));
        }
    }

    CallableModel {
        namespace: namespace.to_owned(),
        name: node.attr("name").to_owned(),
        symbol: node.attr("c:identifier").to_owned(),
        kind,
        throws: node.attr_bool("throws"),
        is_method,
        args,
        return_type,
        instance_transfer,
        may_return_null,
        invoker: node.attr("invoker").to_owned(),
        signal_flags: signal_flags(node),
    }
}

fn parse_arg(node: &XmlNode, namespace: &str) -> ArgModel {
    ArgModel {
        name: node.attr("name").to_owned(),
        direction: if node.attr("direction") == "out" {
            GI_DIRECTION_OUT
        } else {
            GI_DIRECTION_IN
        },
        transfer: transfer_from_attr(node.attr("transfer-ownership")),
        nullable: node.attr_bool("nullable") || node.attr_bool("allow-none"),
        optional: node.attr_bool("optional"),
        caller_allocates: node.attr_bool("caller-allocates"),
        scope: scope_from_attr(node.attr("scope")),
        type_info: parse_type_from_container(node, namespace),
    }
}

fn parse_field(node: &XmlNode, namespace: &str) -> FieldModel {
    FieldModel {
        namespace: namespace.to_owned(),
        name: node.attr("name").to_owned(),
        type_info: parse_type_from_container(node, namespace),
    }
}

fn parse_property(node: &XmlNode, namespace: &str) -> PropertyModel {
    PropertyModel {
        namespace: namespace.to_owned(),
        name: node.attr("name").to_owned(),
        type_info: parse_type_from_container(node, namespace),
    }
}

fn parse_type_from_container(node: &XmlNode, namespace: &str) -> TypeModel {
    if let Some(array) = node.child("array") {
        return parse_array_type(array, namespace);
    }
    if let Some(callback) = node.child("callback") {
        let callable = parse_callable(callback, namespace, CallKind::Callback, false);
        return TypeModel {
            tag: GI_TYPE_TAG_INTERFACE,
            is_pointer: true,
            zero_terminated: false,
            array_type: GI_ARRAY_TYPE_C,
            array_length: None,
            interface: Some(InterfaceRef::Callable(Box::new(callable))),
        };
    }
    if let Some(typ) = node.child("type") {
        return parse_named_type(typ, namespace);
    }
    TypeModel::void()
}

fn parse_array_type(node: &XmlNode, namespace: &str) -> TypeModel {
    let _element = node
        .child("type")
        .map(|child| parse_named_type(child, namespace))
        .unwrap_or_else(TypeModel::void);
    TypeModel {
        tag: GI_TYPE_TAG_ARRAY,
        is_pointer: true,
        zero_terminated: node.attr("zero-terminated") != "0",
        array_type: GI_ARRAY_TYPE_C,
        array_length: parse_usize(node.attr("length")),
        interface: None,
    }
}

fn parse_named_type(node: &XmlNode, namespace: &str) -> TypeModel {
    let name = node.attr("name");
    let c_type = node.attr("c:type");
    let (tag, interface) = type_tag(name, namespace);
    let is_pointer = c_type.contains('*')
        || matches!(
            tag,
            GI_TYPE_TAG_UTF8
                | GI_TYPE_TAG_FILENAME
                | GI_TYPE_TAG_ARRAY
                | GI_TYPE_TAG_INTERFACE
                | GI_TYPE_TAG_GLIST
                | GI_TYPE_TAG_GSLIST
                | GI_TYPE_TAG_GHASH
                | GI_TYPE_TAG_ERROR
        )
        || matches!(name, "gpointer" | "gconstpointer" | "va_list");
    TypeModel {
        tag,
        is_pointer,
        zero_terminated: false,
        array_type: GI_ARRAY_TYPE_C,
        array_length: None,
        interface,
    }
}

fn type_tag(name: &str, namespace: &str) -> (i32, Option<InterfaceRef>) {
    match name {
        "" | "none" => (GI_TYPE_TAG_VOID, None),
        "gboolean" | "boolean" => (GI_TYPE_TAG_BOOLEAN, None),
        "gint8" | "int8" | "gchar" => (GI_TYPE_TAG_INT8, None),
        "guint8" | "uint8" | "guchar" => (GI_TYPE_TAG_UINT8, None),
        "gint16" | "int16" | "gshort" => (GI_TYPE_TAG_INT16, None),
        "guint16" | "uint16" | "gushort" => (GI_TYPE_TAG_UINT16, None),
        "gint" | "gint32" | "int" | "int32" => (GI_TYPE_TAG_INT32, None),
        "guint" | "guint32" | "uint" | "uint32" => (GI_TYPE_TAG_UINT32, None),
        "gint64" | "int64" | "glong" => (GI_TYPE_TAG_INT64, None),
        "guint64" | "uint64" | "gulong" => (GI_TYPE_TAG_UINT64, None),
        "gfloat" | "float" => (GI_TYPE_TAG_FLOAT, None),
        "gdouble" | "double" => (GI_TYPE_TAG_DOUBLE, None),
        "GType" => (GI_TYPE_TAG_GTYPE, None),
        "utf8" => (GI_TYPE_TAG_UTF8, None),
        "filename" => (GI_TYPE_TAG_FILENAME, None),
        "GLib.List" | "List" => (GI_TYPE_TAG_GLIST, None),
        "GLib.SList" | "SList" => (GI_TYPE_TAG_GSLIST, None),
        "GLib.HashTable" | "HashTable" => (GI_TYPE_TAG_GHASH, None),
        "GLib.Error" | "Error" => (GI_TYPE_TAG_ERROR, None),
        "gunichar" | "unichar" => (GI_TYPE_TAG_UNICHAR, None),
        "gpointer" | "gconstpointer" | "va_list" => (GI_TYPE_TAG_VOID, None),
        _ => (
            GI_TYPE_TAG_INTERFACE,
            Some(InterfaceRef::Named(resolve_type_ref(namespace, name))),
        ),
    }
}

fn resolve_type_ref(namespace: &str, name: &str) -> TypeRef {
    if let Some((ns, name)) = name.rsplit_once('.') {
        TypeRef {
            namespace: ns.to_owned(),
            name: name.to_owned(),
        }
    } else {
        TypeRef {
            namespace: namespace.to_owned(),
            name: name.to_owned(),
        }
    }
}

fn transfer_from_attr(value: &str) -> i32 {
    match value {
        "full" => GI_TRANSFER_EVERYTHING,
        "container" => GI_TRANSFER_CONTAINER,
        _ => GI_TRANSFER_NOTHING,
    }
}

fn scope_from_attr(value: &str) -> i32 {
    match value {
        "call" => 1,
        "async" => 2,
        "notified" => 3,
        _ => GI_SCOPE_INVALID,
    }
}

fn signal_flags(node: &XmlNode) -> i32 {
    let mut flags = match node.attr("when") {
        "last" => 2,
        "cleanup" => 4,
        _ => 1,
    };
    if node.attr_bool("no-recurse") {
        flags |= 8;
    }
    if node.attr_bool("detailed") {
        flags |= 16;
    }
    if node.attr_bool("action") {
        flags |= 32;
    }
    if node.attr_bool("no-hooks") {
        flags |= 64;
    }
    flags
}

fn layout_guess(c_type: &str) -> (Option<usize>, Option<usize>) {
    match c_type {
        "GValue" => (Some(24), Some(8)),
        "GDoubleIEEE754" => (Some(8), Some(8)),
        _ => (None, None),
    }
}

fn parse_binary_typelib_metadata(path: &Path, bytes: &[u8]) -> Result<TypelibMetadata, String> {
    if bytes.len() < 60 || &bytes[0..16] != TYPELIB_MAGIC {
        return Err(format!("{}: not a GI typelib", path.display()));
    }
    let n_entries = read_u16(bytes, 20)? as usize;
    let directory = read_u32(bytes, 24)? as usize;
    let dependencies = read_u32(bytes, 36)? as usize;
    let size = read_u32(bytes, 40)? as usize;
    let namespace = read_u32(bytes, 44)? as usize;
    let nsversion = read_u32(bytes, 48)? as usize;
    let shared_library = read_u32(bytes, 52)? as usize;
    let c_prefix = read_u32(bytes, 56)? as usize;
    let entry_size = read_u16(bytes, 60).unwrap_or(12).max(12) as usize;
    if size > bytes.len() {
        return Err(format!("{}: truncated typelib", path.display()));
    }

    let namespace = string_at(bytes, namespace).unwrap_or_default();
    let version = string_at(bytes, nsversion).unwrap_or_default();
    let shared_libraries = split_list(&string_at(bytes, shared_library).unwrap_or_default());
    let dependency_string = string_at(bytes, dependencies).unwrap_or_default();
    let dependencies = split_list(&dependency_string)
        .into_iter()
        .filter_map(|name| {
            let (namespace, version) = name.rsplit_once('-')?;
            Some(Dependency {
                namespace: namespace.to_owned(),
                version: version.to_owned(),
            })
        })
        .collect();
    let c_prefix = string_at(bytes, c_prefix).unwrap_or_default();

    let mut entries = Vec::new();
    for index in 0..n_entries {
        let offset = directory + index * entry_size;
        if offset + 12 > bytes.len() {
            break;
        }
        let blob_type = read_u16(bytes, offset).unwrap_or(0);
        let name_offset = read_u32(bytes, offset + 4).unwrap_or(0) as usize;
        let name = string_at(bytes, name_offset).unwrap_or_default();
        if !name.is_empty() {
            entries.push(TypelibEntry {
                name,
                kind: kind_from_blob_type(blob_type),
            });
        }
    }

    Ok(TypelibMetadata {
        namespace,
        version,
        c_prefix,
        shared_libraries,
        dependencies,
        entries,
        path: path.to_path_buf(),
    })
}

fn kind_from_blob_type(blob_type: u16) -> ItemKind {
    match blob_type {
        1 => ItemKind::Function,
        2 => ItemKind::Callback,
        3 | 4 => ItemKind::Struct,
        5 => ItemKind::Enum,
        6 => ItemKind::Flags,
        7 => ItemKind::Object,
        8 => ItemKind::Interface,
        9 => ItemKind::Constant,
        11 => ItemKind::Union,
        _ => ItemKind::Unresolved,
    }
}

fn read_safe_typelib_gir(bytes: &[u8]) -> Option<&str> {
    if !bytes.starts_with(SAFE_TYPELIB_MAGIC) {
        return None;
    }
    let text = std::str::from_utf8(bytes).ok()?;
    let (_, gir) = text.split_once("\n\n")?;
    Some(gir)
}

fn find_gir_for_typelib(
    metadata: &TypelibMetadata,
    typelib_dir: Option<&Path>,
    gir_dirs: &[PathBuf],
) -> Option<PathBuf> {
    let name = format!("{}-{}.gir", metadata.namespace, metadata.version);
    let mut candidates = Vec::new();
    if let Some(dir) = typelib_dir {
        candidates.push(dir.join(&name));
        candidates.push(dir.join(GIR_SUBDIR).join(&name));
        if let Some(parent) = dir.parent() {
            candidates.push(parent.join(GIR_SUBDIR).join(&name));
            candidates.push(parent.join("share").join(GIR_SUBDIR).join(&name));
        }
    }
    candidates.extend(gir_dirs.iter().map(|dir| dir.join(&name)));
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn find_file(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    dirs.iter()
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

fn parse_xml(text: &str) -> Result<XmlNode, String> {
    let mut stack = vec![XmlNode::new("#document")];
    let mut pos = 0;
    while let Some(rel_start) = text[pos..].find('<') {
        let start = pos + rel_start;
        if text[start..].starts_with("<!--") {
            let Some(rel_end) = text[start + 4..].find("-->") else {
                return Err("unterminated XML comment".to_owned());
            };
            pos = start + 4 + rel_end + 3;
            continue;
        }
        if text[start..].starts_with("<?") {
            let Some(rel_end) = text[start + 2..].find("?>") else {
                return Err("unterminated XML declaration".to_owned());
            };
            pos = start + 2 + rel_end + 2;
            continue;
        }
        if text[start..].starts_with("<!") {
            let Some(end) = find_tag_end(text, start + 2) else {
                return Err("unterminated XML declaration".to_owned());
            };
            pos = end + 1;
            continue;
        }

        let Some(end) = find_tag_end(text, start + 1) else {
            return Err("unterminated XML tag".to_owned());
        };
        let mut tag = text[start + 1..end].trim();
        if tag.starts_with('/') {
            let name = tag[1..].trim();
            let Some(node) = stack.pop() else {
                return Err("XML stack underflow".to_owned());
            };
            if node.name != name {
                return Err(format!(
                    "mismatched XML tag: expected {}, got {name}",
                    node.name
                ));
            }
            let parent = stack
                .last_mut()
                .ok_or_else(|| "XML stack underflow".to_owned())?;
            parent.children.push(node);
        } else {
            let self_closing = tag.ends_with('/');
            if self_closing {
                tag = tag[..tag.len() - 1].trim_end();
            }
            let node = parse_start_tag(tag)?;
            if self_closing {
                let parent = stack
                    .last_mut()
                    .ok_or_else(|| "XML stack underflow".to_owned())?;
                parent.children.push(node);
            } else {
                stack.push(node);
            }
        }
        pos = end + 1;
    }

    while stack.len() > 1 {
        let node = stack.pop().unwrap();
        stack.last_mut().unwrap().children.push(node);
    }
    stack.pop().ok_or_else(|| "empty XML document".to_owned())
}

fn parse_start_tag(tag: &str) -> Result<XmlNode, String> {
    let mut index = 0;
    skip_ws(tag, &mut index);
    let name_start = index;
    while index < tag.len() && !tag.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }
    let name = &tag[name_start..index];
    if name.is_empty() {
        return Err("XML tag without a name".to_owned());
    }
    let mut node = XmlNode::new(name);
    while index < tag.len() {
        skip_ws(tag, &mut index);
        if index >= tag.len() {
            break;
        }
        let key_start = index;
        while index < tag.len()
            && !tag.as_bytes()[index].is_ascii_whitespace()
            && tag.as_bytes()[index] != b'='
        {
            index += 1;
        }
        let key = &tag[key_start..index];
        skip_ws(tag, &mut index);
        if index >= tag.len() || tag.as_bytes()[index] != b'=' {
            node.attrs.insert(key.to_owned(), String::new());
            continue;
        }
        index += 1;
        skip_ws(tag, &mut index);
        if index >= tag.len() {
            node.attrs.insert(key.to_owned(), String::new());
            break;
        }
        let quote = tag.as_bytes()[index];
        if quote != b'"' && quote != b'\'' {
            return Err(format!("XML attribute {key} has no quoted value"));
        }
        index += 1;
        let value_start = index;
        while index < tag.len() && tag.as_bytes()[index] != quote {
            index += 1;
        }
        let value = decode_entities(&tag[value_start..index]);
        if index < tag.len() {
            index += 1;
        }
        node.attrs.insert(key.to_owned(), value);
    }
    Ok(node)
}

fn find_tag_end(text: &str, mut index: usize) -> Option<usize> {
    let mut quote = None;
    while index < text.len() {
        let byte = text.as_bytes()[index];
        match (quote, byte) {
            (Some(q), b) if b == q => quote = None,
            (None, b'"') | (None, b'\'') => quote = Some(byte),
            (None, b'>') => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

fn skip_ws(text: &str, index: &mut usize) {
    while *index < text.len() && text.as_bytes()[*index].is_ascii_whitespace() {
        *index += 1;
    }
}

fn decode_entities(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(['|', ',', ' '])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect()
}

fn first_nonempty<'a>(values: &[&'a str]) -> &'a str {
    values
        .iter()
        .copied()
        .find(|value| !value.is_empty())
        .unwrap_or("")
}

fn parse_usize(value: &str) -> Option<usize> {
    (!value.is_empty()).then(|| value.parse().ok()).flatten()
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let slice = bytes
        .get(offset..offset + 2)
        .ok_or_else(|| "short typelib read".to_owned())?;
    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let slice = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| "short typelib read".to_owned())?;
    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn string_at(bytes: &[u8], offset: usize) -> Option<String> {
    if offset == 0 || offset >= bytes.len() {
        return None;
    }
    let rest = &bytes[offset..];
    let end = rest.iter().position(|byte| *byte == 0)?;
    Some(String::from_utf8_lossy(&rest[..end]).into_owned())
}
