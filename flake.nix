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
          echo "Targeting latest output frames."
          # Get the latest folder name matching pattern sim_*Z
          LATEST_SIM=$(find outputs -maxdepth 1 -type d -name "sim_*Z" | sort | tail -1 | xargs basename)
          
          if [ -z "$LATEST_SIM" ]; then
            echo "No simulation folders found"
            exit 1
          fi
          echo "Rendering video for $LATEST_SIM."
          ffmpeg -framerate 30 -i outputs/$LATEST_SIM/frames/frame_%06d.png \
          -c:v h264_nvenc -preset p7 -rc vbr -b:v 8M -maxrate 8M -bufsize 16M \
          -pix_fmt yuv420p -movflags +faststart \
          outputs/$LATEST_SIM/$LATEST_SIM.mp4

          echo "Video generation done."
        '';
        dev-gen = pkgs.writeShellScriptBin "dev-gen" ''
          dev-genframes
          sleep 1
          dev-genvid
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
              dev-genframes
              dev-genvid
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
            echo "- \`dev-genframes\`: Generate frames."
            echo "- \`dev-genvid\`: Generate video using the latest simulation's frames."
            echo "- \`dev-gen\`: Generate frames then video."
          '';
        };
      }
    );
}
