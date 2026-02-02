{ pkgs, ... }:

{
  packages = with pkgs; [
    shellspec
    git
    argc
    shellcheck
    prek
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
