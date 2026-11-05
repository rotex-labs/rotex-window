//! Clock, logging, and one-shot initialization.
//!
//! The functions here hide the native and `wasm32` split. Scene state and draw
//! lists stay in the application.

pub mod log;
pub mod task;
pub mod time;

pub use task::InitSlot;
pub use time::{Duration, FpsMeter, Instant};
