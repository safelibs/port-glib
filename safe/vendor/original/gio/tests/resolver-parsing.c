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

typedef struct
{
  GResolver parent_instance;
} TestRecordResolver;

typedef struct
{
  GResolverClass parent_class;
} TestRecordResolverClass;

typedef struct
{
  gchar *rrname;
  GResolverRecordType record_type;
} LookupRecordsRequest;

typedef struct
{
  GResolverRecordType record_type;
  const gchar *expected_variant;
} RecordVector;

GType test_record_resolver_get_type (void);

static const RecordVector record_vectors[] =
{
  { G_RESOLVER_RECORD_MX,  "(@q 10, 'mail.example.org')" },
  { G_RESOLVER_RECORD_NS,  "('ns1.example.org',)" },
  { G_RESOLVER_RECORD_SOA, "('ns1.example.org', 'hostmaster.example.org', @u 1, @u 2, @u 3, @u 4, @u 5)" },
  { G_RESOLVER_RECORD_SRV, "(@q 10, @q 5, @q 443, 'service.example.org')" },
  { G_RESOLVER_RECORD_TXT, "(['v=spf1 include:example.org', '~all'],)" },
};

G_DEFINE_TYPE (TestRecordResolver, test_record_resolver, G_TYPE_RESOLVER)

static void
free_records (GList *records)
{
  g_list_free_full (records, (GDestroyNotify) g_variant_unref);
}

static LookupRecordsRequest *
lookup_records_request_new (const gchar         *rrname,
                            GResolverRecordType  record_type)
{
  LookupRecordsRequest *request;

  request = g_new0 (LookupRecordsRequest, 1);
  request->rrname = g_strdup (rrname);
  request->record_type = record_type;

  return request;
}

static void
lookup_records_request_free (LookupRecordsRequest *request)
{
  g_free (request->rrname);
  g_free (request);
}

static GVariant *
lookup_records_variant (const gchar         *rrname,
                        GResolverRecordType  record_type,
                        GError             **error)
{
  const gchar *variant_str = NULL;
  gsize i;

  if (g_strcmp0 (rrname, "example.org") != 0)
    {
      g_set_error (error, G_RESOLVER_ERROR, G_RESOLVER_ERROR_NOT_FOUND,
                   "No test records for %s", rrname);
      return NULL;
    }

  for (i = 0; i < G_N_ELEMENTS (record_vectors); i++)
    {
      if (record_vectors[i].record_type == record_type)
        {
          variant_str = record_vectors[i].expected_variant;
          break;
        }
    }

  if (variant_str == NULL)
    {
      g_set_error (error, G_RESOLVER_ERROR, G_RESOLVER_ERROR_NOT_FOUND,
                   "No test records for type %u", record_type);
      return NULL;
    }

  return g_variant_ref_sink (g_variant_new_parsed (variant_str));
}

static GList *
test_record_resolver_lookup_records (GResolver            *resolver,
                                     const gchar          *rrname,
                                     GResolverRecordType   record_type,
                                     GCancellable         *cancellable,
                                     GError              **error)
{
  GVariant *record;

  if (cancellable != NULL && g_cancellable_set_error_if_cancelled (cancellable, error))
    return NULL;

  record = lookup_records_variant (rrname, record_type, error);
  if (record == NULL)
    return NULL;

  return g_list_append (NULL, record);
}

static gboolean
lookup_records_async_cb (gpointer user_data)
{
  GTask *task = G_TASK (user_data);
  LookupRecordsRequest *request = g_task_get_task_data (task);
  GResolver *resolver = g_task_get_source_object (task);
  GList *records;
  GError *error = NULL;

  records = test_record_resolver_lookup_records (resolver,
                                                 request->rrname,
                                                 request->record_type,
                                                 g_task_get_cancellable (task),
                                                 &error);

  if (records != NULL)
    g_task_return_pointer (task, records, (GDestroyNotify) free_records);
  else
    g_task_return_error (task, g_steal_pointer (&error));

  return G_SOURCE_REMOVE;
}

static void
test_record_resolver_lookup_records_async (GResolver           *resolver,
                                           const gchar         *rrname,
                                           GResolverRecordType  record_type,
                                           GCancellable        *cancellable,
                                           GAsyncReadyCallback  callback,
                                           gpointer             user_data)
{
  GTask *task;

  task = g_task_new (resolver, cancellable, callback, user_data);
  g_task_set_source_tag (task, test_record_resolver_lookup_records_async);
  g_task_set_task_data (task,
                        lookup_records_request_new (rrname, record_type),
                        (GDestroyNotify) lookup_records_request_free);
  g_idle_add_full (G_PRIORITY_DEFAULT, lookup_records_async_cb,
                   g_object_ref (task), g_object_unref);
  g_object_unref (task);
}

static GList *
test_record_resolver_lookup_records_finish (GResolver     *resolver,
                                            GAsyncResult  *result,
                                            GError       **error)
{
  return g_task_propagate_pointer (G_TASK (result), error);
}

static void
test_record_resolver_class_init (TestRecordResolverClass *klass)
{
  GResolverClass *resolver_class = G_RESOLVER_CLASS (klass);

  resolver_class->lookup_records = test_record_resolver_lookup_records;
  resolver_class->lookup_records_async = test_record_resolver_lookup_records_async;
  resolver_class->lookup_records_finish = test_record_resolver_lookup_records_finish;
}

