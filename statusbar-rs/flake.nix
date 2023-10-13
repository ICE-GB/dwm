# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # new! 👇
        nativeBuildInputs = with pkgs; [
          pkg-config
          (
            rust-bin.stable.latest.default.override
            {
              extensions = ["rust-src"];
            }
          )
        ];
        # also new! 👇
        buildInputs = with pkgs; [dbus libpulseaudio notmuch openssl lm_sensors alsa-lib];
      in
        with pkgs; {
          devShells.default = mkShell {
            # 👇 and now we can just inherit them
            inherit buildInputs nativeBuildInputs;
          };
        }
    );
}
