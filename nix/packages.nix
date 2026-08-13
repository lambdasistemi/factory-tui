# Native CLI plus its static musl sibling. `meta.mainProgram` is
# required so the NixOS bundlers can resolve the executable.
#
# `revision` is the flake's own source revision, supplied by evaluation
# rather than discovered during the build: no Git process, no clock, no
# network. It is carried only on the executables, so a new commit does not
# invalidate the shared dependency artifact. An empty string means Nix had no
# source metadata, and the binary then reports its provenance as unknown.
{ craneEnv, revision }:
let
  inherit (craneEnv) craneLib commonArgs cargoArtifacts muslArgs cargoArtifactsMusl;

  identity = { env.FACTORY_TUI_REVISION = revision; };

  cli = craneLib.buildPackage (
    commonArgs
    // identity
    // {
      inherit cargoArtifacts;
      meta.mainProgram = "factory-tui";
    }
  );

  cli-musl = craneLib.buildPackage (
    muslArgs
    // identity
    // {
      cargoArtifacts = cargoArtifactsMusl;
      meta.mainProgram = "factory-tui";
    }
  );
in
{
  inherit cli cli-musl;
}
