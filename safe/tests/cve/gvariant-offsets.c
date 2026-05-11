#include <glib.h>

int
main (void)
{
  const GVariantType *type = G_VARIANT_TYPE ("(sss)");
  const guint8 data[] = { 0xaa };
  GVariant *variant = NULL;
  GVariant *normal = NULL;

  variant = g_variant_new_from_data (type, data, sizeof (data), FALSE, NULL, NULL);
  if (variant == NULL)
    return 1;

  if (g_variant_is_normal_form (variant))
    return 1;

  normal = g_variant_get_normal_form (variant);
  if (normal == NULL)
    return 1;

  g_variant_unref (normal);
  g_variant_unref (variant);
  return 0;
}
