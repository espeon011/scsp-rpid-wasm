{
  description = "Rust environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "rust";
          nativeBuildInputs = [
            rustToolchain
            pkgs.wasm-pack
          ];
          buildInputs = [
          ];
          packages = [];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        packages.default = rustPlatform.buildRustPackage {
          pname = "scsp-rpid-wasm";
          version = "v0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "rpid-0.3.1" = "sha256-1yQZLJh3ZSKDibJscfITeA3TKUuTuWB1A04WmRgissk=";
            };
          };

          nativeBuildInputs = [
            # rustToolchain
          ];
          buildInputs = [];
          packages = [];
        };
      }
    );
}
