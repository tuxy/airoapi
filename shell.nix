# I don't want to cook my system's dependencies
# AHAHAHAHAHAHAHAHAH THIS THING FINALLY WORKS
# To use, enter nix-shell and launch vscode from here
{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    rustup
    rust-analyzer
    cmake
    clang
    pkg-config
    openssl
  ];
}
