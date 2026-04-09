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

#include <utime.h>

typedef struct
{
  const gchar *thumbnail_name;
  const gchar *source_name;
  guint64 mtime;
  gsize size;
  gboolean create_source;
  gboolean expected_validity;
} ThumbnailCase;

typedef struct
{
  gchar *source_path;
  gchar *backup_path;
} SourceGuard;

static const ThumbnailCase thumbnail_cases[] =
{
  { "valid.png", "valid.png", 1382429848, 93654, TRUE, TRUE },
  { "valid-no-size.png", "valid-no-size.png", 1382429848, 93633, TRUE, TRUE },
  { "missing.png", "missing.png", 123456789, 12345, FALSE, FALSE },
  { "no-text-data.png", "no-text-data.png", 123, 26378, TRUE, FALSE },
  { "uri-mismatch.png", "uri-mismatch.png", 1382429848, 93654, TRUE, FALSE },
  { "valid.png", "valid.png", 123, 93654, TRUE, FALSE },
  { "valid.png", "valid.png", 1382429848, 123, TRUE, FALSE },
  { "mtime-zero.png", "mtime-zero.png", 0, 93621, TRUE, TRUE },
  { "valid.png", "valid.png", 9848, 93654, TRUE, FALSE },
  { "bad-header.png", "bad-header.png", 1382429848, 93654, TRUE, FALSE },
  { "header-only.png", "header-only.png", 1382429848, 8, TRUE, FALSE },
  { "header-and-chunk-size.png", "header-and-chunk-size.png", 1382429848, 20, TRUE, FALSE },
  { "huge-chunk-size.png", "huge-chunk-size.png", 1382429848, 93654, TRUE, FALSE },
  { "empty-key.png", "empty-key.png", 1382429848, 93654, TRUE, FALSE },
  { "overlong-value.png", "overlong-value.png", 1382429848, 93660, TRUE, FALSE },
};

static gchar *
build_source_path (const gchar *source_name)
{
  return g_build_filename ("/tmp", source_name, NULL);
}

static gboolean
source_guard_init (SourceGuard  *guard,
                   const gchar  *source_name)
{
  guard->source_path = build_source_path (source_name);
  guard->backup_path = NULL;

  if (g_file_test (guard->source_path, G_FILE_TEST_EXISTS))
    {
      guard->backup_path = g_strdup_printf ("%s.glib-backup-%u",
                                            guard->source_path,
                                            g_random_int ());
      if (g_rename (guard->source_path, guard->backup_path) != 0)
        {
          g_test_skip ("Unable to move a pre-existing /tmp thumbnail fixture aside.");
          g_clear_pointer (&guard->source_path, g_free);
          g_clear_pointer (&guard->backup_path, g_free);
          return FALSE;
        }
    }

  return TRUE;
}

static void
source_guard_clear (SourceGuard *guard)
{
  if (guard->source_path != NULL &&
      g_file_test (guard->source_path, G_FILE_TEST_EXISTS))
    g_assert_cmpint (g_remove (guard->source_path), ==, 0);

  if (guard->backup_path != NULL)
    g_assert_cmpint (g_rename (guard->backup_path, guard->source_path), ==, 0);

  g_clear_pointer (&guard->backup_path, g_free);
  g_clear_pointer (&guard->source_path, g_free);
}

static void
create_source_file (const gchar *path,
                    gsize        size,
                    guint64      mtime)
{
  GError *error = NULL;
  gchar *contents;
  struct utimbuf ut;

  contents = g_malloc0 (MAX (size, 1));
  g_assert_true (g_file_set_contents (path, contents, size, &error));
  g_assert_no_error (error);
  g_free (contents);

  ut.actime = (time_t) mtime;
  ut.modtime = (time_t) mtime;
  g_assert_cmpint (g_utime (path, &ut), ==, 0);
}

static gchar *
get_thumbnail_basename_for_path (const gchar *source_path)
{
  GChecksum *checksum;
  gchar *uri;
  gchar *basename;

  uri = g_filename_to_uri (source_path, NULL, NULL);
  g_assert_nonnull (uri);

  checksum = g_checksum_new (G_CHECKSUM_MD5);
  g_checksum_update (checksum, (const guchar *) uri, strlen (uri));
  basename = g_strconcat (g_checksum_get_string (checksum), ".png", NULL);

  g_checksum_free (checksum);
  g_free (uri);

  return basename;
}

static GFile *
get_cached_thumbnail_file (const gchar *source_path)
{
  GFile *thumbnail;
  gchar *basename;

  basename = get_thumbnail_basename_for_path (source_path);
  thumbnail = g_file_new_build_filename (g_get_user_cache_dir (),
                                         "thumbnails",
                                         "normal",
                                         basename,
                                         NULL);
  g_free (basename);

  return thumbnail;
}

