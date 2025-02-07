{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default self.overlays.default ];
        };
      });
      # Our windows cross package set.
      #pkgs-cross-mingw = pkgs: import pkgs.path {
      #  crossSystem = {
      #      config = "x86_64-w64-mingw32";
      #    };
      #};

      ## Our windows cross compiler plus
      ## the required libraries and headers.
      #mingw_w64_cc = pkgs: (pkgs-cross-mingw pkgs).stdenv.cc;
      #mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
      #mingw_w64_pthreads_w_static = pkgs-cross-mingw.windows.mingw_w64_pthreads.overrideAttrs (oldAttrs: {
      #  # TODO: Remove once / if changed successfully upstreamed.
      #  configureFlags = (oldAttrs.configureFlags or []) ++ [
      #    # Rustc require 'libpthread.a' when targeting 'x86_64-pc-windows-gnu'.
      #    # Enabling this makes it work out of the box instead of failing.
      #    "--enable-static"
      #  ];
      #});
      #wine = pkgs: pkgs.wineWowPackages.stable;
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.stable.latest.default.override {
              extensions = [ "rust-src" "rustfmt" ];
            };
      };

      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            #       (mingw_w64_cc pkgs)
            rustToolchain
            grpc-tools # protoc
            protobuf # required for google import *.proto
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";

            # Avoid polluting home dir
            WINEPREFIX = toString ./.wine;

            LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang.lib}/lib";
          };
        };
      });
    };
}
