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
  GError *error;
} CanReachData;

static void
reach_cb (GObject      *source,
          GAsyncResult *res,
          gpointer      user_data)
{
  CanReachData *data = user_data;
  GNetworkMonitor *monitor = G_NETWORK_MONITOR (source);

  data->reachable = g_network_monitor_can_reach_finish (monitor, res, &data->error);

  g_main_loop_quit (data->loop);
}

#define TEST_TYPE_NETWORK_MONITOR (test_network_monitor_get_type ())
#define TEST_NETWORK_MONITOR(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), TEST_TYPE_NETWORK_MONITOR, TestNetworkMonitor))

GType test_network_monitor_get_type (void);

typedef struct
{
  GObject parent_instance;

  gboolean network_available;
  gboolean network_metered;
  GNetworkConnectivity connectivity;
  GInetAddressMask *reachable_network;
} TestNetworkMonitor;

typedef struct
{
  GObjectClass parent_class;
} TestNetworkMonitorClass;

typedef struct
{
  guint network_changed_count;
  guint network_available_notify_count;
  guint network_metered_notify_count;
  guint connectivity_notify_count;
  gboolean last_network_available;
} MonitorSignalState;

enum
{
  PROP_0,
  PROP_NETWORK_AVAILABLE,
  PROP_NETWORK_METERED,
  PROP_CONNECTIVITY,
};

static void test_network_monitor_iface_init (GNetworkMonitorInterface *iface);
static void test_network_monitor_initable_iface_init (GInitableIface *iface);

G_DEFINE_TYPE_WITH_CODE (TestNetworkMonitor, test_network_monitor, G_TYPE_OBJECT,
                         G_IMPLEMENT_INTERFACE (G_TYPE_INITABLE,
                                                test_network_monitor_initable_iface_init)
                         G_IMPLEMENT_INTERFACE (G_TYPE_NETWORK_MONITOR,
                                                test_network_monitor_iface_init))

static void
test_network_monitor_get_property (GObject    *object,
                                   guint       prop_id,
                                   GValue     *value,
                                   GParamSpec *pspec)
{
  TestNetworkMonitor *monitor = TEST_NETWORK_MONITOR (object);

  switch (prop_id)
    {
    case PROP_NETWORK_AVAILABLE:
      g_value_set_boolean (value, monitor->network_available);
      break;

    case PROP_NETWORK_METERED:
      g_value_set_boolean (value, monitor->network_metered);
      break;

    case PROP_CONNECTIVITY:
      g_value_set_enum (value, monitor->connectivity);
      break;

    default:
      G_OBJECT_WARN_INVALID_PROPERTY_ID (object, prop_id, pspec);
      break;
    }
}

static void
test_network_monitor_finalize (GObject *object)
{
  TestNetworkMonitor *monitor = TEST_NETWORK_MONITOR (object);

  g_clear_object (&monitor->reachable_network);

  G_OBJECT_CLASS (test_network_monitor_parent_class)->finalize (object);
}

static gboolean
test_network_monitor_can_reach (GNetworkMonitor     *monitor,
                                GSocketConnectable  *connectable,
                                GCancellable        *cancellable,
                                GError             **error)
{
  TestNetworkMonitor *test_monitor = TEST_NETWORK_MONITOR (monitor);
  GSocketAddressEnumerator *enumerator;
  GSocketAddress *sockaddr;

  if (cancellable != NULL && g_cancellable_set_error_if_cancelled (cancellable, error))
    return FALSE;

  if (test_monitor->reachable_network == NULL)
    {
      g_set_error_literal (error, G_IO_ERROR, G_IO_ERROR_NETWORK_UNREACHABLE,
                           "Network unreachable");
      return FALSE;
    }

  enumerator = g_socket_connectable_proxy_enumerate (connectable);
  while ((sockaddr = g_socket_address_enumerator_next (enumerator, cancellable, error)) != NULL)
    {
      gboolean matches = FALSE;

      if (G_IS_INET_SOCKET_ADDRESS (sockaddr))
        {
          GInetAddress *address;

          address = g_inet_socket_address_get_address (G_INET_SOCKET_ADDRESS (sockaddr));
          matches = g_inet_address_mask_matches (test_monitor->reachable_network, address);
        }

      g_object_unref (sockaddr);

      if (matches)
        {
          g_object_unref (enumerator);
          return TRUE;
        }
    }
  g_object_unref (enumerator);

  if (error != NULL && *error != NULL)
    return FALSE;

  g_set_error_literal (error, G_IO_ERROR, G_IO_ERROR_HOST_UNREACHABLE,
                       "Host unreachable");
  return FALSE;
}

