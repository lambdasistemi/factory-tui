# Install

Nix is the supported path. GitHub release binaries exist for hosts
without Nix.

## Nix

User profile:

```
nix profile add github:lambdasistemi/factory-tui
```

Pin a tag:

```
nix profile add github:lambdasistemi/factory-tui/v0.1.0
```

One-off from a checkout:

```
nix build .#cli
```

On NixOS, add the flake's `packages.<system>.default` to
`environment.systemPackages`. The flake `default` package is the
CLI.

Do not bind `target/release`. Then follow [Using](using.md).

## Without Nix

macOS (Apple Silicon):

```
brew tap lambdasistemi/tap
brew install factory-tui
```

Linux binaries:

https://github.com/lambdasistemi/factory-tui/releases/latest
