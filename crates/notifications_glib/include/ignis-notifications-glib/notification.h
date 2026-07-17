#ifndef __NOTIFICATION_H__
#define __NOTIFICATION_H__

#include <glib-object.h>
#include <gio/gio.h>
#include <ignis-notifications-glib/urgency.h>

G_BEGIN_DECLS

#define IGNIS_NOTIFICATIONS_GLIB_TYPE_NOTIFICATIONS (ignis_notifications_glib_service_get_type())

G_DECLARE_FINAL_TYPE (IgnisNotificationsGLibNotification, ignis_notifications_glib_notification, IGNIS_NOTIFICATIONS_GLIB, NOTIFICATION, GObject)

IgnisNotificationsGLibNotification* ignis_notifications_glib_notification_new (void);

guint32 ignis_notifications_glib_notification_get_id (IgnisNotificationsGLibNotification* this); 

gchar* ignis_notifications_glib_notification_get_app_name (IgnisNotificationsGLibNotification* this); 

gchar* ignis_notifications_glib_notification_get_icon (IgnisNotificationsGLibNotification* this); 

gchar* ignis_notifications_glib_notification_get_summary (IgnisNotificationsGLibNotification* this); 

gchar* ignis_notifications_glib_notification_get_body (IgnisNotificationsGLibNotification* this); 

/**
 * ignis_notifications_glib_notification_get_actions:
 * @this: a #IgnisNotificationsGLibNotification
 *
 * Returns: (transfer full) (element-type IgnisNotificationsGLibAction)
 */
GPtrArray* ignis_notifications_glib_notification_get_actions (IgnisNotificationsGLibNotification* this); 

IgnisNotificationsGLibUrgency ignis_notifications_glib_notification_get_urgency (IgnisNotificationsGLibNotification* this); 

gint32 ignis_notifications_glib_notification_get_timeout (IgnisNotificationsGLibNotification* this); 

void        ignis_notifications_glib_notification_dismiss_async  (IgnisNotificationsGLibService * self, GCancellable *cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ignis_notifications_glib_notification_dismiss_finish (IgnisNotificationsGLibService * self, GAsyncResult *res, GError **error);

G_END_DECLS

#endif
