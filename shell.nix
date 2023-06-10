with import <nixpkgs> { };

stdenv.mkDerivation rec {
  name = "interactions-compiler";

  buildInputs = [
    python3
    rustup
    zlib
  ];

  shellHook = ''
    export PATH=$PATH:$HOME/.cargo/bin
  '';
}
