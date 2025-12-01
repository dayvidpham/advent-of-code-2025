{
  inputs = {
    nixpkgs.url = "github:NixOS/Nixpkgs/nixos-25.11";
    nixpkgs-unstable.url = "github:NixOS/Nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , nixpkgs-unstable
    , flake-utils
    , fenix
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      lib = pkgs.lib;

      fenixPkgs = fenix.packages."${system}";
      rust-channel = fenixPkgs.default;
      rust-toolchain = rust-channel.toolchain;
      rust-stdlib = "${fenixPkgs.complete.rust-src}/lib/rustlib/src/rust/library";

      shell = pkgs.mkShell {
        packages = [
          rust-toolchain
          fenixPkgs.rust-analyzer
        ];
        RUST_SRC_PATH = rust-stdlib;

        allowSubstitutes = false;
      };
    in
    {
      devShells = {
        default = shell;
      };
    });
}
