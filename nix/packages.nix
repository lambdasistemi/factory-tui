# Native `factory-tui` CLI. `meta.mainProgram` lets `nix run` and
# bundlers find the executable. Building this derivation is the GC
# root the unrooted `target/release` binary lacked.
{ craneEnv }:
let
  inherit (craneEnv) craneLib commonArgs cargoArtifacts;
in
{
  cli = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
      meta.mainProgram = "factory-tui";
    }
  );
}
