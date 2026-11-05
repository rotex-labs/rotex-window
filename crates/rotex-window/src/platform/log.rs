//! Text log that reaches a console on native and in the browser.

/// Severity of one log line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Ordinary progress.
    Info,
    /// A recoverable problem.
    Warn,
    /// A failure.
    Error,
}

/// Writes one line to stdout, stderr, or the browser console.
pub fn write(level: Level, message: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    match level {
        Level::Info => println!("{message}"),
        Level::Warn | Level::Error => eprintln!("{message}"),
    }

    #[cfg(target_arch = "wasm32")]
    match level {
        Level::Info => console_log(message),
        Level::Warn => console_warn(message),
        Level::Error => console_error(message),
    }
}

#[cfg(target_arch = "wasm32")]
mod console {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = log)]
        pub fn console_log(message: &str);
        #[wasm_bindgen(js_namespace = console, js_name = warn)]
        pub fn console_warn(message: &str);
        #[wasm_bindgen(js_namespace = console, js_name = error)]
        pub fn console_error(message: &str);
    }
}

#[cfg(target_arch = "wasm32")]
use console::{console_error, console_log, console_warn};

/// Logs at [`Level::Info`](crate::platform::log::Level::Info).
#[macro_export]
macro_rules! info {
    ($($token:tt)*) => {
        $crate::platform::log::write($crate::platform::log::Level::Info, &format!($($token)*))
    };
}

/// Logs at [`Level::Warn`](crate::platform::log::Level::Warn).
#[macro_export]
macro_rules! warn {
    ($($token:tt)*) => {
        $crate::platform::log::write($crate::platform::log::Level::Warn, &format!($($token)*))
    };
}

/// Logs at [`Level::Error`](crate::platform::log::Level::Error).
#[macro_export]
macro_rules! error {
    ($($token:tt)*) => {
        $crate::platform::log::write($crate::platform::log::Level::Error, &format!($($token)*))
    };
}
