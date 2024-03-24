let
  pkgs = import <nixpkgs> {};
  sources = import ./nix/sources.nix;
  naersk = pkgs.callPackage sources.naersk {};

in
  naersk.buildPackage ./.

#### EXAMPLE with variables and options 
/*
naersk.buildPackage {
  # Assuming there's `Cargo.toml` right in this directory:
  src = ./.;

  someOption = "yass";
  someOtherOption = false;
  CARGO_ENVIRONMENTAL_VARIABLE = "test";
}
*/