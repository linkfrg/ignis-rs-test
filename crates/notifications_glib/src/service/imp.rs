use crate::error::IgnisNotificationsGLibErrorImp;
use crate::notification::GNotificationWrapped;
use glib::prelude::*;
use glib::subclass::{Signal, prelude::*};
use glib::translate::*;
use notifications::NotificationServiceSignal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct IgnisNotificationsGLibServiceImp {
    pub service: notifications::NotificationService,
    pub notifications: RefCell<HashMap<u32, GNotificationWrapped>>,
}

#[glib::object_subclass]
impl ObjectSubclass for IgnisNotificationsGLibServiceImp {
    const NAME: &'static str = "IgnisNotificationsGLibService";
    type Type = super::IgnisNotificationsGLibServiceWrapped;
    type ParentType = glib::Object;
}

fn vec_to_list_store(vec: Vec<GNotificationWrapped>) -> gio::ListStore {
    let store = gio::ListStore::new::<GNotificationWrapped>();

    for notification in vec {
        store.append(&notification);
    }

    store
}

impl ObjectImpl for IgnisNotificationsGLibServiceImp {
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
            "notifications" => vec_to_list_store(self.get_notifications()).to_value(),
            _ => unimplemented!(),
        }
    }
}

impl IgnisNotificationsGLibServiceImp {
    pub async fn run_async(&self) -> Result<(), glib::Error> {
        let (tx, mut rx) = mpsc::channel::<NotificationServiceSignal>(32);

        self.service.run(Some(tx)).await.map_err(|e| {
            let msg = &e.to_string();
            glib::Error::new(<IgnisNotificationsGLibErrorImp as From<_>>::from(e), msg)
        })?;

        *self.notifications.borrow_mut() = self
            .service
            .get_notifications()
            .await
            .into_iter()
            .map(|n| (n.id, GNotificationWrapped::new_from_rust(n)))
            .collect();

        let obj = self.obj().to_owned();

        glib::MainContext::default().spawn_local(async move {
            while let Some(signal) = rx.recv().await {
                match signal {
                    NotificationServiceSignal::Closed { id } => {
                        obj.imp().notifications.borrow_mut().remove(&id);
                        obj.notify("notifications");
                        obj.emit_by_name_with_values("closed", &[id.to_value()]);
                    }
                    NotificationServiceSignal::Notified { id, replace } => {
                        // TODO: implement replace
                        let notification = obj.imp().service.get_notification_by_id(id).await;
                        if let Some(notification) = notification {
                            obj.imp()
                                .notifications
                                .borrow_mut()
                                .insert(id, GNotificationWrapped::new_from_rust(notification));
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

    pub fn get_notifications(&self) -> Vec<GNotificationWrapped> {
        let mut unsorted: Vec<GNotificationWrapped> =
            self.notifications.borrow().values().cloned().collect();

        unsorted.sort_by_key(|v| v.imp().notification.borrow().id);

        unsorted
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
        <super::IgnisNotificationsGLibServiceImp as super::ObjectSubclass>::Instance;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_new()
    -> *mut IgnisNotificationsGLibService {
        glib::Object::builder::<super::super::IgnisNotificationsGLibServiceWrapped>()
            .build()
            .to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_service_get_type() -> glib::ffi::GType {
        <super::super::IgnisNotificationsGLibServiceWrapped as StaticType>::static_type()
            .into_glib()
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_run_async(
        this: *mut IgnisNotificationsGLibService,
        cancellable: *mut gio::ffi::GCancellable,
        callback: gio::ffi::GAsyncReadyCallback,
        user_data: *mut c_void,
    ) {
        let imp = unsafe { (*this).imp() };
        let obj =
            unsafe { &super::super::IgnisNotificationsGLibServiceWrapped::from_glib_none(this) };

        let cancellable = unsafe { gio::Cancellable::from_glib_none(cancellable) };

        let closure =
            move |task: gio::LocalTask<bool>,
                  _: Option<&super::super::IgnisNotificationsGLibServiceWrapped>| {
                let result: *mut gio::ffi::GAsyncResult =
                    task.upcast_ref::<gio::AsyncResult>().to_glib_none().0;

                if let Some(func) = callback {
                    unsafe { func(this as *mut _, result, user_data) }
                }
            };

        let task = unsafe { gio::LocalTask::new(Some(obj), Some(&cancellable), closure) };

        glib::MainContext::ref_thread_default().spawn_local(async move {
            let _guard = runtime().enter();
            let res = imp.run_async().await.map(|_| true);
            task.return_result(res);
        });
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_run_finish(
        _this: *mut IgnisNotificationsGLibService,
        res: *mut gio::ffi::GAsyncResult,
        error: *mut *mut glib::ffi::GError,
    ) -> bool {
        let task = unsafe { gio::Task::<bool>::from_glib_none(res as *mut gio::ffi::GTask) };

        return match unsafe { task.propagate() } {
            Ok(_) => true,
            Err(e) => {
                if !error.is_null() {
                    unsafe { *error = e.into_glib_ptr() };
                }
                false
            }
        };
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_service_get_notifications(
        this: *mut IgnisNotificationsGLibService,
    ) -> *mut glib::ffi::GList {
        let imp = unsafe { (*this).imp() };
        imp.get_notifications().to_glib_full()
    }
}
