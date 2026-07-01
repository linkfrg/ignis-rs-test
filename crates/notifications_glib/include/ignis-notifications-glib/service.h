#ifndef __SERVICE_H__
#define __SERVICE_H__

#include <glib-object.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define IGNIS_NOTIFICATIONS_GLIB_TYPE_SERVICE    (ignis_notifications_glib_service_get_type())

G_DECLARE_FINAL_TYPE (IgnisNotificationsGLibService, ignis_notifications_glib_service, IGNIS_NOTIFICATIONS_GLIB, SERVICE, GObject)

IgnisNotificationsGLibService * ignis_notifications_glib_service_new         (void);

void        ignis_notifications_glib_service_run_async  (IgnisNotificationsGLibService * self, GCancellable *cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ignis_notifications_glib_service_run_finish (IgnisNotificationsGLibService * self, GAsyncResult *res, GError **error);

/**
 * ignis_notifications_glib_service_get_notifications:
 * @self: a #IgnisNotificationsGLibService
 *
 * Returns: (transfer container) (element-type IgnisNotificationsGLibNotification)
 */
GList* ignis_notifications_glib_service_get_notifications(IgnisNotificationsGLibService* self);

void ignis_notifications_glib_service_close_notification_async(IgnisNotificationsGLibService* self, guint32 notification_id, GCancellable* cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ignis_notifications_glib_service_close_notification_finish (IgnisNotificationsGLibService * self, GAsyncResult *res, GError **error);

void ignis_notifications_glib_service_invoke_action_async(IgnisNotificationsGLibService* self, guint32 notification_id, gchar* action_key, GCancellable* cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ignis_notifications_glib_service_invoke_action_finish (IgnisNotificationsGLibService * self, GAsyncResult *res, GError **error);

G_END_DECLS

#endif /* __SERVICE_H__ */
