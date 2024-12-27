/*
based on
https://discourse.nixos.org/t/how-can-i-set-up-my-rust-programming-environment/4501/9
club always attitude shiver pumpkin forum riot van sell deposit else recall
*/
let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "latest";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };
in
pkgs.mkShell {
  buildInputs = [
    #pkgs.darwin.apple_sdk.frameworks.Security
    rust
  ] ++ (with pkgs; [
    clang
    rustup
    cargo
    rust-analyzer
    cmake
  ]);
  RUST_BACKTRACE = 1;
}


