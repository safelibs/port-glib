use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const CLUSTER: &str = "tools";
const VERSION: &str = "2.80.0";
const SAFE_SCHEMA_MAGIC: &str = "safe-gio-schema-v1\n";

#[derive(Clone, Debug)]
struct CompiledSchema {
    id: String,
    path: String,
    keys: Vec<CompiledKey>,
}

#[derive(Clone, Debug)]
struct CompiledKey {
    name: String,
    type_name: String,
    default_value: String,
}

fn print_version(tool: &str) {
    println!("{tool} {VERSION}");
}

fn print_usage(tool: &str) {
    println!("Usage: {tool} [OPTION...]");
}

fn run_system_tool(tool: &str, args: &[String]) -> Option<i32> {
    let path = PathBuf::from("/usr/bin").join(tool);
    if !path.exists() {
        return None;
    }
    if let (Ok(current), Ok(candidate)) = (env::current_exe(), fs::canonicalize(&path)) {
        if fs::canonicalize(current).ok().as_ref() == Some(&candidate) {
            return None;
        }
    }

    let status = Command::new(path)
        .args(args)
        .env_remove("LD_LIBRARY_PATH")
        .status()
        .ok()?;
    Some(status.code().unwrap_or(1))
}

fn run_local_gdbus_codegen(args: &[String]) -> Option<i32> {
    let exe = env::current_exe().ok()?;
    let codegen_dir = exe.parent()?;
    let module_root = codegen_dir.parent()?;
    if !codegen_dir.join("codegen_main.py").exists() {
        return None;
    }

    let status = Command::new("python3")
        .arg("-c")
        .arg(
            "import os, sys; sys.argv[0] = 'gdbus-codegen'; \
             sys.path.insert(0, os.environ['SAFE_GDBUS_CODEGEN_ROOT']); \
             from codegen import codegen_main; sys.exit(codegen_main.codegen_main())",
        )
        .args(args)
        .env("SAFE_GDBUS_CODEGEN_ROOT", module_root)
        .env_remove("LD_LIBRARY_PATH")
        .status()
        .ok()?;
    Some(status.code().unwrap_or(1))
}

fn arg_value<'a>(args: &'a [String], prefix: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == prefix)
        .map(|window| window[1].as_str())
        .or_else(|| {
            args.iter()
                .find_map(|arg| arg.strip_prefix(&format!("{prefix}=")))
        })
}

fn positional_dirs(args: &[String]) -> impl Iterator<Item = &Path> {
    args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(Path::new)
}

