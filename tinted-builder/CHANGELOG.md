# Changelog

## Unreleased

### Fixed

- Fix bug where color `dec` values aren't correctly calculated introduced in
  last release

## 0.10.0 - 2025-11-04

### Changed

- Update dependencies
- Optimise and clean up code

## 0.9.1 - 2025-03-24

### Fixed

- Fix bug where double hash is printed for hex values with `Base16Scheme`
  `fmt::Display`

## 0.9.0 - 2025-03-24

### Added

- Add `Scheme` struct support for `list`, `listbase16` and `listbase24`
  scheme systems

### Changed

- BREAKING: Changed `Color` structs `Display` implementation to prefix with a
- `Base16Scheme` palette hex values are now prepended with a hash `#` to allow
  text editors to optionally highlight the color. This is optional under the
  `0.11.2` builder specification
- Update Ribboncurls crate

## 0.8.0 - 2024-10-05

## Added

- Add `variants` method to `SchemeSystem` to retrieve a
  `&'static [SchemeSystem]`
- Add `Scheme` `get_scheme_author` method
- Add `Scheme` `get_scheme_description` method
- Add `Scheme` `get_scheme_name` method
- Add `Scheme` `get_scheme_slug` method
- Add `Scheme` `get_scheme_variant` method
- Add `PartialEq` derive macro to `SchemeVariant`

## 0.7.0 - 2024-09-07

## Added

- Add support for proposed 0.12.0 builder spec by adding 16bit rgb
  colour variables to the mustache context

## 0.6.0 - 2024-08-28

## Added

- Add basic documentation for docs.rs

## Changed

- Require schemes to be wrapped in `Scheme` enum when creating a
  `Template` struct instance to easily extend builder to support
  different scheme systems
- Use `SchemeSystem` and `SchemeVariant` enums for scheme `system` and
  `variant` properties respectively instead of using string values

## Removed

- `anyhow` crate moved to dev-dependency for tests, but replaced with
  `TintedBuilderError` enum with `thiserror` macros in API

## 0.5.1 - 2024-08-24

## Fixed

- Fix bug where `{{base0X-dec-r|g|b}}` renders `0`

## 0.5.0 - 2024-07-12

## Changed

- Ensure printed scheme puts all values in double quotes

## 0.4.5 - 2024-07-12

## Fixed

- Remove hash from `Color::to_hex` returned string

## 0.4.4 - 2024-06-22

## Fixed

- Fix bug where single quote is not escaped when rendering escaped
  variables

## 0.4.3 - 2024-06-18

## Fixed

- Fix `Scheme` serialize bug

## 0.4.2 - 2024-06-18

## Fixed

- Implement `Serialize` trait for `Scheme` and `Color`

## 0.4.1 - 2024-06-15

## Fixed

- Implement `Display` trait for `Scheme` and `Color`

## 0.4.0 - 2024-06-15

## Added

- Add `Color` struct to public exports to allow users to construct a
  `Scheme` themselves.

## 0.3.0 - 2024-06-11

## Changed

- Updated to latest Ribboncurls

## 0.2.0 - 2024-05-16

### Changed

- Disallow "BSD-3-Clause" licensed packages
- Remove `unidecode` package since it does not have a supported license

## 0.1.0 - 2024-05-03

### Added

- Split tinted-builder-rust library into its own package
- Add new `Template::render` method
- Deprecate `Template::render_to_file` method
