/*
 * Copyright (c) 2021 Igalia S.L.
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
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#include "config.h"

#include <gio/gio.h>

static void
test_resolver_default (void)
{
  GResolver *resolver = g_resolver_get_default ();

  g_assert_nonnull (resolver);
  g_object_unref (resolver);
}

static void
test_resolver_raw_parser_coverage (void)
{
  g_test_skip ("Raw DNS response parsing is only exposed through private GResolver internals.");
}

int
main (int   argc,
      char *argv[])
{
  g_test_init (&argc, &argv, G_TEST_OPTION_ISOLATE_DIRS, NULL);

  g_test_add_func ("/gresolver/default", test_resolver_default);
  g_test_add_func ("/gresolver/raw-parser", test_resolver_raw_parser_coverage);

  return g_test_run ();
}
