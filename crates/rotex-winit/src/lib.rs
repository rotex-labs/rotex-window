#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! winit backend for [`rotex_window`].
//!
//! [`WinitBackend`] implements [`rotex_window::WindowBackend`] and runs an
//! [`rotex_window::EngineApp`] on the winit event loop. It creates the window
//! from a [`rotex_window::WindowDescriptor`] and translates winit input into
//! [`rotex_window::EngineEvent`] values. It does not record GPU commands.
//!
//! Desktop builds run until the window closes. WebAssembly builds append a
//! canvas to the page and drive update and render events from
//! `requestAnimationFrame`. `wasm_pre_present_notify` and `last_rafgap_us` are
//! compiled only for `wasm32`.
//!
//! On Linux, an active RenderDoc capture selects the X11 event loop so Vulkan
//! surface creation does not require `VK_KHR_wayland_surface`.

mod backend;
mod bridge;
mod error_map;
mod event_map;
mod runner;
#[cfg(target_arch = "wasm32")]
mod wasm_frame_scheduler;
#[cfg(target_arch = "wasm32")]
mod wasm_pre_present;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
pub use wasm_frame_scheduler::last_rafgap_us;

/// Notify the canvas that a frame is about to be presented.
///
/// Call this after rendering, on the same turn as the animation-frame callback.
/// The call does nothing until a window exists.
#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
pub fn wasm_pre_present_notify() {
    wasm_pre_present::notify();
}

pub use backend::WinitBackend;
pub use bridge::WinitBridge;

/// Construct a [`WinitBackend`].
pub fn backend() -> WinitBackend {
    WinitBackend
}