static void
test_network_monitor_iface_init (GNetworkMonitorInterface *iface)
{
  iface->can_reach = test_network_monitor_can_reach;
}

static gboolean
test_network_monitor_initable_init (GInitable     *initable,
                                    GCancellable  *cancellable,
                                    GError       **error)
{
  return cancellable == NULL || !g_cancellable_set_error_if_cancelled (cancellable, error);
}

static void
test_network_monitor_initable_iface_init (GInitableIface *iface)
{
  iface->init = test_network_monitor_initable_init;
}

static void
test_network_monitor_class_init (TestNetworkMonitorClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = test_network_monitor_get_property;
  object_class->finalize = test_network_monitor_finalize;

  g_object_class_override_property (object_class, PROP_NETWORK_AVAILABLE, "network-available");
  g_object_class_override_property (object_class, PROP_NETWORK_METERED, "network-metered");
  g_object_class_override_property (object_class, PROP_CONNECTIVITY, "connectivity");
}

static void
test_network_monitor_init (TestNetworkMonitor *monitor)
{
  monitor->connectivity = G_NETWORK_CONNECTIVITY_LOCAL;
}

static TestNetworkMonitor *
test_network_monitor_new (void)
{
  GError *error = NULL;
  TestNetworkMonitor *monitor;

  monitor = g_initable_new (TEST_TYPE_NETWORK_MONITOR, NULL, &error, NULL);
  g_assert_no_error (error);
  g_assert_nonnull (monitor);

  return monitor;
}

static void
test_network_monitor_set_reachable_network (TestNetworkMonitor *monitor,
                                            const gchar        *mask_string)
{
  GError *error = NULL;

  g_clear_object (&monitor->reachable_network);

  if (mask_string == NULL)
    return;

  monitor->reachable_network = g_inet_address_mask_new_from_string (mask_string, &error);
  g_assert_no_error (error);
  g_assert_nonnull (monitor->reachable_network);
}

static void
test_network_monitor_set_state (TestNetworkMonitor    *monitor,
                                gboolean               network_available,
                                gboolean               network_metered,
                                GNetworkConnectivity   connectivity)
{
  gboolean available_changed = monitor->network_available != network_available;
  gboolean metered_changed = monitor->network_metered != network_metered;
  gboolean connectivity_changed = monitor->connectivity != connectivity;

  monitor->network_available = network_available;
  monitor->network_metered = network_metered;
  monitor->connectivity = connectivity;

  if (available_changed)
    {
      g_object_notify (G_OBJECT (monitor), "network-available");
      g_signal_emit_by_name (monitor, "network-changed", network_available);
    }

  if (metered_changed)
    g_object_notify (G_OBJECT (monitor), "network-metered");

  if (connectivity_changed)
    g_object_notify (G_OBJECT (monitor), "connectivity");
}

static GSocketConnectable *
new_connectable_for_address (const gchar *address_string)
{
  GInetAddress *address;
  GSocketConnectable *connectable;

  address = g_inet_address_new_from_string (address_string);
  g_assert_nonnull (address);

  connectable = G_SOCKET_CONNECTABLE (g_inet_socket_address_new (address, 0));
  g_object_unref (address);

  return connectable;
}

