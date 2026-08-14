# `nix run .#<name>` and the justfile share these definitions.
{ pkgs }:
let
  system = pkgs.stdenv.hostPlatform.system;

  mkApp = name: text:
    let
      app = pkgs.writeShellApplication {
        inherit name text;
        runtimeInputs = [ pkgs.nix ];
      };
    in
    {
      type = "app";
      program = pkgs.lib.getExe app;
    };

  checkNames = [ "clippy" "fmt" "nextest" "deny" "doc" "version-consistency" "tag-policy" ];
  checkTargets =
    pkgs.lib.concatMapStringsSep " " (n: ".#checks.${system}.${n}") checkNames;
in
{
  build = mkApp "build" "nix build .#cli";
  test = mkApp "test" "nix build .#checks.${system}.nextest";
  clippy = mkApp "clippy" "nix build .#checks.${system}.clippy";
  fmt-check = mkApp "fmt-check" "nix build .#checks.${system}.fmt";
  deny = mkApp "deny" "nix build .#checks.${system}.deny";
  tag-policy = mkApp "tag-policy" "nix build .#checks.${system}.tag-policy";

  ci = mkApp "ci" ''
    nix build \
      .#cli \
      ${checkTargets}
  '';
}
