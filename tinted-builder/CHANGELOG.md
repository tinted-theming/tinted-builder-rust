# Changelog

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
