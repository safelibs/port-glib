#include <glib.h>

static guint
manual_str_hash (const gchar *value)
{
  const signed char *p;
  guint32 hash = 5381;

  for (p = (const signed char *) value; *p != '\0'; p++)
    hash = (hash << 5) + hash + *p;

  return hash;
}

int
main (void)
{
  const gchar *samples[] = {
    "",
    "hello",
    "GLib advanced hash regression",
    "\377",
    NULL,
  };
  GHashTable *table;
  guint i;

  for (i = 0; samples[i] != NULL; i++)
    {
      if (g_str_hash (samples[i]) != manual_str_hash (samples[i]))
        return 1;
    }

  table = g_hash_table_new (g_str_hash, g_str_equal);
  if (table == NULL)
    return 1;

  g_hash_table_insert (table, "alpha", "one");
  g_hash_table_insert (table, "beta", "two");

  if (!g_hash_table_contains (table, "alpha"))
    return 1;
  if (g_strcmp0 (g_hash_table_lookup (table, "beta"), "two") != 0)
    return 1;

  g_hash_table_destroy (table);
  return 0;
}
