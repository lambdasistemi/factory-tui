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

  # `Cargo.toml` is the only maintained product version. Everything Nix
  # names — packages, checks, release artifacts — derives from this.
  crateName = craneLib.crateNameFromCargoToml { cargoToml = src + "/Cargo.toml"; };

  # Rust checks bind the published configuration contract to the real parser.
  # cleanCargoSource excludes Markdown, so admit exactly that one document.
  checkedSource = pkgs.lib.cleanSourceWith {
    inherit src;
    filter = path: type:
      let
        relative = pkgs.lib.removePrefix "${toString src}/" (toString path);
      in
      craneLib.filterCargoSources path type
      || relative == "skills/factory-tui/references/config.md";
  };

  commonArgs = {
    src = checkedSource;
    strictDeps = true;
    inherit (crateName) pname version;

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
  inherit (crateName) version;
}
