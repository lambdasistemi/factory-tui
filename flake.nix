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

        version = "0.0.1";

        rustToolchain = import ./nix/toolchain.nix { inherit pkgs; };

        craneEnv = import ./nix/crane.nix {
          inherit pkgs crane rustToolchain;
          src = ./.;
        };

        packages = import ./nix/packages.nix { inherit craneEnv; };
        checks = import ./nix/checks.nix { inherit craneEnv; };
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
