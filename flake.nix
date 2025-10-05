{
  description = "A Bevy development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    ,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system;
          inherit overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Libraries needed at runtime
        libraries = with pkgs; [
          alsa-lib
          vulkan-loader
          libxkbcommon
          libxkbcommon.dev # This provides libxkbcommon-x11.so
          systemd
          wayland
          wayland-protocols
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];

        # Dev commands
        dev-preview = pkgs.writeShellScriptBin "dev-preview" ''
          echo "Running the preview."
          cargo run --features bevy/dynamic_linking,preview "$@"
          echo "Preview done."
        '';
        dev-gen = pkgs.writeShellScriptBin "dev-gen" ''
          echo "Running the video generation."
          cargo run "$@"
          echo "Video generation done."
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              # Rust toolchain
              rustToolchain

              # Core Bevy Dependencies
              pkg-config

              # Video generation
              ffmpeg-full

              # Custom commands
              dev-preview
              dev-gen
            ]
            ++ libraries;

          # Set up library path for runtime
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;

          # Set WINIT default scale
          WINIT_X11_SCALE_FACTOR = 0.7;

          # Alternative: you can also use shellHook to set the path
          shellHook = ''
            echo "TeemLabs commands:"
            echo "- \`dev-preview\`: Run a preview."
            echo "- \`dev-gen\`: Generate a video."
          '';
        };
      }
    );
}
