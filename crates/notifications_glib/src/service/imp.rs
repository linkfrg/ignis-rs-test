use crate::error::GNotificationServiceError;
use crate::notification::GDesktopNotification;
use gio::prelude::ListModelExt;
use glib::prelude::*;
use glib::subclass::{Signal, prelude::*};
use glib::translate::*;
use glib_utils::{IntoGLibError, glib_async_method};
use notifications::NotificationServiceSignal;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub struct GNotificationServiceImp {
    pub service: notifications::NotificationService,
    pub notifications: gio::ListStore,
}

impl Default for GNotificationServiceImp {
    fn default() -> Self {
        Self {
            service: notifications::NotificationService::default(),
            notifications: gio::ListStore::new::<GDesktopNotification>(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GNotificationServiceImp {
    const NAME: &'static str = "IgnisNotificationsGLibService";
    type Type = super::GNotificationService;
    type ParentType = glib::Object;
}

impl ObjectImpl for GNotificationServiceImp {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("notified")
                    .param_types([u32::static_type(), bool::static_type()])
                    .build(),
                Signal::builder("closed")
                    .param_types([u32::static_type()])
                    .build(),
            ]
        })
    }
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();

        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecObject::builder::<gio::ListStore>("notifications")
                    .read_only()
                    .build(),
            ]
        })
    }

    fn property(&self, _id: usize, _pspec: &glib::ParamSpec) -> glib::Value {
        match _pspec.name() {
            "notifications" => self.notifications.to_value(),
            _ => unimplemented!(),
        }
    }
}

impl GNotificationServiceImp {
    async fn populate_lists(&self) {
        let initial_notifications = self.service.get_notifications().await;

        for n in initial_notifications {
            let obj = GDesktopNotification::new_from_rust(n);

            self.notifications.append(&obj);
        }
    }
    pub async fn run_async(&self) -> Result<(), glib::Error> {
        let (tx, mut rx) = mpsc::channel::<NotificationServiceSignal>(32);

        self.service
            .run(Some(tx))
            .await
            .into_glib_error::<GNotificationServiceError>()?;

        self.populate_lists().await;

        let obj = self.obj().to_owned();

        glib::MainContext::default().spawn_local(async move {
            while let Some(signal) = rx.recv().await {
                match signal {
                    NotificationServiceSignal::Closed { id } => {
                        for i in 0..obj.imp().notifications.n_items() {
                            let notification = obj
                                .imp()
                                .notifications
                                .item(i)
                                .unwrap()
                                .downcast::<GDesktopNotification>()
                                .unwrap();

                            if notification.imp().get_id() == id {
                                obj.imp().notifications.remove(i);
                                break;
                            }
                        }

                        obj.notify("notifications");
                        obj.emit_by_name_with_values("closed", &[id.to_value()]);
                    }
                    NotificationServiceSignal::Notified { id, replace } => {
                        // TODO: implement replace
                        // TODO: Send DesktopNotification object in this signal
                        let notification = obj.imp().service.get_notification_by_id(id).await;
                        if let Some(notification) = notification {
                            let g_desktop_notification =
                                GDesktopNotification::new_from_rust(notification);

                            obj.imp().notifications.append(&g_desktop_notification);
                        }

                        obj.notify("notifications");
                        obj.emit_by_name_with_values(
                            "notified",
                            &[id.to_value(), replace.to_value()],
                        );
                    }
                }
            }
        });

        Ok(())
    }

    pub fn get_notifications(&self) -> Vec<GDesktopNotification> {
        (0..self.notifications.n_items())
            .map(|i| {
                self.notifications
                    .item(i)
                    .unwrap()
                    .downcast::<GDesktopNotification>()
                    .unwrap()
            })
            .collect()
    }

    pub async fn close_notification(&self, notification_id: u32) -> Result<(), glib::Error> {
        self.service
            .close_notification(notification_id)
            .await
            .into_glib_error::<GNotificationServiceError>()
    }

    pub async fn invoke_action(
        &self,
        notification_id: u32,
        action_key: &str,
    ) -> Result<(), glib::Error> {
        self.service
            .invoke_action(notification_id, action_key)
            .await
            .into_glib_error::<GNotificationServiceError>()
    }
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

pub(crate) mod ffi {
    use super::*;
    use std::ffi::c_void;

    pub type IgnisNotificationsGLibService =
        <super::GNotificationServiceImp as super::ObjectSubclass>::Instance;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_new()
    -> *mut IgnisNotificationsGLibService {
        glib::Object::builder::<super::super::GNotificationService>()
            .build()
            .to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_service_get_type() -> glib::ffi::GType {
        <super::super::GNotificationService as StaticType>::static_type().into_glib()
    }

    glib_async_method!(
        IgnisNotificationsGLibService,
        super::super::GNotificationService,
        ignis_notifications_glib_service_run_async,
        ignis_notifications_glib_service_run_finish,
        run_async,
    );

    glib_async_method!(
        IgnisNotificationsGLibService,
        super::super::GNotificationService,
        ignis_notifications_glib_service_close_notification_async,
        ignis_notifications_glib_service_close_notification_finish,
        close_notification,
        notification_id: u32 => { notification_id }
    );

    glib_async_method!(
        IgnisNotificationsGLibService,
        super::super::GNotificationService,
        ignis_notifications_glib_service_invoke_action_async,
        ignis_notifications_glib_service_invoke_action_finish,
        invoke_action,
        notification_id: u32 => { notification_id },
        action_key: *mut glib::ffi::gchar => {
            let string: String = unsafe { from_glib_none(action_key) };
            &*string.clone()
        }
    );

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_get_notifications(
        this: *mut IgnisNotificationsGLibService,
    ) -> *mut glib::ffi::GList {
        let imp = unsafe { (*this).imp() };
        imp.get_notifications().to_glib_full()
    }
}
