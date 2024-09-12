{
  description = "fuzon";

  nixConfig = {
    substituters = [
      # Add here some other mirror if needed.
      "https://cache.nixos.org/"
    ];
    extra-substituters = [
      # Nix community's cache server
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    # Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # You can access packages and modules from different nixpkgs revs
    # at the same time. Here's an working example:
    nixpkgsStable.url = "github:nixos/nixpkgs/nixos-23.11";
    # Also see the 'stable-packages' overlay at 'overlays/default.nix'.

    flake-utils.url = "github:numtide/flake-utils";

    # The Rust overlay to include the latest toolchain.
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
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem
    # Creates an attribute map `{ devShells.<system>.default = ...}`
    # by calling this function:
    (
      system: let
        overlays = [(import rust-overlay)];

        # Import nixpkgs and load it into pkgs.
        # Overlay the rust toolchain
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Set the rust toolchain from the `rust-toolchain.toml`.
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;

        # Things needed only at compile-time.
        basic-deps = with pkgs; [
          maturin
          findutils
          coreutils
          bash
          zsh
          curl
          git
          jq
        ];

        # Things needed only at compile-time.
        general-deps = [
          rustToolchain
          pkgs.cargo-watch
          pkgs.just

          pkgs.skopeo
          pkgs.dasel
        ];

        benchmark-deps = with pkgs; [
          hyperfine
          heaptrack
        ];

        # The package of this CLI tool.
        # The global version for fuzon.
        # This is gonna get tooled later.
        fuzon = pkgs.callPackage ./pkgs/fuzon {
          inherit self;
          inherit rustToolchain;
        };
      in rec {
        devShells = {
          default = pkgs.mkShell {
            packages = basic-deps ++ general-deps;
          };
          bench = pkgs.mkShell {
            packages =
              basic-deps
              ++ general-deps
              ++ benchmark-deps;
          };
        };

        packages = {
          fuzon = fuzon;

          images = {
            dev = (import ./images/dev.nix) {
              inherit pkgs;
              devShellDrv = devShells.default;
            };

            fuzon = (import ./images/fuzon.nix) {
              inherit pkgs fuzon;
            };
          };
        };
      }
    );
}
