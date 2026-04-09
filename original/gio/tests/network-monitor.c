/* GIO - GLib Input, Output and Streaming Library
 *
 * Copyright 2011 Red Hat, Inc.
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
 * You should have received a copy of the GNU Lesser General
 * Public License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#include <gio/gio.h>
#include <string.h>

typedef struct
{
  GMainLoop *loop;
  gboolean reachable;
  gboolean had_error;
} CanReachData;

static void
reach_cb (GObject      *source,
          GAsyncResult *res,
          gpointer      user_data)
{
  CanReachData *data = user_data;
  GNetworkMonitor *monitor = G_NETWORK_MONITOR (source);
  GError *error = NULL;

  data->reachable = g_network_monitor_can_reach_finish (monitor, res, &error);
  data->had_error = (error != NULL);
  g_clear_error (&error);

  g_main_loop_quit (data->loop);
}

static void
test_default_monitor (void)
{
  GNetworkMonitor *monitor;
  GNetworkMonitor *monitor2;
  GInetAddress *loopback;
  GSocketAddress *sockaddr;
  GError *error = NULL;
  gboolean reachable;
  CanReachData data;

  monitor = g_network_monitor_get_default ();
  monitor2 = g_network_monitor_get_default ();

  g_assert_true (G_IS_NETWORK_MONITOR (monitor));
  g_assert_true (monitor == monitor2);

  /* Exercise the public getters without making assumptions about the host
   * machine’s current network state.
   */
  (void) g_network_monitor_get_network_available (monitor);
  (void) g_network_monitor_get_network_metered (monitor);
  (void) g_network_monitor_get_connectivity (monitor);

  loopback = g_inet_address_new_from_string ("127.0.0.1");
  g_assert_nonnull (loopback);
  sockaddr = g_inet_socket_address_new (loopback, 0);

  reachable = g_network_monitor_can_reach (monitor,
                                           G_SOCKET_CONNECTABLE (sockaddr),
                                           NULL,
                                           &error);

  data.loop = g_main_loop_new (NULL, FALSE);
  data.reachable = FALSE;
  data.had_error = FALSE;

  g_network_monitor_can_reach_async (monitor,
                                     G_SOCKET_CONNECTABLE (sockaddr),
                                     NULL,
                                     reach_cb,
                                     &data);
  g_main_loop_run (data.loop);

  g_assert_cmpint (reachable, ==, data.reachable);
  g_assert_cmpint (error != NULL, ==, data.had_error);

  g_clear_error (&error);
  g_main_loop_unref (data.loop);
  g_object_unref (sockaddr);
  g_object_unref (loopback);
}

static void
watch_network_changed (GNetworkMonitor *monitor,
                       gboolean         available,
                       gpointer         user_data)
{
  g_print ("Network is %s\n", available ? "up" : "down");
}

static void
watch_connectivity_changed (GNetworkMonitor *monitor,
                            GParamSpec      *pspec,
                            gpointer         user_data)
{
  g_print ("Connectivity is %d\n", g_network_monitor_get_connectivity (monitor));
}

static void
watch_metered_changed (GNetworkMonitor *monitor,
                       GParamSpec      *pspec,
                       gpointer         user_data)
{
  g_print ("Metered is %d\n", g_network_monitor_get_network_metered (monitor));
}

static void
do_watch_network (void)
{
  GNetworkMonitor *monitor = g_network_monitor_get_default ();
  GMainLoop *loop;

  g_print ("Monitoring via %s\n", g_type_name_from_instance ((GTypeInstance *) monitor));

  g_signal_connect (monitor, "network-changed",
                    G_CALLBACK (watch_network_changed), NULL);
  g_signal_connect (monitor, "notify::connectivity",
                    G_CALLBACK (watch_connectivity_changed), NULL);
  g_signal_connect (monitor, "notify::network-metered",
                    G_CALLBACK (watch_metered_changed), NULL);
  watch_network_changed (monitor, g_network_monitor_get_network_available (monitor), NULL);
  watch_connectivity_changed (monitor, NULL, NULL);
  watch_metered_changed (monitor, NULL, NULL);

  loop = g_main_loop_new (NULL, FALSE);
  g_main_loop_run (loop);
}

int
main (int argc, char **argv)
{
  if (argc == 2 && !strcmp (argv[1], "--watch"))
    {
      do_watch_network ();
      return 0;
    }

  g_test_init (&argc, &argv, NULL);

  /* Keep proxy resolution out of the picture so sync and async reachability
   * checks observe the same direct-connection path.
   */
  g_setenv ("GIO_USE_PROXY_RESOLVER", "dummy", TRUE);

  g_test_add_func ("/network-monitor/default", test_default_monitor);

  return g_test_run ();
}
