{
  description = "Nix flake to build the tinted-builder-rust binary";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        # Helper to build for a given Rust target triple using cross toolchains.
        # Note: Apple targets require building on macOS due to SDK/toolchain constraints.
        buildFor = target: let
          pkgsTarget = import nixpkgs {
            localSystem = {inherit system;};
            crossSystem = {config = target;};
          };
          naersk' = pkgsTarget.callPackage naersk {};
        in
          naersk'.buildPackage {
            pname = "tinted-builder-rust";
            src = ./.;
            cargoHash = pkgs.lib.fakeHash; # replace with the hash suggested by nix
            cargoBuildOptions = opts: opts ++ ["-p" "tinted-builder-rust"];
            # Tests typically don't run under cross; disable by default
            doCheck = false;
          };

        hostTarget = pkgs.stdenv.hostPlatform.config;
        hostPkg = buildFor hostTarget;

        linuxTargets = [
          "x86_64-unknown-linux-gnu"
          "i686-unknown-linux-gnu"
          "x86_64-unknown-linux-musl"
          "i686-unknown-linux-musl"
          "aarch64-unknown-linux-gnu"
        ];

        darwinTargets = [
          "aarch64-apple-darwin"
          "x86_64-apple-darwin"
        ];

        targets =
          linuxTargets
          ++ (
            if pkgs.stdenv.isDarwin
            then darwinTargets
            else []
          );

        byTarget = builtins.listToAttrs (map (t: {
            name = t;
            value = buildFor t;
          })
          targets);
      in {
        packages =
          byTarget
          // {
            # Host default and alias
            default = hostPkg;
            tinted-builder-rust = hostPkg;
          };

        apps.default = {
          type = "app";
          program = "${hostPkg}/bin/tinted-builder-rust";
        };

        devShells.default = pkgs.mkShell {
          # Basic Rust toolchain for local development
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };
      }
    );
}
