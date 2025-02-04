# PQ Sepia2 Library - Bindgen
This project generates the C bindings for the library, links the library and
provides a shim that can be used for idiomatic Rust.

Furthermore, this enables the Tonic (gRPC) server to be instantiated and
control the driver from your language of choice remotely or traversing VM
isolation using TCP/HTTPS.

## TODO

PQ-RPC development plan:
- [-] Generate C bindings for the PQ library
  - [ ] stdbool.h missing for Sepia2_Def.h; Need stdenv.cc.cc.lib added to LD_LIBRARY_PATH
- [ ] Write shims that transform pointer mutation output parameters in req->resp function calls
  - macro to generate shims for the RPC that transform outputs in immutable response objects
- [ ] Generate protobuf definition from the later API
- [ ] Tonic server implementation that handles gRPC interface to this API

## Dev Env

At the moment I am using a `nix-shell` that generates all configurations locally (rust, wine and cargo), but
lock files easily break due to Wine exiting unexpectedly.

- [cross-rs](https://github.com/cross-rs/cross) can be used as an all rounding
alternative specifically build for cross compilig Rust projects
- Find another nix cross-compile alternative
