# Changelog

## Unreleased

### Added

- Github tests workflow for PRs and merges to main branch
- Homebrew installation instructions
- Release binary installation instructions
- Ensure rendered output contains a new line at end of file

### Fixed

- Fix bug where `template/config.yaml` extension value isn't checked
  properly when the extension has a dot: eg `extension = .theme.json`
- README.md links

### Updated

- `make install` now only installs if deps are missing

## [0.1.0] - 2024-03-08

### Added

- Initial release
- Support for `0.11.0` Tinted Theming builder specification
- Support for consuming the project as a library crate
- `sync` subcommand support to sync with latest Tinted Theming schemes
- `build` subcommand to trigger theme template build

[0.1.0]: https://github.com/tinted-theming/tinted-builder-rust/releases/tag/v0.1.0
