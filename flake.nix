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
        dev-genframes = pkgs.writeShellScriptBin "dev-genframes" ''
          echo "Running the frames generation."
          cargo run "$@"
          echo "Frames generation done."
        '';
        dev-genvid = pkgs.writeShellScriptBin "dev-genvid" ''
          echo "Rendering video."
          ffmpeg -framerate 60 -i outputs/frames/frame_%04d.png -c:v libx264 -crf 20 -preset medium -pix_fmt yuv420p outputs/videos/video.mp4
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
              ffmpeg

              # Custom commands
              dev-preview
              dev-genframes
              dev-genvid
            ]
            ++ libraries;

          # Set up library path for runtime
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;

          # Set WINIT default scale
          WINIT_X11_SCALE_FACTOR = 1.0;

          # Alternative: you can also use shellHook to set the path
          shellHook = ''
            echo "TeemLabs commands:"
            echo "- \`dev-preview\`: Run a preview."
            echo "- \`dev-generate\`: Run a generation, save frames and render the video."
          '';
        };
      }
    );
}
