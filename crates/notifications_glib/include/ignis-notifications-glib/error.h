#ifndef __ERROR_H_
#define __ERROR_H_

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum IgnisNotificationsGLibError
{
  IGNIS_NOTIFICATIONS_GLIB_DBUS_ERROR,
  IGNIS_NOTIFICATIONS_GLIB_NO_CONNECTION,
  IGNIS_NOTIFICATIONS_GLIB_IO_ERROR,
  IGNIS_NOTIFICATIONS_GLIB_JSON_ERROR,
  IGNIS_NOTIFICATIONS_GLIB_NOTIFICATION_NOT_FOUND,
  IGNIS_NOTIFICATIONS_GLIB_NOTIFICATION_CONNECTION_INITIALIZED_TWICE,
} IgnisNotificationsGLibError;

#define IGNIS_NOTIFICATIONS_GLIB_ERROR           (ignis_notifications_glib_error_quark())

GQuark ignis_notifications_glib_error_quark      (void);

G_END_DECLS

#endif /* __ERROR_H_ */
