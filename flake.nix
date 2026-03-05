{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        weather-widget = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          API_KEY = builtins.getEnv "API_KEY";
          buildInputs = with pkgs; [
            gtk3
            glib
            dbus
            openssl
          ];
          nativeBuildInputs = with pkgs.buildPackages; [ pkg-config ];
        };
      in
      {
        packages.default = weather-widget;
        devShells.default = pkgs.mkShell {
          inputsFrom = [ weather-widget ];
          packages = [ pkgs.rust-analyzer ];
        };
      }
    );
}
