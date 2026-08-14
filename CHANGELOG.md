# Changelog

## [0.1.1](https://github.com/lambdasistemi/factory-tui/compare/v0.1.0...v0.1.1) (2026-08-14)


### Bug Fixes

* fall back on invisible titles and prove the real label sites ([5ce90a3](https://github.com/lambdasistemi/factory-tui/commit/5ce90a3517ef1325d7f8361389c13af32c4ffa56))
* identify panes by tmux title ([7190a10](https://github.com/lambdasistemi/factory-tui/commit/7190a100c6052872357897e29de1d09c2cca9402))
* identify panes by tmux title ([0c4cfbc](https://github.com/lambdasistemi/factory-tui/commit/0c4cfbc05b4aeab2443862f360befa5572532403))
* replace projection with raw tmux tree ([ea047e0](https://github.com/lambdasistemi/factory-tui/commit/ea047e04edbcb1ab2ecad2f91b5442d6494903e4))
* replace projection with raw tmux tree ([599f429](https://github.com/lambdasistemi/factory-tui/commit/599f429ad9b906cbb69ba323cd44c56f7fd956c6)), closes [#26](https://github.com/lambdasistemi/factory-tui/issues/26)
* report truthful build identity ([594c882](https://github.com/lambdasistemi/factory-tui/commit/594c882cc9faf8ce4b919f796d662219f0c1c88e))
* report truthful version and build provenance ([f3ac64a](https://github.com/lambdasistemi/factory-tui/commit/f3ac64a4f73a954123a672e4a290298a54329186))

## [0.1.0](https://github.com/lambdasistemi/factory-tui/compare/v0.0.1...v0.1.0) (2026-08-13)


### Features

* **#5:** default tree is sessions and windows; host tables live in config ([5403c31](https://github.com/lambdasistemi/factory-tui/commit/5403c31f3db6131acfde0ab91f0a6036aa09221b))
* default tree is sessions and windows; host tables live in config ([613deb3](https://github.com/lambdasistemi/factory-tui/commit/613deb3d592757af7373a43c0440721ab3faa912))
* fold windows from optional config rules ([4c6290e](https://github.com/lambdasistemi/factory-tui/commit/4c6290e37447650cf8dad67b0b0b1e73f654d263))
* fold windows from optional config rules ([adfe1b9](https://github.com/lambdasistemi/factory-tui/commit/adfe1b900f5788af4a9ddf80f1f7830876b1a80c))

## [0.0.1](https://github.com/lambdasistemi/factory-tui/releases/tag/v0.0.1)

Initial browse-camera prototype.

- Factory tree over live tmux windows (not a session list).
- Coloured snapshot preview that does not resize the live seat.
- Enter / double-click jumps the attached client to the selected seat.
