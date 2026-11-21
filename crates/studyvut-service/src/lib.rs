//! What the STUDYVUT desktop app and `studyvut-cli` share: the data folder,
//! the settings, the loaded content, the progress store and the actions on
//! them. No Tauri dependency.

pub mod paths;
pub mod service;
pub mod settings;
pub mod submissions;
pub mod views;

pub use paths::Paths;
pub use service::{ContentSource, Service, ServiceConfig, ServiceError};
pub use settings::Settings;
