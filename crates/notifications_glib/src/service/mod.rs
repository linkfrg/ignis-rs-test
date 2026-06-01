pub mod imp;
use glib::Object;

use glib::subclass::prelude::*;
use glib::translate::*;

glib::wrapper! {
    pub struct IgnisNotificationsGLibServiceWrapped(ObjectSubclass<imp::IgnisNotificationsGLibServiceImp>);
}


impl IgnisNotificationsGLibServiceWrapped {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub async fn run_async(&self) -> Result<(), glib::Error> {
        self.imp().run_async().await
    }
}
