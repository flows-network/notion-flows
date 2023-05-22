{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    pkg-config
    openssl
    protobuf

    wasmedge

    nodejs

    proxychains
  ];
}
