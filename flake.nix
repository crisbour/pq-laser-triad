{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
            rust-overlay.overlays.default
        ];
      };
      toUpper = pkgs.lib.strings.toUpper;
      cargofy = s: builtins.replaceStrings [ "-" ] [ "_" ] s;
        #env = config: cc: {
        #   packages = [ cc ];
        #   "CC_${cargofy config}" = "${cc.targetPrefix}cc";
        #   "CXX_${cargofy config}" = "${cc.targetPrefix}c++";
        #   "CARGO_TARGET_${toUpper (cargofy config)}_LINKER" = "${cc.targetPrefix}cc";
        # };
       rustToolchain =
         let
           rust = pkgs.rust-bin;
         in
         if builtins.pathExists ./rust-toolchain.toml then
           rust.fromRustupToolchainFile ./rust-toolchain.toml
         else if builtins.pathExists ./rust-toolchain then
           rust.fromRustupToolchainFile ./rust-toolchain
         else
           rust.stable.latest.default.override {
             extensions = [ "rust-src" "rustfmt" ];
           };

        # Our windows cross package set.
        pkgs-cross-mingw = import pkgs.path {
          inherit system;  # Critical fix to avoid impure env
          crossSystem = {
              config = "x86_64-w64-mingw32";
            };
        };

        # Our windows cross compiler plus
        # the required libraries and headers.
        mingw_w64_cc = pkgs-cross-mingw.stdenv.cc;
        mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
        mingw_w64_pthreads_w_static = pkgs-cross-mingw.windows.mingw_w64_pthreads.overrideAttrs (oldAttrs: {
          # TODO: Remove once / if changed successfully upstreamed.
          configureFlags = (oldAttrs.configureFlags or []) ++ [
            # Rustc require 'libpthread.a' when targeting 'x86_64-pc-windows-gnu'.
            # Enabling this makes it work out of the box instead of failing.
            "--enable-static"
          ];
        });

        wine = pkgs.wineWowPackages.stable;
    in
    {
      devShells =
        {
        default = pkgs.mkShell {
          packages = with pkgs; [
            just
            mingw_w64_cc
            rustToolchain
            grpc-tools # protoc
            protobuf # required for google import *.proto
            openssl
            pkg-config
            clang
            llvmPackages_18.libclang
            cargo-deny
            cargo-edit
            cargo-watch
          ]
          ++ (with pkgsCross.mingwW64; [
            # Inspired from: https://github.com/tursodatabase/libsql-c/blob/ab9dc85ab14a5f84caad5fee139f4b4d75752451/flake.nix#L21
            stdenv.cc
          ]);

          # WARN: This make this environment only for Windows, how to let cargo choose the flags based on target, using .cargo/config.toml?
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            mingw_w64
            mingw_w64_pthreads_w_static
          ]);

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";


            # Set wine as our cargo runner to allow the `run` and `test`
            # command to work.
            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "${wine}/bin/wine64";

            # Avoid polluting home dir
            WINEPREFIX = toString ./.wine;

            LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang.lib}/lib";
            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
          };
        };
      };
    });
}