static gboolean
can_reach_async_and_wait (GNetworkMonitor     *monitor,
                          GSocketConnectable  *connectable,
                          GCancellable        *cancellable,
                          GError             **error)
{
  CanReachData data = { 0, };

  data.loop = g_main_loop_new (NULL, FALSE);

  g_network_monitor_can_reach_async (monitor,
                                     connectable,
                                     cancellable,
                                     reach_cb,
                                     &data);
  g_main_loop_run (data.loop);

  g_main_loop_unref (data.loop);

  if (data.error != NULL)
    {
      if (error != NULL)
        g_propagate_error (error, g_steal_pointer (&data.error));
      else
        g_clear_error (&data.error);
    }

  return data.reachable;
}

static void
count_notify_cb (GObject    *object,
                 GParamSpec *pspec,
                 gpointer    user_data)
{
  guint *count = user_data;

  (*count)++;
}

static void
network_changed_cb (GNetworkMonitor *monitor,
                    gboolean         network_available,
                    gpointer         user_data)
{
  MonitorSignalState *state = user_data;

  state->network_changed_count++;
  state->last_network_available = network_available;
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
  gboolean async_reachable;
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
  data.error = NULL;

  g_network_monitor_can_reach_async (monitor,
                                     G_SOCKET_CONNECTABLE (sockaddr),
                                     NULL,
                                     reach_cb,
                                     &data);
  g_main_loop_run (data.loop);

  async_reachable = data.reachable;

  g_assert_cmpint (reachable, ==, async_reachable);
  g_assert_cmpint (error != NULL, ==, data.error != NULL);
  if (error != NULL)
    g_assert_error (data.error, error->domain, error->code);

  g_clear_error (&error);
  g_clear_error (&data.error);
  g_main_loop_unref (data.loop);
  g_object_unref (sockaddr);
  g_object_unref (loopback);
}

static void
test_custom_monitor_properties (void)
{
  TestNetworkMonitor *monitor;
  MonitorSignalState state = { 0, };

  monitor = test_network_monitor_new ();

  g_assert_true (G_IS_NETWORK_MONITOR (monitor));
  g_assert_true (G_IS_INITABLE (monitor));
  g_assert_false (g_network_monitor_get_network_available (G_NETWORK_MONITOR (monitor)));
  g_assert_false (g_network_monitor_get_network_metered (G_NETWORK_MONITOR (monitor)));
  g_assert_cmpint (g_network_monitor_get_connectivity (G_NETWORK_MONITOR (monitor)),
                   ==, G_NETWORK_CONNECTIVITY_LOCAL);

  g_signal_connect (monitor, "notify::network-available",
                    G_CALLBACK (count_notify_cb),
                    &state.network_available_notify_count);
  g_signal_connect (monitor, "notify::network-metered",
                    G_CALLBACK (count_notify_cb),
                    &state.network_metered_notify_count);
  g_signal_connect (monitor, "notify::connectivity",
                    G_CALLBACK (count_notify_cb),
                    &state.connectivity_notify_count);
  g_signal_connect (monitor, "network-changed",
                    G_CALLBACK (network_changed_cb),
                    &state);

  test_network_monitor_set_state (monitor,
                                  TRUE,
                                  TRUE,
                                  G_NETWORK_CONNECTIVITY_LIMITED);

  g_assert_true (g_network_monitor_get_network_available (G_NETWORK_MONITOR (monitor)));
  g_assert_true (g_network_monitor_get_network_metered (G_NETWORK_MONITOR (monitor)));
  g_assert_cmpint (g_network_monitor_get_connectivity (G_NETWORK_MONITOR (monitor)),
                   ==, G_NETWORK_CONNECTIVITY_LIMITED);
  g_assert_cmpuint (state.network_changed_count, ==, 1);
  g_assert_true (state.last_network_available);
  g_assert_cmpuint (state.network_available_notify_count, ==, 1);
  g_assert_cmpuint (state.network_metered_notify_count, ==, 1);
  g_assert_cmpuint (state.connectivity_notify_count, ==, 1);

  test_network_monitor_set_state (monitor,
                                  FALSE,
                                  FALSE,
                                  G_NETWORK_CONNECTIVITY_LOCAL);

  g_assert_false (g_network_monitor_get_network_available (G_NETWORK_MONITOR (monitor)));
  g_assert_false (g_network_monitor_get_network_metered (G_NETWORK_MONITOR (monitor)));
  g_assert_cmpint (g_network_monitor_get_connectivity (G_NETWORK_MONITOR (monitor)),
                   ==, G_NETWORK_CONNECTIVITY_LOCAL);
  g_assert_cmpuint (state.network_changed_count, ==, 2);
  g_assert_false (state.last_network_available);
  g_assert_cmpuint (state.network_available_notify_count, ==, 2);
  g_assert_cmpuint (state.network_metered_notify_count, ==, 2);
  g_assert_cmpuint (state.connectivity_notify_count, ==, 2);

  g_object_unref (monitor);
}

