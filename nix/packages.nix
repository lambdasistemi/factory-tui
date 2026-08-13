# Native CLI plus its static musl sibling. `meta.mainProgram` is
# required so the NixOS bundlers can resolve the executable.
{ craneEnv }:
let
  inherit (craneEnv) craneLib commonArgs cargoArtifacts muslArgs cargoArtifactsMusl;

  cli = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
      meta.mainProgram = "factory-tui";
    }
  );

  cli-musl = craneLib.buildPackage (
    muslArgs
    // {
      cargoArtifacts = cargoArtifactsMusl;
      meta.mainProgram = "factory-tui";
    }
  );
in
{
  inherit cli cli-musl;
}
