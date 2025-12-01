{
  description = "Advent of Code 2025 - Rust and OCaml";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        ocamlPkgs = pkgs.ocamlPackages;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust
            rustToolchain
            pkgs.cargo-watch

            # OCaml
            ocamlPkgs.ocaml
            ocamlPkgs.dune_3
            ocamlPkgs.ocaml-lsp
            ocamlPkgs.ocamlformat
            ocamlPkgs.utop
            ocamlPkgs.core
            ocamlPkgs.ppx_deriving

            # Common utilities
            pkgs.just
          ];

          shellHook = ''
            echo "Advent of Code 2025 - Rust & OCaml Development Environment"
            echo "Rust: $(rustc --version)"
            echo "OCaml: $(ocaml --version)"
          '';
        };
      }
    );
}
