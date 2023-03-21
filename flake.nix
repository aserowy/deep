{
  description = "service spo provisionierung";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        rust-stable = pkgs.rust-bin.stable.latest.default;

        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
      in
      with pkgs; rec {
        devShell =
          pkgs.mkShell rec {
            buildInputs = [

              # coding env
              rnix-lsp
              rust-analyzer
              rust-stable

              # build dependencies
              alsa-lib
              pkg-config
              udev
              vulkan-loader

              ## wayland
              libxkbcommon
              wayland

              ## x11
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ];
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH

              export RUST_BACKTRACE=1
              export WINIT_UNIX_BACKEND=wayland
            '';

            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      });
}
