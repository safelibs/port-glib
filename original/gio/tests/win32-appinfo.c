/* GLib testing framework examples and tests
 * Copyright (C) 2019 Руслан Ижбулатов <lrn1986@gmail.com>
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
 */

#include <glib/glib.h>
#include <gio/gio.h>

#ifdef G_OS_WIN32

typedef struct
{
  const gchar *commandline;
  const gchar *application_name;
  const gchar *expected_executable;
} Win32CommandlineCase;

static void
test_create_from_commandline (void)
{
  static const Win32CommandlineCase cases[] = {
    { "cmd.exe /c exit 0", "Command", "cmd.exe" },
    { "\"some path with spaces\\app.exe\" /flag", "Spaces", "some path with spaces\\app.exe" },
    { "rundll32.exe shell32.dll,ShellExec_RunDLL", "RunDLL", "shell32.dll" },
  };
  gsize i;

  for (i = 0; i < G_N_ELEMENTS (cases); i++)
    {
      GAppInfo *appinfo;
      GAppInfo *duplicate;
      GError *error = NULL;

      appinfo = g_app_info_create_from_commandline (cases[i].commandline,
                                                    cases[i].application_name,
                                                    G_APP_INFO_CREATE_NONE,
                                                    &error);
      g_assert_no_error (error);
      g_assert_nonnull (appinfo);

      g_assert_cmpstr (g_app_info_get_name (appinfo), ==, cases[i].application_name);
      g_assert_cmpstr (g_app_info_get_display_name (appinfo), ==, cases[i].application_name);
      g_assert_cmpstr (g_app_info_get_commandline (appinfo), ==, cases[i].commandline);
      g_assert_cmpstr (g_app_info_get_executable (appinfo), ==, cases[i].expected_executable);

      duplicate = g_app_info_dup (appinfo);
      g_assert_nonnull (duplicate);
      g_assert_cmpstr (g_app_info_get_commandline (duplicate), ==, cases[i].commandline);
      g_assert_cmpstr (g_app_info_get_executable (duplicate), ==, cases[i].expected_executable);

      g_object_unref (duplicate);
      g_object_unref (appinfo);
    }
}

static void
test_invalid_utf8_commandline (void)
{
  const gchar invalid_commandline[] = "cmd.exe \x80";
  GError *error = NULL;
  GAppInfo *appinfo;

  appinfo = g_app_info_create_from_commandline (invalid_commandline,
                                                "Broken",
                                                G_APP_INFO_CREATE_NONE,
                                                &error);
  g_assert_null (appinfo);
  g_assert_no_error (error);
}

#else

static void
test_win32_public_api_only (void)
{
  g_test_skip ("Windows-only public GAppInfo coverage");
}

#endif

int
main (int   argc,
      char *argv[])
{
  g_test_init (&argc, &argv, NULL);

#ifdef G_OS_WIN32
  g_test_add_func ("/appinfo/create-from-commandline", test_create_from_commandline);
  g_test_add_func ("/appinfo/invalid-utf8-commandline", test_invalid_utf8_commandline);
#else
  g_test_add_func ("/appinfo/win32/public-api-only", test_win32_public_api_only);
#endif

  return g_test_run ();
}
