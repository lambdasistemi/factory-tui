# Real sandboxed crane derivations. `nix flake check` runs these.
{ craneEnv, pkgs, packages, self, artifacts }:
let
  inherit (craneEnv) craneLib commonArgs cargoArtifacts;
  inherit (pkgs) lib;

  # The provenance authority, read from the flake source metadata here rather
  # than from the value threaded into the packages. Sourcing it separately is
  # the whole point: a build that pins its revision to anything other than its
  # own source must disagree with this.
  sourceRevision = self.rev or self.dirtyRev or "";
  sourceShortRevision = self.shortRev or self.dirtyShortRev or "";

  onLinux = lib.optionalAttrs pkgs.stdenv.isLinux;
  rows = attrs: lib.concatStringsSep "\n" (lib.mapAttrsToList (k: v: "${k}\t${v}") attrs);
in
{
  # Reconcile every surface that states an identity against its authority: the
  # manifest for versions, the flake source for provenance. Surfaces are
  # tables, so covering a new one is data rather than new logic.
  version-consistency = pkgs.runCommand "factory-tui-version-consistency"
    {
      CARGO_TOML = "${commonArgs.src}/Cargo.toml";
      EXPECTED_REVISION = sourceRevision;
      EXPECTED_SHORT_REVISION = sourceShortRevision;
      # Mirrors build_info::UNKNOWN_REVISION: what a binary prints when the
      # build handed it no revision at all.
      FALLBACK_REVISION = "unknown";
      PACKAGE_VERSIONS = rows (
        { "package.cli" = packages.cli.version; }
        // onLinux { "package.cli-musl" = packages.cli-musl.version; }
      );
      RUNTIME_BINARIES = rows (
        { "runtime.cli" = "${packages.cli}/bin/factory-tui"; }
        // onLinux { "runtime.cli-musl" = "${packages.cli-musl}/bin/factory-tui"; }
      );
      ARTIFACT_NAMES = lib.concatStringsSep "\n" (
        lib.mapAttrsToList (name: a: "${name}\t${a.name}\t${a.kind}") artifacts
      );
    } ''
    bash ${../scripts/release/reconcile-identity}
    touch "$out"
  '';

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
