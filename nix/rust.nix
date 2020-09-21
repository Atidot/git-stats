{ sources ? import ./sources.nix }:

let
pkgs =
  import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla)
  (self: super: {
    libgit2 = super.pkgs.pkgsMusl.libgit2;
  })
  ]; };
  channel = "nightly";
  date = "2020-09-02";
  targets = [ "x86_64-unknown-linux-musl"
              "wasm32-unknown-unknown"
            ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan
