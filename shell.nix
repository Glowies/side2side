{ pkgs ? import <nixpkgs> {} }:
  let

in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
    buildInputs = with pkgs; [
      openssl
      clang
      llvmPackages.bintools
      rustup
    ];
    shellHook = ''
      alias y=yazi
      alias lg=lazygit
      alias vi=nvim
    '';
  }