# Sepia2 in safe idiomatic Rust

- [X] Feature complete Rust shim for the Sepia2 binding library
    - Only implementation supported is `Prima` Laser. Extension to support the
    other modules can be easily extended
- [X] Test implemented for sanity and functionality check
- [X] Rust error type with conversion and tests
- [-] Generate API from `.proto` definition instead of equivalent duplication in order to avoid hard-coding
  twice and make sure there is no human errors introduced at translation
- [ ] Keep state about GetModuleMap because it needs to be deallocated when
`Sepia` goes out of scope
    - Place API inside a struct and implement `Drop` for it, which calls
    `FreeModuleMap` if it was acquired

> [!WARN]
> 
> Since the API is feature complete on this crate, is it necessary to generate
    the struct from proto here, or is it better to write macro that does
  from/into conversion for identical structs from different sources?
