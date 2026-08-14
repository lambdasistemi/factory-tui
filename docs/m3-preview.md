# Milestone 3 preview

The Milestone 3 preview is a **temporary** Nix-first channel for trying M3 work
before it graduates into an ordinary product release. It is not a release.

The supported reference is `milestone-3-preview.1`. The family is
`milestone-3-preview.<n>`, where `<n>` is a positive decimal integer starting
at 1. Note the absence of a leading `v`: preview refs are deliberately outside
the product `v<x.y.z>` namespace, and `scripts/release/check-milestone-tag`
exists to keep them there.

## Install

Nix is the only supported path:

```
nix profile add github:lambdasistemi/factory-tui/milestone-3-preview.1
```

That resolves the Git tag directly from GitHub; no clone and no checkout is
needed. Then follow [Using](using.md).

## What this preview is not

- There is **no GitHub release** object for a preview ref, so it never appears
  on the releases page and never becomes *Latest*.
- There are **no prebuilt binary assets**: no Linux tarball, `.deb`, `.rpm`,
  AppImage, or Homebrew formula. Those exist only for product releases.
- It is **not** `v0.1.0` or any later `v<x.y.z>` tag. `v0.1.0` remains the
  current product release and the thing to install if you want a supported
  build; the preview is a moving snapshot of unfinished work.
- It carries no upgrade, compatibility, or support promise between ordinals.

## Graduation and retirement

M3 graduates through the next ordinary release-please product release — a
normal `v<x.y.z>` tag cut from a release-please pull request, with the usual
changelog, GitHub release, and binary assets.

Once that release exists, the preview channel has no remaining purpose and is
retired: the temporary milestone tag is deleted and no further ordinals are
published. Install the product release instead.
