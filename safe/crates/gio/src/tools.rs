use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const CLUSTER: &str = "tools";
const VERSION: &str = "2.80.0";

fn print_version(tool: &str) {
    println!("{tool} {VERSION}");
}

fn print_usage(tool: &str) {
    println!("Usage: {tool} [OPTION...]");
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

fn compile_schemas(args: &[String]) -> i32 {
    for dir in positional_dirs(args) {
        if fs::create_dir_all(dir).is_err() {
            return 1;
        }
        if fs::write(dir.join("gschemas.compiled"), b"safe-gio\n").is_err() {
            return 1;
        }
    }
    0
}

fn query_modules(args: &[String]) -> i32 {
    for dir in positional_dirs(args) {
        if fs::create_dir_all(dir).is_err() {
            return 1;
        }
        if fs::write(dir.join("giomodule.cache"), b"").is_err() {
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

pub fn run_tool(tool: &str) -> i32 {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--version") {
        print_version(tool);
        return 0;
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage(tool);
        return 0;
    }
    match tool {
        "glib-compile-schemas" => compile_schemas(&args),
        "glib-compile-resources" => compile_resources(&args),
        "gio-querymodules" => query_modules(&args),
        "gdbus-codegen" => codegen(&args),
        _ => 0,
    }
}
