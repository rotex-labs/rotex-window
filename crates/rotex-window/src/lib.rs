#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Platform-neutral window and event orchestration.
//!
//! This crate describes a window, the events it produces, and the application
//! loop that consumes them. It does not choose a window library and it does not
//! open a window by itself. A backend implements [`WindowBackend`] and drives an
//! [`EngineApp`].
//!
//! Build a [`WindowDescriptor`], implement [`EngineApp`], and pass both to a
//! [`WindowOrchestrator`]. The backend delivers [`EngineEvent`] values and a
//! [`WindowBridge`] for title, cursor, redraw, and size changes. Surface handles
//! come from [`raw_window_handle`].

/// Application callback trait.
pub mod app;
/// Event-loop implementation trait.
pub mod backend;
/// Live window operations passed into application callbacks.
pub mod bridge;
/// Window size, title, and mode.
pub mod descriptor;
/// Errors returned while creating or driving a window.
pub mod error;
/// Lifecycle and input events.
pub mod event;
/// Physical key identifiers.
pub mod keyboard;
/// Entry point that runs an application on a backend.
pub mod orchestrator;

pub use app::EngineApp;
pub use backend::WindowBackend;
pub use bridge::WindowBridge;
pub use descriptor::{WindowDescriptor, WindowMode};
pub use error::Error;
pub use event::EngineEvent;
pub use keyboard::{KeyCode, NativeKeyCode};
pub use orchestrator::WindowOrchestrator;
