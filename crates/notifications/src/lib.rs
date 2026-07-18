//! # notifications
//! `notifications` provides a notification daemon which receives and manages notifications sent by
//! applications on GNU/Linux desktops that follow XDG Desktop Notifications Specification.
//!
//! ## Example
//! ```rust
//! use notifications::NotificationService;
//!
//! let service = NotificationService::new();
//! service.run().await.unwrap();
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
