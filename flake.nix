{
    description = "Ty's advent of code 2024 solutions";

    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    inputs.flake-utils.url = "github:numtide/flake-utils";
    inputs.rust-overlay.url = "github:oxalica/rust-overlay";

    outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
        flake-utils.lib.eachDefaultSystem (system: let pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
        }; in {
            devShells.default = pkgs.mkShell {
                packages = with pkgs; [
                    deno
                    kotlin
                    libsecret
                    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                        extensions = [ "rust-src" "rustfmt" ];
                        targets = [ "x86_64-unknown-linux-gnu" ];
                    }))
                ];
            };
        });
}