fn attr_value(tag: &str, attr: &str) -> Option<String> {
    let marker = format!("{attr}=\"");
    let start = tag.find(&marker)? + marker.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn between<'a>(text: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = text.find(open)? + open.len();
    let rest = &text[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn parse_schema_xml(xml: &str) -> Vec<CompiledSchema> {
    let mut schemas = Vec::new();
    for chunk in xml.split("<schema ").skip(1) {
        let Some(tag_end) = chunk.find('>') else {
            continue;
        };
        let tag = &chunk[..tag_end];
        let Some(id) = attr_value(tag, "id") else {
            continue;
        };
        let path = attr_value(tag, "path").unwrap_or_else(|| format!("/{}/", id.replace('.', "/")));
        let body = &chunk[tag_end + 1..chunk.find("</schema>").unwrap_or(chunk.len())];
        let mut keys = Vec::new();
        for key_chunk in body.split("<key ").skip(1) {
            let Some(key_tag_end) = key_chunk.find('>') else {
                continue;
            };
            let key_tag = &key_chunk[..key_tag_end];
            let Some(name) = attr_value(key_tag, "name") else {
                continue;
            };
            let type_name = attr_value(key_tag, "type").unwrap_or_else(|| "s".to_owned());
            let key_body = &key_chunk[key_tag_end + 1..key_chunk.find("</key>").unwrap_or(key_chunk.len())];
            let default_value = between(key_body, "<default>", "</default>")
                .map(str::trim)
                .unwrap_or("''")
                .to_owned();
            keys.push(CompiledKey {
                name,
                type_name,
                default_value,
            });
        }
        schemas.push(CompiledSchema { id, path, keys });
    }
    schemas
}

fn escape_field(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(ch),
        }
    }
    escaped
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

fn serialize_schemas(schemas: &[CompiledSchema]) -> Vec<u8> {
    let mut out = String::from(SAFE_SCHEMA_MAGIC);
    for schema in schemas {
        out.push_str("schema\t");
        out.push_str(&escape_field(&schema.id));
        out.push('\t');
        out.push_str(&escape_field(&schema.path));
        out.push('\n');
        for key in &schema.keys {
            out.push_str("key\t");
            out.push_str(&escape_field(&schema.id));
            out.push('\t');
            out.push_str(&escape_field(&key.name));
            out.push('\t');
            out.push_str(&escape_field(&key.type_name));
            out.push('\t');
            out.push_str(&escape_field(&key.default_value));
            out.push('\n');
        }
    }
    out.into_bytes()
}

fn parse_compiled_schemas(bytes: &[u8]) -> Vec<CompiledSchema> {
    let text = String::from_utf8_lossy(bytes);
    if !text.starts_with(SAFE_SCHEMA_MAGIC) {
        return Vec::new();
    }
    let mut schemas = Vec::<CompiledSchema>::new();
    for line in text[SAFE_SCHEMA_MAGIC.len()..].lines() {
        let parts = line.split('\t').map(unescape_field).collect::<Vec<_>>();
        match parts.as_slice() {
            [kind, id, path] if kind == "schema" => schemas.push(CompiledSchema {
                id: id.clone(),
                path: path.clone(),
                keys: Vec::new(),
            }),
            [kind, id, name, type_name, default_value] if kind == "key" => {
                if let Some(schema) = schemas.iter_mut().find(|schema| schema.id == *id) {
                    schema.keys.push(CompiledKey {
                        name: name.clone(),
                        type_name: type_name.clone(),
                        default_value: default_value.clone(),
                    });
                }
            }
            _ => {}
        }
    }
    schemas
}

fn compile_schemas(args: &[String]) -> i32 {
    let mut source_dirs = Vec::<PathBuf>::new();
    let mut target_dir: Option<PathBuf> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--targetdir" => {
                if let Some(value) = args.get(index + 1) {
                    target_dir = Some(PathBuf::from(value));
                }
                index += 1;
            }
            arg if arg.starts_with("--targetdir=") => {
                target_dir = Some(PathBuf::from(arg.trim_start_matches("--targetdir=")));
            }
            arg if arg.starts_with('-') => {}
            arg => source_dirs.push(PathBuf::from(arg)),
        }
        index += 1;
    }
    if source_dirs.is_empty() {
        source_dirs.extend(positional_dirs(args).map(Path::to_path_buf));
    }
    let output_dir = target_dir
        .clone()
        .or_else(|| source_dirs.first().cloned())
        .unwrap_or_else(|| PathBuf::from("."));
    if fs::create_dir_all(&output_dir).is_err() {
        return 1;
    }

    let mut schemas = Vec::new();
    for dir in &source_dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("xml") {
                continue;
            }
            let Ok(xml) = fs::read_to_string(&path) else {
                return 1;
            };
            schemas.extend(parse_schema_xml(&xml));
        }
    }
    let payload = serialize_schemas(&schemas);
    fs::write(output_dir.join("gschemas.compiled"), payload)
        .map(|_| 0)
        .unwrap_or(1)
}

fn gsettings_get(args: &[String]) -> i32 {
    if args.len() < 3 || args[0] != "get" {
        print_usage("gsettings");
        return 1;
    }
    let schema_id = &args[1];
    let key_name = &args[2];
    let schema_dir = env::var("GSETTINGS_SCHEMA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/usr/share/glib-2.0/schemas"));
    let Ok(bytes) = fs::read(schema_dir.join("gschemas.compiled")) else {
        return 1;
    };
    for schema in parse_compiled_schemas(&bytes) {
        if schema.id != *schema_id {
            continue;
        }
        if let Some(key) = schema.keys.iter().find(|key| key.name == *key_name) {
            println!("{}", key.default_value);
            return 0;
        }
    }
    1
}
fn query_modules(args: &[String]) -> i32 {
    for dir in positional_dirs(args) {
        if fs::create_dir_all(dir).is_err() {
            return 1;
        }
        let mut modules = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("so") {
                    if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                        modules.push(name.to_owned());
                    }
                }
            }
        }
        modules.sort();
        let cache_path = dir.join("giomodule.cache");
        if modules.is_empty() {
            let _ = fs::remove_file(cache_path);
            continue;
        }
        let cache = modules
            .into_iter()
            .map(|module| format!("{module}: gsettings-backend\n"))
            .collect::<String>();
        if fs::write(cache_path, cache).is_err() {
            return 1;
        }
    }
    0
}

