/* constructor.c - Test for constructors
 *
 * Copyright © 2023 Luca Bacci
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#include <glib.h>
#include <stdlib.h>

#ifndef _WIN32
#include <dlfcn.h>
#else
#include <windows.h>
#endif

#if defined (__GNUC__) && (__GNUC__ > 2 || (__GNUC__ == 2 && __GNUC_MINOR__ >= 7))
#define TEST_HAS_CONSTRUCTORS 1
#define TEST_HAS_DESTRUCTORS 1
#define TEST_DEFINE_CONSTRUCTOR(_func) static void __attribute__ ((constructor)) _func (void);
#define TEST_DEFINE_DESTRUCTOR(_func) static void __attribute__ ((destructor)) _func (void);
#elif defined (_MSC_VER)
#define TEST_HAS_CONSTRUCTORS 1
#define TEST_HAS_DESTRUCTORS 1

#ifdef _M_IX86
#define TEST_MSVC_SYMBOL_PREFIX "_"
#else
#define TEST_MSVC_SYMBOL_PREFIX ""
#endif

#define TEST_DEFINE_CONSTRUCTOR(_func) \
  static void _func (void); \
  static int _func##_wrapper (void) { _func (); return 0; } \
  __pragma (comment (linker, "/include:" TEST_MSVC_SYMBOL_PREFIX #_func "_wrapper")) \
  __pragma (section (".CRT$XCU", read)) \
  __declspec (allocate (".CRT$XCU")) static int (*_array_##_func)(void) = _func##_wrapper;

#define TEST_DEFINE_DESTRUCTOR(_func) \
  static void _func (void); \
  static int _func##_constructor (void) { atexit (_func); return 0; } \
  __pragma (comment (linker, "/include:" TEST_MSVC_SYMBOL_PREFIX #_func "_constructor")) \
  __pragma (section (".CRT$XCU", read)) \
  __declspec (allocate (".CRT$XCU")) static int (*_array_##_func)(void) = _func##_constructor;
#else
#define TEST_HAS_CONSTRUCTORS 0
#define TEST_HAS_DESTRUCTORS 0
#endif

#if defined(_WIN32)
  #define MODULE_IMPORT \
    __declspec (dllimport)
#else
  #define MODULE_IMPORT
#endif

MODULE_IMPORT
void string_add_exclusive (const char *string);

MODULE_IMPORT
void string_check (const char *string);

MODULE_IMPORT
int string_find (const char *string);

#if defined (_WIN32)

#ifdef __cplusplus
#define TEST_G_EXTERN_CONST extern const
#else
#define TEST_G_EXTERN_CONST const
#endif

#ifdef _MSC_VER
#define G_HAS_TLS_CALLBACKS 1
#define G_DEFINE_TLS_CALLBACK(func) \
__pragma (section (".CRT$XLCE", long, read))                                \
                                                                            \
static void NTAPI func (PVOID, DWORD, PVOID);                               \
                                                                            \
G_BEGIN_DECLS                                                               \
__declspec (allocate (".CRT$XLCE"))                                         \
TEST_G_EXTERN_CONST PIMAGE_TLS_CALLBACK _ptr_##func = func;                 \
G_END_DECLS                                                                 \
                                                                            \
__pragma (comment (linker, "/INCLUDE:" TEST_MSVC_SYMBOL_PREFIX "_tls_used"))   \
__pragma (comment (linker, "/INCLUDE:" TEST_MSVC_SYMBOL_PREFIX "_ptr_" #func))
#else
#define G_HAS_TLS_CALLBACKS 1
#define G_DEFINE_TLS_CALLBACK(func) \
static void NTAPI func (PVOID, DWORD, PVOID);          \
                                                       \
G_BEGIN_DECLS                                          \
__attribute__ ((section (".CRT$XLCE")))                \
TEST_G_EXTERN_CONST PIMAGE_TLS_CALLBACK _ptr_##func = func; \
G_END_DECLS
#endif

#endif

#if TEST_HAS_CONSTRUCTORS

TEST_DEFINE_CONSTRUCTOR (ctor)

static void
ctor (void)
{
  string_add_exclusive (G_STRINGIFY (PREFIX) "_" "ctor");
}

TEST_DEFINE_DESTRUCTOR (dtor)

static void
dtor (void)
{
  string_add_exclusive (G_STRINGIFY (PREFIX) "_" "dtor");

  if (string_find ("app_dtor") && string_find ("lib_dtor"))
    {
      /* All destructors were invoked, this is the last.
       * Call _Exit (EXIT_SUCCESS) to exit immediately
       * with a success code */
      _Exit (EXIT_SUCCESS);
    }
}

#endif /* TEST_HAS_CONSTRUCTORS */


#if defined (_WIN32) && defined (G_HAS_TLS_CALLBACKS)

