{
  description = "tinted-builder WASI component development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-wasip2"];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.alejandra
            pkgs.cargo-about
            pkgs.cargo-deny
            pkgs.just
            pkgs.wasm-tools
            pkgs.wasmtime
          ];

          shellHook = ''
            echo "WASI Component Model dev shell"
            echo "  rustc:    $(rustc --version)"
            echo "  wasmtime: $(wasmtime --version)"
            echo "  wasm-tools: $(wasm-tools --version)"
            echo ""
            echo "Build with:  cargo build -p tinted-builder --target wasm32-wasip2"
          '';
        };
      };
    };
}
