use crate::error::IgnisNotificationsGLibErrorImp;
use glib::prelude::*;
use glib::subclass::prelude::*;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct IgnisNotificationsGLibServiceImp {
    pub service: Mutex<notifications::NotificationService>,
}

#[glib::object_subclass]
impl ObjectSubclass for IgnisNotificationsGLibServiceImp {
    const NAME: &'static str = "IgnisNotificationsGLibService";
    type Type = super::IgnisNotificationsGLibServiceWrapped;
    type ParentType = glib::Object;
}

impl ObjectImpl for IgnisNotificationsGLibServiceImp {}

impl IgnisNotificationsGLibServiceImp {
    pub async fn run_async(&self) -> Result<(), glib::Error> {
        self.service.lock().await.run().await.map_err(|e| {
            let msg = &e.to_string();
            glib::Error::new(<IgnisNotificationsGLibErrorImp as From<_>>::from(e), msg)
        })?;

        Ok(())
    }
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

pub(crate) mod ffi {
    use super::*;
    use glib::translate::*;
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
}