static GFile *
stage_thumbnail_fixture (const gchar *thumbnail_name,
                         const gchar *source_path)
{
  GFile *fixture;
  GFile *thumbnail;
  GFile *thumbnail_dir;
  GError *error = NULL;
  const gchar *fixture_path;

  fixture_path = g_test_get_filename (G_TEST_DIST, "thumbnails", thumbnail_name, NULL);
  if (!g_file_test (fixture_path, G_FILE_TEST_IS_REGULAR))
    return NULL;

  fixture = g_file_new_for_path (fixture_path);
  thumbnail = get_cached_thumbnail_file (source_path);
  thumbnail_dir = g_file_get_parent (thumbnail);

  g_file_make_directory_with_parents (thumbnail_dir, NULL, &error);
  if (error != NULL)
    {
      g_assert_error (error, G_IO_ERROR, G_IO_ERROR_EXISTS);
      g_clear_error (&error);
    }

  g_file_copy (fixture, thumbnail, G_FILE_COPY_OVERWRITE, NULL, NULL, NULL, &error);
  g_assert_no_error (error);

  g_object_unref (thumbnail_dir);
  g_object_unref (fixture);

  return thumbnail;
}

static void
test_thumbnail_verification_public (gconstpointer data)
{
  const ThumbnailCase *test = data;
  SourceGuard guard = { 0 };
  GFile *source = NULL;
  GFile *thumbnail = NULL;
  GFileInfo *info = NULL;
  GError *error = NULL;
  const gchar *thumbnail_path;

#ifdef G_OS_WIN32
  g_test_skip ("The fixture thumbnails encode Unix file:///tmp URIs.");
  return;
#endif

  if (!source_guard_init (&guard, test->source_name))
    {
      source_guard_clear (&guard);
      return;
    }

  if (test->create_source)
    create_source_file (guard.source_path, test->size, test->mtime);

  thumbnail = stage_thumbnail_fixture (test->thumbnail_name, guard.source_path);
  source = g_file_new_for_path (guard.source_path);
  info = g_file_query_info (source,
                            G_FILE_ATTRIBUTE_THUMBNAIL_PATH ","
                            G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID ","
                            G_FILE_ATTRIBUTE_THUMBNAILING_FAILED ","
                            G_FILE_ATTRIBUTE_THUMBNAIL_PATH_NORMAL ","
                            G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_NORMAL,
                            G_FILE_QUERY_INFO_NONE,
                            NULL,
                            &error);

  if (!test->create_source)
    {
      g_assert_error (error, G_IO_ERROR, G_IO_ERROR_NOT_FOUND);
      g_assert_null (info);
      g_clear_error (&error);
      g_clear_object (&thumbnail);
      g_clear_object (&source);
      source_guard_clear (&guard);
      return;
    }

  g_assert_no_error (error);
  g_assert_nonnull (info);
  g_assert_nonnull (thumbnail);

  thumbnail_path = g_file_peek_path (thumbnail);
  g_assert_cmpstr (g_file_info_get_attribute_byte_string (info,
                                                          G_FILE_ATTRIBUTE_THUMBNAIL_PATH),
                   ==,
                   thumbnail_path);
  g_assert_cmpstr (g_file_info_get_attribute_byte_string (info,
                                                          G_FILE_ATTRIBUTE_THUMBNAIL_PATH_NORMAL),
                   ==,
                   thumbnail_path);
  g_assert_true (g_file_info_get_attribute_boolean (info,
                                                    G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID) ==
                 test->expected_validity);
  g_assert_true (g_file_info_get_attribute_boolean (info,
                                                    G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_NORMAL) ==
                 test->expected_validity);
  g_assert_false (g_file_info_has_attribute (info,
                                             G_FILE_ATTRIBUTE_THUMBNAILING_FAILED));

  g_object_unref (info);
  g_object_unref (thumbnail);
  g_object_unref (source);
  source_guard_clear (&guard);
}

int
main (int   argc,
      char *argv[])
{
  gsize i;

  g_test_init (&argc, &argv, G_TEST_OPTION_ISOLATE_DIRS, NULL);

  for (i = 0; i < G_N_ELEMENTS (thumbnail_cases); i++)
    {
      gchar *test_path;

      test_path = g_strdup_printf ("/png-thumbs/public/%02" G_GSIZE_FORMAT "-%s",
                                   i,
                                   thumbnail_cases[i].thumbnail_name);
      g_test_add_data_func (test_path, &thumbnail_cases[i], test_thumbnail_verification_public);
      g_free (test_path);
    }

  return g_test_run ();
}
