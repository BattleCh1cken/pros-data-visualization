{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs
          {
            inherit system overlays;
          };

        libraries = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];

        packages = with pkgs; [
          curl
          wget
          pkg-config
          dbus
          openssl_3
          glib
          gtk3
          libsoup
          webkitgtk
          librsvg

          rust-bin.nightly.latest.default
          rust-analyzer
          nodePackages_latest.svelte-language-server

          nodejs
          cargo-tauri
        ];
      in
      {
        devShell = pkgs.mkShell
          {
            buildInputs = packages;

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
            WEBKIT_DISABLE_COMPOSITING_MODE = 1;
            GDK_BACKEND = "x11";

            #RUST_LOG = "trace";
          };
      });
}