static void
test_record_resolver_init (TestRecordResolver *resolver)
{
}

static void
assert_record_result (GList       *records,
                      const gchar *expected_variant)
{
  GVariant *expected;

  g_assert_nonnull (records);
  g_assert_null (records->next);

  expected = g_variant_ref_sink (g_variant_new_parsed (expected_variant));
  g_assert_cmpvariant (records->data, expected);
  g_variant_unref (expected);

  free_records (records);
}

static void
test_resolver_default_override (void)
{
  GResolver *original_default;
  GResolver *test_default;
  GResolver *current_default;

  original_default = g_resolver_get_default ();
  test_default = g_object_new (test_record_resolver_get_type (), NULL);

  g_resolver_set_default (test_default);
  current_default = g_resolver_get_default ();
  g_assert_true (current_default == test_default);

  g_object_unref (current_default);
  g_resolver_set_default (original_default);
  g_object_unref (test_default);
  g_object_unref (original_default);
}

static void
test_lookup_records_sync (void)
{
  GResolver *resolver;
  gsize i;

  resolver = g_object_new (test_record_resolver_get_type (), NULL);

  for (i = 0; i < G_N_ELEMENTS (record_vectors); i++)
    {
      GList *records;
      GError *error = NULL;

      records = g_resolver_lookup_records (resolver,
                                           "example.org",
                                           record_vectors[i].record_type,
                                           NULL,
                                           &error);

      g_assert_no_error (error);
      assert_record_result (records, record_vectors[i].expected_variant);
    }

  g_object_unref (resolver);
}

typedef struct
{
  GMainLoop *loop;
  GAsyncResult *result;
} AsyncLookupData;

static void
lookup_records_ready_cb (GObject      *source_object,
                         GAsyncResult *result,
                         gpointer      user_data)
{
  AsyncLookupData *data = user_data;

  data->result = g_object_ref (result);
  g_main_loop_quit (data->loop);
}

static GList *
lookup_records_async_and_finish (GResolver            *resolver,
                                 const gchar          *rrname,
                                 GResolverRecordType   record_type,
                                 GCancellable         *cancellable,
                                 GError              **error)
{
  AsyncLookupData data;
  GList *records;

  data.loop = g_main_loop_new (NULL, FALSE);
  data.result = NULL;

  g_resolver_lookup_records_async (resolver,
                                   rrname,
                                   record_type,
                                   cancellable,
                                   lookup_records_ready_cb,
                                   &data);
  g_main_loop_run (data.loop);

  records = g_resolver_lookup_records_finish (resolver, data.result, error);

  g_object_unref (data.result);
  g_main_loop_unref (data.loop);

  return records;
}

static void
test_lookup_records_async (void)
{
  GResolver *resolver;
  gsize i;

  resolver = g_object_new (test_record_resolver_get_type (), NULL);

  for (i = 0; i < G_N_ELEMENTS (record_vectors); i++)
    {
      GList *records;
      GError *error = NULL;

      records = lookup_records_async_and_finish (resolver,
                                                 "example.org",
                                                 record_vectors[i].record_type,
                                                 NULL,
                                                 &error);

      g_assert_no_error (error);
      assert_record_result (records, record_vectors[i].expected_variant);
    }

  g_object_unref (resolver);
}

static void
test_lookup_records_errors (void)
{
  GResolver *resolver;
  GCancellable *cancellable;
  GList *records;
  GError *error = NULL;

  resolver = g_object_new (test_record_resolver_get_type (), NULL);

  records = g_resolver_lookup_records (resolver,
                                       "missing.example.org",
                                       G_RESOLVER_RECORD_TXT,
                                       NULL,
                                       &error);
  g_assert_error (error, G_RESOLVER_ERROR, G_RESOLVER_ERROR_NOT_FOUND);
  g_assert_null (records);
  g_clear_error (&error);

  records = lookup_records_async_and_finish (resolver,
                                             "example.org",
                                             (GResolverRecordType) 0,
                                             NULL,
                                             &error);
  g_assert_error (error, G_RESOLVER_ERROR, G_RESOLVER_ERROR_NOT_FOUND);
  g_assert_null (records);
  g_clear_error (&error);

  cancellable = g_cancellable_new ();
  g_cancellable_cancel (cancellable);
  records = g_resolver_lookup_records (resolver,
                                       "example.org",
                                       G_RESOLVER_RECORD_TXT,
                                       cancellable,
                                       &error);
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_CANCELLED);
  g_assert_null (records);
  g_clear_error (&error);

  records = lookup_records_async_and_finish (resolver,
                                             "example.org",
                                             G_RESOLVER_RECORD_TXT,
                                             cancellable,
                                             &error);
  g_assert_error (error, G_IO_ERROR, G_IO_ERROR_CANCELLED);
  g_assert_null (records);
  g_clear_error (&error);

  g_object_unref (cancellable);
  g_object_unref (resolver);
}

int
main (int   argc,
      char *argv[])
{
  g_test_init (&argc, &argv, G_TEST_OPTION_ISOLATE_DIRS, NULL);

  g_test_add_func ("/gresolver/default/override", test_resolver_default_override);
  g_test_add_func ("/gresolver/lookup-records/sync", test_lookup_records_sync);
  g_test_add_func ("/gresolver/lookup-records/async", test_lookup_records_async);
  g_test_add_func ("/gresolver/lookup-records/errors", test_lookup_records_errors);

  return g_test_run ();
}
