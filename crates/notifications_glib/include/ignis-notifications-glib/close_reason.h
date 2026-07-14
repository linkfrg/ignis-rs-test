#ifndef __IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_H__
#define __IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum IgnisNotificationsGLibCloseReason
{
  IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_EXPIRED,
  IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_DISMISSED,
  IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_D_BUS_CALL,
  IGNIS_NOTIFICATIONS_GLIB_CLOSE_REASON_OTHER,
} IgnisNotificationsGLibCloseReason;

#define IGNIS_NOTIFICATIONS_GLIB_TYPE_CLOSE_REASON (ignis_notifications_glib_close_reason_get_type())

GType   ignis_notifications_glib_close_reason_get_type       (void);

G_END_DECLS

#endif
