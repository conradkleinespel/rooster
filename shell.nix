{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    python3
  ];
  shellHook = ''
    rustup default stable
    rustup component add rust-src
  '';
}