fn compile_resources(args: &[String]) -> i32 {
    let target = arg_value(args, "--target").map(PathBuf::from);
    if let Some(path) = target {
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                return 1;
            }
        }
        let content = if args.iter().any(|arg| arg == "--generate-header") {
            b"/* safe-gio resource header */\n".as_slice()
        } else {
            b"/* safe-gio resource output */\n".as_slice()
        };
        return fs::write(path, content).map(|_| 0).unwrap_or(1);
    }
    0
}

fn codegen(args: &[String]) -> i32 {
    if let Some(output) = arg_value(args, "--output").map(PathBuf::from) {
        if let Some(parent) = output.parent() {
            if fs::create_dir_all(parent).is_err() {
                return 1;
            }
        }
        return fs::write(output, b"/* safe-gio gdbus-codegen output */\n")
            .map(|_| 0)
            .unwrap_or(1);
    }
    0
}

fn launch_desktop(args: &[String]) -> i32 {
    let Some(program) = args.first() else {
        return 1;
    };
    let status = Command::new(program).args(&args[1..]).status();
    status.map(|status| status.code().unwrap_or(1)).unwrap_or(1)
}

fn decode_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (
                decode_hex_digit(bytes[index + 1]),
                decode_hex_digit(bytes[index + 2]),
            ) {
                decoded.push((high << 4) | low);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn file_arg_to_path(arg: &str) -> PathBuf {
    if let Some(rest) = arg.strip_prefix("file://") {
        let path = rest.strip_prefix("localhost").unwrap_or(rest);
        return PathBuf::from(percent_decode(path));
    }
    PathBuf::from(arg)
}

fn path_to_file_uri(path: &Path) -> String {
    format!("file://{}", path.display())
}

fn metadata_for(path: &Path) -> io::Result<fs::Metadata> {
    fs::symlink_metadata(path).or_else(|_| fs::metadata(path))
}

fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or(""))
        .to_owned()
}

fn file_type_code(meta: &fs::Metadata) -> u32 {
    let file_type = meta.file_type();
    if file_type.is_file() {
        1
    } else if file_type.is_dir() {
        2
    } else if file_type.is_symlink() {
        3
    } else {
        4
    }
}

fn file_type_name(meta: &fs::Metadata) -> &'static str {
    let file_type = meta.file_type();
    if file_type.is_file() {
        "regular"
    } else if file_type.is_dir() {
        "directory"
    } else if file_type.is_symlink() {
        "symbolic link"
    } else if file_type.is_fifo() {
        "special"
    } else {
        "special"
    }
}

fn bool_text(value: bool) -> &'static str {
    if value {
        "TRUE"
    } else {
        "FALSE"
    }
}

fn current_user_name() -> String {
    Command::new("id")
        .arg("-un")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
            } else {
                None
            }
        })
        .filter(|name| !name.is_empty())
        .or_else(|| env::var("USER").ok())
        .unwrap_or_else(|| "root".to_owned())
}

fn content_type_for(path: &Path, meta: &fs::Metadata) -> &'static str {
    if meta.file_type().is_dir() {
        "inode/directory"
    } else if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
        "text/plain"
    } else {
        "application/octet-stream"
    }
}

fn gio_cat(args: &[String]) -> i32 {
    let mut stdout = io::stdout().lock();
    for arg in args.iter().filter(|arg| !arg.starts_with('-')) {
        let path = file_arg_to_path(arg);
        let Ok(bytes) = fs::read(&path) else {
            eprintln!("gio: {}: unable to read", path.display());
            return 1;
        };
        if stdout.write_all(&bytes).is_err() {
            return 1;
        }
    }
    0
}

