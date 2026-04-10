#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InstalledRepository {
    pub namespace: &'static str,
    pub version: &'static str,
    pub gir: &'static str,
    pub typelib: &'static str,
}

pub const GI_REPOSITORY: InstalledRepository = InstalledRepository {
    namespace: "GIRepository",
    version: "3.0",
    gir: "GIRepository-3.0.gir",
    typelib: "GIRepository-3.0.typelib",
};

pub const GLIB_REPOSITORIES: &[InstalledRepository] = &[
    InstalledRepository {
        namespace: "GLib",
        version: "2.0",
        gir: "GLib-2.0.gir",
        typelib: "GLib-2.0.typelib",
    },
    InstalledRepository {
        namespace: "GLibUnix",
        version: "2.0",
        gir: "GLibUnix-2.0.gir",
        typelib: "GLibUnix-2.0.typelib",
    },
    InstalledRepository {
        namespace: "GModule",
        version: "2.0",
        gir: "GModule-2.0.gir",
        typelib: "GModule-2.0.typelib",
    },
    InstalledRepository {
        namespace: "GObject",
        version: "2.0",
        gir: "GObject-2.0.gir",
        typelib: "GObject-2.0.typelib",
    },
    InstalledRepository {
        namespace: "Gio",
        version: "2.0",
        gir: "Gio-2.0.gir",
        typelib: "Gio-2.0.typelib",
    },
    InstalledRepository {
        namespace: "GioUnix",
        version: "2.0",
        gir: "GioUnix-2.0.gir",
        typelib: "GioUnix-2.0.typelib",
    },
];

pub const ALL_REPOSITORIES: &[InstalledRepository] = &[
    GI_REPOSITORY,
    GLIB_REPOSITORIES[0],
    GLIB_REPOSITORIES[1],
    GLIB_REPOSITORIES[2],
    GLIB_REPOSITORIES[3],
    GLIB_REPOSITORIES[4],
    GLIB_REPOSITORIES[5],
];
