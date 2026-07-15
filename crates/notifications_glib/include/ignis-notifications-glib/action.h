#ifndef __ACTION_H__
#define __ACTION_H__

#include <glib-object.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define IGNIS_NOTIFICATIONS_GLIB_TYPE_ACTION    (ignis_notifications_glib_action_get_type())

G_DECLARE_FINAL_TYPE (IgnisNotificationsGLibAction, ignis_notifications_glib_action, IGNIS_NOTIFICATIONS_GLIB, ACTION, GObject)

void        ignis_notifications_glib_action_invoke_async  (IgnisNotificationsGLibAction * self, GCancellable *cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ignis_notifications_glib_action_invoke_finish (IgnisNotificationsGLibAction * self, GAsyncResult *res, GError **error);

guint32 ignis_notifications_glib_action_get_notification_id(IgnisNotificationsGLibAction* self);

gchar* ignis_notifications_glib_action_get_label(IgnisNotificationsGLibAction* self);

gchar* ignis_notifications_glib_action_get_action_key(IgnisNotificationsGLibAction* self);

G_END_DECLS

#endif
