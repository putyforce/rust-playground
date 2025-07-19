{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # some binaries
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
      protobuf
      cmake
      pkg-config
    ];
    # some libs
    buildInputs = with pkgs; [
       openssl
       zlib
    ];
}
