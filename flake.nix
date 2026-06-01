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
    packages = forAllSystems(system: let
       pkgs = import nixpkgs {
           inherit system;
        };
        in {
            ignis-notifications-glib = pkgs.callPackage ./crates/notifications_glib {};
        }
    );

    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
            gobject-introspection
            pkg-config
        ];
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          gdk-pixbuf
          glib
          gtk4
          meson
          ninja
        ];
      };
    });
  };
}
