with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [
    cargo
    rustc
    inkscape
  ];
}
