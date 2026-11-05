<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg">
    <img alt="Rotex: One API. Every GPU. Zero cost." src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg" width="1200">
  </picture>
</p>

# rotex-window

Window description and the winit provider. No GPU API.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

This repository describes windows, events, and the application loop, then runs that loop on winit. `rotex-window` does not open a window by itself. `rotex-winit` implements `WindowBackend` for desktop and WebAssembly. It maps winit input onto `EngineEvent` and does not record GPU commands. No other Rotex crate is involved.

## Crates

| Package | Description | Version |
|---------|-------------|---------|
| [`rotex-window`](crates/rotex-window/README.md) | Platform-neutral window and event orchestration | 0.1.1 |
| [`rotex-winit`](crates/rotex-winit/README.md) | winit implementation of `rotex-window` | 0.1.1 |

## Dependencies

`rotex-winit` depends on `rotex-window` by path plus version inside this repository. A requirement that leaves the slice stays a crates.io version. Each package keeps its own version and its own release tag (`rotex-window-v0.1.1`, `rotex-winit-v0.1.1`).

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](crates/rotex-window/LICENSE-APACHE))
* MIT license ([LICENSE-MIT](crates/rotex-window/LICENSE-MIT))

at your option. Each crate directory carries the same two files.
