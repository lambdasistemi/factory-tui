{
  description = "factory-tui: browse the agent factory as a tree of seats";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    bundlers = {
      url = "github:NixOS/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    dev-assets.url = "github:paolino/dev-assets/v0.1.0";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , crane
    , rust-overlay
    , bundlers
    , dev-assets
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = import ./nix/toolchain.nix { inherit pkgs; };

        craneEnv = import ./nix/crane.nix {
          inherit pkgs crane rustToolchain;
          src = ./.;
        };

        # One version authority: Cargo.toml, read through the crane env and
        # reused by the packages, the checks, and every release artifact.
        inherit (craneEnv) version;

        # Exact source revision when the flake source is clean, the same
        # commit marked `-dirty` when it is not, and an empty string when Nix
        # has no source metadata at all. Evaluation supplies this; the build
        # never runs Git, reads a clock, or touches the network.
        revision = self.rev or self.dirtyRev or "";

        packages = import ./nix/packages.nix { inherit craneEnv revision; };
        checks = import ./nix/checks.nix {
          inherit craneEnv pkgs packages self;
          artifacts = identityArtifacts;
          # The crate derivations see only the cleaned Cargo source, so the tag
          # policy check needs the unfiltered tree to reach the release scripts,
          # their proof, and the milestone doc.
          src = ./.;
        };
        apps = import ./nix/apps.nix { inherit pkgs; };

        linuxArtifacts = dev-assets.lib.mkLinuxArtifacts {
          inherit pkgs system version;
          executableName = "factory-tui";
          glibcPackage = packages.cli;
          muslPackage = packages.cli-musl;
          inherit bundlers;
        };

        devVersion = "${version}-${self.shortRev or (self.dirtyShortRev or "dirty")}";
        linuxDevArtifacts = dev-assets.lib.mkLinuxArtifacts {
          inherit pkgs system version;
          artifactVersion = devVersion;
          executableName = "factory-tui";
          glibcPackage = packages.cli;
          muslPackage = packages.cli-musl;
          inherit bundlers;
        };

        mkDarwin = extra: dev-assets.lib.mkDarwinHomebrewBundle { inherit pkgs; } ({
          pname = "factory-tui";
          inherit version;
          owner = "lambdasistemi";
          desc = "Browse the agent factory as a tree of seats";
          formulaClass = "FactoryTui";
          executables = { factory-tui = packages.cli; };
          smokeCommands = [
            "factory-tui >/tmp/factory-tui-smoke.out 2>&1 || true"
            ''grep -F -- "not inside tmux" /tmp/factory-tui-smoke.out''
          ];
        } // extra);

        darwinArtifacts = mkDarwin { };

        darwinDevArtifacts = mkDarwin {
          artifactVersion = devVersion;
          releaseTag = "dev-homebrew";
          formulaName = "factory-tui-dev";
          formulaClass = "FactoryTuiDev";
          formulaVersion = devVersion;
        };

        # Every release artifact this system publishes, offered to the
        # identity check so its stated version is reconciled rather than
        # trusted. A `dev` artifact additionally has to carry the source
        # revision it claims to be a snapshot of.
        identityArtifacts =
          pkgs.lib.optionalAttrs pkgs.stdenv.isLinux {
            "artifact.linux-release" = { inherit (linuxArtifacts) name; kind = "release"; };
            "artifact.linux-dev" = { inherit (linuxDevArtifacts) name; kind = "dev"; };
          }
          // pkgs.lib.optionalAttrs pkgs.stdenv.isDarwin {
            "artifact.darwin-release" = { inherit (darwinArtifacts) name; kind = "release"; };
            "artifact.darwin-dev" = { inherit (darwinDevArtifacts) name; kind = "dev"; };
          };
      in
      {
        packages = {
          default = packages.cli;
          inherit (packages) cli;
        }
        // pkgs.lib.optionalAttrs pkgs.stdenv.isLinux {
          inherit (packages) cli-musl;
          factory-tui-linux-release-artifacts = linuxArtifacts;
          factory-tui-linux-dev-release-artifacts = linuxDevArtifacts;
          linux-artifact-smoke = dev-assets.lib.mkLinuxArtifactSmoke { inherit pkgs system; };
        }
        // pkgs.lib.optionalAttrs pkgs.stdenv.isDarwin {
          factory-tui-darwin-release-artifacts = darwinArtifacts;
          factory-tui-darwin-dev-homebrew-artifacts = darwinDevArtifacts;
        };

        inherit checks apps;

        devShells.default = craneEnv.craneLib.devShell {
          packages = [
            pkgs.just
            pkgs.cargo-deny
            pkgs.tmux
          ];
        };
      }
    );
}
