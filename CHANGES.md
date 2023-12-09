# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Security

- Dependency Updates

## [0.4.2] - 2023-12-09

### Security

- Dependency Updates

## [0.4.1] - 2022-12-05

### Bug Fixes

- Random bool never returned true (thanks [Ekleog])
- usize for words::sentence (thanks [Ekleog])
- Update chronos to 4.0.23
- Adopt from_timestamp_opt
- Fix nextsv argument identification

### Security

- Dependency Updates

## [0.4.0] - 2022-10-31

### Bug Fixes

- Correct references to person data in other modules
- Update rust crate uuid to 1.1
- Update rust crate chrono to 0.4.20
- Update rust crate chrono to 0.4.21
- Update rust crate chrono to 0.4.22
- (misc) remove duplicates in street list
- 1.63 clippy compliance

### Documentation

- Bring CHANGELOG and CHANGES up to date
- Update documentation as requires feature to be set.
- Document feature requirement for person module
- Update generator module docs to include reference to feature
- ✨ Update crate documentation for features

### Features

- ✨ Place payment module behind payment feature
- ✨ All feature to enable all features.
- Basic Test first to fail testing quickly
- Setup person feature for person module.
- ✨ Place the generator module behind a feature flag
- ✨ Place modules behind features and adaptions
- ✨ Put name module behind feature flag
- ✨ Place address module behind feature flag
- ✨ Place company module behind feature flag
- ✨ Put job module behind a feature flag
- ✨ Put words module behind a feature flag
- ✨ Put hipster module behind a feature flag
- ✨ Put misc module behind feature flag
- ✨ Put vehicle module behind feature flag
- ✨ Put user agent module behind a feature flag
- ✨ Put unique module behind a feature flag
- ✨ Put status_code module behind feature
- ✨ Put password module behind feature
- 🐛 Remove misc dependencies on other modules
- Feature
- ✨ Add datetime feature to CI test set
- ✨ Complete move of datetime and documentation
- ✨ Put log-level module behind a faature flag
- ✨ Put language module behind a feature flag
- ✨ Put file module behind feature flag
- ✨ Put currency ,module behind a feature
- ✨ Put color module behind feature and rename to colour
- ✨ Put bool_rand behind a feature
- ✨ Put the beer module behind a feature.
- ✨ Put animal module behind feature
- ✨ Put image module behind a feature

### Miscellaneous Tasks

- Adopt custom docker image
- Release job to
- Second workflow and triggers

### Refactor

- 🎨 Combined generators and data into a payment module
- 🎨 Removed unnecessary command configurations

### Ci

- Update Release Drafter with more complex release configuration
- Test the features provided in the crate

## [0.3.0](https://github.com/jerusdp/mockd/compare/v0.2.0...v0.3.0) (2022-05-01)

### Features

* Documentation in code files to drive docs.rs

### Bug Fixes

* Dependency updates

## [0.2.0](https://github.com/jerusdp/mockd/compare/v0.1.1...v0.2.0) (2022-03-06)

### Features

* **Generators:** ✨ Generators based on Fakeit with security fixes ([7dc9302](https://github.com/jerusdp/mockd/commit/7dc9302f2f426efd3eb7eed22748aadd142ae2de))

### [0.1.1](https://github.com/jerusdp/mockd/compare/v0.1.0...v0.1.1) (2022-02-27)

### Bug Fixes

* 🐛 Too many keywords for crates.io ([60a41a3](https://github.com/jerusdp/mockd/commit/60a41a3f21427c137b2c60d5b2a89b267a73d8b1))

## 0.1.0 (2022-02-27)

### Features

* ✨ Initial package definition ([6996349](https://github.com/jerusdp/mockd/commit/6996349ca82c1050cb4ed23f72b0cb729950cc2d))

<!-- next-url -->
[Unreleased]: https://github.com/jerusdp/mockd/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/jerusdp/mockd/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/jerusdp/mockd/compare/v0.4.0...v0.4.1
[0.4.1]: https://github.com/jerusdp/mockd/compare/v0.3.0...v0.4.0
[0.3.0]: <https://github.com/jerusdp/mockd/compare/v0.2.0...v0.3.0>"
[0.2.0]: <https://github.com/jerudp/mockd/compare/v0.1.1...v0.2.0>
[0.1.0]: <https://github.com/jerudp/nextsv/compare/...v0.1.0>
[Ekleog]: <https://github.com/Ekleog>
