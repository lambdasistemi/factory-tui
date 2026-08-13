# Install

## macOS (Apple Silicon)

```
brew tap lambdasistemi/tap
brew install factory-tui
```

## Linux

https://github.com/lambdasistemi/factory-tui/releases/latest

```
curl -L https://github.com/lambdasistemi/factory-tui/releases/latest/download/factory-tui -o factory-tui
```

Or grab the versioned AppImage / DEB / RPM / musl tarball from the
same page.

## From source (Nix)

```
nix build github:lambdasistemi/factory-tui
nix build .#cli
just ci
```

Do not bind `target/release`. Then follow [Using](using.md).
