#ifndef __IGNIS_NOTIFICATIONS_GLIB_URGENCY_H__
#define __IGNIS_NOTIFICATIONS_GLIB_URGENCY_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum IgnisNotificationsGLibUrgency
{
  IGNIS_NOTIFICATIONS_GLIB_URGENCY_LOW,
  IGNIS_NOTIFICATIONS_GLIB_URGENCY_NORMAL,
  IGNIS_NOTIFICATIONS_GLIB_URGENCY_CRITICAL,
} IgnisNotificationsGLibUrgency;

#define IGNIS_NOTIFICATIONS_GLIB_TYPE_URGENCY (ignis_notifications_glib_urgency_get_type())

GType   ignis_notifications_glib_urgency_get_type       (void);

G_END_DECLS

#endif
