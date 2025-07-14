{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
  outputs = { nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = pkgs.rustPlatform;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (with toolchain; [
              cargo
              rustc
              rustLibSrc
              rust-analyzer
              wasm-pack
              clippy
              rustfmt
              worker-build
            ])
            (wrangler.overrideAttrs (_: { dontCheckForBrokenSymlinks = true; }))
            lld
            pkg-config
            openssl
          ];
          nativeBuildInputs = [ pkgs.pkg-config ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = toolchain.rustLibSrc;
        };
      });
}
