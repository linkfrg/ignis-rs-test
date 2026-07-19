//! # notifications
//! `notifications` provides a notification daemon which receives and manages notifications sent by
//! applications on GNU/Linux desktops that follow XDG Desktop Notifications Specification.
//!
//! ## Example
//! ```rust
//! use notifications::{NotificationService, NotificationServiceSignal};
//! use tokio::sync::mpsc;
//!
//! # let rt = tokio::runtime::Runtime::new().unwrap();
//! # rt.block_on(async {
//! let (tx, mut rx) = mpsc::channel(32);
//!
//! let service = NotificationService::new(Some(tx), None).unwrap();
//! service.run().await.unwrap();
//!
//! // listen for events
//! tokio::spawn(async move {
//!     while let Some(msg) = rx.recv().await {
//!         match msg {
//!            NotificationServiceSignal::CloseNotification {id, reason} => println!("notificaiton
//!            closed id: {}, reason: {:?}", id, reason),
//!            NotificationServiceSignal::Notified {id, notification, replace} => println!("New
//!            notification! id: {}, summary: {}, replaces old one: {}", id, notification.summary(),
//!            replace)
//!         }
//!     }
//! });
//! # });
//!
//! ```

#![warn(missing_docs)]

mod action;
mod close_reason;
mod data;
mod dbus;
mod error;
mod file_utils;
mod notification;
mod private_prelude;
mod service;
mod signals;
mod urgency;

pub use action::ActionHandle;
pub use close_reason::CloseReason;
pub use error::{Error, Result};
pub use notification::NotificationHandle;
pub use service::NotificationService;
pub use signals::NotificationServiceSignal;
pub use urgency::Urgency;
