# PQ Sepia2 Library - Bindgen
This project generates the C bindings for the library, links the library and
provides a shim that can be used for idiomatic Rust.

Furthermore, this enables the Tonic (gRPC) server to be instantiated and
control the driver from your language of choice remotely or traversing VM
isolation using TCP/HTTPS.

### ==DLL dylib needs loading at runtime==
Explanation 🙃:
- Not sure, but I think that state is maintained within the dynamic library heap
  or stack and effectively has its own runtime
- Bindgen keeps asking for a `.lib` statical library, even though I specified
this is `dylib` 🤷

> [!NOTE]
>
> As a result I am loading the library at runtime instead of dynamically linking in the build step


## Existing bindings

- [PQLaserDrv](https://github.com/PicoQuant/PQLaserDrv/tree/master)
    - [Python calling
    sequence](https://github.com/PicoQuant/PQLaserDrv/blob/master/Demos/Python3/SetSomeDataByPython.py)

## Linking

Try to statically link as much as possible of the application such that there
are no surprise issues at runtime with not able to find libraries

There are two main sources to acquire the libraries for this, the PQ
installation itself and public Windows DLLs:
- https://www.dll-files.com
- https://wikidll.com

Adding the extra libraries is necessary for Wine, but not for native Windows


### Missing `.lib`

The linker requires a `.lib` to get the export functions and types, but we only
have a `.dll`. Generate one as show here [GebLibFromDll](https://github.com/KHeresy/GenLibFromDll)

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


## Investigation resources

- [Cross-platform bindgen example for CoolProp](https://github.com/portyanikhin/rfluids)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust) - Call C
with Bindgen
- [Rustonomicon - FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [bindgen-having-trouble-with-global-variable-in-dll](https://users.rust-lang.org/t/bindgen-having-trouble-with-global-variable-in-dll/55530/6)
- [wintun generate and link dll](https://github.com/hackclub/burrow/blob/85640ffce18eac6ac1b6fa85ff278a457c955198/tun/build.rs)
