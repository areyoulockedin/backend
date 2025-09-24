{
  description = "A simple flake for areyoulocked.in";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        shellConfig = import ./nix/devshell.nix;
      in {
        devShell = shellConfig {inherit pkgs;};
      }
    );
}
