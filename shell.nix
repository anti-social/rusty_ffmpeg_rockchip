{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    libdrm
    meson
    pkg-config
  ];
}
