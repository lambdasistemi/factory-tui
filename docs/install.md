# Install

## macOS (Apple Silicon)

```
brew tap lambdasistemi/tap
brew install factory-tui
```

## Linux

Artifacts (AppImage, DEB, RPM, static-musl tarball; x86_64 and
aarch64) are on

https://github.com/lambdasistemi/factory-tui/releases/latest

Example, v0.0.1 on x86_64:

```
curl -L https://github.com/lambdasistemi/factory-tui/releases/download/v0.0.1/factory-tui-0.0.1-x86_64-linux.AppImage -o factory-tui
chmod +x ./factory-tui
```

## From source (Nix)

```
nix build github:lambdasistemi/factory-tui
# from a clone:
nix build .#cli
just ci
```

`./result/bin/factory-tui` is GC-rooted. A `target/release` binary
from `nix develop` can lose its glibc after a store GC; do not bind
that path.

Then bind the popup as in [Using](using.md).
