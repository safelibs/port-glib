# GLib Rust Port Report

This report documents the checked-out `port-glib` safe workspace for phase
`impl-gobject-rust`. Earlier versions of this living document covered the
`impl-glib-advanced` phase; the still-accurate workspace and packaging facts
have been reconciled here, and the phase-specific content now focuses on the
GObject runtime in `safe/crates/gobject`.

## High-level architecture

The safe port is a Rust workspace under `safe/` that builds Ubuntu 24.04 GLib
replacement shared libraries while preserving the upstream C ABI. The workspace
is declared in `safe/Cargo.toml:1-11` and contains `abi-support`, `safe-glib`,
`safe-gthread`, `safe-gmodule`, `safe-gobject`, `safe-gio`, and
`safe-girepository`. The workspace package settings use Rust 2021,
`LGPL-2.1-or-later`, and version `0.1.0` (`safe/Cargo.toml:13-16`).

The GObject crate itself is `safe-gobject` (`safe/crates/gobject/Cargo.toml:1-6`).
It is compiled as an `rlib` and `staticlib`, not directly as a `cdylib`
(`safe/crates/gobject/Cargo.toml:8-9`). The final shared object is linked by
`safe/tools/build-abi-shell.py`: the GObject entry configures crate
`safe-gobject`, output directory `gobject`, SONAME `libgobject-2.0.so.0`,
realname `libgobject-2.0.so.0.8000.0`, static archive `libgobject-2.0.a`,
version script `safe/abi/version-scripts/libgobject.map`, dependencies on the
safe GLib/GThread/GModule libraries, and native libraries `ffi`, `quadmath`,
`dl`, `pthread`, and `m` (`safe/tools/build-abi-shell.py:60-69`). The crate
`build.rs` enforces local safety comments outside translated-audit files
(`safe/crates/gobject/build.rs:7-70`), watches `src/` and
`safe/abi/version-scripts/libgobject.map` (`safe/crates/gobject/build.rs:71-82`),
keeps historical `SAFE_LINK_*` linker argument hooks as no-ops for the
staticlib path (`safe/crates/gobject/build.rs:84-94`), and emits Cargo link
lines for `dl`, `ffi`, `m`, `pthread`, `glib-2.0`, `gthread-2.0`, and
`gmodule-2.0` (`safe/crates/gobject/build.rs:95-101`). Debian packaging calls
that ABI shell from `override_dh_auto_build` and stages the package tree from
`override_dh_auto_install` (`safe/debian/rules:70-78`). By default it appends
`nodoc noinsttest nogir noudeb` profiles unless `SAFE_FULL_PACKAGE_BUILD` is set
(`safe/debian/rules:3-6`) and skips Debian build-time tests during the ABI shell
phase (`safe/debian/rules:90-94`).

The public boundary is the upstream-compatible GObject C ABI. `safe-gobject`
uses `#![feature(c_variadic, extern_types)]` and the same translated-code lint
allowances as the surrounding GLib-family crates (`safe/crates/gobject/src/lib.rs:1-9`).
It imports shared primitive ABI types from `safe/crates/abi-support/src/ffi.rs`
through a `#[path = "../../abi-support/src/ffi.rs"]` module
(`safe/crates/gobject/src/lib.rs:14-15`), declares runtime modules for
`object`, `signal`, `tools`, `translated`, `type_system`, and `value`
(`safe/crates/gobject/src/lib.rs:17-22`), and re-exports the public `repr(C)`
structs and callback typedefs through `pub mod abi` (`safe/crates/gobject/src/lib.rs:29-54`).
The phase marker is `bootstrap_marker() -> "impl-gobject-rust"`
(`safe/crates/gobject/src/lib.rs:56-60`).

The hand-owned ABI modules define the stable C layouts that translated GObject
code is expected to share:

- `safe/crates/gobject/src/object/mod.rs` defines `GObject`, `GParamSpec`,
  `GObjectClass`, `GParamSpecClass`, `GParamSpecTypeInfo`, and object/property
  callback typedefs (`safe/crates/gobject/src/object/mod.rs:6-102`).
- `safe/crates/gobject/src/type_system/mod.rs` defines the GType class,
  instance, interface, plugin, query, value-table, and interface-info layouts
  plus their callback typedefs (`safe/crates/gobject/src/type_system/mod.rs:4-112`).
- `safe/crates/gobject/src/value/mod.rs` defines `GValueData`, `GValue`,
  `GValueArray`, `GTypeCValue`, and `GValueTransform`
  (`safe/crates/gobject/src/value/mod.rs:5-44`).
- `safe/crates/gobject/src/signal/mod.rs` defines closure marshalling typedefs,
  `GSignalInvocationHint`, `GSignalQuery`, `GClosureNotifyData`, `GClosure`,
  and `GCClosure` (`safe/crates/gobject/src/signal/mod.rs:4-64`).
- `safe/crates/gobject/src/tools/mod.rs:1` records the shipped GObject helper
  tools: `gobject-query`, `glib-genmarshal`, and `glib-mkenums`.

The implementation body is still mostly translated Rust. `safe/crates/gobject/src/translated/mod.rs:1-3`
loads `build_check`, `compat`, and `original`; the current translated runtime is
under `safe/crates/gobject/src/translated/original/gobject/`, corresponding
to upstream files such as `original/gobject/gobject.c`, `original/gobject/gsignal.c`,
`original/gobject/gtype.c`, `original/gobject/gvalue.c`, `original/gobject/gclosure.c`,
`original/gobject/gparam.c`, and `original/gobject/gboxed.c`. The translated
modules expose the real exported `g_object_*`, `g_type_*`, `g_signal_*`,
`g_value_*`, closure, boxed, binding, and parameter symbols as `pub unsafe extern
"C" fn` functions, and the version script constrains the final export surface
(`safe/abi/version-scripts/libgobject.map`, 483 lines). Layout compatibility is
tracked in `safe/abi/layout-manifests/gobject.json` and
`safe/abi/layouts/gobject.json`; both currently contain 21 GObject layout entries,
and the layout probe has matching GObject entries for `GTypeInfo`, `GTypeQuery`,
`GTypeValueTable`, `GInterfaceInfo`, `GTypeInstance`, `GTypeClass`, `GValue`,
`GObject`, `GParamSpec`, `GSignalQuery`, `GClosure`, `GCClosure`, and related
public structs (`safe/crates/abi-support/src/bin/layout-probe.rs:160-330`).

Data flow is therefore ABI-first. A C caller links against `libgobject-2.0.so.0`
and lands on an exported GObject symbol. That symbol enters either a translated
`pub unsafe extern "C" fn` body or a callback stored in a `repr(C)` class,
closure, type, value, or parameter-spec table. The translated code then moves
through raw C-compatible pointers, GLib allocation/quark/container APIs, GType
registration tables, GValue storage, signal/closure marshalling, and libffi when
a generic closure invocation needs runtime ABI marshalling. The upstream test
mirror and manifest focus on the externally visible behavior: `safe/tests/manifests/gobject.txt`
contains 62 GObject rows, including binding, closure ownership, signal ordering,
refcount stress, C++ consumption, and helper-tool tests.

