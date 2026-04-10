#define G_SETTINGS_ENABLE_BACKEND
#include <gio/gio.h>
#include <gio/gsettingsbackend.h>
#include <glib/gstdio.h>
#include <sys/stat.h>
#include <unistd.h>

static int
fail (const char *message)
{
  g_printerr ("%s\n", message);
  return 1;
}

int
main (void)
{
  static const char schema_xml[] =
      "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
      "<schemalist>\n"
      "  <schema id=\"org.safe.cve.keyfile\" path=\"/org/safe/cve/keyfile/\">\n"
      "    <key name=\"greeting\" type=\"s\">\n"
      "      <default>'hello'</default>\n"
      "    </key>\n"
      "  </schema>\n"
      "</schemalist>\n";
  g_autoptr(GError) error = NULL;
  g_autofree gchar *tmp_dir = NULL;
  g_autofree gchar *schema_dir = NULL;
  g_autofree gchar *schema_path = NULL;
  g_autofree gchar *store_dir = NULL;
  g_autofree gchar *store_path = NULL;
  g_autofree gchar *stdout_data = NULL;
  g_autofree gchar *stderr_data = NULL;
  g_autoptr(GSettingsSchemaSource) schema_source = NULL;
  g_autoptr(GSettingsSchema) schema = NULL;
  GSettingsBackend *backend;
  g_autoptr(GSettings) settings = NULL;
  gint wait_status = 0;
  struct stat st;
  mode_t original_umask;
  gchar *compile_argv[] = {
    (gchar *) "glib-compile-schemas",
    (gchar *) "--strict",
    (gchar *) "--targetdir",
    NULL,
    NULL,
    NULL,
  };

  tmp_dir = g_dir_make_tmp ("safe-keyfile-settings-XXXXXX", &error);
  if (tmp_dir == NULL)
    return fail (error->message);

  schema_dir = g_build_filename (tmp_dir, "schemas", NULL);
  schema_path = g_build_filename (schema_dir, "org.safe.cve.keyfile.gschema.xml", NULL);
  store_dir = g_build_filename (tmp_dir, "keyfiles", NULL);
  store_path = g_build_filename (store_dir, "settings.ini", NULL);

  if (g_mkdir_with_parents (schema_dir, 0700) != 0)
    return fail ("failed to create schema directory");
  if (!g_file_set_contents (schema_path, schema_xml, -1, &error))
    return fail (error->message);

  compile_argv[3] = schema_dir;
  compile_argv[4] = schema_dir;
  if (!g_spawn_sync (NULL,
                     compile_argv,
                     NULL,
                     G_SPAWN_SEARCH_PATH,
                     NULL,
                     NULL,
                     &stdout_data,
                     &stderr_data,
                     &wait_status,
                     &error))
    return fail (error->message);
  if (!g_spawn_check_wait_status (wait_status, &error))
    return fail (error->message);

  schema_source = g_settings_schema_source_new_from_directory (schema_dir, NULL, FALSE, &error);
  if (schema_source == NULL)
    return fail (error->message);
  schema = g_settings_schema_source_lookup (schema_source, "org.safe.cve.keyfile", FALSE);
  if (schema == NULL)
    return fail ("failed to load compiled schema");

  original_umask = umask (0);
  backend = g_keyfile_settings_backend_new (store_path, "/org/safe/cve/keyfile/", "root");
  settings = g_settings_new_full (schema, backend, NULL);
  g_object_unref (backend);
  if (settings == NULL)
    return fail ("failed to create GSettings instance");

  if (!g_settings_set_string (settings, "greeting", "safe"))
    return fail ("failed to write keyfile-backed setting");
  g_settings_sync ();
  umask (original_umask);

  if (stat (store_dir, &st) != 0)
    return fail ("failed to stat keyfile directory");
  if ((st.st_mode & 0777) != 0700)
    return fail ("keyfile directory permissions drifted from 0700");

  if (stat (store_path, &st) != 0)
    return fail ("failed to stat keyfile backend file");
  if ((st.st_mode & 0777) != 0600)
    return fail ("keyfile backend permissions drifted from 0600");

  g_remove (store_path);
  g_rmdir (store_dir);
  g_remove (schema_path);
  g_remove (g_build_filename (schema_dir, "gschemas.compiled", NULL));
  g_rmdir (schema_dir);
  g_rmdir (tmp_dir);
  return 0;
}
