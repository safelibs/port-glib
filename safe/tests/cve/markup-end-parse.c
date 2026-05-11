#include <glib.h>

static const GMarkupParser parser = {
  NULL,
  NULL,
  NULL,
  NULL,
  NULL
};

int
main (void)
{
  GMarkupParseContext *context;
  GError *error = NULL;

  context = g_markup_parse_context_new (&parser, 0, NULL, NULL);
  if (context == NULL)
    return 1;

  if (!g_markup_parse_context_parse (context, "<root>", -1, &error))
    return 1;

  if (g_markup_parse_context_end_parse (context, &error))
    return 1;

  if (error == NULL)
    return 1;

  g_error_free (error);
  g_markup_parse_context_free (context);
  return 0;
}
