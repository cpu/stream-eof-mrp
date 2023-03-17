{ pkgs ? import <nixpkgs> { } }:
let
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = with pkgs; [
      rust.packages.stable.rustPlatform.rustLibSrc
      rustc
      cargo
      rustfmt
      clippy
    ];
  };
in pkgs.mkShell {
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_TOOLCHAIN_PATH = "${rust-toolchain}";

  nativeBuildInputs = with pkgs; [ rust-toolchain ];
}
