{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    systems = [
      "aarch64-linux"
      "x86_64-linux"
    ];
    forAllSystems = nixpkgs.lib.genAttrs systems;
    overlays = [
      rust-overlay.overlays.default
    ];
  in {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          gdk-pixbuf
          glib
          gtk4
          pkg-config
        ];
      };
    });
  };
}
