use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, glib};
use notification_service_gobject::IgnisNotifications;

use std::sync::OnceLock;
use tokio::runtime::Runtime;

const APP_ID: &str = "com.github.linkfrg.TestApp";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    let notifications = IgnisNotifications::new();
    runtime().spawn(async move { notifications.run().await.unwrap() });

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).build();

    window.present();
}