Directory map:

```text
safe/
  Cargo.toml                                  workspace and shared package settings
  crates/gobject/Cargo.toml                   safe-gobject package, staticlib build
  crates/gobject/build.rs                     unsafe-audit and link metadata for GObject
  crates/gobject/src/lib.rs                   module map and public ABI re-exports
  crates/gobject/src/object/mod.rs            GObject/GParamSpec layouts and callbacks
  crates/gobject/src/type_system/mod.rs       GType/GTypePlugin layouts and callbacks
  crates/gobject/src/value/mod.rs             GValue/GTypeCValue layouts
  crates/gobject/src/signal/mod.rs            GClosure/GSignal layouts and callbacks
  crates/gobject/src/tools/mod.rs             helper-tool inventory
  crates/gobject/src/translated/compat.rs     translated-code compatibility helpers
  crates/gobject/src/translated/original/     C2Rust-translated GObject runtime
  crates/abi-support/src/ffi.rs               shared GLib-family ABI primitive aliases
  crates/abi-support/src/bin/layout-probe.rs  layout-probe binary for public structs
  abi/version-scripts/libgobject.map          exported libgobject symbol contract
  abi/layout-manifests/gobject.json           expected public layout contract
  abi/layouts/gobject.json                    observed Rust public layouts
  tests/upstream/gobject/                     editable upstream GObject test mirror
  tests/manifests/gobject.txt                 frozen 62-row GObject test manifest
  tools/build-abi-shell.py                    staticlib-to-shared-object linker
  tools/run-meson-manifest.py                 manifest replay and GObject contract checks
  debian/                                     Ubuntu package glue
```

## Where the unsafe Rust lives

`grep`/`rg` evidence for this refresh found 2581 source lines and
2582 `unsafe` tokens under `safe/crates/gobject` and
`safe/crates/abi-support`, excluding only binary/non-Rust files. There are no
`unsafe impl` lines in those paths (`rg -n '^\s*unsafe\s+impl\b' ...` returned
0). The classification table below is line-based and intentionally conservative:
`unsafe extern "C" fn` definitions are also counted in the broader
extern-typedef/cast column because both forms contain the same token sequence.
The extern-definition column only counts named function definitions of the form
`unsafe extern "C" fn name(...)`; callback typedef continuations such as
`safe/crates/gobject/src/signal/mod.rs:5` are counted only as typedef/cast lines.

