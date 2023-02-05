# Changelog

## [Unreleased]

## [0.4.2] - 2023-02-05

### Changed

* Update dependencies

## [0.4.1] - 2022-09-21

### Fixed

* Validate element name, attribute name when displaying
* Encode attribute value when displaying

## [0.4.0] - 2022-09-17

### Changed

* Split `Tag` struct from `Element` enum
* Deprecate map methods for `Element`

### Removed

* The `any-attribute-value-type` feature

## [0.3.0] - 2022-08-30

### Added

* The `any-attribute-value-type` feature
  using `serde_json::value::Value` for attribute value type.

## [0.2.0] - 2022-08-29

### Added

* Simple HTML serialization
* `Clone` and `Default` traits for types
* Map methods (bottom-up and top-down)

## [0.1.0] - 2022-08-27

Initial release.