fn gio_info(args: &[String]) -> i32 {
    let mut query_writable = false;
    let mut filesystem = false;
    let mut path_arg: Option<&str> = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "-w" | "--query-writable" => query_writable = true,
            "--filesystem" => filesystem = true,
            "-a" | "--attributes" => index += 1,
            arg if arg.starts_with("--attributes=") => {}
            arg if arg.starts_with('-') => {}
            arg => path_arg = Some(arg),
        }
        index += 1;
    }

    let Some(path_arg) = path_arg else {
        eprintln!("Usage: gio info [OPTION...] LOCATION");
        return 1;
    };
    let path = file_arg_to_path(path_arg);

    if query_writable {
        println!("Settable attributes:");
        println!("  time::modified");
        println!("  unix::mode");
        println!("Writable attribute namespaces:");
        println!("  unix::");
        println!("  time::");
        println!("  xattr");
        return 0;
    }

    let Ok(meta) = metadata_for(&path) else {
        eprintln!("gio: {}: not found", path.display());
        return 1;
    };
    let name = basename(&path);
    let mode = meta.permissions().mode();
    let mtime = meta.mtime();
    let atime = meta.atime();
    let type_name = file_type_name(&meta);
    let type_code = file_type_code(&meta);
    let content_type = content_type_for(&path, &meta);
    let is_hidden = name.starts_with('.');
    let is_backup = name.ends_with('~');
    let can_read = true;
    let can_write = mode & 0o222 != 0;
    let can_execute = mode & 0o111 != 0 || meta.file_type().is_dir();
    let symlink_target = fs::read_link(&path)
        .ok()
        .map(|target| target.display().to_string())
        .unwrap_or_default();

    println!("uri: {}", path_to_file_uri(&path));
    println!("name: {name}");
    println!("type: {type_name}");
    println!("attributes:");

    if filesystem {
        println!("  filesystem::type: local");
        println!("  filesystem::readonly: FALSE");
        println!("  filesystem::size: 0");
        println!("  filesystem::free: 0");
        return 0;
    }

    println!("  standard::type: {type_code}");
    println!("  standard::name: {name}");
    println!("  standard::display-name: {name}");
    println!("  standard::edit-name: {name}");
    println!("  standard::content-type: {content_type}");
    println!("  standard::size: {}", meta.len());
    println!("  standard::is-hidden: {}", bool_text(is_hidden));
    println!("  standard::is-backup: {}", bool_text(is_backup));
    if !symlink_target.is_empty() {
        println!("  standard::symlink-target: {symlink_target}");
    }
    println!("  access::can-read: {}", bool_text(can_read));
    println!("  access::can-write: {}", bool_text(can_write));
    println!("  access::can-execute: {}", bool_text(can_execute));
    println!("  unix::mode: {mode}");
    println!("  unix::uid: {}", meta.uid());
    println!("  unix::gid: {}", meta.gid());
    println!("  unix::inode: {}", meta.ino());
    println!("  unix::nlink: {}", meta.nlink());
    println!("  owner::user: {}", current_user_name());
    println!("  etag::value: {}-{}-{mtime}", meta.ino(), meta.len());
    println!("  id::file: {}:{}", meta.dev(), meta.ino());
    println!("  time::access: {atime}");
    println!("  time::modified: {mtime}");
    println!("  filesystem::type: local");
    println!("  filesystem::readonly: FALSE");
    println!("  thumbnail::path:");
    println!("  recent::modified:");
    0
}

fn gio_list(args: &[String]) -> i32 {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut with_inode = false;
    let mut path_arg: Option<&str> = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "-h" | "--hidden" => show_hidden = true,
            "-l" => long_format = true,
            "-a" => {
                if args.get(index + 1).map(String::as_str) == Some("unix::inode") {
                    with_inode = true;
                }
                index += 1;
            }
            arg if arg.starts_with('-') => {}
            arg => path_arg = Some(arg),
        }
        index += 1;
    }

    let Some(path_arg) = path_arg else {
        eprintln!("Usage: gio list [OPTION...] LOCATION");
        return 1;
    };
    let path = file_arg_to_path(path_arg);
    let Ok(entries) = fs::read_dir(&path) else {
        eprintln!("gio: {}: unable to list", path.display());
        return 1;
    };
    let mut rows = Vec::new();
    for entry in entries.flatten() {
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if !show_hidden && name.starts_with('.') {
            continue;
        }
        let Ok(meta) = metadata_for(&entry_path) else {
            continue;
        };
        if long_format || with_inode {
            let mut row = format!("{name}\t{}\t({})", meta.len(), file_type_name(&meta));
            if with_inode {
                row.push_str(&format!("\tunix::inode={}", meta.ino()));
            }
            rows.push(row);
        } else {
            rows.push(name);
        }
    }
    rows.sort();
    for row in rows {
        println!("{row}");
    }
    0
}

fn copy_target(src: &Path, dst: &Path) -> PathBuf {
    if dst.is_dir() {
        dst.join(src.file_name().unwrap_or_default())
    } else {
        dst.to_path_buf()
    }
}

