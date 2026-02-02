{ pkgs, lib, ... }:

{
  packages = with pkgs; [
    shellspec
    git
    argc
    shellcheck
    prek
    pkg-config
    openssl
    zlib
  ] ++ lib.optionals pkgs.stdenv.isDarwin [
    libiconv
  ];

  languages.rust.enable = true;

  enterShell = ''
    echo "Yaks development environment loaded"
  '';

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