| File | extern typedef/cast | extern definition | block | unsafe fn | unsafe impl | attr/link_section | other |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `crates/abi-support/src/ffi.rs` | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/gobject/build.rs` | 0 | 0 | 1 | 0 | 0 | 2 | 4 |
| `crates/gobject/src/object/mod.rs` | 14 | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/gobject/src/signal/mod.rs` | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/gobject/src/translated/build_check/gobject/glib_enumtypes.rs` | 4 | 4 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/compat.rs` | 0 | 0 | 0 | 12 | 0 | 0 | 0 |
| `crates/gobject/src/translated/original/gobject/gatomicarray.rs` | 5 | 5 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gbinding.rs` | 97 | 41 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gbindinggroup.rs` | 59 | 22 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gboxed.rs` | 458 | 97 | 1 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gclosure.rs` | 53 | 34 | 0 | 6 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/genums.rs` | 49 | 26 | 1 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gmarshal.rs` | 91 | 45 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gobject.rs` | 282 | 155 | 2 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gparam.rs` | 127 | 63 | 1 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gparamspecs.rs` | 282 | 137 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gsignal.rs` | 169 | 96 | 3 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gsignalgroup.rs` | 70 | 31 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gsourceclosure.rs` | 62 | 13 | 1 | 1 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gtype.rs` | 138 | 119 | 7 | 1 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gtypemodule.rs` | 59 | 18 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gtypeplugin.rs` | 18 | 5 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gvalue.rs` | 33 | 24 | 1 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gvaluearray.rs` | 14 | 11 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gvaluetransform.rs` | 263 | 94 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/gvaluetypes.rs` | 137 | 81 | 10 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/translated/original/gobject/mod.rs` | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| `crates/gobject/src/type_system/mod.rs` | 17 | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/gobject/src/value/mod.rs` | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| TOTAL | 2506 | 1121 | 28 | 20 | 0 | 2 | 26 |

Grouped justifications:

- ABI callback typedefs and `repr(C)` table fields: `safe/crates/abi-support/src/ffi.rs:34,36`,
  `safe/crates/gobject/src/object/mod.rs:37,39,41-43,45-50,52,54,56`,
  `safe/crates/gobject/src/signal/mod.rs:5,7`,
  `safe/crates/gobject/src/type_system/mod.rs:4-18,97-102`, and
  `safe/crates/gobject/src/value/mod.rs:44` model public C callback slots. They
  are required at the ABI boundary because C callers and subclasses install
  function pointers that Rust cannot prove safe.
- Translated GObject exported functions and callbacks: every `unsafe extern "C"`
  definition in the files under `safe/crates/gobject/src/translated/original/gobject/`
  is part of the current C2Rust runtime for upstream GObject behavior. These
  bodies dereference raw `GObject`, `GType`, `GValue`, `GParamSpec`, closure, and
  signal pointers, access translated global state, and call back through C
  vtables; this unsafe surface is required by the current drop-in C ABI strategy
  until the modules are rewritten behind narrower safe internals.
- Translated function-pointer casts and table initialization: the many
  non-definition `unsafe extern "C" fn` matches in translated files populate
  `GTypeValueTable`, class vtables, boxed copy/free hooks, property handlers,
  signal marshallers, and closure callbacks. They are ABI-required because GLib
  stores exact C function pointer signatures in public tables.
- Translated `unsafe fn` helpers: `safe/crates/gobject/src/translated/compat.rs:44,49,54,59,65,74,83,96,109,122,135,148`
  exposes raw-pointer casts and atomic operations needed by translated code;
  `safe/crates/gobject/src/translated/original/gobject/gclosure.rs`,
  `gsourceclosure.rs`, and `gtype.rs` also contain internal translated helper
  `unsafe fn` lines. These are not direct exported C ABI shims, but they support
  translated raw-pointer and atomic behavior inherited from the C implementation.
- Raw `unsafe { ... }` blocks in translated files are localized to generated
  expressions that call unsafe Rust operations inside otherwise translated code,
  for example function pointer dispatch or global/static raw state access. They
  are part of the translated runtime rather than a new handwritten safe
  abstraction boundary.
- `safe/crates/gobject/src/translated/build_check/gobject/glib_enumtypes.rs`
  is a build-check translation unit, not part of the exported runtime, but its
  unsafe lines still mirror C enum type-registration callbacks and remain in the
  source tree.
- `safe/crates/gobject/build.rs:9-14,62` are audit-script string/pattern matches,
  not runtime unsafe Rust. They appear in the grep inventory because the build
  script looks for unsafe syntax and prints an unsafe-audit error message.

The exact current unsafe line inventory is:

```text
- `crates/abi-support/src/ffi.rs: 2 matching line(s): 34, 36`
- `crates/gobject/build.rs: 7 matching line(s): 9-14, 62`
- `crates/gobject/src/object/mod.rs: 14 matching line(s): 37, 39, 41-43, 45-50, 52, 54, 56`
- `crates/gobject/src/signal/mod.rs: 2 matching line(s): 5, 7`
- `crates/gobject/src/translated/build_check/gobject/glib_enumtypes.rs: 5 matching line(s): 1, 281, 473, 761, 1637`
- `crates/gobject/src/translated/compat.rs: 12 matching line(s): 44, 49, 54, 59, 65, 74, 83, 96, 109, 122, 135, 148`
- `crates/gobject/src/translated/original/gobject/gatomicarray.rs: 6 matching line(s): 1, 92, 167, 174, 178, 221`
- `crates/gobject/src/translated/original/gobject/gbinding.rs: 98 matching line(s): 1, 177, 237-238, 291, 310, 312, 314, 337, 339, 341-343, 345-347, 362, 407, 446, 454, 515, 518, 523, 527, 530, 534, 549, 552, 557, 561, 564, 571, 602, 612, 617, 619-620, 622, 626, 628-629, 631, 638, 672, 695, 709, 776, 809, 869, 946, 1024, 1062, 1071, 1082, 1087, 1148, 1192, 1196, 1289, 1306, 1310, 1314, 1317, 1329, 1342, 1347, 1355, 1358, 1372, 1381, 1400-1401, 1404, 1406, 1409, 1411-1413, 1491, 1506, 1532, 1563, 1589, 1620, 1646, 1672, 1698, 1727, 2016, 2035, 2148, 2261, 2275, 2292, 2312, 2335, 2348, 2359`
- `crates/gobject/src/translated/original/gobject/gbindinggroup.rs: 60 matching line(s): 1, 117, 183-184, 237, 256, 272, 274, 276-278, 280-282, 297, 300, 344, 375, 383, 388, 390-391, 393, 397, 399-400, 402, 413, 535, 552, 593, 643, 650, 684, 719, 740, 775, 807, 839, 843-844, 846-847, 850, 852, 855, 857, 875, 878, 882, 887, 920, 990, 1048, 1068, 1091, 1234, 1245, 1265, 1290`
- `crates/gobject/src/translated/original/gobject/gboxed.rs: 460 matching line(s): 1, 207, 281-284, 288-289, 294-295, 297, 439, 441-450, 472, 475, 479-480, 483, 487, 493-494, 500, 506-507, 513, 519-520, 526, 532-533, 539, 545-546, 552, 558-559, 565, 571-572, 578, 584-585, 591, 597-598, 604, 610-611, 617, 623-624, 630, 636-637, 643, 649, 651, 657, 663-664, 670, 676-677, 683, 689-690, 696, 702-703, 709, 715-716, 722, 728-729, 735, 741-742, 748, 754-755, 761, 768, 770, 776, 782-783, 789, 795-796, 802, 808-809, 815, 821-822, 828, 834-835, 841, 847, 849, 855, 861-862, 868, 874-875, 881, 887-888, 894, 900-901, 923, 942, 946, 952-953, 959, 965-966, 980, 986-987, 991, 999, 1008, 1014, 1025, 1062, 1065, 1083, 1115, 1117, 1119-1120, 1123, 1129, 1133, 1139, 1141, 1143-1144, 1147, 1152, 1155, 1161, 1193, 1195, 1197-1198, 1201, 1207, 1211, 1217, 1249, 1251, 1253-1254, 1257, 1263, 1267, 1273, 1305, 1307, 1309-1310, 1313, 1318, 1321, 1327, 1359, 1361, 1363-1364, 1367, 1373, 1377, 1383, 1415, 1447, 1449, 1451-1452, 1455, 1460, 1463, 1469, 1471, 1473-1474, 1477, 1483, 1487, 1493, 1525, 1527, 1529-1530, 1533, 1539, 1543, 1549, 1581, 1583, 1585-1586, 1589, 1594, 1597, 1603, 1635, 1667, 1669, 1671-1672, 1675, 1680, 1683, 1689, 1691, 1693-1694, 1697, 1702, 1705, 1711, 1743, 1745, 1747-1748, 1751, 1757, 1761, 1767, 1799, 1831, 1833, 1835-1836, 1839, 1846, 1851, 1858, 1890, 1892, 1894-1895, 1898, 1905, 1910, 1917, 1949, 1951, 1953-1954, 1957, 1963, 1968, 1975, 1977, 1979-1980, 1983, 1989, 1993, 1999, 2031, 2063, 2065, 2067-2068, 2071, 2077, 2081, 2087, 2089, 2091-2092, 2095, 2101, 2105, 2111, 2143, 2175, 2177, 2179-2180, 2183, 2189, 2193, 2199, 2231, 2233, 2235-2236, 2239, 2245, 2249, 2255, 2287, 2289, 2291-2292, 2295, 2302, 2307, 2314, 2316, 2318-2319, 2322, 2327, 2330, 2336, 2368, 2400, 2402, 2404-2405, 2408, 2413, 2416, 2422, 2424, 2426-2427, 2430, 2435, 2438, 2444, 2476, 2508, 2510, 2512-2513, 2516, 2522, 2526, 2532, 2534, 2536-2537, 2540, 2546, 2551, 2558, 2590, 2622, 2624, 2626-2627, 2630, 2635, 2638, 2644, 2646, 2648-2649, 2652, 2657, 2660, 2666, 2698, 2700, 2702-2703, 2706, 2713, 2719, 2726, 2758, 2760, 2762-2763, 2766, 2771, 2774, 2780, 2812, 2814, 2816-2817, 2820, 2826, 2830, 2836, 2868, 2870, 2872-2873, 2876, 2881, 2884, 2890, 2922, 2954, 2956, 2958-2959, 2962, 2968, 2973, 2980, 3012, 3014, 3016-3017, 3020, 3026, 3031, 3038, 3070, 3072, 3074-3075, 3078, 3084, 3089, 3096, 3118, 3121, 3124, 3127, 3142, 3145, 3149, 3163, 3180, 3183, 3209, 3246, 3251, 3253-3254, 3256, 3259, 3264, 3274, 3344, 3397, 3431, 3480, 3496, 3531, 3576, 3610, 3650, 3688, 3695`
- `crates/gobject/src/translated/original/gobject/gclosure.rs: 60 matching line(s): 1, 32, 252, 271, 273, 275, 285, 314, 318, 322, 326, 330, 334, 394, 413, 658, 720, 835, 925, 980, 1037, 1217, 1321, 1424, 1496, 1581, 1667, 1730, 1937, 2005, 2032, 2059, 2086, 2113, 2254, 2272, 2415, 2449, 2461, 2500, 2524, 2589, 2616, 2648, 2680, 2697, 2719, 2737, 2774, 2812, 2826, 2843, 2857, 2871, 3007, 3136, 3205, 3330, 3345, 3508`
- `crates/gobject/src/translated/original/gobject/genums.rs: 51 matching line(s): 1, 160, 162-171, 226, 245, 296, 298, 300, 304, 310, 320, 398, 401, 408, 436, 460, 469, 472, 510, 519, 522, 560, 598, 601, 607, 645, 648, 653, 701, 738, 790, 842, 895, 948, 987, 1038, 1067, 1186, 1209, 1235, 1261, 1287`
- `crates/gobject/src/translated/original/gobject/gmarshal.rs: 92 matching line(s): 1, 138, 157, 166-167, 169, 171-174, 176-183, 185-190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238, 257, 295, 325, 369, 403, 447, 481, 525, 559, 603, 637, 681, 715, 759, 793, 837, 871, 915, 949, 993, 1027, 1071, 1105, 1149, 1183, 1227, 1275, 1319, 1367, 1411, 1467, 1511, 1545, 1589, 1629, 1673, 1721, 1768, 1804, 1859, 1904, 1965, 2020, 2078`
- `crates/gobject/src/translated/original/gobject/gobject.rs: 284 matching line(s): 1, 306-308, 312, 325, 364, 374-384, 386, 388, 390, 397-398, 400, 486, 488-497, 510, 517, 551-553, 555-556, 586, 605, 607, 609, 639, 671, 673, 675-677, 679-681, 698, 752, 801, 809, 833-834, 838, 841, 855, 878, 883, 888, 903, 906, 915, 926, 936, 940, 945, 965, 1006, 1058, 1085, 1091, 1097, 1105, 1122, 1126, 1150, 1220, 1257, 1269, 1272, 1275, 1278, 1281, 1284, 1291, 1294, 1298, 1300-1301, 1303, 1306, 1311, 1321, 1365, 1370, 1406, 1424, 1442, 1456, 1458-1460, 1463, 1465, 1468, 1470-1474, 1477, 1479, 1499, 1503, 1530, 1596, 1665, 1718, 1733, 1782, 1869, 1880, 1927, 1935, 1978, 2013, 2104, 2139, 2166, 2181, 2189, 2198, 2204, 2212, 2219, 2230, 2236, 2243, 2249, 2257, 2264, 2290, 2316, 2331, 2334, 2352, 2405, 2445, 2466, 2487, 2534, 2567, 2610, 2678, 2684, 2713, 2806, 2994, 2998, 3018, 3057, 3208, 3234, 3245, 3299, 3326, 3384, 3454, 3536, 3793, 3823, 3825, 3870, 3930, 4066, 4097, 4153, 4274, 4298, 4322, 4335, 4422, 4454, 4616, 4648, 4733, 4749, 4833, 4838, 4905, 4933-4934, 4940, 4968-4969, 4974, 5061, 5080, 5127, 5168, 5206, 5233, 5320-5321, 5327, 5406, 5501, 5524, 5601, 5826, 5835, 5859, 5889, 5920, 5960, 6002, 6031, 6060, 6095, 6131, 6171, 6213, 6247, 6251, 6258, 6280, 6307, 6310, 6351, 6386, 6452, 6459, 6520, 6546, 6579, 6634, 6639, 6662, 6693, 6705, 6713, 6784, 6789-6790, 6793-6794, 6826, 6831, 6876, 6930, 6984, 6991, 6998, 7007, 7041, 7049, 7054, 7056-7057, 7059, 7063, 7065-7066, 7068, 7075, 7078-7079, 7088, 7107, 7113, 7124, 7171, 7273, 7313, 7319, 7367`
- `crates/gobject/src/translated/original/gobject/gparam.rs: 129 matching line(s): 1, 140-144, 251, 253-262, 281, 315-317, 319-320, 350, 352-354, 356, 363-365, 367, 370, 390, 397, 405, 407-408, 410, 413, 418, 428, 440, 443, 446, 449, 452, 455, 464, 467, 495, 498-500, 505-506, 512, 527, 542, 568, 601, 637, 674, 691, 717, 742, 753, 759, 784, 875, 899, 929, 971, 1000, 1015, 1078, 1151, 1220, 1292, 1378, 1470, 1474, 1484, 1500, 1528, 1531, 1573, 1609, 1622, 1636, 1654, 1657, 1659-1660, 1667, 1675, 1751, 1789, 1855, 1936, 1949, 1978, 1984, 1999, 2022, 2058, 2081, 2136, 2139, 2153, 2173, 2190, 2192, 2204, 2213, 2273, 2284, 2286, 2288, 2292, 2296, 2299, 2309, 2366, 2373, 2422, 2448, 2483, 2532`
- `crates/gobject/src/translated/original/gobject/gparamspecs.rs: 283 matching line(s): 1, 239-241, 243-244, 253, 255-257, 259, 471, 489, 495, 499, 508, 528, 534, 538, 547, 564, 568, 578, 588, 594, 598, 607, 623, 638, 644, 648, 656, 672, 687, 693, 697, 705, 721, 736, 742, 746, 754, 770, 785, 791, 795, 803, 819, 834, 841, 845, 854, 872, 887, 892, 896, 902, 914, 929, 934, 945, 949, 959, 976, 981, 992, 996, 1004, 1018, 1025, 1029, 1037, 1053, 1072, 1080, 1084, 1094, 1112, 1131, 1141, 1155, 1160, 1247, 1276, 1309-1310, 1314, 1328, 1348-1349, 1353, 1368-1369, 1373, 1388, 1395, 1415, 1427, 1451, 1506, 1559-1560, 1564, 1578, 1600, 1615-1616, 1628, 1636, 1644, 1652, 1661-1662, 1666, 1676, 1691, 1704, 1710, 1722, 1728, 1744, 1760, 1776, 1812, 1818, 1830, 1834, 1837, 1841, 1850, 1852, 1871, 1875, 1878, 1882, 1891, 1893, 1916, 1920, 1924, 1933, 1935, 1954, 1958, 1961, 1965, 1974, 1976, 1995, 1999, 2002, 2006, 2015, 2017, 2036, 2040, 2043, 2047, 2056, 2058, 2077, 2081, 2084, 2088, 2097, 2099, 2118, 2122, 2125, 2129, 2138, 2140, 2159, 2163, 2166, 2170, 2179, 2181, 2200, 2204, 2208, 2212, 2221, 2223, 2242, 2244, 2246, 2249, 2253, 2262, 2264, 2283, 2285, 2287, 2290, 2294, 2303, 2305, 2324, 2328, 2331, 2335, 2344, 2346, 2365, 2369, 2372, 2376, 2385, 2387, 2406, 2408, 2410, 2413, 2417, 2426, 2428, 2447, 2451, 2454, 2458, 2467, 2469, 2488, 2492, 2497, 2521, 2525, 2530, 2554, 2556, 2559, 2563, 2567, 2592, 2596, 2599, 2603, 2612, 2614, 2633, 2635, 2637, 2641, 2645, 2654, 2656, 2675, 2679, 2682, 2686, 2696, 2698, 2717, 2719, 2721, 2725, 2729, 2738, 2740, 2769, 2805, 2841, 2873, 2907, 2941, 2975, 3009, 3043, 3077, 3097, 3143, 3189, 3223, 3257, 3278, 3309, 3349, 3367, 3387, 3425, 3460, 3509`
- `crates/gobject/src/translated/original/gobject/gsignal.rs: 173 matching line(s): 1, 497-499, 545, 618, 620-624, 648, 667, 669, 671, 681, 732, 735, 804, 894, 899, 911, 919, 938, 970, 1009, 1023, 1048, 1116, 1147, 1164, 1168, 1173, 1177, 1182, 1186, 1203, 1210, 1221, 1227, 1238, 1299, 1313, 1325, 1358, 1382, 1385, 1391, 1446, 1460, 1472, 1626, 1662, 1675, 1768, 1816, 1859, 1864, 1890, 1909, 1920, 1942, 1946, 1959-1960, 1966, 1991, 2081, 2091, 2171, 2192, 2241, 2284, 2286, 2298, 2358, 2462, 2516, 2610, 2624, 2651, 2694, 2741, 2781, 2793, 2822, 3079, 3090, 3114, 3125, 3146, 3157, 3178, 3189, 3210, 3221, 3242, 3253, 3274, 3285, 3306, 3317, 3338, 3349, 3370, 3381, 3402, 3413, 3434, 3445, 3466, 3477, 3498, 3509, 3530, 3541, 3562, 3573, 3594, 3605, 3626, 3637, 3656, 3667, 3692, 3735, 3784, 3821, 3878, 3935, 4039, 4377, 4400, 4501, 4610, 4634, 4727, 4736, 4775, 4798, 4834, 4857, 4894, 4920, 4950, 4978, 5025, 5076, 5111, 5157, 5164, 5210, 5217, 5264, 5272, 5348, 5358, 5441, 5465, 5476, 6072, 6083, 6139, 6157, 6758, 6762, 6766, 6774, 6779, 6812, 6827, 6841, 6851`
- `crates/gobject/src/translated/original/gobject/gsignalgroup.rs: 71 matching line(s): 1, 149, 216-217, 270, 289, 291, 293, 323, 348, 350, 352-354, 356-358, 373, 417, 430, 462, 470, 475, 477-478, 480, 484, 486-487, 489, 499, 555, 615, 679, 790, 845, 865, 900, 963, 988, 1082, 1176, 1206, 1255, 1274, 1293, 1311, 1319, 1354, 1389, 1393-1394, 1396-1397, 1399-1400, 1403, 1405, 1408, 1410, 1462, 1465, 1470, 1493, 1588, 1596, 1652, 1683, 1702, 1719, 1736`
- `crates/gobject/src/translated/original/gobject/gsourceclosure.rs: 65 matching line(s): 1, 143-146, 150-151, 156-157, 159, 201, 210, 219, 221, 223-224, 226-227, 299, 318, 321, 340, 343, 347-348, 351, 361, 393, 395, 397-398, 401, 407, 411, 417, 487, 526, 599, 672, 745, 772, 783, 787, 791, 795, 800, 805, 812, 819, 822-823, 825, 828-829, 831, 835, 844, 847, 856, 895, 912, 927, 940, 969, 980`
- `crates/gobject/src/translated/original/gobject/gtype.rs: 147 matching line(s): 1, 151-152, 287, 289-298, 318-319, 445-446, 492, 505, 808, 821, 835, 1108, 1141, 1193, 1244, 1315, 1330, 1381, 1413, 1425, 1499, 1501, 1558, 1617, 1642, 1729, 1885, 1904, 2010, 2030, 2231, 2272, 2322, 2404, 2447, 2470, 2505, 2547, 2706, 2785, 2889, 3029, 3091, 3132, 3156, 3181, 3225, 3262, 3293, 3359, 3593, 3808, 3850, 3919, 3984, 4042, 4346, 4413, 4472, 4542, 4651, 4704, 4734, 4789, 4819, 4874, 5002, 5054, 5143, 5213, 5262, 5314, 5372, 5413, 5454, 5493, 5533, 5576, 5610, 5646, 5695, 5723, 5752, 5765, 5775, 5795, 5805, 5815, 5837, 5894, 5907, 5922, 5942, 5977, 6035, 6061, 6084, 6134, 6164, 6229, 6259, 6263, 6333, 6337, 6347, 6409, 6425, 6434, 6461, 6475, 6495, 6551, 6599, 6629, 6687, 6691, 6696, 6706, 6804, 6814, 6822, 6830, 6838, 6847-6848, 6907, 6930, 6945, 7022, 7072, 7160, 7190, 7255, 7304, 7414`
- `crates/gobject/src/translated/original/gobject/gtypemodule.rs: 60 matching line(s): 1, 187, 189-198, 207-208, 272, 274, 276-278, 280-282, 327-332, 354, 374, 389, 418, 423-424, 426-427, 429, 431, 434-435, 437, 441, 450, 454, 462, 465, 476, 479, 499, 528, 542, 561, 619, 665, 682, 695, 707, 805, 883, 956`
- `crates/gobject/src/translated/original/gobject/gtypeplugin.rs: 19 matching line(s): 1, 80, 82-91, 100-101, 124, 149, 180, 211, 267`
- `crates/gobject/src/translated/original/gobject/gvalue.rs: 35 matching line(s): 1, 126, 128-133, 148, 161, 173, 205, 244, 269, 337, 368, 388, 392, 398, 402, 411, 459, 512, 547, 583, 607, 646, 744, 828, 835, 874, 900, 929, 955, 985`
- `crates/gobject/src/translated/original/gobject/gvaluearray.rs: 15 matching line(s): 1, 41, 58, 60, 91, 116, 155, 170, 196, 231, 247, 263, 319, 368, 392`
- `crates/gobject/src/translated/original/gobject/gvaluetransform.rs: 264 matching line(s): 1, 88, 107, 126, 176, 189, 196, 203, 210, 217, 224, 231, 238, 245, 252, 259, 266, 273, 280, 287, 294, 302, 309, 317, 324, 331, 338, 345, 352, 359, 367, 374, 382, 389, 396, 403, 410, 417, 425, 432, 439, 447, 454, 461, 468, 475, 482, 490, 497, 504, 512, 519, 527, 534, 541, 548, 556, 564, 572, 580, 587, 594, 601, 608, 615, 622, 629, 637, 645, 652, 660, 667, 674, 681, 689, 697, 705, 713, 721, 729, 737, 745, 753, 761, 770, 779, 788, 797, 806, 815, 824, 833, 846, 854, 862, 964, 969, 975, 980, 986, 992, 997, 1002, 1007, 1012, 1017, 1022, 1027, 1032, 1037, 1042, 1048, 1054, 1059, 1065, 1071, 1076, 1081, 1086, 1091, 1096, 1101, 1106, 1111, 1116, 1121, 1127, 1134, 1140, 1145, 1150, 1155, 1160, 1165, 1170, 1175, 1180, 1185, 1190, 1196, 1202, 1207, 1212, 1217, 1222, 1227, 1232, 1237, 1242, 1247, 1252, 1257, 1262, 1267, 1273, 1279, 1284, 1289, 1294, 1299, 1304, 1309, 1314, 1319, 1324, 1329, 1334, 1339, 1344, 1350, 1356, 1361, 1366, 1372, 1378, 1383, 1388, 1393, 1398, 1403, 1408, 1413, 1418, 1423, 1429, 1435, 1441, 1447, 1453, 1459, 1465, 1472, 1478, 1483, 1488, 1493, 1498, 1503, 1508, 1514, 1521, 1527, 1532, 1537, 1543, 1550, 1556, 1561, 1566, 1571, 1576, 1581, 1587, 1594, 1601, 1607, 1613, 1620, 1627, 1634, 1640, 1645, 1650, 1655, 1661, 1667, 1672, 1677, 1683, 1689, 1694, 1699, 1704, 1709, 1714, 1719, 1725, 1731, 1737, 1744, 1751, 1757, 1762, 1767, 1772, 1777, 1782, 1787, 1793, 1800, 1807, 1814, 1820, 1825, 1830, 1835, 1840, 1846, 1853, 1860, 1867, 1874, 1881, 1888`
- `crates/gobject/src/translated/original/gobject/gvaluetypes.rs: 148 matching line(s): 1, 175, 177-186, 206, 225, 275, 278, 282, 305, 328, 338, 361, 371, 394, 397, 401, 411, 434, 437, 441, 451, 474, 477, 481, 491, 514, 518, 526, 542, 568, 603, 607, 611, 614, 624, 647, 658, 671, 690, 726, 743, 745, 748, 754, 764, 810, 812, 815, 821, 831, 859, 861, 864, 870, 880, 926, 928, 931, 937, 947, 993, 995, 998, 1004, 1014, 1060, 1062, 1065, 1071, 1081, 1109, 1111, 1114, 1120, 1130, 1158, 1160-1161, 1163, 1166, 1171, 1181, 1209, 1211, 1214, 1217, 1222, 1232, 1260, 1262-1263, 1265, 1268, 1273, 1283, 1313, 1339, 1365, 1391, 1417, 1443, 1469, 1496, 1522, 1548, 1574, 1600, 1626, 1652, 1678, 1704, 1730, 1756, 1782, 1808, 1834, 1860, 1886, 1912, 1938, 1974, 2011, 2049, 2056, 2090, 2116, 2144, 2180, 2206, 2232, 2239, 2271, 2297, 2323, 2360, 2397, 2423, 2454, 2722`
- `crates/gobject/src/translated/original/gobject/mod.rs: 1 matching line(s): 10`
- `crates/gobject/src/type_system/mod.rs: 17 matching line(s): 4-12, 14, 16-18, 97-98, 100, 102`
- `crates/gobject/src/value/mod.rs: 1 matching line(s): 44`
```

Unsafe that is not required by the public GObject C ABI boundary is limited to
support code around the translated runtime: the `compat.rs` raw conversion and
atomic helpers, the translated `build_check` enumtype file, and the `build.rs`
audit-pattern strings. Everything else in the inventory is either a public C ABI
shape, an exported C-compatible function, a C callback slot, or translated code
that currently implements those ABI-visible behaviors.

## Remaining unsafe FFI beyond the original ABI/API boundary

The intended boundary is the public GLib/GObject C ABI: exported symbols in
`safe/abi/version-scripts/libgobject.map`, public layouts in
`safe/abi/layout-manifests/gobject.json`, and callback/table definitions in
`safe/crates/gobject/src/object/mod.rs`, `safe/crates/gobject/src/signal/mod.rs`,
`safe/crates/gobject/src/type_system/mod.rs`, and
`safe/crates/gobject/src/value/mod.rs`. That boundary is expected for a drop-in
`libgobject-2.0.so.0` replacement and is not counted as extra FFI.

Additional FFI remains inside the implementation:

| Surface | Evidence | Symbols or libraries | Why it remains | Possible future replacement |
| --- | --- | --- | --- | --- |
| Safe GLib-family intra-library calls | `safe/crates/gobject/build.rs:99-101`, `safe/tools/build-abi-shell.py:68`; extern imports such as `g_malloc`, `g_free`, `g_mutex_lock`, `g_quark_from_string`, `g_type_register_static`, and `g_value_init` in translated files | `glib-2.0`, `gthread-2.0`, `gmodule-2.0`; many `g_*` imports in the files under `safe/crates/gobject/src/translated/original/gobject/` | The translated GObject runtime still calls the GLib/GThread/GModule C ABI for allocation, quarks, atomics/once, locks, containers, modules, type registration, and values. In the safe build these resolve mostly to sibling Rust-owned shared libraries, but the call shape is still C FFI. | Replace intra-workspace C calls with Rust module APIs once the GObject runtime is rewritten enough to share internal safe abstractions without breaking the external ABI. |
| libffi closure marshalling | `safe/crates/gobject/build.rs:96`, `safe/tools/build-abi-shell.py:69`, and imports in `safe/crates/gobject/src/translated/original/gobject/gclosure.rs:23,30` | `ffi_prep_cif`, `ffi_call`, `libffi.so.8` | GObject generic closures and marshallers need runtime call-frame construction for arbitrary signal and closure signatures. `readelf -d build-gobject/gobject/libgobject-2.0.so.0.8000.0` shows `NEEDED [libffi.so.8]`. | A Rust-native marshalling layer could replace this only if it preserves libffi's ABI behavior for all supported C signatures. |
| libc and compiler runtime | Translated extern imports include `memcpy`, `memmove`, `memset`, `strcmp`, `strncmp`, `strncpy`, `strchr`, `strlen`, and `qsort`, for example in `safe/crates/gobject/src/translated/original/gobject/gobject.rs:10-34`, `gparam.rs:5-26`, and `gsignal.rs:6-27`. The built shared object needs `libc.so.6`, `libgcc_s.so.1`, and `ld-linux-x86-64.so.2`. | C runtime memory/string/sort functions and ELF runtime support | C2Rust preserved upstream C operations and ABI-compatible varargs/string behavior. | Replace mechanical translations with safe Rust slices, strings, sorting, and ownership helpers where doing so does not alter C-observable behavior. |
| Link flags for `dl`, `pthread`, `m`, and `quadmath` | `safe/crates/gobject/build.rs:95,97-98`; `safe/tools/build-abi-shell.py:69` | `libdl`, `pthread`, `libm`, `quadmath` at link time | The ABI shell links with the same GLib-family native support libraries used by surrounding crates and translated code. On this build, `readelf -d` did not show separate `libdl`, `libpthread`, `libm`, or `libquadmath` `NEEDED` entries for `libgobject-2.0.so.0.8000.0`, but the build configuration still supplies them. | Prune link flags once the translated objects and sibling library contracts no longer require them for supported targets. |
| Dynamic type/plugin ABI | `safe/crates/gobject/src/type_system/mod.rs:97-102`, translated `gtypemodule.rs`, `gtypeplugin.rs`, and upstream dynamic tests in `safe/tests/upstream/gobject/meson.build:73-75` | `GTypePluginUse`, `GTypePluginUnuse`, `GTypePluginCompleteTypeInfo`, `GTypePluginCompleteInterfaceInfo`, `GTypeModule`/`GTypePlugin` symbols | GObject's public API lets C plugins and modules provide dynamic types and interfaces. This is part of GObject's intended ABI, but it extends trust to callback code outside Rust. | The external plugin ABI cannot be removed while remaining GLib-compatible; only the internal bookkeeping around plugin callbacks can become safer. |

I found no `bindgen` or `cbindgen` usage in `safe/Cargo.toml`, `safe/Cargo.lock`,
`safe/crates/`, `safe/tools/`, `safe/debian/`, or `safe/meson.build`. There are
no third-party C++ libraries linked by the GObject crate itself. The current
runtime shared object reported these dynamic dependencies: `libglib-2.0.so.0`,
`libffi.so.8`, `libgcc_s.so.1`, `libc.so.6`, `ld-linux-x86-64.so.2`, and SONAME
`libgobject-2.0.so.0`.

## Remaining issues

- The GObject runtime is still mostly mechanically translated Rust, not a small
  safe Rust core. The unsafe inventory above has 2581 matching
  lines, and `safe/crates/gobject/src/lib.rs:2-9` still carries broad lint
  allowances such as `dead_code`, `unused_unsafe`, and `improper_ctypes` for the
  translated code.
- The generated/build-check surface remains in the crate tree:
  `safe/crates/gobject/src/translated/mod.rs:1` loads `build_check`, and
  `safe/crates/gobject/src/translated/build_check/gobject/glib_enumtypes.rs`
  still has unsafe translated enumtype registration callbacks. This is useful
  as a compile check, but it is not a cleaned handwritten implementation.
- `cargo geiger -p safe-gobject --all-features --output-format GitHubMarkdown`
  could not run in this environment because Cargo reported `no such command:
  geiger`. The unsafe inventory therefore comes from `rg`/scripted source scans,
  not cargo-geiger.
- `cargo check --workspace` passed, but the warning surface is still large in
  the workspace. The completed check reported hundreds to thousands of warnings
  in surrounding crates, including `safe-glib` and `safe-gio`, and the GObject
  crate itself intentionally allows several translated-code warning classes.
- The upstream GObject test mirror still carries skipped-test and TODO/FIXME
  evidence. The current source-level skip calls are
  `safe/tests/upstream/gobject/param.c:1403` for slow `/param/implement`,
  `safe/tests/upstream/gobject/object.c:126` for debug-only
  `/object/constructor/infanticide`, `safe/tests/upstream/gobject/threadtests.c:488`
  for unreliable 32-bit ARM toggle-reference coverage, and
  `safe/tests/upstream/gobject/closure-refcount.c:268` for flaky arm/aarch64
  closure refcount coverage. The verifier output for this refresh also showed
  `/param/implement` and `/object/constructor/infanticide` as skipped under the
  current environment. Current TODO/FIXME markers include
  `safe/tests/upstream/gobject/meson.build:218-221` for the arm/aarch64
  `closure-refcount` timeout multiplier,
  `safe/tests/upstream/gobject/performance/meson.build:4-5` for the GNU
  performance harness `can_fail` marker,
  `safe/tests/upstream/gobject/reference.c:1244` for disabled notify-handler
  resurrection coverage, `safe/tests/upstream/gobject/performance/performance-threaded.c:274`
  for outlier handling in performance results, and
  `safe/tests/upstream/gobject/genmarshal.py:683` for a test diff TODO. The
  manifest runner also overrides `closure-refcount` to 240 seconds and
  `performance` to 180 seconds (`safe/tools/run-meson-manifest.py:45-53`).
- The verification runs for this document passed the GObject link-compat command
  and the 62-row GObject manifest command, but they are not dependent-specific
  application tests. `dependents.json` lists 12 representative GLib dependents:
  `qemu-system-x86`, `network-manager`, `bluez`, `flatpak`, `modemmanager`,
  `fwupd`, `gvfs-daemons`, `gstreamer1.0-tools`, `libvirt-daemon`, `udisks2`,
  `tracker-miner-fs`, and `pocillo-icon-theme`. Several use GObject and GIO, but
  this phase did not run those applications' own test suites.
- The CVE data in `relevant_cves.json` contains 15 rows, all tracked through
  GLib/GIO-facing APIs. `safe/docs/cve-matrix.md` marks those rows implemented,
  but it does not add a GObject-specific CVE row. This GObject documentation pass
  therefore cannot claim that the translated GObject runtime eliminates every
  memory-safety class; it can only claim the current ABI/test/CVE evidence listed
  here.
- Debian package builds skip build-time tests during the ABI shell phase
  (`safe/debian/rules:90-94`), and the default profile omits docs, installed
  tests, GIR, and udeb packages unless `SAFE_FULL_PACKAGE_BUILD` is set
  (`safe/debian/rules:3-6`). Maintainers need to run the standalone verifier
  commands when changing GObject behavior.

The successful verifier evidence used for this refresh was:

```text
cd safe
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/link-compat.py --phase gobject --build-root build-gobject --compile-original-objects --run