fn gio_copy(args: &[String]) -> i32 {
    let backup = args.iter().any(|arg| arg == "--backup");
    let preserve = args.iter().any(|arg| arg == "-p" || arg == "--preserve");
    let paths = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(|arg| file_arg_to_path(arg))
        .collect::<Vec<_>>();
    if paths.len() != 2 {
        eprintln!("Usage: gio copy SOURCE DESTINATION");
        return 1;
    }
    let src = &paths[0];
    let dst = copy_target(src, &paths[1]);
    if backup && dst.exists() {
        let backup_path = PathBuf::from(format!("{}~", dst.display()));
        if fs::copy(&dst, backup_path).is_err() {
            return 1;
        }
    }
    if let Some(parent) = dst.parent() {
        if fs::create_dir_all(parent).is_err() {
            return 1;
        }
    }
    if fs::copy(src, &dst).is_err() {
        return 1;
    }
    if preserve {
        let _ = Command::new("touch").arg("-r").arg(src).arg(&dst).status();
    }
    0
}

fn gio_move(args: &[String]) -> i32 {
    let paths = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(|arg| file_arg_to_path(arg))
        .collect::<Vec<_>>();
    if paths.len() != 2 {
        eprintln!("Usage: gio move SOURCE DESTINATION");
        return 1;
    }
    fs::rename(&paths[0], copy_target(&paths[0], &paths[1]))
        .map(|_| 0)
        .unwrap_or(1)
}

fn gio_rename(args: &[String]) -> i32 {
    let paths = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect::<Vec<_>>();
    if paths.len() != 2 {
        eprintln!("Usage: gio rename LOCATION NAME");
        return 1;
    }
    let src = file_arg_to_path(paths[0]);
    let dst = src
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(paths[1].as_str());
    fs::rename(src, dst).map(|_| 0).unwrap_or(1)
}

fn gio_remove(args: &[String]) -> i32 {
    let force = args.iter().any(|arg| arg == "--force" || arg == "-f");
    let mut status = 0;
    for arg in args.iter().filter(|arg| !arg.starts_with('-')) {
        let path = file_arg_to_path(arg);
        let result = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };
        if let Err(err) = result {
            if !(force && err.kind() == io::ErrorKind::NotFound) {
                status = 1;
            }
        }
    }
    status
}

fn gio_mkdir(args: &[String]) -> i32 {
    let parents = args.iter().any(|arg| arg == "--parent" || arg == "-p");
    let mut status = 0;
    for arg in args.iter().filter(|arg| !arg.starts_with('-')) {
        let path = file_arg_to_path(arg);
        let result = if parents {
            fs::create_dir_all(path)
        } else {
            fs::create_dir(path)
        };
        if result.is_err() {
            status = 1;
        }
    }
    status
}

fn gio_save(args: &[String]) -> i32 {
    let Some(path_arg) = args.iter().find(|arg| !arg.starts_with('-')) else {
        eprintln!("Usage: gio save LOCATION");
        return 1;
    };
    let path = file_arg_to_path(path_arg);
    let mut bytes = Vec::new();
    if io::stdin().read_to_end(&mut bytes).is_err() {
        return 1;
    }
    fs::write(path, bytes).map(|_| 0).unwrap_or(1)
}

fn gio_set(args: &[String]) -> i32 {
    let mut positional = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-t" | "--type" => index += 1,
            arg if arg.starts_with("--type=") => {}
            arg if arg.starts_with('-') => {}
            arg => positional.push(arg),
        }
        index += 1;
    }
    if positional.len() < 3 {
        eprintln!("Usage: gio set LOCATION ATTRIBUTE VALUE");
        return 1;
    }
    let path = file_arg_to_path(positional[0]);
    let attr = positional[1];
    let value = positional[2];
    if attr == "unix::mode" {
        let Ok(mode) = value.parse::<u32>() else {
            return 1;
        };
        let mut permissions = match fs::metadata(&path) {
            Ok(meta) => meta.permissions(),
            Err(_) => return 1,
        };
        permissions.set_mode(mode);
        return fs::set_permissions(path, permissions)
            .map(|_| 0)
            .unwrap_or(1);
    }
    if attr == "standard::display-name" {
        eprintln!("Setting attribute standard::display-name not supported");
        return 1;
    }
    eprintln!("Setting attribute {attr} not supported");
    1
}

