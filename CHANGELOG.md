# Changelog

## [0.5.0] - 2024-05-16

### Changed

- Use latest `tinted-builder` lib which removes dependency on all 
  BSD-3-Clause licensed packages

## [0.4.0] - 2024-05-03

### Changed

- Split the tinted-builder-rust cli and library into two different
  packages.

## [0.3.0] - 2024-04-01

### Changed

- Changed from GPL-3 to MIT or Apache-2.0 license

## [0.2.2] - 2024-03-27

### Fixed

- Fixed bug where required `build` arg was not required to be there

## [0.2.1] - 2024-03-11

### Fixed

- Generated files should not end with a new line

## [0.2.0] - 2024-03-11

### Added

- Github tests workflow for PRs and merges to main branch
- Homebrew installation instructions
- Release binary installation instructions
- Ensure rendered output contains a new line at end of file
- Add error when a template config mustache file is not found

### Fixed

- Fix bug where `template/config.yaml` extension value isn't checked
  properly when the extension has a dot: eg `extension = .theme.json`
- README.md links
- Builder now reads scheme slug instead of always inferring based on
  scheme name
- Fix template slugify function to ensure it approximates unicode to
  ascii

### Updated

- `make install` now only installs if deps are missing

## [0.1.0] - 2024-03-08

### Added

- Initial release
- Support for `0.11.0` Tinted Theming builder specification
- Support for consuming the project as a library crate
- `sync` subcommand support to sync with latest Tinted Theming schemes
- `build` subcommand to trigger theme template build

[0.4.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tinted-theming/tinted-builder-rust/releases/tag/v0.1.0
