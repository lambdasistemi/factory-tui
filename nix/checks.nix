# Real sandboxed crane derivations. `nix flake check` runs these.
{ craneEnv }:
let
  inherit (craneEnv) craneLib commonArgs cargoArtifacts;
in
{
  clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
    }
  );

  fmt = craneLib.cargoFmt { inherit (commonArgs) src; };

  nextest = craneLib.cargoNextest (
    commonArgs
    // {
      inherit cargoArtifacts;
    }
  );

  deny = craneLib.cargoDeny { inherit (commonArgs) src; };

  doc = craneLib.cargoDoc (
    commonArgs
    // {
      inherit cargoArtifacts;
      env.RUSTDOCFLAGS = "--deny warnings";
    }
  );
}