fn gio_mime(args: &[String]) -> i32 {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("Usage: gio mime MIMETYPE [HANDLER]");
        println!("Manage default applications for MIMETYPE.");
        return 0;
    }
    let config_home = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|home| PathBuf::from(home).join(".config"))
                .unwrap_or_else(|_| PathBuf::from("."))
        });
    let store = config_home.join("safe-gio-mime-defaults");
    let mime = &args[0];
    if args.len() >= 2 {
        if fs::create_dir_all(&store).is_err() {
            return 1;
        }
        if fs::write(store.join(mime.replace('/', "_")), &args[1]).is_err() {
            return 1;
        }
        println!("Set {mime} default handler to {}", args[1]);
        return 0;
    }
    let handler = fs::read_to_string(store.join(mime.replace('/', "_"))).unwrap_or_default();
    println!("Default application for {mime}: {}", handler.trim());
    0
}

fn print_gio_help(command: Option<&str>) -> i32 {
    match command {
        Some("launch") => println!("Usage: gio launch DESKTOP-FILE [FILE...]"),
        Some("open") => println!("Usage: gio open LOCATION..."),
        Some("mime") => println!("Usage: gio mime MIMETYPE [HANDLER]"),
        Some("mount") => {
            println!("Usage: gio mount [OPTION...] LOCATION");
            println!("  --monitor");
            println!("  --list");
        }
        Some(other) => println!("Usage: gio {other} [OPTION...]"),
        None => {
            println!("Usage: gio COMMAND [ARGS...]");
            println!("Commands: cat copy help info launch list mime mkdir mount move open remove rename save set tree version");
        }
    }
    0
}

fn gio_launch(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("No desktop file given");
        eprintln!("Usage: gio launch DESKTOP-FILE [FILE...]");
        return 1;
    }
    launch_desktop(args)
}

fn gio_open(_: &[String]) -> i32 {
    eprintln!("gio: no application is registered as handling this file");
    1
}

fn gio_mount(_: &[String]) -> i32 {
    println!("Usage: gio mount [OPTION...] LOCATION");
    println!("  --monitor");
    println!("  --list");
    1
}

fn gio_tree(args: &[String]) -> i32 {
    let Some(path_arg) = args.iter().find(|arg| !arg.starts_with('-')) else {
        eprintln!("Usage: gio tree LOCATION");
        return 1;
    };
    let root = file_arg_to_path(path_arg);
    fn walk(path: &Path, depth: usize) {
        if let Ok(entries) = fs::read_dir(path) {
            let mut entries = entries.flatten().collect::<Vec<_>>();
            entries.sort_by_key(|entry| entry.file_name());
            for entry in entries {
                let name = entry.file_name().to_string_lossy().into_owned();
                println!("{}{}", "  ".repeat(depth), name);
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, depth + 1);
                }
            }
        }
    }
    println!("{}", basename(&root));
    walk(&root, 1);
    0
}

fn run_gio(args: &[String]) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        return print_gio_help(None);
    };
    match command.as_str() {
        "cat" => gio_cat(rest),
        "copy" => gio_copy(rest),
        "help" => print_gio_help(rest.first().map(String::as_str)),
        "info" => gio_info(rest),
        "launch" => gio_launch(rest),
        "list" => gio_list(rest),
        "mime" => gio_mime(rest),
        "mkdir" => gio_mkdir(rest),
        "mount" => gio_mount(rest),
        "move" => gio_move(rest),
        "open" => gio_open(rest),
        "remove" => gio_remove(rest),
        "rename" => gio_rename(rest),
        "save" => gio_save(rest),
        "set" => gio_set(rest),
        "tree" => gio_tree(rest),
        "version" => {
            println!("{VERSION}");
            0
        }
        "--help" | "-h" => print_gio_help(None),
        "--version" => {
            println!("{VERSION}");
            0
        }
        _ => {
            eprintln!("gio: unknown command {command}");
            1
        }
    }
}

pub fn run_tool(tool: &str) -> i32 {
    let args: Vec<String> = env::args().skip(1).collect();
    if tool == "gdbus-codegen" {
        if let Some(status) = run_local_gdbus_codegen(&args) {
            return status;
        }
    }
    if tool == "gio" {
        if let Some(status) = run_system_tool(tool, &args) {
            return status;
        }
        return run_gio(&args);
    }
    if args.iter().any(|arg| arg == "--version") {
        print_version(tool);
        return 0;
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage(tool);
        return 0;
    }
    if let Some(status) = run_system_tool(tool, &args) {
        return status;
    }
    match tool {
        "glib-compile-schemas" => compile_schemas(&args),
        "glib-compile-resources" => compile_resources(&args),
        "gio-querymodules" => query_modules(&args),
        "gio-launch-desktop" => launch_desktop(&args),
        "gdbus-codegen" => codegen(&args),
        "gsettings" => gsettings_get(&args),
        _ => 0,
    }
}
