#include <glib.h>
#include <locale.h>

static int
require_utf8_locale (void)
{
  return setlocale (LC_ALL, "C.UTF-8") != NULL ||
         setlocale (LC_ALL, "en_US.UTF-8") != NULL;
}

int
main (void)
{
  const gchar *charset = NULL;
  gchar *converted = NULL;
  GError *error = NULL;

  if (!require_utf8_locale ())
    return 77;

  g_setenv ("SAFE_GLIB_TEST_FORCE_PRIVILEGED", "1", TRUE);
  g_setenv ("CHARSET", "US-ASCII", TRUE);
  g_setenv ("G_FILENAME_ENCODING", "US-ASCII", TRUE);

  if (!g_get_charset (&charset))
    return 1;
  if (g_strcmp0 (charset, "UTF-8") != 0)
    return 1;

  converted = g_locale_from_utf8 ("\303\251", -1, NULL, NULL, &error);
  if (error != NULL || converted == NULL)
    return 1;
  g_free (converted);

  converted = g_filename_from_utf8 ("\303\251", -1, NULL, NULL, &error);
  if (error != NULL || converted == NULL)
    return 1;
  g_free (converted);

  return 0;
}
