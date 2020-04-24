with import <nixpkgs> { };

stdenv.mkDerivation rec {
  name = "interactions-compiler";

  buildInputs = [
    rustup
  ];
}
