let
  sources = import ./sources.nix;
  rust = import ./rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.pkgconfig
    pkgs.freetype
    pkgs.fontconfig
  ];

  LIBZ_SYS_STATIC=1;

  OPENSSL_STATIC=1;
  OPENSSL_DIR = pkgs.pkgsStatic.openssl.dev;
  OPENSSL_LIB_DIR = "${pkgs.pkgsStatic.openssl.out}/lib";
}