cd safe
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/run-meson-manifest.py --build-root build-gobject --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gobject/meson-info/intro-tests.json --manifest tests/manifests/gobject.txt --print-errorlogs
cargo check --workspace
```

Both verifier sequences exited 0 in this workspace. The built GObject shared
object was inspected at `safe/build-gobject/gobject/libgobject-2.0.so.0.8000.0`;
`nm -D --defined-only` found 479 defined dynamic symbols, and `readelf -d`
reported SONAME `libgobject-2.0.so.0`.

## Dependencies and other libraries used

Direct Cargo dependencies for `safe-gobject` come from
`safe/crates/gobject/Cargo.toml:11-13` and resolved as follows in
`cargo tree -p safe-gobject -e normal,build,dev`:

| Dependency | Version | Purpose | Safety note |
| --- | --- | --- | --- |
| `c2rust-asm-casts` | `0.20` requirement, resolved `0.20.0` | Supports casts emitted by C2Rust-translated code. | It exists to preserve C ABI/transliteration behavior and is acceptable only while translated modules remain. |
| `c2rust-bitfields` | `0.20` requirement, resolved `0.20.0` | Provides C-compatible bitfield layout support, used for public layouts such as `GClosure` (`safe/crates/gobject/src/signal/mod.rs:37-57`). | This is layout-sensitive generated-support code rather than a high-level safe abstraction. |

`c2rust-bitfields` pulls the proc-macro crate `c2rust-bitfields-derive 0.20.0`,
which pulls `proc-macro2 1.0.103`, `quote 1.0.40`, `syn 1.0.109`, and
`unicode-ident 1.0.22`. `safe-gobject` also uses `safe/crates/abi-support/src/ffi.rs`
through a source `#[path]` include (`safe/crates/gobject/src/lib.rs:14-15`), but
`abi-support` is not listed as a direct Cargo dependency in
`safe/crates/gobject/Cargo.toml`.

