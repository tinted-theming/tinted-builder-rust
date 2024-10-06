# Changelog

## Unreleased

### Changed

- Change `get_scheme_files` and `SchemeFile::new` type arguments from
  `&Path` to `impl AsRef<Path>` to allow for more flexibility

## [0.12.1] - 2024-10-06

### Fixed

- Fix bug where CLI gives an error if `output` or `extension`
  `template/config.yaml` properties are empty

## Changed

- BREAKING: Remove `tinted_builder` exports since they should be
  imported from `tinted_builder` crate itself
- Export `get_scheme_files` to allow Rust users to get a
  `&'static [SchemeFile]` from a directory

## [0.12.0] - 2024-10-05

## Changed

- BREAKING: Remove `tinted_builder` exports since they should be
  imported from `tinted_builder` crate itself
- Export `get_scheme_files` to allow Rust users to get a
  `&'static [SchemeFile]` from a directory

## [0.11.1] - 2024-10-02

## Fixed

- Add missing support for `filename` config.yaml property as per 0.11.2
  builder specification

## [0.11.0] - 2024-09-07

## Added

- Add support for proposed 0.12.0 builder spec by adding 16bit rgb
  colour variables to the mustache context

## [0.10.1] - 2024-09-03

## Fixed

- Fix bug where `templates/config.yaml` extension property is always
  prepended by a period `.`. The property is now considered the full
  extension.

## [0.10.0] - 2024-08-28

## Changed

- Breaking change: `tinted-builder` library includes an API change and
  `tinted-builder-rust` now exports the new `tinted-builder` API
- Breaking change: Use `SchemeSystem` and `SchemeVariant` enums for
  scheme `system` and `variant` properties respectively instead of using
  string values
- Schemes are not required to be in directories named after the scheme
  `system`. Any `.yaml` or `.yml` scheme file will be collected into the
  scheme system based on the `system` property defined in the scheme
  file. Current supported systems are `base16` and `base24`

## Added

- Support for `0.11.1` Tinted Theming builder specification. Dotfile
  (`.*.yaml` and `.*.yml`) will be ignored
- Documentation for public operation functions

## Removed

- Remove deprecated `render_to_file` `Template` method

## [0.9.5] - 2024-08-24

## Fixed

- Bump tinted-builder version

## [0.9.4] - 2024-08-24

## Fixed

- Use latest tinted-builder which fixes bug where `{{base0X-dec-r|g|b}}`
  renders `0`

## [0.9.3] - 2024-07-12

### Updated

- Use latest tinted-builder lib which includes patch fixes

## [0.9.2] - 2024-07-12

### Fixed

- Use latest tinted-builder lib which does not generate schemes with
  a hash prefix value

## [0.9.1] - 2024-06-24

### Fixed

- Fix bug where `--quiet` flag was ignored during a `git pull`

## [0.9.0] - 2024-06-23

### Added

- Add `--sync` flag for the `build` subcommand to sync before building

## [0.8.0] - 2024-06-23

### Added

- Add `--quiet` flag for the `sync` subcommand to silence stdout

## [0.7.0] - 2024-06-22

### Added

- Add `--quiet` flag for the `build` subcommand to silence stdout

### Changed

- Remove requirement that all output dir files need the same extension

## [0.6.1] - 2024-06-22

### Changed

- Use latest `tinted-builder` crate

### Fixed

- Allow output directories to contain any extension registered in the
  template config.yaml

## [0.6.0] - 2024-06-11

### Changed

- Update to the latest ribboncurls

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

[0.12.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.11.1...v0.12.0
[0.11.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.5...v0.10.0
[0.9.5]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.3...v0.9.5
[0.9.4]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/tinted-theming/tinted-builder-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tinted-theming/tinted-builder-rust/releases/tag/v0.1.0
