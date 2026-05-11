#include <glib.h>

int
main (void)
{
#if SIZE_WIDTH <= UINT_WIDTH
  return 77;
#else
  GByteArray *array = g_byte_array_new_take (NULL, (gsize) G_MAXUINT + 1);
  return array == NULL ? 0 : 1;
#endif
}