System and runtime libraries observed or configured for GObject are:

- Build-script link lines: `dl`, `ffi`, `m`, `pthread`, `glib-2.0`,
  `gthread-2.0`, and `gmodule-2.0` (`safe/crates/gobject/build.rs:95-101`).
- ABI shell native libraries: `ffi`, `quadmath`, `dl`, `pthread`, and `m`
  (`safe/tools/build-abi-shell.py:69`).
- Dynamic dependencies observed in the built shared object:
  `libglib-2.0.so.0`, `libffi.so.8`, `libgcc_s.so.1`, `libc.so.6`, and
  `ld-linux-x86-64.so.2`.
- Debian build dependencies include Cargo/Rust and packaging tools plus GLib
  native dependencies: `cargo`, `rustc`, `debhelper-compat (= 13)`,
  `dh-sequence-gnome`, `dh-sequence-python3`, `dpkg-dev`, `meson`, `patchelf`,
  `pkgconf`, `python3`, `python3-packaging`, `libffi-dev`, `libmount-dev`,
  `libpcre2-dev`, `libselinux1-dev`, `libdbus-1-dev`, `libelf-dev`,
  `linux-libc-dev`, `zlib1g-dev`, and documentation/test tools gated by build
  profiles (`safe/debian/control:7-45`). The `libglib2.0-dev` package also
  depends on `libffi-dev`, `libmount-dev`, `libpcre2-dev`, `libselinux1-dev`,
  `pkgconf`, Python or QEMU, and `zlib1g-dev` (`safe/debian/control:138-155`).

