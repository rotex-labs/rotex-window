# rotex-winit

winit implementation of `rotex-window`.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-winit` runs an `EngineApp` on the winit event loop for desktop and WebAssembly. It maps winit input onto `EngineEvent` and does not record GPU commands.

## Responsibilities

- `WindowBackend` for winit
- Window creation from `WindowDescriptor`
- Key and window events translated into `rotex-window` types

## Dependencies

`rotex-window`.

## Status

Version 0.1.0. On Linux, an active RenderDoc capture selects the X11 event loop so Vulkan surface creation does not require `VK_KHR_wayland_surface`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
