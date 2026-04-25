{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "rooster";
  version = "2.14.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  doCheck = false;

  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.python3
  ];
  buildInputs = [
    pkgs.xorg.libX11
    pkgs.xorg.libXmu
    pkgs.xsel
    pkgs.wl-clipboard
  ];

  meta = with pkgs.lib; {
    description = "A simple password manager";
    homepage = "https://github.com/conradkleinespel/rooster";
    license = licenses.asl20;
    maintainers = [];
  };
}