The unsafe-heavy dependencies and generated support are accepted here because
the current phase prioritizes ABI-compatible replacement of a C library. They
should be treated as migration scaffolding, not as proof that the GObject runtime
is already idiomatic safe Rust. No crate in this workspace uses `#![forbid(unsafe_code)]`
for `safe-gobject`.

## How this document was produced

Commands and files consulted for this refresh:

```bash
cd safe
cargo tree -p safe-gobject -e normal,build,dev
cargo geiger -p safe-gobject --all-features --output-format GitHubMarkdown  # failed: no such cargo command
rg -n "\bunsafe\b" crates/gobject crates/abi-support --glob '*.rs'
rg -n "extern \"C\"|GObject|GType|GValue|GSignal|closure|TODO|FIXME|SKIP|g_test_skip|can_fail|allow\(|translated" crates/gobject tests/upstream/gobject tests/manifests/gobject.txt
rg -n "\b(cbindgen|bindgen)\b" Cargo.toml Cargo.lock crates tools debian meson.build
cargo check --workspace
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/link-compat.py --phase gobject --build-root build-gobject --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-gobject --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gobject/meson-info/intro-tests.json --manifest tests/manifests/gobject.txt --print-errorlogs
nm -D --defined-only build-gobject/gobject/libgobject-2.0.so.0.8000.0
readelf -d build-gobject/gobject/libgobject-2.0.so.0.8000.0
wc -l tests/manifests/gobject.txt abi/version-scripts/libgobject.map abi/link-compat/gobject.json abi/layouts/gobject.json abi/layout-manifests/gobject.json
```

