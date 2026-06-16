## [1.6.1](https://github.com/Pragma8123/game-of-life/compare/v1.6.0...v1.6.1) (2026-06-16)


### Bug Fixes

* **input:** ignore key release events to prevent double triggering on Windows ([7834d13](https://github.com/Pragma8123/game-of-life/commit/7834d13d268959d172c541c0cf2d138017ee6a14))

# [1.6.0](https://github.com/Pragma8123/game-of-life/compare/v1.5.3...v1.6.0) (2026-06-16)


### Bug Fixes

* replace newlines with carriage returns in raw terminal mode ([65311b2](https://github.com/Pragma8123/game-of-life/commit/65311b2904604d482eeec80620b958d35a547e92))


### Features

* add interactive in-game help screen and update clap help description ([544ea4a](https://github.com/Pragma8123/game-of-life/commit/544ea4aaa5bf92fce5acc84d2899ed1cd07e43e6))
* add speed controls, toroidal wrapping, age-based heatmaps, and pattern stamping ([fdf4b96](https://github.com/Pragma8123/game-of-life/commit/fdf4b968190d1f3f7e7b51ee9e15ef7c7908e60b))
* integrate crossterm for raw mode and non-blocking input handling ([13c9095](https://github.com/Pragma8123/game-of-life/commit/13c90958b1b76e8b594358865799c63e9b903515))

## [1.5.3](https://github.com/Pragma8123/game-of-life/compare/v1.5.2...v1.5.3) (2026-05-19)


### Bug Fixes

* correct dep scope error for rand ([36396c2](https://github.com/Pragma8123/game-of-life/commit/36396c243e3a9081c4f393035b73cd72038fbce8))

## [1.5.2](https://github.com/Pragma8123/game-of-life/compare/v1.5.1...v1.5.2) (2025-11-19)


### Bug Fixes

* move away from deprecated rng code ([7a04f85](https://github.com/Pragma8123/game-of-life/commit/7a04f85993d5cf58a559fe708aa00207624f5ffa))

## [1.5.1](https://github.com/Pragma8123/game-of-life/compare/v1.5.0...v1.5.1) (2024-09-12)


### Bug Fixes

* switch from term_size crate to terminal_size ([ae64eb4](https://github.com/Pragma8123/game-of-life/commit/ae64eb48f98cd553e88c042273a76f0721d7f408))

# [1.5.0](https://github.com/Pragma8123/game-of-life/compare/v1.4.0...v1.5.0) (2024-09-12)


### Features

* add full-screen option ([a7d1fad](https://github.com/Pragma8123/game-of-life/commit/a7d1fad923136bf91f2454c91dd68f906eafa720))

# [1.4.0](https://github.com/Pragma8123/game-of-life/compare/v1.3.1...v1.4.0) (2024-09-10)


### Features

* add color and borders to game view ([cd8f6f4](https://github.com/Pragma8123/game-of-life/commit/cd8f6f4212159c377950b06986a9a2b6ebef861a))

## [1.3.1](https://github.com/Pragma8123/game-of-life/compare/v1.3.0...v1.3.1) (2024-09-10)


### Bug Fixes

* re-release ([d20f094](https://github.com/Pragma8123/game-of-life/commit/d20f094312d4a519962ac393e093382414c924f1))

# [1.3.0](https://github.com/Pragma8123/game-of-life/compare/v1.2.3...v1.3.0) (2024-09-10)


### Features

* improve visual quality of rendering ([802f83d](https://github.com/Pragma8123/game-of-life/commit/802f83d8a1930bff290ea050fef3fe746e601926))

## [1.2.3](https://github.com/Pragma8123/game-of-life/compare/v1.2.2...v1.2.3) (2023-02-16)


### Bug Fixes

* **GameOfLife:** :bug: fix out of bounds error when counting neighbors ([d8584ac](https://github.com/Pragma8123/game-of-life/commit/d8584ac8f4dbec1e0d0c8b1839f6fc0a221412f6))

## [1.2.2](https://github.com/Pragma8123/game-of-life/compare/v1.2.1...v1.2.2) (2023-02-16)


### Bug Fixes

* **cli:** :bug: ensure width and height are at least 2 ([673510d](https://github.com/Pragma8123/game-of-life/commit/673510dc4033bab824f1b2f3459d6f3a7412074c))

## [1.2.1](https://github.com/Pragma8123/game-of-life/compare/v1.2.0...v1.2.1) (2023-02-13)


### Bug Fixes

* **cli:** :bug: ensure --speed is at least 1 ([14eadce](https://github.com/Pragma8123/game-of-life/commit/14eadcec0c851cf8dc5e98063e23c509566c0a36))

# [1.2.0](https://github.com/Pragma8123/game-of-life/compare/v1.1.2...v1.2.0) (2023-02-13)


### Features

* **cli:** :sparkles: add game speed cli option ([d8a058a](https://github.com/Pragma8123/game-of-life/commit/d8a058a58d8daddfc141477e8942d7a3097f206a))

## [1.1.2](https://github.com/Pragma8123/game-of-life/compare/v1.1.1...v1.1.2) (2023-02-13)


### Bug Fixes

* ensure Cargo.lock gets updated version on release ([87978a4](https://github.com/Pragma8123/game-of-life/commit/87978a4fcf14f462578972427f52eff0816728bc))

## [1.1.1](https://github.com/Pragma8123/game-of-life/compare/v1.1.0...v1.1.1) (2023-02-13)


### Bug Fixes

* fix issue with Cargo version not updating on release ([5af16d8](https://github.com/Pragma8123/game-of-life/commit/5af16d879a995415c1e1abc1122ac1c8d9e8ef62))

# [1.1.0](https://github.com/Pragma8123/game-of-life/compare/v1.0.0...v1.1.0) (2023-02-13)


### Features

* **cli:** add author and version information ([0877f20](https://github.com/Pragma8123/game-of-life/commit/0877f206559fb7ade118cf4320d69e9f80745f33))
