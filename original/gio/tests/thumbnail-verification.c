/* GLib testing framework examples and tests
 *
 * Copyright (C) 2013 Collabora, Ltd.
 *
 * SPDX-License-Identifier: LicenseRef-old-glib-tests
 *
 * This work is provided "as is"; redistribution and modification
 * in whole or in part, in any medium, physical or electronic is
 * permitted without restriction.
 *
 * This work is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *
 * In no event shall the authors or contributors be liable for any
 * direct, indirect, incidental, special, exemplary, or consequential
 * damages (including, but not limited to, procurement of substitute
 * goods or services; loss of use, data, or profits; or business
 * interruption) however caused and on any theory of liability, whether
 * in contract, strict liability, or tort (including negligence or
 * otherwise) arising in any way out of the use of this software, even
 * if advised of the possibility of such damage.
 *
 * Author: Philip Withnall <philip.withnall@collabora.co.uk>
 */

#include <gio/gio.h>
#include <glib/gstdio.h>

static void
test_thumbnail_attributes (void)
{
  GError *error = NULL;
  gchar *path = NULL;
  GFile *file = NULL;
  GFileInfo *info = NULL;
  gint fd;

  fd = g_file_open_tmp ("gio-thumbnail-verification-XXXXXX", &path, &error);
  g_assert_cmpint (fd, !=, -1);
  g_assert_no_error (error);
  g_assert_true (g_close (fd, &error));
  g_assert_no_error (error);

  file = g_file_new_for_path (path);
  info = g_file_query_info (file,
                            G_FILE_ATTRIBUTE_THUMBNAIL_PATH ","
                            G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID ","
                            G_FILE_ATTRIBUTE_THUMBNAILING_FAILED,
                            G_FILE_QUERY_INFO_NONE,
                            NULL,
                            &error);
  g_assert_no_error (error);
  g_assert_nonnull (info);

  g_assert_null (g_file_info_get_attribute_byte_string (info,
                                                        G_FILE_ATTRIBUTE_THUMBNAIL_PATH));
  g_assert_false (g_file_info_get_attribute_boolean (info,
                                                     G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID));
  g_assert_false (g_file_info_get_attribute_boolean (info,
                                                     G_FILE_ATTRIBUTE_THUMBNAILING_FAILED));

  g_object_unref (info);
  g_object_unref (file);
  g_assert_cmpint (g_remove (path), ==, 0);
  g_free (path);
}

int
main (int   argc,
      char *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/png-thumbs/attributes", test_thumbnail_attributes);

  return g_test_run ();
}