Primary files read include `.plan/phases/04-gobject-runtime.md`,
`safe/Cargo.toml`, `safe/crates/gobject/Cargo.toml`,
`safe/crates/gobject/src/lib.rs`, `safe/crates/gobject/src/object/mod.rs`,
`safe/crates/gobject/src/signal/mod.rs`, `safe/crates/gobject/src/type_system/mod.rs`,
`safe/crates/gobject/src/value/mod.rs`, `safe/crates/gobject/src/tools/mod.rs`,
`safe/crates/gobject/src/translated/compat.rs`, the translated GObject files
(`crates/gobject/src/translated/original/gobject/gatomicarray.rs`, `crates/gobject/src/translated/original/gobject/gbinding.rs`, `crates/gobject/src/translated/original/gobject/gbindinggroup.rs`, `crates/gobject/src/translated/original/gobject/gboxed.rs`, `crates/gobject/src/translated/original/gobject/gclosure.rs`, `crates/gobject/src/translated/original/gobject/genums.rs`, `crates/gobject/src/translated/original/gobject/gmarshal.rs`, `crates/gobject/src/translated/original/gobject/gobject.rs`, `crates/gobject/src/translated/original/gobject/gparam.rs`, `crates/gobject/src/translated/original/gobject/gparamspecs.rs`, `crates/gobject/src/translated/original/gobject/gsignal.rs`, `crates/gobject/src/translated/original/gobject/gsignalgroup.rs`, `crates/gobject/src/translated/original/gobject/gsourceclosure.rs`, `crates/gobject/src/translated/original/gobject/gtype.rs`, `crates/gobject/src/translated/original/gobject/gtypemodule.rs`, `crates/gobject/src/translated/original/gobject/gtypeplugin.rs`, `crates/gobject/src/translated/original/gobject/gvalue.rs`, `crates/gobject/src/translated/original/gobject/gvaluearray.rs`, `crates/gobject/src/translated/original/gobject/gvaluetransform.rs`, `crates/gobject/src/translated/original/gobject/gvaluetypes.rs`, `crates/gobject/src/translated/original/gobject/mod.rs`), `safe/crates/gobject/build.rs`,
`safe/crates/abi-support/src/ffi.rs`, `safe/crates/abi-support/src/bin/layout-probe.rs`,
`safe/tools/build-abi-shell.py`, `safe/tools/run-meson-manifest.py`,
`safe/tests/upstream/gobject/meson.build`,
`safe/tests/upstream/gobject/performance/meson.build`,
`safe/tests/manifests/gobject.txt`, `safe/docs/cve-matrix.md`,
`dependents.json`, `relevant_cves.json`, and `safe/debian/control` /
`safe/debian/rules`.

After writing this report, the line/path sanity checks were rerun against the
current files so that every cited path exists, the dependency list matches
`safe/crates/gobject/Cargo.toml`, and the unsafe inventory matches the current
`rg -n '\bunsafe\b' crates/gobject crates/abi-support --glob '*.rs'` scan modulo
comments and strings.
