{
  pkgs ?
    import <nixpkgs> {
      overlays = [(import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))];
      config.allowUnfree = true;
    },
}:
pkgs.mkShell {
  # Get dependencies from the main package
  # Additional tooling
  buildInputs = with pkgs; [
    (rust-bin.stable.latest.default.override
      {
        targets = ["wasm32-unknown-unknown"];
      })
    fontconfig
    freetype
    freetype.dev
    libGL
    pkg-config
    mold
    trunk
  ];
}