static void
test_custom_monitor_can_reach_sync (void)
{
  TestNetworkMonitor *monitor;
  GSocketConnectable *loopback;
  GSocketConnectable *remote;
  GError *error = NULL;

  monitor = test_network_monitor_new ();
  loopback = new_connectable_for_address ("127.0.0.1");
  remote = new_connectable_for_address ("192.0.2.1");

  g_assert_false (g_network_monitor_can_reach (G_NETWORK_MONITOR (monitor),
                                               loopback,
                                               NULL,
                                               &error));
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_NETWORK_UNREACHABLE);
  g_clear_error (&error);

  test_network_monitor_set_reachable_network (monitor, "127.0.0.0/8");
  g_assert_false (g_network_monitor_get_network_available (G_NETWORK_MONITOR (monitor)));

  g_assert_true (g_network_monitor_can_reach (G_NETWORK_MONITOR (monitor),
                                              loopback,
                                              NULL,
                                              &error));
  g_assert_no_error (error);

  g_assert_false (g_network_monitor_can_reach (G_NETWORK_MONITOR (monitor),
                                               remote,
                                               NULL,
                                               &error));
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_HOST_UNREACHABLE);
  g_clear_error (&error);

  g_object_unref (remote);
  g_object_unref (loopback);
  g_object_unref (monitor);
}

static void
test_custom_monitor_can_reach_async (void)
{
  TestNetworkMonitor *monitor;
  GSocketConnectable *loopback;
  GSocketConnectable *remote;
  GCancellable *cancellable;
  GError *error = NULL;

  monitor = test_network_monitor_new ();
  loopback = new_connectable_for_address ("127.0.0.1");
  remote = new_connectable_for_address ("192.0.2.1");

  test_network_monitor_set_reachable_network (monitor, "127.0.0.0/8");

  g_assert_true (can_reach_async_and_wait (G_NETWORK_MONITOR (monitor),
                                           loopback,
                                           NULL,
                                           &error));
  g_assert_no_error (error);

  g_assert_false (can_reach_async_and_wait (G_NETWORK_MONITOR (monitor),
                                            remote,
                                            NULL,
                                            &error));
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_HOST_UNREACHABLE);
  g_clear_error (&error);

  cancellable = g_cancellable_new ();
  g_cancellable_cancel (cancellable);
  g_assert_false (can_reach_async_and_wait (G_NETWORK_MONITOR (monitor),
                                            loopback,
                                            cancellable,
                                            &error));
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_CANCELLED);
  g_clear_error (&error);

  g_object_unref (cancellable);
  g_object_unref (remote);
  g_object_unref (loopback);
  g_object_unref (monitor);
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
  g_test_add_func ("/network-monitor/custom/properties", test_custom_monitor_properties);
  g_test_add_func ("/network-monitor/custom/can-reach-sync", test_custom_monitor_can_reach_sync);
  g_test_add_func ("/network-monitor/custom/can-reach-async", test_custom_monitor_can_reach_async);

  return g_test_run ();
}
