# rotex-window

Platform-neutral window and event orchestration.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-window` describes windows, events, and the application loop without choosing a window library. A provider such as `rotex-winit` implements `WindowBackend`.

## Responsibilities

- `WindowDescriptor`, `WindowMode`, and `EngineEvent`
- `EngineApp`, `WindowBridge`, and `WindowOrchestrator`
- `WindowBackend` for a concrete event loop

## Dependencies

None.

## Status

Version 0.1.0. This crate does not open a window by itself.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under [MIT](LICENSE).
