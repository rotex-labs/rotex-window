//! Runs one initialization future on the calling thread.

use std::cell::RefCell;
use std::fmt::Display;
use std::future::Future;
use std::rc::Rc;

use super::log::{self, Level};

enum SlotState<T> {
    Idle,
    Pending,
    Ready(T),
    Failed,
}

/// Holds the result of a single initialization future.
///
/// [`InitSlot::spawn`] runs the future to completion on native targets and
/// schedules it on `wasm32`. [`InitSlot::take`] returns the value once, on a
/// later call if the future is still running.
pub struct InitSlot<T> {
    state: Rc<RefCell<SlotState<T>>>,
}

impl<T: 'static> Default for InitSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> InitSlot<T> {
    /// An idle slot.
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(SlotState::Idle)),
        }
    }

    /// Starts `future` unless a start is already in progress or finished.
    ///
    /// On native targets the future finishes before this returns. On `wasm32`
    /// it is polled by the browser, and [`InitSlot::take`] observes it later.
    /// A failure is logged and recorded by [`InitSlot::is_failed`].
    pub fn spawn<E>(&mut self, future: impl Future<Output = Result<T, E>> + 'static)
    where
        E: Display + 'static,
    {
        if !matches!(*self.state.borrow(), SlotState::Idle) {
            return;
        }
        *self.state.borrow_mut() = SlotState::Pending;
        let state = Rc::clone(&self.state);
        let run = async move {
            match future.await {
                Ok(value) => *state.borrow_mut() = SlotState::Ready(value),
                Err(err) => {
                    log::write(Level::Error, &format!("initialization failed: {err}"));
                    *state.borrow_mut() = SlotState::Failed;
                }
            }
        };
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(run);
        #[cfg(not(target_arch = "wasm32"))]
        pollster::block_on(run);
    }

    /// Returns the finished value once.
    pub fn take(&mut self) -> Option<T> {
        let mut state = self.state.borrow_mut();
        match std::mem::replace(&mut *state, SlotState::Idle) {
            SlotState::Ready(value) => Some(value),
            other => {
                *state = other;
                None
            }
        }
    }

    /// `true` after [`InitSlot::spawn`] until the slot is idle again.
    pub fn is_started(&self) -> bool {
        !matches!(*self.state.borrow(), SlotState::Idle)
    }

    /// `true` when the future returned an error.
    pub fn is_failed(&self) -> bool {
        matches!(*self.state.borrow(), SlotState::Failed)
    }
}
