# Changelog

## [Unreleased]

## 0.7.0 - 2026-07-18

* Revert the `Array` and `Object` attribute values added in 0.6.0. Attribute
  values stay scalar, as in the original JsonML syntax; structured
  configuration is represented as child elements.
* Revert the non-exhaustive marking of `Element` and `AttributeValue`.

## 0.6.0 - 2026-07-18

* Add `Array` and `Object` attribute values, completing JSON's value set.
* Widen `Number` from `f32` to `f64` to match JSON's double precision.
* Mark `Element` and `AttributeValue` as non-exhaustive.
* Render structured attribute values as JSON in the HTML projection, with
  object keys sorted so output does not vary with hash iteration order.

## 0.5.1 - 2026-05-10

* Enabled trusted publishing.
* Move project repository to GitHub.

## 0.5.0 - 2026-03-27

* Remove `Tag` struct.
* Delete deprecated methods.
* Update Rust edition.
* Add pre/post order iterators.

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