extern IMAGE_DOS_HEADER __ImageBase;

static inline HMODULE
this_module (void)
{
  return (HMODULE) &__ImageBase;
}

G_DEFINE_TLS_CALLBACK (tls_callback)

static void NTAPI
tls_callback (PVOID  hInstance,
              DWORD  dwReason,
              LPVOID lpvReserved)
{
  /* The HINSTANCE we get must match the address of __ImageBase */
  g_assert_true (hInstance == this_module ());

#ifdef BUILD_TEST_EXECUTABLE
  /* Yes, we can call GetModuleHandle (NULL) with the loader lock */
  g_assert_true (hInstance == GetModuleHandle (NULL));
#endif

  switch (dwReason)
    {
    case DLL_PROCESS_ATTACH:
      {
#ifndef BUILD_TEST_EXECUTABLE
        /* the library is explicitly loaded */
        g_assert_null (lpvReserved);
#endif
        string_add_exclusive (G_STRINGIFY (PREFIX) "_" "tlscb_process_attach");
      }
      break;

    case DLL_THREAD_ATTACH:
      break;

    case DLL_THREAD_DETACH:
      break;

    case DLL_PROCESS_DETACH:
      {
#ifndef BUILD_TEST_EXECUTABLE
        /* the library is explicitly unloaded */
        g_assert_null (lpvReserved);
#endif
        string_add_exclusive (G_STRINGIFY (PREFIX) "_" "tlscb_process_detach");
      }
      break;

    default:
      g_assert_not_reached ();
      break;
    }
}

#endif /* _WIN32 && G_HAS_TLS_CALLBACKS */

#ifdef BUILD_TEST_EXECUTABLE

void *library;

static void
load_library (const char *path)
{
#ifndef _WIN32
  library = dlopen (path, RTLD_NOW);
  if (!library)
    {
      g_error ("%s (%s) failed: %s", "dlopen", path, dlerror ());
    }
#else
  wchar_t *path_utf16 = g_utf8_to_utf16 (path, -1, NULL, NULL, NULL);
  g_assert_nonnull (path_utf16);

  library = LoadLibraryW (path_utf16);
  if (!library)
    {
      g_error ("%s (%s) failed with error code %u",
               "FreeLibrary", path, (unsigned int) GetLastError ());
    }

  g_free (path_utf16);
#endif
}

static void
unload_library (void)
{
#ifndef _WIN32
  if (dlclose (library) != 0)
    {
      g_error ("%s failed: %s", "dlclose", dlerror ());
    }
#else
  if (!FreeLibrary (library))
    {
      g_error ("%s failed with error code %u",
               "FreeLibrary", (unsigned int) GetLastError ());
    }
#endif
}

static void
test_app (void)
{
#if TEST_HAS_CONSTRUCTORS
  string_check ("app_" "ctor");
#endif
#if defined (_WIN32) && defined (G_HAS_TLS_CALLBACKS)
  string_check ("app_" "tlscb_process_attach");
#endif
}

static void
test_lib (gconstpointer data)
{
  const char *library_path = (const char*) data;

  /* Constructors */
  load_library (library_path);

#if TEST_HAS_CONSTRUCTORS
  string_check ("lib_" "ctor");
#endif
#if defined (_WIN32) && defined (G_HAS_TLS_CALLBACKS)
  string_check ("lib_" "tlscb_process_attach");
#endif

  /* Destructors */
  unload_library ();

#if TEST_HAS_DESTRUCTORS
  /* Destructors in dynamically-loaded libraries do not
   * necessarily run on dlclose. On some systems dlclose
   * is effectively a no-op (e.g with the Musl LibC) and
   * destructors run at program exit */
  g_test_message ("Destructors run on module unload: %s\n",
                  string_find ("lib_" "dtor") ? "yes" : "no");
#endif
#if defined (_WIN32) && defined (G_HAS_TLS_CALLBACKS)
  string_check ("lib_" "tlscb_process_detach");
#endif
}

int
main (int argc, char *argv[])
{

  const char *libname = G_STRINGIFY (LIB_NAME);
  const char *builddir;
  char *path;
  int ret;

  g_assert_nonnull ((builddir = g_getenv ("G_TEST_BUILDDIR")));

  path = g_build_filename (builddir, libname, NULL);

  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/constructor/application", test_app);
  g_test_add_data_func ("/constructor/library", path, test_lib);

  ret = g_test_run ();
  g_assert_cmpint (ret, ==, 0);

  g_free (path);

  /* Return EXIT_FAILURE from main. The last destructor will
   * call _Exit (EXIT_SUCCESS) if everything went well. This
   * is a way to test that destructors get invoked */
  return EXIT_FAILURE;
}

#endif /* BUILD_TEST_EXECUTABLE */
