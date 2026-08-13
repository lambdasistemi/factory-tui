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

  # Static musl tarball (Linux only). Pure Rust (ratatui + crossterm).
  muslTarget =
    if pkgs.stdenv.hostPlatform.isAarch64
    then "aarch64-unknown-linux-musl"
    else "x86_64-unknown-linux-musl";
  muslArgs = commonArgs // {
    CARGO_BUILD_TARGET = muslTarget;
    CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
    doCheck = false;
  };
  cargoArtifactsMusl = craneLib.buildDepsOnly muslArgs;
in
{
  inherit craneLib commonArgs cargoArtifacts muslArgs cargoArtifactsMusl;
}
