# Crane library bound to the pinned toolchain. One dependency-only
# artifact warms the store; the CLI, clippy, fmt, nextest, deny, and
# doc all reuse it.
{ pkgs
, crane
, rustToolchain
, src
}:
let
  craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

  commonArgs = {
    src = craneLib.cleanCargoSource src;
    strictDeps = true;
    pname = "factory-tui";
    version = "0.0.1";

    buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.libiconv
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
{
  inherit craneLib commonArgs cargoArtifacts;
}
