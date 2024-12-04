{
    description = "Ty's advent of code 2024 solutions";

    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    inputs.flake-utils.url = "github:numtide/flake-utils";

    outputs = { nixpkgs, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem (system: let pkgs = nixpkgs.legacyPackages.${system}; in {
            devShells.default = pkgs.mkShellNoCC {
                packages = with pkgs; [
                    deno
                    kotlin
                    libsecret
                ];
            };
        });
}